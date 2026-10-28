import type { Peer } from "tw2-jsonrpc"
import YAML from "yaml"
import type { JsonObject, Runtime } from "./types.js"
import { definedParams } from "./utils.js"

export type TableRuntime = Runtime

export interface TableColumn {
  name: string
  type: string
}

export interface TableGenerator {
  name: string
  inputs: string[]
}

export interface TableMetadata {
  description?: string
  columns: TableColumn[]
  generators: TableGenerator[]
  configSchema?: JsonObject
}

interface TableMetadataWire {
  description?: string
  columns: TableColumn[]
  generators: TableGenerator[]
  config_schema?: JsonObject
}

export interface TableConnector {
  name: string
  tag?: string
}

interface TableWire {
  meta: TableMetadataWire
  code: string
  runtime?: TableRuntime
  connector?: TableConnector
  configured: boolean
}

export type TableDefinition =
  | { prompt: string; runtime?: TableRuntime }
  | { code: string; meta: TableMetadata | string; runtime?: TableRuntime }

interface TableList {
  tables: string[]
}

export interface TablesSyncOptions {
  tables?: string[]
  connectors?: string[]
}

export class TablesService {
  readonly #peer: Peer

  constructor(peer: Peer) {
    this.#peer = peer
  }

  async create(name: string, definition: TableDefinition): Promise<Table> {
    const wire = await this.#peer.call<TableWire>(
      "tables.create",
      tableParams(name, definition),
    )
    return new Table(this.#peer, name, wire)
  }

  async get(name: string): Promise<Table> {
    const wire = await this.#peer.call<TableWire>("tables.get", { name })
    return new Table(this.#peer, name, wire)
  }

  async list(): Promise<string[]> {
    const response = await this.#peer.call<TableList>("tables.list")
    return response.tables
  }

  sync(options: TablesSyncOptions = {}): Promise<void> {
    return this.#peer.call<void>("tables.sync", definedParams(options))
  }

  configure(name: string, config: JsonObject): Promise<void> {
    return this.#peer.call<void>("tables.configure", { name, config })
  }
}

export class Table {
  readonly #peer: Peer
  #name: string
  #meta: TableMetadata
  #code: string
  #runtime?: TableRuntime
  #connector?: TableConnector
  #configured: boolean

  constructor(peer: Peer, name: string, wire: TableWire) {
    this.#peer = peer
    this.#name = name
    this.#meta = metadataFromWire(wire.meta)
    this.#code = wire.code
    this.#runtime = wire.runtime
    this.#connector = wire.connector
    this.#configured = wire.configured
  }

  get name(): string {
    return this.#name
  }

  get meta(): TableMetadata {
    return this.#meta
  }

  get code(): string {
    return this.#code
  }

  get runtime(): TableRuntime | undefined {
    return this.#runtime
  }

  get connector(): TableConnector | undefined {
    return this.#connector
  }

  get configured(): boolean {
    return this.#configured
  }

  async update(definition: TableDefinition): Promise<void> {
    const wire = await this.#peer.call<TableWire>(
      "tables.update",
      tableParams(this.#name, definition),
    )
    this.#replace(wire)
  }

  async configure(config: JsonObject): Promise<void> {
    await this.#peer.call<void>("tables.configure", {
      name: this.#name,
      config,
    })
    this.#configured = true
  }

  delete(): Promise<void> {
    return this.#peer.call<void>("tables.delete", { name: this.#name })
  }

  #replace(wire: TableWire): void {
    this.#meta = metadataFromWire(wire.meta)
    this.#code = wire.code
    this.#runtime = wire.runtime
    this.#connector = wire.connector
    this.#configured = wire.configured
  }
}

function tableParams(name: string, definition: TableDefinition): JsonObject {
  return definedParams({
    name,
    prompt: "prompt" in definition ? definition.prompt : undefined,
    code: "code" in definition ? definition.code : undefined,
    meta: "meta" in definition ? metadataToWire(definition.meta) : undefined,
    runtime: definition.runtime,
  })
}

export function tableMetadataFromString(contents: string): TableMetadata {
  let parsed: unknown
  try {
    parsed = YAML.parse(contents)
  } catch {
    parsed = JSON.parse(contents)
  }
  if (!isObject(parsed)) {
    throw new Error("table metadata must be an object")
  }
  return metadataFromWire(parsed as unknown as TableMetadataWire)
}

function metadataToWire(meta: TableMetadata | string): TableMetadataWire {
  if (typeof meta === "string") {
    return metadataToWire(tableMetadataFromString(meta))
  }
  return definedParams({
    description: meta.description,
    columns: meta.columns,
    generators: meta.generators,
    config_schema: meta.configSchema,
  }) as unknown as TableMetadataWire
}

function metadataFromWire(meta: TableMetadataWire): TableMetadata {
  return definedParams({
    description: meta.description,
    columns: meta.columns,
    generators: meta.generators,
    configSchema: meta.config_schema,
  }) as unknown as TableMetadata
}

function isObject(value: unknown): value is JsonObject {
  return typeof value === "object" && value !== null && !Array.isArray(value)
}
