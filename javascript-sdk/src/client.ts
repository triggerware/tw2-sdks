import { connectTcp as rpcConnectTcp, connectWs as rpcConnectWs, type Peer } from "tw2-jsonrpc"
import { ConfigurationService } from "./configuration.js"
import { ConnectorsService } from "./connectors.js"
import { CursorsService } from "./cursors.js"
import { ExtensionsService } from "./extensions.js"
import { AgentsService } from "./agents.js"
import { QueriesService } from "./queries.js"
import { RuntimesService } from "./runtimes.js"
import { SystemService } from "./system.js"
import { TriggersService } from "./triggers.js"
import { splitAddress } from "./utils.js"

export class TriggerwareClient {
  readonly configuration: ConfigurationService
  readonly connectors: ConnectorsService
  readonly cursors: CursorsService
  readonly extensions: ExtensionsService
  readonly agents: AgentsService
  readonly queries: QueriesService
  readonly runtimes: RuntimesService
  readonly system: SystemService
  readonly triggers: TriggersService

  readonly #peer: Peer

  constructor(peer: Peer) {
    this.#peer = peer
    this.configuration = new ConfigurationService(peer)
    this.connectors = new ConnectorsService(peer)
    this.cursors = new CursorsService(peer)
    this.extensions = new ExtensionsService(peer)
    this.agents = new AgentsService(peer)
    this.queries = new QueriesService(peer)
    this.runtimes = new RuntimesService(peer)
    this.system = new SystemService(peer)
    this.triggers = new TriggersService(peer)
  }

  static async connectTcp(address: string): Promise<TriggerwareClient> {
    const [host, port] = splitAddress(address)
    return new TriggerwareClient(await rpcConnectTcp(host, port))
  }

  static async connectWs(url: string, apiKey: string): Promise<TriggerwareClient> {
    const client = new TriggerwareClient(await rpcConnectWs(url))
    await client.system.authorize(apiKey)
    return client
  }

  get peer(): Peer {
    return this.#peer
  }

  close(): Promise<void> {
    return this.#peer.close()
  }

  waitClosed(): Promise<void> {
    return this.#peer.waitClosed()
  }

  async [Symbol.asyncDispose](): Promise<void> {
    await this.close()
  }
}
