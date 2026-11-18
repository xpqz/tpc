# Size tracking

Targets (plan.md §1, provisional, revised once after Phase 4): compiler WASM 250 KB
stripped, hello-world output 20 KB, IDE WASM 120 KB, source 15,000 lines.

| Tag | Source LOC (tools/loc.sh) | Compiler WASM (make wasm) | Hello-world WASM | Notes |
|---|---|---|---|---|
| phase-0 | 271 | 281 B | n/a | emitter only; no compiler entry points exported yet |
| phase-1 | 2637 | 281 B | 389 B | compiler API exports (plan §3) not yet present, so the compiler WASM is still empty; hello-world is `WriteLn('Hello, world')` |
| post-phase-1 (#32) | 2706 | 119,546 B (unstripped) | 389 B | compiler API exported; first meaningful compiler size |
| phase-2 | 3190 | 139,213 B (unstripped) | 414 B | frames, var params, nested routines, procedural types; checker+codegen 1537 / 4700 |
