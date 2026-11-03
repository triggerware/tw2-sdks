import type { Peer } from "tw2-jsonrpc"
import type { AuthorizationResult, JsonObject } from "./types.js"
import { definedParams } from "./utils.js"

export type ConnectorRuntime = "python" | "node"

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
  oauthProvider?: string
  runtime?: ConnectorRuntime
  dependencies?: string[]
  tables: string[]
  tools: string[]
  agents: string[]
}

interface ConnectorDescriptionWire {
  description: string
  config_schema: JsonObject
  oauth_provider?: string
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
  secure?: boolean
}

export interface ConnectorCatalogEntry {
  name: string
  description: string
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

  install(install: ConnectorInstall): Promise<AuthorizationResult> {
    return this.#peer.call<AuthorizationResult>(
      "connectors.install",
      connectorParams(install),
    )
  }

  async get(
    name: string | string[],
    options: { tag?: string } = {},
  ): Promise<Connector | Connector[]> {
    if (Array.isArray(name)) {
      const response = await this.#peer.call<{ connectors: ConnectorWire[] }>(
        "connectors.get",
        definedParams({ name, tag: options.tag }),
      )
      return response.connectors.map((connector, index) =>
        connectorFromWire(name[index]!, options.tag, connector),
      )
    }
    const response = await this.#peer.call<ConnectorWire>(
      "connectors.get",
      connectorParams({ name, tag: options.tag }),
    )
    return connectorFromWire(name, options.tag, response)
  }

  async list(): Promise<string[]> {
    const response = await this.#peer.call<ConnectorList>("connectors.list")
    return response.connectors
  }

  configure(
    name: string,
    options: { config?: JsonObject; tag?: string; secure?: boolean } = {},
  ): Promise<AuthorizationResult> {
    return this.#peer.call<AuthorizationResult>(
      "connectors.configure",
      connectorParams({ name, ...options }),
    )
  }

  async authorize(name: string, options: { tag?: string } = {}): Promise<string> {
    const response = await this.#peer.call<{ url: string }>(
      "connectors.authorize",
      connectorParams({ name, tag: options.tag }),
    )
    return response.url
  }

  async catalog(): Promise<ConnectorCatalogEntry[]> {
    const response = await this.#peer.call<{ connectors: ConnectorCatalogEntry[] }>("connectors.catalog")
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
      oauthProvider: response.oauth_provider,
      runtime: response.runtime,
      dependencies: response.dependencies,
      tables: response.tables,
      tools: response.tools,
      agents: response.agents,
    }
  }

  uninstall(name: string | string[], options: { tag?: string } = {}): Promise<void> {
    return this.#peer.call<void>(
      "connectors.uninstall",
      definedParams({ name, tag: options.tag }),
    )
  }
}

function connectorParams(values: ConnectorInstall | ConnectorReference): JsonObject {
  return definedParams(values as JsonObject)
}

function connectorFromWire(name: string, tag: string | undefined, response: ConnectorWire): Connector {
  return {
    name,
    tag,
    config: response.config,
    tables: response.tables,
    tools: response.tools,
    agents: response.agents,
  }
}
