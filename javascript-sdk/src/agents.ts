import type { Peer } from "tw2-jsonrpc"

export class AgentsService {
  readonly #peer: Peer

  constructor(peer: Peer) {
    this.#peer = peer
  }

  async run(name: string, data: Record<string, unknown>): Promise<void> {
    await this.#peer.call("agents.run", { name, data })
  }
}
