import type { Peer } from "tw2-jsonrpc"
import type { JsonObject, Runtime } from "./types.js"
import { definedParams } from "./utils.js"

export type ConnectorRuntime = Runtime

export interface Connector {
  readonly name: string
  readonly tag?: string
  readonly config: JsonObject | null
  readonly tables: string[]
  readonly tools: string[]
  readonly agents: string[]
}

export interface ConnectorDescription {
  description: string
  configSchema: JsonObject
  runtime?: ConnectorRuntime
  dependencies?: string[]
  tables: string[]
  tools: string[]
  agents: string[]
}

interface ConnectorDescriptionWire {
  description: string
  config_schema: JsonObject
  runtime?: ConnectorRuntime
  dependencies?: string[]
  tables: string[]
  tools: string[]
  agents: string[]
}

export interface ConnectorInstall {
  name?: string
  url?: string
  tag?: string
  config?: JsonObject
}

export interface ConnectorReference {
  name?: string
  url?: string
}

interface ConnectorList {
  connectors: string[]
}

interface ConnectorWire {
  config: JsonObject | null
  tables: string[]
  tools: string[]
  agents: string[]
}

export class ConnectorsService {
  readonly #peer: Peer

  constructor(peer: Peer) {
    this.#peer = peer
  }

  install(install: ConnectorInstall): Promise<void> {
    return this.#peer.call<void>(
      "connectors.install",
      connectorParams({ ...install, config: install.config ?? {} }),
    )
  }

  async get(name: string, options: { tag?: string } = {}): Promise<Connector> {
    const response = await this.#peer.call<ConnectorWire>(
      "connectors.get",
      connectorParams({ name, tag: options.tag }),
    )
    return {
      name,
      tag: options.tag,
      config: response.config,
      tables: response.tables,
      tools: response.tools,
      agents: response.agents,
    }
  }

  async list(): Promise<string[]> {
    const response = await this.#peer.call<ConnectorList>("connectors.list")
    return response.connectors
  }

  configure(name: string, config: JsonObject, options: { tag?: string } = {}): Promise<void> {
    return this.#peer.call<void>(
      "connectors.configure",
      connectorParams({ name, tag: options.tag, config }),
    )
  }

  async catalog(): Promise<string[]> {
    const response = await this.#peer.call<ConnectorList>("connectors.catalog")
    return response.connectors
  }

  async describe(reference: ConnectorReference): Promise<ConnectorDescription> {
    const response = await this.#peer.call<ConnectorDescriptionWire>(
      "connectors.describe",
      connectorParams(reference),
    )
    return {
      description: response.description,
      configSchema: response.config_schema,
      runtime: response.runtime,
      dependencies: response.dependencies,
      tables: response.tables,
      tools: response.tools,
      agents: response.agents,
    }
  }

  uninstall(name: string, options: { tag?: string } = {}): Promise<void> {
    return this.#peer.call<void>(
      "connectors.uninstall",
      connectorParams({ name, tag: options.tag }),
    )
  }
}

function connectorParams(values: ConnectorInstall | ConnectorReference): JsonObject {
  return definedParams(values as JsonObject)
}
