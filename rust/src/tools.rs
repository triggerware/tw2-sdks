use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use tw2_jsonrpc::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolParameter {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde_with::skip_serializing_none]
pub struct ToolMetadata {
    pub description: String,
    pub signature: Vec<ToolParameter>,
    pub config_schema: Option<Map<String, Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    pub name: String,
    pub prompt: Option<String>,
    pub meta: ToolMetadata,
    pub code: String,
    pub runtime: crate::Runtime,
    pub connector: Option<crate::TableConnector>,
    pub configured: bool,
}

/// Creation requires name, code, and meta unless prompt is supplied.
/// Updates require name; omitted fields keep their current values.
#[derive(Default, Serialize, Params)]
#[serde_with::skip_serializing_none]
pub struct ToolWrite {
    pub name: Option<String>,
    pub prompt: Option<String>,
    pub code: Option<String>,
    pub meta: Option<ToolMetadata>,
    pub runtime: Option<crate::Runtime>,
}

pub struct ToolsService {
    peer: Arc<Peer>,
}

impl ToolsService {
    pub fn new(peer: Arc<Peer>) -> Self {
        Self { peer }
    }

    pub async fn create(&self, options: ToolWrite) -> Result<Tool> {
        if options.prompt.is_none()
            && (options.name.is_none() || options.code.is_none() || options.meta.is_none())
        {
            return Err(JsonRpcError::invalid_params());
        }
        self.peer.call("tools.create", options).await
    }

    pub async fn update(&self, options: ToolWrite) -> Result<Tool> {
        if options.name.is_none() {
            return Err(JsonRpcError::invalid_params());
        }
        self.peer.call("tools.update", options).await
    }

    pub async fn get(&self, names: Vec<String>) -> Result<Vec<Tool>> {
        #[derive(Deserialize)]
        struct ToolList {
            tools: Vec<Tool>,
        }
        let response: ToolList = self.peer.call("tools.get", params!({"name": names})).await?;
        Ok(response.tools)
    }

    pub async fn delete(&self, names: Vec<String>) -> Result<()> {
        self.peer.call("tools.delete", params!({"name": names})).await
    }

    pub async fn list(&self) -> Result<Vec<String>> {
        #[derive(Deserialize)]
        struct ToolList {
            tools: Vec<String>,
        }
        let response: ToolList = self.peer.call("tools.list", ()).await?;
        Ok(response.tools)
    }

    pub async fn sync(&self, tools: Option<Vec<String>>, connectors: Option<Vec<String>>) -> Result<()> {
        let mut params = params!();
        if let Some(tools) = tools {
            params = params.with("tools", tools)?;
        }
        if let Some(connectors) = connectors {
            params = params.with("connectors", connectors)?;
        }
        self.peer.call("tools.sync", params).await
    }

    pub async fn configure(&self, options: crate::ResourceConfiguration) -> Result<crate::AuthorizationResult> {
        self.peer.call("tools.configure", options).await
    }
}
