//! Sandbox lifecycle, operation status, usage, and capacity models.

mod api;
/// Capacity, quota, and billing information.
pub mod limits;
/// Options accepted by sandbox mutations.
pub mod requests;
/// Sandbox records and operation metadata.
pub mod resources;
/// Lifecycle API response envelopes.
pub mod responses;

/// Machine sizes accepted by create, resume, and fork.
#[derive(Clone, Copy, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MachineSize {
    /// Small machine.
    Small,
    /// Standard machine.
    Default,
    /// Large machine.
    Large,
    /// Extra-large machine.
    Xlarge,
}

/// Explicit auto-stop setting. Omit the request field to use Boat's default.
#[derive(Clone, Copy, serde::Serialize)]
#[serde(untagged)]
pub enum AutoStop {
    /// Archive the sandbox after this many seconds.
    AfterSeconds(u32),
    /// Disable automatic archival (JSON `null`).
    Disabled,
}

/// Filters and pagination for [`crate::Client::list_sandboxes`].
#[derive(Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SandboxListQuery {
    /// Organization owning the sandboxes.
    pub org: Option<String>,
    /// Maximum number of results on this page.
    pub limit: Option<u32>,
    /// Cursor returned by the previous page.
    pub cursor: Option<String>,
    /// API sort order.
    pub sort: Option<String>,
    /// Comma-separated states to include.
    pub state: Option<String>,
    /// Match an exact display name.
    pub name: Option<String>,
    /// Match a display name prefix.
    pub name_prefix: Option<String>,
    /// Select sandboxes stopped before this ISO 8601 timestamp.
    pub stopped_before: Option<String>,
    /// Filter by whether retention has removed the data.
    pub data_expired: Option<bool>,
    /// Require each `key=value` label.
    #[serde(skip)]
    pub labels: Vec<String>,
    /// Comma-separated optional fields such as `cost,deletedCost`.
    pub include: Option<String>,
}

/// Optional time bounds for a sandbox's billable usage.
#[derive(Default, serde::Serialize)]
pub struct UsageWindow {
    /// Inclusive start, as ISO 8601 or Unix epoch seconds.
    pub since: Option<String>,
    /// End, as ISO 8601 or Unix epoch seconds.
    pub until: Option<String>,
}
