//! Boat command results, history records, and process samples.

/// Command Response returned by Boat.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandResponse {
    /// Boat `ok` field.
    pub ok: bool,
    /// Boat `type` field.
    #[serde(rename = "type")]
    pub kind: String,
    /// Boat `success` field.
    pub success: bool,
    /// The command's record in the command history.
    pub command_id: Option<String>,
    /// Boat `exitCode` field.
    pub exit_code: Option<i64>,
    /// Boat `signal` field.
    pub signal: Option<String>,
    /// True when the memory ceiling killed the command or one of its processes and the command failed.
    pub oom_killed: Option<bool>,
    /// Boat `stdout` field.
    pub stdout: String,
    /// Boat `stderr` field.
    pub stderr: String,
    /// Boat `stdoutTruncated` field.
    pub stdout_truncated: Option<bool>,
    /// Boat `stderrTruncated` field.
    pub stderr_truncated: Option<bool>,
    /// Boat `timedOut` field.
    pub timed_out: bool,
    /// Boat `cwd` field.
    pub cwd: Option<String>,
    /// Boat `startedAt` field.
    pub started_at: Option<String>,
    /// Boat `finishedAt` field.
    pub finished_at: Option<String>,
}

/// Command Started Response returned by Boat.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandStartedResponse {
    /// Boat `ok` field.
    pub ok: bool,
    /// Boat `type` field.
    #[serde(rename = "type")]
    pub kind: String,
    /// Boat `success` field.
    pub success: bool,
    /// The command's record in the command history.
    pub command_id: Option<String>,
    /// Process id to poll with the command status endpoint.
    pub process_id: u64,
    /// Boat `pid` field.
    pub pid: i64,
    /// Boat `command` field.
    pub command: String,
    /// Boat `cwd` field.
    pub cwd: Option<String>,
    /// Boat `startedAt` field.
    pub started_at: String,
    /// Path to the stdout log file inside the sandbox.
    pub log_path: Option<String>,
    /// Stderr log file on the sandbox.
    pub err_log_path: Option<String>,
}

/// Command Status Response returned by Boat.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandStatusResponse {
    /// Boat `ok` field.
    pub ok: bool,
    /// Boat `type` field.
    #[serde(rename = "type")]
    pub kind: String,
    /// Boat `success` field.
    pub success: bool,
    /// Boat `processId` field.
    pub process_id: u64,
    /// Boat `pid` field.
    pub pid: Option<i64>,
    /// lost: the sandbox agent restarted and forgot the process; running/exitCode are then a best-effort probe and the logs come from the on-disk files.
    pub status: String,
    /// Whether the process is still tracked by the sandbox agent.
    pub known: Option<bool>,
    /// Boat `running` field.
    pub running: bool,
    /// Boat `exitCode` field.
    pub exit_code: Option<i64>,
    /// Boat `signal` field.
    pub signal: Option<String>,
    /// True when the memory ceiling killed the command or one of its processes and the command failed.
    pub oom_killed: Option<bool>,
    /// Boat `command` field.
    pub command: Option<String>,
    /// Boat `cwd` field.
    pub cwd: Option<String>,
    /// Boat `startedAt` field.
    pub started_at: Option<String>,
    /// Boat `finishedAt` field.
    pub finished_at: Option<String>,
    /// Tail of the stdout log file.
    pub stdout: String,
    /// Tail of the stderr log file.
    pub stderr: String,
    /// Boat `stdoutTruncated` field.
    pub stdout_truncated: Option<bool>,
    /// Boat `stderrTruncated` field.
    pub stderr_truncated: Option<bool>,
    /// Boat `logPath` field.
    pub log_path: Option<String>,
    /// Boat `errLogPath` field.
    pub err_log_path: Option<String>,
    /// Byte offset where `stdout` starts: the offset you asked for, or 0 when the log was replaced.
    pub stdout_offset: Option<u64>,
    /// Byte offset where `stderr` starts: the offset you asked for, or 0 when the log was replaced.
    pub stderr_offset: Option<u64>,
    /// The `stdoutOffset` for the next read.
    pub stdout_next_offset: Option<u64>,
    /// The `stderrOffset` for the next read.
    pub stderr_next_offset: Option<u64>,
    /// True when more output is already written.
    pub more: Option<bool>,
}

/// Command Record returned by Boat.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandRecord {
    /// Id of the record in the command history.
    pub command_id: String,
    /// Boat `sandboxId` field.
    pub sandbox_id: String,
    /// The command text as sent (first 64 KiB).
    pub command: String,
    /// Boat `cwd` field.
    pub cwd: Option<String>,
    /// sync: the request waited for the result.
    pub mode: String,
    /// detached: the process id on the sandbox.
    pub process_id: Option<u64>,
    /// failed: the command could not start or its result never arrived.
    pub status: String,
    /// Boat `exitCode` field.
    pub exit_code: Option<i64>,
    /// Boat `signal` field.
    pub signal: Option<String>,
    /// Bytes of stdout written so far.
    pub stdout_bytes: u64,
    /// Bytes of stderr written so far.
    pub stderr_bytes: u64,
    /// True when part of the output is not kept: the sandbox cut the output of a waiting command at 8 MiB, or the account wrote more than 10 MiB (compressed) of command output, or of process output, in one hour.
    pub output_truncated: bool,
    /// Boat `startedAt` field.
    pub started_at: String,
    /// Boat `finishedAt` field.
    pub finished_at: Option<String>,
}

/// Command List Response returned by Boat.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandListResponse {
    /// Boat `ok` field.
    pub ok: bool,
    /// Boat `type` field.
    #[serde(rename = "type")]
    pub kind: String,
    /// Newest first.
    pub commands: Vec<CommandRecord>,
    /// Pass as `cursor` to get the next page.
    pub next_cursor: Option<String>,
}

/// Command Info Response returned by Boat.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandInfoResponse {
    /// Boat `ok` field.
    pub ok: bool,
    /// Boat `type` field.
    #[serde(rename = "type")]
    pub kind: String,
    /// Boat `command` field.
    pub command: CommandRecord,
}

/// Command Logs Response returned by Boat.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandLogsResponse {
    /// Boat `ok` field.
    pub ok: bool,
    /// Boat `type` field.
    #[serde(rename = "type")]
    pub kind: String,
    /// Boat `commandId` field.
    pub command_id: String,
    /// Boat `stream` field.
    pub stream: String,
    /// Byte position of the first byte in `data`.
    pub offset: u64,
    /// Pass as `offset` to read on.
    pub next_offset: u64,
    /// Bytes the stream has so far.
    pub size: u64,
    /// Boat `encoding` field.
    pub encoding: String,
    /// Boat `data` field.
    pub data: String,
    /// True when the command ended and this read reached the end of the stream.
    pub done: bool,
}

/// Command Stop Response returned by Boat.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandStopResponse {
    /// Boat `ok` field.
    pub ok: bool,
    /// Boat `type` field.
    #[serde(rename = "type")]
    pub kind: String,
    /// Boat `processId` field.
    pub process_id: u64,
    /// Always false after a stop.
    pub running: bool,
    /// False when the process had already ended.
    pub signaled: bool,
    /// The history record of the command, with its final state.
    pub command: Option<CommandRecord>,
}

/// Sandbox Process returned by Boat.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SandboxProcess {
    /// Boat `pid` field.
    pub pid: i64,
    /// Live list only.
    pub parent_pid: Option<i64>,
    /// Live list only.
    pub user: Option<String>,
    /// Boat `cpuPercent` field.
    pub cpu_percent: Option<f64>,
    /// Resident memory.
    #[serde(rename = "memoryMB")]
    pub memory_mb: Option<f64>,
    /// Live list only.
    pub started_at: Option<String>,
    /// The full command line (first 1000 characters; 120 in a stored sample).
    pub command: String,
    /// Live list: the history record holding this process's output, when it has one.
    pub command_id: Option<String>,
}

/// Process List Response returned by Boat.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessListResponse {
    /// Boat `ok` field.
    pub ok: bool,
    /// Boat `type` field.
    #[serde(rename = "type")]
    pub kind: String,
    /// True: read from the running sandbox now.
    pub live: bool,
    /// Boat `sampledAt` field.
    pub sampled_at: Option<String>,
    /// Processes on the sandbox.
    pub total: i64,
    /// Boat `processes` field.
    pub processes: Vec<SandboxProcess>,
}
