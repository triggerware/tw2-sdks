import type { Peer } from "tw2-jsonrpc"

interface PromptResponse {
  prompt: string
}

export class PromptsService {
  readonly #peer: Peer

  constructor(peer: Peer) {
    this.#peer = peer
  }

  async get(name: string): Promise<string> {
    const response = await this.#peer.call<PromptResponse>("prompts.get", { name })
    return response.prompt
  }

  set(name: string, prompt: string): Promise<void> {
    return this.#peer.call<void>("prompts.set", { name, prompt })
  }

  reset(name?: string): Promise<void> {
    return this.#peer.call<void>("prompts.reset", name === undefined ? {} : { name })
  }
}
