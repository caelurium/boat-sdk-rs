use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use reqwest::{
    header::{HeaderValue, ETAG, IF_NONE_MATCH},
    Method, RequestBuilder, StatusCode,
};
use serde::Serialize;
use std::time::Duration;
use tokio::time::{sleep_until, Instant};

use super::models::{EventsResponse, SandboxEvent};
use crate::{Client, Error};

/// Chronological ordering of an event page.
#[derive(Clone, Copy, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum EventOrder {
    /// Oldest first, used when following live events.
    Asc,
    /// Newest first.
    Desc,
}

/// Filters and cursor for one event page or a resumable live feed.
#[derive(Clone, Default, Serialize)]
pub struct EventQuery {
    /// Page size (1–200, default 100).
    pub limit: Option<u16>,
    /// Start after this cursor in the chosen order.
    pub cursor: Option<String>,
    /// Page order. The live feed always uses ascending order.
    pub sort: Option<EventOrder>,
    /// Comma-separated event types, e.g. `prompt,response`.
    #[serde(rename = "type")]
    pub types: Option<String>,
    /// One conversation ID, or several separated by commas. Omit to watch all.
    pub conversation: Option<String>,
}

impl Client {
    fn event_request(&self, id: &str, query: &EventQuery) -> Result<RequestBuilder, Error> {
        Ok(self
            .request(Method::GET, &["sandboxes", id, "events"])?
            .query(query))
    }

    /// Fetch one bounded JSON page of extensible events.
    pub async fn events(&self, id: &str, query: &EventQuery) -> Result<EventsResponse, Error> {
        Self::send(self.event_request(id, query)?).await
    }

    /// Follow Boat's cursor-based event feed. No request is sent until the
    /// first call to [`EventStream::next_event`].
    pub fn stream_events(&self, id: &str, mut query: EventQuery) -> EventStream {
        query.sort = Some(EventOrder::Asc);
        EventStream {
            client: self.clone(),
            id: id.into(),
            query,
            pending: Vec::new().into_iter(),
            page_cursor: None,
            etag: None,
            next_poll: None,
            ended: false,
        }
    }
}

/// Demand-driven live feed. Boat exposes paginated JSON rather than SSE.
/// Holds at most one bounded response page, polls idle pages once per second,
/// and never spawns a background task. Drop it to stop watching; a finished
/// prompt does not close a sandbox-wide feed.
pub struct EventStream {
    client: Client,
    id: String,
    query: EventQuery,
    pending: std::vec::IntoIter<SandboxEvent>,
    page_cursor: Option<String>,
    etag: Option<HeaderValue>,
    next_poll: Option<Instant>,
    ended: bool,
}

impl EventStream {
    /// Resume position after the last delivered event (not the last fetched page).
    /// Pass this back in `EventQuery::cursor` with the same filters to reconnect.
    pub fn cursor(&self) -> Option<&str> {
        self.query.cursor.as_deref()
    }

    /// Wait for the next event. An HTTP, decoding, or transport error is
    /// returned once and terminates the feed; subsequent reads return `None`.
    pub async fn next_event(&mut self) -> Result<Option<SandboxEvent>, Error> {
        if self.ended {
            return Ok(None);
        }
        let result = self.next().await;
        self.ended = result.is_err();
        result.map(Some)
    }

    async fn next(&mut self) -> Result<SandboxEvent, Error> {
        loop {
            if let Some(event) = self.pending.next() {
                // Prefer Boat's opaque next-page cursor at a page boundary.
                let cursor = self
                    .page_cursor
                    .take_if(|_| self.pending.len() == 0)
                    .or_else(|| {
                        Some(URL_SAFE_NO_PAD.encode(format!(
                            "{}:{}",
                            event.timestamp?,
                            event.id.as_deref()?
                        )))
                    })
                    .ok_or(Error::Decode)?;
                if self.cursor() == Some(cursor.as_str()) {
                    return Err(Error::Decode);
                }
                self.query.cursor = Some(cursor);
                return Ok(event);
            }
            if let Some(deadline) = self.next_poll {
                sleep_until(deadline).await;
            }
            let mut request = self.client.event_request(&self.id, &self.query)?;
            if let Some(tag) = &self.etag {
                request = request.header(IF_NONE_MATCH, tag);
            }
            let response = Client::checked_response(request, self.etag.is_some()).await?;
            self.next_poll = Some(Instant::now() + Duration::from_secs(1));
            if response.status() == StatusCode::NOT_MODIFIED {
                continue;
            }
            let tag = response.headers().get(ETAG).cloned();
            let page: EventsResponse = Client::decode(response).await?;
            if page.events.is_empty() {
                // An ETag is valid only for the exact cursor that produced it.
                self.etag = tag;
                continue;
            }
            self.etag = None;
            if page.page_info.as_ref().is_some_and(|info| info.has_more) {
                self.next_poll = None;
            }
            self.page_cursor = page.page_info.and_then(|info| info.next_cursor);
            self.pending = page.events.into_iter();
        }
    }
}
