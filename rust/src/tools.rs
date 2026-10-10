use std::sync::Arc;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use tw2_jsonrpc::prelude::*;

#[derive(Default, Serialize, Params)]
#[serde_with::skip_serializing_none]
pub struct ToolWrite {
    pub name: String,
    pub prompt: Option<String>,
    pub code: Option<String>,
    pub meta: Option<Value>,
    pub runtime: Option<crate::ConnectorRuntime>,
}

pub struct ToolsService { peer: Arc<Peer> }

impl ToolsService {
    pub fn new(peer: Arc<Peer>) -> Self { Self { peer } }

    pub async fn create(&self, options: ToolWrite) -> Result<Value> {
        self.peer.call("tools.create", options).await
    }

    pub async fn update(&self, options: ToolWrite) -> Result<Value> {
        self.peer.call("tools.update", options).await
    }

    pub async fn get(&self, name: impl Into<String>) -> Result<Value> {
        self.peer.call("tools.get", params!({"name":name.into()})).await
    }

    pub async fn delete(&self, name: impl Into<String>) -> Result<()> {
        self.peer.call("tools.delete", params!({"name":name.into()})).await
    }

    pub async fn list(&self) -> Result<Vec<String>> {
        #[derive(Deserialize)]
        struct Response { tools: Vec<String> }
        let response: Response = self.peer.call("tools.list", ()).await?;
        Ok(response.tools)
    }

    pub async fn sync(&self, tools: Option<Vec<String>>, extensions: Option<Vec<String>>) -> Result<()> {
        self.peer.call("tools.sync", params!({"tools":tools,"extensions":extensions})).await
    }

    pub async fn configure(&self, name: impl Into<String>, config: Option<Map<String, Value>>, secure: bool) -> Result<Value> {
        self.peer.call("tools.configure", params!({"name":name.into(),"config":config,"secure":secure})).await
    }
}
