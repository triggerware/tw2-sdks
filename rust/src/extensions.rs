use std::sync::Arc;

use tw2_jsonrpc::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Map;

pub struct ExtensionsService {
    peer: Arc<Peer>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Extension {
    #[serde(skip_deserializing)]
    name: String,
    #[serde(skip_deserializing)]
    tag: Option<String>,
    pub config: Option<Map<String, Value>>,
    pub connectors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionDescription {
    pub description: String,
    pub config_schema: Map<String, Value>,
    #[serde(default)]
    pub oauth_provider: Option<String>,
    pub runtime: Option<ExtensionRuntime>,
    pub dependencies: Option<Vec<String>>,
    pub connectors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionCatalogEntry {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExtensionRuntime {
    #[default]
    Lua,
    Python,
    Node,
}

#[derive(Default, Serialize, Params)]
#[serde_with::skip_serializing_none]
pub struct ExtensionInstall {
    pub name: Option<String>,
    pub url: Option<String>,
    pub tag: Option<String>,
    pub config: Map<String, Value>,
}

#[derive(Default, Serialize, Params)]
#[serde_with::skip_serializing_none]
pub struct ExtensionReference {
    pub name: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ExtensionAuthorization {
    pub url: Option<String>,
}

#[derive(Deserialize)]
struct ExtensionList {
    extensions: Vec<String>,
}

#[derive(Deserialize)]
struct ExtensionCatalog {
    extensions: Vec<ExtensionCatalogEntry>,
}

impl ExtensionsService {
    pub fn new(peer: Arc<Peer>) -> ExtensionsService {
        ExtensionsService { peer }
    }

    pub async fn install(&self, install: ExtensionInstall) -> Result<ExtensionAuthorization> {
        self.peer.call("extensions.install", install).await
    }

    pub async fn authorize(
        &self,
        name: impl Into<String>,
        tag: Option<impl Into<String>>,
    ) -> Result<ExtensionAuthorization> {
        let mut params = params!().with("name", name.into())?;
        if let Some(tag) = tag {
            params = params.with("tag", tag.into())?;
        }
        self.peer.call("extensions.authorize", params).await
    }

    pub async fn get(
        &self,
        name: impl Into<String>,
        tag: Option<impl Into<String>>,
    ) -> Result<Extension> {
        let name = name.into();
        let tag = tag.map(Into::into);
        let mut params = params!().with("name", name.clone())?;
        if let Some(tag) = tag.clone() {
            params = params.with("tag", tag)?;
        }
        let mut extension: Extension = self
            .peer
            .call("extensions.get", params)
            .await?;
        extension.name = name;
        extension.tag = tag;
        Ok(extension)
    }

    pub async fn list(&self) -> Result<Vec<String>> {
        self.peer
            .call::<ExtensionList>("extensions.list", ())
            .await
            .map(|response| response.extensions)
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
            .call::<()>("extensions.configure", params)
            .await
    }

    pub async fn catalog(&self) -> Result<Vec<ExtensionCatalogEntry>> {
        self.peer
            .call::<ExtensionCatalog>("extensions.catalog", ())
            .await
            .map(|response| response.extensions)
    }

    pub async fn describe(&self, reference: ExtensionReference) -> Result<ExtensionDescription> {
        self.peer.call("extensions.describe", reference).await
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
            .call("extensions.uninstall", params)
            .await
    }
}

impl Extension {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn tag(&self) -> Option<&str> {
        self.tag.as_deref()
    }
}
