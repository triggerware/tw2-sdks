import type { Peer } from "tw2-jsonrpc"
import YAML from "yaml"
import type { AuthorizationResult, JsonObject, Runtime } from "./types.js"
import { definedParams } from "./utils.js"

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

export interface Table {
  name: string
  prompt?: string
  meta: TableMetadata
  code: string
  runtime: Runtime
  connector?: TableConnector
  configured: boolean
}

interface TableWire {
  name: string
  prompt?: string
  meta: TableMetadataWire
  code: string
  runtime: Runtime
  connector?: TableConnector
  configured: boolean
}

export interface TableUpdate {
  prompt?: string
  code?: string
  meta?: TableMetadata | string
  runtime?: Runtime
}

export type TableCreation =
  | (TableUpdate & { name?: string; prompt: string })
  | (TableUpdate & { name: string; code: string; meta: TableMetadata | string })

export interface TablesSyncOptions {
  tables?: string[]
  connectors?: string[]
}

export class TablesService {
  readonly #peer: Peer

  constructor(peer: Peer) {
    this.#peer = peer
  }

  async create(creation: TableCreation): Promise<Table> {
    return tableFromWire(await this.#peer.call<TableWire>(
      "tables.create",
      tableParams(creation.name, creation),
    ))
  }

  async get(name: string | string[]): Promise<Table | Table[]> {
    if (typeof name === "string") {
      return tableFromWire(await this.#peer.call<TableWire>("tables.get", { name }))
    }
    const response = await this.#peer.call<{ tables: TableWire[] }>("tables.get", { name })
    return response.tables.map(tableFromWire)
  }

  async update(name: string, update: TableUpdate): Promise<Table> {
    return tableFromWire(await this.#peer.call<TableWire>(
      "tables.update",
      tableParams(name, update),
    ))
  }

  async list(): Promise<string[]> {
    const response = await this.#peer.call<{ tables: string[] }>("tables.list")
    return response.tables
  }

  delete(name: string | string[]): Promise<void> {
    return this.#peer.call<void>("tables.delete", { name })
  }

  sync(options: TablesSyncOptions = {}): Promise<void> {
    return this.#peer.call<void>("tables.sync", definedParams(options))
  }

  configure(
    name: string,
    options: { config?: JsonObject; secure?: boolean } = {},
  ): Promise<AuthorizationResult> {
    return this.#peer.call<AuthorizationResult>("tables.configure", definedParams({ name, ...options }))
  }
}

function tableParams(name: string | undefined, definition: TableUpdate): JsonObject {
  return definedParams({
    name,
    prompt: definition.prompt,
    code: definition.code,
    meta: definition.meta === undefined ? undefined : metadataToWire(definition.meta),
    runtime: definition.runtime,
  })
}

export function tableMetadataFromString(contents: string): TableMetadata {
  const parsed: unknown = YAML.parse(contents)
  if (typeof parsed !== "object" || parsed === null || Array.isArray(parsed)) {
    throw new Error("table metadata must be an object")
  }
  return metadataFromWire(parsed as TableMetadataWire)
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

function tableFromWire(table: TableWire): Table {
  return { ...table, meta: metadataFromWire(table.meta) }
}
