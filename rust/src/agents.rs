use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use tw2_jsonrpc::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde_with::skip_serializing_none]
pub struct AgentMetadata {
    pub description: String,
    pub tools: Option<Vec<String>>,
    pub local_tools: Option<Vec<String>>,
    pub mcp_servers: Option<Vec<Vec<String>>>,
    pub triggers: Option<Vec<String>>,
    pub output_schema: Option<Map<String, Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub name: String,
    pub prompt: Option<String>,
    pub meta: AgentMetadata,
    pub instructions: String,
}

/// Creation requires name, meta, and instructions unless prompt is supplied.
/// Updates require name; omitted fields keep their current values.
#[derive(Default, Serialize, Params)]
#[serde_with::skip_serializing_none]
pub struct AgentWrite {
    pub name: Option<String>,
    pub prompt: Option<String>,
    pub meta: Option<AgentMetadata>,
    pub instructions: Option<String>,
}

pub struct AgentsService {
    peer: Arc<Peer>,
}

impl AgentsService {
    pub fn new(peer: Arc<Peer>) -> Self {
        Self { peer }
    }

    pub async fn create(&self, options: AgentWrite) -> Result<Agent> {
        if options.prompt.is_none()
            && (options.name.is_none() || options.meta.is_none() || options.instructions.is_none())
        {
            return Err(JsonRpcError::invalid_params());
        }
        self.peer.call("agents.create", options).await
    }

    pub async fn update(&self, options: AgentWrite) -> Result<Agent> {
        if options.name.is_none() {
            return Err(JsonRpcError::invalid_params());
        }
        self.peer.call("agents.update", options).await
    }

    pub async fn get(&self, names: Vec<String>) -> Result<Vec<Agent>> {
        #[derive(Deserialize)]
        struct AgentList {
            agents: Vec<Agent>,
        }
        let response: AgentList = self.peer.call("agents.get", params!({"name": names})).await?;
        Ok(response.agents)
    }

    pub async fn delete(&self, names: Vec<String>) -> Result<()> {
        self.peer.call("agents.delete", params!({"name": names})).await
    }

    pub async fn list(&self) -> Result<Vec<String>> {
        #[derive(Deserialize)]
        struct AgentList {
            agents: Vec<String>,
        }
        let response: AgentList = self.peer.call("agents.list", ()).await?;
        Ok(response.agents)
    }

    pub async fn sync(&self, agents: Option<Vec<String>>, connectors: Option<Vec<String>>) -> Result<()> {
        let mut params = params!();
        if let Some(agents) = agents {
            params = params.with("agents", agents)?;
        }
        if let Some(connectors) = connectors {
            params = params.with("connectors", connectors)?;
        }
        self.peer.call("agents.sync", params).await
    }

    pub async fn run(
        &self,
        name: impl Into<String>,
        data: Map<String, Value>,
    ) -> Result<Map<String, Value>> {
        self.peer.call("agents.run", params!({"name": name.into(), "data": data})).await
    }
}
