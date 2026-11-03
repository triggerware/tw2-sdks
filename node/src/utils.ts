import type { JsonObject, RunOptions } from "./types.js"

export function definedParams(values: object): JsonObject {
  return Object.fromEntries(
    Object.entries(values).filter(([, value]) => value !== undefined),
  )
}

export function runOptions(options: RunOptions = {}): JsonObject {
  return definedParams({
    batch_size: options.batchSize,
    timeout_ms: options.timeoutMs,
  })
}

export function withRunOptions(params: JsonObject, options: RunOptions = {}): JsonObject {
  return {
    ...params,
    ...runOptions(options),
  }
}

export function splitAddress(address: string): [string, number] {
  const split = address.lastIndexOf(":")
  if (split < 0) {
    throw new Error("TCP address must be in host:port form")
  }

  const host = address.slice(0, split)
  const port = Number(address.slice(split + 1))
  if (!Number.isInteger(port) || port <= 0) {
    throw new Error("TCP address must include a valid port")
  }

  return [host, port]
}
