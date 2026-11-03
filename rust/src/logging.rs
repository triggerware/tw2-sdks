use std::sync::Arc;

use tw2_jsonrpc::prelude::*;
use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Debug,
    Info,
    Error,
}

pub struct LoggingService {
    peer: Arc<Peer>,
}

impl LoggingService {
    pub fn new(peer: Arc<Peer>) -> LoggingService {
        LoggingService { peer }
    }

    pub async fn set(&self, level: LogLevel) -> Result<()> {
        self.peer
            .call("logging.set", params!({"level": level}))
            .await
    }
}
