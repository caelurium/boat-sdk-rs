//! Execute commands, follow live output, and read persisted command history.

mod api;
pub mod models;
mod stream;

pub use stream::CommandFrame;
pub use stream::CommandStream;

use serde::Serialize;

/// Command input. The client method selects foreground, detached, or streaming execution.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandRequest {
    /// Shell command to run inside the sandbox.
    pub command: String,
    /// Working directory relative to the sandbox workspace.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cwd: Option<String>,
    /// Server-side timeout in seconds (1–600, default 30 for foreground commands).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<u32>,
}

impl CommandRequest {
    /// Use the default working directory and timeout.
    pub fn new(command: impl Into<String>) -> Self {
        Self {
            command: command.into(),
            cwd: None,
            timeout_seconds: None,
        }
    }
}

/// Byte offsets for resuming each output channel independently.
#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputOffsets {
    /// Starting byte in stdout.
    pub stdout_offset: Option<u64>,
    /// Starting byte in stderr.
    pub stderr_offset: Option<u64>,
}

/// Select a tail or incremental detached command output.
#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusQuery {
    /// Bytes per channel (1–8 MiB); ignored when offsets are supplied.
    pub tail_bytes: Option<u32>,
    /// Continue from a previous status response.
    #[serde(flatten)]
    pub offsets: OutputOffsets,
}

/// Filters and pagination for persisted command history.
#[derive(Default, Serialize)]
pub struct HistoryQuery {
    /// Comma-separated modes, or `commands` for all SDK-started commands.
    pub mode: Option<String>,
    /// Case-insensitive substring of the command line.
    pub q: Option<String>,
    /// Page size (1–100, default 20).
    pub limit: Option<u32>,
    /// Cursor from the preceding page.
    pub cursor: Option<String>,
}

/// Select stdout or stderr for a stored log read.
#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum LogChannel {
    /// Standard output.
    Stdout,
    /// Standard error.
    Stderr,
}

/// Encoding of returned log data.
#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum LogEncoding {
    /// UTF-8 text.
    Utf8,
    /// Base64 for byte-exact reads.
    Base64,
}

/// Select a bounded page of historical command output.
#[derive(Default, Serialize)]
pub struct LogsQuery {
    /// Channel to read (default stdout).
    pub stream: Option<LogChannel>,
    /// Starting byte position.
    pub offset: Option<u64>,
    /// Maximum bytes to return (at most 1 MiB).
    pub limit: Option<u32>,
    /// Data encoding (default UTF-8).
    pub encoding: Option<LogEncoding>,
}
