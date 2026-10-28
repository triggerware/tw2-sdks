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
pub use agents::AgentWrite;
pub use tools::ToolWrite;
mod queries;
mod runtimes;
mod system;
mod triggers;

pub use configuration::TriggerwareConfiguration;
pub use types::Runtime;
pub use tables::{
    Table, TableColumn, TableCreation, TableConnector, TableGenerator,
    TableMetadata, TablesSyncOptions,
};
pub use cursors::{Cursor, CursorColumn, CursorCreation, RunOptions};
pub use connectors::{
    Connector, ConnectorAuthorization, ConnectorCatalogEntry, ConnectorDescription,
    ConnectorInstall, ConnectorReference,
};
pub use queries::{Query, QueryCreation, QuerySignature};
pub use system::{Health, SystemUpdate};
pub use triggers::{Trigger, TriggerCreation, TriggerDelta, TriggerOptions, TriggerSignature};

pub mod prelude {
    pub use tw2_jsonrpc::prelude::{Result, Value};

    pub use crate::configuration::TriggerwareConfiguration;
    pub use crate::tables::{TableCreation, TablesSyncOptions};
    pub use crate::Runtime;
    pub use crate::cursors::{CursorCreation, RunOptions};
    pub use crate::connectors::{ConnectorInstall, ConnectorReference};
    pub use crate::queries::QueryCreation;
    pub use crate::triggers::{TriggerCreation, TriggerOptions};
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
            queries: queries::QueriesService::new(peer.clone()),
            runtimes: runtimes::RuntimesService::new(peer.clone()),
            system: system::SystemService::new(peer.clone()),
            triggers: triggers::TriggersService::new(peer.clone()),
        }
    }
}
