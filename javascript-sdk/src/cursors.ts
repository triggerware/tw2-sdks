import type { Peer } from "npmsonrpc"
import type { Row, RunOptions } from "./types.js"
import { definedParams, withRunOptions } from "./utils.js"

export interface CursorColumn {
  name: string
  type: string
}

interface CursorOpen {
  sql: string
  signature: CursorColumn[]
  rows: Row[]
  cursor: string
  exhausted: boolean
}

interface CursorFetch {
  rows: Row[]
  exhausted: boolean
}

export type CursorOpenOptions =
  | ({ sql: string; prompt?: never } & RunOptions)
  | ({ prompt: string; sql?: never } & RunOptions)

export class CursorsService {
  readonly #peer: Peer

  constructor(peer: Peer) {
    this.#peer = peer
  }

  async open(options: CursorOpenOptions): Promise<Cursor> {
    const response = await this.#peer.call<CursorOpen>(
      "cursors.open",
      definedParams({
        sql: options.sql,
        prompt: options.prompt,
        batch_size: options.batchSize,
        timeout_ms: options.timeoutMs,
      }),
    )
    return new Cursor(this.#peer, response, options)
  }
}

export class Cursor implements AsyncIterable<Row> {
  readonly #peer: Peer
  readonly #cursor: string
  readonly #options: RunOptions
  readonly #sql: string
  readonly #signature: CursorColumn[]
  #exhausted: boolean
  #cache: Row[]
  #cachePosition = 0

  constructor(peer: Peer, open: CursorOpen, options: RunOptions = {}) {
    this.#peer = peer
    this.#cursor = open.cursor
    this.#options = options
    this.#sql = open.sql
    this.#signature = open.signature
    this.#exhausted = open.exhausted
    this.#cache = open.rows
  }

  get sql(): string {
    return this.#sql
  }

  get signature(): readonly CursorColumn[] {
    return this.#signature
  }

  get exhausted(): boolean {
    return this.#exhausted
  }

  async fetch(): Promise<Row[]> {
    if (this.#exhausted) {
      return []
    }

    const response = await this.#peer.call<CursorFetch>(
      "cursors.fetch",
      withRunOptions({ cursor: this.#cursor }, this.#options),
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

  async close(): Promise<void> {
    await this.#peer.call<void>("cursors.close", { cursor: this.#cursor })
    this.#exhausted = true
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
}
