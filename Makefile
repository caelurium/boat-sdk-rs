.DEFAULT_GOAL := help

STRUCTURE_ARGS ?=

.PHONY: help structure format lint test docs check

help:
	@printf '%s\n' \
	  'Boat Rust SDK development:' \
	  '  make structure Check Rust source and module-folder guardrails' \
	  '  make format   Check Rust formatting' \
	  '  make lint     Run Clippy with warnings denied' \
	  '  make test     Run workspace tests' \
	  '  make docs     Build API documentation' \
	  '  make check    Run all checks'

structure:
	nix run github:caelurium/rust-structure/v1.0.0 -- --root . --base HEAD --config .github/rust-structure.json $(STRUCTURE_ARGS)

format:
	cargo fmt --all -- --check

lint:
	cargo clippy --locked --workspace --all-targets -- -D warnings

test:
	cargo test --locked --workspace

docs:
	RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps

check: structure format lint test docs
