use bytes::Bytes;
use reqwest::{header, Method, Response, StatusCode};

use crate::{Client, Error};

/// Incremental file or tar download from a snapshot. Drop it to disconnect.
pub struct SnapshotFile(Response);

impl SnapshotFile {
    /// HTTP status, including `206 Partial Content` for a satisfied byte range.
    pub fn status(&self) -> StatusCode {
        self.0.status()
    }

    /// Response metadata, including `Content-Range` and Boat's
    /// `X-Snapshot-Entry-Kind` (`file` or `dir`) and archive counts.
    pub fn headers(&self) -> &header::HeaderMap {
        self.0.headers()
    }

    /// Read the next binary chunk without collecting the rest of the body.
    pub async fn next_chunk(&mut self) -> Result<Option<Bytes>, Error> {
        self.0.chunk().await.map_err(|_| Error::Transport)
    }
}

impl Client {
    /// Stream file bytes or a folder as tar. Omit `path` for the whole snapshot.
    /// For a single file, supply an HTTP byte range (e.g. `bytes=1024-`) to resume.
    /// Inspect the status and content range before appending to a partial file.
    pub async fn snapshot_file(
        &self,
        id: &str,
        path: Option<&str>,
        range: Option<&str>,
    ) -> Result<SnapshotFile, Error> {
        let mut request = self
            .request(Method::GET, &["snapshots", id, "files"])?
            .query(&[("path", path)]);
        if let Some(range) = range {
            request = request.header(header::RANGE, range);
        }
        Ok(SnapshotFile(Self::send_response(request).await?))
    }
}
