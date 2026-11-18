# Size tracking

Targets (plan.md §1, provisional, revised once after Phase 4): compiler WASM 250 KB
stripped, hello-world output 20 KB, IDE WASM 120 KB, source 15,000 lines.

| Tag | Source LOC (tools/loc.sh) | Compiler WASM (make wasm) | Hello-world WASM | Notes |
|---|---|---|---|---|
| phase-0 | 271 | 281 B | n/a | emitter only; no compiler entry points exported yet |
