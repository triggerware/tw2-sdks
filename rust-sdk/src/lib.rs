use std::sync::Arc;

use rustsonrpc::prelude::*;

mod configuration;
mod connectors;
mod cursors;
mod extensions;
mod prompts;
mod queries;
mod runtimes;
mod system;
mod triggers;

pub use configuration::TriggerwareConfiguration;
pub use connectors::{
    Connector, ConnectorColumn, ConnectorCreation, ConnectorExtension, ConnectorGenerator,
    ConnectorMetadata, ConnectorRuntime, ConnectorsSyncOptions,
};
pub use cursors::{Cursor, CursorColumn, CursorCreation, RunOptions};
pub use extensions::{
    Extension, ExtensionCatalogEntry, ExtensionDescription, ExtensionInstall, ExtensionReference,
};
pub use queries::{Query, QueryCreation, QuerySignature};
pub use system::{Health, SystemUpdate};
pub use triggers::{Trigger, TriggerCreation, TriggerDelta, TriggerOptions, TriggerSignature};

pub mod prelude {
    pub use rustsonrpc::prelude::{Result, Value};

    pub use crate::configuration::TriggerwareConfiguration;
    pub use crate::connectors::{ConnectorCreation, ConnectorRuntime, ConnectorsSyncOptions};
    pub use crate::cursors::{CursorCreation, RunOptions};
    pub use crate::extensions::{ExtensionInstall, ExtensionReference};
    pub use crate::queries::QueryCreation;
    pub use crate::triggers::{TriggerCreation, TriggerOptions};
    pub use crate::TriggerwareClient;
}

pub struct TriggerwareClient {
    pub configuration: configuration::ConfigurationService,
    pub connectors: connectors::ConnectorsService,
    pub cursors: cursors::CursorsService,
    pub extensions: extensions::ExtensionsService,
    pub prompts: prompts::PromptsService,
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

    pub async fn connect_ws(url: &str, api_key: impl Into<String>) -> Result<TriggerwareClient> {
        let peer = PeerBuilder::new().connect_ws(url).await?;
        let client = Self::new(peer);
        client.system.authorize(api_key).await?;
        Ok(client)
    }

    pub fn new(peer: Peer) -> TriggerwareClient {
        let peer = Arc::new(peer);
        Self {
            configuration: configuration::ConfigurationService::new(peer.clone()),
            connectors: connectors::ConnectorsService::new(peer.clone()),
            cursors: cursors::CursorsService::new(peer.clone()),
            extensions: extensions::ExtensionsService::new(peer.clone()),
            prompts: prompts::PromptsService::new(peer.clone()),
            queries: queries::QueriesService::new(peer.clone()),
            runtimes: runtimes::RuntimesService::new(peer.clone()),
            system: system::SystemService::new(peer.clone()),
            triggers: triggers::TriggersService::new(peer.clone()),
        }
    }
}
