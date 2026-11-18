use boat_sdk::commands::{CommandFrame, CommandRequest, OutputOffsets, StatusQuery};
use boat_sdk::{Client, Config, Error};
use serde_json::json;
use wiremock::matchers::{body_json, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

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
async fn detached_lifecycle_keeps_process_and_history_ids_distinct() {
    let (server, client) = setup().await;
    let endpoint = "/sandboxes/bx_example/commands";
    Mock::given(method("POST")).and(path(endpoint))
        .and(body_json(json!({"command":"pwd","stream":false,"detached":true})))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"ok":true,"type":"command.started","success":true,"commandId":"cmd_example","processId":7,"pid":42,"command":"pwd","startedAt":"2026-01-01T00:00:00Z"})))
        .expect(1).mount(&server).await;
    let started = client
        .start_command("bx_example", &CommandRequest::new("pwd"))
        .await
        .unwrap();
    assert_eq!(started.command_id.as_deref(), Some("cmd_example"));
    Mock::given(method("GET")).and(path(format!("{endpoint}/7"))).and(query_param("stdoutOffset","8"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"ok":true,"type":"command.status","success":true,"processId":7,"running":true,"status":"running","exitCode":null,"stdout":"x","stderr":"","stdoutNextOffset":9})))
        .mount(&server).await;
    let status = client
        .command_status(
            "bx_example",
            started.process_id,
            &StatusQuery {
                offsets: OutputOffsets {
                    stdout_offset: Some(8),
                    stderr_offset: None,
                },
                ..Default::default()
            },
        )
        .await
        .unwrap();
    assert_eq!(status.stdout_next_offset, Some(9));
    Mock::given(method("DELETE")).and(path(format!("{endpoint}/7")))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"ok":true,"type":"command.stopped","processId":7,"running":false,"signaled":true,"command":null})))
        .mount(&server).await;
    assert!(
        client
            .stop_command("bx_example", started.process_id)
            .await
            .unwrap()
            .signaled
    );
}

#[tokio::test]
async fn attachment_delivers_output_and_reconnect_offsets() {
    let (server, client) = setup().await;
    Mock::given(method("GET")).and(path("/sandboxes/bx_example/commands/7/stream"))
        .and(query_param("stdoutOffset","3")).respond_with(ResponseTemplate::new(200).set_body_raw(
        "{\"type\":\"stdout\",\"data\":\"ok\",\"nextOffset\":5}\r\n{\"type\":\"reconnect\",\"stdoutOffset\":5,\"stderrOffset\":0}\n","application/x-ndjson")).expect(1).mount(&server).await;
    let mut stream = client
        .attach_command(
            "bx_example",
            7,
            &OutputOffsets {
                stdout_offset: Some(3),
                stderr_offset: None,
            },
        )
        .await
        .unwrap();
    assert!(
        matches!(stream.next_frame().await,Ok(Some(CommandFrame::Stdout{data,next_offset:Some(5)})) if data=="ok")
    );
    assert!(matches!(
        stream.next_frame().await,
        Ok(Some(CommandFrame::Reconnect {
            stdout_offset: 5,
            stderr_offset: 0
        }))
    ));
    assert!(stream.next_frame().await.unwrap().is_none());
}

#[tokio::test]
async fn foreground_stream_reports_invalid_lines_size_and_premature_eof() {
    for (body, oversized) in [
        ("not json\n".to_owned(), false),
        ("x".repeat(1024 * 1024 + 1), true),
        (String::new(), false),
    ] {
        let (server, client) = setup().await;
        Mock::given(method("POST"))
            .and(body_json(
                json!({"command":"pwd","stream":true,"detached":false}),
            ))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                format!("{{\"type\":\"started\"}}\n{body}"),
                "application/x-ndjson",
            ))
            .expect(1)
            .mount(&server)
            .await;
        let mut stream = client
            .stream_command("bx_example", &CommandRequest::new("pwd"))
            .await
            .unwrap();
        assert!(matches!(
            stream.next_frame().await,
            Ok(Some(CommandFrame::Started { .. }))
        ));
        match stream.next_frame().await {
            Err(Error::ResponseTooLarge) if oversized => {}
            Err(Error::Decode) if !oversized => {}
            _ => panic!("incorrect stream result"),
        }
        assert!(stream.next_frame().await.unwrap().is_none());
    }
}

#[tokio::test]
async fn stream_open_preserves_http_failure_without_rerunning_command() {
    let (server, client) = setup().await;
    Mock::given(method("POST"))
        .respond_with(
            ResponseTemplate::new(502)
                .set_body_json(json!({"code":"boat_direct_failed","requestId":"req_example"})),
        )
        .expect(1)
        .mount(&server)
        .await;
    assert!(
        matches!(client.stream_command("bx_example",&CommandRequest::new("pwd")).await,
        Err(Error::Api{status,code,request_id}) if status.as_u16()==502 && code.as_deref()==Some("boat_direct_failed") && request_id.as_deref()==Some("req_example"))
    );
}
