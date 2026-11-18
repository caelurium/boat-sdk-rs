use boat_sdk::sandbox::requests::{CreateSandboxRequest, ForkRequest, UpdateSandboxRequest};
use boat_sdk::sandbox::{AutoStop, MachineSize, SandboxListQuery};
use boat_sdk::{Client, Config, Error};
use serde_json::{json, Value};
use wiremock::matchers::{body_json, header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

async fn setup() -> (MockServer, Client) {
    let server = MockServer::start().await;
    let config = Config::new("example-key")
        .unwrap()
        .with_base_url(&server.uri())
        .unwrap();
    (server, Client::from_config(config).unwrap())
}

fn sandbox() -> Value {
    json!({"id":"bx_example","name":"example","state":"provisioning","desktopAvailable":false,"snapshotAvailable":false})
}

#[tokio::test]
async fn create_and_fork_send_idempotency_keys_but_decode_different_responses() {
    let (server, client) = setup().await;
    Mock::given(method("POST")).and(path("/sandboxes"))
        .and(header("idempotency-key","create-example"))
        .and(body_json(json!({"noEnv":true,"type":"small"})))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
            "ok":true,"type":"sandbox.created","sandbox":sandbox(),"status":"provisioning","ttlSeconds":3600
        }))).expect(1).mount(&server).await;
    let created = client
        .create_sandbox(
            &CreateSandboxRequest {
                no_env: Some(true),
                machine_type: Some(MachineSize::Small),
                ..Default::default()
            },
            Some("create-example"),
        )
        .await
        .unwrap();
    assert_eq!(created.sandbox.id, "bx_example");
    assert!(created.hosted_ports.is_none());

    Mock::given(method("POST"))
        .and(path("/sandboxes/bx_example/fork"))
        .and(header("idempotency-key", "fork-example"))
        .and(body_json(json!({"ttlSeconds":300})))
        .respond_with(ResponseTemplate::new(202).set_body_json(
            json!({"ok":true,"type":"sandbox.forking","id":"bx_fork","status":"provisioning"}),
        ))
        .expect(1)
        .mount(&server)
        .await;
    let fork = client
        .fork_sandbox(
            &created.sandbox.id,
            &ForkRequest {
                ttl_seconds: Some(AutoStop::AfterSeconds(300)),
                ..Default::default()
            },
            Some("fork-example"),
        )
        .await
        .unwrap();
    assert_eq!(fork.id, "bx_fork");
}

#[tokio::test]
async fn patch_can_disable_auto_stop_without_changing_other_fields() {
    let (server, client) = setup().await;
    Mock::given(method("PATCH"))
        .and(path("/sandboxes/bx_example"))
        .and(body_json(json!({"ttlSeconds":null})))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"ok":true,"type":"sandbox.info","sandbox":sandbox()})),
        )
        .expect(1)
        .mount(&server)
        .await;
    let updated = client
        .update_sandbox(
            "bx_example",
            &UpdateSandboxRequest {
                ttl_seconds: Some(AutoStop::Disabled),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    assert!(updated.sandbox.ttl_seconds.is_none());
}

#[tokio::test]
async fn list_preserves_repeated_filters_and_pagination() {
    let (server, client) = setup().await;
    Mock::given(method("GET"))
        .and(path("/sandboxes"))
        .and(query_param("label", "env=test"))
        .and(query_param("label", "group=a b"))
        .and(query_param("namePrefix", "example"))
        .and(query_param("limit", "10"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "ok":true,"type":"sandbox.list","sandboxes":[sandbox()],
            "pageInfo":{"limit":10,"hasMore":true,"nextCursor":"next"}
        })))
        .mount(&server)
        .await;
    let page = client
        .list_sandboxes(&SandboxListQuery {
            labels: vec!["env=test".into(), "group=a b".into()],
            name_prefix: Some("example".into()),
            limit: Some(10),
            ..Default::default()
        })
        .await
        .unwrap();
    assert_eq!(page.page_info.unwrap().next_cursor.as_deref(), Some("next"));
}

#[tokio::test]
async fn delete_confirms_target_and_preserves_boat_error() {
    let (server, client) = setup().await;
    Mock::given(method("DELETE"))
        .and(path("/sandboxes/bx_example"))
        .and(header("x-ascii-confirm-delete", "bx_example"))
        .respond_with(
            ResponseTemplate::new(409)
                .set_body_json(json!({"code":"conflict","requestId":"req_example"})),
        )
        .expect(1)
        .mount(&server)
        .await;
    assert!(matches!(client.delete_sandbox("bx_example").await,
        Err(Error::Api {status,code,request_id}) if status.as_u16()==409 && code.as_deref()==Some("conflict") && request_id.as_deref()==Some("req_example")));
}
