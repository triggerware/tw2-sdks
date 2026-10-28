use std::sync::Arc;

use tw2_jsonrpc::prelude::*;
use serde::{Deserialize, Serialize};

pub struct SystemService {
    peer: Arc<Peer>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Health {
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemUpdate {
    pub runtimes: Vec<String>,
    pub connector: String,
    pub artifact: String,
    pub restart_required: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct AuthorizationStatus {
    pub ok: bool,
}

impl SystemService {
    pub fn new(peer: Arc<Peer>) -> SystemService {
        SystemService { peer }
    }

    pub async fn health(&self) -> Result<Health> {
        self.peer.call("system.health", ()).await
    }

    pub async fn update(&self) -> Result<SystemUpdate> {
        self.peer.call("system.update", ()).await
    }

    pub async fn authorize(&self, api_key: impl Into<String>) -> Result<bool> {
        self.peer
            .call::<AuthorizationStatus>("system.authorize", params!({"api_key": api_key.into()}))
            .await
            .map(|x| x.ok)
    }

    pub async fn authorize_jwt(&self, jwt: impl Into<String>) -> Result<bool> {
        self.peer
            .call::<AuthorizationStatus>("system.authorize", params!({"jwt": jwt.into()}))
            .await
            .map(|x| x.ok)
    }

    pub async fn authorized(&self) -> Result<bool> {
        self.peer
            .call::<AuthorizationStatus>("system.authorized", ())
            .await
            .map(|x| x.ok)
    }
}
