CARGO_TARGET_DIR := $(shell scripts/cargo-target.sh)
export CARGO_TARGET_DIR

.PHONY: build test fmt clippy check wasm loc hooks oracle oracle-tables

build:
	cargo build

test:
	cargo test --no-fail-fast
	sh tests/tools/loc_test.sh

fmt:
	cargo fmt

clippy:
	cargo clippy --all-targets -- -D warnings

check:
	cargo fmt --check
	cargo clippy --all-targets -- -D warnings
	$(MAKE) test
	tools/loc.sh

wasm:
	cargo build --release --lib --target wasm32-unknown-unknown

loc:
	tools/loc.sh

hooks:
	install -m 755 scripts/pre-commit "$$(git rev-parse --git-path hooks)/pre-commit"

oracle: oracle-tables
	tools/oracle.sh

oracle-tables:
	cargo test -- --ignored oracle_agrees
