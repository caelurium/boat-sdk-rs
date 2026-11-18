//! Request and response types for Boat environments.

/// Add Environment Repo Request.
#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddEnvironmentRepoRequest {
    /// The repository `databaseId` from `GET /repos`.
    pub repository_id: String,
    /// Boat `baseBranch` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_branch: Option<String>,
}

/// Create Sandbox Environment Request.
#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSandboxEnvironmentRequest {
    /// Unique environment name.
    pub name: String,
}

/// Environment Item Change Response.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnvironmentItemChangeResponse {
    /// Boat `success` field.
    pub success: bool,
    /// Id of the newly minted environment version.
    pub version_id: Option<String>,
    /// Boat `versionNumber` field.
    pub version_number: Option<i64>,
}

/// Environment Repository.
#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EnvironmentRepository {
    /// Internal repository databaseId.
    pub repository_id: String,
    /// Boat `baseBranch` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_branch: Option<String>,
    /// Boat `setupScript` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_script: Option<String>,
    /// Boat `setupBlocking` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_blocking: Option<bool>,
}

/// Repo Selection Request.
#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RepoSelectionRequest {
    /// Internal repository `databaseId` returned by `GET /repos`.
    pub repository_id: String,
    /// Boat `baseBranch` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_branch: Option<String>,
}

/// Repo Selection Response.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RepoSelectionResponse {
    /// Boat `ok` field.
    pub ok: bool,
    /// Stable success envelope discriminator added by v1.
    #[serde(rename = "type")]
    pub kind: String,
    /// Boat `success` field.
    pub success: bool,
    /// Boat `environmentId` field.
    pub environment_id: String,
    /// Id of the default environment.
    pub subenvironment_id: Option<String>,
    /// Boat `selectedRepositories` field.
    pub selected_repositories: Vec<SelectedRepository>,
}

/// Repos Query.
#[derive(Clone, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReposQuery {
    /// When true, sync from GitHub before returning repository groups.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync: Option<bool>,
    /// Maximum items to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Opaque pagination cursor returned as `pageInfo.nextCursor`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Sort direction for cursor pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    /// Case-insensitive repository name/fullName filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
    /// Filter to selected or unselected repositories.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected: Option<bool>,
}

/// Repos Response.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReposResponse {
    /// Boat `ok` field.
    pub ok: bool,
    /// Stable success envelope discriminator added by v1.
    #[serde(rename = "type")]
    pub kind: String,
    /// Boat `installations` field.
    pub installations: Vec<RepositoryInstallation>,
    /// Boat `environmentId` field.
    pub environment_id: String,
    /// Id of the default environment.
    pub subenvironment_id: Option<String>,
    /// Boat `selectedRepositories` field.
    pub selected_repositories: Vec<SelectedRepository>,
    /// Boat `pageInfo` field.
    pub page_info: Option<crate::sandbox::resources::PageInfo>,
}

/// Repository.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Repository {
    /// GitHub repository id.
    pub id: Option<i64>,
    /// Internal repository id used when selecting repositories.
    pub database_id: Option<String>,
    /// Boat `name` field.
    pub name: Option<String>,
    /// Boat `fullName` field.
    pub full_name: Option<String>,
    /// Boat `description` field.
    pub description: Option<String>,
    /// Boat `url` field.
    pub url: Option<String>,
    /// Boat `private` field.
    pub private: Option<bool>,
    /// Boat `permissions` field.
    pub permissions: Option<String>,
    /// Boat `pushedAt` field.
    pub pushed_at: Option<String>,
    /// Reserved; always null.
    pub last_user_commit_at: Option<String>,
}

/// Repository Installation.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RepositoryInstallation {
    /// Boat `type` field.
    #[serde(rename = "type")]
    pub kind: Option<String>,
    /// Boat `accountLogin` field.
    pub account_login: Option<String>,
    /// Boat `accountAvatarUrl` field.
    pub account_avatar_url: Option<String>,
    /// Boat `repositories` field.
    pub repositories: Option<Vec<Repository>>,
}

/// Sandbox Environment.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SandboxEnvironment {
    /// Boat `id` field.
    pub id: String,
    /// Boat `name` field.
    pub name: String,
    /// Exactly one environment is the default; sandboxes created without an `environment` name use it.
    pub is_default: bool,
    /// Boat `latestVersionId` field.
    pub latest_version_id: Option<String>,
    /// Number of the latest version, 0 when there is none.
    pub latest_version_number: Option<i64>,
    /// When the owner last answered whether this environment is safe for third parties, or null if never
    /// asked.
    pub safety_answered_at: Option<String>,
    /// When true the environment passes nothing to a sandbox (repos, secrets, and all credentials
    /// withheld), overriding the fine-grained flags below.
    pub safe_for_third_parties: bool,
    /// Attach the environment's GitHub repositories and the GitHub token (so `gh` and pushes work).
    pub pass_github: bool,
    /// Attach the environment's env variables and secret files.
    pub pass_secrets: bool,
    /// Attach the sandbox's own service/preview credentials.
    pub pass_sandbox_credentials: bool,
    /// Attach agent-provider credentials configured on the Agents page.
    pub pass_agents_credentials: bool,
    /// The AI credential profile the environment's sandboxes get.
    pub credential_profile_id: Option<String>,
    /// Only with `safeForThirdParties` true.
    pub proxy_ai_credentials: Option<bool>,
    /// Boat `access` field.
    pub access: Option<crate::account::models::ResourceAccess>,
    /// The latest version's .env-style content.
    pub env_contents: crate::SecretString,
    /// Boat `secretFiles` field.
    pub secret_files: Vec<SecretFile>,
    /// Repositories attached to the latest version, with base branch and setup script.
    pub selected_repositories: Option<Vec<SelectedRepository>>,
    /// Boat `versions` field.
    pub versions: Vec<SandboxEnvironmentVersionSummary>,
}

/// Sandbox Environment List Response.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SandboxEnvironmentListResponse {
    /// Boat `ok` field.
    pub ok: Option<bool>,
    /// Boat `type` field.
    #[serde(rename = "type")]
    pub kind: Option<String>,
    /// Boat `environments` field.
    pub environments: Vec<SandboxEnvironment>,
}

/// Sandbox Environment Response.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SandboxEnvironmentResponse {
    /// Boat `success` field.
    pub success: bool,
    /// Boat `environment` field.
    pub environment: Option<SandboxEnvironment>,
}

/// Sandbox Environment Version Summary.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SandboxEnvironmentVersionSummary {
    /// Boat `id` field.
    pub id: String,
    /// Monotonically increasing per environment; version 1 is the first.
    pub version_number: i64,
    /// Number of the caller's active sandboxes currently pinned to this version.
    pub sandbox_count: i64,
    /// Boat `createdAt` field.
    pub created_at: String,
}

/// Secret File.
#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecretFile {
    /// Boat `path` field.
    pub path: String,
    /// Secret file contents.
    pub contents: crate::SecretString,
}

/// Secrets Response.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecretsResponse {
    /// Boat `ok` field.
    pub ok: bool,
    /// Stable success envelope discriminator added by v1.
    #[serde(rename = "type")]
    pub kind: String,
    /// Boat `success` field.
    pub success: Option<bool>,
    /// Boat `environmentId` field.
    pub environment_id: String,
    /// Id of the default environment.
    pub subenvironment_id: Option<String>,
    /// Boat `envContents` field.
    pub env_contents: crate::SecretString,
    /// Boat `secretFiles` field.
    pub secret_files: Vec<SecretFile>,
    /// Present on update; counts how many active sandboxes received the new environment.
    pub pushed: Option<std::collections::BTreeMap<String, serde_json::Value>>,
}

/// Secrets Update Request.
#[derive(Clone, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SecretsUpdateRequest {
    /// Full .env-style content to sync into sandboxes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env_contents: Option<crate::SecretString>,
    /// Full list of secret files to keep configured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_files: Option<Vec<SecretFile>>,
}

/// Selected Repository.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelectedRepository {
    /// GitHub repository id.
    pub id: Option<i64>,
    /// Internal repository id used when selecting repositories.
    pub database_id: Option<String>,
    /// Boat `name` field.
    pub name: Option<String>,
    /// Boat `fullName` field.
    pub full_name: Option<String>,
    /// Boat `description` field.
    pub description: Option<String>,
    /// Boat `url` field.
    pub url: Option<String>,
    /// Boat `private` field.
    pub private: Option<bool>,
    /// Boat `permissions` field.
    pub permissions: Option<String>,
    /// Boat `pushedAt` field.
    pub pushed_at: Option<String>,
    /// Reserved; always null.
    pub last_user_commit_at: Option<String>,
    /// Boat `baseBranch` field.
    pub base_branch: Option<String>,
    /// Boat `setupRoutineId` field.
    pub setup_routine_id: Option<String>,
    /// Boat `setupScript` field.
    pub setup_script: Option<String>,
    /// Boat `setupBlocking` field.
    pub setup_blocking: Option<bool>,
}

/// Set Environment Secret File Request.
#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetEnvironmentSecretFileRequest {
    /// In-sandbox path relative to the workspace, e.g.
    pub path: String,
    /// Boat `contents` field.
    pub contents: crate::SecretString,
}

/// Set Environment Var Request.
#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetEnvironmentVarRequest {
    /// Boat `value` field.
    pub value: crate::SecretString,
}

/// Update Sandbox Environment Request.
#[derive(Clone, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSandboxEnvironmentRequest {
    /// New environment name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Set to true to make this the default environment (clears the flag on the previous default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    /// Boat `safeForThirdParties` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safe_for_third_parties: Option<bool>,
    /// Boat `passGithub` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pass_github: Option<bool>,
    /// Boat `passSecrets` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pass_secrets: Option<bool>,
    /// Boat `passSandboxCredentials` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pass_sandbox_credentials: Option<bool>,
    /// Boat `passAgentsCredentials` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pass_agents_credentials: Option<bool>,
    /// Id or name of a credential profile you can use, or null for the sandbox owner's default profile.
    /// `None` leaves it unchanged; `Some(None)` explicitly clears it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_profile_id: Option<Option<String>>,
    /// With `safeForThirdParties` true, pass AI credentials as proxy placeholders.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_ai_credentials: Option<bool>,
    /// Boat `access` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access: Option<crate::account::models::ResourceAccessUpdate>,
    /// Full .env-style content for the new version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env_contents: Option<crate::SecretString>,
    /// Boat `secretFiles` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_files: Option<Vec<SecretFile>>,
    /// Full replacement of the version's repository selection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repositories: Option<Vec<EnvironmentRepository>>,
}

/// Upgrade Sandbox Environment Request.
#[derive(Clone, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpgradeSandboxEnvironmentRequest {
    /// Restrict the upgrade to these sandbox (agent) ids.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_ids: Option<Vec<String>>,
}

/// Upgrade Sandbox Environment Response.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpgradeSandboxEnvironmentResponse {
    /// Boat `success` field.
    pub success: bool,
    /// Boat `upgraded` field.
    pub upgraded: i64,
    /// Running sandboxes that received the new configuration now.
    pub upgraded_live: Option<i64>,
    /// Stopped sandboxes moved to the latest version; they pick it up when they next resume.
    pub upgraded_stopped: Option<i64>,
    /// Boat `failed` field.
    pub failed: i64,
    /// Id of the version the sandboxes were moved to.
    pub latest_version_id: Option<String>,
}
