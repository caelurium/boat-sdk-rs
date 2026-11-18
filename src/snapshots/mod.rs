//! Browse stored snapshots or save named snapshots for later sandbox creation.

mod binary;
pub mod models;
mod saved;

pub use binary::SnapshotFile;
pub use saved::SavedSnapshotQuery;

use models::{
    DeletionOperationListResponse, SnapshotDownloadResponse, SnapshotLatestResponse,
    SnapshotListResponse, SnapshotTreeResponse,
};
use reqwest::Method;
use serde::Serialize;

use crate::sandbox::responses::DeletionOperationResponse;
use crate::{Client, Error};

/// Pagination and ordering for snapshot listings.
#[derive(Default, Serialize)]
pub struct SnapshotListQuery {
    /// Maximum number of snapshots on this page.
    pub limit: Option<u32>,
    /// Cursor returned by the previous page.
    pub cursor: Option<String>,
    /// Boat sort order.
    pub sort: Option<String>,
}

impl Client {
    /// List snapshots visible to this account.
    pub async fn list_snapshots(
        &self,
        query: &SnapshotListQuery,
    ) -> Result<SnapshotListResponse, Error> {
        self.get_json(&["snapshots"], query).await
    }

    /// List snapshots for one sandbox.
    pub async fn list_sandbox_snapshots(
        &self,
        id: &str,
        query: &SnapshotListQuery,
    ) -> Result<SnapshotListResponse, Error> {
        self.get_json(&["sandboxes", id, "snapshots"], query).await
    }

    /// Read the latest snapshot, if one exists.
    pub async fn latest_sandbox_snapshot(&self, id: &str) -> Result<SnapshotLatestResponse, Error> {
        self.get_json(&["sandboxes", id, "snapshots", "latest"], &())
            .await
    }

    /// Browse a stored snapshot without starting the sandbox.
    pub async fn snapshot_tree(&self, id: &str) -> Result<SnapshotTreeResponse, Error> {
        self.get_json(&["snapshots", id, "tree"], &()).await
    }

    /// Get the chunk manifest. Each signed URL grants direct access until
    /// `expires_in_seconds` elapses; do not log URLs or send the Boat bearer key to them.
    pub async fn snapshot_download(&self, id: &str) -> Result<SnapshotDownloadResponse, Error> {
        self.get_json(&["snapshots", id, "download"], &()).await
    }

    /// Request permanent deletion of this snapshot and return the operation.
    pub async fn delete_snapshot(&self, id: &str) -> Result<DeletionOperationResponse, Error> {
        Self::send(
            self.request(Method::DELETE, &["snapshots", id])?
                .header("X-Ascii-Confirm-Delete", id),
        )
        .await
    }

    /// Request permanent deletion of a sandbox's snapshots.
    pub async fn delete_sandbox_snapshots(
        &self,
        id: &str,
    ) -> Result<DeletionOperationListResponse, Error> {
        Self::send(
            self.request(Method::DELETE, &["sandboxes", id, "snapshots"])?
                .header("X-Ascii-Confirm-Delete", id),
        )
        .await
    }
}
