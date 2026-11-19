# Size tracking

Targets (plan.md §1, provisional, revised once after Phase 4): compiler WASM 250 KB
stripped, hello-world output 20 KB, IDE WASM 120 KB, source 15,000 lines.

| Tag | Source LOC (tools/loc.sh) | Compiler WASM (make wasm) | Hello-world WASM | Notes |
|---|---|---|---|---|
| phase-0 | 271 | 281 B | n/a | emitter only; no compiler entry points exported yet |
| phase-1 | 2637 | 281 B | 389 B | compiler API exports (plan §3) not yet present, so the compiler WASM is still empty; hello-world is `WriteLn('Hello, world')` |
| post-phase-1 (#32) | 2706 | 119,546 B (unstripped) | 389 B | compiler API exported; first meaningful compiler size |
| phase-2 | 3190 | 139,213 B (unstripped) | 414 B | frames, var params, nested routines, procedural types; checker+codegen 1537 / 4700 |
| phase-3 | 4272 (Rust 4173, Pascal 99) | 168,694 B (unstripped) | 415 B | aggregates, strings, System, DCE; checkpoint Rust <= 6,500 met; checker+codegen 2176 / 4700 |
| phase-4 | 5377 (Rust 4991, Pascal 363, JS 23) | 195,830 B unstripped / 160,907 B stripped | 429 B | pointers, heap, sets, Real, intrinsics, range checks; checker 1592, checker+codegen 2791 / 4700 (Phase 4 ceiling 5,200) |

## Target revision (once, after Phase 4)

Measured stripped (custom sections removed) at phase-4: compiler 160,907 B, hello-world
429 B. Revised targets:

| Metric | Old target | New target | Reason |
|---|---|---|---|
| Compiler WASM, stripped | 250 KB | 250 KB (kept) | 161 KB with Phases 5-7 still to add; no evidence to move it |
| Hello-world output WASM | 20 KB | 2 KB | DCE keeps it at 0.4 KB; a tighter target catches regressions |
| IDE WASM | 120 KB | 120 KB (kept) | not yet measurable |
| phase-5 | 8340 | 264704 B unstripped / 210655 B stripped | 683 B | units, includes, conditionals, typed consts, text files, Dos; checker 1887 / 2200 |
| phase-6 | 9183 | 276165 B unstripped / 222182 B stripped | 875 B | Crt unit, browser terminal and workers; Chrome and Firefox browser tests pass; host 471 / 500 |
