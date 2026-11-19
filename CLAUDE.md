# CLAUDE.md

## Project

TP5-WASM: a Turbo Pascal 5.0 work-alike in Rust that compiles a defined TP5 subset to a
single WebAssembly module. The runtime (`System`, `Crt`, `Dos`) and the IDE are written in
Pascal and compiled by this compiler. `plan.md` is the specification: language subset (§2),
architecture (§3), ABI (§4), phases with exit tests (§5), test strategy (§6). Read the
relevant section before starting any issue. `docs/abi.md`, `docs/subset.md` and
`docs/deviations.md` refine it once they exist.

Layout: `compiler/` (Rust), `runtime/` (Pascal), `ide/` (Pascal), `host/` (JS), `tests/`,
`tools/`, `docs/`.

## Readability comes first

The code is meant to be studied by people learning how a compiler works. It must be simple
and obvious: plain control flow, descriptive names, one idea per function, comments that
explain the technique where it is not self-evident. No code-golf tricks to get under a
budget: no dense one-liners, clever bit tricks without explanation, cryptic abbreviations,
or merged unrelated logic. When a feature cannot be written clearly within its budget,
cut or simplify the feature (a recorded decision), never the clarity.

## The LOC budget is the hard constraint

Compiler, runtime, IDE and host glue total at most **15,000 lines**; the per-component
budgets are in `plan.md` §1. The 500-line margin is not spent before Phase 7.

- Count with `tools/loc.sh`: rustfmt at `max_width = 100`, non-blank, non-comment lines.
  Same rule for Pascal and JS. It reports per component and in total; over budget fails.
- Run `tools/loc.sh` before and after every issue and record both numbers, with the delta,
  in `docs/prs/{id}.md`.
- Counted paths: `compiler/src/` (plus `build.rs`), `runtime/`, `ide/`, `host/`. **No test
  code counts towards any budget**: not Rust tests, not the Node runner, not test programs,
  not test harness JS or Pascal. Test code never lives in a counted path: Rust tests
  (including the Phase 0 emitter test) go in `compiler/tests/`, not `#[cfg(test)]` modules;
  the Node runner lives in `tests/`. No `include!`, `#[path]` or generated source to move
  production code out of view. The one exception is decision 11: `runtime/system.pas` is
  embedded with `include_bytes!` and counted where it lives.
- Because tests are free, test richly. Thin coverage is never a way to save lines.
- Checker + codegen are budgeted at 4,700 (2,200 + 2,500); 5,200 is the Phase 4
  stop-and-cut ceiling, not an allowance. Other checkpoints: Rust total 6,500 at Phase 3,
  IDE 2,200 at feature-complete. At a checkpoint breach, stop feature work, cut scope or
  simplify (with the advisor), and record the decision in `docs/decisions.md`.
- Do not game the count: no long lines to dodge rustfmt, no stuffing logic into strings,
  macros or data tables to hide it, no moving production code into `tools/` or tests, no
  compressing code into clever forms. Write straight, readable Rust (see "Readability
  comes first").
- If a feature cannot fit, the lead decides a scope or algorithmic reduction. Cut
  features, not the cap. Do not contort code to squeeze it in.
- Before adding code, look for existing code to reuse. Every issue ends with a pass asking
  whether the change can be simpler and clearer; shorter only where that also reads better.

## What belongs in which file

One line per file of `compiler/src/`. Code goes where its line says; a function that needs
two files' worth of knowledge is split, not placed in the nearer one.

- `lex.rs`: bytes to tokens, with one token of context (`^`, the final `END .`). No other
  grammar.
- `ast.rs`: the node arena, handles and name interning. No semantics.
- `parse.rs`: tokens to nodes with TP's error positions. No name resolution or types.
- `types.rs`: the type table, compatibility, operator typing, and the plain records the
  checker produces (`Symbol`, `Routine`, `Intrinsic`, `TypedConst`). Pure data and rules; no scopes, no diagnostics
  about constructs.
- `symbols.rs`: scope stack, declaration, lookup, predefined and System names, storage choice
  and frame layout, and `Helper`, the System routines the compiler calls by name. Never imports
  `check`, never builds a `Diag`, never reads or rewrites nodes beyond interning names.
- `check.rs`: walks the tree: construct rules, typing, constant folding, desugaring, and every
  construct-specific error. Asks `symbols.rs` where things live.
- `codegen.rs`: checked tree to WASM instructions and the memory map. Never re-decides types or
  storage.
- `wasm.rs`: the binary format only: sections, LEB128, validated structured control.
- `diag.rs`: error codes, messages and rendering.
- `directives.rs`: expands compiler directives between lexing and parsing: `{$I file}`
  includes, conditional compilation, and the switch state (`{$R}`, `{$I}`) carried on each token.
- `units.rs`: finds the unit files a program uses, parses them, and orders them (interface
  order, initialisation order, cycles). No scoping and no types.
- `driver.rs`: runs the stages and the native command line. `ffi.rs`: the wasm32 exports.
  `lib.rs`: the module list and pipeline map. `main.rs`: the process entry point.

## Engineering rules (from `plan.md` §8)

- `[dependencies]` in the compiler stays empty. Dev-dependencies too, unless the lead records a decision.
- `#![forbid(unsafe_code)]`, except the two-line FFI exports.
- No `format!` or `std::fmt` in the compiler; diagnostics come from static tables plus a
  small integer-to-string routine.
- AST: `Vec<Node>` arenas with `u32` handles. No `Box`, no `Rc`.
- No `wasm-bindgen`; raw `extern "C"` exports and a manually managed buffer.
- Release profile: `panic = "abort"`, `opt-level = "z"`, `lto = true`, `codegen-units = 1`.
- Pascal runtime and IDE use only constructs covered by completed phases. Never add a
  compiler feature to make the runtime prettier.
- Omitted features (§2) produce a specific error, never silent misbehaviour.
- Every deviation from TP5 behaviour is recorded in `docs/deviations.md`.
- Provisional size targets: compiler WASM 250 KB stripped, hello-world 20 KB, IDE 120 KB.
  Tracked in `docs/size.md` with source LOC at every phase tag; revised once, explicitly.
- No backwards compatibility. When changing something, remove the old implementation.
- No workarounds; fix root causes.

## Tools

- `cargo build`, `cargo test`, `cargo fmt`, `cargo clippy --all-targets -- -D warnings`
- `make test` runs everything except browser tests (Node, no browser).
- `tools/loc.sh` for the budget. There is no remote CI: `make check` (fmt, clippy, all
  tests, loc) is the merge gate, run by the lead on the branch after merging `main` in. The
  pre-commit hook (`make hooks`) runs only fmt, clippy and loc, so RED commits with failing
  tests go through. An epic closes only when its exit test passes under `make check`.
- **The real Turbo Pascal 5.0 is the oracle**: `tools/tp5.sh prog.pas` compiles and runs
  it with Borland's TPC.EXE under DOSBox-X (about a second per program) and prints the
  output, or `P.PAS(LINE): Error N: Message.` with exit 1. Expected `.out` files for
  `tests/lang/` and `tests/diff/`, and `.err` numbers and lines for `tests/neg/`, come from
  it. Hand-written expectations are not accepted. Output differs from TP5 only where
  `docs/deviations.md` says so (e.g. `Real` formatting, D1).
- Free Pascal 3.2.2 (`fpc -Mtp`) is secondary: for natively unit-testing pure-logic Pascal
  modules, and for cross-checks. Node runs the emitted modules.
  - Pure-logic Pascal modules (such as the IDE's `Buffer`) must also compile under
    `fpc -Mtp`, so they can be checked and unit-tested natively before our compiler supports
    them. `System`, `Crt` and other host-bound units are exempt.
  - fpc evaluates integer intermediates at native width and differs from TP5 on
    intermediate overflow and on error numbers; never use it for expected output.
- Target `wasm32-unknown-unknown` is installed.
- Scratch files go in `tmp/` in the repo (git-ignored), never `/tmp`.
- Build artefacts go to `/Volumes/Space2/tpa/<checkout name>` (or `<checkout>/target-shared`
  when the SSD is absent), never `target/`. `scripts/cargo-target.sh` decides. The Makefile
  exports `CARGO_TARGET_DIR` from it, so run builds and tests through `make`. For direct
  `cargo` use, `scripts/zshenv.zsh` (to be appended to `~/.zshenv` by the user) exports it;
  if `$CARGO_TARGET_DIR` is empty, prefix with `export CARGO_TARGET_DIR=$(scripts/cargo-target.sh);`.

## Testing

Test layout follows `plan.md` §6: `tests/lang/` (`.pas` + `.out`, one feature per program),
`tests/diff/`, `tests/abi/`, `tests/neg/` (expected error number and line), `tests/rt/`
(expected runtime error code), `tests/ide/`, `tests/host/`. Name test files after the
behaviour they check, never after an issue id, phase or date.

- Expected output comes from `tools/tp5.sh`, not from our compiler or fpc. Where we deviate on purpose,
  the deviation is in `docs/deviations.md` and the test says which entry.
- Test richly: every feature gets positive, boundary and negative cases (16-bit wrap, empty
  strings, `String[n]` truncation, set bounds, deep nesting, recursion depth). Tests cost
  nothing against the budget, so there is no reason to skimp.
- Accepting an invalid program is a failure. Every new construct gets a negative test.
- Before claiming a pin is RED, confirm it fails for the stated reason, not because it
  cannot compile, does not reach the code under test, or compares empty with empty.

### Tests are the specification

Never change a test to make it pass. A test may change only if it is provably wrong
(logic error, contradicts `plan.md` or TP5 behaviour as shown by `tools/tp5.sh`, or a typo),
and then only with a written justification in `docs/prs/{id}.md` and the lead's approval
in `docs/reviews/{id}.md`.
Forbidden: adjusting expected output to match actual output, shrinking scope, removing
assertions.

## Workflow: the lead and its implementers

The main session is the **lead**. It manages a team of implementer sub-agents (Sonnet,
`.claude/agents/implementer.md`), consults the advisor, and owns: the conversation with the
user, skis, each issue's brief, the RED and GREEN verdicts, `docs/reviews/`, merges and
closing issues. The lead does not write production code, except as the fallback below.

Issues live in `skis` (`skis issue list`, `skis issue view N`, `skis issue create -T epic|task|bug`).
Use the `skis` CLI only. Never read, write, query or copy `.skis/issues.db` directly, with
sqlite3 or anything else. It is git-ignored. Epics map to the phases in `plan.md` §5; a
phase's exit test is its epic's acceptance criterion.

### Worktrees

The main checkout stays on `main` and belongs to the lead (merges, reviews, skis). Every
implementer works in its own worktree, created by the lead before the brief:
`git worktree add worktrees/{id}-{slug} -b {id}-{slug} main` (`worktrees/` is git-ignored).
Each worktree builds into its own target directory (`scripts/cargo-target.sh` keys it by
checkout name), so parallel builds never share a cargo lock. Parallel units must be
independent (no shared files beyond append-only docs). Nobody edits files on an
implementer's branch while it is working. After merge, the lead removes the worktree and
the branch. Do not start more parallel units than the lead can review promptly.

### Per-issue loop

1. **Pick.** Next open issue in the current epic (lowest-numbered open epic if none is
   active). `main` is clean and up to date.
2. **Brief.** The lead briefs an implementer: issue id and title, branch name
   `{id}-{slug-from-issue-title}`, acceptance criteria, files in scope, the `plan.md`
   sections that apply, the tests expected.
3. **RED.** The implementer branches from `main`, writes the tests, shows they fail for
   the stated reason, writes `docs/prs/{id}.md`, commits, and reports.
4. **RED verdict.** The lead checks the tests: do they cover the acceptance criteria, fail
   for the right reason (run them), come from `fpc -Mtp` where applicable, and include
   negative and boundary cases? The lead creates or appends to `docs/reviews/{id}.md`
   with the verdict (`red-approved` or `changes`) and numbered points, commits it on the
   branch, and tells the implementer (same agent, via SendMessage) to read it.
5. **GREEN.** The implementer implements until the tests pass, appends to the PR doc,
   commits, and reports.
6. **GREEN verdict.** The lead re-runs the full checks itself, reviews the diff for
   correctness, budget and conciseness, and appends and commits `approve` or `changes`.
7. **Merge.** On `approve`: merge into `main`
   (`--no-ff`), delete the local branch (and its worktree, if any), `skis issue close {id}`, proceed to the next issue.
8. **Epic end.** When an epic's issues are closed and its exit test passes: tag the commit,
   update `docs/size.md`, close the epic, summarise to the user, continue.

### No endless review loops

Each `changes` verdict counts as one review. After three reviews of an issue without
success (and no external blocker), the lead stops delegating it, consults the advisor, and
finishes the issue itself on the same branch, recording that in the review doc. Review
points are concrete and complete in one round; do not drip-feed.

### Autonomy and design decisions

The lead works autonomously. The budget is locked and the target is defined by `plan.md`;
the lead settles questions itself, consulting the advisor when a decision is non-trivial,
and documents every design decision in `docs/decisions.md` (numbered, one paragraph each:
the question, the decision, the reason, alternatives rejected). Deviations from TP5 also go
in `docs/deviations.md`. When a feature cannot fit the budget, the lead cuts scope, records
the cut as a decision, and continues.

Stop and ask the user only for actions outside the local repo (push, publishing, changes to
the user's machine configuration) or a contradiction in `plan.md` that blocks progress and
that the advisor cannot resolve.

### Review artefacts

- `docs/prs/{id}.md`: written by the implementer, append-only.
- `docs/reviews/{id}.md`: written by the lead, append-only, committed with the issue.

### Claims must match the committed tree

Before saying an issue is ready for review:

- `git status --short` is empty, and every file named in `docs/prs/{id}.md` appears in the
  committed diff.
- Every test, function and file the notes mention exists in the committed tree.
- Results in the notes come from commands actually run on that tree. Anything not done is
  marked deferred.

### Follow-ups

A finding outside the current issue becomes a new skis issue (linked to the epic), not
scope creep in the current branch.

## Definition of done

- Approved tests green; full suite green with no regressions.
- `cargo fmt --check` and clippy clean.
- `tools/loc.sh` green per component and in total, and the delta is recorded.
- Self-review done: is this the clearest version that fits? Clarity beats brevity.

## Writing style

- Comments are sparse and state the invariant or the why. No narrative, no history, no
  skis ids in code or test names.
- Commit messages, PR notes and issues: terse, factual, no emoji, no claims beyond what was
  run. Never mention Claude, Anthropic or AI assistance.
- Prose is direct. Say what you mean; no metaphor or flourish.
- Never use `--no-verify`.
