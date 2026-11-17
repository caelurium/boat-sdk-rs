.DEFAULT_GOAL := help

.PHONY: help format lint test docs check

help:
	@printf '%s\n' \
	  'Boat Rust SDK development:' \
	  '  make format   Check Rust formatting' \
	  '  make lint     Run Clippy with warnings denied' \
	  '  make test     Run workspace tests' \
	  '  make docs     Build API documentation' \
	  '  make check    Run all checks'

format:
	cargo fmt --all -- --check

lint:
	cargo clippy --locked --workspace --all-targets -- -D warnings

test:
	cargo test --locked --workspace

docs:
	RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps

check: format lint test docs
