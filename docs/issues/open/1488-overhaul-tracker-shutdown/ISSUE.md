---
doc-type: epic
status: open
github-issue: 1488
spec-path: docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
epic-owner: josecelano
last-updated-utc: 2026-07-16
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/analysis/20260716-shutdown-process/README.md
    - docs/features/shutdown-process/README.md
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

The secondary goal is **internal consistency**: standardize all jobs to use the
`CancellationToken` from `JobManager`, remove direct `ctrl_c` listeners from
individual jobs, and align timeouts so shutdown actually completes cleanly.

## Why This Is Needed

The current shutdown process has several problems identified in the
[shutdown analysis](../../analysis/20260716-shutdown-process/README.md):

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
- Consistent shutdown mechanism for all jobs (prefer `CancellationToken`).
- Migrate torrent cleanup and activity metrics updater to use `CancellationToken`.
- Configurable grace periods (add `[shutdown]` configuration section).
- Observable shutdown progress (which jobs are still running).
- Grace period alignment between `JobManager` and server-level shutdown.
- Review and align the Axum `graceful_shutdown` timeout with the `JobManager` timeout.
- UDP server shutdown improvements (drain or at least log in-flight work).

### Out of Scope

- Hot-reload / restart without process exit.
- Dynamic job lifecycle (start/stop jobs at runtime via admin API).
- Windows-specific signal handling beyond what Tokio provides.
- The **profiling binary** (`src/console/profiling.rs`) — it is a developer-only
  tool for profiling (valgrind/callgrind), not a user-facing entry point. It can
  be updated independently as needed.

## Sub-issues

Items marked **Blocked** depend on an open question in
[open-questions.md](../../features/shutdown-process/open-questions.md).
Spec files for draft items live under `docs/issues/drafts/`.

| #     | Title                                                        | Spec                                                                              | Status | Notes                                         |
| ----- | ------------------------------------------------------------ | --------------------------------------------------------------------------------- | ------ | --------------------------------------------- |
| #1588 | Review shutdown process for all tasks/jobs                   | [1588/ISSUE.md](../1588-review-shutdown-process-for-all-tasks-jobs/ISSUE.md)      | Open   | Pre-existing; inventory and gap analysis      |
| SI-1  | Add `SIGTERM` handler to `main.rs`                           | [SI-1/ISSUE.md](../../drafts/1488-si-1-add-sigterm-to-main/ISSUE.md)              | Draft  | Highest priority; fixes the Unix contract     |
| SI-2  | Remove `global_shutdown_signal()` from per-server shutdown   | [SI-2/ISSUE.md](../../drafts/1488-si-2-remove-global-shutdown-signal/ISSUE.md)    | Draft  | Blocked: Q1, Q5; touches `torrust-server-lib` |
| SI-3  | Fix `Environment::stop()` in standalone library examples     | [SI-3/ISSUE.md](../../drafts/1488-si-3-fix-environment-stop/ISSUE.md)             | Draft  | Blocked: Q1; abort vs cancel + SIGTERM        |
| SI-4  | Migrate torrent cleanup to `CancellationToken`               | [SI-4/ISSUE.md](../../drafts/1488-si-4-migrate-torrent-cleanup/ISSUE.md)          | Draft  | Removes direct `ctrl_c` listener              |
| SI-5  | Migrate activity metrics updater to `CancellationToken`      | [SI-5/ISSUE.md](../../drafts/1488-si-5-migrate-activity-metrics-updater/ISSUE.md) | Draft  | Removes direct `ctrl_c` listener              |
| SI-6  | Align `JobManager` grace period with Axum server timeout     | [SI-6/ISSUE.md](../../drafts/1488-si-6-align-grace-periods/ISSUE.md)              | Draft  | Blocked: Q4; fixes the 10s vs 90s mismatch    |
| SI-7  | Implement observable shutdown progress in `JobManager`       | [SI-7/ISSUE.md](../../drafts/1488-si-7-observable-shutdown-progress/ISSUE.md)     | Draft  |                                               |
| SI-8  | Add configurable grace periods (`[shutdown]` config section) | [SI-8/ISSUE.md](../../drafts/1488-si-8-configurable-grace-periods/ISSUE.md)       | Draft  | Blocked: Q3, Q4                               |
| SI-9  | Improve UDP server shutdown                                  | [SI-9/ISSUE.md](../../drafts/1488-si-9-improve-udp-shutdown/ISSUE.md)             | Draft  | Low priority; UDP is stateless by nature      |

## Dependencies

- **#1405** (Overhaul stats: graceful shutdown for broadcast channels) — ✅ Closed.
  Implemented with `CancellationToken`, which is the foundation for this EPIC.
- **#1477** (Fix shutdown message and improve it) — ✅ Closed.
  Introduced the `JobManager` type.

## Related Documents

- [Analysis: Shutdown Process](../../analysis/20260716-shutdown-process/README.md) — detailed code-level analysis
- [Feature: Shutdown Process](../../features/shutdown-process/README.md) — product-oriented feature description
- [Open Questions](../../features/shutdown-process/open-questions.md) — gaps and risks to resolve before implementation
