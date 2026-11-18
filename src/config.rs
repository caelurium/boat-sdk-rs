use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::Url;
use std::fmt;

use crate::Error;

/// Connection settings for the Boat API.
pub struct Config {
    pub(crate) base: Url,
    pub(crate) headers: HeaderMap,
}

impl Config {
    /// Configure the hosted Boat API with a bearer key.
    pub fn new(api_key: &str) -> Result<Self, Error> {
        if api_key.is_empty() || !api_key.bytes().all(|c| c.is_ascii_graphic()) {
            return Err(Error::Configuration);
        }
        let mut auth = HeaderValue::from_str(&format!("Bearer {api_key}"))
            .map_err(|_| Error::Configuration)?;
        auth.set_sensitive(true);
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, auth);
        Ok(Self {
            base: Url::parse("https://boat.dev/api/v1").expect("valid hosted API URL"),
            headers,
        })
    }

    /// Use a trusted HTTP(S) API root, including its path prefix.
    pub fn with_base_url(mut self, base: &str) -> Result<Self, Error> {
        let url = Url::parse(base).map_err(|_| Error::Configuration)?;
        if !matches!(url.scheme(), "http" | "https")
            || url.host().is_none()
            || !url.username().is_empty()
            || url.password().is_some()
            || url.query().is_some()
            || url.fragment().is_some()
        {
            return Err(Error::Configuration);
        }
        self.base = url;
        Ok(self)
    }

    /// Select an organization via the `X-Boat-Org` header.
    pub fn with_organization(mut self, organization: &str) -> Result<Self, Error> {
        if organization.trim().is_empty() {
            return Err(Error::Configuration);
        }
        let org = HeaderValue::from_str(organization).map_err(|_| Error::Configuration)?;
        self.headers.insert("x-boat-org", org);
        Ok(self)
    }
}

impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Config").finish_non_exhaustive()
    }
}
