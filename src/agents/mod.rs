//! Boat's integrated agent harnesses. Queue a prompt, follow its conversation's
//! events, and check `promptRun.done` for completion. Sandbox state is independent
//! of individual turns, which can run concurrently.

mod catalogue;
mod events;
pub mod models;

pub use catalogue::provider_models;
pub use catalogue::HarnessModels;
pub use catalogue::ProviderCatalogue;
pub use catalogue::ProviderModel;
pub use catalogue::ReasoningEffort;
pub use events::EventOrder;
pub use events::EventQuery;
pub use events::EventStream;

use models::{ConversationsResponse, PromptResponse, PromptRunResponse, SteerResponse};
use reqwest::Method;
use serde::Serialize;

use crate::sandbox::responses::SandboxActionResponse;
use crate::{Client, Error};

/// A task for an integrated harness. Omitting `provider` and `model` uses the
/// dashboard selections. Model IDs and reasoning levels are harness-specific.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptRequest {
    /// Natural-language task.
    pub prompt: String,
    /// Harness name, e.g. `codex`, `claude`, or `opencode`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    /// Boat model ID for the selected harness: [`ProviderModel::id`], not its SDK ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Optional model-specific reasoning level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_effort: Option<String>,
    /// Force fast or standard speed, if supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fast: Option<bool>,
    /// Set `true` for a fresh conversation. Mutually exclusive with `conversation_id`.
    #[serde(rename = "new", skip_serializing_if = "Option::is_none")]
    pub new_conversation: Option<bool>,
    /// Continue this conversation; otherwise Boat continues the most recently active one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
}

impl PromptRequest {
    /// Prompt the default harness in the most recently active conversation.
    pub fn new(prompt: impl Into<String>) -> Self {
        Self {
            prompt: prompt.into(),
            provider: None,
            model: None,
            reasoning_effort: None,
            fast: None,
            new_conversation: None,
            conversation_id: None,
        }
    }
}

impl Client {
    /// Queue one turn. The response identifies both the prompt run and its conversation.
    pub async fn prompt(&self, id: &str, body: &PromptRequest) -> Result<PromptResponse, Error> {
        if body.new_conversation == Some(true) && body.conversation_id.is_some() {
            return Err(Error::Request);
        }
        self.post_json(&["sandboxes", id, "prompt"], body).await
    }

    /// Read one turn's status, including the authoritative completion flag.
    pub async fn prompt_status(
        &self,
        id: &str,
        prompt_id: &str,
    ) -> Result<PromptRunResponse, Error> {
        self.get_json(&["sandboxes", id, "prompts", prompt_id], &())
            .await
    }

    /// List the sandbox's conversations, most recently prompted first.
    pub async fn conversations(&self, id: &str) -> Result<ConversationsResponse, Error> {
        self.get_json(&["sandboxes", id, "conversations"], &())
            .await
    }

    /// Send an instruction to an active turn. `None` targets the most recently active conversation.
    pub async fn steer(
        &self,
        id: &str,
        conversation: Option<&str>,
        message: &str,
    ) -> Result<SteerResponse, Error> {
        #[derive(Serialize)]
        struct Body<'a> {
            message: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            conversation: Option<&'a str>,
        }
        self.post_json(
            &["sandboxes", id, "steer"],
            &Body {
                message,
                conversation,
            },
        )
        .await
    }

    /// Interrupt one conversation, or **all** conversations if `conversation` is `None`.
    pub async fn interrupt(
        &self,
        id: &str,
        conversation: Option<&str>,
    ) -> Result<SandboxActionResponse, Error> {
        Self::send(
            self.request(Method::POST, &["sandboxes", id, "interrupt"])?
                .query(&[("conversation", conversation)]),
        )
        .await
    }
}
