use std::sync::Arc;

use tw2_jsonrpc::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Map;

pub struct ConnectorsService {
    peer: Arc<Peer>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connector {
    #[serde(skip_deserializing)]
    name: String,
    #[serde(skip_deserializing)]
    tag: Option<String>,
    pub config: Option<Map<String, Value>>,
    pub tables: Vec<String>,
    pub tools: Vec<String>,
    pub agents: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectorDescription {
    pub description: String,
    pub config_schema: Map<String, Value>,
    #[serde(default)]
    pub oauth_provider: Option<String>,
    pub runtime: Option<crate::Runtime>,
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
    pub config: Map<String, Value>,
}

#[derive(Default, Serialize, Params)]
#[serde_with::skip_serializing_none]
pub struct ConnectorReference {
    pub name: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ConnectorAuthorization {
    pub url: Option<String>,
}

#[derive(Deserialize)]
struct ConnectorList {
    connectors: Vec<String>,
}

#[derive(Deserialize)]
struct ConnectorCatalog {
    connectors: Vec<ConnectorCatalogEntry>,
}

impl ConnectorsService {
    pub fn new(peer: Arc<Peer>) -> ConnectorsService {
        ConnectorsService { peer }
    }

    pub async fn install(&self, install: ConnectorInstall) -> Result<ConnectorAuthorization> {
        self.peer.call("connectors.install", install).await
    }

    pub async fn install_secure(
        &self,
        name: Option<impl Into<String>>,
        url: Option<impl Into<String>>,
        tag: Option<impl Into<String>>,
    ) -> Result<ConnectorAuthorization> {
        let mut params = params!().with("secure", true)?;
        if let Some(name) = name {
            params = params.with("name", name.into())?;
        }
        if let Some(url) = url {
            params = params.with("url", url.into())?;
        }
        if let Some(tag) = tag {
            params = params.with("tag", tag.into())?;
        }
        self.peer.call("connectors.install", params).await
    }

    pub async fn authorize(
        &self,
        name: impl Into<String>,
        tag: Option<impl Into<String>>,
    ) -> Result<ConnectorAuthorization> {
        let mut params = params!().with("name", name.into())?;
        if let Some(tag) = tag {
            params = params.with("tag", tag.into())?;
        }
        self.peer.call("connectors.authorize", params).await
    }

    pub async fn get(
        &self,
        name: impl Into<String>,
        tag: Option<impl Into<String>>,
    ) -> Result<Connector> {
        let name = name.into();
        let tag = tag.map(Into::into);
        let mut params = params!().with("name", name.clone())?;
        if let Some(tag) = tag.clone() {
            params = params.with("tag", tag)?;
        }
        let mut connector: Connector = self
            .peer
            .call("connectors.get", params)
            .await?;
        connector.name = name;
        connector.tag = tag;
        Ok(connector)
    }

    pub async fn list(&self) -> Result<Vec<String>> {
        self.peer
            .call::<ConnectorList>("connectors.list", ())
            .await
            .map(|response| response.connectors)
    }

    pub async fn configure<C>(
        &self,
        name: impl Into<String>,
        tag: Option<impl Into<String>>,
        config: C,
    ) -> Result<()>
    where
        C: Serialize,
    {
        let mut params = params!()
            .with("name", name.into())?
            .with("config", config)?;
        if let Some(tag) = tag {
            params = params.with("tag", tag.into())?;
        }
        self.peer
            .call::<()>("connectors.configure", params)
            .await
    }

    pub async fn configure_secure(
        &self,
        name: impl Into<String>,
        tag: Option<impl Into<String>>,
    ) -> Result<Option<String>> {
        #[derive(Deserialize)]
        struct SecureConfiguration {
            url: String,
        }

        let mut params = params!()
            .with("name", name.into())?
            .with("secure", true)?;
        if let Some(tag) = tag {
            params = params.with("tag", tag.into())?;
        }
        let response: Option<SecureConfiguration> =
            self.peer.call("connectors.configure", params).await?;
        Ok(response.map(|response| response.url))
    }

    pub async fn catalog(&self) -> Result<Vec<ConnectorCatalogEntry>> {
        self.peer
            .call::<ConnectorCatalog>("connectors.catalog", ())
            .await
            .map(|response| response.connectors)
    }

    pub async fn describe(&self, reference: ConnectorReference) -> Result<ConnectorDescription> {
        self.peer.call("connectors.describe", reference).await
    }

    pub async fn uninstall(
        &self,
        name: impl Into<String>,
        tag: Option<impl Into<String>>,
    ) -> Result<()> {
        let mut params = params!().with("name", name.into())?;
        if let Some(tag) = tag {
            params = params.with("tag", tag.into())?;
        }
        self.peer
            .call("connectors.uninstall", params)
            .await
    }
}

impl Connector {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn tag(&self) -> Option<&str> {
        self.tag.as_deref()
    }
}
