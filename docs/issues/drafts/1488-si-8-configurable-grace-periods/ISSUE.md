---
doc-type: issue
issue-type: task
status: superseded
priority: p3
github-issue: null
spec-path: docs/issues/drafts/1488-si-8-configurable-grace-periods/ISSUE.md
branch: null
related-pr: null
last-updated-utc: 2026-09-01
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/configuration/src/v3_0_0/
    - src/main.rs
    - src/bootstrap/jobs/manager.rs
    - packages/axum-server/src/signals.rs
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
    - docs/features/shutdown-process/questions.md
---

<!-- skill-link: create-issue -->

# Superseded Draft SI-8 — Split Shutdown Policy Configuration

> **Status**: Superseded for implementation planning. [SI-20](../1488-si-20-configure-shutdown-policy/ISSUE.md)
> replaces this draft after Q3 and Q4 defined the final outcome and deadline
> semantics.

## Why This Draft Is Superseded

This draft hardcodes values before Q4 defines the deadline hierarchy and treats
the old per-job timeout as the final model. The replacement must configure the
final policy: an overall process deadline, component budgets, validation rules,
and the required margin below the orchestrator deadline. SI-20 is the active
replacement now that Q3 and Q4 have defined those contracts.

Do not implement this draft.

## Original Goal

Replace the hardcoded grace period constants with a `[shutdown]` configuration
section so operators can tune the shutdown timeout to match their deployment
environment (Docker, Kubernetes, systemd, etc.).

## Background

Grace periods are currently hardcoded in two places:

- `src/main.rs`: `jobs.wait_for_all(Duration::from_secs(10))`
- `packages/axum-server/src/signals.rs`:
  `let grace_period = Duration::from_secs(90);`
  `let max_wait = Duration::from_secs(95);`

These magic numbers cannot be tuned by operators. For example:

- Docker's default `stop_grace_period` is 10s — operators who want graceful
  drain must increase it, but they also need to increase the tracker's own
  timeout to match.
- Kubernetes `terminationGracePeriodSeconds` defaults to 30s.
- Systemd `TimeoutStopSec` defaults to 90s on most distros.

## Proposed Configuration Schema

```toml
[shutdown]
# Total time the tracker will wait for all jobs to finish before forcing exit.
# Set this lower than your container/service manager's own grace period.
# Default: 30s
grace_period_secs = 30

# Time each Axum server waits for active HTTP connections to finish.
# Must be < grace_period_secs to ensure the process exits within the total budget.
# Default: 25s
connection_drain_secs = 25
```

## Implementation

1. Add `Shutdown` struct to `packages/configuration/src/v3_0_0/`.
2. Add optional `shutdown: Option<Shutdown>` field to `Configuration`.
3. Pass the config through the bootstrap to `start_job` calls and `wait_for_all`.
4. Use the config values in `main.rs` and `packages/axum-server/src/signals.rs`.

## Acceptance Criteria

- [ ] Q3 and Q4 are resolved.
- [ ] A `[shutdown]` section is added to the configuration schema (v3.0.0).
- [ ] `grace_period_secs` controls the `JobManager` timeout.
- [ ] `connection_drain_secs` controls the Axum server drain timeout.
- [ ] Default values produce the same behavior as the current hardcoded values
      (or the improved values agreed in SI-6).
- [ ] Config documentation is updated.
- [ ] `linter all` passes.

## Dependencies

- Q3 (exit codes) and Q4 (grace period target values) must be resolved.
- Should land after SI-6 (which sets the correct non-configurable defaults).
- Coordinates with the Configuration Overhaul EPIC (#1978) for schema placement.

## Manual Verification

Evidence of these steps must be recorded in `verification.md` in this folder
before the issue can be closed.

### Test 1: Default values produce correct behavior

Run the tracker **without** a `[shutdown]` section in the config. Confirm that
default values are used and the shutdown behavior matches the behavior from SI-6.

**Record in `verification.md`**: log output confirming the default timeout value
is logged at startup (e.g., `INFO  Shutdown grace period: 30s`).

### Test 2: Custom `grace_period_secs` is respected

Add to the config:

```toml
[shutdown]
grace_period_secs = 5
```

Start the tracker and hold open an HTTP connection during shutdown. The tracker
should time out the job after 5 seconds.

**Expected**:

```text
WARN  Job did not complete in time job=http_instance_0_...
```

Time the shutdown from SIGINT to process exit — it should be approximately 5s.

**Record in `verification.md`**: timing evidence (timestamps from log).

### Test 3: Custom `connection_drain_secs` is respected

Add to the config:

```toml
[shutdown]
connection_drain_secs = 3
```

Hold an HTTP connection open and send Ctrl+C. The Axum server should close
connections after 3 seconds and report its drain timeout.

### Test 4: Invalid config values are rejected at startup

Set `connection_drain_secs` > `grace_period_secs` (logically invalid). Confirm
the tracker refuses to start with a clear error message.

**Expected**: startup error mentioning the invalid relationship.

### Test 5: Config documentation is accurate

Read `share/default/config/tracker.development.sqlite3.toml` and any
documentation added for `[shutdown]`. Confirm the documented defaults match
the actual behavior observed in Tests 1–3.
