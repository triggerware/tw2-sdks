use std::fmt;
use std::str::FromStr;
use std::sync::Arc;

use tw2_jsonrpc::prelude::*;

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DefaultOnNull};

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct Connector {
    #[serde(skip)]
    peer: Option<Arc<Peer>>,
    #[serde(skip_deserializing)]
    name: String,
    meta: ConnectorMetadata,
    code: String,
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    runtime: ConnectorRuntime,
    extension: Option<ConnectorExtension>,
    configured: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectorExtension {
    pub name: String,
    pub tag: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ConnectorMetadata {
    pub description: String,
    pub columns: Vec<ConnectorColumn>,
    pub generators: Vec<ConnectorGenerator>,
    #[serde(default)]
    pub config_schema: Value,
}

impl ConnectorMetadata {
    pub fn from_string(contents: impl AsRef<str>) -> Result<Self> {
        contents.as_ref().parse()
    }
}

impl FromStr for ConnectorMetadata {
    type Err = JsonRpcError;

    fn from_str(contents: &str) -> Result<Self> {
        match serde_yaml::from_str(contents) {
            Ok(meta) => Ok(meta),
            Err(_) => serde_json::from_str(contents).map_err(|error| JsonRpcError {
                message: format!("invalid connector metadata: {error}"),
                ..JsonRpcError::invalid_params()
            }),
        }
    }
}

impl fmt::Display for ConnectorMetadata {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = serde_yaml::to_string(self).map_err(|_| fmt::Error)?;
        formatter.write_str(&value)
    }
}

#[derive(Serialize, Deserialize)]
pub struct ConnectorColumn {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: String,
}

#[derive(Serialize, Deserialize)]
pub struct ConnectorGenerator {
    pub name: String,
    pub inputs: Vec<String>,
}

#[derive(Clone, Copy, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConnectorRuntime {
    #[default]
    Lua,
    Python,
    Node,
}

pub struct ConnectorsService {
    peer: Arc<Peer>,
}

#[derive(Default, Serialize, Params)]
#[serde_with::skip_serializing_none]
pub struct ConnectorsSyncOptions {
    pub connectors: Option<Vec<String>>,
    pub extensions: Option<Vec<String>>,
}

pub enum ConnectorCreation {
    Prompt {
        prompt: String,
        code: Option<String>,
        meta: Option<ConnectorMetadata>,
    },
    Code {
        code: String,
        meta: ConnectorMetadata,
    },
}

impl ConnectorsService {
    pub fn new(peer: Arc<Peer>) -> ConnectorsService {
        ConnectorsService { peer }
    }

    pub async fn create(
        &self,
        name: impl Into<String>,
        create: ConnectorCreation,
        runtime: ConnectorRuntime,
    ) -> Result<Connector> {
        let name = name.into();
        let mut params = params!()
            .with("name", name.clone())?
            .with("runtime", runtime)?;

        params = match create {
            ConnectorCreation::Prompt { prompt, code, meta } => params
                .with("prompt", prompt)?
                .with("code", code)?
                .with("meta", meta)?,
            ConnectorCreation::Code { code, meta } => {
                params.with("code", code)?.with("meta", meta)?
            }
        };

        let mut connector: Connector = self.peer.call("connectors.create", params).await?;
        connector.peer = Some(self.peer.clone());
        connector.name = name;
        Ok(connector)
    }

    pub async fn get(&self, name: impl Into<String>) -> Result<Connector> {
        let name = name.into();
        let mut conn: Connector = self
            .peer
            .call("connectors.get", params!({"name": name}))
            .await?;
        conn.peer = Some(self.peer.clone());
        conn.name = name;
        Ok(conn)
    }

    pub async fn list(&self) -> Result<Vec<String>> {
        #[derive(Deserialize)]
        struct ConnectorList {
            connectors: Vec<String>,
        }
        let response: ConnectorList = self.peer.call("connectors.list", ()).await?;
        Ok(response.connectors)
    }

    pub async fn sync(&self, options: ConnectorsSyncOptions) -> Result<()> {
        self.peer.call("connectors.sync", options).await
    }

    pub async fn configure<C>(&self, name: impl Into<String>, config: C) -> Result<()>
    where
        C: Serialize,
    {
        let params = params!({
            "name": name.into(),
            "config": config
        });
        self.peer.call::<()>("connectors.configure", params).await
    }

    pub async fn configure_secure(&self, name: impl Into<String>) -> Result<Option<String>> {
        #[derive(Deserialize)]
        struct SecureConfiguration {
            url: String,
        }

        let response: Option<SecureConfiguration> = self
            .peer
            .call(
                "connectors.configure",
                params!({
                    "name": name.into(),
                    "secure": true
                }),
            )
            .await?;
        Ok(response.map(|response| response.url))
    }
}

impl Connector {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn meta(&self) -> &ConnectorMetadata {
        &self.meta
    }
    pub fn code(&self) -> &str {
        &self.code
    }
    pub fn runtime(&self) -> &ConnectorRuntime {
        &self.runtime
    }
    pub fn extension(&self) -> Option<&ConnectorExtension> {
        self.extension.as_ref()
    }
    pub fn is_configured(&self) -> bool {
        self.configured
    }
    pub async fn update(
        &mut self,
        update: ConnectorCreation,
        runtime: ConnectorRuntime,
    ) -> Result<()> {
        let peer = self.peer()?;
        let mut params = params!({
            "name": self.name.clone(),
            "runtime": runtime
        });

        params = match update {
            ConnectorCreation::Prompt { prompt, code, meta } => params
                .with("prompt", prompt)?
                .with("code", code)?
                .with("meta", meta)?,
            ConnectorCreation::Code { code, meta } => {
                params.with("code", code)?.with("meta", meta)?
            }
        };

        let name = self.name.clone();
        let mut connector: Connector = peer.call("connectors.update", params).await?;
        connector.peer = Some(peer);
        connector.name = name;
        *self = connector;

        Ok(())
    }

    pub async fn delete(self) -> Result<()> {
        self.peer()?
            .call("connectors.delete", params!({"name": self.name}))
            .await
    }

    pub async fn configure<C>(&mut self, config: C) -> Result<()>
    where
        C: Serialize,
    {
        let params = params!({
            "name": self.name.clone(),
            "config": config
        });
        self.peer()?.call::<()>("connectors.configure", params).await?;
        self.configured = true;
        Ok(())
    }

    fn peer(&self) -> Result<Arc<Peer>> {
        self.peer
            .clone()
            .ok_or_else(|| JsonRpcError::internal_error("connector is not attached to a peer"))
    }
}
