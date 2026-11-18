//! Boat access request and response models.

/// Optional desktop/VNC setup options used by the Boat dashboard and CLI.
#[derive(Clone, serde::Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DesktopRequest {
    /// For `?vnc=1`, return a noVNC URL that does not require an access token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_access: Option<bool>,
}

/// Desktop Response.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DesktopResponse {
    /// Boat `ok` field.
    pub ok: bool,
    /// Boat `type` field.
    #[serde(rename = "type")]
    pub response_type: String,
    /// Boat `success` field.
    pub success: Option<bool>,
    /// Secret-bearing desktop or noVNC URL.
    pub desktop_url: Option<String>,
    /// Boat `ip` field.
    pub ip: Option<String>,
    /// Boat `mode` field.
    pub mode: Option<String>,
    /// For `?vnc=1`, true means VNC is still being prepared; poll again.
    pub provisioning: Option<bool>,
    /// Boat `message` field.
    pub message: Option<String>,
}

/// Host Port Response.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HostPortResponse {
    /// Boat `ok` field.
    pub ok: bool,
    /// Boat `type` field.
    #[serde(rename = "type")]
    pub response_type: String,
    /// Boat `success` field.
    pub success: Option<bool>,
    /// Boat `port` field.
    pub port: Option<u64>,
    /// Public HTTPS URL.
    pub url: Option<String>,
    /// Boat `isProtected` field.
    pub is_protected: Option<bool>,
    /// Boat `access` field.
    pub access: Option<String>,
    /// The sandbox relays the URL to a service that listens on localhost only.
    pub localhost: Option<bool>,
}

/// Ssh Key Request.
#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SshKeyRequest {
    /// Public SSH key in OpenSSH format.
    pub key: String,
}

/// Ssh Key Response.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SshKeyResponse {
    /// Boat `ok` field.
    pub ok: bool,
    /// Stable success envelope discriminator added by v1.
    #[serde(rename = "type")]
    pub response_type: String,
    /// Boat `success` field.
    pub success: Option<bool>,
    /// Boat `machineIp` field.
    pub machine_ip: Option<String>,
    /// Boat `sshUser` field.
    pub ssh_user: Option<String>,
    /// Public IPv4 `host:port` that forwards to the sandbox SSH server, set only when the machine has no public
    /// IPv4 of its own.
    pub ssh_endpoint: Option<String>,
    /// The sandbox SSH server's public host key.
    pub host_key: Option<String>,
}
