---
doc-type: issue
issue-type: task
status: in-progress
priority: p1
epic: null
github-issue: 2134
spec-path: docs/issues/open/2134-fix-cognitive-complexity-lint-enforcement.md
branch: "2134-fix-cognitive-complexity-lint-enforcement"
related-pr: null
last-updated-utc: 2026-09-05 00:00
semantic-links:
  skill-links:
    - create-issue
    - write-markdown-docs
  related-artifacts:
    - Cargo.toml
    - .github/workflows/testing.yaml
    - packages/swarm-coordination-registry/src/statistics/event/handler.rs
    - packages/swarm-coordination-registry/src/statistics/event/listener.rs
    - src/console/profiling.rs
    - packages/e2e-tools/src/bin/profiling.rs
    - .github/skills/dev/planning/create-issue/SKILL.md
---

# Issue #2134 - Fix cognitive-complexity violations and enforce the Clippy lint

## Goal

Refactor the existing cognitive-complexity violations and make `clippy::cognitive_complexity` a tracked Cargo lint policy that the existing `linter all` Clippy run enforces in CI for all workspace code.

## Background

The explicit command below currently fails because two functions exceed Clippy's default cognitive-complexity threshold of 25:

```text
cargo clippy --workspace --tests -- -W clippy::cognitive_complexity -D warnings
```

The failing command was run with the following toolchain:

```text
rustc 1.98.0 (88d9e12ae 2026-08-18)
clippy 0.1.98 (88d9e12ae1 2026-08-18)
```

Current output:

```text
Blocking waiting for file lock on build directory
Checking torrust-tracker-swarm-coordination-registry v0.1.0
Checking torrust-tracker-test-helpers v3.0.0
Checking torrust-tracker-client v0.1.0
Checking torrust-tracker-axum-server v0.1.0
Checking torrust-tracker-axum-health-check-api-server v0.1.0
error: the function has a cognitive complexity of (59/25)
  --> packages/swarm-coordination-registry/src/statistics/event/handler.rs:19:14
  |
19 | pub async fn handle_event(event: Event, stats_repository: &Arc<Repository>, now: DurationSinceUnixEpoch) {
  |              ^^^^^^^^^^^^
  |
  = help: you could split it up into multiple smaller functions
  = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.98.0/index.html#cognitive_complexity
  = note: `-D clippy::cognitive-complexity` implied by `-D warnings`
  = help: to override `-D warnings` add `#[allow(clippy::cognitive_complexity)]`

error: the function has a cognitive complexity of (29/25)
  --> packages/swarm-coordination-registry/src/statistics/event/listener.rs:30:10
  |
30 | async fn dispatch_events(mut receiver: Receiver, cancellation_token: CancellationToken, stats_repository: Arc<Repository>) {
  |          ^^^^^^^^^^^^^^^
  |
  = help: you could split it up into multiple smaller functions
  = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.98.0/index.html#cognitive_complexity

error: could not compile `torrust-tracker-swarm-coordination-registry` (lib) due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `torrust-tracker-swarm-coordination-registry` (lib test) due to 2 previous errors
```

- `handle_event` in `packages/swarm-coordination-registry/src/statistics/event/handler.rs` has complexity 59.
- `dispatch_events` in `packages/swarm-coordination-registry/src/statistics/event/listener.rs` has complexity 29.

After the swarm-coordination-registry refactor exposed subsequent workspace compilation, a
temporary root-policy enforcement check on 2026-09-04 found four additional violations in
`packages/tracker-core`:

- `handle_in_memory_event` in `src/statistics/event/handler.rs` has complexity 46.
- `handle_persistent_completed_statistics_event` in `src/statistics/event/handler.rs` has complexity 35.
- `dispatch_in_memory_events` in `src/statistics/event/listener.rs` has complexity 29.
- `dispatch_persistent_completed_statistics_events` in `src/statistics/event/listener.rs` has complexity 29.

After the tracker-core refactor exposed subsequent workspace compilation, a further temporary
root-policy enforcement check on 2026-09-04 found four additional violations:

- `handle_event` in `packages/http-core/src/statistics/event/handler.rs` has complexity 43.
- `dispatch_events` in `packages/http-core/src/statistics/event/listener.rs` has complexity 37.
- `handle_event` in `packages/udp-core/src/statistics/event/handler.rs` has complexity 38.
- `dispatch_events` in `packages/udp-core/src/statistics/event/listener.rs` has complexity 37.

After the HTTP and UDP core refactors exposed subsequent workspace compilation, a further
temporary root-policy enforcement check on 2026-09-05 found three additional violations in
`packages/udp-server`:

- `dispatch_events` in `src/banning/event/listener.rs` has complexity 29.
- `log_error` in `src/handlers/error.rs` has complexity 32.
- `dispatch_events` in `src/statistics/event/listener.rs` has complexity 37.

After the UDP-server refactor exposed subsequent workspace compilation, a further temporary
root-policy enforcement check on 2026-09-05 found one additional violation in the root
`torrust-tracker` crate:

- `run` in `src/console/profiling.rs` has complexity 27.

`Cargo.toml` defines the workspace Clippy policy, but `cognitive_complexity` is not included. Workspace lint settings are only inherited by manifests that opt into `[lints] workspace = true`; the affected package does not currently opt in. CI enforces Clippy through `linter all` (which runs `cargo clippy` for the workspace), so once the lint is declared in `Cargo.toml` and inherited by every package, the existing CI step enforces it without extra workflow changes.

Implementation baseline verified on 2026-09-03:

- `cargo metadata --no-deps --format-version=1` reports 26 workspace packages.
- Only six manifests, including the root manifest, currently declare `[lints] workspace = true`; 20 workspace packages do not inherit `[workspace.lints]`.
- `cargo clippy --workspace --all-targets --all-features -- -D clippy::cognitive_complexity -D warnings` reaches the same two violations and no additional cognitive-complexity violation before compilation stops. The command-line `-D` flag applies independently of manifest lint inheritance.
- `.github/workflows/testing.yaml` runs `linter all` for both nightly and stable toolchains; `linter all` includes the workspace Clippy run, so lints declared in `Cargo.toml` are enforced in CI through that step. No dedicated `cargo clippy` workflow step is needed.
- Listener testing is feasible without changing production design: `torrust_tracker_events::receiver::Receiver` is an object-safe async trait with `recv(&mut self) -> BoxFuture<'_, Result<Event, RecvError>>`. A scripted test receiver can exercise successful delivery, `Closed`, and `Lagged` results; a `CancellationToken` can exercise cancellation.

On 2026-09-03, all 20 remaining package manifests were temporarily updated to inherit workspace lints to establish a baseline, then reverted pending implementation. The resulting flag-free command, `cargo clippy --workspace --all-targets --all-features`, reported 87 error diagnostics before stopping. The largest reported groups were 32 `clippy::use_self`, 26 `clippy::missing_const_for_fn`, 14 `clippy::derive_partial_eq_without_eq`, and four `clippy::option_if_let_else` violations, primarily in `http-protocol` and `udp-protocol`. This issue includes remediation of every newly exposed diagnostic before enabling the cognitive-complexity lint, so the workspace remains clean throughout the policy change.

## Scope

### In Scope

- Refactor all fourteen identified handlers, listeners, error-log functions, and the profiling runner in `swarm-coordination-registry`, `tracker-core`, `http-core`, `udp-core`, `udp-server`, and the root crate until none exceeds the default `clippy::cognitive_complexity` threshold.
- Preserve event-to-metric and persistence updates, labels, timestamps, metrics-policy routing, listener cancellation priority, receiver-closed termination, lagged-receiver continuation, UDP error-response construction, profiling CLI/startup reporting, timer-versus-Ctrl+C behavior, and logging behavior.
- Retain and extend focused automated coverage where needed to protect the refactored behavior, especially listener receive-result handling, metrics-policy routing, persistence failure handling, UDP error-event publication, and profiling CLI behavior.
- Add `[lints] workspace = true` to every workspace package manifest that does not yet inherit the workspace lint policy, so the lint applies to all 26 packages.
- Fix every diagnostic exposed by the newly inherited workspace lint policy, preserving behavior and adding or adjusting focused regression coverage where a fix changes executable code.
- Add `cognitive_complexity` with level `deny` to `[workspace.lints.clippy]` in the root `Cargo.toml` only after the workspace is clean under the inherited existing policy.
- Confirm that the existing `linter all` step in CI (which runs `cargo clippy`) fails on a cognitive-complexity violation once the lint is declared in `Cargo.toml`.
- Correct documentation that incorrectly identifies `.cargo/config.toml` as the source of the Rust warning/lint policy, if that documentation is changed as part of enforcing this policy.

### Out of Scope

- Raising or lowering Clippy's default cognitive-complexity threshold.
- Adding `#[allow(clippy::cognitive_complexity)]` to bypass the lint.
- Changing the `Event` enum, event publication sites, metric names, metric labels, or listener lifecycle design.
- Changing the external `torrust-linting` repository.
- Adding a dedicated `cargo clippy` step to CI; `linter all` already runs Clippy.

## Architectural Decisions

- Related ADRs: `docs/adrs/20260727000000_events_are_objective_facts.md`.
- ADRs to create: None known. Create an ADR if the implementation introduces a repository-wide lint-inheritance or CI-policy approach with meaningful alternatives and lasting architectural consequences.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                            | Notes / Expected Output                                                                                                                                                         |
| --- | ------ | ----------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | DONE   | Establish enforcement baseline                  | Verified 26 workspace packages, temporarily enabled lint inheritance in all 20 remaining manifests, recorded the 87-diagnostic baseline, then reverted the experiment.          |
| T2  | IN_PROGRESS | Remediate workspace lint baseline           | Fix all 87 diagnostics exposed by full workspace-lint inheritance, retaining behavior and adding focused regression tests where needed.                                         |
| T3  | IN_PROGRESS | Refactor event-processing functions          | All event handlers/listeners and the UDP-server error logger are complete. Refactor the profiling runner while preserving its asymmetric shutdown behavior.                  |
| T4  | IN_PROGRESS | Refactor event listener loops                 | Swarm, tracker-core, HTTP core, and UDP core are complete. Apply the receive-result extraction to both UDP-server dispatchers.                                                  |
| T5  | IN_PROGRESS | Add focused regression tests                  | Event-processing coverage is complete. Add proportionate profiling CLI coverage where a deterministic executable fixture is practical.                                         |
| T6  | TODO   | Complete Cargo lint policy                      | Add `[lints] workspace = true` to every package manifest and add `cognitive_complexity = { level = "deny", priority = -1 }` only after T2 through T5 leave the workspace clean. |
| T7  | TODO   | Verify CI enforcement                           | Confirm `linter all` (as run in `testing.yaml`) fails on a cognitive-complexity violation and passes after the complete remediation; no workflow change expected.               |
| T8  | TODO   | Update affected documentation                   | Align lint-policy documentation with the final `Cargo.toml` and CI ownership.                                                                                                   |
| T9  | TODO   | Run verification and review acceptance criteria | Record automated and mandatory manual evidence, then re-review every acceptance criterion.                                                                                      |

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted in `docs/issues/drafts/`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue #2134 created and specification moved to `docs/issues/open/`
- [x] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded (status + evidence)
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [x] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-09-03 00:00 UTC - GitHub Copilot - Drafted from the failing workspace Clippy command: `handle_event` reported 59/25 and `dispatch_events` reported 29/25. - Terminal output in this session
- 2026-09-03 00:00 UTC - GitHub Copilot - Verified lint scope, CI coverage, and listener-test feasibility. The workspace has 26 packages, only six manifests opt into workspace lints, and CI enforces Clippy through `linter all`. - Terminal output and source inspection in this session
- 2026-09-03 00:00 UTC - GitHub Copilot - Temporarily enabled workspace lint inheritance in all 20 remaining package manifests and recorded a flag-free full-workspace baseline of 87 diagnostics, chiefly `use_self`, `missing_const_for_fn`, and `derive_partial_eq_without_eq`. The experiment was reverted; user expanded this issue to remediate every diagnostic before enabling the cognitive-complexity lint. - Terminal output and user direction in this session
- 2026-09-03 00:00 UTC - GitHub Copilot - Created GitHub issue #2134. - https://github.com/torrust/torrust-tracker/issues/2134
- 2026-09-03 00:00 UTC - Committer - Verified the specification progress and staged scope before the specification commit. - Local commit workflow
- 2026-09-03 00:15 UTC - GitHub Copilot - Created the implementation branch from merged `develop`. Began package-scoped workspace lint inheritance and baseline remediation; `udp-protocol` is complete in commit `df3bab7a`. - Local commit workflow
- 2026-09-04 UTC - GitHub Copilot - Completed workspace lint inheritance and all baseline remediation. Flag-free workspace Clippy passed. - Local commit workflow
- 2026-09-04 UTC - GitHub Copilot - Refactored the swarm-coordination-registry event handler and listener, adding deterministic lifecycle and metric regression coverage in `7e347911`. - Local commit workflow
- 2026-09-04 UTC - GitHub Copilot - A temporary root cognitive-complexity policy check exposed four additional tracker-core violations (46/25, 35/25, 29/25, and 29/25). Reverted the temporary policy and expanded this specification before implementation. - Terminal output and source inspection in this session
- 2026-09-04 UTC - GitHub Copilot - Refactored the tracker-core event handlers and listeners, adding deterministic lifecycle and persistence-failure regression coverage in `abc0c54e`. - Local commit workflow
- 2026-09-04 UTC - GitHub Copilot - A subsequent temporary root cognitive-complexity policy check exposed four additional HTTP and UDP core violations (43/25, 37/25, 38/25, and 37/25). Reverted the temporary policy and expanded this specification before implementation. - Terminal output and source inspection in this session
- 2026-09-04 UTC - GitHub Copilot - Refactored the HTTP and UDP core event handlers and listeners, adding deterministic lifecycle, policy-routing, and label-propagation coverage in `d635566e` and `7c6c609d`. - Local commit workflow
- 2026-09-05 UTC - GitHub Copilot - A subsequent temporary root cognitive-complexity policy check exposed three additional UDP-server violations (29/25, 32/25, and 37/25). Reverted the temporary policy and expanded this specification before implementation. - Terminal output and source inspection in this session
- 2026-09-05 UTC - GitHub Copilot - Refactored both UDP-server event listeners and the UDP error logger, adding deterministic listener and error-event regression coverage in `33387541`. - Local commit workflow
- 2026-09-05 UTC - GitHub Copilot - A subsequent temporary root cognitive-complexity policy check exposed the root profiling runner violation (27/25). Reverted the temporary policy and expanded this specification before implementation. - Terminal output and source inspection in this session

## Acceptance Criteria

- [ ] AC1: All fourteen identified handlers, listeners, error-log functions, and the profiling runner in `swarm-coordination-registry`, `tracker-core`, `http-core`, `udp-core`, `udp-server`, and the root crate comply with Clippy's default cognitive-complexity threshold without a `clippy::cognitive_complexity` allowance.
- [ ] AC2: Existing observable event-metric and persistence behavior, metric labels, timestamps, metrics-policy routing, listener ordering, termination behavior, UDP error responses and event publication, profiling CLI/startup reporting, timer-versus-Ctrl+C behavior, and logging semantics are preserved.
- [ ] AC3: Root `Cargo.toml` declares `clippy::cognitive_complexity` as a denied workspace lint, and every workspace package manifest inherits workspace lints via `[lints] workspace = true`.
- [ ] AC4: `cargo clippy --workspace --all-targets --all-features` (with no extra `-D` flags) fails on a cognitive-complexity violation, so the existing `linter all` CI step enforces it.
- [ ] AC5: Every diagnostic exposed by enabling workspace lint inheritance, including the 87-diagnostic baseline, is fixed without lint allowances that weaken the workspace policy.
- [ ] AC6: Focused regression tests cover behavior affected by the baseline remediation, including listener receive outcomes where practicable.
- [ ] `linter all` exits with code `0`.
- [ ] Relevant tests pass.
- [ ] Manual verification scenarios are executed and documented (status + evidence).
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior.
- [ ] Documentation is updated when behavior/workflow changes.

## Verification Plan

Define verification before implementation starts and execute it before closing the issue.

### Automatic Checks

- `cargo fmt --check`
- `cargo test -p torrust-tracker-swarm-coordination-registry`
- `cargo clippy -p torrust-tracker-swarm-coordination-registry --all-targets --all-features`
- `cargo test -p torrust-tracker-core --all-features`
- `cargo clippy -p torrust-tracker-core --all-targets --all-features`
- `cargo test -p torrust-tracker-http-core --all-features`
- `cargo clippy -p torrust-tracker-http-core --all-targets --all-features`
- `cargo test -p torrust-tracker-udp-core --all-features`
- `cargo clippy -p torrust-tracker-udp-core --all-targets --all-features`
- `cargo test -p torrust-tracker-udp-server --all-features`
- `cargo clippy -p torrust-tracker-udp-server --all-targets --all-features`
- `cargo clippy -p torrust-tracker --lib -- -D clippy::cognitive_complexity -D warnings`
- `cargo clippy --workspace --all-targets --all-features` (must fail before the refactor once the lint is declared, and pass after)
- `linter all`
- `cargo test --doc --workspace`
- `cargo test --tests --benches --examples --workspace --all-targets --all-features`
- `TORRUST_GIT_HOOKS_LOG_DIR=.tmp ./contrib/dev-tools/git/hooks/pre-commit.sh`
- `TORRUST_GIT_HOOKS_LOG_DIR=.tmp ./contrib/dev-tools/git/hooks/pre-push.sh` when applicable

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                       | Command/Steps                                                                                                                                                                | Expected Result                                                                                                                                             | Status | Evidence                          |
| --- | ------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------- | ------ | --------------------------------- |
| M1  | Validate workspace remediation | Run `cargo clippy --workspace --all-targets --all-features` after completing all baseline fixes and again after enabling cognitive complexity.                               | The command exits 0 both before and after the new lint is enabled.                                                                                          | TODO   | Pending workspace Clippy output   |
| M2  | Validate CI enforcement        | Temporarily reintroduce one cognitive-complexity violation (or check out the pre-refactor commit) with the lint declared, run `linter all`, then restore the fix and re-run. | `linter all` fails on the violation and passes after the fix, proving the existing CI step enforces the lint.                                               | TODO   | Pending local `linter all` output |
| M3  | Validate listener behavior     | Exercise cancellation, closed receiver, successful event delivery, and lagged receiver paths with focused tests or deterministic test harness steps.                         | Cancellation and closed receiver terminate; successful events are handled; lagged receivers continue; existing log and ordering semantics remain unchanged. | TODO   | Pending focused test output       |

Notes:

- Manual verification is mandatory even when automated tests pass.
- If a scenario fails, record the failure and diagnosis in the progress log before proceeding.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                                                              |
| ----- | ---------------------- | --------------------------------------------------------------------- |
| AC1   | TODO                   | Pending focused and workspace Clippy output                           |
| AC2   | TODO                   | Pending regression-test and manual-scenario evidence                  |
| AC3   | TODO                   | Pending Cargo lint-inheritance inspection and workspace Clippy output |
| AC4   | TODO                   | Pending `linter all` fail/pass evidence and CI run                    |
| AC5   | TODO                   | Pending clean workspace Clippy output                                 |
| AC6   | TODO                   | Pending focused test output                                           |

## Risks and Trade-offs

- Workspace lint declarations are ineffective for packages that do not inherit them. Mitigate by adding `[lints] workspace = true` to every manifest and verifying with a flag-free `cargo clippy --workspace`.
- Remediating all 87 baseline diagnostics expands the change across multiple packages. Mitigate by grouping independent fixes into small commits, running focused package tests after each group, and retaining behavior-focused tests.
- Helper extraction can subtly alter metrics, labels, listener priority, or termination behavior. Preserve current control flow at externally significant boundaries and protect it with focused regression tests.
- `linter all` is installed unpinned from `torrust-linting` in CI, so its exact Clippy flags could change. Because the lint lives in `Cargo.toml`, it is enforced by any `cargo clippy` invocation regardless of the linter's flags.

## References

- Current failing command: `cargo clippy --workspace --tests -- -W clippy::cognitive_complexity -D warnings`
- `Cargo.toml`
- `.github/workflows/testing.yaml`
- `packages/swarm-coordination-registry/src/statistics/event/handler.rs`
- `packages/swarm-coordination-registry/src/statistics/event/listener.rs`
- `packages/tracker-core/src/statistics/event/handler.rs`
- `packages/tracker-core/src/statistics/event/listener.rs`
- `packages/http-core/src/statistics/event/handler.rs`
- `packages/http-core/src/statistics/event/listener.rs`
- `packages/udp-core/src/statistics/event/handler.rs`
- `packages/udp-core/src/statistics/event/listener.rs`
- `packages/udp-server/src/banning/event/listener.rs`
- `packages/udp-server/src/handlers/error.rs`
- `packages/udp-server/src/statistics/event/listener.rs`
- `src/console/profiling.rs`
- `packages/e2e-tools/src/bin/profiling.rs`
- `docs/adrs/20260727000000_events_are_objective_facts.md`
- `docs/issues/closed/1786-tighten-lint-config.md`
