import type { Peer } from "tw2-jsonrpc"
import type { Health } from "./types.js"

interface AuthorizationStatus {
  ok: boolean
}

export interface SystemUpdate {
  runtimes: string[]
  connector: string
  artifact: string
  restartRequired: boolean
}

interface SystemUpdateWire {
  runtimes: string[]
  connector: string
  artifact: string
  restart_required: boolean
}

export class SystemService {
  readonly #peer: Peer

  constructor(peer: Peer) {
    this.#peer = peer
  }

  health(): Promise<Health> {
    return this.#peer.call<Health>("system.health")
  }

  async update(): Promise<SystemUpdate> {
    const response = await this.#peer.call<SystemUpdateWire>("system.update")
    return {
      runtimes: response.runtimes,
      connector: response.connector,
      artifact: response.artifact,
      restartRequired: response.restart_required,
    }
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
