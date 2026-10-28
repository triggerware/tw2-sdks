use std::fmt;
use std::str::FromStr;
use std::sync::Arc;

use tw2_jsonrpc::prelude::*;

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DefaultOnNull};

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct Table {
    #[serde(skip)]
    peer: Option<Arc<Peer>>,
    #[serde(skip_deserializing)]
    name: String,
    meta: TableMetadata,
    code: String,
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    runtime: crate::Runtime,
    connector: Option<TableConnector>,
    configured: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableConnector {
    pub name: String,
    pub tag: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct TableMetadata {
    pub description: String,
    pub columns: Vec<TableColumn>,
    pub generators: Vec<TableGenerator>,
    #[serde(default)]
    pub config_schema: Value,
}

impl TableMetadata {
    pub fn from_string(contents: impl AsRef<str>) -> Result<Self> {
        contents.as_ref().parse()
    }
}

impl FromStr for TableMetadata {
    type Err = JsonRpcError;

    fn from_str(contents: &str) -> Result<Self> {
        match serde_yaml::from_str(contents) {
            Ok(meta) => Ok(meta),
            Err(_) => serde_json::from_str(contents).map_err(|error| JsonRpcError {
                message: format!("invalid table metadata: {error}"),
                ..JsonRpcError::invalid_params()
            }),
        }
    }
}

impl fmt::Display for TableMetadata {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = serde_yaml::to_string(self).map_err(|_| fmt::Error)?;
        formatter.write_str(&value)
    }
}

#[derive(Serialize, Deserialize)]
pub struct TableColumn {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: String,
}

#[derive(Serialize, Deserialize)]
pub struct TableGenerator {
    pub name: String,
    pub inputs: Vec<String>,
}

pub struct TablesService {
    peer: Arc<Peer>,
}

#[derive(Default, Serialize, Params)]
#[serde_with::skip_serializing_none]
pub struct TablesSyncOptions {
    pub tables: Option<Vec<String>>,
    pub connectors: Option<Vec<String>>,
}

pub enum TableCreation {
    Prompt {
        prompt: String,
        code: Option<String>,
        meta: Option<TableMetadata>,
    },
    Code {
        code: String,
        meta: TableMetadata,
    },
}

impl TablesService {
    pub fn new(peer: Arc<Peer>) -> TablesService {
        TablesService { peer }
    }

    pub async fn create(
        &self,
        name: impl Into<String>,
        create: TableCreation,
        runtime: crate::Runtime,
    ) -> Result<Table> {
        let name = name.into();
        let mut params = params!()
            .with("name", name.clone())?
            .with("runtime", runtime)?;

        params = match create {
            TableCreation::Prompt { prompt, code, meta } => params
                .with("prompt", prompt)?
                .with("code", code)?
                .with("meta", meta)?,
            TableCreation::Code { code, meta } => {
                params.with("code", code)?.with("meta", meta)?
            }
        };

        let mut table: Table = self.peer.call("tables.create", params).await?;
        table.peer = Some(self.peer.clone());
        table.name = name;
        Ok(table)
    }

    pub async fn get(&self, name: impl Into<String>) -> Result<Table> {
        let name = name.into();
        let mut table: Table = self
            .peer
            .call("tables.get", params!({"name": name}))
            .await?;
        table.peer = Some(self.peer.clone());
        table.name = name;
        Ok(table)
    }

    pub async fn list(&self) -> Result<Vec<String>> {
        #[derive(Deserialize)]
        struct TableList {
            tables: Vec<String>,
        }
        let response: TableList = self.peer.call("tables.list", ()).await?;
        Ok(response.tables)
    }

    pub async fn sync(&self, options: TablesSyncOptions) -> Result<()> {
        self.peer.call("tables.sync", options).await
    }

    pub async fn configure<C>(&self, name: impl Into<String>, config: C) -> Result<()>
    where
        C: Serialize,
    {
        let params = params!({
            "name": name.into(),
            "config": config
        });
        self.peer.call::<()>("tables.configure", params).await
    }

    pub async fn configure_secure(&self, name: impl Into<String>) -> Result<Option<String>> {
        #[derive(Deserialize)]
        struct SecureConfiguration {
            url: String,
        }

        let response: Option<SecureConfiguration> = self
            .peer
            .call(
                "tables.configure",
                params!({
                    "name": name.into(),
                    "secure": true
                }),
            )
            .await?;
        Ok(response.map(|response| response.url))
    }
}

impl Table {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn meta(&self) -> &TableMetadata {
        &self.meta
    }
    pub fn code(&self) -> &str {
        &self.code
    }
    pub fn runtime(&self) -> &crate::Runtime {
        &self.runtime
    }
    pub fn connector(&self) -> Option<&TableConnector> {
        self.connector.as_ref()
    }
    pub fn is_configured(&self) -> bool {
        self.configured
    }
    pub async fn update(
        &mut self,
        update: TableCreation,
        runtime: crate::Runtime,
    ) -> Result<()> {
        let peer = self.peer()?;
        let mut params = params!({
            "name": self.name.clone(),
            "runtime": runtime
        });

        params = match update {
            TableCreation::Prompt { prompt, code, meta } => params
                .with("prompt", prompt)?
                .with("code", code)?
                .with("meta", meta)?,
            TableCreation::Code { code, meta } => {
                params.with("code", code)?.with("meta", meta)?
            }
        };

        let name = self.name.clone();
        let mut table: Table = peer.call("tables.update", params).await?;
        table.peer = Some(peer);
        table.name = name;
        *self = table;

        Ok(())
    }

    pub async fn delete(self) -> Result<()> {
        self.peer()?
            .call("tables.delete", params!({"name": self.name}))
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
        self.peer()?.call::<()>("tables.configure", params).await?;
        self.configured = true;
        Ok(())
    }

    fn peer(&self) -> Result<Arc<Peer>> {
        self.peer
            .clone()
            .ok_or_else(|| JsonRpcError::internal_error("table is not attached to a peer"))
    }
}
