//! Request and response types for Boat api keys.

/// Api Key.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKey {
    /// Boat `id` field.
    pub id: String,
    /// Boat `name` field.
    pub name: String,
    /// Credential storage lane.
    pub credential_lane: String,
    /// Boat `keyPrefix` field.
    pub key_prefix: String,
    /// Boat `keyLastFour` field.
    pub key_last_four: String,
    /// Sandbox ID for a platform-managed machine key, or null for a user-created key.
    pub sandbox_id: Option<String>,
    /// Boat `createdAt` field.
    pub created_at: String,
    /// Boat `lastUsedAt` field.
    pub last_used_at: Option<String>,
    /// Boat `usage` field.
    pub usage: ApiKeyRequestUsage,
    /// Boat `resources` field.
    pub resources: ApiKeyResourceTotals,
    /// Boat `expiresAt` field.
    pub expires_at: Option<String>,
    /// Boat `expired` field.
    pub expired: Option<bool>,
    /// Boat `expiringSoon` field.
    pub expiring_soon: Option<bool>,
    /// Boat `scope` field.
    pub scope: Option<ApiKeyScope>,
}

/// Api Key Catalog.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKeyCatalog {
    /// Boat `actions` field.
    pub actions: Vec<String>,
    /// Boat `presets` field.
    pub presets: std::collections::BTreeMap<String, Vec<String>>,
    /// Boat `defaultTtl` field.
    pub default_ttl: String,
    /// Boat `maxTtl` field.
    pub max_ttl: String,
    /// Whether POST /api-keys/scoped currently accepts creation requests.
    pub scoped_creation_enabled: bool,
    /// Session-only selector resources.
    pub scope_resources: Option<ScopeResources>,
}

/// Api Key Created Resource.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKeyCreatedResource {
    /// Boat `kind` field.
    pub kind: String,
    /// Boat `id` field.
    pub id: String,
    /// Boat `name` field.
    pub name: String,
    /// Boat `state` field.
    pub state: String,
    /// Boat `createdAt` field.
    pub created_at: Option<String>,
}

/// Api Key Request Usage.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKeyRequestUsage {
    /// Requests authenticated with this key during the current UTC day and the previous 29 UTC days.
    pub requests: i64,
    /// Boat `windowDays` field.
    pub window_days: i64,
}

/// Api Key Resource Totals.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKeyResourceTotals {
    /// Boat `total` field.
    pub total: i64,
    /// Boat `sandboxes` field.
    pub sandboxes: i64,
    /// Boat `agents` field.
    pub agents: i64,
}

/// Api Key Revoke Response.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKeyRevokeResponse {
    /// Boat `ok` field.
    pub ok: bool,
    /// Stable success envelope discriminator added by v1.
    #[serde(rename = "type")]
    pub kind: String,
    /// Boat `apiKeys` field.
    pub api_keys: Vec<ApiKey>,
}

/// Api Key Scope.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKeyScope {
    /// Boat `expiresAt` field.
    pub expires_at: Option<String>,
    /// Boat `actions` field.
    pub actions: Option<Vec<String>>,
    /// Boat `sandboxes` field.
    pub sandboxes: Option<super::ResourceScope>,
    /// Boat `environments` field.
    pub environments: Option<super::ResourceScope>,
    /// Boat `grandfathered` field.
    pub grandfathered: Option<bool>,
}

/// Api Key Scope Resource.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKeyScopeResource {
    /// Boat `id` field.
    pub id: String,
    /// Boat `name` field.
    pub name: String,
    /// Boat `organizationId` field.
    pub organization_id: String,
    /// Boat `organizationName` field.
    pub organization_name: String,
}

/// Api Key Secret Response.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKeySecretResponse {
    /// Boat `ok` field.
    pub ok: bool,
    /// Stable success envelope discriminator added by v1.
    #[serde(rename = "type")]
    pub kind: String,
    /// Boat `apiKey` field.
    pub api_key: ApiKey,
    /// Raw secret returned once.
    pub secret: crate::SecretString,
    /// Boat `apiKeys` field.
    pub api_keys: Vec<ApiKey>,
}

/// Api Key Usage Response.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKeyUsageResponse {
    /// Boat `ok` field.
    pub ok: bool,
    /// Stable success envelope discriminator added by v1.
    #[serde(rename = "type")]
    pub kind: String,
    /// Metadata, scope, and usage counters of the queried key.
    #[serde(flatten)]
    pub api_key: ApiKey,
    /// Boat `createdResources` field.
    pub created_resources: Vec<ApiKeyCreatedResource>,
}

/// Api Keys Response.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKeysResponse {
    /// Boat `ok` field.
    pub ok: bool,
    /// Stable success envelope discriminator added by v1.
    #[serde(rename = "type")]
    pub kind: String,
    /// Boat `apiKeys` field.
    pub api_keys: Vec<ApiKey>,
    /// Boat `catalog` field.
    pub catalog: ApiKeyCatalog,
}

/// Create Scoped Api Key Request.
#[derive(Clone, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateScopedApiKeyRequest {
    /// Boat `name` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Duration such as 90d, 24h, or seconds, up to 365d; or `never` for a key that works until revoked.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<String>,
    /// Boat `preset` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preset: Option<super::KeyPreset>,
    /// Boat `actions` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<String>>,
    /// Boat `sandboxIds` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sandbox_ids: Option<Vec<String>>,
    /// Boat `environmentIds` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_ids: Option<Vec<String>>,
}

/// Scope Resources.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScopeResources {
    /// Boat `sandboxes` field.
    pub sandboxes: Vec<ApiKeyScopeResource>,
    /// Boat `environments` field.
    pub environments: Vec<ApiKeyScopeResource>,
}
