//! Integrated-agent responses and extensible event payloads.
use crate::sandbox::resources::PageInfo;

/// Prompt Response returned by Boat.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptResponse {
    /// Boat `ok` field.
    pub ok: bool,
    /// Boat `type` field.
    #[serde(rename = "type")]
    pub kind: String,
    /// Sandbox id.
    pub id: String,
    /// Boat `promptId` field.
    pub prompt_id: String,
    /// The conversation this prompt was queued in.
    pub conversation_id: Option<String>,
    /// Boat `promptRun` field.
    pub prompt_run: PromptRun,
    /// Boat `status` field.
    pub status: String,
    /// Boat `provider` field.
    pub provider: String,
    /// Boat `model` field.
    pub model: Option<String>,
    /// Boat `reasoningEffort` field.
    pub reasoning_effort: Option<String>,
    /// The prompt runs in fast mode.
    pub fast: Option<bool>,
}

/// Prompt Run returned by Boat.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptRun {
    /// Boat `id` field.
    pub id: String,
    /// Boat `promptId` field.
    pub prompt_id: String,
    /// Boat `sandboxId` field.
    pub sandbox_id: String,
    /// Boat `status` field.
    pub status: String,
    /// Boat `done` field.
    pub done: bool,
    /// Boat `createdAt` field.
    pub created_at: Option<String>,
    /// Boat `model` field.
    pub model: Option<String>,
    /// Boat `reasoningEffort` field.
    pub reasoning_effort: Option<String>,
    /// The prompt runs in fast mode.
    pub fast: Option<bool>,
    /// The conversation this prompt ran in.
    pub conversation_id: Option<String>,
}

/// Prompt Run Response returned by Boat.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptRunResponse {
    /// Boat `ok` field.
    pub ok: bool,
    /// Boat `type` field.
    #[serde(rename = "type")]
    pub kind: String,
    /// Boat `id` field.
    pub id: String,
    /// Boat `promptRun` field.
    pub prompt_run: PromptRun,
}

/// Steer Response returned by Boat.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SteerResponse {
    /// Boat `ok` field.
    pub ok: bool,
    /// Boat `type` field.
    #[serde(rename = "type")]
    pub kind: String,
    /// Sandbox id.
    pub id: String,
    /// The conversation whose running turn was steered.
    pub conversation_id: String,
    /// Id of the steer record.
    pub prompt_id: String,
    /// `true` when the harness accepted the message into the turn that was already running, so nothing was interrupted.
    pub native: bool,
    /// How the message was delivered.
    pub mode: Option<String>,
    /// Boat `status` field.
    pub status: String,
}

/// Conversation returned by Boat.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Conversation {
    /// Conversation id.
    pub id: String,
    /// When the first prompt of this conversation was queued.
    pub created_at: Option<String>,
    /// When its most recent prompt was queued.
    pub last_prompt_at: Option<String>,
    /// Number of prompts sent to this conversation.
    pub prompts: i64,
    /// `true` while a turn is queued or running in this conversation.
    pub running: bool,
    /// Harness the most recent prompt ran on (`claude`, `codex`, `pi`, `opencode`, `prime-agent`, `kimi`, `mistral`).
    pub last_harness: Option<String>,
    /// Model id of the most recent prompt that chose one, or `null` when every prompt used the harness default.
    pub last_model: Option<String>,
    /// The first 80 characters of the most recent prompt.
    pub last_prompt_preview: Option<String>,
    /// `true` on the conversation a `POST /prompt` without `new` or `conversationId` would continue: the sandbox's most recently prompted one.
    pub current: bool,
}

/// Conversations Response returned by Boat.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationsResponse {
    /// Boat `ok` field.
    pub ok: bool,
    /// Boat `type` field.
    #[serde(rename = "type")]
    pub kind: String,
    /// Sandbox id.
    pub id: String,
    /// Boat `conversations` field.
    pub conversations: Vec<Conversation>,
}

/// Sandbox Event returned by Boat.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SandboxEvent {
    /// Boat `id` field.
    pub id: Option<String>,
    /// Boat `type` field.
    #[serde(rename = "type")]
    pub kind: String,
    /// Boat `timestamp` field.
    pub timestamp: Option<i64>,
    /// Boat `taskId` field.
    pub task_id: Option<String>,
    /// The conversation this event belongs to.
    pub conversation_id: Option<String>,
    /// Boat `data` field.
    pub data: Option<serde_json::Map<String, serde_json::Value>>,
    /// Additional fields supplied by the harness.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// Events Response returned by Boat.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventsResponse {
    /// Boat `ok` field.
    pub ok: bool,
    /// Boat `type` field.
    #[serde(rename = "type")]
    pub kind: String,
    /// Boat `id` field.
    pub id: String,
    /// Sandbox work and lifecycle event objects.
    pub events: Vec<SandboxEvent>,
    /// Boat `pageInfo` field.
    pub page_info: Option<PageInfo>,
}
