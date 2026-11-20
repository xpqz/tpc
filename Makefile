CARGO_TARGET_DIR := $(shell scripts/cargo-target.sh)
export CARGO_TARGET_DIR

.PHONY: no-conflict-markers build test test-browser fmt clippy check wasm loc hooks oracle oracle-tables ide

build:
	cargo build

test:
	cargo test --no-fail-fast
	sh tests/tools/loc_test.sh
	sh tests/tools/tp5_test.sh
	sh tests/tools/oracle_test.sh
	node tests/tools/abi_imports_test.mjs
	node --test "tests/vscode/*.test.mjs"

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
	node tools/wasm_sections.mjs --no-custom $(CARGO_TARGET_DIR)/wasm32-unknown-unknown/release/tp5c.wasm

loc:
	tools/loc.sh

# Builds the IDE and the in-browser compiler into tmp/, serves the repository with the
# COOP/COEP headers the page needs, and opens the IDE in the default browser (macOS `open`).
# Ctrl-C stops the server. PORT=n picks another port.
PORT ?= 8080
IDE_URL = http://localhost:$(PORT)/host/index.html?wasm=/tmp/ide.wasm&compiler=/tmp/tp5c.wasm
ide: build wasm
	mkdir -p tmp
	$(CARGO_TARGET_DIR)/debug/tp5c ide/ide.pas -o tmp/ide.wasm
	cp $(CARGO_TARGET_DIR)/wasm32-unknown-unknown/release/tp5c.wasm tmp/tp5c.wasm
	@echo "IDE: $(IDE_URL)"
	@(sleep 1; command -v open >/dev/null && open "$(IDE_URL)") &
	node tools/serve.mjs $(PORT)

hooks:
	install -m 755 scripts/pre-commit "$$(git rev-parse --git-path hooks)/pre-commit"

oracle: oracle-tables
	tools/oracle.sh

oracle-tables:
	cargo test -- --ignored oracle_agrees

# A merge conflict left in any tracked file fails the gate.
no-conflict-markers:
	@! git grep -nE '^(<<<<<<<|>>>>>>>) |^=======$$' -- . ':!*.out'
