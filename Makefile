CARGO_TARGET_DIR := $(shell scripts/cargo-target.sh)
export CARGO_TARGET_DIR

.PHONY: no-conflict-markers build test test-browser test-vscode vscode fmt clippy check wasm loc hooks oracle oracle-tables ide site deploy-site test-site demo-gif

build:
	cargo build

test:
	cargo test --no-fail-fast
	sh tests/tools/loc_test.sh
	sh tests/tools/tp5_test.sh
	sh tests/tools/oracle_test.sh
	node tests/tools/abi_imports_test.mjs
	node --test --test-concurrency=1 "tests/vscode/*.test.mjs"

# Drives the installed Chrome headless; needs a machine that can run it, so not part of `check`.
test-browser:
	node tests/host/browser/run.mjs

# Runs the extension in the installed VS Code (VSCODE_BIN overrides where it is) and checks it
# through the real editor API. Opens a window, so not part of `check`.
test-vscode: wasm
	node tests/vscode/editor/run.mjs

# The extension as a self-contained directory, tmp/vscode-ext/, and as tmp/tp5-pascal-VERSION.vsix.
# Install it with `code --install-extension tmp/tp5-pascal-VERSION.vsix`.
vscode: wasm
	node tools/package_vscode.mjs $(CARGO_TARGET_DIR)/wasm32-unknown-unknown/release/tp5c.wasm

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

# The static site for GitHub Pages in tmp/site/: the host page, both WASM modules, a sample
# program and the service worker that adds COOP/COEP (site/). The site is the `ide` build
# with different files around it.
site: build wasm
	rm -rf tmp/site
	mkdir -p tmp/site/host
	cp host/*.js host/index.html tmp/site/host/
	cp site/index.html site/coi-sw.js site/demo.pas tmp/site/
	$(CARGO_TARGET_DIR)/debug/tp5c ide/ide.pas -o tmp/site/ide.wasm
	cp $(CARGO_TARGET_DIR)/wasm32-unknown-unknown/release/tp5c.wasm tmp/site/tp5c.wasm

# Pushes tmp/site/ to the gh-pages branch of origin (see site/deploy.sh).
deploy-site: site
	site/deploy.sh

# Opens Chrome headless against a server without COOP/COEP headers, at / and at /tpc/.
test-site: site
	node tests/host/browser/site.mjs

# Records docs/images/ide-demo.gif from the IDE in headless Chrome; needs ffmpeg.
demo-gif: site
	node tools/demo_gif.mjs

hooks:
	install -m 755 scripts/pre-commit "$$(git rev-parse --git-path hooks)/pre-commit"

oracle: oracle-tables
	tools/oracle.sh

oracle-tables:
	cargo test -- --ignored oracle_agrees

# A merge conflict left in any tracked file fails the gate.
no-conflict-markers:
	@! git grep -nE '^(<<<<<<<|>>>>>>>) |^=======$$' -- . ':!*.out'
