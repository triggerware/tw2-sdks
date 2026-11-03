use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tw2_jsonrpc::prelude::*;

use crate::RunOptions;

/// Name, description, and SQL are required unless prompt is supplied.
#[derive(Default, Serialize, Params)]
#[serde_with::skip_serializing_none]
pub struct QueryWrite {
    pub name: Option<String>,
    pub description: Option<String>,
    pub sql: Option<String>,
    pub prompt: Option<String>,
    pub params: Option<Vec<QuerySignature>>,
    #[serde(flatten)]
    pub run: RunOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryParameterValue {
    pub name: String,
    pub value: Value,
}

#[derive(Clone, Default, Serialize, Params)]
#[serde_with::skip_serializing_none]
pub struct QueryFetchOptions {
    pub params: Option<Vec<QueryParameterValue>>,
    pub rows: Option<usize>,
}

#[derive(Default, Serialize, Params)]
#[serde_with::skip_serializing_none]
pub struct QueryConvertOptions {
    pub name: String,
    pub trigger_name: Option<String>,
    pub prompt: Option<String>,
    pub params: Option<Vec<QueryParameterValue>>,
    pub schedule: Option<String>,
    pub batch_size: Option<usize>,
    pub timeout_ms: Option<u64>,
    pub report_noops: Option<bool>,
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryConvertResult {
    pub rows: Vec<Vec<Value>>,
    pub exhausted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuerySignature {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: String,
}

#[derive(Serialize, Deserialize)]
pub struct Query {
    #[serde(skip)]
    peer: Option<Arc<Peer>>,
    name: String,
    description: String,
    sql: Option<String>,
    prompt: Option<String>,
    batch_size: Option<usize>,
    timeout_ms: Option<u64>,
    params: Option<Vec<QuerySignature>>,
    signature: Vec<QuerySignature>,
    #[serde(skip)]
    fetch_options: QueryFetchOptions,
    #[serde(skip)]
    cache: Vec<Vec<Value>>,
    #[serde(skip)]
    cache_position: usize,
    #[serde(skip)]
    exhausted: bool,
}

pub struct QueriesService {
    peer: Arc<Peer>,
}

impl QueriesService {
    pub fn new(peer: Arc<Peer>) -> Self {
        Self { peer }
    }

    pub async fn create(&self, options: QueryWrite) -> Result<Query> {
        if options.prompt.is_none()
            && (options.name.is_none() || options.description.is_none() || options.sql.is_none())
        {
            return Err(JsonRpcError::invalid_params());
        }
        let mut query: Query = self.peer.call("queries.create", options).await?;
        query.peer = Some(self.peer.clone());
        Ok(query)
    }

    pub async fn convert(&self, options: QueryConvertOptions) -> Result<QueryConvertResult> {
        self.peer.call("queries.convert", options).await
    }

    pub async fn get(&self, names: Vec<String>) -> Result<Vec<Query>> {
        #[derive(Deserialize)]
        struct QueryList {
            queries: Vec<Query>,
        }
        let mut response: QueryList = self.peer.call("queries.get", params!({"name": names})).await?;
        for query in &mut response.queries {
            query.peer = Some(self.peer.clone());
        }
        Ok(response.queries)
    }

    pub async fn delete(&self, names: Vec<String>) -> Result<()> {
        self.peer.call("queries.delete", params!({"name": names})).await
    }

    pub async fn list(&self) -> Result<Vec<String>> {
        #[derive(Deserialize)]
        struct QueryList {
            queries: Vec<String>,
        }
        let response: QueryList = self.peer.call("queries.list", ()).await?;
        Ok(response.queries)
    }
}

impl Query {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn params(&self) -> Option<&[QuerySignature]> {
        self.params.as_deref()
    }
    pub fn sql(&self) -> Option<&str> {
        self.sql.as_deref()
    }
    pub fn prompt(&self) -> Option<&str> {
        self.prompt.as_deref()
    }
    pub fn batch_size(&self) -> Option<usize> {
        self.batch_size
    }
    pub fn timeout_ms(&self) -> Option<u64> {
        self.timeout_ms
    }
    pub fn signature(&self) -> &[QuerySignature] {
        &self.signature
    }
    pub fn is_exhausted(&self) -> bool {
        self.exhausted
    }

    /// The resource documentation's prose specifies the current query definition.
    pub async fn reset(&mut self) -> Result<()> {
        let peer = self.peer()?;
        let mut query: Query = peer.call("queries.reset", params!({"name": self.name.clone()})).await?;
        query.peer = Some(peer);
        query.fetch_options = self.fetch_options.clone();
        *self = query;
        Ok(())
    }

    /// Subsequent next() calls reuse these parameters and row limit.
    pub async fn fetch(&mut self, options: QueryFetchOptions) -> Result<Vec<Vec<Value>>> {
        #[derive(Deserialize)]
        struct QueryFetch {
            rows: Vec<Vec<Value>>,
            exhausted: bool,
        }

        if self.exhausted {
            return Ok(vec![]);
        }

        let params = params!({"name": self.name.clone()}).extend(options.clone())?;
        let response: QueryFetch = self.peer()?.call("queries.fetch", params).await?;
        self.fetch_options = options;
        self.exhausted = response.exhausted;
        self.cache.extend(response.rows.clone());
        Ok(response.rows)
    }

    pub async fn next(&mut self) -> Result<Option<Vec<Value>>> {
        loop {
            if let Some(row) = self.cache.get(self.cache_position).cloned() {
                self.cache_position += 1;
                return Ok(Some(row));
            }
            if self.exhausted {
                return Ok(None);
            }
            self.fetch(self.fetch_options.clone()).await?;
        }
    }

    fn peer(&self) -> Result<Arc<Peer>> {
        self.peer
            .clone()
            .ok_or_else(|| JsonRpcError::internal_error("query is not attached to a peer"))
    }
}
