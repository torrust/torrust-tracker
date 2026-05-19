---
doc-type: issue
issue-type: task
status: draft
priority: p2
github-issue: null
spec-path: docs/issues/drafts/cli-output-contract-migration.md
branch: null
related-pr: null
last-updated-utc: 2026-05-19 20:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/adrs/20260519000000_define_global_cli_output_contract.md
    - src/bin/http_health_check.rs
    - console/tracker-client/src/bin/tracker_client.rs
    - packages/configuration/src/lib.rs
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Migrate Existing Binaries to the Global CLI Output Contract

## Goal

Bring the codebase into compliance with the global CLI output contract defined in
[ADR 20260519000000](../../adrs/20260519000000_define_global_cli_output_contract.md).
Once all non-compliant uses of `print!`, `println!`, `eprint!`, and `eprintln!` are
resolved, enable `clippy::print_stdout` and `clippy::print_stderr` as workspace-level
`deny` lints to make the contract a compile-time guarantee.

## Background

ADR 20260519000000 is prescriptive: it defines what every first-party binary must do but
explicitly defers migration of existing code to this follow-up issue. New commands and
features must already comply; only pre-existing usages are migrated here.

A workspace-wide grep found **46 occurrences** of direct print macros across the codebase
(as of 2026-05-19). The breakdown by area is:

| Area                                                                 | Files | Occurrences | Action                                             |
| -------------------------------------------------------------------- | ----- | ----------- | -------------------------------------------------- |
| `src/bin/http_health_check.rs`                                       | 1     | 5           | Migrate to JSON stdout/stderr                      |
| `src/console/profiling.rs`                                           | 1     | 3           | Out of scope (developer harness; excluded by ADR)  |
| `console/tracker-client/src/bin/tracker_client.rs`                   | 1     | 2           | Wire TTY refusal; already nearly compliant         |
| `console/tracker-client/src/bin/udp_tracker_client.rs`               | 1     | 1           | Remove (deprecated binary)                         |
| `console/tracker-client/src/bin/http_tracker_client.rs`              | 1     | 1           | Remove (deprecated binary)                         |
| `console/tracker-client/src/bin/tracker_checker.rs`                  | 1     | 2           | Remove (deprecated binary)                         |
| `console/tracker-client/src/console/clients/`                        | ~6    | ~16         | Rewrite console abstraction layer to emit JSON     |
| `packages/configuration/src/lib.rs`                                  | 1     | 3           | Replace with `tracing::info!`                      |
| `packages/udp-tracker-core/src/services/banning.rs`                  | 1     | 1           | Replace or remove debug print                      |
| `packages/tracker-core/src/databases/driver/{mysql,postgres}/mod.rs` | 2     | 2           | Replace with `tracing::info!` (test-skip messages) |
| `packages/tracker-core/src/bin/persistence_benchmark/runner.rs`      | 1     | 1           | Assess: JSON output or out of scope                |
| `packages/test-helpers/src/logging.rs`                               | 1     | 1           | Assess: test-only; may warrant `#[allow]`          |
| `contrib/dev-tools/analysis/workspace-coupling/src/main.rs`          | 1     | 6           | Assess: dev tool; may be out of scope              |

## Out of Scope

- `src/console/profiling.rs` — explicitly excluded from the contract by ADR section 3.
- `contrib/dev-tools/` — developer tooling; not operator-facing binaries. Excluded unless
  the team decides otherwise.

## Acceptance Criteria

| ID  | Criterion                                                                                                                                                                                                          |
| --- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| AC1 | `src/bin/http_health_check.rs` emits a single JSON object on stdout on success and a JSON record on stderr on failure; no `println!` or `eprintln!` remain.                                                        |
| AC2 | `console/tracker-client/src/bin/tracker_client.rs` refuses to run when stdout is a TTY (exit 2, JSON stderr diagnostic).                                                                                           |
| AC3 | Deprecated binaries `udp_tracker_client`, `http_tracker_client`, and `tracker_checker` are removed from the repository.                                                                                            |
| AC4 | `packages/configuration/src/lib.rs` uses `tracing` for configuration loading notifications; no `println!` remain.                                                                                                  |
| AC5 | All remaining in-scope `print!`/`println!`/`eprint!`/`eprintln!` usages are either migrated or carry an explicit `#[allow(clippy::print_stdout)]` / `#[allow(clippy::print_stderr)]` with a justification comment. |
| AC6 | `clippy::print_stdout = "deny"` and `clippy::print_stderr = "deny"` are added to `[workspace.lints.clippy]` in the root `Cargo.toml`.                                                                              |
| AC7 | `cargo clippy --workspace --all-targets --all-features` passes with no new warnings or errors.                                                                                                                     |
| AC8 | All existing tests pass.                                                                                                                                                                                           |

## Implementation Plan

| ID  | Status | Task                                                                       | Notes                                                                                                                                    |
| --- | ------ | -------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | TODO   | Remove deprecated binaries                                                 | Delete `udp_tracker_client.rs`, `http_tracker_client.rs`, `tracker_checker.rs` and their `Cargo.toml` entries                            |
| T2  | TODO   | Migrate `http_health_check` to JSON output                                 | Rewrite to emit `{"status":"ok"}` / `{"status":"error","message":"..."}` on stdout; usage errors as JSON on stderr                       |
| T3  | TODO   | Wire TTY refusal into `tracker_client`                                     | Check `stdout.is_terminal()` at entry; exit 2 with JSON stderr diagnostic if true                                                        |
| T4  | TODO   | Rewrite tracker-client console abstraction layer                           | Replace `console.rs` and related print calls in `clients/` with JSON emitters                                                            |
| T5  | TODO   | Replace `println!` in `packages/configuration` with `tracing`              | Three config-loading notification messages                                                                                               |
| T6  | TODO   | Replace debug print in `packages/udp-tracker-core/src/services/banning.rs` | Remove or replace with `tracing::debug!`                                                                                                 |
| T7  | TODO   | Replace test-skip `println!` in database drivers                           | Replace with `tracing::info!` or `eprintln!` under `#[allow]` with justification                                                         |
| T8  | TODO   | Assess `persistence_benchmark` and `test-helpers` usages                   | Decide: JSON output, `tracing`, or `#[allow]` with justification                                                                         |
| T9  | TODO   | Enable workspace-level lint denials                                        | Add `print_stdout = "deny"` and `print_stderr = "deny"` to `[workspace.lints.clippy]` in root `Cargo.toml`; ensure `cargo clippy` passes |

## Progress Tracking

### Workflow Checkpoints

- [ ] Spec reviewed and approved by user/maintainer
- [ ] GitHub issue created and issue number added to this spec
- [ ] Deprecated binaries removed (T1)
- [ ] `http_health_check` migrated to JSON output (T2)
- [ ] TTY refusal wired into `tracker_client` (T3)
- [ ] Tracker-client console abstraction layer rewritten (T4)
- [ ] Library `println!` usages replaced (T5–T8)
- [ ] Workspace lint denials enabled and `cargo clippy` passes (T9)
- [ ] All tests pass
- [ ] Issue closed and spec moved to `docs/issues/closed/`

## References

- Global CLI output contract ADR: `docs/adrs/20260519000000_define_global_cli_output_contract.md`
- Parent issue: [#1798](https://github.com/torrust/torrust-tracker/issues/1798)
- Workspace lints migration: [#1786](https://github.com/torrust/torrust-tracker/issues/1786)
  (coordinate on `print_stdout`/`print_stderr` deny timing)
