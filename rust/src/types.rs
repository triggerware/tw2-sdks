use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use tw2_jsonrpc::prelude::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthorizationResult {
    pub url: Option<String>,
}

#[derive(Default, Serialize, Params)]
#[serde_with::skip_serializing_none]
pub struct ResourceConfiguration {
    pub name: String,
    pub config: Option<Map<String, Value>>,
    pub secure: Option<bool>,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Runtime {
    #[default]
    Lua,
    Python,
    Node,
}
