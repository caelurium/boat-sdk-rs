use reqwest::Method;

use super::limits::LimitsResponse;
use super::requests::{
    CreateSandboxRequest, ForkRequest, ResumeRequest, StopRequest, UpdateSandboxRequest,
};
use super::responses::{
    CreateSandboxResponse, DeletionOperationResponse, SandboxActionResponse, SandboxInfoResponse,
    SandboxListResponse, SandboxUsageResponse, StopOperationResponse,
};
use super::{SandboxListQuery, UsageWindow};
use crate::{Client, Error};

impl Client {
    /// Start provisioning. Reuse the same idempotency key and body to retry a create.
    pub async fn create_sandbox(
        &self,
        body: &CreateSandboxRequest,
        idempotency_key: Option<&str>,
    ) -> Result<CreateSandboxResponse, Error> {
        self.provision(&["sandboxes"], body, idempotency_key).await
    }

    /// List one page of matching sandboxes.
    pub async fn list_sandboxes(
        &self,
        query: &SandboxListQuery,
    ) -> Result<SandboxListResponse, Error> {
        let request = self
            .request(Method::GET, &["sandboxes"])?
            .query(query)
            .query(
                &query
                    .labels
                    .iter()
                    .map(|label| ("label", label))
                    .collect::<Vec<_>>(),
            );
        Self::send(request).await
    }

    /// Get the current sandbox state and metadata.
    pub async fn get_sandbox(&self, id: &str) -> Result<SandboxInfoResponse, Error> {
        self.get_json(&["sandboxes", id], &()).await
    }

    /// Update the supplied fields of a sandbox.
    pub async fn update_sandbox(
        &self,
        id: &str,
        body: &UpdateSandboxRequest,
    ) -> Result<SandboxInfoResponse, Error> {
        Self::send(self.request(Method::PATCH, &["sandboxes", id])?.json(body)).await
    }

    /// Request a stop; the returned operation may still be in progress.
    pub async fn stop_sandbox(
        &self,
        id: &str,
        body: &StopRequest,
    ) -> Result<SandboxActionResponse, Error> {
        self.post_json(&["sandboxes", id, "stop"], body).await
    }

    /// Read a previously requested stop operation.
    pub async fn get_stop(&self, id: &str, stop_id: &str) -> Result<StopOperationResponse, Error> {
        self.get_json(&["sandboxes", id, "stops", stop_id], &())
            .await
    }

    /// Start restoring a stopped sandbox.
    pub async fn resume_sandbox(
        &self,
        id: &str,
        body: &ResumeRequest,
    ) -> Result<SandboxActionResponse, Error> {
        self.post_json(&["sandboxes", id, "resume"], body).await
    }

    /// Fork into a new sandbox. The response ID identifies the fork, not its source.
    pub async fn fork_sandbox(
        &self,
        id: &str,
        body: &ForkRequest,
        idempotency_key: Option<&str>,
    ) -> Result<SandboxActionResponse, Error> {
        self.provision(&["sandboxes", id, "fork"], body, idempotency_key)
            .await
    }

    async fn provision<T: serde::de::DeserializeOwned, B: serde::Serialize>(
        &self,
        path: &[&str],
        body: &B,
        key: Option<&str>,
    ) -> Result<T, Error> {
        let mut request = self.request(Method::POST, path)?.json(body);
        if let Some(key) = key {
            request = request.header("Idempotency-Key", key);
        }
        Self::send(request).await
    }

    /// Permanently delete this sandbox and its snapshots. Sends the target ID
    /// as Boat's required confirmation header. Deletion completes asynchronously.
    pub async fn delete_sandbox(&self, id: &str) -> Result<DeletionOperationResponse, Error> {
        Self::send(
            self.request(Method::DELETE, &["sandboxes", id])?
                .header("X-Ascii-Confirm-Delete", id),
        )
        .await
    }

    /// Read metered machine time and cost over the requested window.
    pub async fn sandbox_usage(
        &self,
        id: &str,
        window: &UsageWindow,
    ) -> Result<SandboxUsageResponse, Error> {
        self.get_json(&["sandboxes", id, "usage"], window).await
    }

    /// Read sandbox capacity and billing limits in the client's organization context.
    pub async fn limits(&self) -> Result<LimitsResponse, Error> {
        self.get_json(&["limits"], &()).await
    }
}
