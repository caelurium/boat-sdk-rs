use std::collections::BTreeMap;

use serde::Deserialize;

use crate::{client::transport, Client, Error};

/// Boat's live model catalogue, keyed by harness name (for example, `opencode`).
pub type ProviderCatalogue = BTreeMap<String, HarnessModels>;

/// Models available through one harness.
#[derive(Clone, Debug, Deserialize)]
pub struct HarnessModels {
    /// Models in Boat's display order.
    pub models: Vec<ProviderModel>,
    /// Default Boat model ID, when advertised.
    pub default: Option<String>,
    /// Utility Boat model ID, when advertised.
    pub utility: Option<String>,
}

/// One model entry in the Boat catalogue.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderModel {
    /// Boat identifier to send as [`super::PromptRequest::model`].
    pub id: String,
    /// Human-readable display label.
    pub label: String,
    /// Harness-specific identifier. This can differ from `id`, including for OpenCode.
    pub sdk_id: Option<String>,
    /// Credential types that can serve this model.
    pub credential_ids: Option<Vec<String>>,
    /// Reasoning effort choices, if advertised.
    pub reasoning_effort: Option<ReasoningEffort>,
    /// Whether the model advertises fast mode.
    pub fast_mode: Option<bool>,
}

/// Reasoning effort levels advertised for a model.
#[derive(Clone, Debug, Deserialize)]
pub struct ReasoningEffort {
    /// Supported levels in catalogue order.
    #[serde(default)]
    pub supported: Vec<String>,
    /// Default level, when provided.
    pub default: Option<String>,
}

/// Fetch `https://boat.dev/api/provider-models` without credentials. The same
/// 16 MiB JSON limit and typed errors apply as for authenticated API requests.
///
/// ```no_run
/// use boat_sdk::agents::{provider_models, PromptRequest};
///
/// # async fn example() -> Result<(), boat_sdk::Error> {
/// let catalogue = provider_models().await?;
/// if let Some(model) = catalogue.get("opencode").and_then(|h| h.models.first()) {
///     let mut prompt = PromptRequest::new("Summarize this project.");
///     prompt.provider = Some("opencode".into());
///     prompt.model = Some(model.id.clone());
/// }
/// # Ok(()) }
/// ```
pub async fn provider_models() -> Result<ProviderCatalogue, Error> {
    fetch_catalogue("https://boat.dev").await
}

async fn fetch_catalogue(origin: &str) -> Result<ProviderCatalogue, Error> {
    let http = transport().build().map_err(|_| Error::Configuration)?;
    Client::send(http.get(format!("{origin}/api/provider-models"))).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    async fn mock(response: ResponseTemplate) -> MockServer {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/provider-models"))
            .respond_with(response)
            .expect(1)
            .mount(&server)
            .await;
        server
    }

    #[tokio::test]
    async fn public_catalogue_distinguishes_boat_ids_and_accepts_new_harnesses() {
        let server = mock(ResponseTemplate::new(200).set_body_json(json!({
            "opencode":{"models":[{"id":"boat-example","label":"Example","sdkId":"vendor/example","credentialIds":["example-key"],"reasoningEffort":{"supported":["high"],"default":"high"},"fastMode":false,"future":42}]},
            "future-harness":{"models":[{"id":"minimal","label":"Minimal"}],"newField":true}
        }))).await;
        let catalogue = fetch_catalogue(&server.uri()).await.unwrap();
        let requests = server.received_requests().await.unwrap();
        assert_eq!(requests[0].url.query(), None);
        assert!(!requests[0].headers.contains_key("authorization"));
        assert!(!requests[0].headers.contains_key("x-boat-org"));
        let model = &catalogue["opencode"].models[0];
        assert_eq!(model.id, "boat-example");
        assert_eq!(model.label, "Example");
        assert_eq!(model.sdk_id.as_deref(), Some("vendor/example"));
        assert_eq!(model.credential_ids.as_ref().unwrap(), &["example-key"]);
        assert_eq!(model.reasoning_effort.as_ref().unwrap().supported, ["high"]);
        assert_eq!(model.fast_mode, Some(false));
        let minimal = &catalogue["future-harness"].models[0];
        assert!(
            minimal.sdk_id.is_none()
                && minimal.credential_ids.is_none()
                && minimal.reasoning_effort.is_none()
                && minimal.fast_mode.is_none()
        );
    }

    #[tokio::test]
    async fn preserves_http_errors_and_rejects_invalid_catalogues() {
        let unavailable = mock(
            ResponseTemplate::new(503)
                .insert_header("x-request-id", "req_example")
                .set_body_json(json!({"code":"unavailable"})),
        )
        .await;
        assert!(
            matches!(fetch_catalogue(&unavailable.uri()).await, Err(Error::Api {status,code,request_id}) if status.as_u16()==503 && code.as_deref()==Some("unavailable") && request_id.as_deref()==Some("req_example"))
        );
        let malformed = mock(
            ResponseTemplate::new(200)
                .set_body_json(json!({"codex":{"models":[{"id":"missing-label"}]}})),
        )
        .await;
        assert!(matches!(
            fetch_catalogue(&malformed.uri()).await,
            Err(Error::Decode)
        ));
    }
}
