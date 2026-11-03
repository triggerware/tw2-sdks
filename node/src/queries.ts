import type { Peer } from "tw2-jsonrpc"
import type { Row, RunOptions } from "./types.js"
import { definedParams } from "./utils.js"

export interface QuerySignature {
  name: string
  type: string
}

interface QueryDefinition {
  name: string
  description: string
  params?: QuerySignature[]
  sql?: string
  prompt?: string
  batch_size?: number
  timeout_ms?: number
  signature: QuerySignature[]
}

interface QueryList {
  queries: string[]
}

export interface QueryFetch {
  rows: Row[]
  exhausted: boolean
}

export interface QueryOptions extends RunOptions {
  description?: string
  params?: QuerySignature[]
}

export type QueryCreation =
  | (QueryOptions & { name: string; sql: string; description: string; prompt?: never })
  | (QueryOptions & { name?: string; prompt: string; sql?: string })

export interface QueryParameterValue {
  name: string
  value: unknown
}

export interface QueryFetchOptions {
  params?: QueryParameterValue[]
  rows?: number
}

export interface QueryConversion extends RunOptions {
  name: string
  triggerName?: string
  prompt?: string
  params?: QueryParameterValue[]
  schedule?: string
  reportNoops?: boolean
  enabled?: boolean
}

export class QueriesService {
  readonly #peer: Peer

  constructor(peer: Peer) {
    this.#peer = peer
  }

  async create(creation: QueryCreation): Promise<Query> {
    const definition = await this.#peer.call<QueryDefinition>(
      "queries.create",
      queryParams(creation),
    )
    return new Query(this.#peer, definition)
  }

  convert(conversion: QueryConversion): Promise<QueryFetch> {
    return this.#peer.call<QueryFetch>(
      "queries.convert",
      definedParams({
        name: conversion.name,
        trigger_name: conversion.triggerName,
        prompt: conversion.prompt,
        params: conversion.params,
        schedule: conversion.schedule,
        batch_size: conversion.batchSize,
        timeout_ms: conversion.timeoutMs,
        report_noops: conversion.reportNoops,
        enabled: conversion.enabled,
      }),
    )
  }

  async get(name: string | string[]): Promise<Query | Query[]> {
    if (typeof name === "string") {
      const definition = await this.#peer.call<QueryDefinition>("queries.get", { name })
      return new Query(this.#peer, definition)
    }
    const response = await this.#peer.call<{ queries: QueryDefinition[] }>("queries.get", { name })
    return response.queries.map(definition => new Query(this.#peer, definition))
  }

  async list(): Promise<string[]> {
    const response = await this.#peer.call<QueryList>("queries.list")
    return response.queries
  }

  delete(name: string | string[]): Promise<void> {
    return this.#peer.call<void>("queries.delete", { name })
  }
}

export class Query implements AsyncIterable<Row> {
  readonly #peer: Peer
  #name!: string
  #description!: string
  #params?: QuerySignature[]
  #fetchOptions: QueryFetchOptions = {}
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

  get description(): string {
    return this.#description
  }

  get params(): readonly QuerySignature[] | undefined {
    return this.#params
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

  async reset(): Promise<void> {
    // The resource documentation's prose specifies the current query definition.
    const definition = await this.#peer.call<QueryDefinition>("queries.reset", {
      name: this.#name,
    })
    this.#replace(definition)
  }

  async fetch(options: QueryFetchOptions = this.#fetchOptions): Promise<Row[]> {
    if (this.#exhausted) {
      return []
    }

    const response = await this.#peer.call<QueryFetch>(
      "queries.fetch",
      definedParams({ name: this.#name, params: options.params, rows: options.rows }),
    )
    this.#fetchOptions = { ...options }
    this.#exhausted = response.exhausted
    this.#cache.push(...response.rows)
    return response.rows
  }

  async next(options: QueryFetchOptions = this.#fetchOptions): Promise<Row | null> {
    if (this.#cachePosition < this.#cache.length) {
      return this.#cache[this.#cachePosition++]!
    }

    if (this.#exhausted) {
      return null
    }

    await this.fetch(options)

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
    this.#description = definition.description
    this.#params = definition.params
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

function queryParams(
  values: QueryOptions & { name?: string; sql?: string; prompt?: string },
): Record<string, unknown> {
  return definedParams({
    name: values.name,
    description: values.description,
    params: values.params,
    sql: values.sql,
    prompt: values.prompt,
    batch_size: values.batchSize,
    timeout_ms: values.timeoutMs,
  })
}
