use std::fmt;

/// A sensitive Boat wire value, such as a newly issued key or an environment
/// secret. Debug formatting is redacted. Serialization intentionally emits
/// the actual value for API requests; never log the serialized payload.
#[derive(Clone, serde::Deserialize, serde::Serialize)]
#[serde(transparent)]
pub struct SecretString(String);

impl SecretString {
    /// Wrap a sensitive string for an API request.
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Access the underlying secret, for example to persist a newly issued API key.
    pub fn expose_secret(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for SecretString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("[REDACTED]")
    }
}
