# Boat Rust SDK

An independent async Rust client for the [Boat VM API](https://docs.boat.dev/api/v1).
Create sandboxes, run commands, manage files and snapshots, or prompt a coding agent.

## Quickstart

Add the SDK and a Tokio runtime to `Cargo.toml` (Rust 1.95 or later):

```toml
[dependencies]
boat-sdk = { git = "https://github.com/caelurium/boat-sdk-rs", tag = "v0.5.0" }
tokio = { version = "1", features = ["macros", "rt"] }
```

Set `BOAT_API_KEY` to a [Boat API key](https://docs.boat.dev/api-keys) with
`sandbox.read` access. This program lists the first page of sandboxes:

```no_run
use boat_sdk::Client;
use boat_sdk::sandbox::SandboxListQuery;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(&std::env::var("BOAT_API_KEY")?)?;
    let page = client.list_sandboxes(&SandboxListQuery::default()).await?;
    for sandbox in page.sandboxes {
        println!("{}: {}", sandbox.id, sandbox.state);
    }
    Ok(())
}
```

Reuse the client across calls. `Client::new` sends the key as a bearer token to
the hosted API. Use `Config::with_organization` to select an organization for
this client, or `Config::with_base_url` to point it at another trusted API root.

## Runnable examples

From a checkout, with `BOAT_API_KEY` set:

| Command | What it does |
| --- | --- |
| `cargo run --example sandbox` | Creates a sandbox with a five-minute auto-stop, runs `pwd`, then requests a stop. Requires a key that can create, execute in, and stop sandboxes. |
| `cargo run --example prompt -- <sandbox-id>` | Queues a prompt in a new conversation on an existing sandbox and reports its final status. Connect an agent credential in Boat first; the example uses your configured harness and model. |

## API coverage and errors

Typed methods cover sandbox lifecycle, commands, files, snapshots, access,
integrated agents, accounts, environments, and AI credentials. For JSON GET or
POST endpoints without a typed method (such as listing webhooks), use
`Client::get_json` or `Client::post_json` with your own response type. Agent
events use cursor-based polling; use a prompt's run status to tell when it ends.

Calls return `Result<T, boat_sdk::Error>`. On `Error::Api`, inspect `status`,
`code`, and `request_id`. `Error::Transport` means the exchange failed; a
mutation may already have taken effect. For a create or fork that you intend
to retry, supply the same idempotency key and request body on each attempt.
