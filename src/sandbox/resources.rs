/// Sandbox Hydration in the Boat API.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SandboxHydration {
    /// Restore step: `dvol` (docker volumes), `sysdelta` (system files), `images` (saved docker images),
    /// `hydrating` (the home directory), `reconciling` (the last check).
    pub phase: String,
    /// Files, directories and links copied in so far.
    pub entries_done: i64,
    /// Files, directories and links this restore copies in.
    pub entries_total: Option<i64>,
    /// entriesDone of entriesTotal, 0-99.
    pub percent: Option<i64>,
}

/// Sandbox Team in the Boat API.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SandboxTeam {
    /// Boat `id` field.
    pub id: String,
    /// Boat `name` field.
    pub name: String,
}

/// Sandbox Cost in the Boat API.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SandboxCost {
    /// The 1st of this month, 00:00 UTC.
    pub since: String,
    /// Billable machine-seconds, size multiplier applied.
    pub seconds: i64,
    /// seconds / secondsPerDollar, at list price.
    pub dollars: f64,
}

/// Sandbox Last Reboot in the Boat API.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SandboxLastReboot {
    /// Boat `at` field.
    pub at: Option<String>,
    /// Boat `reason` field.
    pub reason: Option<String>,
}

/// Sandbox in the Boat API.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sandbox {
    /// Boat `id` field.
    pub id: String,
    /// Boat `name` field.
    pub name: String,
    /// `cancelled` is terminal: a create or fork that could not get a machine was removed.
    pub state: String,
    /// Why the sandbox is stopped, failed or cancelled, or null.
    pub error: Option<String>,
    /// When boat deleted the snapshot data after this sandbox was stopped for at least 14 days.
    pub data_expired_at: Option<String>,
    /// When the sandbox last stopped, by any cause: your stop, its auto-stop, a plan limit or Boat.
    pub stopped_at: Option<String>,
    /// The labels set at create or fork.
    pub labels: Option<std::collections::BTreeMap<String, String>>,
    /// `degraded` when the machine is alive but Boat cannot fully reach it (for example a firewall rule
    /// inside the sandbox blocks the tunnel).
    pub health: Option<String>,
    /// Why the sandbox is degraded, or null.
    pub health_reason: Option<String>,
    /// When the sandbox became degraded, or null.
    pub degraded_since: Option<String>,
    /// True once every file of the sandbox's snapshot is on its disk.
    pub hydrated: Option<bool>,
    /// When the last resume or fork finished copying files in, or null.
    pub hydrated_at: Option<String>,
    /// Restore progress while `hydrated` is false, on GET /sandboxes/{id} only.
    pub hydration: Option<SandboxHydration>,
    /// Current machine size: what the sandbox was created with, or the size it was last resumed or forked
    /// onto.
    #[serde(rename = "type")]
    pub machine_type: Option<String>,
    /// vCPUs guaranteed by this sandbox's type.
    pub vcpu: Option<i64>,
    /// RAM in GB guaranteed by this sandbox's type.
    #[serde(rename = "memoryGB")]
    pub memory_gb: Option<i64>,
    /// Rate at which this sandbox consumes machine time.
    pub billing_multiplier: Option<f64>,
    /// Always `baremetal`.
    pub machine_provider: Option<String>,
    /// Machine URL when assigned.
    pub url: Option<String>,
    /// Machine IPv6 or IPv4 address when assigned.
    pub ip: Option<String>,
    /// Public IPv4 `host:port` that forwards to the sandbox SSH server.
    pub ssh_endpoint: Option<String>,
    /// Boat `createdAt` field.
    pub created_at: Option<String>,
    /// Boat `updatedAt` field.
    pub updated_at: Option<String>,
    /// When the sandbox stops by itself (the stop starts within about 5 seconds), or null when auto-stop is
    /// disabled.
    pub archive_after: Option<String>,
    /// The auto-stop length as last set on create, resume, fork or PATCH, or null when auto-stop is
    /// disabled.
    pub ttl_seconds: Option<i64>,
    /// Boat `desktopAvailable` field.
    pub desktop_available: bool,
    /// Secret-bearing desktop stream URL when available.
    pub desktop_url: Option<String>,
    /// False when the sandbox was created with snapshots off.
    pub snapshots: Option<bool>,
    /// False when the sandbox was created with monitoring off.
    pub monitoring: Option<bool>,
    /// Boat `snapshotAvailable` field.
    pub snapshot_available: bool,
    /// Timestamp of the most recent successfully completed snapshot, or null.
    pub snapshot_completed_at: Option<String>,
    /// Last time a snapshot confirmed the sandbox's saved state, including checks that found nothing new to
    /// save.
    pub snapshot_verified_at: Option<String>,
    /// The organization billed for this sandbox, or null when the owner is billed.
    pub team: Option<SandboxTeam>,
    /// Display name (or email) of the person who created the sandbox.
    pub created_by: Option<String>,
    /// Account id of the person who created the sandbox.
    pub created_by_id: Option<String>,
    /// What you may do with this sandbox.
    pub access: Option<String>,
    /// True while the sandbox still holds its creator's personal logins (GitHub token, model logins,
    /// secrets).
    pub holds_creator_logins: Option<bool>,
    /// Organization sandboxes only; null for a personal sandbox.
    pub visibility: Option<String>,
    /// Only with `include=cost` on `GET /sandboxes`.
    pub cost: Option<SandboxCost>,
    /// True once the sandbox is shared but not restarted yet.
    pub wipe_pending_until_restart: Option<bool>,
    /// The sandbox's stable three-word subdomain slug (e.g.
    pub subdomain: Option<String>,
    /// Timestamp of the most recent snapshot attempt of any status (queued, in_progress, completed, failed,
    /// cancelled), or null.
    pub last_snapshot_attempt_at: Option<String>,
    /// Status of the most recent snapshot attempt, or null if none.
    pub last_snapshot_status: Option<String>,
    /// The last time the sandbox's kernel restarted on its machine, or null if it never did.
    pub last_reboot: Option<SandboxLastReboot>,
    /// Boat `setupStatus` field.
    pub setup_status: Option<String>,
    /// Short failure detail (exit code plus a stderr tail) when `setupStatus` is `failed`; while `pending`,
    /// may carry the last start/upload error from a retry in progress.
    pub setup_error: Option<String>,
    /// Name of the sandbox environment this sandbox is running, or null if it is attached to none (a
    /// `noEnv` sandbox, or one whose environment was deleted).
    pub environment: Option<String>,
    /// Version number of `environment` that this sandbox is pinned to.
    pub environment_version: Option<i64>,
    /// The last stop you requested (`POST /sandboxes/{sandboxId}/stop`), or null if you never stopped this
    /// sandbox.
    pub stop: Option<StopOperation>,
}

/// Stop Operation in the Boat API.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StopOperation {
    /// Stable id of this stop.
    pub id: String,
    /// `pending`: accepted, the sandbox is archiving.
    pub status: String,
    /// When the stop was accepted.
    pub requested_at: Option<String>,
    /// When the last archival attempt ended.
    pub last_attempt_at: Option<String>,
    /// Why the last attempt failed, while `status` is `failing`.
    pub error: Option<String>,
    /// When the stop completed or was superseded.
    pub ended_at: Option<String>,
    /// True when a force (`--force`) ended this stop.
    pub forced: Option<bool>,
}

/// Deletion Operation in the Boat API.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeletionOperation {
    /// Boat `id` field.
    pub id: String,
    /// Boat `kind` field.
    pub kind: String,
    /// Boat `targetId` field.
    pub target_id: String,
    /// Boat `reason` field.
    pub reason: String,
    /// Boat `status` field.
    pub status: String,
    /// What the background purge is doing.
    pub stage: Option<String>,
    /// When `waiting_for_uploads` ends.
    pub expected_by: Option<String>,
    /// Boat `attemptCount` field.
    pub attempt_count: i64,
    /// Boat `requestedAt` field.
    pub requested_at: String,
    /// Boat `completedAt` field.
    pub completed_at: Option<String>,
}

/// Page Info in the Boat API.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageInfo {
    /// Boat `nextCursor` field.
    pub next_cursor: Option<String>,
    /// Boat `hasMore` field.
    pub has_more: bool,
    /// Boat `limit` field.
    pub limit: i64,
}
