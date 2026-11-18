use boat_sdk::{Client, Config, Error};
use serde_json::{json, Value};
use wiremock::matchers::{body_json, header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

const KEY: &str = "synthetic-test-key";

fn client(server: &MockServer) -> Client {
    let config = Config::new(KEY)
        .unwrap()
        .with_base_url(&format!("{}/api/v1/", server.uri()))
        .unwrap()
        .with_organization("example-org")
        .unwrap();
    assert!(!format!("{config:?}").contains(KEY));
    Client::from_config(config).unwrap()
}

#[tokio::test]
async fn authenticated_requests_keep_prefix_and_encode_query_and_body() {
    let server = MockServer::start().await;
    let client = client(&server);
    assert!(!format!("{client:?}").contains(KEY));
    Mock::given(method("GET"))
        .and(path("/api/v1/sandboxes"))
        .and(query_param("cursor", "a & b"))
        .and(header("authorization", "Bearer synthetic-test-key"))
        .and(header("x-boat-org", "example-org"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"items": []})))
        .mount(&server)
        .await;
    let list: Value = client
        .get_json(&["sandboxes"], &[("cursor", "a & b")])
        .await
        .unwrap();
    assert_eq!(list["items"], json!([]));

    Mock::given(method("POST"))
        .and(path("/api/v1/sandboxes"))
        .and(header("authorization", "Bearer synthetic-test-key"))
        .and(body_json(json!({"noEnv": true})))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!("created")))
        .mount(&server)
        .await;
    let result: String = client
        .post_json(&["sandboxes"], &json!({"noEnv":true}))
        .await
        .unwrap();
    assert_eq!(result, "created");
}

#[tokio::test]
async fn preserves_api_metadata_without_formatting_response_content() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(403).set_body_json(json!({
            "error": {"code": "scope_denied", "message": KEY},
            "requestId": "req_example"
        })))
        .mount(&server)
        .await;
    let err = client(&server)
        .get_json::<Value, _>(&["me"], &())
        .await
        .unwrap_err();
    assert!(!format!("{err:?} {err}").contains(KEY));
    assert!(matches!(err, Error::Api { status, code, request_id }
        if status.as_u16() == 403 && code.as_deref() == Some("scope_denied")
        && request_id.as_deref() == Some("req_example")));
}

#[tokio::test]
async fn redirect_is_not_followed_and_non_json_error_retains_header_id() {
    let server = MockServer::start().await;
    Mock::given(path("/api/v1/me"))
        .respond_with(
            ResponseTemplate::new(302)
                .insert_header("location", "/elsewhere")
                .insert_header("x-request-id", "req_redirect")
                .set_body_string(KEY),
        )
        .mount(&server)
        .await;
    let err = client(&server)
        .get_json::<Value, _>(&["me"], &())
        .await
        .unwrap_err();
    assert!(matches!(err, Error::Api { status, code: None, request_id }
        if status.as_u16() == 302 && request_id.as_deref() == Some("req_redirect")));
}

#[tokio::test]
async fn malformed_success_is_a_redacted_decode_error() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_string(KEY))
        .mount(&server)
        .await;
    let err = client(&server)
        .get_json::<Value, _>(&["me"], &())
        .await
        .unwrap_err();
    assert!(matches!(err, Error::Decode));
    assert!(!format!("{err:?} {err}").contains(KEY));
}
