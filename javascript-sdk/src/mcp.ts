import type { Peer } from "tw2-jsonrpc"
import type { JsonObject } from "./types.js"
import { definedParams } from "./utils.js"

export class McpService {
  readonly #peer: Peer

  constructor(peer: Peer) {
    this.#peer = peer
  }

  call<T = unknown>(method: string, params?: JsonObject): Promise<T> {
    return this.#peer.call<T>("mcp", definedParams({ method, params }))
  }
}
