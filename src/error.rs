use reqwest::StatusCode;
use std::fmt;

/// Failure configuring a client, sending a request, or processing a response.
#[derive(Clone)]
#[non_exhaustive]
pub enum Error {
    /// Invalid client configuration.
    Configuration,
    /// Invalid endpoint or unserializable request.
    Request,
    /// The connection or response body could not be read.
    Transport,
    /// Boat returned a non-success HTTP status.
    Api {
        /// HTTP status from the response.
        status: StatusCode,
        /// Boat's machine-readable error code, when provided.
        code: Option<String>,
        /// Boat's request identifier, when provided.
        request_id: Option<String>,
    },
    /// The successful response did not match the requested JSON type.
    Decode,
    /// The successful JSON response exceeded the client's size limit.
    ResponseTooLarge,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Configuration => "invalid Boat client configuration",
            Self::Request => "invalid Boat API request",
            Self::Transport => "Boat API transport failed",
            Self::Api { status, .. } => return write!(f, "Boat API returned HTTP {status}"),
            Self::Decode => "invalid Boat API JSON response",
            Self::ResponseTooLarge => "Boat API JSON response exceeded size limit",
        })
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl std::error::Error for Error {}
