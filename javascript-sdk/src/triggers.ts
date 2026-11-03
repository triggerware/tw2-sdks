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

export interface TriggerUpdate extends RunOptions {
  description?: string
  prompt?: string
  sql?: string
  schedule?: string
  reportNoops?: boolean
  enabled?: boolean
}

export type TriggerCreation =
  | (TriggerUpdate & { name?: string; prompt: string })
  | (TriggerUpdate & { name: string; description: string; sql: string; schedule: string })

export interface Trigger extends RunOptions {
  name: string
  description: string
  prompt?: string
  sql: string
  schedule?: string
  reportNoops: boolean
  enabled: boolean
  signature: TriggerSignature[]
}

interface TriggerWire {
  name: string
  description: string
  prompt?: string
  sql: string
  schedule?: string
  batch_size?: number
  timeout_ms?: number
  report_noops: boolean
  enabled: boolean
  signature: TriggerSignature[]
}

export type TriggerCallback = (
  added: Row[],
  deleted: Row[],
) => void | Promise<void>

export class TriggersService {
  readonly #peer: Peer

  constructor(peer: Peer) {
    this.#peer = peer
  }

  async create(creation: TriggerCreation): Promise<Trigger> {
    return triggerFromWire(await this.#peer.call<TriggerWire>(
      "triggers.create",
      triggerParams(creation.name, creation),
    ))
  }

  async get(name: string | string[]): Promise<Trigger | Trigger[]> {
    if (typeof name === "string") {
      return triggerFromWire(await this.#peer.call<TriggerWire>("triggers.get", { name }))
    }
    const response = await this.#peer.call<{ triggers: TriggerWire[] }>("triggers.get", { name })
    return response.triggers.map(triggerFromWire)
  }

  async update(name: string, update: TriggerUpdate): Promise<Trigger> {
    return triggerFromWire(await this.#peer.call<TriggerWire>(
      "triggers.update",
      triggerParams(name, update),
    ))
  }

  async list(): Promise<string[]> {
    const response = await this.#peer.call<{ triggers: string[] }>("triggers.list")
    return response.triggers
  }

  delete(name: string | string[]): Promise<void> {
    return this.#peer.call<void>("triggers.delete", { name })
  }

  enable(name: string): Promise<void> {
    return this.#peer.call<void>("triggers.enable", { name })
  }

  disable(name: string): Promise<void> {
    return this.#peer.call<void>("triggers.disable", { name })
  }

  run(name: string, options: RunOptions = {}): Promise<TriggerDelta> {
    return this.#peer.call<TriggerDelta>("triggers.run", withRunOptions({ name }, options))
  }

  async poll(name: string): Promise<TriggerDelta[]> {
    const response = await this.#peer.call<{ deltas: TriggerDelta[] }>("triggers.poll", { name })
    return response.deltas
  }

  async watch(
    name: string,
    callback: TriggerCallback,
    options: { resumeWindowMs?: number } = {},
  ): Promise<string> {
    this.#registerWatcher(name, callback)
    const response = await this.#peer.call<{ resume_token: string }>(
      "triggers.watch",
      definedParams({ name, resume_window_ms: options.resumeWindowMs }),
    )
    return response.resume_token
  }

  async resume(
    name: string,
    resumeToken: string,
    callback: TriggerCallback,
    options: { resumeWindowMs?: number } = {},
  ): Promise<string> {
    this.#registerWatcher(name, callback)
    const response = await this.#peer.call<{ resume_token: string }>(
      "triggers.resume",
      definedParams({ name, resume_token: resumeToken, resume_window_ms: options.resumeWindowMs }),
    )
    return response.resume_token
  }

  unwatch(options: { name?: string; resumeToken?: string }): Promise<void> {
    return this.#peer.call<void>("triggers.unwatch", definedParams({
      name: options.name,
      resume_token: options.resumeToken,
    }))
  }

  #registerWatcher(name: string, callback: TriggerCallback): void {
    this.#peer.addMethod(name, async params => {
      for (const delta of params.deltas as TriggerDelta[]) {
        await callback(delta.added, delta.deleted)
      }
    })
  }
}

function triggerParams(name: string | undefined, values: TriggerUpdate): Record<string, unknown> {
  return definedParams({
    name,
    description: values.description,
    sql: values.sql,
    prompt: values.prompt,
    schedule: values.schedule,
    batch_size: values.batchSize,
    timeout_ms: values.timeoutMs,
    report_noops: values.reportNoops,
    enabled: values.enabled,
  })
}

function triggerFromWire(trigger: TriggerWire): Trigger {
  return {
    name: trigger.name,
    description: trigger.description,
    prompt: trigger.prompt,
    sql: trigger.sql,
    schedule: trigger.schedule,
    batchSize: trigger.batch_size,
    timeoutMs: trigger.timeout_ms,
    reportNoops: trigger.report_noops,
    enabled: trigger.enabled,
    signature: trigger.signature,
  }
}
