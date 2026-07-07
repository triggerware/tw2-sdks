import type { Peer } from "tw2-jsonrpc"
import type { Health } from "./types.js"

interface AuthorizationStatus {
  ok: boolean
}

export class SystemService {
  readonly #peer: Peer

  constructor(peer: Peer) {
    this.#peer = peer
  }

  health(): Promise<Health> {
    return this.#peer.call<Health>("system.health")
  }

  async authorize(apiKey: string): Promise<boolean> {
    const response = await this.#peer.call<AuthorizationStatus>("system.authorize", {
      api_key: apiKey,
    })
    return response.ok
  }

  async authorized(): Promise<boolean> {
    const response = await this.#peer.call<AuthorizationStatus>("system.authorized")
    return response.ok
  }
}
