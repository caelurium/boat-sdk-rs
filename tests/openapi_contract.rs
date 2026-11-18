//! Exercise the public client against examples extracted from Boat's published
//! OpenAPI document. The fixture records the source URL and content digest so
//! the tests run offline and contract changes can be reviewed explicitly.

use boat_sdk::agents::PromptRequest;
use boat_sdk::sandbox::requests::CreateSandboxRequest;
use boat_sdk::sandbox::AutoStop;
use boat_sdk::{Client, Config};
use serde::Deserialize;
use serde_json::Value;
use std::collections::BTreeMap;
use wiremock::matchers::{body_json, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[derive(Deserialize)]
struct Fixture {
    operations: BTreeMap<String, Operation>,
}

#[derive(Deserialize)]
struct Operation {
    method: String,
    path: String,
    status: u16,
    request: Option<Value>,
    response: Value,
}

impl Operation {
    async fn mount(&self, server: &MockServer, params: &[(&str, &str)]) {
        let mut route = self.path.clone();
        for (name, value) in params {
            route = route.replace(name, value);
        }
        let mut mock = Mock::given(method(self.method.as_str())).and(path(route));
        if let Some(body) = &self.request {
            mock = mock.and(body_json(body));
        }
        mock.respond_with(ResponseTemplate::new(self.status).set_body_json(&self.response))
            .expect(1)
            .mount(server)
            .await;
    }
}

async fn setup() -> (MockServer, Client, BTreeMap<String, Operation>) {
    let fixture: Fixture =
        serde_json::from_str(include_str!("fixtures/openapi-lifecycle.json")).unwrap();
    let server = MockServer::start().await;
    let config = Config::new("example-test-key")
        .unwrap()
        .with_base_url(&server.uri())
        .unwrap();
    (
        server,
        Client::from_config(config).unwrap(),
        fixture.operations,
    )
}

#[tokio::test]
async fn lifecycle_and_prompt_match_published_wire_examples() {
    let (server, client, ops) = setup().await;
    ops["create"].mount(&server, &[]).await;
    let sandbox = client
        .create_sandbox(
            &CreateSandboxRequest {
                ttl_seconds: Some(AutoStop::AfterSeconds(3600)),
                ..Default::default()
            },
            None,
        )
        .await
        .unwrap()
        .sandbox;
    assert_eq!(sandbox.state, "provisioning");

    let params = [("{sandboxId}", sandbox.id.as_str())];
    let prompt = &ops["prompt"];
    prompt.mount(&server, &params).await;
    let text = prompt.request.as_ref().unwrap()["prompt"].as_str().unwrap();
    let queued = client
        .prompt(
            &sandbox.id,
            &PromptRequest {
                provider: Some("codex".into()),
                model: Some("gpt-5.4".into()),
                reasoning_effort: Some("medium".into()),
                ..PromptRequest::new(text)
            },
        )
        .await
        .unwrap();
    assert!(!queued.prompt_run.done);
    assert_eq!(queued.prompt_run.sandbox_id, sandbox.id);
    assert_eq!(queued.prompt_run.conversation_id, queued.conversation_id);

    ops["stop"].mount(&server, &params).await;
    let stop = client
        .stop_sandbox(&sandbox.id, &Default::default())
        .await
        .unwrap();
    assert_eq!(stop.status, "archiving");
    assert!(stop.operation.is_none());
}

#[tokio::test]
async fn decodes_legacy_snapshot_and_org_specific_optional_fields() {
    let (server, client, ops) = setup().await;
    let legacy = &ops["getSnapshotTree"];
    let id = legacy.response["snapshotId"].as_str().unwrap();
    legacy.mount(&server, &[("{snapshotId}", id)]).await;
    let tree = client.snapshot_tree(id).await.unwrap();
    assert!(!tree.tree_available);
    assert_eq!(tree.reason.as_deref(), Some("legacy_snapshot"));

    ops["listOrganizations"].mount(&server, &[]).await;
    let wallets = client.list_organizations().await.unwrap().orgs;
    let personal = wallets.iter().find(|org| org.kind == "personal").unwrap();
    assert!(personal.subscription_status.is_none());
    assert!(wallets
        .iter()
        .any(|org| org.kind == "org" && org.subscription_status.as_deref() == Some("active")));
}
