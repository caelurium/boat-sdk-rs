use reqwest::Method;
use serde::Serialize;

use super::models::{
    CommandInfoResponse, CommandListResponse, CommandLogsResponse, CommandResponse,
    CommandStartedResponse, CommandStatusResponse, CommandStopResponse, ProcessListResponse,
};
use super::{CommandRequest, CommandStream, HistoryQuery, LogsQuery, OutputOffsets, StatusQuery};
use crate::{Client, Error};

#[derive(Serialize)]
struct Execution<'a> {
    #[serde(flatten)]
    input: &'a CommandRequest,
    detached: bool,
    stream: bool,
}

impl Client {
    /// Execute once and wait for the result. A transport failure does not imply the command never ran.
    pub async fn run_command(
        &self,
        id: &str,
        input: &CommandRequest,
    ) -> Result<CommandResponse, Error> {
        self.post_json(&["sandboxes", id, "commands"], input).await
    }

    /// Start a background command and return its process ID.
    pub async fn start_command(
        &self,
        id: &str,
        input: &CommandRequest,
    ) -> Result<CommandStartedResponse, Error> {
        self.post_json(
            &["sandboxes", id, "commands"],
            &Execution {
                input,
                detached: true,
                stream: false,
            },
        )
        .await
    }

    /// Start a foreground command and consume its NDJSON output incrementally.
    pub async fn stream_command(
        &self,
        id: &str,
        input: &CommandRequest,
    ) -> Result<CommandStream, Error> {
        let request = self
            .request(Method::POST, &["sandboxes", id, "commands"])?
            .json(&Execution {
                input,
                detached: false,
                stream: true,
            });
        Ok(CommandStream::new(Self::send_response(request).await?))
    }

    /// Read the status and output of a detached command.
    pub async fn command_status(
        &self,
        id: &str,
        process_id: u64,
        query: &StatusQuery,
    ) -> Result<CommandStatusResponse, Error> {
        self.get_json(
            &["sandboxes", id, "commands", &process_id.to_string()],
            query,
        )
        .await
    }

    /// Stop a detached command and its descendants.
    pub async fn stop_command(
        &self,
        id: &str,
        process_id: u64,
    ) -> Result<CommandStopResponse, Error> {
        Self::send(self.request(
            Method::DELETE,
            &["sandboxes", id, "commands", &process_id.to_string()],
        )?)
        .await
    }

    /// Follow an existing command. After ten minutes Boat can return a reconnect
    /// frame; attach again with the provided offsets to continue.
    pub async fn attach_command(
        &self,
        id: &str,
        process_id: u64,
        offsets: &OutputOffsets,
    ) -> Result<CommandStream, Error> {
        let req = self
            .request(
                Method::GET,
                &[
                    "sandboxes",
                    id,
                    "commands",
                    &process_id.to_string(),
                    "stream",
                ],
            )?
            .query(offsets);
        Ok(CommandStream::new(Self::send_response(req).await?))
    }

    /// List one page of recorded commands and processes.
    pub async fn command_history(
        &self,
        id: &str,
        query: &HistoryQuery,
    ) -> Result<CommandListResponse, Error> {
        self.get_json(&["sandboxes", id, "command-history"], query)
            .await
    }

    /// Read a recorded command by its history ID (not its process ID).
    pub async fn get_command(
        &self,
        id: &str,
        command_id: &str,
    ) -> Result<CommandInfoResponse, Error> {
        self.get_json(&["sandboxes", id, "command-history", command_id], &())
            .await
    }

    /// Read one page of persisted output, including while a sandbox is stopped.
    pub async fn command_logs(
        &self,
        id: &str,
        command_id: &str,
        query: &LogsQuery,
    ) -> Result<CommandLogsResponse, Error> {
        self.get_json(
            &["sandboxes", id, "command-history", command_id, "logs"],
            query,
        )
        .await
    }

    /// List live processes, or the last sample if the sandbox is stopped.
    pub async fn processes(&self, id: &str) -> Result<ProcessListResponse, Error> {
        self.get_json(&["sandboxes", id, "processes"], &()).await
    }
}
