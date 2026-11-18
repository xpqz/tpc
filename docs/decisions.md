# Design decisions

Numbered, append-only. Each entry: question, decision, reason, alternatives rejected.
A later decision may supersede an earlier one by number; the earlier entry stays.

## 1. Implementers work in worktrees

**Question.** How do parallel implementers and the lead share the repository?
**Decision.** The main checkout stays on `main` for the lead. Each implementer works in
`worktrees/{id}-{slug}` with its own Cargo target directory.
**Reason.** A merge in a checkout that an implementer had switched to its branch landed on
the wrong branch. Separate checkouts remove that failure and the cargo lock contention.
**Rejected.** One shared checkout with serial implementers: no parallelism.

## 2. Counted paths and test code

**Question.** What counts towards the LOC budget?
**Decision.** `compiler/src/` (plus `build.rs`), `runtime/`, `ide/`, `host/`. No test code
counts: Rust tests, the Node runner, test programs and fixtures are all outside the budget.
Test code never lives in a counted path.
**Reason.** User direction; tests are meant to be rich.
