use std::fmt;
use std::str::FromStr;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tw2_jsonrpc::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Table {
    pub name: String,
    pub prompt: Option<String>,
    pub meta: TableMetadata,
    pub code: String,
    pub runtime: crate::Runtime,
    pub connector: Option<TableConnector>,
    pub configured: bool,
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

#[derive(Default, Serialize, Params)]
#[serde_with::skip_serializing_none]
pub struct TablesSyncOptions {
    pub tables: Option<Vec<String>>,
    pub connectors: Option<Vec<String>>,
}

/// Creation requires name, code, and meta unless prompt is supplied.
/// Updates require name; omitted fields keep their current values.
#[derive(Default, Serialize, Params)]
#[serde_with::skip_serializing_none]
pub struct TableWrite {
    pub name: Option<String>,
    pub prompt: Option<String>,
    pub code: Option<String>,
    pub meta: Option<TableMetadata>,
    pub runtime: Option<crate::Runtime>,
}

pub struct TablesService {
    peer: Arc<Peer>,
}

impl TablesService {
    pub fn new(peer: Arc<Peer>) -> Self {
        Self { peer }
    }

    pub async fn create(&self, options: TableWrite) -> Result<Table> {
        if options.prompt.is_none()
            && (options.name.is_none() || options.code.is_none() || options.meta.is_none())
        {
            return Err(JsonRpcError::invalid_params());
        }
        self.peer.call("tables.create", options).await
    }

    pub async fn update(&self, options: TableWrite) -> Result<Table> {
        if options.name.is_none() {
            return Err(JsonRpcError::invalid_params());
        }
        self.peer.call("tables.update", options).await
    }

    pub async fn get(&self, names: Vec<String>) -> Result<Vec<Table>> {
        #[derive(Deserialize)]
        struct TableList {
            tables: Vec<Table>,
        }
        let response: TableList = self.peer.call("tables.get", params!({"name": names})).await?;
        Ok(response.tables)
    }

    pub async fn delete(&self, names: Vec<String>) -> Result<()> {
        self.peer.call("tables.delete", params!({"name": names})).await
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

    pub async fn configure(&self, options: crate::ResourceConfiguration) -> Result<crate::AuthorizationResult> {
        self.peer.call("tables.configure", options).await
    }
}
