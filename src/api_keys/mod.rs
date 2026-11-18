//! Scoped Boat API keys and their usage. Create and rotate return a one-time
//! [`crate::SecretString`]; later inventory reads (`account.read`) return metadata
//! only. Key management requires an account key with admin authority; the caller
//! cannot create or manage keys outside its own allowed scope.

mod api;
pub mod models;

/// Boat's built-in permission presets for issuing a scoped key.
#[derive(Clone, Copy, serde::Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum KeyPreset {
    /// Read-only sandbox and account operations.
    ReadOnly,
    /// Sandbox operations.
    FullSandbox,
    /// Continuous integration operations.
    Ci,
    /// Explicit administrative scope.
    Admin,
}

/// Resources a scoped key may access.
#[derive(Clone, serde::Deserialize)]
#[serde(untagged)]
pub enum ResourceScope {
    /// Wildcard (`"*"`).
    Wildcard(Wildcard),
    /// An explicit allowlist of sandbox or environment IDs.
    Ids(Vec<String>),
}

/// The literal wildcard in a Boat resource scope.
#[derive(Clone, Copy, serde::Deserialize)]
pub enum Wildcard {
    /// Every resource.
    #[serde(rename = "*")]
    All,
}
