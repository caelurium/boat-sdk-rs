use bytes::{Buf, Bytes};
use reqwest::Response;
use serde::Deserialize;

use crate::Error;

const MAX_FRAME: usize = 1024 * 1024;

/// One NDJSON record from a live command or a detached command attachment.
/// Output and provider messages may contain private sandbox data.
#[derive(Deserialize)]
#[serde(
    tag = "type",
    rename_all = "lowercase",
    rename_all_fields = "camelCase"
)]
#[non_exhaustive]
pub enum CommandFrame {
    /// The foreground command started.
    Started {
        /// History identifier, when available.
        command_id: Option<String>,
    },
    /// Attached to the requested detached process.
    Attached {
        /// Boat process identifier.
        process_id: u64,
        /// First byte requested in stdout.
        stdout_offset: u64,
        /// First byte requested in stderr.
        stderr_offset: u64,
    },
    /// Standard output chunk.
    Stdout {
        /// UTF-8 text.
        data: String,
        /// Next byte position, on attached streams.
        next_offset: Option<u64>,
    },
    /// Standard error chunk.
    Stderr {
        /// UTF-8 text.
        data: String,
        /// Next byte position, on attached streams.
        next_offset: Option<u64>,
    },
    /// Idle attachment keepalive.
    Heartbeat,
    /// The command ended.
    Exit {
        /// Command exit code, if known.
        exit_code: Option<i32>,
        /// Whether the foreground command succeeded.
        success: Option<bool>,
        /// Whether the command timed out.
        timed_out: Option<bool>,
        /// Attached process status.
        status: Option<String>,
        /// Terminating signal, if known.
        signal: Option<String>,
        /// Final stdout byte position.
        stdout_offset: Option<u64>,
        /// Final stderr byte position.
        stderr_offset: Option<u64>,
    },
    /// Attachment reached its lifetime; open a new attachment at these offsets.
    Reconnect {
        /// Resume stdout here.
        stdout_offset: u64,
        /// Resume stderr here.
        stderr_offset: u64,
    },
    /// Boat reported a terminal failure after the HTTP request was accepted.
    Error {
        /// Boat error code.
        error: String,
        /// Boat error description.
        message: String,
        /// Whether Boat considers the command safe to retry.
        retryable: Option<bool>,
        /// Last stdout position on an attached stream.
        stdout_offset: Option<u64>,
        /// Last stderr position on an attached stream.
        stderr_offset: Option<u64>,
    },
}

/// Incremental command output reader. Drop to disconnect (this does not stop the process).
pub struct CommandStream {
    response: Option<Response>,
    chunk: Bytes,
    line: Vec<u8>,
}

impl CommandStream {
    pub(super) fn new(response: Response) -> Self {
        Self {
            response: Some(response),
            chunk: Bytes::new(),
            line: Vec::new(),
        }
    }

    /// Read one frame (at most 1 MiB). A premature EOF, malformed line, oversized
    /// frame, or read failure returns an error once. Returns `None` after a
    /// terminal frame or error. Never restarts or reattaches automatically.
    pub async fn next_frame(&mut self) -> Result<Option<CommandFrame>, Error> {
        if self.response.is_none() {
            return Ok(None);
        }
        let result = self.read_frame().await;
        if !matches!(
            &result,
            Ok(CommandFrame::Started { .. }
                | CommandFrame::Attached { .. }
                | CommandFrame::Stdout { .. }
                | CommandFrame::Stderr { .. }
                | CommandFrame::Heartbeat)
        ) {
            self.response = None;
            self.chunk = Bytes::new();
        }
        result.map(Some)
    }

    async fn read_frame(&mut self) -> Result<CommandFrame, Error> {
        loop {
            if self.chunk.is_empty() {
                let response = self.response.as_mut().ok_or(Error::Transport)?;
                match response.chunk().await.map_err(|_| Error::Transport)? {
                    Some(chunk) => self.chunk = chunk,
                    None => return Err(Error::Decode),
                }
            }
            let newline = self.chunk.iter().position(|&b| b == b'\n');
            let len = newline.unwrap_or(self.chunk.len());
            if len > MAX_FRAME - self.line.len() {
                return Err(Error::ResponseTooLarge);
            }
            self.line.extend_from_slice(&self.chunk[..len]);
            self.chunk.advance(len + usize::from(newline.is_some()));
            if newline.is_some() {
                let frame = serde_json::from_slice(&self.line).map_err(|_| Error::Decode);
                self.line.clear();
                return frame;
            }
        }
    }
}
