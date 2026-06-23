use std::sync::Arc;

use rustsonrpc::prelude::*;
use serde::{Deserialize, Serialize};

use crate::prelude::RunOptions;

pub enum TriggerCreation {
    Sql(String),
    Prompt(String),
}

#[derive(Clone, Default, Serialize, Deserialize, Params)]
#[serde_with::skip_serializing_none]
pub struct TriggerOptions {
    pub schedule: Option<String>,
    pub delivery: Option<Vec<String>>,
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

#[derive(Serialize, Deserialize)]
pub struct Trigger {
    #[serde(skip)]
    peer: Option<Arc<Peer>>,
    name: String,
    prompt: Option<String>,
    sql: Option<String>,
    signature: Vec<TriggerSignature>,
    #[serde(flatten)]
    options: TriggerOptions,
    #[serde(skip)]
    resume_token: Option<String>,
}

pub struct TriggersService {
    peer: Arc<Peer>,
}

impl TriggersService {
    pub fn new(peer: Arc<Peer>) -> TriggersService {
        TriggersService { peer }
    }

    pub async fn create(
        &self,
        name: impl Into<String>,
        create: TriggerCreation,
        options: TriggerOptions,
    ) -> Result<Trigger> {
        let name = name.into();
        let params = create.with_params(params!({"name": name.clone()}))?;

        let mut trigger: Trigger = self
            .peer
            .call("triggers.create", params.extend(options)?)
            .await?;
        trigger.peer = Some(self.peer.clone());
        Ok(trigger)
    }

    pub async fn get(&self, name: impl Into<String>) -> Result<Trigger> {
        let mut trigger: Trigger = self
            .peer
            .call("triggers.get", params!({"name": name.into()}))
            .await?;
        trigger.peer = Some(self.peer.clone());
        Ok(trigger)
    }

    pub async fn list(&self) -> Result<Vec<String>> {
        #[derive(Deserialize)]
        struct TriggerList {
            triggers: Vec<String>,
        }

        let response: TriggerList = self.peer.call("triggers.list", ()).await?;
        Ok(response.triggers)
    }
}

impl Trigger {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn prompt(&self) -> Option<&str> {
        self.prompt.as_deref()
    }
    pub fn sql(&self) -> Option<&str> {
        self.sql.as_deref()
    }
    pub fn options(&self) -> TriggerOptions {
        self.options.clone()
    }
    pub fn is_enabled(&self) -> bool {
        self.options.enabled.unwrap_or(false)
    }
    pub fn signature(&self) -> &[TriggerSignature] {
        &self.signature
    }
    pub fn is_watched(&self) -> bool {
        self.resume_token.is_some()
    }

    pub async fn update(
        &mut self,
        update: Option<TriggerCreation>,
        options: TriggerOptions,
    ) -> Result<()> {
        let peer = self.peer()?;
        let mut params = params!({"name": self.name.clone()});

        if let Some(update) = update {
            params = update.with_params(params)?;
        }

        let resume_token = self.resume_token.clone();
        let mut trigger: Trigger = peer.call("triggers.update", params.extend(options)?).await?;
        trigger.peer = Some(peer);
        trigger.resume_token = resume_token;
        *self = trigger;
        Ok(())
    }

    pub async fn delete(self) -> Result<()> {
        self.peer()?
            .call("triggers.delete", params!({"name": self.name}))
            .await
    }

    pub async fn enable(&mut self) -> Result<()> {
        self.peer()?
            .call::<()>("triggers.enable", params!({"name": self.name.clone()}))
            .await?;
        self.options.enabled = Some(true);
        Ok(())
    }

    pub async fn disable(&mut self) -> Result<()> {
        self.peer()?
            .call::<()>("triggers.disable", params!({"name": self.name.clone()}))
            .await?;
        self.options.enabled = Some(false);
        Ok(())
    }

    pub async fn run(&self, options: RunOptions) -> Result<TriggerDelta> {
        self.peer()?
            .call("triggers.run", params!({"name": self.name.clone()}).extend(options)?)
            .await
    }

    pub async fn poll(&self) -> Result<Vec<TriggerDelta>> {
        #[derive(Deserialize)]
        struct TriggerPoll {
            deltas: Vec<TriggerDelta>,
        }

        let response: TriggerPoll = self
            .peer()?
            .call("triggers.poll", params!({"name": self.name.clone()}))
            .await?;
        Ok(response.deltas)
    }

    pub async fn watch<F>(&mut self, resume_window_ms: Option<u32>, callback: F) -> Result<String>
    where
        F: Fn(Vec<Vec<Value>>, Vec<Vec<Value>>) + Send + Sync + 'static,
    {
        #[derive(Deserialize)]
        struct TriggerWatch {
            resume_token: String,
        }

        if self.resume_token.is_some() {
            return Err(JsonRpcError::internal_error("trigger is already watching"));
        }

        let peer = self.peer()?;
        let name = self.name.clone();
        register_watcher(&peer, name.clone(), callback);

        let mut params = params!({"name": name});

        if let Some(resume_window_ms) = resume_window_ms {
            params = params.with("resume_window_ms", resume_window_ms)?;
        }

        let response: TriggerWatch = peer.call("triggers.watch", params).await?;

        self.resume_token = Some(response.resume_token.clone());

        Ok(response.resume_token)
    }

    pub async fn resume<F>(
        &mut self,
        resume_token: impl Into<String>,
        resume_window_ms: Option<u32>,
        callback: F,
    ) -> Result<String>
    where
        F: Fn(Vec<Vec<Value>>, Vec<Vec<Value>>) + Send + Sync + 'static,
    {
        #[derive(Deserialize)]
        struct TriggerResume {
            resume_token: String,
        }

        if self.resume_token.is_some() {
            return Err(JsonRpcError::internal_error("trigger is already watching"));
        }

        let peer = self.peer()?;
        let name = self.name.clone();
        register_watcher(&peer, name.clone(), callback);

        let mut params = params!({
            "name": name,
            "resume_token": resume_token.into(),
        });

        if let Some(resume_window_ms) = resume_window_ms {
            params = params.with("resume_window_ms", resume_window_ms)?;
        }

        let response: TriggerResume = peer.call("triggers.resume", params).await?;

        self.resume_token = Some(response.resume_token.clone());

        Ok(response.resume_token)
    }

    pub async fn unwatch(&mut self) -> Result<()> {
        let Some(resume_token) = self.resume_token.clone() else {
            return Ok(());
        };

        self.peer()?
            .call::<()>("triggers.unwatch", params!({"resume_token": resume_token}))
            .await?;

        self.resume_token = None;

        Ok(())
    }

    async fn refresh(&mut self, peer: Arc<Peer>) -> Result<()> {
        let resume_token = self.resume_token.clone();
        let mut trigger: Trigger = peer
            .call("triggers.get", params!({"name": self.name.clone()}))
            .await?;
        trigger.peer = Some(peer);
        trigger.resume_token = resume_token;
        *self = trigger;
        Ok(())
    }

    fn peer(&self) -> Result<Arc<Peer>> {
        self.peer
            .clone()
            .ok_or_else(|| JsonRpcError::internal_error("trigger is not attached to a peer"))
    }
}

impl TriggerCreation {
    fn with_params(self, params: DynamicParams) -> Result<DynamicParams> {
        match self {
            TriggerCreation::Sql(sql) => params.with("sql", sql),
            TriggerCreation::Prompt(prompt) => params.with("prompt", prompt),
        }
    }
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
