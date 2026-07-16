---
doc-type: issue
issue-type: task
status: open
priority: p2
github-issue: 1588
spec-path: docs/issues/open/1588-review-shutdown-process-for-all-tasks-jobs/ISSUE.md
branch: "1588-review-shutdown-process"
related-pr: null
last-updated-utc: 2026-07-16
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/analysis/20260716-shutdown-process/README.md
    - docs/features/shutdown-process/README.md
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
    - src/bootstrap/jobs/manager.rs
    - src/bootstrap/jobs/torrent_cleanup.rs
    - src/bootstrap/jobs/activity_metrics_updater.rs
    - packages/axum-server/src/signals.rs
    - packages/udp-server/src/server/launcher.rs
    - packages/swarm-coordination-registry/src/statistics/activity_metrics_updater.rs
---

<!-- skill-link: create-issue -->

# Issue #1588 - Review shutdown process for all tasks/jobs

> **EPIC position**: Subissue #1 of #1488. First step — inventory and analysis before any implementation.

## Goal

Review all jobs and tasks in the Torrust Tracker application to identify which ones still handle the Ctrl+C signal directly instead of using the centralized `CancellationToken` from the `JobManager`. Document the current state and produce a list of remaining jobs that need migration.

## Background

The `JobManager` type was introduced in PR #1587 to centralize job management. It provides a shared `CancellationToken` that can be used to signal all jobs to stop. However, some jobs were not updated to use it:

- **HTTP servers (Axum)**: Use `torrust_server_lib::signals::global_shutdown_signal` inside the `shutdown_signal()` function, which listens for `ctrl_c` and `SIGTERM` directly.
- **Activity metrics updater**: Uses `tokio::signal::ctrl_c()` directly in its loop.
- **Torrent cleanup job**: Uses `tokio::signal::ctrl_c()` directly in its loop.

Additionally, the `main.rs` entry point only handles `SIGINT` (Ctrl+C), not `SIGTERM`.

## Tasks

### Task 1: Inventory all jobs and their shutdown mechanism

Create a complete inventory of all jobs spawned by the application, including:

- Event listeners (swarm, core, http-core, udp-core, udp-server stats, udp-server banning)
- UDP tracker instances
- HTTP tracker instances
- REST API server
- Health Check API server
- Torrent cleanup
- Activity metrics updater

For each job, document:

- What shutdown mechanism it uses (`CancellationToken`, direct `ctrl_c`, halt channel)
- Whether it responds to `jobs.cancel()`
- Whether it would stop on `SIGTERM`

### Task 2: Identify gaps

From the inventory, produce a list of jobs that:

- Do not respond to the `CancellationToken`
- Do not respond to `SIGTERM`
- Have inconsistent shutdown behavior

### Task 3: Propose migration plan

For each gap, propose a concrete migration:

- Which jobs can be migrated to `CancellationToken` directly
- Which jobs need a two-phase shutdown (e.g., Axum servers)
- Whether the `global_shutdown_signal` in servers should be removed or kept

## Acceptance Criteria

- [ ] Complete inventory documented in a table
- [ ] Gaps identified and categorized
- [ ] Migration plan reviewed and approved
- [ ] New sub-issues created for each migration task

## References

- [PR #1587](https://github.com/torrust/torrust-tracker/pull/1587) — introduced centralized shutdown for event listeners
- [Shutdown Analysis](../../analysis/20260716-shutdown-process/README.md) — detailed code-level analysis
- [Feature: Shutdown Process](../../features/shutdown-process/README.md) — product-oriented feature description

## Manual Verification

Evidence of these steps must be recorded in `verification.md` in this folder
before the issue can be closed.

### Test 1: Complete inventory table exists

After completing Task 1, confirm the inventory table in this issue (or in a
linked document) covers all of the following jobs:

- [ ] swarm coordination registry event listener
- [ ] tracker core event listener
- [ ] HTTP core event listener
- [ ] UDP core event listener
- [ ] UDP server stats event listener
- [ ] UDP server banning event listener
- [ ] UDP tracker instances (one per configured port)
- [ ] HTTP tracker instances (one per configured port)
- [ ] REST API server
- [ ] Health Check API server
- [ ] Torrent cleanup
- [ ] Activity metrics updater (peers inactivity update)

For each job, the inventory must document all three columns:

- Shutdown mechanism (`CancellationToken` / direct `ctrl_c` / halt channel)
- Whether it responds to `jobs.cancel()`
- Whether it would stop on `SIGTERM`

**Record in `verification.md`**: a copy of or link to the completed inventory table.

### Test 2: Gaps identified match the analysis

Confirm the gaps identified in Task 2 are consistent with the findings in the
[shutdown analysis §7](../../analysis/20260716-shutdown-process/README.md).

Specifically, at minimum these gaps must be identified:

- [ ] Torrent cleanup uses direct `ctrl_c` — does not respond to `jobs.cancel()`
- [ ] Activity metrics updater uses direct `ctrl_c` — does not respond to `jobs.cancel()`
- [ ] HTTP/REST API/Health Check servers use `global_shutdown_signal()` independently
- [ ] `main.rs` does not handle `SIGTERM`

**Record in `verification.md`**: the gap list, confirming it matches or extends
the analysis findings.

### Test 3: Migration plan covers all gaps

Confirm Task 3 produces a migration entry for every gap found in Test 2, and
that each entry maps to an existing draft sub-issue (SI-1 through SI-9) or
proposes a new one.

**Record in `verification.md`**: the migration mapping table (gap → sub-issue).
