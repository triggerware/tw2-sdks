use std::sync::Arc;

use tw2_jsonrpc::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Default, Serialize, Deserialize, Params)]
#[serde_with::skip_serializing_none]
pub struct RunOptions {
    pub batch_size: Option<usize>,
    pub timeout_ms: Option<u64>,
}

#[derive(Default, Serialize, Params)]
#[serde_with::skip_serializing_none]
pub struct CursorOpenOptions {
    pub sql: Option<String>,
    pub prompt: Option<String>,
    #[serde(flatten)]
    pub run: RunOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CursorColumn {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: String,
}

#[derive(Deserialize)]
pub struct Cursor {
    #[serde(skip)]
    peer: Option<Arc<Peer>>,
    #[serde(rename = "cursor")]
    id: String,
    sql: String,
    signature: Vec<CursorColumn>,
    #[serde(rename = "rows")]
    cache: Vec<Vec<Value>>,
    #[serde(skip)]
    cache_position: usize,
    exhausted: bool,
}

pub struct CursorsService {
    peer: Arc<Peer>,
}

impl CursorsService {
    pub fn new(peer: Arc<Peer>) -> CursorsService {
        CursorsService { peer }
    }

    pub async fn open(&self, options: CursorOpenOptions) -> Result<Cursor> {
        if options.sql.is_some() == options.prompt.is_some() {
            return Err(JsonRpcError::invalid_params());
        }
        let mut cursor: Cursor = self.peer.call("cursors.open", options).await?;
        cursor.peer = Some(self.peer.clone());
        Ok(cursor)
    }
}

impl Cursor {
    pub fn sql(&self) -> &str {
        &self.sql
    }
    pub fn signature(&self) -> &[CursorColumn] {
        &self.signature
    }
    pub fn is_exhausted(&self) -> bool {
        self.exhausted
    }
    pub async fn fetch(&mut self) -> Result<Vec<Vec<Value>>> {
        #[derive(Debug, Clone, Deserialize)]
        struct CursorFetch {
            rows: Vec<Vec<Value>>,
            exhausted: bool,
        }

        if self.exhausted {
            return Ok(vec![]);
        }
        let fetch: CursorFetch = self
            .peer()?
            .call("cursors.fetch", params!({"cursor": self.id}))
            .await?;

        self.exhausted = fetch.exhausted;
        self.cache.extend(fetch.rows.clone());
        Ok(fetch.rows)
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
            self.fetch().await?;
        }
    }

    pub async fn close(&mut self) -> Result<()> {
        self.peer()?
            .call::<()>("cursors.close", params!({"cursor": self.id}))
            .await?;
        self.exhausted = true;
        Ok(())
    }

    fn peer(&self) -> Result<Arc<Peer>> {
        self.peer
            .clone()
            .ok_or_else(|| JsonRpcError::internal_error("cursor is not attached to a peer"))
    }
}
