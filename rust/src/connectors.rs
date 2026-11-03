use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::Map;
use tw2_jsonrpc::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connector {
    pub config: Map<String, Value>,
    pub tables: Vec<String>,
    pub tools: Vec<String>,
    pub agents: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectorDescription {
    pub description: String,
    pub config_schema: Map<String, Value>,
    pub oauth_provider: Option<String>,
    pub runtime: Option<crate::ExternalRuntime>,
    pub dependencies: Option<Vec<String>>,
    pub tables: Vec<String>,
    pub tools: Vec<String>,
    pub agents: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectorCatalogEntry {
    pub name: String,
    pub description: String,
}

#[derive(Default, Serialize, Params)]
#[serde_with::skip_serializing_none]
pub struct ConnectorInstall {
    pub name: Option<String>,
    pub url: Option<String>,
    pub tag: Option<String>,
    pub config: Option<Map<String, Value>>,
    pub secure: Option<bool>,
}

#[derive(Default, Serialize, Params)]
#[serde_with::skip_serializing_none]
pub struct ConnectorConfiguration {
    pub name: String,
    pub tag: Option<String>,
    pub config: Option<Map<String, Value>>,
    pub secure: Option<bool>,
}

#[derive(Default, Serialize, Params)]
#[serde_with::skip_serializing_none]
pub struct ConnectorReference {
    pub name: Option<String>,
    pub url: Option<String>,
}

pub struct ConnectorsService {
    peer: Arc<Peer>,
}

impl ConnectorsService {
    pub fn new(peer: Arc<Peer>) -> Self {
        Self { peer }
    }

    pub async fn install(&self, options: ConnectorInstall) -> Result<crate::AuthorizationResult> {
        if options.name.is_none() && options.url.is_none() {
            return Err(JsonRpcError::invalid_params());
        }
        self.peer.call("connectors.install", options).await
    }

    pub async fn get(&self, names: Vec<String>, tag: Option<String>) -> Result<Vec<Connector>> {
        #[derive(Deserialize)]
        struct ConnectorList {
            connectors: Vec<Connector>,
        }
        let mut params = params!({"name": names});
        if let Some(tag) = tag {
            params = params.with("tag", tag)?;
        }
        let response: ConnectorList = self.peer.call("connectors.get", params).await?;
        Ok(response.connectors)
    }

    pub async fn list(&self) -> Result<Vec<String>> {
        #[derive(Deserialize)]
        struct ConnectorList {
            connectors: Vec<String>,
        }
        let response: ConnectorList = self.peer.call("connectors.list", ()).await?;
        Ok(response.connectors)
    }

    pub async fn uninstall(&self, names: Vec<String>, tag: Option<String>) -> Result<()> {
        let mut params = params!({"name": names});
        if let Some(tag) = tag {
            params = params.with("tag", tag)?;
        }
        self.peer.call("connectors.uninstall", params).await
    }

    pub async fn configure(&self, options: ConnectorConfiguration) -> Result<crate::AuthorizationResult> {
        self.peer.call("connectors.configure", options).await
    }

    pub async fn authorize(&self, name: impl Into<String>, tag: Option<String>) -> Result<String> {
        #[derive(Deserialize)]
        struct Authorization {
            url: String,
        }
        let mut params = params!({"name": name.into()});
        if let Some(tag) = tag {
            params = params.with("tag", tag)?;
        }
        let response: Authorization = self.peer.call("connectors.authorize", params).await?;
        Ok(response.url)
    }

    pub async fn catalog(&self) -> Result<Vec<ConnectorCatalogEntry>> {
        #[derive(Deserialize)]
        struct ConnectorCatalog {
            connectors: Vec<ConnectorCatalogEntry>,
        }
        let response: ConnectorCatalog = self.peer.call("connectors.catalog", ()).await?;
        Ok(response.connectors)
    }

    pub async fn describe(&self, reference: ConnectorReference) -> Result<ConnectorDescription> {
        if reference.name.is_none() && reference.url.is_none() {
            return Err(JsonRpcError::invalid_params());
        }
        self.peer.call("connectors.describe", reference).await
    }
}
