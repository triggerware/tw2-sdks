import type { Peer } from "tw2-jsonrpc"
import { definedParams } from "./utils.js"

export interface Configuration {
  defaultTimeoutMs?: number
  defaultBatchSize?: number
  defaultTriggerBatchSize?: number
  llmProvider?: string
  llmModel?: string
  maxLogFileSize?: string
}

export interface ConfigurationUpdate extends Configuration {
  llmApiKey?: string
}

interface ConfigurationWire {
  default_timeout_ms?: number
  default_batch_size?: number
  default_trigger_batch_size?: number
  llm_provider?: string
  llm_model?: string
  max_log_file_size?: string
}

export class ConfigurationService {
  readonly #peer: Peer

  constructor(peer: Peer) {
    this.#peer = peer
  }

  async get(): Promise<Configuration> {
    return configurationFromWire(await this.#peer.call<ConfigurationWire>("configuration.get"))
  }

  async set(config: ConfigurationUpdate): Promise<Configuration> {
    const response = await this.#peer.call<ConfigurationWire>(
      "configuration.set",
      configurationToWire(config),
    )
    return configurationFromWire(response)
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
    llm_provider: config.llmProvider,
    llm_model: config.llmModel,
    llm_api_key: config.llmApiKey,
    max_log_file_size: config.maxLogFileSize,
  })
}

function configurationFromWire(config: ConfigurationWire): Configuration {
  return definedParams({
    defaultTimeoutMs: config.default_timeout_ms,
    defaultBatchSize: config.default_batch_size,
    defaultTriggerBatchSize: config.default_trigger_batch_size,
    llmProvider: config.llm_provider,
    llmModel: config.llm_model,
    maxLogFileSize: config.max_log_file_size,
  }) as Configuration
}
