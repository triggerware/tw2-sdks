use std::sync::Arc;

use serde_json::Map;
use tw2_jsonrpc::prelude::*;

pub struct McpService {
    peer: Arc<Peer>,
}

impl McpService {
    pub fn new(peer: Arc<Peer>) -> Self {
        Self { peer }
    }

    pub async fn call(
        &self,
        method: impl Into<String>,
        params: Option<Map<String, Value>>,
    ) -> Result<Value> {
        let mut request = params!({"method": method.into()});
        if let Some(params) = params {
            request = request.with("params", params)?;
        }
        self.peer.call("mcp", request).await
    }
}
