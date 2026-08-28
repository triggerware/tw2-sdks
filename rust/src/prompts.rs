use std::sync::Arc;

use tw2_jsonrpc::prelude::*;
use serde::Deserialize;

pub struct PromptsService {
    peer: Arc<Peer>,
}

#[derive(Deserialize)]
struct PromptList {
    prompts: Vec<String>,
}

impl PromptsService {
    pub fn new(peer: Arc<Peer>) -> PromptsService {
        PromptsService { peer }
    }

    pub async fn list(&self) -> Result<Vec<String>> {
        self.peer
            .call::<PromptList>("prompts.list", ())
            .await
            .map(|response| response.prompts)
    }

    pub async fn get(&self, name: impl Into<String>) -> Result<String> {
        #[derive(Deserialize)]
        struct PromptResponse {
            prompt: String,
        }

        let response: PromptResponse = self
            .peer
            .call("prompts.get", params!({"name": name.into()}))
            .await?;
        Ok(response.prompt)
    }

    pub async fn set(&self, name: impl Into<String>, prompt: impl Into<String>) -> Result<()> {
        self.peer
            .call(
                "prompts.set",
                params!({
                    "name": name.into(),
                    "prompt": prompt.into(),
                }),
            )
            .await
    }

    pub async fn reset(&self, name: Option<impl Into<String>>) -> Result<()> {
        let params = match name {
            Some(name) => params!({"name": name.into()}),
            None => params!(),
        };

        self.peer.call("prompts.reset", params).await
    }
}
