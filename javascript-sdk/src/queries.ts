import type { Peer } from "npmsonrpc"
import type { Row, RunOptions } from "./types.js"
import { definedParams, withRunOptions } from "./utils.js"

export interface QuerySignature {
  name: string
  type: string
}

interface QueryDefinition {
  name: string
  sql?: string
  prompt?: string
  batch_size?: number
  timeout_ms?: number
  signature: QuerySignature[]
}

interface QueryList {
  queries: string[]
}

interface QueryFetch {
  rows: Row[]
  exhausted: boolean
}

export type QueryCreation =
  | ({ sql: string; prompt?: never } & RunOptions)
  | ({ prompt: string; sql?: never } & RunOptions)

export type QueryUpdate = Partial<{
  sql: string
  prompt: string
  batchSize: number
  timeoutMs: number
}>

export class QueriesService {
  readonly #peer: Peer

  constructor(peer: Peer) {
    this.#peer = peer
  }

  async create(name: string, creation: QueryCreation): Promise<Query> {
    const definition = await this.#peer.call<QueryDefinition>(
      "queries.create",
      queryParams(name, creation),
    )
    return new Query(this.#peer, definition)
  }

  async get(name: string): Promise<Query> {
    const definition = await this.#peer.call<QueryDefinition>("queries.get", { name })
    return new Query(this.#peer, definition)
  }

  async list(): Promise<string[]> {
    const response = await this.#peer.call<QueryList>("queries.list")
    return response.queries
  }
}

export class Query implements AsyncIterable<Row> {
  readonly #peer: Peer
  #name!: string
  #sql?: string
  #prompt?: string
  #batchSize?: number
  #timeoutMs?: number
  #signature!: QuerySignature[]
  #exhausted = false
  #cache: Row[] = []
  #cachePosition = 0

  constructor(peer: Peer, definition: QueryDefinition) {
    this.#peer = peer
    this.#replace(definition)
  }

  get name(): string {
    return this.#name
  }

  get sql(): string | undefined {
    return this.#sql
  }

  get prompt(): string | undefined {
    return this.#prompt
  }

  get batchSize(): number | undefined {
    return this.#batchSize
  }

  get timeoutMs(): number | undefined {
    return this.#timeoutMs
  }

  get signature(): readonly QuerySignature[] {
    return this.#signature
  }

  get exhausted(): boolean {
    return this.#exhausted
  }

  async update(update: QueryUpdate = {}): Promise<void> {
    const definition = await this.#peer.call<QueryDefinition>(
      "queries.update",
      queryParams(this.#name, {
        sql: update.sql,
        prompt: update.prompt,
        batchSize: update.batchSize,
        timeoutMs: update.timeoutMs,
      }),
    )
    this.#replace(definition)
  }

  async reset(): Promise<void> {
    const definition = await this.#peer.call<QueryDefinition>("queries.reset", {
      name: this.#name,
    })
    this.#replace(definition)
  }

  delete(): Promise<void> {
    return this.#peer.call<void>("queries.delete", { name: this.#name })
  }

  async fetch(): Promise<Row[]> {
    if (this.#exhausted) {
      return []
    }

    const response = await this.#peer.call<QueryFetch>(
      "queries.fetch",
      withRunOptions(
        { name: this.#name },
        { batchSize: this.#batchSize, timeoutMs: this.#timeoutMs },
      ),
    )
    this.#exhausted = response.exhausted
    this.#cache.push(...response.rows)
    return response.rows
  }

  async next(): Promise<Row | null> {
    if (this.#cachePosition < this.#cache.length) {
      return this.#cache[this.#cachePosition++]!
    }

    if (this.#exhausted) {
      return null
    }

    await this.fetch()

    if (this.#cachePosition < this.#cache.length) {
      return this.#cache[this.#cachePosition++]!
    }

    return null
  }

  async *[Symbol.asyncIterator](): AsyncIterator<Row> {
    for (;;) {
      const row = await this.next()
      if (row === null) {
        return
      }
      yield row
    }
  }

  #replace(definition: QueryDefinition): void {
    this.#name = definition.name
    this.#sql = definition.sql
    this.#prompt = definition.prompt
    this.#batchSize = definition.batch_size
    this.#timeoutMs = definition.timeout_ms
    this.#signature = definition.signature
    this.#exhausted = false
    this.#cache = []
    this.#cachePosition = 0
  }
}

function queryParams(name: string, values: QueryUpdate): Record<string, unknown> {
  return definedParams({
    name,
    sql: values.sql,
    prompt: values.prompt,
    batch_size: values.batchSize,
    timeout_ms: values.timeoutMs,
  })
}
