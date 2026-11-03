use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tw2_jsonrpc::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Configuration {
    pub default_timeout_ms: u64,
    pub default_batch_size: usize,
    pub default_trigger_batch_size: usize,
    pub agent_provider: String,
    pub agent_model: String,
    pub max_log_file_size: String,
}

#[derive(Default, Serialize, Params)]
#[serde_with::skip_serializing_none]
pub struct ConfigurationUpdate {
    pub default_timeout_ms: Option<u64>,
    pub default_batch_size: Option<usize>,
    pub default_trigger_batch_size: Option<usize>,
    pub agent_provider: Option<String>,
    pub agent_model: Option<String>,
    pub agent_api_key: Option<String>,
    pub max_log_file_size: Option<String>,
    pub secure: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum ConfigurationResult {
    Configuration(Configuration),
    Authorization { url: String },
}

pub struct ConfigurationService {
    peer: Arc<Peer>,
}

impl ConfigurationService {
    pub fn new(peer: Arc<Peer>) -> Self {
        Self { peer }
    }

    pub async fn get(&self) -> Result<Configuration> {
        self.peer.call("configuration.get", ()).await
    }

    pub async fn set(&self, options: ConfigurationUpdate) -> Result<ConfigurationResult> {
        self.peer.call("configuration.set", options).await
    }

    pub async fn refresh(&self) -> Result<Configuration> {
        self.peer.call("configuration.refresh", ()).await
    }
}
