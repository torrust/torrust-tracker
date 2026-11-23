---
doc-type: test-refactor-plan-guidance
issue: 2902
package: torrust-tracker-core
status: in-progress
---

# Tracker Core File Test Plan Guidance

This folder holds one **file test plan** per Rust source file in `packages/tracker-core/src/` that
needs one (`<module>-tests.md`, with `/` replaced by `-`, for example
`statistics-repository-tests.md`). The workflow, the two maintainer gates, the commit points, the
slices and the processing order are defined once in
[ISSUE.md: Per-File Workflow](../ISSUE.md#per-file-workflow), the
[Pull-Request Slices](../ISSUE.md#pull-request-slices) and the
[Source File Ledger](../ISSUE.md#source-file-ledger); do not repeat them here. The
[coverage evidence](../coverage-evidence.md) holds the numbers, the uncovered line groups and the
T1 hypothesis of each file.

Create a plan right before starting its file, never in advance, so that each plan learns from the
previous ones. A plan authorizes neither test changes nor work on another file until the maintainer
approves it. A group of declaration-only files may share one plan.

## Plans

| Plan sequence | Plan | State | Result |
| ---: | --- | --- | --- |
| 1 | [statistics-repository-tests.md](statistics-repository-tests.md) | DONE | Approved 2026-11-17 (D1: test the unused gauge methods); completed file approved 2026-11-17 |
| 2 | [test-support-ephemeral-database-tests.md](test-support-ephemeral-database-tests.md) | DONE | F5 fix and the SQLite half of checkpoint T2b; shape, scope and guard owner decided and the completed change approved 2026-11-17; merged in #2923 |
| 3 | [statistics-metrics-tests.md](statistics-metrics-tests.md) | DONE | No change: covered by the repository tests, all six mutants killed; approved 2026-11-18 |
| 4 | [statistics-persisted-mod-tests.md](statistics-persisted-mod-tests.md) | DONE | Plan approved 2026-11-18 (pins the non-atomic load); completed file approved 2026-11-18 |
| 5 | [statistics-event-listener-tests.md](statistics-event-listener-tests.md) | DONE | Plan approved 2026-11-18; completed file and the T2b fixture shape approved 2026-11-18 |
| 6 | [statistics-event-handler-tests.md](statistics-event-handler-tests.md) | DONE | Plan approved 2026-11-18; completed file approved 2026-11-18 |
| 7 | [statistics-persisted-downloads-tests.md](statistics-persisted-downloads-tests.md) | DONE | Plan approved 2026-11-18 (a surviving mutant despite full coverage); completed file approved 2026-11-18 |
| 8 | [statistics-mod-tests.md](statistics-mod-tests.md) | DONE | Plan approved 2026-11-18; completed file approved 2026-11-18 |
| 9 | [sqlite-auth-key-store-tests.md](sqlite-auth-key-store-tests.md) | DONE | Plan approved 2026-11-22; extracts the shared `parse_valid_until` and `peer_key_from_row` (Q4); completed file approved 2026-11-22 |
| 10 | [sqlite-torrent-metrics-store-tests.md](sqlite-torrent-metrics-store-tests.md) | DONE | Plan approved 2026-11-23; moves the SQLite store test setup into a shared fixture; completed file approved 2026-11-23 |
| 11 | [sqlite-whitelist-store-tests.md](sqlite-whitelist-store-tests.md) | IN_PROGRESS | Plan approved 2026-11-23 |

Add a row when a plan is created; the remaining files and their order are in the ledger.

## The Two-Phase Sequence

Each file goes through the same two phases, with a maintainer gate after each:

1. **Plan:** write the plan (current state, review of the existing tests, uncovered lines grouped by
   behavior and classified, property-test decision, named mutations, steps). Stop for approval.
2. **Tests:** apply the approved steps (R1 refactor existing tests, R2 add unit tests, R3 the
   integration increment, R4 measure), fill the prose-first AAA review and the results. Stop for
   the completed-file review, then commit the file alone.

## Shared Guardrails

- Unit-only coverage (`--all-features --lib`) is the primary metric. A package integration test
  needs a recorded reason that the public-API boundary is clearer or necessary.
- Do not add tests only for coverage, and do not test what another package owns: the metric
  collection rules (`torrust-metrics`), the swarm registry and its events
  (`swarm-coordination-registry`), the drain of queued events (`torrust-tracker-events`), the
  clock (`torrust-clock`), or the configuration parsing (`torrust-tracker-configuration`).
- Judge existing tests against the `write-unit-test` skill (names `it_should_*`, visible AAA,
  state-centred Arrange, one reason to fail, no production-derived expectations, no hidden Act or
  fixture coupling, no logic or loops in the test). List the concrete smells in the plan; "clean" is
  also a recorded conclusion.
- Never write a test that pins behavior a finding reports as wrong without a recorded maintainer
  decision. Follow the [findings procedure](../ISSUE.md#findings-during-implementation); the open
  findings F1 to F5 are in the [register](../ISSUE.md#findings-register).
- Asynchronous work and cleanup follow
  [Asynchronous Work, Cleanup and Test Fixtures](../ISSUE.md#asynchronous-work-cleanup-and-test-fixtures):
  a test that spawns a listener cancels it and awaits its `JoinHandle` under a 5 second
  `tokio::time::timeout` that names the operation; a test that opens a SQLite file owns it in a
  `tempfile::TempDir`; no test sleeps a fixed time to wait for a result. The first listener test
  and the first `TempDir`-owned SQLite test stop at the design-review checkpoint T2b.
- Property-based tests are tried only where the plan names a natural property and kept only when
  they protect state that examples miss; otherwise the plan records why examples suffice (Q5).
- Every added or materially changed test gets a **Prose-First AAA Review** row in its plan before
  the completed-file review: temporary Arrange, Act and Assert prose, compared with the code, and
  the outcome.
- Named mutations are temporary hand-made changes, each applied alone and reverted exactly. A
  mutant that leaves an import or a variable unused does not compile under `unused = "deny"`; redo
  it so that it compiles.
- New dev-dependencies follow the `add-rust-dependency` skill (latest stable version, minimal
  features, rationale in the plan that needs them) and pass `cargo machete --with-metadata`.

## Commands

Use stable Rust for tests and coverage and the nightly toolchain for formatting only; record the
toolchain with every result. Every figure of this issue uses stable Rust 1.100.0 and
`cargo-llvm-cov 0.9.1` (re-measured on 2026-11-18; the first figures used Rust 1.99.1 and
`cargo-llvm-cov 0.6.16`, which cannot read the test-binary layout of Cargo 1.100.0). Measure from a
clean coverage target directory: a stale binary there is merged into the report.

```bash
# Focused tests
cargo test -p torrust-tracker-core --all-features --lib

# Unit-only coverage, in a separate target directory, after a clean
export CARGO_LLVM_COV_TARGET_DIR="$PWD/.tmp/llvm-cov-target"
cargo llvm-cov clean --workspace
cargo llvm-cov --package torrust-tracker-core --all-features --lib --json --output-path .tmp/cov/unit.json

# Formatting, lints and whitespace
cargo +nightly fmt --all -- --check
cargo clippy -p torrust-tracker-core --all-targets --all-features
git diff --check

# After changing these records
linter markdown
linter cspell
```

The other scopes (integration-only, aggregate, `--lib` without the feature and the container-gated
scope) are measured at the start and the end of each slice with the commands in
[coverage-evidence.md](../coverage-evidence.md).

## Testing Lessons Carried From #2637, #1348 And #1349

- Re-check an "untestable" path against the exported test doubles (the `testing` feature's mocks)
  and the derives before recording it as untestable.
- Assert whole values against literals written in the test, never against values computed by the
  production code under test.
- Re-measure the package at the start of every slice and compare it with the previous baseline,
  because `develop` moves under the work.

## Reconciliation

Before final verification, reconcile every plan state with the ledger, `coverage-evidence.md` and
EPIC #1347. Do not cite branch SHAs as evidence until the implementation is merged; cite unique
Conventional Commit subjects instead.
