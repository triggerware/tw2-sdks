import type { Peer } from "tw2-jsonrpc"
import type { JsonObject, Runtime } from "./types.js"
import { definedParams } from "./utils.js"

export type ExtensionRuntime = Runtime

export interface Extension {
  readonly name: string
  readonly tag?: string
  readonly config: JsonObject | null
  readonly connectors: string[]
  readonly tools: string[]
  readonly agents: string[]
}

export interface ExtensionDescription {
  description: string
  configSchema: JsonObject
  runtime?: ExtensionRuntime
  dependencies?: string[]
  connectors: string[]
  tools: string[]
  agents: string[]
}

interface ExtensionDescriptionWire {
  description: string
  config_schema: JsonObject
  runtime?: ExtensionRuntime
  dependencies?: string[]
  connectors: string[]
  tools: string[]
  agents: string[]
}

export interface ExtensionInstall {
  name?: string
  url?: string
  tag?: string
  config?: JsonObject
}

export interface ExtensionReference {
  name?: string
  url?: string
}

interface ExtensionList {
  extensions: string[]
}

interface ExtensionWire {
  config: JsonObject | null
  connectors: string[]
  tools: string[]
  agents: string[]
}

export class ExtensionsService {
  readonly #peer: Peer

  constructor(peer: Peer) {
    this.#peer = peer
  }

  install(install: ExtensionInstall): Promise<void> {
    return this.#peer.call<void>(
      "extensions.install",
      extensionParams({ ...install, config: install.config ?? {} }),
    )
  }

  async get(name: string, options: { tag?: string } = {}): Promise<Extension> {
    const response = await this.#peer.call<ExtensionWire>(
      "extensions.get",
      extensionParams({ name, tag: options.tag }),
    )
    return {
      name,
      tag: options.tag,
      config: response.config,
      connectors: response.connectors,
      tools: response.tools,
      agents: response.agents,
    }
  }

  async list(): Promise<string[]> {
    const response = await this.#peer.call<ExtensionList>("extensions.list")
    return response.extensions
  }

  configure(name: string, config: JsonObject, options: { tag?: string } = {}): Promise<void> {
    return this.#peer.call<void>(
      "extensions.configure",
      extensionParams({ name, tag: options.tag, config }),
    )
  }

  async catalog(): Promise<string[]> {
    const response = await this.#peer.call<ExtensionList>("extensions.catalog")
    return response.extensions
  }

  async describe(reference: ExtensionReference): Promise<ExtensionDescription> {
    const response = await this.#peer.call<ExtensionDescriptionWire>(
      "extensions.describe",
      extensionParams(reference),
    )
    return {
      description: response.description,
      configSchema: response.config_schema,
      runtime: response.runtime,
      dependencies: response.dependencies,
      connectors: response.connectors,
      tools: response.tools,
      agents: response.agents,
    }
  }

  uninstall(name: string, options: { tag?: string } = {}): Promise<void> {
    return this.#peer.call<void>(
      "extensions.uninstall",
      extensionParams({ name, tag: options.tag }),
    )
  }
}

function extensionParams(values: ExtensionInstall | ExtensionReference): JsonObject {
  return definedParams(values as JsonObject)
}
