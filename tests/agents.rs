use boat_sdk::agents::{EventQuery, PromptRequest};
use boat_sdk::{Client, Config, Error};
use serde_json::json;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
use wiremock::matchers::{body_json, method, path, query_param};
use wiremock::{Mock, MockServer, Request, ResponseTemplate};

async fn setup() -> (MockServer, Client) {
    let server = MockServer::start().await;
    let client = Client::from_config(
        Config::new("example-key")
            .unwrap()
            .with_base_url(&server.uri())
            .unwrap(),
    )
    .unwrap();
    (server, client)
}

#[tokio::test]
async fn queues_a_conversation_turn_and_reads_its_completion() {
    let (server, client) = setup().await;
    let mut run =
        json!({"id":"run","promptId":"p","sandboxId":"bx_example","status":"queued","done":false});
    Mock::given(method("POST")).and(path("/sandboxes/bx_example/prompt"))
        .and(body_json(json!({"provider":"opencode","model":"example-model","prompt":"Summarize","conversationId":"c"})))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({"ok":true,"type":"prompt.queued","id":"bx_example","promptId":"p","conversationId":"c","promptRun":run,"status":"queued","provider":"opencode"}))).expect(1).mount(&server).await;
    let input = PromptRequest {
        provider: Some("opencode".into()),
        model: Some("example-model".into()),
        conversation_id: Some("c".into()),
        ..PromptRequest::new("Summarize")
    };
    let queued = client.prompt("bx_example", &input).await.unwrap();
    run["done"] = true.into();
    run["status"] = "finished".into();
    Mock::given(method("GET"))
        .and(path("/sandboxes/bx_example/prompts/p"))
        .respond_with(ResponseTemplate::new(200).set_body_json(
            json!({"ok":true,"type":"prompt.run","id":"bx_example","promptRun":run}),
        ))
        .mount(&server)
        .await;
    assert!(
        client
            .prompt_status("bx_example", &queued.prompt_id)
            .await
            .unwrap()
            .prompt_run
            .done
    );
    assert!(matches!(
        client
            .prompt(
                "bx_example",
                &PromptRequest {
                    new_conversation: Some(true),
                    ..input
                }
            )
            .await,
        Err(Error::Request)
    ));
}

#[tokio::test]
async fn feed_checkpoints_consumed_events_and_fuses_on_api_failure() {
    let (server, client) = setup().await;
    Mock::given(path("/sandboxes/bx_example/events")).and(query_param("cursor","initial")).and(query_param("sort","asc")).and(query_param("conversation","c"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"ok":true,"type":"events.list","id":"bx_example","events":[{"type":"response","id":"a","timestamp":1,"conversationId":"c","data":{"is_streaming":true},"future":7},{"type":"example.new","id":"b","timestamp":2}],"pageInfo":{"limit":2,"hasMore":true,"nextCursor":"opaque"}}))).expect(1).mount(&server).await;
    Mock::given(path("/sandboxes/bx_example/events"))
        .and(query_param("cursor", "opaque"))
        .respond_with(ResponseTemplate::new(403).set_body_json(json!({"code":"scope_denied"})))
        .expect(1)
        .mount(&server)
        .await;
    let mut feed = client.stream_events(
        "bx_example",
        EventQuery {
            cursor: Some("initial".into()),
            conversation: Some("c".into()),
            ..Default::default()
        },
    );
    let event = feed.next_event().await.unwrap().unwrap();
    assert_eq!(event.extra["future"], 7);
    assert_eq!(event.data.unwrap()["is_streaming"], true);
    assert_eq!(feed.cursor(), Some("MTph"));
    assert_eq!(
        feed.next_event().await.unwrap().unwrap().kind,
        "example.new"
    );
    assert_eq!(feed.cursor(), Some("opaque"));
    assert!(
        matches!(feed.next_event().await,Err(Error::Api{code,..}) if code.as_deref()==Some("scope_denied"))
    );
    assert!(feed.next_event().await.unwrap().is_none());
}

#[tokio::test]
async fn idle_feed_reuses_etag_across_not_modified_then_delivers_a_new_event() {
    let (server, client) = setup().await;
    let calls = AtomicUsize::new(0);
    Mock::given(path("/sandboxes/bx_example/events"))
        .respond_with(move |req: &Request| {
            let index = calls.fetch_add(1, Ordering::SeqCst);
            if index != 0
                && req
                    .headers
                    .get("if-none-match")
                    .and_then(|v| v.to_str().ok())
                    != Some("\"page\"")
            {
                return ResponseTemplate::new(412);
            }
            if index == 1 {
                return ResponseTemplate::new(304);
            }
            let events = if index == 0 {
                json!([])
            } else {
                json!([{"type":"response","id":"a","timestamp":1}])
            };
            ResponseTemplate::new(200)
                .insert_header("etag", "\"page\"")
                .set_body_json(
                    json!({"ok":true,"type":"events.list","id":"bx_example","events":events}),
                )
        })
        .expect(3)
        .mount(&server)
        .await;
    let mut feed = client.stream_events("bx_example", EventQuery::default());
    let event = tokio::time::timeout(Duration::from_secs(8), feed.next_event())
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    assert_eq!(event.id.as_deref(), Some("a"));
    assert_eq!(feed.cursor(), Some("MTph"));
}
