import type { Peer } from "npmsonrpc"
import YAML from "yaml"
import type { JsonObject, Runtime } from "./types.js"
import { definedParams } from "./utils.js"

export type ConnectorRuntime = Runtime

export interface ConnectorColumn {
  name: string
  type: string
}

export interface ConnectorGenerator {
  name: string
  inputs: string[]
}

export interface ConnectorMetadata {
  description?: string
  columns: ConnectorColumn[]
  generators: ConnectorGenerator[]
  configSchema?: JsonObject
}

interface ConnectorMetadataWire {
  description?: string
  columns: ConnectorColumn[]
  generators: ConnectorGenerator[]
  config_schema?: JsonObject
}

export interface ConnectorExtension {
  name: string
  tag?: string
}

interface ConnectorWire {
  meta: ConnectorMetadataWire
  code: string
  runtime?: ConnectorRuntime
  extension?: ConnectorExtension
  configured: boolean
}

export type ConnectorDefinition =
  | { prompt: string; runtime?: ConnectorRuntime }
  | { code: string; meta: ConnectorMetadata | string; runtime?: ConnectorRuntime }

interface ConnectorList {
  connectors: string[]
}

export interface ConnectorsSyncOptions {
  connectors?: string[]
  extensions?: string[]
}

export class ConnectorsService {
  readonly #peer: Peer

  constructor(peer: Peer) {
    this.#peer = peer
  }

  async create(name: string, definition: ConnectorDefinition): Promise<Connector> {
    const wire = await this.#peer.call<ConnectorWire>(
      "connectors.create",
      connectorParams(name, definition),
    )
    return new Connector(this.#peer, name, wire)
  }

  async get(name: string): Promise<Connector> {
    const wire = await this.#peer.call<ConnectorWire>("connectors.get", { name })
    return new Connector(this.#peer, name, wire)
  }

  async list(): Promise<string[]> {
    const response = await this.#peer.call<ConnectorList>("connectors.list")
    return response.connectors
  }

  sync(options: ConnectorsSyncOptions = {}): Promise<void> {
    return this.#peer.call<void>("connectors.sync", definedParams(options))
  }

  configure(name: string, config: JsonObject): Promise<void> {
    return this.#peer.call<void>("connectors.configure", { name, config })
  }
}

export class Connector {
  readonly #peer: Peer
  #name: string
  #meta: ConnectorMetadata
  #code: string
  #runtime?: ConnectorRuntime
  #extension?: ConnectorExtension
  #configured: boolean

  constructor(peer: Peer, name: string, wire: ConnectorWire) {
    this.#peer = peer
    this.#name = name
    this.#meta = metadataFromWire(wire.meta)
    this.#code = wire.code
    this.#runtime = wire.runtime
    this.#extension = wire.extension
    this.#configured = wire.configured
  }

  get name(): string {
    return this.#name
  }

  get meta(): ConnectorMetadata {
    return this.#meta
  }

  get code(): string {
    return this.#code
  }

  get runtime(): ConnectorRuntime | undefined {
    return this.#runtime
  }

  get extension(): ConnectorExtension | undefined {
    return this.#extension
  }

  get configured(): boolean {
    return this.#configured
  }

  async update(definition: ConnectorDefinition): Promise<void> {
    const wire = await this.#peer.call<ConnectorWire>(
      "connectors.update",
      connectorParams(this.#name, definition),
    )
    this.#replace(wire)
  }

  async configure(config: JsonObject): Promise<void> {
    await this.#peer.call<void>("connectors.configure", {
      name: this.#name,
      config,
    })
    this.#configured = true
  }

  delete(): Promise<void> {
    return this.#peer.call<void>("connectors.delete", { name: this.#name })
  }

  #replace(wire: ConnectorWire): void {
    this.#meta = metadataFromWire(wire.meta)
    this.#code = wire.code
    this.#runtime = wire.runtime
    this.#extension = wire.extension
    this.#configured = wire.configured
  }
}

function connectorParams(name: string, definition: ConnectorDefinition): JsonObject {
  return definedParams({
    name,
    prompt: "prompt" in definition ? definition.prompt : undefined,
    code: "code" in definition ? definition.code : undefined,
    meta: "meta" in definition ? metadataToWire(definition.meta) : undefined,
    runtime: definition.runtime,
  })
}

export function connectorMetadataFromString(contents: string): ConnectorMetadata {
  let parsed: unknown
  try {
    parsed = YAML.parse(contents)
  } catch {
    parsed = JSON.parse(contents)
  }
  if (!isObject(parsed)) {
    throw new Error("connector metadata must be an object")
  }
  return metadataFromWire(parsed as ConnectorMetadataWire)
}

function metadataToWire(meta: ConnectorMetadata | string): ConnectorMetadataWire {
  if (typeof meta === "string") {
    return metadataToWire(connectorMetadataFromString(meta))
  }
  return definedParams({
    description: meta.description,
    columns: meta.columns,
    generators: meta.generators,
    config_schema: meta.configSchema,
  }) as ConnectorMetadataWire
}

function metadataFromWire(meta: ConnectorMetadataWire): ConnectorMetadata {
  return definedParams({
    description: meta.description,
    columns: meta.columns,
    generators: meta.generators,
    configSchema: meta.config_schema,
  }) as ConnectorMetadata
}

function isObject(value: unknown): value is JsonObject {
  return typeof value === "object" && value !== null && !Array.isArray(value)
}
