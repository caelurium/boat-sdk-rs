use reqwest::{RequestBuilder, Response, Url};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fmt;
use std::time::Duration;

use crate::{Config, Error};

/// Reusable asynchronous Boat API client. Clones share the connection pool.
#[derive(Clone)]
pub struct Client {
    http: reqwest::Client,
    base: Url,
}

impl Client {
    /// Connect to the hosted API using a Boat API key.
    pub fn new(api_key: &str) -> Result<Self, Error> {
        Self::from_config(Config::new(api_key)?)
    }

    /// Build a client for the configured root and organization.
    pub fn from_config(config: Config) -> Result<Self, Error> {
        let http = reqwest::Client::builder()
            .default_headers(config.headers)
            .redirect(reqwest::redirect::Policy::none())
            .connect_timeout(Duration::from_secs(30))
            .build()
            .map_err(|_| Error::Configuration)?;
        Ok(Self {
            http,
            base: config.base,
        })
    }

    fn endpoint(&self, path: &[&str]) -> Result<Url, Error> {
        let mut url = self.base.clone();
        if path.is_empty() || path.iter().any(|s| matches!(*s, "" | "." | "..")) {
            return Err(Error::Request);
        }
        url.path_segments_mut()
            .map_err(|_| Error::Request)?
            .pop_if_empty()
            .extend(path);
        Ok(url)
    }

    /// GET a JSON endpoint with path segments relative to the API root and
    /// URL-encoded query parameters. Pass `&()` for an empty query.
    /// Buffered JSON responses are limited to 16 MiB.
    pub async fn get_json<T: DeserializeOwned, Q: Serialize + ?Sized>(
        &self,
        path: &[&str],
        query: &Q,
    ) -> Result<T, Error> {
        Self::send(self.http.get(self.endpoint(path)?).query(query)).await
    }

    /// POST a serializable JSON body and decode the JSON response (up to 16 MiB).
    pub async fn post_json<T: DeserializeOwned, B: Serialize + ?Sized>(
        &self,
        path: &[&str],
        body: &B,
    ) -> Result<T, Error> {
        Self::send(self.http.post(self.endpoint(path)?).json(body)).await
    }

    async fn send<T: DeserializeOwned>(request: RequestBuilder) -> Result<T, Error> {
        let response = request.send().await.map_err(|e| {
            if e.is_builder() {
                Error::Request
            } else {
                Error::Transport
            }
        })?;
        let status = response.status();
        if !status.is_success() {
            let header_id = response
                .headers()
                .get("x-request-id")
                .and_then(|v| v.to_str().ok())
                .map(str::to_owned);
            let envelope: Envelope = match read_body(response, 64 * 1024).await {
                Ok(bytes) => serde_json::from_slice(&bytes).unwrap_or_default(),
                Err(_) => Envelope::default(),
            };
            return Err(Error::Api {
                status,
                code: envelope
                    .code
                    .or_else(|| envelope.error.and_then(|e| e.code)),
                request_id: envelope.request_id.or(header_id),
            });
        }
        let body = read_body(response, 16 * 1024 * 1024).await?;
        serde_json::from_slice(&body).map_err(|_| Error::Decode)
    }
}

impl fmt::Debug for Client {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Client").finish_non_exhaustive()
    }
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Envelope {
    code: Option<String>,
    error: Option<Detail>,
    request_id: Option<String>,
}

#[derive(Deserialize)]
struct Detail {
    code: Option<String>,
}

async fn read_body(mut response: Response, limit: usize) -> Result<Vec<u8>, Error> {
    let mut bytes = Vec::new();
    while let Some(chunk) = response.chunk().await.map_err(|_| Error::Transport)? {
        if chunk.len() > limit - bytes.len() {
            return Err(Error::ResponseTooLarge);
        }
        bytes.extend_from_slice(&chunk);
    }
    Ok(bytes)
}
