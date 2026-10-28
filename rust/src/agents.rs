use std::sync::Arc;

use serde_json::{Map, Value};
use tw2_jsonrpc::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Params)]
#[serde_with::skip_serializing_none]
pub struct AgentWrite {
    pub name: String,
    pub prompt: Option<String>,
    pub instructions: Option<String>,
    pub tools: Option<Vec<String>>,
    pub mcp_servers: Option<Vec<Vec<String>>>,
    pub triggers: Option<Vec<String>>,
}

pub struct AgentsService {
    peer: Arc<Peer>,
}

impl AgentsService {
    pub async fn create(&self, options: AgentWrite) -> Result<Value> {
        self.peer.call("agents.create", options).await
    }

    pub async fn update(&self, options: AgentWrite) -> Result<()> {
        self.peer.call("agents.update", options).await
    }

    pub async fn get(&self, name: impl Into<String>) -> Result<Value> {
        self.peer.call("agents.get", params!({"name":name.into()})).await
    }

    pub async fn delete(&self, name: impl Into<String>) -> Result<()> {
        self.peer.call("agents.delete", params!({"name":name.into()})).await
    }

    pub async fn list(&self) -> Result<Vec<String>> {
        #[derive(Deserialize)]
        struct Response { agents: Vec<String> }
        let response: Response = self.peer.call("agents.list", ()).await?;
        Ok(response.agents)
    }

    pub async fn sync(&self, agents: Option<Vec<String>>, connectors: Option<Vec<String>>) -> Result<()> {
        self.peer.call("agents.sync", params!({"agents":agents,"connectors":connectors})).await
    }
    pub fn new(peer: Arc<Peer>) -> Self {
        Self { peer }
    }

    pub async fn run(&self, name: impl Into<String>, data: Map<String, Value>) -> Result<()> {
        self.peer.call("agents.run", params!({"name": name.into(), "data": data})).await
    }
}
