import type { Peer } from "tw2-jsonrpc"

export type ExternalRuntime = "python" | "node"

interface RuntimeList {
  runtimes: string[]
}

export class RuntimesService {
  readonly #peer: Peer

  constructor(peer: Peer) {
    this.#peer = peer
  }

  async list(): Promise<string[]> {
    const response = await this.#peer.call<RuntimeList>("runtimes.list")
    return response.runtimes
  }

  install(name: ExternalRuntime): Promise<void> {
    return this.#peer.call<void>("runtimes.install", { name })
  }

  uninstall(name: ExternalRuntime): Promise<void> {
    return this.#peer.call<void>("runtimes.uninstall", { name })
  }
}
