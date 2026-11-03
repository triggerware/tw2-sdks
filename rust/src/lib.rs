use std::sync::Arc;

use tw2_jsonrpc::prelude::*;

mod configuration;
mod types;
mod tables;
mod cursors;
mod connectors;
mod logging;
mod agents;
mod tools;
mod mcp;
pub use agents::{Agent, AgentMetadata, AgentWrite};
pub use tools::{Tool, ToolMetadata, ToolParameter, ToolWrite};
mod queries;
mod runtimes;
mod system;
mod triggers;

pub use configuration::{Configuration, ConfigurationResult, ConfigurationUpdate};
pub use types::{AuthorizationResult, ResourceConfiguration, Runtime};
pub use tables::{
    Table, TableColumn, TableConnector, TableGenerator,
    TableMetadata, TablesSyncOptions, TableWrite,
};
pub use cursors::{Cursor, CursorColumn, CursorOpenOptions, RunOptions};
pub use connectors::{
    Connector, ConnectorCatalogEntry, ConnectorConfiguration, ConnectorDescription,
    ConnectorInstall, ConnectorReference,
};
pub use queries::{
    Query, QueryConvertOptions, QueryConvertResult, QueryFetchOptions, QueryParameterValue,
    QuerySignature, QueryWrite,
};
pub use logging::LogLevel;
pub use runtimes::ExternalRuntime;
pub use system::{Health, SystemUpdate};
pub use triggers::{
    Trigger, TriggerDelta, TriggerSignature, TriggerWrite,
};

pub mod prelude {
    pub use tw2_jsonrpc::prelude::{Result, Value};

    pub use crate::configuration::{ConfigurationResult, ConfigurationUpdate};
    pub use crate::agents::{AgentMetadata, AgentWrite};
    pub use crate::tools::{ToolMetadata, ToolParameter, ToolWrite};
    pub use crate::tables::{TablesSyncOptions, TableWrite};
    pub use crate::{ExternalRuntime, LogLevel, ResourceConfiguration, Runtime};
    pub use crate::cursors::{CursorOpenOptions, RunOptions};
    pub use crate::connectors::{ConnectorConfiguration, ConnectorInstall, ConnectorReference};
    pub use crate::queries::{QueryConvertOptions, QueryFetchOptions, QueryParameterValue, QueryWrite};
    pub use crate::triggers::TriggerWrite;
    pub use crate::TriggerwareClient;
}

pub struct TriggerwareClient {
    pub configuration: configuration::ConfigurationService,
    pub tables: tables::TablesService,
    pub cursors: cursors::CursorsService,
    pub connectors: connectors::ConnectorsService,
    pub logging: logging::LoggingService,
    pub agents: agents::AgentsService,
    pub tools: tools::ToolsService,
    pub mcp: mcp::McpService,
    pub queries: queries::QueriesService,
    pub runtimes: runtimes::RuntimesService,
    pub system: system::SystemService,
    pub triggers: triggers::TriggersService,
}

impl TriggerwareClient {
    pub async fn connect_tcp(addr: impl tokio::net::ToSocketAddrs) -> Result<TriggerwareClient> {
        let peer = PeerBuilder::new().connect_tcp(addr).await?;
        Ok(Self::new(peer))
    }

    pub async fn connect_http(
        url: &str,
        bearer_token: impl Into<String>,
    ) -> Result<TriggerwareClient> {
        let peer = PeerBuilder::new()
            .connect_http(url, bearer_token)
            .await?;
        Ok(Self::new(peer))
    }

    pub fn new(peer: Peer) -> TriggerwareClient {
        let peer = Arc::new(peer);
        Self {
            configuration: configuration::ConfigurationService::new(peer.clone()),
            tables: tables::TablesService::new(peer.clone()),
            cursors: cursors::CursorsService::new(peer.clone()),
            connectors: connectors::ConnectorsService::new(peer.clone()),
            logging: logging::LoggingService::new(peer.clone()),
            agents: agents::AgentsService::new(peer.clone()),
            tools: tools::ToolsService::new(peer.clone()),
            mcp: mcp::McpService::new(peer.clone()),
            queries: queries::QueriesService::new(peer.clone()),
            runtimes: runtimes::RuntimesService::new(peer.clone()),
            system: system::SystemService::new(peer.clone()),
            triggers: triggers::TriggersService::new(peer.clone()),
        }
    }
}
