---
doc-type: issue
issue-type: task
status: draft
priority: p2
github-issue: null
spec-path: docs/issues/drafts/1488-si-20-configure-shutdown-policy/ISSUE.md
branch: null
related-pr: null
last-updated-utc: 2026-09-01
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/configuration/src/v3_0_0/
    - src/main.rs
    - src/app.rs
    - src/bootstrap/jobs/manager.rs
    - packages/axum-server/src/signals.rs
    - packages/udp-server/src/server/launcher.rs
    - share/default/config/tracker.development.sqlite3.toml
    - docs/containers.md
    - docs/features/shutdown-process/README.md
    - docs/features/shutdown-process/questions.md
    - docs/issues/open/1586-evaluate-job-manager-join-set/ISSUE.md
    - docs/issues/drafts/1488-si-8-configurable-grace-periods/ISSUE.md
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
---

<!-- skill-link: create-issue -->

# Draft SI-20 — Configure Shutdown Policy and Deployment Contract

> **EPIC position**: Final policy/configuration task. Implements the approved
> Q3/Q4 process result and deadline contract without changing lifecycle ownership.

## Goal

Expose the approved shutdown policy through validated tracker configuration,
wire it into the already established supervisor and component lifecycle APIs,
and document the minimum deployment deadlines. Map `JobManager` aggregate
outcomes to the process exit result without allowing component tasks to exit the
process directly.

This task configures an existing supervised cancellation tree; it does not
introduce or migrate a lifecycle mechanism.

## Approved Policy

| Policy                                                                 | Default / rule                                                         |
| ---------------------------------------------------------------------- | ---------------------------------------------------------------------- |
| Fully graceful shutdown                                                | Process exit code `0`                                                  |
| Startup failure or any component failure, timeout, or deliberate abort | Process exit code `1`                                                  |
| Process shutdown deadline                                              | 25 seconds; one concurrent deadline for all top-level components       |
| HTTP, REST API, and health-check drain budget                          | 20 seconds                                                             |
| UDP active-request completion budget                                   | 5 seconds                                                              |
| Orchestrator grace period                                              | At least 30 seconds; at least five seconds beyond the process deadline |

The process deadline is not a per-job timeout. An OS signal that cannot be
handled, such as SIGKILL, has an OS-defined result. See Q3/Q4 for rationale.

## Scope

### In scope

- Add a `[shutdown]` configuration section with defaults for the approved
  process, HTTP drain, and UDP active-request budgets.
- Validate that all durations are non-zero and that component budgets fit within
  the process deadline.
- Pass the configured budgets to `JobManager`, the token-aware Axum drain path,
  and the token-aware UDP active-request path.
- Map structured `JobManager` aggregate outcomes to exit code 0 or 1 in the
  executable entry point.
- Update default configuration fixtures and canonical container/deployment
  documentation.
- Add configuration unit tests, invalid-configuration tests, and end-to-end
  shutdown verification using configured deadlines.

### Out of scope

- Changing cancellation propagation, lifecycle ownership, server APIs, or child
  task join/abort behavior.
- Removing legacy APIs or OS-signal helpers; SI-19 owns that breaking removal.
- Implementing readiness behavior during shutdown; SI-21 owns the Q6-approved
  readiness-before-drain change.
- Supporting automatic discovery of an orchestrator's configured deadline.

## Configuration Contract

The exact Rust type names may vary, but the configuration must express these
three values and defaults:

```toml
[shutdown]
process_deadline_secs = 25
http_connection_drain_secs = 20
udp_active_request_deadline_secs = 5
```

Validation must reject:

- zero values;
- HTTP drain budget greater than or equal to the process deadline;
- UDP active-request budget greater than or equal to the process deadline;
- any future component budget that cannot complete within the process deadline.

The tracker cannot validate the external orchestrator deadline from inside its
process. Documentation must instead require:

$$
T_{\text{orchestrator}} \ge T_{\text{process}} + 5\ \text{s}
$$

For default tracker values, Docker/Podman `stop_grace_period`, Kubernetes
`terminationGracePeriodSeconds`, and systemd `TimeoutStopSec` must be at least
30 seconds. Production guidance should recommend 35 seconds or more where
possible.

## Exit-Result Contract

`JobManager` returns structured named outcomes to `main()`. `main()` exits with:

- code 0 only when every top-level component completed within the configured
  process deadline;
- code 1 when startup fails or any component failed, panicked, timed out, or was
  deliberately aborted.

Components must return outcomes to their owner and must not call
`std::process::exit`.

## Acceptance Criteria

- [ ] Configuration supplies defaults of 25s process, 20s HTTP drain, and 5s
      UDP active-request deadline.
- [ ] Validation rejects zero values and component budgets greater than or equal
      to the process deadline with actionable startup errors.
- [ ] Configured budgets reach `JobManager`, token-aware Axum drain, and UDP
      active-request policy without introducing per-job process deadlines.
- [ ] `main()` maps aggregate outcomes to code 0 only for fully graceful
      completion and code 1 for startup/shutdown failure.
- [ ] No component task calls `std::process::exit`.
- [ ] Default configuration fixtures and `docs/containers.md` state the
      30-second minimum external grace period and recommend 35 seconds or more
      where supported.
- [ ] Deterministic tests cover defaults, overrides, invalid relationships, and
      outcome-to-exit mapping without OS signals.
- [ ] End-to-end verification tests the tracker under a configured 30-second or
      longer container/service-manager deadline; Docker's default 10-second
      deadline is documented as insufficient.
- [ ] `linter all` passes.

## Dependencies

- Q3 and Q4 are resolved.
- Issue #1586 provides structured concurrent supervisor outcomes.
- SI-10 through SI-15 provide token-aware Axum and UDP paths that consume the
  configured component budgets.
- SI-21 follows this task and must use the configured process deadline without
  adding a separate timer.

## Rollback

Restore the prior default-only policy and remove the new configuration section,
exit mapping, and deployment documentation together. This is safe only before
operators rely on the published configuration. After release, preserve the
configuration fields and defaults in a compatibility patch rather than silently
changing their meaning.

## Manual Verification

Record evidence in `verification.md` before closing this issue.

1. Run deterministic configuration and outcome-to-exit tests without OS signals.
2. Verify default fixtures apply the approved 25s/20s/5s policy.
3. Verify invalid zero and budget-relationship configurations fail at startup
   with actionable errors.
4. Run the tracker in a container configured with at least 30 seconds grace;
   record SIGTERM, named component outcomes, and process result.
5. Verify the deployment documentation warns that Docker/Podman's default
   10-second deadline is insufficient and shows a configured grace period.
