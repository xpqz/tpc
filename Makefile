CARGO_TARGET_DIR := $(shell scripts/cargo-target.sh)
export CARGO_TARGET_DIR

.PHONY: build test fmt clippy check wasm

build:
	cargo build

test:
	cargo test --no-fail-fast

fmt:
	cargo fmt

clippy:
	cargo clippy --all-targets -- -D warnings

check:
	cargo fmt --check
	cargo clippy --all-targets -- -D warnings
	cargo test --no-fail-fast

wasm:
	cargo build --release --lib --target wasm32-unknown-unknown
