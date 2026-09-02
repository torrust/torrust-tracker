---
doc-type: issue
issue-type: task
status: open
priority: p2
github-issue: 1588
spec-path: docs/issues/open/1588-review-shutdown-process-for-all-tasks-jobs/ISSUE.md
branch: "1588-review-shutdown-process"
related-pr: null
last-updated-utc: 2026-09-02 07:44
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/analysis/20260716-shutdown-process/README.md
    - docs/features/shutdown-process/README.md
    - docs/features/shutdown-process/questions.md
    - docs/features/shutdown-process/task-inventory.md
    - docs/issues/open/1586-evaluate-job-manager-join-set/ISSUE.md
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
    - src/bootstrap/jobs/manager.rs
    - src/bootstrap/jobs/torrent_cleanup.rs
    - src/bootstrap/jobs/activity_metrics_updater.rs
    - packages/axum-server/src/signals.rs
    - packages/udp-server/src/server/launcher.rs
    - packages/swarm-coordination-registry/src/statistics/activity_metrics_updater.rs
---

<!-- skill-link: create-issue -->

# Issue #1588 — Review Shutdown Process for All Tasks/Jobs

> **EPIC position**: Roadmap sequence 0. Validate the final task inventory and
> migration boundaries before closing this analysis issue; it is not a blocker
> for the additive supervisor evaluation in #1586.

## Goal

Revalidate the complete task inventory against the implementation and confirm
the final migration boundaries for the supervised cancellation tree. The
planning-time [task inventory](../../../features/shutdown-process/task-inventory.md)
is the baseline; this issue produces implementation-time evidence before it
closes.

## Background

PR #1587 introduced `JobManager` and token-based cancellation for event
listeners. The selected architecture now requires root/child cancellation
tokens, component-owned child joining, and named supervisor outcomes. The
initial issue scope remains relevant, but it must cover nested and detached
tasks rather than only direct Ctrl+C listeners.

- **HTTP servers (Axum)**: Use `torrust_server_lib::signals::global_shutdown_signal` inside the `shutdown_signal()` function, which listens for `ctrl_c` and `SIGTERM` directly.
- **Activity metrics updater**: Uses `tokio::signal::ctrl_c()` directly in its loop.
- **Torrent cleanup job**: Uses `tokio::signal::ctrl_c()` directly in its loop.

Additionally, the `main.rs` entry point only handles `SIGINT` (Ctrl+C), not `SIGTERM`.

## Tasks

### Task 1: Revalidate the task inventory and ownership tree

Create a complete inventory of all jobs spawned by the application, including:

- Event listeners (swarm, core, http-core, udp-core, udp-server stats, udp-server banning)
- UDP tracker instances
- HTTP tracker instances
- REST API server
- Health Check API server
- Torrent cleanup
- Activity metrics updater

For each top-level component and owned child task, document:

- Owner and retained handle (or intentionally framework-managed task)
- Current and target cancellation mechanism
- Child completion, timeout, or deliberate-abort policy
- Whether it responds to root cancellation and how a binary maps SIGTERM

### Task 2: Identify implementation gaps

From the inventory, produce a list of jobs that:

- Do not respond to the cancellation tree
- Are detached without an explicit owner or completion policy
- Observe OS signals in library code
- Have inconsistent shutdown, timeout, or outcome behavior

### Task 3: Validate the approved migration plan

Map each confirmed gap to the active #1488 roadmap draft. Do not create a new
competing migration design; update an existing draft only when evidence shows
its stated boundary is incomplete.

## Acceptance Criteria

- [ ] Complete revalidated inventory documents ownership, token propagation,
      completion policy, and configuration-dependent task cardinality.
- [ ] Gaps are identified and mapped to active #1586, SI-1, SI-2, SI-4–SI-5,
      and SI-10–SI-21 work items.
- [ ] The final inventory confirms the #1586 supervisor boundary: direct
      top-level components only, not component-owned child handles.
- [ ] The EPIC roadmap is updated only if implementation evidence exposes a
      missing independently releasable migration slice.

## References

- [PR #1587](https://github.com/torrust/torrust-tracker/pull/1587) — introduced centralized shutdown for event listeners
- [Shutdown Analysis](../../../analysis/20260716-shutdown-process/README.md) — detailed code-level analysis
- [Feature: Shutdown Process](../../../features/shutdown-process/README.md) — product-oriented feature description

## Manual Verification

Evidence of these steps must be recorded in `verification.md` in this folder
before the issue can be closed.

### Test 1: Complete revalidated inventory exists

After completing Task 1, confirm the inventory table in this issue (or the
linked feature inventory) covers all of the following top-level components:

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

For each component, the inventory must document:

- Its owner and retained handle.
- Current and target cancellation mechanism.
- Its child completion, timeout, or deliberate-abort policy.
- How it responds to root cancellation and how executable signal handling
  reaches that path.

**Record in `verification.md`**: a copy of or link to the completed inventory table.

### Test 2: Gaps and ownership boundaries match the analysis

Confirm the gaps identified in Task 2 are consistent with the findings in the
[shutdown analysis §7](../../../analysis/20260716-shutdown-process/README.md).

Specifically, at minimum these gaps and boundaries must be identified:

- [ ] Torrent cleanup uses direct `ctrl_c` — does not respond to `jobs.cancel()`
- [ ] Activity metrics updater uses direct `ctrl_c` — does not respond to `jobs.cancel()`
- [ ] HTTP/REST API/Health Check servers use `global_shutdown_signal()` independently
- [ ] `main.rs` does not handle `SIGTERM`
- [ ] The detached Axum drain controllers require component-owned join policies;
      the separate UDP IP-ban cleanup job remains manager-owned and
      token-cancellable.

**Record in `verification.md`**: the gap list, confirming it matches or extends
the analysis findings.

### Test 3: Active roadmap covers all gaps

Confirm Task 3 maps every gap found in Test 2 to the active EPIC roadmap:
existing issues #1586/#1588 or SI-1, SI-2, SI-4–SI-5, and SI-10–SI-21. Do not
map work to superseded SI-3 or SI-6–SI-9.

**Record in `verification.md`**: the migration mapping table (gap → sub-issue).
