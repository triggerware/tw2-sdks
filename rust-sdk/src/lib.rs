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

use configuration::ConfigurationService;
use connectors::ConnectorsService;
use cursors::CursorsService;
use extensions::ExtensionsService;
use prompts::PromptsService;
use queries::QueriesService;
use runtimes::RuntimesService;
use system::SystemService;
use triggers::TriggersService;

pub use connectors::Connector;
pub use cursors::{Cursor, CursorColumn};
pub use extensions::{Extension, ExtensionDescription};
pub use queries::{Query, QuerySignature};
pub use system::Health;
pub use triggers::{Trigger, TriggerDelta, TriggerSignature};

pub mod prelude {
    pub use rustsonrpc::prelude::{Result, Value};

    pub use crate::configuration::TriggerwareConfiguration;
    pub use crate::connectors::{
        ConnectorColumn, ConnectorCreation, ConnectorGenerator, ConnectorMetadata,
        ConnectorRuntime, ConnectorsSyncOptions,
    };
    pub use crate::cursors::{CursorCreation, RunOptions};
    pub use crate::extensions::{ExtensionInstall, ExtensionReference, ExtensionRuntime};
    pub use crate::queries::QueryCreation;
    pub use crate::runtimes::RuntimesService;
    pub use crate::triggers::{TriggerCreation, TriggerOptions};
    pub use crate::TriggerwareClient;
}

pub struct TriggerwareClient {
    pub configuration: ConfigurationService,
    pub connectors: ConnectorsService,
    pub cursors: CursorsService,
    pub extensions: ExtensionsService,
    pub prompts: PromptsService,
    pub queries: QueriesService,
    pub runtimes: RuntimesService,
    pub system: SystemService,
    pub triggers: TriggersService,
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
            configuration: ConfigurationService::new(peer.clone()),
            connectors: ConnectorsService::new(peer.clone()),
            cursors: CursorsService::new(peer.clone()),
            extensions: ExtensionsService::new(peer.clone()),
            prompts: PromptsService::new(peer.clone()),
            queries: QueriesService::new(peer.clone()),
            runtimes: RuntimesService::new(peer.clone()),
            system: SystemService::new(peer.clone()),
            triggers: TriggersService::new(peer.clone()),
        }
    }
}
