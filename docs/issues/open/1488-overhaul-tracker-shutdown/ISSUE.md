---
doc-type: epic
status: open
github-issue: 1488
spec-path: docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
epic-owner: josecelano
last-updated-utc: 2026-09-01
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/analysis/20260716-shutdown-process/README.md
    - docs/features/shutdown-process/README.md
    - docs/features/shutdown-process/task-inventory.md
    - docs/features/shutdown-process/shutdown-architecture-examples.md
    - src/main.rs
    - src/bootstrap/jobs/manager.rs
    - src/bootstrap/jobs/torrent_cleanup.rs
    - src/bootstrap/jobs/activity_metrics_updater.rs
    - packages/axum-server/src/signals.rs
    - packages/udp-server/src/server/launcher.rs
    - packages/swarm-coordination-registry/src/statistics/activity_metrics_updater.rs
    - docs/research/20260716-console-shutdown-patterns/README.md
---

<!-- skill-link: create-issue -->

# EPIC #1488 - Overhaul: Tracker Shutdown

## Goal

Bring the Torrust Tracker into compliance with the **Unix and container process
lifecycle contracts** — the well-proven standards that govern how a long-running
service is expected to stop. Then, as a second step, normalize and clean up the
internal shutdown implementation so all jobs follow a single consistent pattern.

The primary goal is **correctness and lack of surprise**: every standard stop
mechanism (`kill`, `docker stop`, `systemctl stop`, Kubernetes pod termination)
should trigger a graceful shutdown, exactly as any operator or automation tool
would expect.

The secondary goal is **internal consistency**: use a supervised cancellation
tree. `JobManager` supervises named top-level components; cancellation flows
from its root token to component child tokens; each component joins or
deliberately aborts its own children before reporting its outcome upward.

## Why This Is Needed

The current shutdown process has several problems identified in the
[shutdown analysis](../../../analysis/20260716-shutdown-process/README.md):

1. **No `SIGTERM` in `main.rs`** — only `SIGINT` (Ctrl+C) is handled at the top
   level. Container orchestrators (Docker/Podman) send `SIGTERM` by default,
   which means `jobs.cancel()` and `jobs.wait_for_all()` are never called.
2. **Three inconsistent shutdown mechanisms** — jobs use `CancellationToken`,
   direct `tokio::signal::ctrl_c()`, or oneshot `Halted` channels. Some jobs
   ignore the central `JobManager` entirely.
3. **Torrent cleanup and activity metrics ignore `CancellationToken`** — they
   listen for `ctrl_c` directly instead of using the shared token.
4. **Grace period mismatch** — `JobManager` waits 10s per job sequentially,
   while Axum servers have a 90s graceful shutdown. The main process may exit
   before servers finish draining connections.
5. **No graceful UDP shutdown** — the UDP server simply aborts its main loop.
6. **Hardcoded timeouts** — grace periods are magic numbers with no configuration
   surface.
7. **No observable shutdown progress** — operators cannot tell which job is
   blocking shutdown.
8. **Double-signal on Ctrl+C** — both `main.rs` and each server's
   `global_shutdown_signal()` catch the same signal, creating a potential race.

## The Contracts Being Implemented

These are not new features — they are standard behaviors that every process
manager, container runtime, and operator already expects:

```bash
# These all SHOULD work — and currently DON'T (except Ctrl+C):
kill <pid>               # SIGTERM — currently ignored ❌
docker stop <container>  # SIGTERM — currently ignored ❌
systemctl stop <service> # SIGTERM — currently ignored ❌
# Kubernetes pod delete    # SIGTERM — currently ignored ❌

# This works but is non-standard:
kill -INT <pid>          # SIGINT — works ✅

# This should be the last resort, never needed in normal operation:
kill -9 <pid>            # SIGKILL — force kill ❌
```

Adding a `SIGTERM` handler in `main.rs` is the single most impactful change in
this EPIC — it fixes all four broken cases above with a few lines of code.

## Background

This EPIC was originally created after closing issue #1477 ("Fix shutdown message
and improve it"), which introduced the `JobManager` type and centralized job
management. The current EPIC builds on that foundation to complete the
centralization and address remaining gaps.

Issue #1588 ("Review shutdown process for all tasks/jobs") is the first sub-issue
and identified the remaining jobs that still handle `ctrl_c` directly.

## Scope

### In Scope

- Centralize signal handling in `main.rs` (both `SIGINT` and `SIGTERM`).
- Replace shutdown `Halted` oneshot channels with `CancellationToken` propagation;
  retain the separate `Started` oneshot for startup reporting.
- Require every component to own and join, or deliberately abort, its child
  tasks before its top-level task completes.
- Migrate torrent cleanup and activity metrics updater to use `CancellationToken`.
- Configurable grace periods (add `[shutdown]` configuration section).
- Observable shutdown progress (which jobs are still running).
- Grace period alignment between `JobManager` and server-level shutdown.
- Review and align the Axum `graceful_shutdown` timeout with the `JobManager` timeout.
- UDP server shutdown improvements (drain or at least log in-flight work).

### Out of Scope

- Hot-reload / restart without process exit.
- `SIGHUP` configuration reload. Configuration changes require a normal graceful
  restart; dynamic reload is deferred to a separate future feature.
- Dynamic job lifecycle (start/stop jobs at runtime via admin API).
- Windows-specific signal handling beyond what Tokio provides.
- The **profiling binary** (`src/console/profiling.rs`) — it is a developer-only
  tool for profiling (valgrind/callgrind), not a user-facing entry point. It can
  be updated independently as needed.

## Implementation Roadmap

This catalog lists every shutdown draft and existing GitHub child issue by its
immutable identifier. The
**execution sequence** deliberately differs from the SI number: replacement
drafts SI-10 through SI-20 were added after SI-1 through SI-9 had already been
named. A blank sequence means the draft is superseded and must not be
implemented.

No task may remove a shutdown path used by an existing supported consumer.
Shared lifecycle APIs follow this sequence: **add → migrate every consumer →
deprecate → remove**. Each component migration is a vertical slice:
cancellation request, owned-child completion policy, named outcome,
deterministic tests, and manual evidence.

| Sequence | Draft | Work item                                                                                                                        | Status     | Independently releasable scope                                                         |
| -------- | ----- | -------------------------------------------------------------------------------------------------------------------------------- | ---------- | -------------------------------------------------------------------------------------- |
| 0        | #1588 | [Revalidate task inventory](../1588-review-shutdown-process-for-all-tasks-jobs/ISSUE.md)                                         | Open       | Implementation-time inventory and ownership evidence; no runtime behavior changes.     |
| 1        | SI-1  | [Add `SIGTERM` at `main()`](../../drafts/1488-si-1-add-sigterm-to-main/ISSUE.md)                                                 | Draft      | Incremental signal-boundary compatibility fix.                                         |
| 2        | #1586 | [Evaluate `JoinSet` for `JobManager`](../1586-evaluate-job-manager-join-set/ISSUE.md)                                            | Open       | Direct supervisor task ownership, concurrent outcomes, and explicit escalation policy. |
| 3        | SI-4  | [Migrate torrent cleanup](../../drafts/1488-si-4-migrate-torrent-cleanup/ISSUE.md)                                               | Draft      | One periodic component adopts token cancellation.                                      |
| 4        | SI-5  | [Migrate activity metrics](../../drafts/1488-si-5-migrate-activity-metrics-updater/ISSUE.md)                                     | Draft      | One periodic component adopts token cancellation.                                      |
| 5        | SI-2  | [Add token-aware server lifecycle API](../../drafts/1488-si-2-remove-global-shutdown-signal/ISSUE.md)                            | Draft      | Additive `torrust-server-lib` API; retain legacy shutdown compatibility.               |
| 6        | SI-10 | [Add token-aware, joinable Axum drain helper](../../drafts/1488-si-10-add-token-aware-axum-drain-helper/ISSUE.md)                | Draft      | Additive helper alongside existing API; no consumer breaks.                            |
| 7        | SI-11 | [Migrate HTTP tracker to token lifecycle](../../drafts/1488-si-11-migrate-http-tracker-token-lifecycle/ISSUE.md)                 | Draft      | One complete HTTP vertical slice.                                                      |
| 8        | SI-12 | [Migrate REST API to token lifecycle](../../drafts/1488-si-12-migrate-rest-api-token-lifecycle/ISSUE.md)                         | Draft      | One complete REST API vertical slice.                                                  |
| 9        | SI-13 | [Migrate health-check API to token lifecycle](../../drafts/1488-si-13-migrate-health-check-api-token-lifecycle/ISSUE.md)         | Draft      | One health-check vertical slice; SI-21 separately implements readiness-before-drain.   |
| 10       | SI-14 | [Migrate UDP receive/reset tasks to token lifecycle](../../drafts/1488-si-14-migrate-udp-receive-reset-token-lifecycle/ISSUE.md) | Draft      | Token-aware UDP stop; join receive/reset; retain request abort fallback.               |
| 11       | SI-15 | [Define UDP active-request shutdown policy](../../drafts/1488-si-15-define-udp-active-request-policy/ISSUE.md)                   | Draft      | Request deadline, abort behavior, outcomes, and verification.                          |
| 12       | SI-16 | [Migrate standalone HTTP environment/example](../../drafts/1488-si-16-migrate-standalone-http-environment/ISSUE.md)              | Draft      | One supported standalone HTTP consumer migration.                                      |
| 13       | SI-17 | [Migrate standalone UDP environment/example](../../drafts/1488-si-17-migrate-standalone-udp-environment/ISSUE.md)                | Draft      | One supported standalone UDP consumer migration.                                       |
| 14       | SI-18 | [Deprecate legacy shutdown API](../../drafts/1488-si-18-deprecate-legacy-shutdown-api/ISSUE.md)                                  | Draft      | Compatibility-preserving source deprecation only.                                      |
| 15       | SI-19 | [Remove legacy shutdown API and library OS signals](../../drafts/1488-si-19-remove-legacy-shutdown-api/ISSUE.md)                 | Draft      | Breaking release after migration, deprecation, and compatibility gates.                |
| 16       | SI-20 | [Configure shutdown policy and deployment contract](../../drafts/1488-si-20-configure-shutdown-policy/ISSUE.md)                  | Draft      | Apply approved Q3/Q4 outcomes, budgets, configuration, and deployment guidance.        |
| 17       | SI-21 | [Mark health check unhealthy during shutdown](../../drafts/1488-si-21-mark-health-unhealthy-during-shutdown/ISSUE.md)            | Draft      | Set readiness to not ready before root cancellation and component drain.               |
| —        | SI-3  | [Combined standalone environment migration](../../drafts/1488-si-3-fix-environment-stop/ISSUE.md)                                | Superseded | Replaced by SI-16 and SI-17. Do not implement.                                         |
| —        | SI-6  | [Concurrent supervisor outcomes](../../drafts/1488-si-6-align-grace-periods/ISSUE.md)                                            | Superseded | Replaced by existing issue #1586. Do not implement separately.                         |
| —        | SI-7  | [Standalone shutdown-progress reporting](../../drafts/1488-si-7-observable-shutdown-progress/ISSUE.md)                           | Superseded | Structured outcomes are incorporated into issue #1586. Do not implement.               |
| —        | SI-8  | [Original shutdown configuration](../../drafts/1488-si-8-configurable-grace-periods/ISSUE.md)                                    | Superseded | Replaced by SI-20 after Q3/Q4 decisions. Do not implement.                             |
| —        | SI-9  | [Combined UDP shutdown migration](../../drafts/1488-si-9-improve-udp-shutdown/ISSUE.md)                                          | Superseded | Replaced by SI-14 and SI-15. Do not implement.                                         |

### Release and Review Requirements

- Each work item must preserve a supported shutdown path for the tracker and
  every affected standalone consumer.
- A shared API task must be additive until all consumers migrate; no API removal
  may be bundled with the first consumer migration.
- Each component task must demonstrate top-down cancellation and bottom-up,
  owner-joined completion without exposing nested task handles to `JobManager`.
- Each task requires deterministic token/lifecycle tests. OS signals and
  Docker/Podman behavior are end-to-end verification, not unit-test mechanisms.
- Each task must have a documented rollback/revert story: revert the component
  migration while the legacy API remains available, or revert an additive API
  without changing existing consumers.

## Dependencies

- **#1405** (Overhaul stats: graceful shutdown for broadcast channels) — ✅ Closed.
  Implemented with `CancellationToken`, which is the foundation for this EPIC.
- **#1477** (Fix shutdown message and improve it) — ✅ Closed.
  Introduced the `JobManager` type.
- **#1586** (Evaluate `JoinSet` for `JobManager`) — Open.
  Roadmap sequence 2; direct supervisor task ownership and outcome handling.
- **#1588** (Review shutdown process for all tasks/jobs) — Open.
  Roadmap sequence 0; final implementation-time task inventory and ownership
  evidence.

## Related Documents

- [Analysis: Shutdown Process](../../../analysis/20260716-shutdown-process/README.md) — detailed code-level analysis
- [Feature: Shutdown Process](../../../features/shutdown-process/README.md) — product-oriented feature description
- [Questions and Decisions](../../../features/shutdown-process/questions.md) — resolved specification decisions and risks
