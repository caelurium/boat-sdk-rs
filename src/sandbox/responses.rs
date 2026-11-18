use super::resources::{DeletionOperation, PageInfo, Sandbox, StopOperation};

/// Hosted Port Result in the Boat API.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HostedPortResult {
    /// Boat `port` field.
    pub port: i64,
    /// Public HTTPS URL.
    pub url: Option<String>,
    /// Boat `isProtected` field.
    pub is_protected: Option<bool>,
    /// Boat `access` field.
    pub access: Option<String>,
    /// Boat `localhost` field.
    pub localhost: Option<bool>,
    /// Boat `error` field.
    pub error: Option<String>,
    /// Boat `message` field.
    pub message: Option<String>,
}

/// Create Sandbox Response in the Boat API.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSandboxResponse {
    /// Boat `ok` field.
    pub ok: bool,
    /// Boat `type` field.
    #[serde(rename = "type")]
    pub kind: String,
    /// `ready` only when an idempotent retry finds a sandbox that is already past provisioning.
    pub status: String,
    /// Boat `ttlSeconds` field.
    pub ttl_seconds: Option<i64>,
    /// Boat `sandbox` field.
    pub sandbox: Sandbox,
    /// Present when the request set `count` above 1.
    pub sandboxes: Option<Vec<Sandbox>>,
    /// Present when the request set `hostedPorts`, in request order.
    pub hosted_ports: Option<Vec<HostedPortResult>>,
    /// Present when a fair-use cap on a gifted account shortened the requested auto-stop time.
    pub gift_limit_notice: Option<String>,
}

/// Sandbox Info Response in the Boat API.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SandboxInfoResponse {
    /// Boat `ok` field.
    pub ok: bool,
    /// Boat `type` field.
    #[serde(rename = "type")]
    pub kind: String,
    /// Boat `sandbox` field.
    pub sandbox: Sandbox,
    /// Present when a fair-use cap on a gifted account shortened the requested auto-stop time.
    pub gift_limit_notice: Option<String>,
}

/// Deleted Cost in the Boat API.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeletedCost {
    /// The 1st of this month, 00:00 UTC.
    pub since: String,
    /// Billable machine-seconds, size multiplier applied.
    pub seconds: i64,
    /// seconds / secondsPerDollar, at list price.
    pub dollars: f64,
}

/// Sandbox List Response in the Boat API.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SandboxListResponse {
    /// Boat `ok` field.
    pub ok: bool,
    /// Boat `type` field.
    #[serde(rename = "type")]
    pub kind: String,
    /// Boat `sandboxes` field.
    pub sandboxes: Vec<Sandbox>,
    /// Boat `pageInfo` field.
    pub page_info: Option<PageInfo>,
    /// Only with `include=deletedCost`, and not for an API key limited to some sandboxes.
    pub deleted_cost: Option<DeletedCost>,
}

/// Sandbox Action Response in the Boat API.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SandboxActionResponse {
    /// Boat `ok` field.
    pub ok: bool,
    /// Boat `type` field.
    #[serde(rename = "type")]
    pub kind: String,
    /// Boat `id` field.
    pub id: String,
    /// Boat `status` field.
    pub status: String,
    /// Boat `sandbox` field.
    pub sandbox: Option<Sandbox>,
    /// On stop, true when this stop discards work written after the last snapshot.
    pub forced: Option<bool>,
    /// On stop, the stop operation (also on a sandbox that is already stopping or stopped).
    pub operation: Option<StopOperation>,
    /// On stop, the path that reads this stop operation, for example
    /// `/api/v1/sandboxes/bx_23456789/stops/stop_0123456789abcdef`.
    pub status_url: Option<String>,
    /// Present on resume or fork when a fair-use cap on a gifted account shortened the requested auto-stop
    /// time.
    pub gift_limit_notice: Option<String>,
    /// On interrupt, the conversation whose turn was interrupted.
    pub conversation_id: Option<String>,
}

/// Stop Operation Response in the Boat API.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StopOperationResponse {
    /// Boat `ok` field.
    pub ok: bool,
    /// Boat `type` field.
    #[serde(rename = "type")]
    pub kind: String,
    /// Boat `id` field.
    pub id: String,
    /// Boat `stop` field.
    pub stop: StopOperation,
}

/// Deletion Operation Response in the Boat API.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeletionOperationResponse {
    /// Boat `ok` field.
    pub ok: bool,
    /// Boat `type` field.
    #[serde(rename = "type")]
    pub kind: String,
    /// Boat `operation` field.
    pub operation: DeletionOperation,
}

/// Sandbox Usage Response in the Boat API.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SandboxUsageResponse {
    /// Boat `ok` field.
    pub ok: bool,
    /// Boat `type` field.
    #[serde(rename = "type")]
    pub kind: String,
    /// Boat `sandboxId` field.
    pub sandbox_id: String,
    /// The sandbox's current machine size, which sets `billingMultiplier`.
    pub sandbox_type: String,
    /// Rate at which this sandbox consumes machine time.
    pub billing_multiplier: f64,
    /// Start of the window the figures cover.
    pub since: String,
    /// End of the window.
    pub until: String,
    /// Billable machine-seconds inside the window, the type multiplier already applied: a `large` sandbox
    /// that ran ten minutes reads 1200.
    pub seconds: i64,
    /// `seconds` at list price (`seconds / secondsPerDollar`), whatever plan, trial or gift actually paid
    /// for it.
    pub dollars: f64,
    /// How many billable seconds one dollar buys, so you can price `seconds` yourself.
    pub seconds_per_dollar: i64,
    /// The meter is still moving: the sandbox is up and no refused stop is holding it paused.
    pub running: bool,
}
