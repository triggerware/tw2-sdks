import { connectTcp as rpcConnectTcp, connectWs as rpcConnectWs, type Peer } from "tw2-jsonrpc"
import { ConfigurationService } from "./configuration.js"
import { TablesService } from "./tables.js"
import { CursorsService } from "./cursors.js"
import { ConnectorsService } from "./connectors.js"
import { AgentsService } from "./agents.js"
import { ToolsService } from "./tools.js"
import { LoggingService } from "./logging.js"
import { McpService } from "./mcp.js"
import { QueriesService } from "./queries.js"
import { RuntimesService } from "./runtimes.js"
import { SystemService } from "./system.js"
import { TriggersService } from "./triggers.js"
import { splitAddress } from "./utils.js"

export class TriggerwareClient {
  readonly configuration: ConfigurationService
  readonly tables: TablesService
  readonly cursors: CursorsService
  readonly connectors: ConnectorsService
  readonly agents: AgentsService
  readonly tools: ToolsService
  readonly logging: LoggingService
  readonly mcp: McpService
  readonly queries: QueriesService
  readonly runtimes: RuntimesService
  readonly system: SystemService
  readonly triggers: TriggersService

  readonly #peer: Peer

  constructor(peer: Peer) {
    this.#peer = peer
    this.configuration = new ConfigurationService(peer)
    this.tables = new TablesService(peer)
    this.cursors = new CursorsService(peer)
    this.connectors = new ConnectorsService(peer)
    this.agents = new AgentsService(peer)
    this.tools = new ToolsService(peer)
    this.logging = new LoggingService(peer)
    this.mcp = new McpService(peer)
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
