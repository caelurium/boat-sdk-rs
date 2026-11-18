//! Sandbox filesystem operations. Use signed transfer URLs for files over 5 MiB.

pub mod models;

use models::*;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{Client, Error};

/// Encoding of the `content` field in a JSON file read or write.
#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FileEncoding {
    /// UTF-8 text.
    Utf8,
    /// Base64 for byte-exact file contents.
    Base64,
}

impl Client {
    /// Read a text or base64 file (up to 5 MiB before encoding).
    pub async fn read_file(
        &self,
        id: &str,
        path: &str,
        encoding: FileEncoding,
    ) -> Result<FileReadResponse, Error> {
        #[derive(Serialize)]
        struct Query<'a> {
            path: &'a str,
            encoding: FileEncoding,
        }
        self.get_json(&["sandboxes", id, "files"], &Query { path, encoding })
            .await
    }

    /// Write a file (up to 5 MiB decoded) into `/home/user` or `/tmp`.
    pub async fn write_file(
        &self,
        id: &str,
        body: &FileWriteRequest,
    ) -> Result<FileWriteResponse, Error> {
        Self::send(
            self.request(Method::PUT, &["sandboxes", id, "files"])?
                .json(body),
        )
        .await
    }

    /// List a directory, using the sandbox workspace when `path` is omitted.
    pub async fn list_files(
        &self,
        id: &str,
        path: Option<&str>,
    ) -> Result<FileListResponse, Error> {
        self.get_json(&["sandboxes", id, "files", "list"], &[("path", path)])
            .await
    }

    /// Read a file or directory's metadata.
    pub async fn file_info(&self, id: &str, path: &str) -> Result<FileInfoResponse, Error> {
        self.get_json(&["sandboxes", id, "files", "info"], &[("path", path)])
            .await
    }

    /// Search for filenames matching a pattern.
    pub async fn search_files(
        &self,
        id: &str,
        path: Option<&str>,
        pattern: &str,
    ) -> Result<FileSearchResponse, Error> {
        self.get_json(
            &["sandboxes", id, "files", "search"],
            &[("path", path), ("pattern", Some(pattern))],
        )
        .await
    }

    /// Search file contents and return matching lines.
    pub async fn find_in_files(
        &self,
        id: &str,
        path: Option<&str>,
        pattern: &str,
        regex: bool,
    ) -> Result<FileFindResponse, Error> {
        self.get_json(
            &["sandboxes", id, "files", "find"],
            &[
                ("path", path),
                ("pattern", Some(pattern)),
                ("regex", Some(if regex { "true" } else { "false" })),
            ],
        )
        .await
    }

    /// Create a directory.
    pub async fn create_folder(
        &self,
        id: &str,
        body: &FileFolderRequest,
    ) -> Result<FileInfoResponse, Error> {
        self.post_json(&["sandboxes", id, "files", "folder"], body)
            .await
    }

    /// Move a file or directory.
    pub async fn move_file(
        &self,
        id: &str,
        body: &FileMoveRequest,
    ) -> Result<FileMoveResponse, Error> {
        self.post_json(&["sandboxes", id, "files", "move"], body)
            .await
    }

    /// Change a file's permission bits.
    pub async fn set_file_permissions(
        &self,
        id: &str,
        body: &FilePermissionsRequest,
    ) -> Result<FileInfoResponse, Error> {
        self.post_json(&["sandboxes", id, "files", "permissions"], body)
            .await
    }

    /// Replace matching text in the specified files.
    pub async fn replace_in_files(
        &self,
        id: &str,
        body: &FileReplaceRequest,
    ) -> Result<FileReplaceResponse, Error> {
        self.post_json(&["sandboxes", id, "files", "replace"], body)
            .await
    }

    /// Delete a file, or a directory when `recursive` is true.
    pub async fn delete_file(
        &self,
        id: &str,
        path: &str,
        recursive: bool,
    ) -> Result<FileDeleteResponse, Error> {
        Self::send(
            self.request(Method::DELETE, &["sandboxes", id, "files"])?
                .query(&[
                    ("path", path),
                    ("recursive", if recursive { "true" } else { "false" }),
                ]),
        )
        .await
    }

    /// Mint a temporary GET URL. Treat it as a secret until `expires_at`;
    /// stream the download using a separate client without Boat authorization.
    pub async fn create_download_url(
        &self,
        id: &str,
        body: &FileUrlRequest,
    ) -> Result<FileUrlResponse, Error> {
        self.post_json(&["sandboxes", id, "files", "download-url"], body)
            .await
    }

    /// Mint a temporary PUT URL. Upload directly without Boat authorization.
    /// The URL is secret and expires after the requested TTL (default 900 s).
    pub async fn create_upload_url(
        &self,
        id: &str,
        body: &FileUrlRequest,
    ) -> Result<FileUrlResponse, Error> {
        self.post_json(&["sandboxes", id, "files", "upload-url"], body)
            .await
    }
}
