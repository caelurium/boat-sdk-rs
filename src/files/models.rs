//! Boat files request and response models.

/// File Read Response.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileReadResponse {
    /// Boat `ok` field.
    pub ok: bool,
    /// Boat `type` field.
    #[serde(rename = "type")]
    pub response_type: String,
    /// Boat `success` field.
    pub success: bool,
    /// Boat `path` field.
    pub path: String,
    /// Boat `encoding` field.
    pub encoding: super::FileEncoding,
    /// Boat `size` field.
    pub size: u64,
    /// Boat `content` field.
    pub content: String,
}

/// File Write Request.
#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FileWriteRequest {
    /// Absolute path, or path relative to the sandbox work directory (/home/user).
    pub path: String,
    /// Boat `content` field.
    pub content: String,
    /// Boat `encoding` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<super::FileEncoding>,
}

/// File Write Response.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileWriteResponse {
    /// Boat `ok` field.
    pub ok: bool,
    /// Boat `type` field.
    #[serde(rename = "type")]
    pub response_type: String,
    /// Boat `success` field.
    pub success: bool,
    /// Boat `path` field.
    pub path: String,
    /// Boat `encoding` field.
    pub encoding: super::FileEncoding,
    /// Boat `size` field.
    pub size: u64,
}

/// File Entry.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileEntry {
    /// Boat `name` field.
    pub name: String,
    /// Absolute path on the sandbox.
    pub path: String,
    /// Boat `isDir` field.
    pub is_dir: bool,
    /// Boat `isSymlink` field.
    pub is_symlink: bool,
    /// Boat `size` field.
    pub size: u64,
    /// Octal permission bits, for example "0644".
    pub mode: String,
    /// Boat `modifiedAt` field.
    pub modified_at: String,
}

/// File List Response.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileListResponse {
    /// Boat `ok` field.
    pub ok: bool,
    /// Boat `type` field.
    #[serde(rename = "type")]
    pub response_type: String,
    /// Boat `success` field.
    pub success: Option<bool>,
    /// Boat `path` field.
    pub path: String,
    /// Boat `files` field.
    pub files: Vec<FileEntry>,
    /// True when the directory holds more than 10,000 entries.
    pub truncated: bool,
}

/// File Info Response.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileInfoResponse {
    /// Boat `ok` field.
    pub ok: bool,
    /// Boat `type` field.
    #[serde(rename = "type")]
    pub response_type: String,
    /// Boat `name` field.
    pub name: String,
    /// Absolute path on the sandbox.
    pub path: String,
    /// Boat `isDir` field.
    pub is_dir: bool,
    /// Boat `isSymlink` field.
    pub is_symlink: bool,
    /// Boat `size` field.
    pub size: u64,
    /// Octal permission bits, for example "0644".
    pub mode: String,
    /// Boat `modifiedAt` field.
    pub modified_at: String,
    /// Boat `success` field.
    pub success: Option<bool>,
}

/// File Search Response.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileSearchResponse {
    /// Boat `ok` field.
    pub ok: bool,
    /// Boat `type` field.
    #[serde(rename = "type")]
    pub response_type: String,
    /// Boat `success` field.
    pub success: Option<bool>,
    /// Absolute paths, at most 10,000.
    pub files: Vec<String>,
    /// Boat `truncated` field.
    pub truncated: bool,
    /// Present only when part of the tree could not be searched (for example a folder you cannot read).
    pub errors: Option<Vec<String>>,
}

/// File Match.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileMatch {
    /// Boat `file` field.
    pub file: String,
    /// Boat `line` field.
    pub line: u64,
    /// Boat `content` field.
    pub content: String,
}

/// File Find Response.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileFindResponse {
    /// Boat `ok` field.
    pub ok: bool,
    /// Boat `type` field.
    #[serde(rename = "type")]
    pub response_type: String,
    /// Boat `success` field.
    pub success: Option<bool>,
    /// At most 1,000 matches.
    pub matches: Vec<FileMatch>,
    /// Boat `truncated` field.
    pub truncated: bool,
    /// Present only when part of the tree could not be searched.
    pub errors: Option<Vec<String>>,
}

/// File Folder Request.
#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FileFolderRequest {
    /// Missing parent directories are created.
    pub path: String,
    /// Octal permission bits, for example "755".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
}

/// File Move Request.
#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FileMoveRequest {
    /// Boat `source` field.
    pub source: String,
    /// The new full path.
    pub destination: String,
    /// Replace an existing destination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overwrite: Option<bool>,
}

/// File Move Response.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileMoveResponse {
    /// Boat `ok` field.
    pub ok: bool,
    /// Boat `type` field.
    #[serde(rename = "type")]
    pub response_type: String,
    /// Boat `success` field.
    pub success: Option<bool>,
    /// Boat `path` field.
    pub path: String,
    /// Boat `destination` field.
    pub destination: String,
}

/// File Permissions Request.
#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FilePermissionsRequest {
    /// Boat `path` field.
    pub path: String,
    /// Octal permission bits, for example "755".
    pub mode: String,
}

/// File Replace Request.
#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FileReplaceRequest {
    /// Files up to 5 MiB each.
    pub paths: Vec<String>,
    /// Plain text.
    pub pattern: String,
    /// Required.
    pub new_value: String,
}

/// Replacement Result.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplacementResult {
    /// Boat `file` field.
    pub file: String,
    /// Boat `success` field.
    pub success: bool,
    /// Boat `replacements` field.
    pub replacements: Option<u64>,
    /// Boat `error` field.
    pub error: Option<String>,
}

/// File Replace Response.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileReplaceResponse {
    /// Boat `ok` field.
    pub ok: bool,
    /// Boat `type` field.
    #[serde(rename = "type")]
    pub response_type: String,
    /// Boat `success` field.
    pub success: Option<bool>,
    /// One result per file.
    pub results: Vec<ReplacementResult>,
}

/// File Delete Response.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileDeleteResponse {
    /// Boat `ok` field.
    pub ok: bool,
    /// Boat `type` field.
    #[serde(rename = "type")]
    pub response_type: String,
    /// Boat `success` field.
    pub success: Option<bool>,
    /// Boat `path` field.
    pub path: String,
}

/// File Url Request.
#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FileUrlRequest {
    /// Absolute path, or path relative to the sandbox work directory (/home/user).
    pub path: String,
    /// How long the URL stays valid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl_seconds: Option<u64>,
}

/// File Url Response.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileUrlResponse {
    /// Boat `ok` field.
    pub ok: bool,
    /// Boat `type` field.
    #[serde(rename = "type")]
    pub response_type: String,
    /// The signed URL.
    pub url: String,
    /// GET downloads the file.
    pub method: String,
    /// The canonical path the URL is bound to.
    pub path: String,
    /// Boat `expiresAt` field.
    pub expires_at: String,
}
