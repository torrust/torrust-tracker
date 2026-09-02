---
semantic-links:
  skill-links:
    - create-adr
  related-artifacts:
    - docs/features/shutdown-process/README.md
    - docs/features/shutdown-process/questions.md
    - docs/features/shutdown-process/shutdown-architecture-examples.md
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
    - docs/issues/open/1586-evaluate-job-manager-join-set/ISSUE.md
    - src/main.rs
    - src/bootstrap/jobs/manager.rs
---

# Adopt a Supervised Cancellation Tree for Shutdown

## Description

Tracker shutdown currently combines root-level `SIGINT` handling, direct
library OS-signal subscriptions, `CancellationToken` listeners, and `Halted`
oneshot channels. Server wrappers already translate the manager token to
private `Halted::Normal` messages, but that bridge retains two normal
cancellation models. Several component-owned child tasks also cannot yet be
joined through their owner.

The shutdown contract spans the tracker executable, `JobManager`, HTTP, REST,
health-check, UDP, standalone package consumers, and external deployment
supervisors. It is therefore a repository-wide architectural decision rather
than a package-local implementation choice.

## Agreement

Adopt a supervised cancellation tree as the target shutdown architecture.

1. Executable entry points are the only OS-signal boundary. On Unix they map
   `SIGINT` and `SIGTERM`; on Windows they map supported Tokio `ctrl_c()` console
   events. They translate these events into one in-process shutdown request.
2. `JobManager` is the application supervisor. It owns only named, direct
   top-level component tasks and the root `CancellationToken`; it does not
   collect nested child handles.
3. A component receives a child token. Cancellation flows top-down from owner
   to child. Completion, failure, timeout, and deliberate-abort outcomes flow
   bottom-up through awaited handles.
4. Every component owns its nested tasks and joins them, or deliberately aborts
   them under a documented bounded policy, before reporting its own outcome.
5. Server libraries expose deterministic in-process lifecycle operations. They
   do not subscribe to OS signals in the target architecture. The existing
   token-to-`Halted` forwarding remains only a temporary compatibility bridge.
6. `Started` remains a one-time startup notification. Shutdown `Halted`
   channels are migration compatibility only and are not the target lifecycle
   contract.

The deployment and exit-result policy is specified by the shutdown feature:
fully graceful completion exits with code `0`; startup/component failure,
timeout, panic, or deliberate abort exits with code `1`. The initial budget is
a 25-second shared process deadline, with a 20-second HTTP-family drain budget
and a 5-second UDP active-request budget. Orchestrators must provide at least
30 seconds, with a larger margin recommended where practical.

### Alternatives Considered

**Supervisor-owned raw `Halted` senders.** Rejected as the target because
transport-specific channels leak into application supervision and do not define
component-owned child completion.

**Cancellation token without lifecycle ownership.** Rejected because token
cancellation only requests stop; it does not prove task completion, failure, or
timeout handling.

**Token-to-oneshot forwarding.** Retained temporarily for source and behavior
compatibility, but rejected as the final architecture because it leaves two
normal cancellation paths and library OS-signal subscriptions.

## Consequences

- Shared lifecycle APIs follow add → migrate consumers → deprecate → remove.
- Each migration must include deterministic token/lifecycle tests; OS-signal
  and container behavior remain executable-level integration evidence.
- A supervisor deadline applies concurrently across top-level components, not
  sequentially as a per-job budget.
- The task inventory must be revalidated against current code before issue
  #1588 closes, and must be revisited whenever lifecycle topology changes.
- The architecture is implemented progressively by EPIC #1488; this ADR does
  not claim that the current transitional implementation already satisfies it.

## Date

2026-09-02

## References

- EPIC #1488: [Overhaul Tracker Shutdown](../issues/open/1488-overhaul-tracker-shutdown/ISSUE.md)
- Issue #1586: [Evaluate `JoinSet` for `JobManager`](../issues/open/1586-evaluate-job-manager-join-set/ISSUE.md)
- Issue #1588: [Review Shutdown Process for All Tasks/Jobs](../issues/open/1588-review-shutdown-process-for-all-tasks-jobs/ISSUE.md)
- [Shutdown feature definition](../features/shutdown-process/README.md)
- [Shutdown decisions](../features/shutdown-process/questions.md)
- [Shutdown architecture examples](../features/shutdown-process/shutdown-architecture-examples.md)
