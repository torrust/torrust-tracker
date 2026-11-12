---
schema-version: 1
doc-type: issue
issue-type: task
status: planned
priority: p3
epic: 1488
github-issue: 2845
spec-path: docs/issues/open/2845-1488-si-26-consolidate-shutdown-supervisor/ISSUE.md
branch: "2845-consolidate-shutdown-supervisor"
related-pr: null
last-updated-utc: "2026-11-12 14:30"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - src/bootstrap/jobs/manager.rs
    - src/bootstrap/jobs/http_tracker.rs
    - src/bootstrap/jobs/tracker_apis.rs
    - src/bootstrap/jobs/health_check_api.rs
    - src/bootstrap/jobs/udp_tracker.rs
    - src/bootstrap/jobs/udp_tracker_server.rs
    - src/app.rs
    - packages/udp-server/src/server/launcher.rs
    - docs/adrs/20260902074438_adopt_supervised_cancellation_tree_for_shutdown.md
    - docs/adrs/20261021184946_order_shutdown_by_event_flow_and_drain_event_listeners.md
    - docs/adrs/20261104120915_stop_the_health_check_api_last_to_report_not_ready_during_shutdown.md
    - docs/architecture/application-jobs.md
    - docs/issues/closed/2801-1488-si-19-remove-legacy-shutdown-api/ISSUE.md
    - docs/issues/closed/2697-1488-si-21-mark-health-unhealthy-during-shutdown/ISSUE.md
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
---

<!-- skill-link: create-issue -->

# Issue #2845 - Consolidate the Shutdown Supervisors and Migrate the Last Legacy Job (SI-26)

> **EPIC position**: Roadmap row 23, sub-issue SI-26 of [EPIC #1488](../1488-overhaul-tracker-shutdown/ISSUE.md).
> A follow-up of [SI-19](../../closed/2801-1488-si-19-remove-legacy-shutdown-api/ISSUE.md) and
> [SI-21](../../closed/2697-1488-si-21-mark-health-unhealthy-during-shutdown/ISSUE.md).
> Implementation order 2 of 4. Priority p3; must land before the first publish (#2583, see Dependencies).

## Goal

Finish the supervised cancellation tree: one shared component supervisor for the token-aware servers,
a reviewed set of owner types, and no component left on the legacy root-token path. After this task
every job is a reserved component, so the stop-last edge of the health check (SI-21) and the
event-flow edges (SI-22) cover all of them.

## Background

Checked on 2026-11-12 against `torrust/develop` at `388faf59a`. SI-19 split three items out of its
scope (it would have widened the breaking surface), and SI-21 left a fourth:

1. **Four near-duplicate supervisors.** `supervise_token_aware_server` exists three times
   (`src/bootstrap/jobs/http_tracker.rs`, `tracker_apis.rs`, `health_check_api.rs`) and
   `supervise_receive_loop` once (`udp_tracker.rs`). Each races the cancellation token against the
   server task, joins the owned children and maps the result to a `ComponentResult`. They differ
   only in labels and in the join order below.
2. **Join order.** The health-check supervisor joins the server task, then **always** joins the drain
   controller, and only then maps the errors, so a failed server join never leaves the controller
   unjoined and the drain outcome is not lost. The HTTP and REST supervisors map a server-join error
   with `?` first, so on that error the controller is dropped (aborted) without being joined and its
   outcome is lost.
3. **Owner types.** `TokenAwareServerTask<T, U>` and `OwnedTask<T>` (public, in
   `src/bootstrap/jobs/manager.rs`) abort their children on drop. `OwnedReceiveLoop` is a private
   twin in `packages/udp-server/src/server/launcher.rs` (`pub(crate)`, hands the task on with
   `into_task`). The package cannot use the root types, because the dependency runs the other way.
4. **The last legacy job.** `udp_ban_cleanup` is the only `JobManager::register_legacy` user
   (`src/app.rs`, `start_udp_ban_cleanup_job`). It keeps the manager's root token
   (`legacy_cancellation_token`), has no component id, and so is not covered by the stop-last edge:
   at shutdown the health check may stop before it. The ADR of SI-21 documents this as accepted, and
   the event-flow ADR keeps `register_legacy` "until `udp_ban_cleanup` migrates". The job holds no
   listener, serves no traffic and publishes nothing, so the practical impact is small; it is the
   reason the legacy API still exists (about 48 references in `manager.rs`).

## Scope

### In Scope

- One generic token-aware component supervisor in `src/bootstrap/jobs/`, used by the HTTP tracker,
  REST API and health-check API components. It adopts the health-check join order: join the server,
  join the drain controller, then map errors, on both the cancelled branch and the unexpected-stop
  branch. The per-component names appear only in the error messages and logs, which keep today's
  wording.
- A reviewed decision on the owner types, recorded in the spec before the code change (T1): keep
  `OwnedTask` and `TokenAwareServerTask` in the root; for `OwnedReceiveLoop`, either keep it as the
  package-local type it is, or unify it with `OwnedTask`, with the reasons. Unification must not add
  a dependency from a package to the root crate.
- Migrate `udp_ban_cleanup` to a reserved component: `reserve` a component id, `spawn` its runner
  with the reservation token and a `ComponentResult`, then remove `register_legacy`, `LegacyJob`,
  `legacy_cancellation_token`, the root-token handling in `cancel` and the legacy-outcome path in
  `wait_for_all`, with their tests. The root token stays only as the manager's internal handle if
  something still needs it; otherwise it goes too.
- Edge for the migrated job: it stops before the health check (the stop-last edge covers it) and has
  no event-flow edge, because it publishes nothing (`docs/architecture/events.md`); T1 re-checks that
  against `reset_bans` and `PublicationOrder`, and the job declares the edge if it publishes.
- Update the ADRs, `docs/architecture/application-jobs.md`, `docs/architecture/glossary.md`, the
  shutdown-process docs and the SI-21 ADR note that the job is not covered, and keep the code and
  `// issue: #1488` comments consistent.

### Out of Scope

- Changing cancellation-tree behavior, the stop order of the existing components, timeout policy,
  exit-code policy or readiness behavior. The only order change is that `udp_ban_cleanup` joins the
  stop-last edge.
- Removing the unused error variants (SI-25), removing services from the `Registar` as they stop (SI-27),
  and the real-binary deadline-abort evidence (SI-28).
- The periodic jobs that already use `reserve` and `spawn` (torrent cleanup, activity metrics).
- Public API of the unpublished packages: `OwnedReceiveLoop` is `pub(crate)`, so a change to it is
  not breaking.

## Dependencies

- SI-19 (#2801) and SI-21 (#2697) are done and merged.
- Independent of SI-25 and SI-27 (no shared file apart from docs); implementation order puts SI-25 first.
- SI-28 depends on this task: its evidence should cover the final supervisor.
- Maintainer decision of 2026-11-12: SI-25, SI-26 and SI-27 land before the first publish of the
  workspace crates (#2583). #2583 is blocked on EPIC #1488 as a whole.

## Architectural Decisions

- Related ADRs: `docs/adrs/20260902074438_adopt_supervised_cancellation_tree_for_shutdown.md`,
  `docs/adrs/20261021184946_order_shutdown_by_event_flow_and_drain_event_listeners.md` (the
  `register_legacy` note), and
  `docs/adrs/20261104120915_stop_the_health_check_api_last_to_report_not_ready_during_shutdown.md`
  (the `udp_ban_cleanup` note).
- The owner-type decision (T1) is recorded in this spec. An ADR is created only if the review changes
  an accepted ADR rather than updating the two notes above.

## Design and Ownership Review

Responsibility and ownership map (unchanged by this task, listed so the review can confirm it):

| Resource | Owner | Joined or aborted by |
| -------- | ----- | -------------------- |
| Server task and drain controller of an Axum component | `TokenAwareServerTask`, held by the component future | The shared supervisor joins both; its drop aborts both. |
| UDP receive loop | `OwnedTask` (from `OwnedReceiveLoop` until handed on) | `supervise_receive_loop` joins it; drop aborts it. |
| Ban-cleanup task | The component future that `spawn` wraps (new) | The manager joins the component; the deadline aborts it. |

Invariants to keep, in the normal and the failure and drop paths: a component never reports an outcome
before its owned children finished or were deliberately aborted; a dropped owner aborts its children;
every awaited operation is bounded by the process deadline `P`. A design-review checkpoint follows the
first passing vertical slice (the shared supervisor with the health check migrated).

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| -- | ------ | ---- | ----------------------- |
| T1 | TODO | Design note: diff the three supervisors, decide the owner types, re-check that `udp_ban_cleanup` publishes nothing. Present it to the maintainer before coding. | A short section in this spec; the maintainer's decision is recorded. |
| T2 | TODO | Add the shared supervisor with tests (a table of the join and error cases, including a server-join error that must still join the controller), migrate the health check first, then HTTP and REST. | Existing component tests keep passing unchanged; the controller-join case is new. Design-review checkpoint. |
| T3 | TODO | Apply the owner-type decision. | Code and tests per T1. |
| T4 | TODO | Migrate `udp_ban_cleanup` to `reserve` and `spawn`; remove the legacy registration path from `JobManager`. | The stop-order test in `src/app.rs` asserts the job stops before `health_check_api`; no `register_legacy` or `legacy_cancellation_token` remains. |
| T5 | TODO | Update the ADRs, architecture and shutdown-process docs. | `git grep` finds no live mention of the legacy job. |
| T6 | TODO | Verify and record evidence. | `manual-verification-evidence.md`. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T1 | None (design note in the spec) | Commit the spec edit after the maintainer decides. |
| T2 | Shared supervisor and its three call sites | Commit after focused validation and review. |
| T3 | Owner types | Commit after focused validation. |
| T4 | `udp_ban_cleanup` migration and legacy path removal | Commit after focused validation and review. |
| T5 | Documentation | Commit after the last production change. |
| T6 | Verification evidence | Commit after the last production change. |

Sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/`, moved to `docs/issues/open/`
- [x] Spec reviewed and approved by the maintainer (decisions of 2026-11-12, in chat)
- [x] GitHub issue created and issue number added to this spec (#2845)
- [ ] Spec-only PR merged into `develop` before implementation
- [ ] Owner-type design decision recorded (T1)
- [ ] Implementation completed
- [ ] Design review after the first passing vertical slice recorded
- [ ] Automatic verification completed (`linter all`, relevant tests, pre-push checks)
- [ ] Manual verification scenarios executed and recorded in `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-11-11 09:40 UTC - AI agent (Claude Sonnet 5.5) - Draft created as a follow-up of SI-19; registered as SI-26 of EPIC 1488 at 16:30 UTC.
- 2026-11-12 14:30 UTC - AI agent (Claude Sonnet 5.5) - Refreshed against `torrust/develop` at `388faf59a`: replaced the vague scope with the four findings above, folded in the SI-21 follow-up (the legacy `udp_ban_cleanup` job is not ordered by the stop-last edge), and recorded the maintainer's decisions of 2026-11-12 (SI-26 absorbs the migration; lands before #2583).

## Acceptance Criteria

- [ ] One shared supervisor serves the HTTP tracker, REST API and health-check API components, with the
      health-check join order, and a test proves the drain controller is joined when the server join fails.
- [ ] The owner-type decision is recorded and applied.
- [ ] `udp_ban_cleanup` is a reserved component, stops before the health check, and a test proves it.
- [ ] No `register_legacy`, `LegacyJob` or `legacy_cancellation_token` remains, and no live document
      describes the legacy path.
- [ ] Shutdown behavior is otherwise unchanged: same stop order, same exit codes, same log wording for
      the existing components.
- [ ] `linter all` passes.

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo check --workspace --all-targets` warning-free; `cargo test --workspace` and the focused tests of
  `src/bootstrap/jobs` and `src/app.rs`
- Pre-push checks before the implementation push

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

Executable-boundary verification follows the EPIC protocol (signal the actual binary PID, bounded wait,
exit status and shutdown output, then prove the bindings are released).

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| -- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1 | Legacy path gone | `git grep` for `register_legacy`, `legacy_cancellation_token`, `LegacyJob`. | No match outside archived specs and history. | TODO | `manual-verification-evidence.md` |
| M2 | Orderly shutdown, all services | Start the tracker with HTTP, REST, UDP, health check and a UDP ban reset interval; send `SIGTERM`, then `SIGINT` in a second run. | Exit code 0; the log shows `udp_ban_cleanup` stopped before `health_check_api` was released; the stop order of the other components is unchanged; ports released. | TODO | `manual-verification-evidence.md` |
| M3 | Failure path | Hold a REST request open past its drain so a component fails (the SI-21 V6 recipe). | Exit code 1 within `P`; the failed outcome is reported as before; the health server does not delay the exit past `P`. | TODO | `manual-verification-evidence.md` |
| M4 | Ban cleanup still resets bans | Run with a short `ip_bans_reset_interval_in_secs`, ban an IP, wait one interval. | The ban is reset and the metric updates, as before the migration. | TODO | `manual-verification-evidence.md` |

### Disposable Verification Scripts

None. Reuse the signalling script of SI-21 if it is still in its evidence folder.

### Acceptance Verification

| AC | Status | Evidence |
| -- | ------ | -------- |
| Shared supervisor, join order, controller-join test | TODO | Automatic checks, M2 |
| Owner-type decision recorded and applied | TODO | T1, T3 |
| `udp_ban_cleanup` is a component that stops before the health check | TODO | M2 |
| Legacy path removed | TODO | M1 |
| Behavior otherwise unchanged | TODO | M2, M3 |
| `linter all` passes | TODO | Automatic checks |

## Risks and Trade-offs

- The supervisors look identical but differ in details (error text, the `Result` inside the server
  task). A merge that loses a difference changes a log line or an outcome; T1 lists the differences and
  T2 keeps the existing tests unchanged to catch it.
- Joining the controller even after a server-join error can make a failed component take longer to
  report (it waits for the drain). The wait is bounded by the drain timeout and by `P`.
- Migrating `udp_ban_cleanup` adds it to the stop-last edge: the health check now also waits for it.
  The job exits on cancellation within one select iteration, so the cost is negligible; a hung job
  would keep the health check serving 503 until `P`, the same as any other component.
- If the maintainer decides in T1 to keep `OwnedReceiveLoop` as it is, the task still removes the
  duplicate supervisors and the legacy path; that is not a partial failure.

## Implementation Completion Review

Before closing, review what the implementation discovered. If it found anything material (a difference
between the supervisors that mattered, a design change from T1), create
`implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this
directory; otherwise add a progress-log entry saying why none was needed.

## References

- Parent EPIC: #1488 (roadmap row 23)
- Related: #2801 (SI-19, scope split), #2697 (SI-21, the follow-up), #1453 (ban-cleanup job origin),
  #2583 (first publish)

## Rollback

Revert the commits in reverse order. T2 and T3 are internal refactors with unchanged behavior; T4 is
reverted by restoring the legacy registration. Nothing is published, so there is no release to
withdraw. If T4 alone proves wrong, revert it and keep T2 and T3.
