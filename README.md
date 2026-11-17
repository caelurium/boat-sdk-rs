# boat-sdk-rs

An independent Rust client for the [Boat VM API](https://docs.boat.dev/api/v1),
intended to make Boat sandboxes usable by Rust applications such as
[Caelum](https://github.com/caelurium/caelum).

This repository is bootstrapped, but the SDK API is not implemented yet. Caelum's
initial integration is tracked in
[issue #214](https://github.com/caelurium/caelum/issues/214).

## Development

```sh
nix develop -c make check
```

The checks can also be run individually with `make format`, `make lint`,
`make test`, and `make docs`.
