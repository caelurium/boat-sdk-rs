//! Access a sandbox over SSH, through its desktop, or via a hosted service.

pub mod models;

pub use crate::sandbox::requests::HostPortRequest;
use crate::{Client, Error};
use models::{DesktopRequest, DesktopResponse, HostPortResponse, SshKeyRequest, SshKeyResponse};
use reqwest::Method;

/// Select a desktop streaming mode.
#[derive(Clone, Copy, Default)]
pub enum DesktopMode {
    /// Use the server's default mode and theme.
    #[default]
    Default,
    /// Light theme.
    Light,
    /// Dark theme.
    Dark,
    /// Use noVNC; `provisioning` indicates when polling is necessary.
    Vnc,
}

impl Client {
    /// Request a desktop URL. Private desktop URLs carry access tokens.
    pub async fn desktop(
        &self,
        id: &str,
        mode: DesktopMode,
        body: &DesktopRequest,
    ) -> Result<DesktopResponse, Error> {
        let request = self.request(Method::POST, &["sandboxes", id, "desktop"])?;
        let request = match mode {
            DesktopMode::Default => request,
            DesktopMode::Light => request.query(&[("theme", "light")]),
            DesktopMode::Dark => request.query(&[("theme", "dark")]),
            DesktopMode::Vnc => request.query(&[("vnc", "1")]),
        };
        Self::send(request.json(body)).await
    }

    /// Expose a sandbox port. Protected URLs contain an access token.
    pub async fn host_port(
        &self,
        id: &str,
        body: &HostPortRequest,
    ) -> Result<HostPortResponse, Error> {
        self.post_json(&["sandboxes", id, "host"], body).await
    }

    /// Install an OpenSSH public key and return connection and host-key details.
    pub async fn add_ssh_key(
        &self,
        id: &str,
        body: &SshKeyRequest,
    ) -> Result<SshKeyResponse, Error> {
        self.post_json(&["sandboxes", id, "sshkey"], body).await
    }
}
