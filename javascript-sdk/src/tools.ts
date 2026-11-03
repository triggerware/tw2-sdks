import type { Peer } from "tw2-jsonrpc"
import YAML from "yaml"
import type { AuthorizationResult, JsonObject, NamedColumn, Runtime } from "./types.js"
import { definedParams } from "./utils.js"

export interface ToolMetadata {
  description: string
  signature: NamedColumn[]
  configSchema?: JsonObject
}

interface ToolMetadataWire {
  description: string
  signature: NamedColumn[]
  config_schema?: JsonObject
}

export interface ToolConnector {
  name: string
  tag?: string
}

export interface Tool {
  name: string
  prompt?: string
  meta: ToolMetadata
  code: string
  runtime: Runtime
  connector?: ToolConnector
  configured: boolean
}

interface ToolWire {
  name: string
  prompt?: string
  meta: ToolMetadataWire
  code: string
  runtime: Runtime
  connector?: ToolConnector
  configured: boolean
}

export interface ToolUpdate {
  prompt?: string
  code?: string
  meta?: ToolMetadata | string
  runtime?: Runtime
}

export type ToolCreation =
  | (ToolUpdate & { name?: string; prompt: string })
  | (ToolUpdate & { name: string; code: string; meta: ToolMetadata | string })

export interface ToolsSyncOptions {
  tools?: string[]
  connectors?: string[]
}

export class ToolsService {
  readonly #peer: Peer

  constructor(peer: Peer) {
    this.#peer = peer
  }

  async create(creation: ToolCreation): Promise<Tool> {
    return toolFromWire(await this.#peer.call<ToolWire>(
      "tools.create",
      toolParams(creation.name, creation),
    ))
  }

  async get(name: string | string[]): Promise<Tool | Tool[]> {
    if (typeof name === "string") {
      return toolFromWire(await this.#peer.call<ToolWire>("tools.get", { name }))
    }
    const response = await this.#peer.call<{ tools: ToolWire[] }>("tools.get", { name })
    return response.tools.map(toolFromWire)
  }

  async update(name: string, update: ToolUpdate): Promise<Tool> {
    return toolFromWire(await this.#peer.call<ToolWire>(
      "tools.update",
      toolParams(name, update),
    ))
  }

  async list(): Promise<string[]> {
    const response = await this.#peer.call<{ tools: string[] }>("tools.list")
    return response.tools
  }

  delete(name: string | string[]): Promise<void> {
    return this.#peer.call<void>("tools.delete", { name })
  }

  sync(options: ToolsSyncOptions = {}): Promise<void> {
    return this.#peer.call<void>("tools.sync", definedParams(options))
  }

  configure(
    name: string,
    options: { config?: JsonObject; secure?: boolean } = {},
  ): Promise<AuthorizationResult> {
    return this.#peer.call<AuthorizationResult>("tools.configure", definedParams({ name, ...options }))
  }
}

function toolParams(name: string | undefined, definition: ToolUpdate): JsonObject {
  return definedParams({
    name,
    prompt: definition.prompt,
    code: definition.code,
    meta: definition.meta === undefined ? undefined : metadataToWire(definition.meta),
    runtime: definition.runtime,
  })
}

export function toolMetadataFromString(contents: string): ToolMetadata {
  const parsed: unknown = YAML.parse(contents)
  if (typeof parsed !== "object" || parsed === null || Array.isArray(parsed)) {
    throw new Error("tool metadata must be an object")
  }
  return metadataFromWire(parsed as ToolMetadataWire)
}

function metadataToWire(meta: ToolMetadata | string): ToolMetadataWire {
  if (typeof meta === "string") {
    return metadataToWire(toolMetadataFromString(meta))
  }
  return definedParams({
    description: meta.description,
    signature: meta.signature,
    config_schema: meta.configSchema,
  }) as unknown as ToolMetadataWire
}

function metadataFromWire(meta: ToolMetadataWire): ToolMetadata {
  return definedParams({
    description: meta.description,
    signature: meta.signature,
    configSchema: meta.config_schema,
  }) as unknown as ToolMetadata
}

function toolFromWire(tool: ToolWire): Tool {
  return { ...tool, meta: metadataFromWire(tool.meta) }
}
