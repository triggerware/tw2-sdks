use std::sync::Arc;

use tw2_jsonrpc::prelude::*;
use serde::Deserialize;

pub struct RuntimesService {
    peer: Arc<Peer>,
}

#[derive(Deserialize)]
struct RuntimeList {
    runtimes: Vec<String>,
}

impl RuntimesService {
    pub fn new(peer: Arc<Peer>) -> RuntimesService {
        RuntimesService { peer }
    }

    pub async fn list(&self) -> Result<Vec<String>> {
        self.peer
            .call::<RuntimeList>("runtimes.list", ())
            .await
            .map(|response| response.runtimes)
    }

    pub async fn install(&self, name: impl Into<String>) -> Result<()> {
        self.peer
            .call("runtimes.install", params!({"name": name.into()}))
            .await
    }

    pub async fn uninstall(&self, name: impl Into<String>) -> Result<()> {
        self.peer
            .call("runtimes.uninstall", params!({"name": name.into()}))
            .await
    }
}
