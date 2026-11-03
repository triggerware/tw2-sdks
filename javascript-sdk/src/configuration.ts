import type { Peer } from "tw2-jsonrpc"
import { definedParams } from "./utils.js"

export interface Configuration {
  defaultTimeoutMs: number
  defaultBatchSize: number
  defaultTriggerBatchSize: number
  agentProvider: string
  agentModel: string
  maxLogFileSize: string
}

export interface ConfigurationUpdate extends Partial<Configuration> {
  agentApiKey?: string
  secure?: boolean
}

interface ConfigurationWire {
  default_timeout_ms: number
  default_batch_size: number
  default_trigger_batch_size: number
  agent_provider: string
  agent_model: string
  max_log_file_size: string
}

export class ConfigurationService {
  readonly #peer: Peer

  constructor(peer: Peer) {
    this.#peer = peer
  }

  async get(): Promise<Configuration> {
    return configurationFromWire(await this.#peer.call<ConfigurationWire>("configuration.get"))
  }

  async set(config: ConfigurationUpdate): Promise<Configuration | { url: string }> {
    const response = await this.#peer.call<ConfigurationWire | { url: string }>(
      "configuration.set",
      configurationToWire(config),
    )
    return "url" in response ? response : configurationFromWire(response)
  }

  async refresh(): Promise<Configuration> {
    return configurationFromWire(await this.#peer.call<ConfigurationWire>("configuration.refresh"))
  }
}

function configurationToWire(config: ConfigurationUpdate): Record<string, unknown> {
  return definedParams({
    default_timeout_ms: config.defaultTimeoutMs,
    default_batch_size: config.defaultBatchSize,
    default_trigger_batch_size: config.defaultTriggerBatchSize,
    agent_provider: config.agentProvider,
    agent_model: config.agentModel,
    agent_api_key: config.agentApiKey,
    max_log_file_size: config.maxLogFileSize,
    secure: config.secure,
  })
}

function configurationFromWire(config: ConfigurationWire): Configuration {
  return definedParams({
    defaultTimeoutMs: config.default_timeout_ms,
    defaultBatchSize: config.default_batch_size,
    defaultTriggerBatchSize: config.default_trigger_batch_size,
    agentProvider: config.agent_provider,
    agentModel: config.agent_model,
    maxLogFileSize: config.max_log_file_size,
  }) as Configuration
}
