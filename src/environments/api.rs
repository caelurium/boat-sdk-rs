use super::models::*;
use crate::{Client, Error};
use reqwest::Method;

impl Client {
    /// List GitHub repositories available to Boat.
    pub async fn repos(&self, query: &ReposQuery) -> Result<ReposResponse, Error> {
        Self::send(self.request(Method::GET, &["repos"])?.query(query)).await
    }

    /// Select repository for sandboxes.
    pub async fn select_repo(
        &self,
        body: &RepoSelectionRequest,
    ) -> Result<RepoSelectionResponse, Error> {
        Self::send(self.request(Method::POST, &["repos"])?.json(body)).await
    }

    /// Get Boat secrets setup. Returns sensitive environment contents to authorized callers.
    pub async fn secrets(&self) -> Result<SecretsResponse, Error> {
        Self::send(self.request(Method::GET, &["secrets"])?).await
    }

    /// Update Boat secrets setup. Full replacement: omitted values clear the previous contents.
    pub async fn update_secrets(
        &self,
        body: &SecretsUpdateRequest,
    ) -> Result<SecretsResponse, Error> {
        Self::send(self.request(Method::POST, &["secrets"])?.json(body)).await
    }

    /// List Sandbox environments.
    pub async fn environments(
        &self,
        org: Option<&str>,
    ) -> Result<SandboxEnvironmentListResponse, Error> {
        Self::send(
            self.request(Method::GET, &["environments"])?
                .query(&[("org", org)]),
        )
        .await
    }

    /// Create a sandbox environment.
    pub async fn create_environment(
        &self,
        org: Option<&str>,
        body: &CreateSandboxEnvironmentRequest,
    ) -> Result<SandboxEnvironmentListResponse, Error> {
        Self::send(
            self.request(Method::POST, &["environments"])?
                .query(&[("org", org)])
                .json(body),
        )
        .await
    }

    /// Update a sandbox environment.
    pub async fn update_environment(
        &self,
        environment_id: &str,
        body: &UpdateSandboxEnvironmentRequest,
    ) -> Result<SandboxEnvironmentResponse, Error> {
        Self::send(
            self.request(Method::PUT, &["environments", environment_id])?
                .json(body),
        )
        .await
    }

    /// Delete a sandbox environment.
    pub async fn delete_environment(
        &self,
        environment_id: &str,
    ) -> Result<SandboxEnvironmentResponse, Error> {
        Self::send(self.request(Method::DELETE, &["environments", environment_id])?).await
    }

    /// Upgrade sandboxes to an environment's latest version.
    pub async fn upgrade_environment(
        &self,
        environment_id: &str,
        org: Option<&str>,
        body: &UpgradeSandboxEnvironmentRequest,
    ) -> Result<UpgradeSandboxEnvironmentResponse, Error> {
        Self::send(
            self.request(Method::POST, &["environments", environment_id, "upgrade"])?
                .query(&[("org", org)])
                .json(body),
        )
        .await
    }

    /// Set one environment variable.
    pub async fn set_environment_var(
        &self,
        environment_id: &str,
        key: &str,
        body: &SetEnvironmentVarRequest,
    ) -> Result<EnvironmentItemChangeResponse, Error> {
        Self::send(
            self.request(Method::PUT, &["environments", environment_id, "vars", key])?
                .json(body),
        )
        .await
    }

    /// Remove one environment variable.
    pub async fn delete_environment_var(
        &self,
        environment_id: &str,
        key: &str,
    ) -> Result<EnvironmentItemChangeResponse, Error> {
        Self::send(self.request(
            Method::DELETE,
            &["environments", environment_id, "vars", key],
        )?)
        .await
    }

    /// Write one secret file.
    pub async fn set_environment_secret_file(
        &self,
        environment_id: &str,
        body: &SetEnvironmentSecretFileRequest,
    ) -> Result<EnvironmentItemChangeResponse, Error> {
        Self::send(
            self.request(
                Method::PUT,
                &["environments", environment_id, "secret-files"],
            )?
            .json(body),
        )
        .await
    }

    /// Remove one secret file.
    pub async fn delete_environment_secret_file(
        &self,
        environment_id: &str,
        path: &str,
    ) -> Result<EnvironmentItemChangeResponse, Error> {
        Self::send(
            self.request(
                Method::DELETE,
                &["environments", environment_id, "secret-files"],
            )?
            .query(&[("path", path)]),
        )
        .await
    }

    /// Add a repository to an environment.
    pub async fn add_environment_repo(
        &self,
        environment_id: &str,
        org: Option<&str>,
        body: &AddEnvironmentRepoRequest,
    ) -> Result<EnvironmentItemChangeResponse, Error> {
        Self::send(
            self.request(Method::POST, &["environments", environment_id, "repos"])?
                .query(&[("org", org)])
                .json(body),
        )
        .await
    }

    /// Remove a repository from an environment.
    pub async fn delete_environment_repo(
        &self,
        environment_id: &str,
        repository_id: &str,
        org: Option<&str>,
    ) -> Result<EnvironmentItemChangeResponse, Error> {
        Self::send(
            self.request(
                Method::DELETE,
                &["environments", environment_id, "repos", repository_id],
            )?
            .query(&[("org", org)]),
        )
        .await
    }
}
