use boat_sdk::account::models::ActiveOrgUpdateRequest;
use boat_sdk::ai_credentials::models::SetCredentialKeyRequest;
use boat_sdk::api_keys::{models::CreateScopedApiKeyRequest, KeyPreset, ResourceScope};
use boat_sdk::environments::models::{
    SetEnvironmentSecretFileRequest, UpdateSandboxEnvironmentRequest,
};
use boat_sdk::{Client, Config, Error, SecretString};
use serde_json::json;
use wiremock::matchers::{body_json, header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

async fn setup() -> (MockServer, Client) {
    let server = MockServer::start().await;
    let config = Config::new("example-boat-key")
        .unwrap()
        .with_base_url(&server.uri())
        .unwrap()
        .with_organization("example-team")
        .unwrap();
    (server, Client::from_config(config).unwrap())
}

#[tokio::test]
async fn request_scope_and_active_wallet_are_separate() {
    let (server, client) = setup().await;
    Mock::given(method("GET"))
        .and(path("/environments"))
        .and(query_param("org", "example-team"))
        .and(header("x-boat-org", "example-team"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"environments":[]})))
        .mount(&server)
        .await;
    assert!(client
        .environments(Some("example-team"))
        .await
        .unwrap()
        .environments
        .is_empty());
    Mock::given(method("PATCH")).and(path("/orgs/active")).and(body_json(json!({"org":null})))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"ok":true,"type":"org.active","active":{"id":"example-user","name":"personal","type":"personal"}}))).mount(&server).await;
    assert_eq!(
        client
            .set_active_organization(&ActiveOrgUpdateRequest { org: None })
            .await
            .unwrap()
            .active
            .kind,
        "personal"
    );
}

#[tokio::test]
async fn scoped_key_creation_decodes_one_time_secret_and_resource_selectors() {
    let (server, client) = setup().await;
    Mock::given(method("POST")).and(path("/api-keys/scoped"))
        .and(body_json(json!({"preset":"read-only","environmentIds":["example-env"],"ttl":"24h"})))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({
            "ok":true,"type":"api_key.created","secret":"synthetic-issued-secret","apiKeys":[],
            "apiKey":{"id":"example-id","name":"test","credentialLane":"scoped-v1","keyPrefix":"example","keyLastFour":"test","createdAt":"2026-01-01T00:00:00Z",
                "usage":{"requests":0,"windowDays":30},"resources":{"total":0,"sandboxes":0,"agents":0},"scope":{"sandboxes":"*","environments":["example-env"]}}
        }))).mount(&server).await;
    let created = client
        .create_scoped_api_key(&CreateScopedApiKeyRequest {
            preset: Some(KeyPreset::ReadOnly),
            environment_ids: Some(vec!["example-env".into()]),
            ttl: Some("24h".into()),
            ..Default::default()
        })
        .await
        .unwrap();
    assert_eq!(created.secret.expose_secret(), "synthetic-issued-secret");
    assert_eq!(format!("{:?}", created.secret), "[REDACTED]");
    let scope = created.api_key.scope.unwrap();
    assert!(matches!(scope.sandboxes, Some(ResourceScope::Wildcard(_))));
    assert!(matches!(scope.environments,Some(ResourceScope::Ids(ids)) if ids==["example-env"]));
}

#[tokio::test]
async fn secret_file_and_profile_binding_are_independent_updates() {
    let (server, client) = setup().await;
    Mock::given(method("PUT"))
        .and(path("/environments/example/secret-files"))
        .and(body_json(
            json!({"path":"example.conf","contents":"synthetic-file-content"}),
        ))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"success":true,"versionNumber":3})),
        )
        .mount(&server)
        .await;
    let change = client
        .set_environment_secret_file(
            "example",
            &SetEnvironmentSecretFileRequest {
                path: "example.conf".into(),
                contents: SecretString::new("synthetic-file-content"),
            },
        )
        .await
        .unwrap();
    assert_eq!(change.version_number, Some(3));
    Mock::given(method("PUT"))
        .and(path("/environments/example"))
        .and(body_json(json!({"credentialProfileId":null})))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"success":true})))
        .mount(&server)
        .await;
    assert!(
        client
            .update_environment(
                "example",
                &UpdateSandboxEnvironmentRequest {
                    credential_profile_id: Some(None),
                    ..Default::default()
                }
            )
            .await
            .unwrap()
            .success
    );
}

#[tokio::test]
async fn forbidden_credential_write_returns_status_and_code_without_echoing_secret() {
    let (server, client) = setup().await;
    Mock::given(method("PUT")).and(path("/credential-profiles/example/keys/openai-api-key"))
        .and(body_json(json!({"value":"synthetic-provider-key","keepCurrent":true})))
        .respond_with(ResponseTemplate::new(403).set_body_json(json!({"code":"scope_denied","requestId":"req_example","message":"synthetic-provider-key"}))).mount(&server).await;
    let result = client
        .set_credential_profile_key(
            "example",
            "openai-api-key",
            &SetCredentialKeyRequest {
                value: SecretString::new("synthetic-provider-key"),
                keep_current: Some(true),
            },
        )
        .await;
    let err = result.err().expect("scope failure");
    assert!(!format!("{err} {err:?}").contains("synthetic-provider-key"));
    assert!(
        matches!(err,Error::Api{status,code,request_id} if status.as_u16()==403 && code.as_deref()==Some("scope_denied") && request_id.as_deref()==Some("req_example"))
    );
}
