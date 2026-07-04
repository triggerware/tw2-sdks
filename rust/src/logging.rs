use std::sync::Arc;

use tw2_jsonrpc::prelude::*;

pub struct LoggingService {
    peer: Arc<Peer>,
}

impl LoggingService {
    pub fn new(peer: Arc<Peer>) -> LoggingService {
        LoggingService { peer }
    }

    pub async fn set(&self, level: impl Into<String>) -> Result<()> {
        self.peer
            .call("logging.set", params!({"level": level.into()}))
            .await
    }
}
