//! Boat snapshots request and response models.

use crate::sandbox::resources::{DeletionOperation, PageInfo};

/// Snapshot Summary.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotSummary {
    /// Boat `id` field.
    pub id: String,
    /// Public Sandbox id this snapshot belongs to.
    pub sandbox_id: String,
    /// Boat `status` field.
    pub status: String,
    /// `base` (full) or `incremental` (delta on a base).
    pub kind: Option<String>,
    /// Position in the incremental chain (0 = base).
    pub generation: Option<u64>,
    /// Boat `chainId` field.
    pub chain_id: Option<String>,
    /// Boat `createdAt` field.
    pub created_at: String,
    /// Boat `completedAt` field.
    pub completed_at: Option<String>,
    /// Bytes this snapshot added (its delta), not the full restored size.
    pub size_bytes: Option<u64>,
    /// Inventory entries alive in the chain at this generation (includes base-image system entries).
    pub file_count: Option<u64>,
    /// Total bytes of your data restored by this snapshot (what resume/download returns; base image excluded).
    pub content_size_bytes: Option<u64>,
    /// Number of your files restored by this snapshot (base image excluded).
    pub content_file_count: Option<u64>,
    /// `template` = this Sandbox has snapshots off, so Boat never captures it: the row is a copy of the
    /// template, saved snapshot or Sandbox it was created from.
    pub origin: Option<String>,
}

/// Snapshot List Response.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotListResponse {
    /// Boat `ok` field.
    pub ok: bool,
    /// Boat `type` field.
    #[serde(rename = "type")]
    pub response_type: String,
    /// Boat `snapshots` field.
    pub snapshots: Vec<SnapshotSummary>,
    /// Boat `pageInfo` field.
    pub page_info: Option<PageInfo>,
}

/// Snapshot Latest Response.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotLatestResponse {
    /// Boat `ok` field.
    pub ok: bool,
    /// Boat `type` field.
    #[serde(rename = "type")]
    pub response_type: String,
    /// Boat `snapshot` field.
    pub snapshot: Option<SnapshotSummary>,
}

/// A saved snapshot: a frozen copy of a sandbox's disk at one moment, saved under a name you pick.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SavedSnapshot {
    /// The user-chosen handle, unique within your account.
    pub name: String,
    /// Boat `status` field.
    pub status: String,
    /// Failure reason.
    pub error: Option<String>,
    /// The sandbox this snapshot was saved from (display only).
    pub source_sandbox_id: String,
    /// The frozen artifact behind the name.
    pub snapshot_id: Option<String>,
    /// Sandbox type the snapshot was saved from.
    #[serde(rename = "type")]
    pub machine_type: Option<String>,
    /// Restored content size of the frozen state, in bytes.
    pub size_bytes: Option<u64>,
    /// Boat `createdAt` field.
    pub created_at: String,
}

/// Saved Snapshot List Response.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SavedSnapshotListResponse {
    /// Boat `ok` field.
    pub ok: bool,
    /// Boat `type` field.
    #[serde(rename = "type")]
    pub response_type: String,
    /// Boat `snapshots` field.
    pub snapshots: Vec<SavedSnapshot>,
    /// Boat `allowance` field.
    pub allowance: Option<SavedSnapshotAllowance>,
}

/// What the request's wallet (your active organization, `?org=`, or personal) pays for saved snapshots.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SavedSnapshotAllowance {
    /// Organization the numbers are for.
    pub org: Option<String>,
    /// That organization's name.
    pub org_name: Option<String>,
    /// Saved snapshots the plan includes: 10 per $20 of plan per seat, never fewer than 10.
    pub included: Option<u64>,
    /// Same value as included.
    pub free: Option<u64>,
    /// Saved snapshots this wallet keeps.
    pub used: Option<u64>,
    /// Saved snapshots above the included ones.
    pub extra: Option<u64>,
    /// What the extra ones cost per month.
    pub extra_monthly_dollars: Option<f64>,
    /// What the wallet pays per day for the extra ones, in dollars.
    pub extra_daily_dollars: Option<f64>,
    /// Price of one extra saved snapshot for 30 days (1.68).
    pub price_per_extra_monthly_dollars: Option<f64>,
    /// Daily price of one extra saved snapshot (0.056).
    pub price_per_extra_daily_dollars: Option<f64>,
}

/// Saved Snapshot Info Response.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SavedSnapshotInfoResponse {
    /// Boat `ok` field.
    pub ok: bool,
    /// Boat `type` field.
    #[serde(rename = "type")]
    pub response_type: String,
    /// Boat `snapshot` field.
    pub snapshot: SavedSnapshot,
}

/// Saved Snapshot Save Request.
#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SavedSnapshotSaveRequest {
    /// The sandbox whose current state to freeze.
    pub sandbox_id: String,
    /// Name to save under.
    pub name: String,
}

/// Saved Snapshot Saving Response.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SavedSnapshotSavingResponse {
    /// Boat `ok` field.
    pub ok: bool,
    /// Boat `type` field.
    #[serde(rename = "type")]
    pub response_type: String,
    /// Boat `status` field.
    pub status: String,
    /// Boat `snapshot` field.
    pub snapshot: SavedSnapshot,
}

/// Saved Snapshot Deleted Response.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SavedSnapshotDeletedResponse {
    /// Boat `ok` field.
    pub ok: bool,
    /// Boat `type` field.
    #[serde(rename = "type")]
    pub response_type: String,
    /// Boat `name` field.
    pub name: String,
    /// Boat `status` field.
    pub status: String,
}

/// A share code for a named snapshot.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamedSnapshotShare {
    /// Share id, for example `shr_...`.
    pub id: String,
    /// The first characters of the code, for example `bsh_xxxxxxxx`.
    pub code_prefix: String,
    /// Boat `status` field.
    pub status: String,
    /// Number of sandboxes created with this code.
    pub use_count: u64,
    /// When a sandbox was last created with this code.
    pub last_used_at: Option<String>,
    /// When the code was revoked.
    pub revoked_at: Option<String>,
    /// Boat `createdAt` field.
    pub created_at: String,
}

/// Named Snapshot Share Created Response.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamedSnapshotShareCreatedResponse {
    /// Boat `ok` field.
    pub ok: bool,
    /// Boat `type` field.
    #[serde(rename = "type")]
    pub response_type: String,
    /// The named snapshot the code deploys.
    pub name: String,
    /// The share code, for example `bsh_...`.
    pub share_code: String,
    /// Boat `share` field.
    pub share: NamedSnapshotShare,
}

/// Named Snapshot Share List Response.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamedSnapshotShareListResponse {
    /// Boat `ok` field.
    pub ok: bool,
    /// Boat `type` field.
    #[serde(rename = "type")]
    pub response_type: String,
    /// Boat `name` field.
    pub name: String,
    /// Boat `shares` field.
    pub shares: Vec<NamedSnapshotShare>,
}

/// Named Snapshot Share Revoked Response.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamedSnapshotShareRevokedResponse {
    /// Boat `ok` field.
    pub ok: bool,
    /// Boat `type` field.
    #[serde(rename = "type")]
    pub response_type: String,
    /// Boat `name` field.
    pub name: String,
    /// Boat `share` field.
    pub share: NamedSnapshotShare,
}

/// Snapshot Tree Entry.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotTreeEntry {
    /// Path relative to the snapshot root.
    pub path: String,
    /// Boat `kind` field.
    pub kind: String,
    /// File size in bytes.
    pub size: Option<u64>,
}

/// Snapshot Tree Response.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotTreeResponse {
    /// Boat `ok` field.
    pub ok: bool,
    /// Boat `type` field.
    #[serde(rename = "type")]
    pub response_type: String,
    /// Boat `snapshotId` field.
    pub snapshot_id: String,
    /// Boat `sandboxId` field.
    pub sandbox_id: String,
    /// Boat `generation` field.
    pub generation: u64,
    /// `false` for legacy snapshots or inventories too large to expand; see `reason`.
    pub tree_available: bool,
    /// `true` when the file list was capped; not every entry is returned.
    pub truncated: bool,
    /// Number of your files in this snapshot (base-image system files excluded).
    pub file_count: u64,
    /// Total bytes of your data in this snapshot (base-image system files excluded).
    pub total_size_bytes: u64,
    /// Exactly the files/dirs a resume or download returns, your data only; base-image system entries are not
    /// listed.
    pub entries: Vec<SnapshotTreeEntry>,
    /// Why the tree is unavailable, when `treeAvailable` is `false`.
    pub reason: Option<String>,
}

/// Snapshot Chunk.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotChunk {
    /// Boat `snapshotId` field.
    pub snapshot_id: String,
    /// Boat `generation` field.
    pub generation: u64,
    /// Boat `chunkIndex` field.
    pub chunk_index: u64,
    /// Boat `r2Key` field.
    pub r2_key: String,
    /// Boat `sizeBytes` field.
    pub size_bytes: Option<u64>,
    /// Boat `sha256` field.
    pub sha256: Option<String>,
    /// Time-limited download URL (see `expiresInSeconds`).
    pub signed_url: String,
}

/// Snapshot Inventory.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotInventory {
    /// Boat `r2Key` field.
    pub r2_key: String,
    /// Boat `signedUrl` field.
    pub signed_url: String,
}

/// Snapshot Download Response.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotDownloadResponse {
    /// Boat `ok` field.
    pub ok: bool,
    /// Boat `type` field.
    #[serde(rename = "type")]
    pub response_type: String,
    /// Boat `snapshotId` field.
    pub snapshot_id: String,
    /// Boat `sandboxId` field.
    pub sandbox_id: String,
    /// Boat `kind` field.
    pub kind: String,
    /// Boat `generation` field.
    pub generation: u64,
    /// Lifetime of every `signedUrl` in this response.
    pub expires_in_seconds: u64,
    /// Human-readable note on how to reassemble the chunks into a filesystem.
    pub reconstruct: String,
    /// Boat `inventory` field.
    pub inventory: Option<SnapshotInventory>,
    /// Every chunk across the chain (all generations up to and including this snapshot), ordered by
    /// `(generation, chunkIndex)`.
    pub chunks: Vec<SnapshotChunk>,
}

/// Deletion Operation List Response.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeletionOperationListResponse {
    /// Boat `ok` field.
    pub ok: bool,
    /// Boat `type` field.
    #[serde(rename = "type")]
    pub response_type: String,
    /// Boat `operations` field.
    pub operations: Vec<DeletionOperation>,
}
