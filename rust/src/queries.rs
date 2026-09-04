use std::sync::Arc;

use tw2_jsonrpc::{DynamicParams, prelude::*};
use serde::{Deserialize, Serialize};

use crate::prelude::RunOptions;

pub enum QueryCreation {
    Sql(String),
    Prompt(String),
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
    sql: Option<String>,
    prompt: Option<String>,
    batch_size: Option<u32>,
    timeout_ms: Option<u32>,
    signature: Vec<QuerySignature>,
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
    pub fn new(peer: Arc<Peer>) -> QueriesService {
        QueriesService { peer }
    }

    pub async fn create(
        &self,
        name: impl Into<String>,
        create: QueryCreation,
        config: RunOptions,
    ) -> Result<Query> {
        let name = name.into();
        let params = create.with_params(params!({"name": name.clone()}))?;

        let mut query: Query = self
            .peer
            .call("queries.create", config.with_params(params)?)
            .await?;
        query.peer = Some(self.peer.clone());
        Ok(query)
    }

    pub async fn get(&self, name: impl Into<String>) -> Result<Query> {
        let mut query: Query = self
            .peer
            .call("queries.get", params!({"name": name.into()}))
            .await?;
        query.peer = Some(self.peer.clone());
        Ok(query)
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
    pub fn sql(&self) -> Option<&str> {
        self.sql.as_deref()
    }
    pub fn prompt(&self) -> Option<&str> {
        self.prompt.as_deref()
    }
    pub fn batch_size(&self) -> Option<u32> {
        self.batch_size
    }
    pub fn timeout_ms(&self) -> Option<u32> {
        self.timeout_ms
    }
    pub fn signature(&self) -> &[QuerySignature] {
        &self.signature
    }
    pub fn is_exhausted(&self) -> bool {
        self.exhausted
    }
    pub async fn reset(&mut self) -> Result<()> {
        let peer = self.peer()?;

        let mut query: Query = peer
            .call("queries.reset", params!({"name": self.name.clone()}))
            .await?;
        query.peer = Some(peer);
        *self = query;

        Ok(())
    }

    pub async fn delete(self) -> Result<()> {
        self.peer()?
            .call("queries.delete", params!({"name": self.name}))
            .await
    }

    pub async fn fetch(&mut self) -> Result<Vec<Vec<Value>>> {
        self.fetch_with_rows(None).await
    }

    pub async fn fetch_rows(&mut self, rows: usize) -> Result<Vec<Vec<Value>>> {
        if rows == 0 {
            return Ok(vec![]);
        }
        self.fetch_with_rows(Some(rows)).await
    }

    async fn fetch_with_rows(&mut self, rows: Option<usize>) -> Result<Vec<Vec<Value>>> {
        #[derive(Deserialize)]
        struct QueryFetch {
            rows: Vec<Vec<Value>>,
            exhausted: bool,
        }

        if self.exhausted {
            return Ok(vec![]);
        }

        let mut params = params!({"name": self.name.clone()});
        if let Some(rows) = rows {
            params = params.with("rows", rows)?;
        }
        let fetch: QueryFetch = self.peer()?.call("queries.fetch", params).await?;
        self.exhausted = fetch.exhausted;
        self.cache.extend(fetch.rows.clone());
        Ok(fetch.rows)
    }

    pub async fn next(&mut self) -> Result<Option<Vec<Value>>> {
        if let Some(row) = self.cache.get(self.cache_position).cloned() {
            self.cache_position += 1;
            return Ok(Some(row));
        }

        if self.exhausted {
            return Ok(None);
        }

        self.fetch().await?;

        if let Some(row) = self.cache.get(self.cache_position).cloned() {
            self.cache_position += 1;
            return Ok(Some(row));
        }

        Ok(None)
    }

    fn peer(&self) -> Result<Arc<Peer>> {
        self.peer
            .clone()
            .ok_or_else(|| JsonRpcError::internal_error("query is not attached to a peer"))
    }
}

impl QueryCreation {
    fn with_params(self, params: DynamicParams) -> Result<DynamicParams> {
        match self {
            QueryCreation::Sql(sql) => params.with("sql", sql),
            QueryCreation::Prompt(prompt) => params.with("prompt", prompt),
        }
    }
}
