# Boat Rust SDK

A standalone Rust SDK for the Boat VM API. This crate owns Boat-specific HTTP
transport, API models, and errors. Application-specific and provider-neutral
interfaces belong in the consuming application, not in this SDK.

## Rust modules

- Keep modules focused on API capabilities and responsibilities.
- Aim for **800 non-test lines per Rust file**; **1,500 is the hard cap**.
- Keep test files and inline test modules under **1,000 lines**.
- Split capability families into directories; a folder over 40 Rust source files
  warns and over 60 fails. See `.github/rust-structure.json` and the shared
  [checker](https://github.com/caelurium/rust-structure) for the full rules.
- Keep module declarations and re-exports sorted, one item per line.
- Keep visibility narrow and document the public API.

## API, async work, and security

- Keep network operations asynchronous and avoid blocking executor threads.
- Preserve useful HTTP status, API error codes, and request identifiers in typed
  errors without exposing credentials or sensitive response data.
- Never log bearer tokens or include real credentials, account data, or private
  sandbox content in tests, examples, or fixtures.
- Treat streaming responses as bounded, fallible input. Retry only operations
  that are safe to retry or protected by an idempotency key.
- Keep the SDK's public types focused on Boat's API; do not add Caelum-specific
  session or UI concepts here.

## Commands and verification

- `nix develop -c make check`: formatting, Clippy, tests, and documentation.
- `make structure`: shared Rust source/test size and module-folder guardrails.
- `make format`: `cargo fmt --all -- --check`.
- `make lint`: Clippy with warnings denied.
- `make test`: workspace tests.
- `make docs`: build public API documentation with warnings denied.
- Run the relevant checks while iterating and complete `make check` before
  reporting a change as finished.

## Contributions

- Keep changes focused and use Conventional Commits.
- Create a PR for completed work and follow
  `.github/PULL_REQUEST_TEMPLATE.md`.
- Use synthetic identifiers and example hosts in committed content. Keep
  credentials, private sandbox data, and machine-specific diagnostics out of
  commits.
