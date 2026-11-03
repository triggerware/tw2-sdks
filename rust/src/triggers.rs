use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tw2_jsonrpc::prelude::*;

use crate::RunOptions;

/// Creation requires name, description, SQL, and schedule unless prompt is supplied.
/// Updates require name; omitted fields keep their current values.
#[derive(Default, Serialize, Params)]
#[serde_with::skip_serializing_none]
pub struct TriggerWrite {
    pub name: Option<String>,
    pub description: Option<String>,
    pub sql: Option<String>,
    pub prompt: Option<String>,
    pub schedule: Option<String>,
    pub report_noops: Option<bool>,
    pub enabled: Option<bool>,
    #[serde(flatten)]
    pub run: RunOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerSignature {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerDelta {
    pub added: Vec<Vec<Value>>,
    pub deleted: Vec<Vec<Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trigger {
    pub name: String,
    pub description: String,
    pub prompt: Option<String>,
    pub sql: String,
    pub schedule: Option<String>,
    pub batch_size: Option<usize>,
    pub timeout_ms: Option<u64>,
    pub report_noops: bool,
    pub enabled: bool,
    pub signature: Vec<TriggerSignature>,
}

pub struct TriggersService {
    peer: Arc<Peer>,
}

impl TriggersService {
    pub fn new(peer: Arc<Peer>) -> Self {
        Self { peer }
    }

    pub async fn create(&self, options: TriggerWrite) -> Result<Trigger> {
        if options.prompt.is_none()
            && (options.name.is_none() || options.description.is_none()
                || options.sql.is_none() || options.schedule.is_none())
        {
            return Err(JsonRpcError::invalid_params());
        }
        self.peer.call("triggers.create", options).await
    }

    pub async fn update(&self, options: TriggerWrite) -> Result<Trigger> {
        if options.name.is_none() {
            return Err(JsonRpcError::invalid_params());
        }
        self.peer.call("triggers.update", options).await
    }

    pub async fn get(&self, names: Vec<String>) -> Result<Vec<Trigger>> {
        #[derive(Deserialize)]
        struct TriggerList {
            triggers: Vec<Trigger>,
        }
        let response: TriggerList = self.peer.call("triggers.get", params!({"name": names})).await?;
        Ok(response.triggers)
    }

    pub async fn delete(&self, names: Vec<String>) -> Result<()> {
        self.peer.call("triggers.delete", params!({"name": names})).await
    }

    pub async fn list(&self) -> Result<Vec<String>> {
        #[derive(Deserialize)]
        struct TriggerList {
            triggers: Vec<String>,
        }
        let response: TriggerList = self.peer.call("triggers.list", ()).await?;
        Ok(response.triggers)
    }

    pub async fn enable(&self, name: impl Into<String>) -> Result<()> {
        self.peer.call("triggers.enable", params!({"name": name.into()})).await
    }

    pub async fn disable(&self, name: impl Into<String>) -> Result<()> {
        self.peer.call("triggers.disable", params!({"name": name.into()})).await
    }

    pub async fn run(&self, name: impl Into<String>, options: RunOptions) -> Result<TriggerDelta> {
        self.peer.call("triggers.run", params!({"name": name.into()}).extend(options)?).await
    }

    pub async fn poll(&self, name: impl Into<String>) -> Result<Vec<TriggerDelta>> {
        #[derive(Deserialize)]
        struct TriggerPoll {
            deltas: Vec<TriggerDelta>,
        }
        let response: TriggerPoll = self.peer.call("triggers.poll", params!({"name": name.into()})).await?;
        Ok(response.deltas)
    }

    pub async fn watch<F>(
        &self,
        name: impl Into<String>,
        resume_window_ms: Option<u64>,
        callback: F,
    ) -> Result<String>
    where
        F: Fn(Vec<Vec<Value>>, Vec<Vec<Value>>) + Send + Sync + 'static,
    {
        #[derive(Deserialize)]
        struct TriggerWatch {
            resume_token: String,
        }
        ensure_streaming(&self.peer)?;
        let name = name.into();
        register_watcher(&self.peer, name.clone(), callback);
        let mut params = params!({"name": name});
        if let Some(resume_window_ms) = resume_window_ms {
            params = params.with("resume_window_ms", resume_window_ms)?;
        }
        let response: TriggerWatch = self.peer.call("triggers.watch", params).await?;
        Ok(response.resume_token)
    }

    pub async fn resume<F>(
        &self,
        name: impl Into<String>,
        resume_token: impl Into<String>,
        resume_window_ms: Option<u64>,
        callback: F,
    ) -> Result<String>
    where
        F: Fn(Vec<Vec<Value>>, Vec<Vec<Value>>) + Send + Sync + 'static,
    {
        #[derive(Deserialize)]
        struct TriggerResume {
            resume_token: String,
        }
        ensure_streaming(&self.peer)?;
        let name = name.into();
        register_watcher(&self.peer, name.clone(), callback);
        let mut params = params!({"name": name, "resume_token": resume_token.into()});
        if let Some(resume_window_ms) = resume_window_ms {
            params = params.with("resume_window_ms", resume_window_ms)?;
        }
        let response: TriggerResume = self.peer.call("triggers.resume", params).await?;
        Ok(response.resume_token)
    }

    pub async fn unwatch(&self, name: Option<String>, resume_token: Option<String>) -> Result<()> {
        if name.is_none() && resume_token.is_none() {
            return Err(JsonRpcError::invalid_params());
        }
        let mut params = params!();
        if let Some(name) = name {
            params = params.with("name", name)?;
        }
        if let Some(resume_token) = resume_token {
            params = params.with("resume_token", resume_token)?;
        }
        self.peer.call("triggers.unwatch", params).await
    }
}

fn ensure_streaming(peer: &Peer) -> Result<()> {
    let metadata = peer.metadata();
    let metadata = metadata
        .read()
        .map_err(|_| JsonRpcError::internal_error("peer metadata is unavailable"))?;
    if metadata.get("transport").and_then(Value::as_str) == Some("http") {
        return Err(JsonRpcError::internal_error(
            "trigger watching requires a streaming connection",
        ));
    }
    Ok(())
}

fn register_watcher<F>(peer: &Arc<Peer>, name: String, callback: F)
where
    F: Fn(Vec<Vec<Value>>, Vec<Vec<Value>>) + Send + Sync + 'static,
{
    let callback = Arc::new(callback);
    peer.add_method(name, move |params| {
        let callback = callback.clone();
        async move {
            let deltas: Vec<TriggerDelta> = params.get("deltas")?;
            for delta in deltas {
                callback(delta.added, delta.deleted);
            }
            Ok(Value::Null)
        }
    });
}
