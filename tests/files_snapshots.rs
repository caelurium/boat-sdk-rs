use boat_sdk::access::{models::DesktopRequest, DesktopMode};
use boat_sdk::files::{
    models::{FileUrlRequest, FileWriteRequest},
    FileEncoding,
};
use boat_sdk::snapshots::models::SavedSnapshotSaveRequest;
use boat_sdk::{Client, Config, Error};
use serde_json::json;
use wiremock::matchers::{body_json, header, method, path, query_param};
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
async fn read_and_write_base64_preserve_contents_and_encode_paths() {
    let (server, client) = setup().await;
    let file = "work/a & b.bin";
    Mock::given(method("PUT")).and(path("/sandboxes/bx_example/files")).and(body_json(json!({"path":file,"encoding":"base64","content":"/wA="})))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"ok":true,"type":"file.written","success":true,"path":file,"encoding":"base64","size":2}))).expect(1).mount(&server).await;
    let written = client
        .write_file(
            "bx_example",
            &FileWriteRequest {
                path: file.into(),
                encoding: Some(FileEncoding::Base64),
                content: "/wA=".into(),
            },
        )
        .await
        .unwrap();
    assert_eq!(written.size, 2);
    Mock::given(method("GET")).and(path("/sandboxes/bx_example/files")).and(query_param("path",file)).and(query_param("encoding","base64"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"ok":true,"type":"file.read","success":true,"path":file,"encoding":"base64","size":2,"content":"/wA="}))).mount(&server).await;
    assert_eq!(
        client
            .read_file("bx_example", file, FileEncoding::Base64)
            .await
            .unwrap()
            .content,
        "/wA="
    );
}

#[tokio::test]
async fn signed_upload_returns_direct_transfer_details() {
    let (server, client) = setup().await;
    Mock::given(method("POST")).and(path("/sandboxes/bx_example/files/upload-url"))
        .and(body_json(json!({"path":"archive.tar","ttlSeconds":120})))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"ok":true,"type":"file.upload_url","url":"https://transfer.example/upload","method":"PUT","path":"/home/user/archive.tar","expiresAt":"2026-01-01T00:02:00Z"}))).mount(&server).await;
    let transfer = client
        .create_upload_url(
            "bx_example",
            &FileUrlRequest {
                path: "archive.tar".into(),
                ttl_seconds: Some(120),
            },
        )
        .await
        .unwrap();
    assert_eq!(transfer.method, "PUT");
    assert_eq!(transfer.expires_at, "2026-01-01T00:02:00Z");
    assert_eq!(transfer.url, "https://transfer.example/upload");
}

#[tokio::test]
async fn partial_snapshot_response_streams_raw_bytes_and_preserves_range() {
    let (server, client) = setup().await;
    Mock::given(method("GET"))
        .and(path("/snapshots/example/files"))
        .and(query_param("path", "folder/a b"))
        .and(header("range", "bytes=4-"))
        .respond_with(
            ResponseTemplate::new(206)
                .insert_header("content-range", "bytes 4-6/7")
                .set_body_raw(vec![0xff, 0, 0x80], "application/octet-stream"),
        )
        .mount(&server)
        .await;
    let mut download = client
        .snapshot_file("example", Some("folder/a b"), Some("bytes=4-"))
        .await
        .unwrap();
    assert_eq!(download.status().as_u16(), 206);
    assert_eq!(download.headers()["content-range"], "bytes 4-6/7");
    let mut received = Vec::new();
    while let Some(chunk) = download.next_chunk().await.unwrap() {
        received.extend_from_slice(&chunk);
    }
    assert_eq!(received, [0xff, 0, 0x80]);
}

#[tokio::test]
async fn binary_endpoint_preserves_api_error_before_exposing_a_stream() {
    let (server, client) = setup().await;
    Mock::given(method("GET"))
        .and(path("/snapshots/missing/files"))
        .respond_with(
            ResponseTemplate::new(404)
                .set_body_json(json!({"code":"not_found","requestId":"req_example"})),
        )
        .mount(&server)
        .await;
    assert!(
        matches!(client.snapshot_file("missing",None,None).await,Err(Error::Api{status,code,request_id}) if status.as_u16()==404 && code.as_deref()==Some("not_found") && request_id.as_deref()==Some("req_example"))
    );
}

#[tokio::test]
async fn saved_snapshot_can_be_pending_without_artifact_id() {
    let (server, client) = setup().await;
    Mock::given(method("POST")).and(path("/saved-snapshots")).and(body_json(json!({"sandboxId":"bx_example","name":"example"})))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({"ok":true,"type":"snapshot.saving","status":"saving","snapshot":{"name":"example","status":"saving","sourceSandboxId":"bx_example","createdAt":"2026-01-01T00:00:00Z"}}))).mount(&server).await;
    let saved = client
        .save_snapshot(&SavedSnapshotSaveRequest {
            sandbox_id: "bx_example".into(),
            name: "example".into(),
        })
        .await
        .unwrap();
    assert_eq!(saved.status, "saving");
    assert!(saved.snapshot.snapshot_id.is_none());
}

#[tokio::test]
async fn vnc_provisioning_needs_no_desktop_url_yet() {
    let (server, client) = setup().await;
    Mock::given(method("POST"))
        .and(path("/sandboxes/bx_example/desktop"))
        .and(query_param("vnc", "1"))
        .and(body_json(json!({})))
        .respond_with(ResponseTemplate::new(200).set_body_json(
            json!({"ok":true,"type":"desktop.url","provisioning":true,"desktopUrl":null}),
        ))
        .mount(&server)
        .await;
    let access = client
        .desktop("bx_example", DesktopMode::Vnc, &DesktopRequest::default())
        .await
        .unwrap();
    assert_eq!(access.provisioning, Some(true));
    assert!(access.desktop_url.is_none());
}
