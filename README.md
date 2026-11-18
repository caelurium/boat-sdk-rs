# boat-sdk-rs

An independent Rust client for the [Boat VM API](https://docs.boat.dev/api/v1),
intended to make Boat sandboxes usable by Rust applications such as
[Caelum](https://github.com/caelurium/caelum).

The async client supports authenticated JSON requests to Boat. For example,
from a Tokio runtime:

```rust
use boat_sdk::Client;

async fn account() -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let client = Client::new(&std::env::var("BOAT_API_KEY")?)?;
    Ok(client.get_json(&["me"], &()).await?)
}
```

Use `Config` and `Client::from_config` to override the API root or select an
organization. Caelum's initial integration is tracked in
[issue #214](https://github.com/caelurium/caelum/issues/214).

## Development

```sh
nix develop -c make check
```

The checks can also be run individually with `make structure`, `make format`,
`make lint`, `make test`, and `make docs`.

## Releases

To release, bump `package.version` in `Cargo.toml`, update `Cargo.lock`, and
merge the change into `main`. After all CI checks pass, GitHub Actions creates
an annotated `v<version>` tag on the tested commit and publishes a GitHub
Release with generated notes. Versions such as `0.2.0-rc.1` are marked as
prereleases. Merges without a version bump do not publish a release.
