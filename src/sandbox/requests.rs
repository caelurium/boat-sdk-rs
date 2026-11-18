/// Initial File in the Boat API.
#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InitialFile {
    /// Boat `path` field.
    pub path: String,
    /// File contents, as text or base64 (see `encoding`).
    pub content: String,
    /// Boat `encoding` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
}

/// Create Sandbox Request in the Boat API.
#[derive(Clone, serde::Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreateSandboxRequest {
    /// Machine size.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub machine_type: Option<super::MachineSize>,
    /// Omit to keep the server default or existing TTL; `Disabled` sends explicit null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl_seconds: Option<super::AutoStop>,
    /// Key-value labels, for example your session id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<std::collections::BTreeMap<String, String>>,
    /// Per-sandbox environment variables injected into the sandbox's tool environment, on top of the
    /// account environment's variables (per-sandbox values win on conflicts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<std::collections::BTreeMap<String, String>>,
    /// Name of the sandbox environment to attach to this sandbox.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    /// Create a sandbox with none of the secrets attached to your account (no environment variables, secret
    /// files, or credentials), confined to itself so it cannot act on your account or other sandboxes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_env: Option<bool>,
    /// Pass `false` to never snapshot this sandbox.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshots: Option<bool>,
    /// Pass `false` for monitoring off.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring: Option<bool>,
    /// Pass `true` to get a sandbox only if a machine of this type is ready right now.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_fast: Option<bool>,
    /// With `from`: create this many sandboxes from the saved snapshot in one request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// Shell script that runs on the sandbox after it is ready.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_script: Option<String>,
    /// Display name of the sandbox, the same value `PATCH /sandboxes/{id}` sets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Who can use a sandbox billed to an organization, with the same rules as the sandbox's `visibility`
    /// field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
    /// Ports to host on public HTTPS URLs, each exactly as `POST /sandboxes/{id}/host` does.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_ports: Option<Vec<HostPortRequest>>,
    /// Files to write on the sandbox, each like `PUT /sandboxes/{id}/files` (a path under `/home/user` or
    /// `/tmp`, relative paths resolve against `/home/user`; missing folders are created).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<InitialFile>>,
    /// Bill this sandbox to an organization you belong to (its shared wallet), by id or by name as `GET
    /// /orgs` lists it; `personal` or your own account id means personal billing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org: Option<String>,
    /// Legacy alias for `org`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    /// Boat `from` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
}

/// Fork Request in the Boat API.
#[derive(Clone, serde::Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ForkRequest {
    /// Labels of the fork.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<std::collections::BTreeMap<String, String>>,
    /// Pass `true` to fork only if a machine is ready right now.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_fast: Option<bool>,
    /// `false` creates the fork with monitoring off: Boat keeps no command or process output, process list
    /// or CPU/RAM samples for it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring: Option<bool>,
    /// Replaces the env the fork would otherwise inherit from the source sandbox.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<std::collections::BTreeMap<String, String>>,
    /// Optionally pin the fork to a different named sandbox environment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    /// Make the fork no-env (see `CreateSandboxRequest.noEnv`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_env: Option<bool>,
    /// Machine size for the fork.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub machine_type: Option<super::MachineSize>,
    /// Omit to keep the server default or existing TTL; `Disabled` sends explicit null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl_seconds: Option<super::AutoStop>,
}

/// Host Port Request in the Boat API.
#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HostPortRequest {
    /// Port the service listens on inside the sandbox.
    pub port: i64,
    /// Return an ungated URL instead of one that requires the `_token` query parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
    /// Set when the service listens on localhost (`127.0.0.1` or `::1`) only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub localhost: Option<bool>,
    /// Display title for the hosted port.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// Resume Request in the Boat API.
#[derive(Clone, serde::Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResumeRequest {
    /// Pass `true` to resume only if a machine is ready right now.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_fast: Option<bool>,
    /// Resume onto a different machine size.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub machine_type: Option<super::MachineSize>,
    /// Replaces the sandbox's per-sandbox environment variables.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<std::collections::BTreeMap<String, String>>,
    /// Optionally re-pin the sandbox to a different named sandbox environment on resume.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    /// Resume into a no-env sandbox: withhold account secrets and scrub inherited owner secrets from the
    /// restored snapshot before the sandbox is exposed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_env: Option<bool>,
    /// Omit to keep the server default or existing TTL; `Disabled` sends explicit null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl_seconds: Option<super::AutoStop>,
}

/// Stop Request in the Boat API.
#[derive(Clone, serde::Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StopRequest {
    /// Stop the sandbox even if its disk cannot be snapshotted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
}

/// Update Sandbox Request in the Boat API.
#[derive(Clone, serde::Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSandboxRequest {
    /// New display name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Omit to keep the server default or existing TTL; `Disabled` sends explicit null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl_seconds: Option<super::AutoStop>,
    /// Boat `subdomain` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdomain: Option<String>,
}
