use std::sync::Arc;

use rustsonrpc::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Params)]
#[serde_with::skip_serializing_none]
pub struct TriggerwareConfiguration {
  pub default_timeout_ms: Option<u64>,
  pub default_batch_size: Option<u64>,
  pub default_trigger_batch_size: Option<u64>,
  pub llm_provider: Option<String>,
  pub llm_model: Option<String>,
  pub llm_api_key: Option<String>,
  pub max_log_file_size: Option<String>,
}

pub struct ConfigurationService {
    peer: Arc<Peer>,
}

impl ConfigurationService {
    pub fn new(peer: Arc<Peer>) -> ConfigurationService {
        ConfigurationService { peer }
    }

    pub async fn get(&self) -> Result<TriggerwareConfiguration> {
        self.peer.call("configuration.get", params!()).await
    }
    pub async fn set(&self, config: TriggerwareConfiguration) -> Result<TriggerwareConfiguration> {
        self.peer.call("configuration.set", config).await
    }
    pub async fn refresh(&self) -> Result<TriggerwareConfiguration> {
        self.peer.call("configuration.refresh", params!()).await
    }
}
