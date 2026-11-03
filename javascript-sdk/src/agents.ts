import type { Peer } from "tw2-jsonrpc"
import type { JsonObject } from "./types.js"
import { definedParams } from "./utils.js"

export interface AgentMetadata {
  description: string
  tools?: string[]
  mcpServers?: string[][]
  triggers?: string[]
  outputSchema?: JsonObject
}

interface AgentMetadataWire {
  description: string
  tools?: string[]
  mcp_servers?: string[][]
  triggers?: string[]
  output_schema?: JsonObject
}

export interface Agent {
  name: string
  prompt?: string
  meta: AgentMetadata
  instructions: string
}

interface AgentWire {
  name: string
  prompt?: string
  meta: AgentMetadataWire
  instructions: string
}

export interface AgentUpdate {
  prompt?: string
  meta?: AgentMetadata
  instructions?: string
}

export type AgentCreation =
  | (AgentUpdate & { prompt: string; name?: string })
  | (AgentUpdate & { name: string; meta: AgentMetadata; instructions: string })

export class AgentsService {
  readonly #peer: Peer

  constructor(peer: Peer) {
    this.#peer = peer
  }

  async create(creation: AgentCreation): Promise<Agent> {
    return agentFromWire(await this.#peer.call<AgentWire>("agents.create", agentParams(creation)))
  }

  async get(name: string | string[]): Promise<Agent | Agent[]> {
    if (typeof name === "string") {
      return agentFromWire(await this.#peer.call<AgentWire>("agents.get", { name }))
    }
    const response = await this.#peer.call<{ agents: AgentWire[] }>("agents.get", { name })
    return response.agents.map(agentFromWire)
  }

  async update(name: string, update: AgentUpdate): Promise<Agent> {
    return agentFromWire(await this.#peer.call<AgentWire>(
      "agents.update",
      agentParams({ name, ...update }),
    ))
  }

  async list(): Promise<string[]> {
    const response = await this.#peer.call<{ agents: string[] }>("agents.list")
    return response.agents
  }

  delete(name: string | string[]): Promise<void> {
    return this.#peer.call<void>("agents.delete", { name })
  }

  run(name: string, data: JsonObject): Promise<JsonObject> {
    return this.#peer.call<JsonObject>("agents.run", { name, data })
  }

  async sync(options: { agents?: string[]; connectors?: string[] } = {}): Promise<void> {
    await this.#peer.call("agents.sync", definedParams(options))
  }
}

function agentParams(values: AgentUpdate & { name?: string }): JsonObject {
  return definedParams({
    name: values.name,
    prompt: values.prompt,
    instructions: values.instructions,
    meta: values.meta === undefined ? undefined : definedParams({
      description: values.meta.description,
      tools: values.meta.tools,
      mcp_servers: values.meta.mcpServers,
      triggers: values.meta.triggers,
      output_schema: values.meta.outputSchema,
    }),
  })
}

function agentFromWire(agent: AgentWire): Agent {
  return {
    name: agent.name,
    prompt: agent.prompt,
    instructions: agent.instructions,
    meta: definedParams({
      description: agent.meta.description,
      tools: agent.meta.tools,
      mcpServers: agent.meta.mcp_servers,
      triggers: agent.meta.triggers,
      outputSchema: agent.meta.output_schema,
    }) as unknown as AgentMetadata,
  }
}
