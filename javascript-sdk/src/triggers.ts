import type { Peer } from "tw2-jsonrpc"
import type { Row, RunOptions } from "./types.js"
import { definedParams, withRunOptions } from "./utils.js"

export interface TriggerSignature {
  name: string
  type: string
}

export interface TriggerDelta {
  added: Row[]
  deleted: Row[]
}

export interface TriggerOptions extends RunOptions {
  schedule?: string
  reportNoops?: boolean
  enabled?: boolean
}

interface TriggerDefinition {
  name: string
  prompt?: string
  sql?: string
  schedule?: string
  batch_size?: number
  timeout_ms?: number
  report_noops: boolean
  enabled: boolean
  signature: TriggerSignature[]
}

interface TriggerList {
  triggers: string[]
}

interface TriggerPoll {
  deltas: TriggerDelta[]
}

interface TriggerWatch {
  resume_token: string
}

export type TriggerCreation =
  | ({ sql: string; prompt?: never } & TriggerOptions)
  | ({ prompt: string; sql?: never } & TriggerOptions)

export type TriggerUpdate = Partial<{
  sql: string
  prompt: string
  schedule: string
  batchSize: number
  timeoutMs: number
  reportNoops: boolean
  enabled: boolean
}>

export type TriggerCallback = (
  added: Row[],
  deleted: Row[],
) => void | Promise<void>

export class TriggersService {
  readonly #peer: Peer

  constructor(peer: Peer) {
    this.#peer = peer
  }

  async create(name: string, creation: TriggerCreation): Promise<Trigger> {
    const definition = await this.#peer.call<TriggerDefinition>(
      "triggers.create",
      triggerParams({ name, ...creation }),
    )
    return new Trigger(this.#peer, definition)
  }

  async get(name: string): Promise<Trigger> {
    const definition = await this.#peer.call<TriggerDefinition>("triggers.get", { name })
    return new Trigger(this.#peer, definition)
  }

  async list(): Promise<string[]> {
    const response = await this.#peer.call<TriggerList>("triggers.list")
    return response.triggers
  }
}

export class Trigger {
  readonly #peer: Peer
  #name!: string
  #prompt?: string
  #sql?: string
  #signature!: TriggerSignature[]
  #options!: Required<Pick<TriggerOptions, "reportNoops" | "enabled">> & TriggerOptions
  #resumeToken?: string

  constructor(peer: Peer, definition: TriggerDefinition) {
    this.#peer = peer
    this.#replace(definition)
  }

  get name(): string {
    return this.#name
  }

  get prompt(): string | undefined {
    return this.#prompt
  }

  get sql(): string | undefined {
    return this.#sql
  }

  get signature(): readonly TriggerSignature[] {
    return this.#signature
  }

  get options(): TriggerOptions {
    return { ...this.#options }
  }

  get enabled(): boolean {
    return this.#options.enabled
  }

  get watched(): boolean {
    return this.#resumeToken !== undefined
  }

  async update(update: TriggerUpdate = {}): Promise<void> {
    const resumeToken = this.#resumeToken
    const definition = await this.#peer.call<TriggerDefinition>(
      "triggers.update",
      triggerParams({ name: this.#name, ...update }),
    )
    this.#replace(definition)
    this.#resumeToken = resumeToken
  }

  delete(): Promise<void> {
    return this.#peer.call<void>("triggers.delete", { name: this.#name })
  }

  async enable(): Promise<void> {
    await this.#peer.call<void>("triggers.enable", { name: this.#name })
    this.#options.enabled = true
  }

  async disable(): Promise<void> {
    await this.#peer.call<void>("triggers.disable", { name: this.#name })
    this.#options.enabled = false
  }

  run(options: RunOptions = {}): Promise<TriggerDelta> {
    return this.#peer.call<TriggerDelta>(
      "triggers.run",
      withRunOptions({ name: this.#name }, options),
    )
  }

  async poll(): Promise<TriggerDelta[]> {
    const response = await this.#peer.call<TriggerPoll>("triggers.poll", {
      name: this.#name,
    })
    return response.deltas
  }

  async watch(
    callback: TriggerCallback,
    options: { resumeWindowMs?: number } = {},
  ): Promise<string> {
    if (this.#resumeToken !== undefined) {
      throw new Error("trigger is already watching")
    }

    this.#registerWatcher(callback)
    const response = await this.#peer.call<TriggerWatch>(
      "triggers.watch",
      definedParams({
        name: this.#name,
        resume_window_ms: options.resumeWindowMs,
      }),
    )
    this.#resumeToken = response.resume_token
    return response.resume_token
  }

  async resume(
    resumeToken: string,
    callback: TriggerCallback,
    options: { resumeWindowMs?: number } = {},
  ): Promise<string> {
    if (this.#resumeToken !== undefined) {
      throw new Error("trigger is already watching")
    }

    this.#registerWatcher(callback)
    const response = await this.#peer.call<TriggerWatch>(
      "triggers.resume",
      definedParams({
        name: this.#name,
        resume_token: resumeToken,
        resume_window_ms: options.resumeWindowMs,
      }),
    )
    this.#resumeToken = response.resume_token
    return response.resume_token
  }

  async unwatch(): Promise<void> {
    if (this.#resumeToken === undefined) {
      return
    }

    await this.#peer.call<void>("triggers.unwatch", {
      resume_token: this.#resumeToken,
    })
    this.#resumeToken = undefined
  }

  #registerWatcher(callback: TriggerCallback): void {
    this.#peer.addMethod(this.#name, async params => {
      const deltas = params.deltas as TriggerDelta[]
      for (const delta of deltas) {
        await callback(delta.added, delta.deleted)
      }
    })
  }

  #replace(definition: TriggerDefinition): void {
    this.#name = definition.name
    this.#prompt = definition.prompt
    this.#sql = definition.sql
    this.#signature = definition.signature
    this.#options = {
      schedule: definition.schedule,
      batchSize: definition.batch_size,
      timeoutMs: definition.timeout_ms,
      reportNoops: definition.report_noops,
      enabled: definition.enabled,
    }
  }
}

function triggerParams(values: TriggerUpdate & { name: string }): Record<string, unknown> {
  return definedParams({
    name: values.name,
    sql: values.sql,
    prompt: values.prompt,
    schedule: values.schedule,
    batch_size: values.batchSize,
    timeout_ms: values.timeoutMs,
    report_noops: values.reportNoops,
    enabled: values.enabled,
  })
}
