# TP5-WASM

A Turbo Pascal 5.0 compatible compiler in WebAssembly. See `plan.md` for the design.

## Running in the browser

The program Worker and the page share memory (the screen and the key queue are
`SharedArrayBuffer`s), and browsers only offer `SharedArrayBuffer` to cross-origin isolated
pages. Whatever serves `host/` must send these headers on every response:

```
Cross-Origin-Opener-Policy: same-origin
Cross-Origin-Embedder-Policy: require-corp
```

`tools/serve.mjs` is a dev server that sends them:

```
node tools/serve.mjs 8080
# open http://localhost:8080/host/index.html?wasm=/path/to/program.wasm
```

Without the headers `crossOriginIsolated` is false and the page cannot run programs. There is
no fallback mode.
