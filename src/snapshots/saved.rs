use reqwest::Method;
use serde::Serialize;

use super::models::{
    NamedSnapshotShareCreatedResponse, NamedSnapshotShareListResponse,
    NamedSnapshotShareRevokedResponse, SavedSnapshotDeletedResponse, SavedSnapshotInfoResponse,
    SavedSnapshotListResponse, SavedSnapshotSaveRequest, SavedSnapshotSavingResponse,
};
use crate::{Client, Error};

/// Filters for saved snapshots, within the client's organization context.
#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SavedSnapshotQuery {
    /// Filter by an exact name.
    pub name: Option<String>,
    /// Filter by a name prefix.
    pub name_prefix: Option<String>,
}

impl Client {
    /// Save the current sandbox state under a name. Poll [`Self::get_saved_snapshot`]
    /// until ready, then set `CreateSandboxRequest::from` to deploy it.
    pub async fn save_snapshot(
        &self,
        body: &SavedSnapshotSaveRequest,
    ) -> Result<SavedSnapshotSavingResponse, Error> {
        self.post_json(&["saved-snapshots"], body).await
    }

    /// List reusable saved snapshots.
    pub async fn list_saved_snapshots(
        &self,
        query: &SavedSnapshotQuery,
    ) -> Result<SavedSnapshotListResponse, Error> {
        self.get_json(&["saved-snapshots"], query).await
    }

    /// Read a named snapshot's metadata and save status.
    pub async fn get_saved_snapshot(&self, name: &str) -> Result<SavedSnapshotInfoResponse, Error> {
        self.get_json(&["saved-snapshots", name], &()).await
    }

    /// Delete the named saved snapshot.
    pub async fn delete_saved_snapshot(
        &self,
        name: &str,
    ) -> Result<SavedSnapshotDeletedResponse, Error> {
        Self::send(self.request(Method::DELETE, &["saved-snapshots", name])?).await
    }

    /// List share metadata for a saved snapshot.
    pub async fn list_snapshot_shares(
        &self,
        name: &str,
    ) -> Result<NamedSnapshotShareListResponse, Error> {
        self.get_json(&["saved-snapshots", name, "shares"], &())
            .await
    }

    /// Create a share code for deploying a saved snapshot. Treat the returned code as a secret.
    pub async fn create_snapshot_share(
        &self,
        name: &str,
    ) -> Result<NamedSnapshotShareCreatedResponse, Error> {
        Self::send(self.request(Method::POST, &["saved-snapshots", name, "shares"])?).await
    }

    /// Revoke a saved snapshot share.
    pub async fn revoke_snapshot_share(
        &self,
        name: &str,
        share_id: &str,
    ) -> Result<NamedSnapshotShareRevokedResponse, Error> {
        Self::send(self.request(
            Method::DELETE,
            &["saved-snapshots", name, "shares", share_id],
        )?)
        .await
    }
}
