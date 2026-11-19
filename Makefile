CARGO_TARGET_DIR := $(shell scripts/cargo-target.sh)
export CARGO_TARGET_DIR

.PHONY: no-conflict-markers build test test-browser fmt clippy check wasm loc hooks oracle oracle-tables

build:
	cargo build

test:
	cargo test --no-fail-fast
	sh tests/tools/loc_test.sh
	sh tests/tools/tp5_test.sh
	sh tests/tools/oracle_test.sh
	node tests/tools/abi_imports_test.mjs

# Drives the installed Chrome headless; needs a machine that can run it, so not part of `check`.
test-browser:
	node tests/host/browser/run.mjs

fmt:
	cargo fmt

clippy:
	cargo clippy --all-targets -- -D warnings

check: no-conflict-markers
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

# A merge conflict left in any tracked file fails the gate.
no-conflict-markers:
	@! git grep -nE '^(<<<<<<<|>>>>>>>) |^=======$$' -- . ':!*.out'
