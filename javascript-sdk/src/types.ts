export type JsonObject = Record<string, unknown>
export type Row = unknown[]

export type Runtime = "lua" | "python" | "node"

export interface RunOptions {
  batchSize?: number
  timeoutMs?: number
}

export interface NamedColumn {
  name: string
  type: string
}

export interface Health {
  status: string
}

export interface AuthorizationResult {
  url?: string
}
