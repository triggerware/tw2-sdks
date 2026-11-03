use std::sync::Arc;

use tw2_jsonrpc::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExternalRuntime {
    Python,
    Node,
}

pub struct RuntimesService {
    peer: Arc<Peer>,
}

#[derive(Deserialize)]
struct RuntimeList {
    runtimes: Vec<ExternalRuntime>,
}

impl RuntimesService {
    pub fn new(peer: Arc<Peer>) -> RuntimesService {
        RuntimesService { peer }
    }

    pub async fn list(&self) -> Result<Vec<ExternalRuntime>> {
        self.peer
            .call::<RuntimeList>("runtimes.list", ())
            .await
            .map(|response| response.runtimes)
    }

    pub async fn install(&self, name: ExternalRuntime) -> Result<()> {
        self.peer
            .call("runtimes.install", params!({"name": name}))
            .await
    }

    pub async fn uninstall(&self, name: ExternalRuntime) -> Result<()> {
        self.peer
            .call("runtimes.uninstall", params!({"name": name}))
            .await
    }
}
