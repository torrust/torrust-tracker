---
schema-version: 1
doc-type: issue
issue-type: task
status: done
priority: p2
epic: 1488
github-issue: 2697
spec-path: docs/issues/closed/2697-1488-si-21-mark-health-unhealthy-during-shutdown/ISSUE.md
branch: "2697-mark-health-unhealthy-during-shutdown"
related-pr: 2720
last-updated-utc: "2026-11-04 17:38"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - src/main.rs
    - src/app.rs
    - src/bootstrap/jobs/manager.rs
    - src/bootstrap/jobs/health_check_api.rs
    - packages/axum-health-check-api-server/src/server.rs
    - packages/axum-health-check-api-server/src/handlers.rs
    - docs/features/shutdown-process/README.md
    - docs/features/shutdown-process/questions.md
    - docs/issues/closed/2324-1488-si-13-migrate-health-check-api-token-lifecycle/ISSUE.md
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
    - docs/issues/closed/2697-1488-si-21-mark-health-unhealthy-during-shutdown/manual-verification-evidence.md
    - docs/adrs/20261021184946_order_shutdown_by_event_flow_and_drain_event_listeners.md
---

<!-- skill-link: create-issue -->

# Issue #2697 - Mark Health Check Unhealthy During Shutdown (SI-21)

> **EPIC position**: Readiness-before-drain vertical slice (roadmap row 21).
> Every prerequisite is done on `develop`: SI-13 (#2324), SI-22 (#2410) and
> SI-20 (#2668). It uses the configured process deadline `P`
> (`[application.shutdown]`, default 25 s) and adds no timer.
>
> Refreshed on 2026-10-28 against `develop` (merge of #2686). Facts marked
> "read from source, not run" were read, not executed; the measured behaviour is
> in `manual-verification-evidence.md`.

## Goal

On a normal shutdown request, mark the tracker as not ready before server
components begin draining. While draining, `/health_check` returns HTTP 503 so
Kubernetes readiness probes and readiness-aware load balancers stop routing new
traffic to this instance. Existing accepted connections continue through their
component-specific graceful shutdown policy.

**Measured on the real binary (baseline, before any change):** after SIGINT or
SIGTERM the health-check server stops accepting new connections at once, so a
new probe sees a reset or `Connection refused`, never a 503, and the 503 would
be visible only on already-open connections, which also close. Setting a
readiness flag before the root-token cancellation therefore gives no observable
503 on its own: see "Design Decision" below.

This task changes readiness state only. It neither changes token propagation nor
introduces an independent shutdown timer.

## Decision Context

Q6 approved a two-phase service shutdown:

```text
shutdown request → mark not ready → drain existing work → process exits
```

The readiness state is observable only by infrastructure that uses the health
endpoint. It cannot prevent direct clients from sending UDP packets or opening
new direct TCP connections; protocol components still own admission and drain
behavior.

## Design Decision

Cause (read from source, not run): `src/app.rs` `start_health_check_api`
spawns the health-check component with no stop-before edge, so
`JobManager::cancel()` (called from `src/main.rs` right after the "shutting
down" log) cancels its token together with every component that waits for no
other job. Its `graceful_shutdown_on_cancellation` then stops axum accepting.

Maintainer decisions (recorded 2026-11-03 from the review chat; the maintainer
approved creating the issue from this text on the same day):

- Option (a) is accepted: "I think option A is fine".
- The health server's own near-instant idle drain is not budgeted inside `P`.
- Reserving `health_check_api` first is accepted; it is still bound and spawned
  last, so startup order and port-conflict behaviour do not change. Every other
  supervised component declares `StopBefore` on it. Read from source, not run:
  a target only has to be reserved, not spawned, and a reservation dropped by a
  failed startup is released (`release_if_reserved`). A test must assert that
  every supervised component has the health server as a successor, so a
  component added later cannot silently stop concurrently with it.
- Readiness state shape (decided in the implementation review chat, 2026-11-04): a one-way
  `Readiness` handle (`Arc<AtomicBool>`, default ready, `is_ready()`, `mark_not_ready()`) in
  `torrust-tracker-primitives` (3.1.0), owned by `AppContainer` (so the `app::start*` return types do
  not change) and injected into the health check API (0.2.0). `begin_shutdown` in `src/app.rs` marks it
  not ready and then cancels; `main` calls it. The maintainer's answers count as the Phase 2 design
  review, so implementation proceeded in one pass.
- `/health_check` returns 503 before it touches the `Registar` (no probe fan-out), with the body
  `{"status":"Error","message":"shutting down","details":[]}` (the existing `Report` shape).
- The stop-last edge is added by the manager, not by each spawn helper: `JobManager::reserve_stop_last`
  reserves the health check first and every component spawned afterwards stops before it. This covers
  future components without touching the helpers. The legacy `udp_ban_cleanup` job has no component id
  and is left alone (documented).
- A short ADR records the "health check stops last" invariant
  (`docs/adrs/20261104120915_stop_the_health_check_api_last_to_report_not_ready_during_shutdown.md`).

Options considered:

- **(a) Health-check server stops last (accepted).** Spawn it with
  `StopBefore` the other supervised components, as SI-22 does for event
  listeners and as `src/app.rs` does for the runner. The readiness flip happens
  before `jobs.cancel()`; the health server keeps serving 503 for the whole
  drain and is cancelled only after every component it waits for completed.
  Uses the existing `JobManager` API, no timer, no Q6 amendment.
  Risk check (all read from source in `src/bootstrap/jobs/manager.rs`, not run):
  - The exit stays bounded by P: `wait_for_all` shares one deadline across all
    jobs, and a last-stopping health server is aborted at P at the latest.
  - A failure is not hidden: failed, panicked and aborted jobs all appear in
    the outcomes, and `report_shutdown` returns exit 1 for each of them. A
    health server aborted at P also counts as aborted.
  - A failed or panicked predecessor still releases its dependents, because
    `mark_finished` runs for every outcome that has a component id.
  - Worst case, a hung predecessor keeps the health server serving 503 until P.
  - Budget: the health server's own graceful drain starts only after the others
    finish, with `P - elapsed` left, and `max(H, U) + L < P` does not budget for
    it. Idle, its drain is near-instant; the maintainer accepted this.
  - Reservation order: `add_stop_before` needs target ids strictly earlier than
    the added id, and `health_check_api` is reserved last (`src/app.rs`), so it
    must be reserved first (its server may still start last).
  - The 503 handler must be a constant-time atomic read of application-owned
    readiness state, with no `Registar` probe fan-out.
  Also, the health list still shows services that stopped (see the Registar
  note).
- **(b) Flip then cancel, document only.** Simple, but a refused connection also
  fails a Kubernetes readiness probe, so the 503 adds almost nothing and
  acceptance criteria 3 and 7 could not be met on new connections.
- **(c) Short documented grace between flip and cancel.** Amends Q6 (which
  forbids a separate budget), needs an ADR and a spec change, and adds latency
  to every shutdown.

NOTE (not implemented here): the maintainer suggested removing services
from the `Registar` as they shut down so the health list stops reporting
services that are no longer available. `Registar::deregister` exists since
`torrust-server-lib` 0.4.0 (read from source, not run).

## Scope

### In scope

- Introduce application-owned readiness state with healthy as its startup
  default and not-ready as its irreversible shutdown state.
- Set readiness to not ready immediately when `main()` / `JobManager` initiates
  normal shutdown, before the health-check server is cancelled (before `jobs.cancel()` in
  `src/main.rs`; under option (a) the server stays up afterwards).
- Make the health-check endpoint return HTTP 503 while not ready, without
  running downstream service probes.
- Preserve the current healthy response behavior and probe fan-out before a
  shutdown request.
- Add deterministic tests that set readiness without OS signals and verify 503
  plus the absence of downstream probe execution.
- Add focused manual verification that records readiness transition before
  component drain and process exit.

### Out of scope

- Changing tracker HTTP/UDP listener admission or direct-client behavior.
- New health endpoint routes, retry timers, or an independent readiness timeout.
- Changing cancellation-tree ownership, component drain budgets, or exit codes.
- Health-check API token lifecycle migration; SI-13 owns that prerequisite.
- Kubernetes manifests or deployment automation beyond documenting the required
  readiness-probe behavior.

## Implementation Constraints

1. Readiness is owned by the application lifecycle, not by an individual server
   or by a request handler-local value.
2. The transition from ready to not ready is one-way for a process lifetime.
   Restarting the tracker creates a new ready lifecycle.
3. The readiness state must be safe to read concurrently from health requests
   and safe to set during shutdown initiation.
4. A not-ready response has HTTP status 503 and must not execute service probes;
   it reports lifecycle state, not a transient probe failure.
5. The readiness transition happens before the health-check server stops and
   consumes no separate time budget within the process deadline `P`
   (`[application.shutdown] deadline_in_secs`, default 25 s, SI-20).
6. If startup fails before readiness becomes ready, the process follows Q3's
   startup-failure exit result and must not advertise readiness.
7. The health-check crate must not depend on the root crate: the readiness
   state is a type owned outside it and injected (read `docs/packages.md`; the
   exact home is fixed at the Phase 2 design review).

## Architectural Decisions

- Related ADRs:
  `docs/adrs/20261021184946_order_shutdown_by_event_flow_and_drain_event_listeners.md`
  (stop order by event flow; the health-check component gains a stop-after
  relationship to every other component) and
  `docs/adrs/20260629000000_adopt_independent_package_versioning.md` (a public
  API change in a published package).
- ADRs to create: none known. Option (c) would need one (it amends Q6), but it
  was not chosen. If the "health server stops last" rule proves to be a lasting
  ordering decision, record it in an ADR at implementation time.

## Design and Ownership Review

Applicable: this task changes asynchronous shutdown ordering and a network
readiness response.

- **Readiness state**: owned by the application lifecycle. Interface: a one-way
  `mark not ready` write held by `main`, and a cheap read held by the
  health-check handler. The type is defined where the health crate can use it
  without depending on the root crate; its final shape is fixed at the design
  review below.
- **Stop order**: the health-check component is reserved first, bound and
  spawned last, and stops after every other supervised component. `JobManager`
  owns the edges; `app.rs` declares them.
- **Lifetime invariants**: normal path, the flag flips once before
  `jobs.cancel()` and the health server is cancelled after the last other
  component completes. Failure path, a failed or panicked predecessor still
  releases the health server, and a hung one keeps it serving 503 until the
  shared process deadline `P` aborts everything (exit 1). Drop path, a
  reservation dropped by a failed startup is released by the manager.
- **Deadline**: every wait in a new test carries an absolute bound; the real
  binary evidence bounds the process wait by `P` plus a margin.
- **Design-review checkpoint**: after the first implementation step fixes the
  readiness API shape, stop for maintainer review before wiring it through
  `main` and `JobManager`.

## Bug-Fix Process

Not applicable: this is a planned feature slice (readiness during shutdown). The
measured baseline that a new probe sees no 503 is recorded in
`manual-verification-evidence.md` (V1, V2) and is the reason for design (a).

## Regression Test Strategy

Not applicable: not substantively a bug. The behaviour under test is new, and the
deterministic tests are listed in the Implementation Plan.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                                    | Notes / Expected Output                                                                                              |
| --- | ------ | ----------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------- |
| T1  | DONE   | Define the one-way readiness state and its handler read.                | Unit tests: ready, not ready, one-way, concurrent reads. Stop for design review after this step.                     |
| T2  | DONE   | Return 503 from `/health_check` while not ready, with no probe fan-out. | Handler test proves no `Registar` probe runs; healthy body unchanged.                                                |
| T3  | DONE   | Reserve `health_check_api` first, spawn last, and stop it after the rest. | Test: every supervised component has the health server as a successor; failed and hung predecessors still end in P. |
| T4  | DONE   | Flip readiness in `main` before `jobs.cancel()`.                        | Ordering test without OS signals.                                                                                    |
| T5  | DONE   | Real-binary evidence for SIGTERM and SIGINT.                            | `manual-verification-evidence.md`: timestamped 503 probes on new connections, exit status, listener release.         |
| T6  | DONE   | Document the readiness contract for operators.                          | Deployment docs name `/health_check` as the readiness probe and the 503 semantics.                                   |

## Commit Points

| Task | Coherent change set                                          | Commit policy                                        |
| ---- | ------------------------------------------------------------ | ---------------------------------------------------- |
| T1   | Readiness type and its unit tests                            | Commit after focused validation and required review. |
| T2   | Handler 503 path and tests                                   | Commit after focused validation and required review. |
| T3   | Reservation order, stop-after edges and their tests          | Commit after focused validation and required review. |
| T4   | Readiness flip in `main` and its ordering test               | Commit after focused validation and required review. |
| T5   | Real-binary evidence                                         | Commit after the last production change.             |
| T6   | Operator documentation                                       | Commit after focused validation and required review. |

Use the `write-unit-test` skill for every test increment, record the prose-first
Arrange-Act-Assert design review in task evidence, mutation-prove each new test,
and sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/1488-si-21-mark-health-unhealthy-during-shutdown/ISSUE.md`, moved to `docs/issues/open/`
- [x] Spec reviewed and approved by user/maintainer (design and issue creation approved in the review chat, 2026-11-03)
- [x] GitHub issue created (#2697) and issue number added to this spec
- [x] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation (#2698)
- [x] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [x] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md`
- [ ] Committer verified spec progress is up to date before commit
- [x] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-10-28 18:06 UTC - AI agent (model not stated by the runtime) - Refreshed the 2026-09-25 draft against `develop` and measured that a new health-check connection gets no 503 during a drain - `manual-verification-evidence.md` V1, V2
- 2026-11-03 16:44 UTC - AI agent (model not stated by the runtime) - Recorded the maintainer's decisions: option (a), idle drain not budgeted inside `P`, health server reserved first; readiness state shape left to the design review
- 2026-11-03 16:50 UTC - AI agent (model not stated by the runtime) - Added the template sections (plan, commit points, verification plan, risks) required by the `create-issue` skill
- 2026-11-03 16:51 UTC - AI agent (model not stated by the runtime) - Created issue #2697, linked it under #1488 and moved the spec to `docs/issues/open/`
- 2026-11-03 17:02 UTC - AI agent (model not stated by the runtime) - Opened the specification-only PR #2698 (`Related to #2697`); CI: Docs Lint passed, no reviews or comments yet
- 2026-11-03 17:44 UTC - AI agent (model not stated by the runtime) - `review-finding:pr-2698-f2`: replaced the stale clause saying the maintainer had not yet approved creating the issue with the approval that already happened on 2026-11-03
- 2026-11-04 12:25 UTC - AI agent (Claude Sonnet 5.5) - Recorded the maintainer's design answers, implemented T1-T6 on `2697-mark-health-unhealthy-during-shutdown` (readiness type, 503 path, stop-last edge, `begin_shutdown`, docs, ADR) and recorded real-binary evidence V3-V8; awaiting maintainer review before any PR
- 2026-11-04 17:38 UTC - AI agent (Claude Sonnet 5.5) - Implementation PR #2720 merged and issue #2697 closed on GitHub; archived the spec to `docs/issues/closed/`, set `status: done` and marked EPIC #1488 roadmap row 21 Done. The acceptance criteria stay unticked for the maintainer.

## Acceptance Criteria

- [ ] Application readiness defaults to ready only after successful startup.
- [ ] Normal shutdown sets readiness to not ready before `jobs.cancel()` and,
      under the chosen design, a NEW connection to the health-check API during
      the drain receives HTTP 503 (not a refused connection).
- [ ] `/health_check` returns HTTP 503 while not ready and does not execute
      downstream registered-service probes.
- [ ] Healthy `/health_check` behavior and response body remain unchanged before
      shutdown initiation.
- [ ] Readiness transition is one-way and safe under concurrent health requests.
- [ ] Deterministic tests cover ready response, not-ready 503, no probe fan-out,
      and ordering before cancellation without OS signals.
- [ ] The health-check component stops after every other supervised component
      (a test asserts each has it as a successor), and a failed, panicked or
      aborted component still yields exit code 1 within `P`.
- [ ] Focused manual verification records readiness becoming 503 before the
      relevant component drain logs and before process exit, with timestamped
      probes on new connections, for SIGTERM and SIGINT.
- [ ] `linter all` passes.
- [ ] Relevant tests pass.
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`.
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior.
- [ ] Documentation is updated for the readiness contract.

All done on `develop`:

- SI-13 health-check API token lifecycle migration (#2324).
- Supervisor shutdown initiation and SIGTERM handling (#1586, SI-1).
- SI-22 event-flow ordering (#2410) and its ADR.
- SI-20 configured process deadline and drain budgets (#2668).

This task adds no timer or separate deadline.

## Rollback

Before release and operational adoption, revert only the readiness state and
503 response path. The health-check API continues to use its token lifecycle and
all component shutdown behavior remains unchanged. After release, preserve the
published 503 readiness contract and repair defects through a compatible patch
rather than silently restoring always-probe behavior.

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test --doc --workspace` and the focused tests of the changed packages
- Pre-push checks before the implementation push

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

Follow the EPIC executable-boundary protocol: signal the direct tracker binary
PID, capture bounded completion and exit status, and prove readiness changes
before affected bindings are released.

| ID  | Scenario                                    | Human-oriented command/steps                                                                                                                                   | Expected Result                                                                                                  | Status | Evidence                                          |
| --- | ------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------- | ------ | ------------------------------------------------- |
| M1  | Baseline before the change                  | Hold a partial request on the REST API, send SIGTERM and SIGINT to the PID, poll `/health_check` with new connections (done during drafting).                  | No 503 on new connections: refused or reset.                                                                     | DONE   | `manual-verification-evidence.md` sections V1, V2 |
| M2  | Deterministic readiness tests               | Toggle readiness in handler and application tests without an OS signal.                                                                                        | Not ready returns 503 with no probe fan-out; ready returns the unchanged body.                                   | DONE   | `manual-verification-evidence.md` section V3      |
| M3  | Drain with SIGTERM and SIGINT on the binary | Same holder as M1, after the change; poll `/health_check` on new and open connections with timestamps; record exit status and that the listeners are released. | New connections get 503 for the whole drain, before the drain-complete and exit logs; exit status as before.     | DONE   | `manual-verification-evidence.md` section V4      |
| M4  | Healthy tracker unchanged                   | `curl` `/health_check` on a running tracker.                                                                                                                   | The existing health response.                                                                                    | DONE   | `manual-verification-evidence.md` section V5      |
| M5  | Failure path                                | Hold the drain open past the configured drain so a component fails or is aborted at `P`.                                                                       | Exit 1 within `P`; the failed outcome is reported; the health server does not delay the exit past `P`.          | DONE   | `manual-verification-evidence.md` section V6      |
| M6  | Operator documentation                      | Read the deployment docs.                                                                                                                                      | Readiness probes use `/health_check` and not-ready instances leave the traffic.                                  | DONE   | `manual-verification-evidence.md` section V7      |

Record every command result with its toolchain or runtime when it can affect
behaviour. If a scenario fails, record the failure and diagnosis in the progress
log before proceeding.

### Disposable Verification Scripts

None in this specification-only change: the baseline probes used a throwaway
script outside the repository, and its results are embedded in the evidence.
If the implementation needs a retained probe script, its rationale, issue-local
path, removal owner and, for Python, why Rust is unsuitable are recorded here
and the script is added only in the implementation pull request.

### Acceptance Verification

| AC ID                                  | Status | Evidence                                |
| -------------------------------------- | ------ | --------------------------------------- |
| Every criterion in Acceptance Criteria | TODO   | Filled in after implementation review. |

## Risks and Trade-offs

- A hung component keeps the health server serving 503 until `P`; the exit stays
  bounded by `P` and reports exit 1. Accepted in the design decision.
- The health server's own drain is not covered by `max(H, U) + L < P`; it is
  idle and near-instant. Accepted; the real-binary evidence measures it.
- A component added later without the stop-after edge would stop concurrently
  with the health server; T3 adds a test for it.
- A published public API change needs a version bump under the independent
  versioning ADR; decided at the design review.

## Implementation Completion Review

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from the repository
  template at `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this issue
  specification's directory; otherwise add a progress-log entry saying why none
  was needed.
- The independent reviewer records its result in `agent-review-reports.md`
  using `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- Parent EPIC: #1488 (roadmap row 21)
- Related issues: #2324 (SI-13), #2410 (SI-22), #2668 (SI-20)
- Related ADRs: `docs/adrs/20261021184946_order_shutdown_by_event_flow_and_drain_event_listeners.md`
- Q6: `docs/features/shutdown-process/questions.md`
