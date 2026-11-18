//! Request and response types for Boat ai credentials.

/// Account Ai Usage Response.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountAiUsageResponse {
    /// Boat `ok` field.
    pub ok: bool,
    /// Stable success envelope discriminator added by v1.
    #[serde(rename = "type")]
    pub kind: String,
    /// The wallet the usage is charged to.
    pub account_id: Option<String>,
    /// The credential profile whose requests are counted.
    pub profile_id: Option<String>,
    /// First UTC day included.
    pub since: String,
    /// Boat `sandboxes` field.
    pub sandboxes: Vec<SandboxAiUsage>,
    /// Boat `usage` field.
    pub usage: Vec<AiUsageRow>,
    /// Boat `totals` field.
    pub totals: AiUsageTotals,
    /// Boat `dollars` field.
    pub dollars: f64,
}

/// Ai Proxy Settings.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiProxySettings {
    /// Boat `ok` field.
    pub ok: bool,
    /// Stable success envelope discriminator added by v1.
    #[serde(rename = "type")]
    pub kind: String,
    /// Boat `success` field.
    pub success: bool,
    /// The account the switch belongs to, a person or an organization.
    pub account_id: String,
    /// True when `accountId` is an organization.
    pub org: bool,
    /// Boat `enabled` field.
    pub enabled: bool,
    /// Boat `enabledAt` field.
    pub enabled_at: Option<String>,
    /// Boat `pricing` field.
    pub pricing: AiProxySettingsPricing,
    /// The base URL sandboxes call instead of the providers.
    pub proxy_url: String,
}

/// Ai Proxy Settings Pricing.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiProxySettingsPricing {
    /// Charged to the account for each UTC day the proxy is on.
    pub daily_dollars: f64,
    /// Charged per 1,000 proxied requests.
    pub dollars_per1000_requests: f64,
}

/// Ai Usage Row.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiUsageRow {
    /// UTC day.
    pub day: String,
    /// Boat `credential` field.
    pub credential: String,
    /// The model the provider reported, or empty.
    pub model: String,
    /// Boat `requests` field.
    pub requests: i64,
    /// Requests the provider answered with an HTTP error, or did not answer.
    pub errors: i64,
    /// Boat `inputTokens` field.
    pub input_tokens: i64,
    /// Boat `outputTokens` field.
    pub output_tokens: i64,
    /// Boat `cacheReadTokens` field.
    pub cache_read_tokens: i64,
    /// Boat `cacheWriteTokens` field.
    pub cache_write_tokens: i64,
}

/// Ai Usage Totals.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiUsageTotals {
    /// Boat `requests` field.
    pub requests: i64,
    /// Requests the provider answered with an HTTP error, or did not answer.
    pub errors: i64,
    /// Boat `inputTokens` field.
    pub input_tokens: i64,
    /// Boat `outputTokens` field.
    pub output_tokens: i64,
    /// Boat `cacheReadTokens` field.
    pub cache_read_tokens: i64,
    /// Boat `cacheWriteTokens` field.
    pub cache_write_tokens: i64,
}

/// Create Credential Profile Request.
#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCredentialProfileRequest {
    /// Unique among the owner's profiles.
    pub name: String,
    /// Organization id or name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org: Option<String>,
    /// Id or name of a profile you manage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_from: Option<String>,
}

/// Credential Account.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CredentialAccount {
    /// The account's id.
    pub id: String,
    /// The account's email for a sign-in, or the key's last characters (…1234).
    pub label: String,
    /// True for the account every sandbox on the profile uses.
    pub active: bool,
    /// When the account was set aside (accounts not in use).
    pub added_at: Option<String>,
}

/// Credential Accounts Response.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CredentialAccountsResponse {
    /// Boat `ok` field.
    pub ok: bool,
    /// Stable success envelope discriminator added by v1.
    #[serde(rename = "type")]
    pub kind: String,
    /// Boat `success` field.
    pub success: bool,
    /// Boat `profileId` field.
    pub profile_id: String,
    /// Boat `credential` field.
    pub credential: String,
    /// In the order the accounts were added, so their numbers stay the same across switches.
    pub accounts: Vec<CredentialAccount>,
}

/// Credential Profile.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CredentialProfile {
    /// Boat `id` field.
    pub id: String,
    /// Boat `name` field.
    pub name: String,
    /// True for the one profile of its owner that sandboxes get when nothing names a profile.
    pub is_default: bool,
    /// Boat `owner` field.
    pub owner: CredentialProfileOwner,
    /// True when you can change the profile.
    pub can_manage: bool,
    /// The credentials the profile holds.
    pub connected: Vec<String>,
    /// Credentials with several accounts, keyed by credential id.
    pub accounts: Option<std::collections::BTreeMap<String, Vec<CredentialAccount>>>,
    /// The names of the profile's custom APIs.
    pub custom_apis: Vec<String>,
    /// Boat `createdAt` field.
    pub created_at: String,
    /// Boat `updatedAt` field.
    pub updated_at: String,
    /// Boat `access` field.
    pub access: Option<crate::account::models::ResourceAccess>,
}

/// Credential Profile Deleted Response.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CredentialProfileDeletedResponse {
    /// Boat `ok` field.
    pub ok: bool,
    /// Stable success envelope discriminator added by v1.
    #[serde(rename = "type")]
    pub kind: String,
    /// Boat `success` field.
    pub success: bool,
}

/// Credential Profile List Response.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CredentialProfileListResponse {
    /// Boat `ok` field.
    pub ok: bool,
    /// Stable success envelope discriminator added by v1.
    #[serde(rename = "type")]
    pub kind: String,
    /// Boat `profiles` field.
    pub profiles: Vec<CredentialProfile>,
}

/// Credential Profile Owner.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CredentialProfileOwner {
    /// The account that owns the profile, a person or an organization.
    pub id: String,
    /// Boat `kind` field.
    pub kind: String,
    /// Boat `name` field.
    pub name: String,
    /// True when the owner has the AI credentials proxy on.
    pub ai_proxy_enabled: bool,
}

/// Credential Profile Response.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CredentialProfileResponse {
    /// Boat `ok` field.
    pub ok: bool,
    /// Stable success envelope discriminator added by v1.
    #[serde(rename = "type")]
    pub kind: String,
    /// Boat `success` field.
    pub success: bool,
    /// Boat `profile` field.
    pub profile: CredentialProfile,
}

/// Get Ai Usage Query.
#[derive(Clone, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAiUsageQuery {
    /// Days of history, today included.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days: Option<i64>,
    /// A credential profile id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,
    /// Billing wallet for this request: an organization you belong to, by id (`team_…`) or by name as `GET
    /// /orgs` lists it (case-insensitive), or `personal`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org: Option<String>,
}

/// Sandbox Ai Access Response.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SandboxAiAccessResponse {
    /// Boat `ok` field.
    pub ok: bool,
    /// Stable success envelope discriminator added by v1.
    #[serde(rename = "type")]
    pub kind: String,
    /// Boat `success` field.
    pub success: bool,
    /// Boat `sandboxId` field.
    pub sandbox_id: String,
    /// Boat `blocked` field.
    pub blocked: bool,
}

/// Sandbox Ai Usage.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SandboxAiUsage {
    /// Boat `sandboxId` field.
    pub sandbox_id: Option<String>,
    /// Boat `name` field.
    pub name: Option<String>,
    /// Boat `blocked` field.
    pub blocked: bool,
    /// Boat `usage` field.
    pub usage: Vec<AiUsageRow>,
    /// Boat `totals` field.
    pub totals: AiUsageTotals,
    /// Request charge for the period, at the price per 1,000 requests.
    pub dollars: f64,
}

/// Sandbox Ai Usage Response.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SandboxAiUsageResponse {
    /// Boat `ok` field.
    pub ok: bool,
    /// Stable success envelope discriminator added by v1.
    #[serde(rename = "type")]
    pub kind: String,
    /// Boat `sandboxId` field.
    pub sandbox_id: String,
    /// True when the proxy refuses this sandbox's placeholders.
    pub blocked: bool,
    /// True when the sandbox holds placeholders, not real keys.
    pub proxied: bool,
    /// The profile the sandbox uses.
    pub credential_profile_id: Option<String>,
    /// Boat `since` field.
    pub since: String,
    /// Boat `usage` field.
    pub usage: Vec<AiUsageRow>,
    /// Boat `totals` field.
    pub totals: AiUsageTotals,
    /// Boat `dollars` field.
    pub dollars: f64,
}

/// Set Credential Key Request.
#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetCredentialKeyRequest {
    /// The API key.
    pub value: crate::SecretString,
    /// Keep the key this one replaces as another account of the credential, ready to switch back to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_current: Option<bool>,
}

/// Set Custom Api Request.
#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetCustomApiRequest {
    /// The secret.
    pub value: crate::SecretString,
    /// Public HTTPS base URL of the API, on the default port, with a DNS name.
    pub base_url: String,
}

/// Set Sandbox Ai Access Request.
#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetSandboxAiAccessRequest {
    /// Boat `blocked` field.
    pub blocked: bool,
}

/// Update Ai Proxy Request.
#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAiProxyRequest {
    /// Boat `enabled` field.
    pub enabled: bool,
}

/// Update Credential Profile Request.
#[derive(Clone, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCredentialProfileRequest {
    /// New profile name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Send true to make this profile its owner's default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    /// Organization profiles only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access: Option<crate::account::models::ResourceAccessUpdate>,
}
