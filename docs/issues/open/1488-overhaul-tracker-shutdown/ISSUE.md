---
schema-version: 1
doc-type: epic
status: in-progress
epic: null
github-issue: 1488
spec-path: docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
epic-owner: josecelano
last-updated-utc: "2026-11-12 14:30"
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

The accepted architecture and alternatives are recorded in the
[supervised cancellation-tree ADR](../../../adrs/20260902074438_adopt_supervised_cancellation_tree_for_shutdown.md).

## Why This Is Needed

The current shutdown process has several problems identified in the
[shutdown analysis](../../../analysis/20260716-shutdown-process/README.md).
The list keeps the problems as originally found; each item ends with its
status as of 2026-10-07.

1. **No `SIGTERM` in `main.rs`** — only `SIGINT` (Ctrl+C) is handled at the top
   level. Container orchestrators (Docker/Podman) send `SIGTERM` by default,
   which means `jobs.cancel()` and `jobs.wait_for_all()` are never called.
   **Status: fixed** by SI-1 (#2132).
2. **Three inconsistent shutdown mechanisms** — jobs use `CancellationToken`,
   direct `tokio::signal::ctrl_c()`, or oneshot `Halted` channels. Server
   wrappers currently bridge the manager token to `Halted`, leaving two normal
   cancellation layers; periodic jobs still ignore `JobManager` cancellation.
   **Status: fixed in the application** (sequences 3 to 11) and in the
   standalone HTTP, UDP, REST API, and health-check API environments (SI-16,
   SI-17, SI-23, and SI-24); the legacy API is removed by SI-18 and SI-19.
3. **Torrent cleanup and activity metrics ignore `CancellationToken`** — they
   listen for `ctrl_c` directly instead of using the shared token.
   **Status: fixed** by SI-4 (#2169) and #2221.
4. **Grace period mismatch** — `JobManager` waits 10s per job sequentially and
   force-aborts timed-out wrappers, while Axum servers have a 90s graceful
   shutdown with detached drain controllers. **Status: fixed**: #1586
   made `JobManager` wait concurrently under one deadline, #2274 made the
   Axum drain joinable, and SI-20 (#2668) replaced the 90s drain and the 10s
   deadline with configured budgets that the startup validation keeps
   consistent (defaults 15s and 25s).
5. **No graceful UDP shutdown** — the UDP server simply aborts its main loop.
   **Status: fixed** by #2342 and #2370.
6. **Hardcoded timeouts** — grace periods are magic numbers with no configuration
   surface. **Status: fixed** by SI-20 (#2668), which adds the
   `[application.shutdown]` configuration section.
7. **Incomplete shutdown observability** — named per-job waiting and timeout
   logs exist, but the supervisor has no concurrent aggregate outcomes or
   complete component-level drain progress. **Status: partly fixed**: #1586
   added concurrent named outcomes, and the migrated components log their
   drains; listener drain logs were added by SI-22 (#2414).
8. **Double-signal on Ctrl+C** — both `main.rs` and each server's
   `global_shutdown_signal()` catch the same signal, creating a potential race.
   **Status: fixed in the application**: production servers use the token
   lifecycle, and the test environments no longer use the legacy stop path
   (SI-24 migrated the last one); SI-19 removes the legacy API.
9. **Event listeners drop events on shutdown** — found on 2026-10-01, after
   the original analysis. Every event listener stops as soon as cancellation is
   requested, even with events still queued, and servers that are still
   draining keep emitting events after the listeners have stopped. Persisted
   completed-download counts can be lost. This predates the cancellation-token
   refactor (#1405 changed the trigger, not the policy). **Status: fixed**
   (SI-22, sub-EPIC #2410): listeners drain their queues within an admission
   budget and are stopped after the components that publish to them.

## The Contracts Being Implemented

These are not new features — they are standard behaviors that every process
manager, container runtime, and operator already expects. Status as of
2026-10-02:

```bash
# These all trigger coordinated shutdown since SI-1 (#2132):
kill <pid>               # SIGTERM — works ✅
docker stop <container>  # SIGTERM — works ✅
systemctl stop <service> # SIGTERM — works ✅
# Kubernetes pod delete    # SIGTERM — works ✅

# This also works:
kill -INT <pid>          # SIGINT — works ✅

# This should be the last resort, never needed in normal operation:
kill -9 <pid>            # SIGKILL — force kill ❌
```

Before SI-1, `SIGTERM` reached the server libraries but bypassed `main`, so
only the servers stopped. Adding the `SIGTERM` handler in `main.rs` was the
single most impactful change in this EPIC. SI-22 later added ordered listener
draining with an explicit admission timeout and failure reporting, not an
unconditional zero-loss guarantee. SI-20 (#2668) then made the shutdown budget
configurable and documented how it fits the orchestrator's grace period, and
made the exit code report the outcome, and SI-21 (#2697) made the health check
report not ready during shutdown. The whole contract is now implemented.

The signal examples establish that coordinated shutdown starts, not that every
deployment waits long enough for it to finish. Docker, systemd, and Kubernetes
can still enforce an external deadline and kill the process; SI-20 documented
that budget alignment and operator guidance.

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
- Event listeners process queued events, and event producers stop before
  their consumers, subject to drain budgets and explicit failure reporting (SI-22).

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

| Sequence | Draft | Work item                                                                                                                 | Status     | Independently releasable scope                                                                       |
| -------- | ----- | ------------------------------------------------------------------------------------------------------------------------- | ---------- | ---------------------------------------------------------------------------------------------------- |
| 0        | #1588 | [Revalidate task inventory](../../closed/1588-review-shutdown-process-for-all-tasks-jobs/ISSUE.md)                         | Done       | Implementation-time inventory and ownership evidence; no runtime behavior changes.                   |
| 1        | SI-1  | [Add `SIGTERM` at `main()`](../../closed/2132-add-sigterm-to-main/ISSUE.md)                                                | Done       | Incremental signal-boundary compatibility fix.                                                       |
| 2        | #1586 | [Evaluate `JoinSet` for `JobManager`](../../closed/1586-evaluate-job-manager-join-set/ISSUE.md)                            | Done       | Direct supervisor task ownership, concurrent outcomes, and explicit escalation policy.               |
| 3        | SI-4  | [Migrate torrent cleanup](../../closed/2169-1488-si-4-migrate-torrent-cleanup/ISSUE.md)                                    | Done       | One periodic component adopts token cancellation.                                                    |
| 4        | #2221 | [Migrate activity metrics](../../closed/2221-1488-si-5-migrate-activity-metrics-updater/ISSUE.md)                           | Done       | One periodic component adopts token cancellation.                                                    |
| 5        | #2234 | [Add token-aware server lifecycle API](../../closed/2234-1488-si-2-remove-global-shutdown-signal/ISSUE.md)                         | Done       | Additive `torrust-server-lib` API; retain legacy shutdown compatibility.                             |
| 6        | #2274 | [Add token-aware, joinable Axum drain helper](../../closed/2274-1488-si-10-add-token-aware-axum-drain-helper/ISSUE.md)               | Done       | Additive helper alongside existing API; no consumer breaks.                                          |
| 7        | #2289 | [Migrate HTTP tracker to token lifecycle](../../closed/2289-1488-si-11-migrate-http-tracker-token-lifecycle/ISSUE.md)      | Done       | One complete HTTP vertical slice.                                                                    |
| 8        | #2309 | [Migrate REST API to token lifecycle](../../closed/2309-1488-si-12-migrate-rest-api-token-lifecycle/ISSUE.md)             | Done       | One complete REST API vertical slice.                                                                |
| 9        | #2324 | [Migrate health-check API to token lifecycle](../../closed/2324-1488-si-13-migrate-health-check-api-token-lifecycle/ISSUE.md)      | Done       | One health-check vertical slice; SI-21 separately implements readiness-before-drain.                 |
| 10       | #2342 | [Migrate UDP tracker to token lifecycle](../../closed/2342-1488-si-14-migrate-udp-receive-reset-token-lifecycle/ISSUE.md) | Done       | Cooperative token-aware UDP stop; owned receive loop; safe legacy adapter; request abort fallback unchanged. |
| 11       | #2370 | [Define UDP active-request shutdown policy](../../closed/2370-1488-si-15-define-udp-active-request-policy/ISSUE.md)  | Done       | Loop-owned request processors; five-second drain with deadline abort and one outcome summary.        |
| 12       | #2412 | [Migrate standalone HTTP environment/example](../../closed/2412-1488-si-16-migrate-standalone-http-environment/ISSUE.md) | Done       | One supported standalone HTTP consumer migration; the token-aware drain now force-closes at its deadline (#2439). |
| 13       | #2448 | [Migrate standalone UDP environment/example](../../closed/2448-1488-si-17-migrate-standalone-udp-environment/ISSUE.md) | Done       | SI-17. One supported standalone UDP consumer migration; a dropped environment now releases its socket (#2459). |
| 14       | #2471 | [Stop the HTTP test environment when dropped without `stop()`](../../closed/2471-1488-fix-http-environment-drop-path/ISSUE.md) | Done       | Bug. Gap found on 2026-10-07: a dropped HTTP test environment keeps its server running, before and after SI-16 (checked on the tree before SI-16). Hold the token as a `DropGuard`, as #2459 did for UDP. Merged in PR #2544 on 2026-10-16. |
| 15       | #2410 | [Process queued events before listeners stop](../../closed/2410-1488-si-22-process-queued-events-before-listeners-stop/EPIC.md) | Done | Bug, sub-EPIC (SI-22) with four sub-issues, #2413 to #2416 (docs and ADR draft; listener drain; per-component tokens; stop order): event producers stop before consumers; listeners process queued events within an admission budget. Merged in PRs #2553, #2578, #2591, and #2623, closed by the close-out PR. |
| 16       | #2449 | [Migrate REST API test environment](../../closed/2449-1488-si-23-migrate-rest-api-test-environment/ISSUE.md) | Done       | SI-23. One test environment migration; found while refreshing SI-17 (2026-10-06). Merged in PR #2643 on 2026-10-25; one token and one `DropGuard`, because the environment owns no listener. |
| 17       | #2450 | [Migrate health-check API test environment](../../closed/2450-1488-si-24-migrate-health-check-api-test-environment/ISSUE.md) | Done       | SI-24. One test environment migration; found while refreshing SI-17 (2026-10-06). Merged in PR #2652 on 2026-10-26; one token and one `DropGuard`, because the environment owns no listener. Review found that a fixed-port restart panicked because the registry could not remove a registration, so `torrust-server-lib` 0.4.0 added `Registar::deregister` and `stop()` removes its own binding. |
| 18       | #2751 | [Deprecate legacy shutdown API](../../closed/2751-1488-si-18-deprecate-legacy-shutdown-api/ISSUE.md)                           | Done       | Compatibility-preserving source deprecation only.                                                    |
| 19       | #2801 | [Remove legacy shutdown API and library OS signals](../../closed/2801-1488-si-19-remove-legacy-shutdown-api/ISSUE.md)     | Done       | Removed the legacy API; merged in PR #2807 with `torrust-server-lib` 0.6.0.                  |
| 20       | #2668 | [Configure shutdown policy and deployment contract](../../closed/2668-1488-si-20-configure-shutdown-policy/ISSUE.md)     | Done       | Apply approved Q3/Q4 outcomes, budgets, configuration, and deployment guidance.                      |
| 21       | #2697 | [Mark health check unhealthy during shutdown](../../closed/2697-1488-si-21-mark-health-unhealthy-during-shutdown/ISSUE.md)         | Done       | Set readiness to not ready before root cancellation and component drain.                             |
| 22       | #2844 | [Remove the unused public error variants](../2844-1488-si-25-remove-unused-error-variants/ISSUE.md) | Planned | Follow-up of SI-19. Remove the shutdown error variants nothing constructs (`Error::Stop`, `StartupNotification`, `StartupNotificationDropped` of the HTTP and REST servers; three `UdpError` variants). Breaking change to public enums of unpublished crates, so it lands before the first publish (#2583). |
| 23       | #2845 | [Consolidate the shutdown supervisors and migrate the last legacy job](../2845-1488-si-26-consolidate-shutdown-supervisor/ISSUE.md) | Planned | Follow-up of SI-19 and SI-21. One shared token-aware supervisor with the health-check join order, review of the owner types (`OwnedTask`, `TokenAwareServerTask`, `OwnedReceiveLoop`), and `udp_ban_cleanup` (the only `register_legacy` job, not ordered by the stop-last edge) becomes a reserved component. Lands before #2583. |
| 24       | #2846 | [Deregister stopped services from the Registar](../2846-1488-si-27-deregister-stopped-services/ISSUE.md) | Planned | Follow-up of SI-21. An orderly stop removes the service's registration (`Registar::deregister`); a failed stop keeps it. Starts with a go/no-go decision. Lands before #2583. |
| 25       | #2847 | [Verify the hung-component abort on the real binary](../2847-1488-si-28-verify-hung-component-abort/ISSUE.md) | Planned | Follow-up of SI-21 and SI-20. Executable-boundary evidence that a never-finishing component is aborted at the process deadline with exit code 1. Evidence only; after SI-26. |
| —        | SI-3  | [Combined standalone environment migration](discarded-drafts/si-3-fix-environment-stop/discarded-draft.md) | Superseded | Replaced by SI-16 and SI-17. Do not implement. Discarded draft, kept as evidence in `discarded-drafts/`. |
| —        | SI-6  | [Concurrent supervisor outcomes](discarded-drafts/si-6-align-grace-periods/discarded-draft.md) | Superseded | Replaced by existing issue #1586. Do not implement separately. Discarded draft, kept as evidence in `discarded-drafts/`. |
| —        | SI-7  | [Standalone shutdown-progress reporting](discarded-drafts/si-7-observable-shutdown-progress/discarded-draft.md) | Superseded | Structured outcomes are incorporated into issue #1586. Do not implement. Discarded draft, kept as evidence in `discarded-drafts/`. |
| —        | SI-8  | [Original shutdown configuration](discarded-drafts/si-8-configurable-grace-periods/discarded-draft.md) | Superseded | Replaced by SI-20 after Q3/Q4 decisions. Do not implement. Discarded draft, kept as evidence in `discarded-drafts/`. |
| —        | SI-9  | [Combined UDP shutdown migration](discarded-drafts/si-9-improve-udp-shutdown/discarded-draft.md) | Superseded | Replaced by SI-14 and SI-15. Do not implement. Discarded draft, kept as evidence in `discarded-drafts/`. |

SI-25 and SI-26 were added on 2026-11-11 as follow-ups split out of SI-19 scope, and SI-27 and SI-28
on 2026-11-12 from the SI-21 follow-ups, when the maintainer asked for a GitHub issue and a consistent
spec for every pending item. They do not change the shutdown contract, which is implemented; the EPIC
stays open until they are done or dropped. Recommended implementation order: SI-25 (#2844), SI-26
(#2845), SI-27 (#2846), SI-28 (#2847); SI-28 follows SI-26 so its evidence covers the final supervisor.
The maintainer decided on 2026-11-12 that SI-25, SI-26 and SI-27 land before the first publish of the
workspace crates (#2583), which is blocked on this EPIC as a whole.

The four specs were written in one combined spec PR, so their `branch:` frontmatter names the reserved
implementation branch (`{issue}-{description}`), not the spec branch; the implementing PR uses that name.

**Why SI-16 and SI-17 come before SI-20 and SI-21**: the HTTP and UDP package
integration tests run through the standalone test environments, which still use
the legacy `Halted` shutdown path. Migrating them first makes those tests
exercise the token-aware path the tracker uses in production, so the later
budget, exit-code, and readiness changes are covered by an existing safety net.
SI-20 has no dependency on SI-16 to SI-19; the order is a deliberate safety
choice, not a technical constraint. SI-22 follows SI-17 so the fix can be
applied to the application and both migrated test environments at once, and
precedes SI-20 because it prevents possible loss of persisted data. The
maintainer prefers safe, incremental steps over doing the highest-impact item
first, and favors tasks that strengthen the test safety net before the changes
that rely on it (review of 2026-10-01). SI-23 and SI-24 were added on
2026-10-06, when the SI-17 refresh found the REST API and health-check API test
environments still on the legacy path; they must precede SI-18 and SI-19.
On 2026-10-15 the maintainer agreed that SI-23 and SI-24 do not depend on
SI-22 and may be done alongside it, not only after it: they unblock the
environment moves of EPIC #2482, which waits for them. The sequence numbers
stay as they are.
Issue #2471 was added on 2026-10-07, when checking SI-22 against SI-17 showed
that the HTTP test environment does not stop its server when dropped without
`stop()`, a gap that predates SI-16 (the legacy halt sender's drop only made
the shutdown task panic). It precedes SI-22 because SI-22 T9 changes the same
environment.

### Discarded Drafts

The superseded drafts SI-3, SI-6, SI-7, SI-8, and SI-9 were never opened as GitHub issues, so they have
no closed-issue folder. They record why the original plan was split or replaced, and several closed
specs link to them. They live in [`discarded-drafts/`](discarded-drafts/) in this EPIC folder as
artifacts of the EPIC: documented evidence of decisions taken during it. They are not deleted in the
close-out. Each lost its frontmatter on the move, because the validator treats a file with issue
frontmatter in `docs/issues/open/` as a live spec; git history has the originals.

Policy for drafts that are never opened as issues and are replaced or dropped: a draft that documents a
decision stays as an artifact of its EPIC (here); any other discarded draft is removed, its decision
recorded in the replacing issue or the EPIC, or it moves to a discarded-drafts folder. The issue
workflow, the lifecycle folders and the validator are not changed for now; the policy may be revisited
later.

### Release and Review Requirements

- Each work item must preserve a supported shutdown path for the tracker and
  every affected standalone consumer.
- A shared API task must be additive until all consumers migrate; no API removal
  may be bundled with the first consumer migration.
- Each component task must demonstrate top-down cancellation and bottom-up,
  owner-joined completion without exposing nested task handles to `JobManager`.
- Each task requires deterministic token/lifecycle tests. OS signals and
  Docker/Podman behavior are end-to-end verification, not unit-test mechanisms.
- Every runtime-affecting shutdown work item must record issue-local
  executable-boundary verification in `manual-verification-evidence.md`: start
  the affected tracker or standalone binary with a configuration that enables
  the changed service, establish readiness, signal the actual binary PID (not
  only a `cargo run` parent) with `SIGTERM`, capture bounded exit status and
  shutdown output, then restart or otherwise prove listener/resource release.
  Repeat with `SIGINT` when the work changes an executable signal boundary.
  An additive API that has no production consumer records this as supported-path
  regression evidence and states that limitation; its first consumer migration
  proves the new production path. Pure documentation or test-only tasks may
  record a reasoned non-applicability decision instead.
- Each task must have a documented rollback/revert story: revert the component
  migration while the legacy API remains available, or revert an additive API
  without changing existing consumers.

### Completion Follow-up: Issue #1419

Before declaring this EPIC complete, reassess and complete
[issue #1419](https://github.com/torrust/torrust-tracker/issues/1419), **Allow
multiple integration tests at the main app level**. Its remaining shared-event-
bus policy integration test used to depend on canonical identity and
registration metadata from #2036 and #2041, and on the event-metrics policy work
in #2039. All three are closed; #2039 was reopened on 2026-08-18 after a
premature automatic closure and closed again on 2026-08-19, when PR #2064
implemented it. This EPIC must also leave every started application instance with deterministic,
awaitable shutdown and no leaked tasks or listener bindings.

The final #1419 verification must demonstrate concurrent main-application test
instances can start, discover their canonical services, exercise the enabled
and metrics-disabled HTTP/UDP listener policy, and complete shutdown without
port, storage, environment, logging, or lingering-task interference. Do not
absorb the canonical-identity or metrics-policy requirements into this EPIC;
use #1419 to verify their interaction with the completed shutdown lifecycle.

## Dependencies

- **#1405** (Overhaul stats: graceful shutdown for broadcast channels) — ✅ Closed.
  Implemented with `CancellationToken`, which is the foundation for this EPIC.
- **#1477** (Fix shutdown message and improve it) — ✅ Closed.
  Introduced the `JobManager` type.
- **#1586** (Evaluate `JoinSet` for `JobManager`) — ✅ Closed.
  Implemented direct supervisor task ownership and outcome handling.
- **#1588** (Review shutdown process for all tasks/jobs) — ✅ Closed.
  Produced the implementation-time task inventory and ownership evidence.

## Related Documents

- [Analysis: Shutdown Process](../../../analysis/20260716-shutdown-process/README.md) — detailed code-level analysis
- [Feature: Shutdown Process](../../../features/shutdown-process/README.md) — product-oriented feature description
- [Questions and Decisions](../../../features/shutdown-process/questions.md) — resolved specification decisions and risks
