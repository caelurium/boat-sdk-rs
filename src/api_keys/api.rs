use super::models::*;
use crate::{Client, Error};
use reqwest::Method;

impl Client {
    /// List API keys. Restricted callers receive an empty inventory.
    pub async fn api_keys(&self) -> Result<ApiKeysResponse, Error> {
        Self::send(self.request(Method::GET, &["api-keys"])?).await
    }

    /// Create a scoped API key. Requires admin authority; the returned secret is available only once.
    pub async fn create_scoped_api_key(
        &self,
        body: &CreateScopedApiKeyRequest,
    ) -> Result<ApiKeySecretResponse, Error> {
        Self::send(
            self.request(Method::POST, &["api-keys", "scoped"])?
                .json(body),
        )
        .await
    }

    /// Revoke an API key. Requires admin authority over the target key.
    pub async fn revoke_api_key(&self, api_key_id: &str) -> Result<ApiKeyRevokeResponse, Error> {
        Self::send(self.request(Method::DELETE, &["api-keys", api_key_id])?).await
    }

    /// Rotate an API key. Requires admin authority; the returned secret is available only once.
    pub async fn rotate_api_key(&self, api_key_id: &str) -> Result<ApiKeySecretResponse, Error> {
        Self::send(self.request(Method::POST, &["api-keys", api_key_id, "rotate"])?).await
    }

    /// Get API key usage.
    pub async fn api_key_usage(&self, api_key_id: &str) -> Result<ApiKeyUsageResponse, Error> {
        Self::send(self.request(Method::GET, &["api-keys", api_key_id, "usage"])?).await
    }
}
