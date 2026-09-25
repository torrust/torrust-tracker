doc-type: issue
issue-type: task
status: done
priority: p2
epic: 1488
github-issue: 2234
spec-path: docs/issues/closed/2234-1488-si-2-remove-global-shutdown-signal/ISSUE.md
branch: "2234-1488-si-2-token-server-lifecycle"
related-pr: null
last-updated-utc: 2026-09-25
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
    - docs/features/shutdown-process/questions.md
    - docs/adrs/20260902074438_adopt_supervised_cancellation_tree_for_shutdown.md
    - packages/axum-server/src/signals.rs
    - packages/axum-http-server/src/server.rs
    - packages/axum-rest-api-server/src/server.rs
    - packages/axum-health-check-api-server/src/server.rs
    - packages/udp-server/src/server/launcher.rs
---

<!-- markdownlint-disable MD003 -->
<!-- skill-link: create-issue -->

# Issue #2234 - Add Token-Aware Server Lifecycle API

Parent: [EPIC #1488 - Overhaul: Tracker Shutdown](../../open/1488-overhaul-tracker-shutdown/ISSUE.md)

## Goal

Add an additive `torrust-server-lib` shutdown-wait primitive that resolves from an injected `CancellationToken` and never subscribes to OS signals. Release it, then update this workspace to consume that released version without migrating any existing server consumer yet.

## Background

The tracker owns its root cancellation lifecycle through `JobManager`, but HTTP, REST API, health-check API, and UDP server code still bridge that lifecycle into legacy `Halted` channels. The legacy `torrust_server_lib::signals::shutdown_signal()` also races the application by independently subscribing to `SIGINT` and `SIGTERM` through `global_shutdown_signal()`.

`torrust-server-lib` is a standalone crate consumed here at version `0.2.0`. Its `signals` module is a small wait utility: it defines `Started`, `Halted`, `global_shutdown_signal()`, `shutdown_signal()`, and `shutdown_signal_with_message()`. It does not own Axum handles, drain controllers, or server tasks; those live in this workspace's server packages. SI-2 therefore adds only the cancellation-wait contract. Joinable drain controllers are SI-10, and consumer migrations are SI-11 through SI-17, so legacy `Halted` and OS-signal paths must remain source- and behavior-compatible until SI-18 and SI-19.

The crate is maintained by this project's agents and has no third-party consumers, tracked issues, or agent configuration. The work is done directly in the existing local clone with a focused branch and PR in `torrust/torrust-server-lib`; no separate upstream issue handoff is required. The tracker keeps this specification as the authoritative requirement source.

## Scope

### In Scope

- In the local `torrust-server-lib` clone, add an additive token-aware shutdown-wait API in `src/signals.rs` that awaits an injected `CancellationToken` and does not call `tokio::signal`.
- Add deterministic server-lib tests proving the new primitive resolves on token cancellation without an OS signal, and that legacy `Halted` waiting is unchanged.
- Open and merge a focused `torrust/torrust-server-lib` PR, publish the next crate version, and record the exact version, PR, commit, and release evidence in this issue's evidence before tracker adoption.
- Update this workspace from `torrust-server-lib` `0.2.0` to that exact released version, including the lockfile.
- Add tracker-side compile or contract coverage proving the new API is available without migrating existing HTTP, REST API, health-check API, UDP, or standalone consumers.
- Document the consumer contract for SI-10 through SI-17: a component derives a child token, passes it to the new wait primitive, and owns and joins every graceful-stop controller before its top-level future completes.

### Out of Scope

- Making `torrust-server-lib` own, spawn, or join Axum drain controllers or server tasks; the joinable drain helper is SI-10.
- Migrating any tracker server component or standalone example to the new API; those are SI-10 through SI-17.
- Removing, deprecating, or changing the behavior of `global_shutdown_signal()`, `shutdown_signal()`, `shutdown_signal_with_message()`, `Halted`, or existing start APIs.
- Changing `JobManager` cancellation wiring, which is already established.
- Defining shutdown timeout configuration or readiness-before-drain behavior.

## Architectural Decisions

- Related ADR: [supervised cancellation tree](../../../adrs/20260902074438_adopt_supervised_cancellation_tree_for_shutdown.md).
- The lifecycle API is additive and accepts injected cancellation; libraries do not own operating-system signal handling.
- `torrust-server-lib` stays a wait/signal utility. Controller ownership and joining stay in the workspace server packages, starting with SI-10.
- The compatible release is the exact version published after the server-lib API and compatibility tests pass. Do not preselect a version number or substitute an unpublished Git dependency.
- Each server component owns and joins its internal graceful-stop controller. `JobManager` owns only top-level component futures and never nested server task handles.
- ADRs to create: None known. Create one during implementation if the public API requires a consequential lifecycle or compatibility decision not covered by the existing ADR.

## Design and Ownership Review

| Concern | Contract |
| ------- | -------- |
| Application | `main()` is the only OS-signal boundary; `JobManager` owns root cancellation and awaits named top-level components. |
| `torrust-server-lib` | Provides the token-aware wait primitive and legacy wait functions; it spawns no tasks and owns no controllers. |
| Server component (later SIs) | Derives or receives a child token, passes it to the wait primitive, and does not expose child handles to `JobManager`. |
| Server implementation (later SIs) | Owns the graceful-stop controller and joins it on normal cancellation, startup failure, and collaborator-drop paths before completing. |
| Legacy consumer | Continues to use current `Halted` and OS-signal behavior unchanged until explicitly migrated. |
| Awaited readiness | SI-2 introduces no readiness await. Any future await added by later SIs must have an explicit, caller-owned deadline. |

After the server-lib change passes its focused tests, perform a design review against this map. Confirm the primitive cannot subscribe to OS signals, adds no task ownership to the library, and preserves every legacy signature.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| -- | ------ | ---- | ----------------------- |
| T1 | DONE | Branch the local server-lib clone | Created `2234-token-aware-shutdown-wait` from `origin/main`. |
| T2 | DONE | Add token-aware wait primitive | Added `signals::cancellation_signal(CancellationToken)` with direct `tokio-util` `rt` dependency; legacy APIs remain unchanged in `ea333ec`. |
| T3 | DONE | Add server-lib tests | Added deterministic token-cancellation and legacy `Halted::Normal` tests in `ea333ec`; focused and full suites pass. |
| T4 | DONE | Merge and release server-lib | Merged [server-lib PR #1](https://github.com/torrust/torrust-server-lib/pull/1) at `2b3f8ec` after CI and Copilot approval; published `torrust-server-lib` `0.3.0` to crates.io. |
| T5 | DONE | Adopt released crate in tracker | Updated all six workspace server-lib declarations and lockfile to the published `0.3.0` version. |
| T6 | DONE | Prove tracker compatibility | Added focused `axum-server` API contract test; it compiles and awaits `cancellation_signal` from the published crate while production consumers retain legacy paths. |
| T7 | DONE | Review consumer guidance | Confirmed SI-10 owns joinable drain-controller implementation; SI-11 through SI-17 own consumer migration and controller joins. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T1 | Branch creation | No commit; record branch name in this spec. |
| T2 | Additive server-lib primitive and dependency | Signed server-lib commit after focused validation. |
| T3 | Server-lib tests | Signed server-lib commit after the prose-first Arrange-Act-Assert review. |
| T4 | Server-lib version bump and release | Signed server-lib release commit; record immutable release evidence in this issue. |
| T5 | Tracker dependency update and lockfile | Signed `chore(deps)` commit after focused build/test validation. |
| T6 | Tracker contract coverage | Signed `test` commit after the required prose-first Arrange-Act-Assert review. |
| T7 | Consumer guidance and completion evidence | Signed documentation commit when tracked documentation changes. |

For every test-producing task, use the `write-unit-test` skill and complete the prose-first Arrange-Act-Assert comparison before commit. Record the review in issue evidence, ensuring the test exposes one causal initial-state difference and keeps the production Act and expected outcome visible.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted and promoted to `docs/issues/open/2234-1488-si-2-remove-global-shutdown-signal/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue #2234 created, attached to EPIC #1488, and added to this spec
- [x] Spec committed on this implementation branch before server-lib work begins
- [x] Server-lib branch `2234-token-aware-shutdown-wait` created in the local clone and recorded here
- [x] Additive server-lib API implemented, reviewed, merged, and released as `torrust-server-lib` `0.3.0`
- [x] Tracker dependency updated to the released server-lib version
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests, and pre-push checks)
- [x] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [x] Evidence-based implementation completion review recorded
- [x] Reviewer validated acceptance criteria and updated checkboxes
- [x] Independent reviewer reports recorded in issue-local `agent-review-reports.md`
- [x] Committer verified spec progress is up to date before commit
- [x] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-09-25 - GitHub Copilot - Verified GitHub issue #2234 is closed and archived this specification in `docs/issues/closed/`.

- 2026-09-15 13:20 UTC - Copilot - Reworked the existing SI-2 draft to define the release-first dependency and current repository verification requirements.
- 2026-09-16 09:00 UTC - Copilot - Narrowed server-lib scope to the token-aware wait primitive; controller ownership moved to SI-10; documented direct work in the local server-lib clone instead of an upstream issue handoff.
- 2026-09-16 09:10 UTC - Copilot - Created GitHub issue #2234, attached it to EPIC #1488, renamed this branch to `2234-1488-si-2-token-server-lifecycle`, and promoted the folder specification to `open`.
- 2026-09-16 09:15 UTC - Copilot - Committed and pushed the approved specification as `1d4ddb10` after the full lint, pre-commit, and pre-push gates passed.
- 2026-09-16 09:20 UTC - Copilot - Created server-lib branch `2234-token-aware-shutdown-wait`; implemented and tested the additive token-aware waiter in signed commit `ea333ec`; opened server-lib PR #1. The required CI is in progress, so merge and publication remain pending.
- 2026-09-16 09:30 UTC - Copilot - Server-lib PR #1 merged at `2b3f8ec` after successful CI and a Copilot approval with no threads; published `torrust-server-lib` `0.3.0` to crates.io. Updated tracker pins and added a focused API contract test; `cargo test -p torrust-tracker-axum-server it_should_compile_and_resolve_the_server_lib_cancellation_signal_when_token_is_cancelled` passed.
- 2026-09-16 09:30 UTC - Copilot - Completed the required prose-first test review for the tracker contract test. Arrange exposes a fresh token and waiting task; Act visibly cancels that token; Assert awaits the external API's completion. The test has one causal condition, does not derive expected behavior from production code, and uses no fixture or OS signal.
- 2026-09-16 10:22 UTC - Copilot - Validated tracker adoption with `cargo machete --with-metadata`, `linter all`, workspace documentation tests, the complete `axum-server` test suite, and `cargo check --workspace`. Direct-binary verification recorded a clean legacy `SIGINT` exit and two consecutive `SIGTERM` exits using the same isolated loopback configuration; the focused contract test confirmed token-only completion without an OS signal. No material API deviation or retrospective was needed.
- 2026-09-16 10:50 UTC - Copilot - Ran the mandatory pre-commit gate before signed tracker adoption commit `25f0ab34`; the required pre-push suite then passed nightly formatting, nightly workspace checks, documentation build, and the full test suite. Independent review findings are resolved and recorded in `agent-review-reports.md`.

## Acceptance Criteria

- [x] AC1: A published `torrust-server-lib` version provides an additive wait primitive that resolves from an injected `CancellationToken` without subscribing to OS signals.
- [x] AC2: Existing `global_shutdown_signal()`, `shutdown_signal()`, `shutdown_signal_with_message()`, and `Halted` remain source- and behavior-compatible in that release.
- [x] AC3: The new primitive does not take or expose a shutdown `Halted` channel; `Started` startup signaling remains unaffected.
- [x] AC4: The tracker consumes the exact published version, with no unpublished Git dependency.
- [x] AC5: Server-lib tests prove token cancellation resolves the primitive and legacy `Halted` waiting is unchanged; tracker contract coverage compiles against the new API.
- [x] AC6: Consumer documentation requires later server components to own and join graceful-stop controllers before top-level completion, and names SI-10 as the joinable drain helper.
- [x] `linter all` exits with code `0`.
- [x] Relevant server-lib and tracker tests pass.
- [x] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`.
- [x] Acceptance criteria are re-reviewed after implementation and reflect actual behavior.
- [x] Documentation is updated when behavior or workflow changes.

## Verification Plan

### Automatic Checks

- In the `torrust-server-lib` clone: `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`, and `cargo publish --dry-run` before release.
- In this repository: `linter all`, `cargo machete`, focused tests for each affected server package, `cargo test --doc --workspace`, and the pre-push checks before publishing tracker changes.
- After the tracker dependency update: compile the complete workspace and run its relevant server lifecycle tests using the released crate version.

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| -- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1 | Legacy consumer remains usable | Build and run the tracker against the released crate without changing any legacy shutdown call site, then stop it with `SIGINT`. | Existing `Halted`-based shutdown behavior remains available and the tracker exits cleanly. | DONE | `manual-verification-evidence.md` V1 |
| M2 | Token-aware wait has no signal dependency | Run a minimal example that awaits the new primitive, cancel its injected token from another task, and observe completion without sending `SIGINT` or `SIGTERM`. | The wait resolves through the token alone. | DONE | `manual-verification-evidence.md` V2 |
| M3 | Tracker process cleanly restarts | Build the release tracker, signal the actual tracker binary with `SIGTERM`, await exit, then restart it immediately. | Shutdown completes and all configured services rebind without address-in-use errors. | DONE | `manual-verification-evidence.md` V3 |

Create `manual-verification-evidence.md` from the repository template when executing scenarios. Record actual commands, output, relevant logs, release version, and outcomes. A failure must be recorded in the progress log before proceeding.

### Disposable Verification Scripts

No disposable verification script is planned. Durable lifecycle behavior must be covered by upstream and tracker Rust tests; manual scenarios record the human-operated release and process behavior.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | DONE | Published `torrust-server-lib` `0.3.0`; server-lib token-wait test and tracker M2 contract test |
| AC2 | DONE | Server-lib legacy compatibility test, unchanged tracker legacy call sites, and tracker M1 direct-binary verification |
| AC3 | DONE | Server-lib public `cancellation_signal(CancellationToken)` signature review; no `Halted` parameter or result |
| AC4 | DONE | Six tracker manifests and `Cargo.lock` resolve registry `torrust-server-lib` `0.3.0`; workspace build passed |
| AC5 | DONE | Server-lib release validation; focused tracker contract test; complete `axum-server` suite |
| AC6 | DONE | Scope, design-and-ownership review, and SI-10 through SI-17 guidance in this specification |

## Risks and Trade-offs

- Tracker adoption is blocked until the crate is published. Mitigation: land and release the small additive primitive as the first independently reviewable increment.
- A new API could accidentally alter legacy paths. Mitigation: preserve their public signatures and add server-lib compatibility tests before release.
- Scope creep into controller ownership would couple the library to Axum. Mitigation: keep the library a wait utility; defer joinable drain controllers to SI-10.
- The API may need revision during later component migrations. Mitigation: do not migrate tracker consumers until the released public contract is stable; a second additive release is acceptable.

## Implementation Completion Review

After implementation, compare the release and tracker adoption with this specification. Record invalidated assumptions, API changes, validation findings, and reusable lessons.

- Retrospective: No separate retrospective required. The released API exactly matches the approved additive wait-utility scope; controller ownership remains deferred to SI-10 and consumer migrations to SI-11 through SI-17.
- Create `implementation-retrospective.md` from the repository template for material server-lib API discoveries or deviations; otherwise record the justified no-retrospective decision in the progress log.
- An independent reviewer records results in `agent-review-reports.md` after receiving this folder-style specification.

## References

- Parent EPIC: [#1488](https://github.com/torrust/torrust-tracker/issues/1488)
- Closed foundations: [#1586](https://github.com/torrust/torrust-tracker/issues/1586), [#1588](https://github.com/torrust/torrust-tracker/issues/1588), [#2169](https://github.com/torrust/torrust-tracker/issues/2169), and [#2221](https://github.com/torrust/torrust-tracker/issues/2221)
- Server-lib repository: [torrust/torrust-server-lib](https://github.com/torrust/torrust-server-lib) (local clone: `../torrust-server-lib` relative to this workspace's parent directory)
- Next roadmap item consuming this API: [#2274](https://github.com/torrust/torrust-tracker/issues/2274), [joinable Axum drain helper](../2274-1488-si-10-add-token-aware-axum-drain-helper/ISSUE.md)
