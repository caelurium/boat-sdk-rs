.DEFAULT_GOAL := help

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
	python3 scripts/structure/check.py
	python3 -m unittest discover -s scripts/structure -p 'test_*.py'

format:
	cargo fmt --all -- --check

lint:
	cargo clippy --locked --workspace --all-targets -- -D warnings

test:
	cargo test --locked --workspace

docs:
	RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps

check: structure format lint test docs
