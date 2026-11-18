use super::models::*;
use crate::{Client, Error};
use reqwest::Method;

impl Client {
    /// List credential profiles. Requires `account.admin`.
    pub async fn list_credential_profiles(&self) -> Result<CredentialProfileListResponse, Error> {
        Self::send(self.request(Method::GET, &["credential-profiles"])?).await
    }

    /// Create a credential profile. Requires `account.admin`.
    pub async fn create_credential_profile(
        &self,
        body: &CreateCredentialProfileRequest,
    ) -> Result<CredentialProfileResponse, Error> {
        Self::send(
            self.request(Method::POST, &["credential-profiles"])?
                .json(body),
        )
        .await
    }

    /// Update a credential profile. Requires `account.admin`.
    pub async fn update_credential_profile(
        &self,
        profile: &str,
        body: &UpdateCredentialProfileRequest,
    ) -> Result<CredentialProfileResponse, Error> {
        Self::send(
            self.request(Method::PUT, &["credential-profiles", profile])?
                .json(body),
        )
        .await
    }

    /// Delete a credential profile. Requires `account.admin`.
    pub async fn delete_credential_profile(
        &self,
        profile: &str,
    ) -> Result<CredentialProfileDeletedResponse, Error> {
        Self::send(self.request(Method::DELETE, &["credential-profiles", profile])?).await
    }

    /// Set a credential profile API key. Requires `account.admin`.
    pub async fn set_credential_profile_key(
        &self,
        profile: &str,
        credential: &str,
        body: &SetCredentialKeyRequest,
    ) -> Result<CredentialProfileResponse, Error> {
        Self::send(
            self.request(
                Method::PUT,
                &["credential-profiles", profile, "keys", credential],
            )?
            .json(body),
        )
        .await
    }

    /// Remove a credential profile credential. Requires `account.admin`.
    pub async fn delete_credential_profile_key(
        &self,
        profile: &str,
        credential: &str,
    ) -> Result<CredentialProfileResponse, Error> {
        Self::send(self.request(
            Method::DELETE,
            &["credential-profiles", profile, "keys", credential],
        )?)
        .await
    }

    /// List the accounts of a credential. Requires `account.admin`.
    pub async fn list_credential_accounts(
        &self,
        profile: &str,
        credential: &str,
    ) -> Result<CredentialAccountsResponse, Error> {
        Self::send(self.request(
            Method::GET,
            &["credential-profiles", profile, "accounts", credential],
        )?)
        .await
    }

    /// Add another API key to a credential. Requires `account.admin`.
    pub async fn add_credential_account(
        &self,
        profile: &str,
        credential: &str,
        body: &SetCredentialKeyRequest,
    ) -> Result<CredentialAccountsResponse, Error> {
        Self::send(
            self.request(
                Method::POST,
                &["credential-profiles", profile, "accounts", credential],
            )?
            .json(body),
        )
        .await
    }

    /// Switch a credential to another account. Requires `account.admin`.
    pub async fn switch_credential_account(
        &self,
        profile: &str,
        credential: &str,
        account: &str,
    ) -> Result<CredentialAccountsResponse, Error> {
        Self::send(self.request(
            Method::PUT,
            &[
                "credential-profiles",
                profile,
                "accounts",
                credential,
                account,
                "active",
            ],
        )?)
        .await
    }

    /// Forget an account of a credential. Requires `account.admin`.
    pub async fn delete_credential_account(
        &self,
        profile: &str,
        credential: &str,
        account: &str,
    ) -> Result<CredentialAccountsResponse, Error> {
        Self::send(self.request(
            Method::DELETE,
            &[
                "credential-profiles",
                profile,
                "accounts",
                credential,
                account,
            ],
        )?)
        .await
    }

    /// Set a custom API. Requires `account.admin`.
    pub async fn set_credential_profile_custom_api(
        &self,
        profile: &str,
        name: &str,
        body: &SetCustomApiRequest,
    ) -> Result<CredentialProfileResponse, Error> {
        Self::send(
            self.request(
                Method::PUT,
                &["credential-profiles", profile, "custom-apis", name],
            )?
            .json(body),
        )
        .await
    }

    /// Remove a custom API. Requires `account.admin`.
    pub async fn delete_credential_profile_custom_api(
        &self,
        profile: &str,
        name: &str,
    ) -> Result<CredentialProfileResponse, Error> {
        Self::send(self.request(
            Method::DELETE,
            &["credential-profiles", profile, "custom-apis", name],
        )?)
        .await
    }

    /// Get AI credentials proxy. Requires `account.admin`.
    pub async fn get_ai_proxy(&self, org: Option<&str>) -> Result<AiProxySettings, Error> {
        Self::send(
            self.request(Method::GET, &["ai-proxy"])?
                .query(&[("org", org)]),
        )
        .await
    }

    /// Turn the AI credentials proxy on or off. Requires `account.admin`.
    pub async fn update_ai_proxy(
        &self,
        org: Option<&str>,
        body: &UpdateAiProxyRequest,
    ) -> Result<AiProxySettings, Error> {
        Self::send(
            self.request(Method::PUT, &["ai-proxy"])?
                .query(&[("org", org)])
                .json(body),
        )
        .await
    }

    /// Get AI usage. Requires `account.read`.
    pub async fn get_ai_usage(
        &self,
        query: &GetAiUsageQuery,
    ) -> Result<AccountAiUsageResponse, Error> {
        Self::send(self.request(Method::GET, &["ai-usage"])?.query(query)).await
    }

    /// Get sandbox AI usage. Requires `sandbox.read`.
    pub async fn get_sandbox_ai_usage(
        &self,
        sandbox_id: &str,
        days: Option<u32>,
    ) -> Result<SandboxAiUsageResponse, Error> {
        Self::send(
            self.request(Method::GET, &["sandboxes", sandbox_id, "ai-usage"])?
                .query(&[("days", days)]),
        )
        .await
    }

    /// Block or unblock sandbox AI access. Requires `account.admin`.
    pub async fn set_sandbox_ai_access(
        &self,
        sandbox_id: &str,
        body: &SetSandboxAiAccessRequest,
    ) -> Result<SandboxAiAccessResponse, Error> {
        Self::send(
            self.request(Method::PUT, &["sandboxes", sandbox_id, "ai-access"])?
                .json(body),
        )
        .await
    }
}
