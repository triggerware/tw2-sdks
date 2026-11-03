import type { Peer } from "tw2-jsonrpc"

export type LogLevel = "debug" | "info" | "error"

export class LoggingService {
  readonly #peer: Peer

  constructor(peer: Peer) {
    this.#peer = peer
  }

  set(level: LogLevel): Promise<void> {
    return this.#peer.call<void>("logging.set", { level })
  }
}
