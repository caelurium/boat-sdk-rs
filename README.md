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

Use Conventional Commits and leave the version files alone. After checks pass
on `main`, release-please opens or updates a release PR with the next version
in `Cargo.toml` and `Cargo.lock` and a changelog. A `fix:` bumps the patch;
a `feat:` bumps the minor. Breaking changes bump the minor before 1.0 and
the major thereafter.

Merge the release PR to publish. Once the merged commit passes CI, the release
workflow creates its annotated `v<version>` tag and GitHub Release.

**Maintainer setup:** Set the Actions secret `RELEASE_PLEASE_TOKEN` to a
repository-scoped bot token with contents, pull requests, and issues write
permissions. Using a separate token allows release PRs to trigger the
required CI checks on the protected `main` branch.
