---
schema-version: 1
doc-type: issue
issue-type: task
status: blocked
priority: p2
epic: null
github-issue: 2387
spec-path: docs/issues/open/2387-review-tracker-lifecycle-logs/ISSUE.md
branch: "2387-review-tracker-lifecycle-logs"
related-pr: null
last-updated-utc: "2026-09-30 12:24"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - src/main.rs
    - src/bootstrap/jobs/manager.rs
    - packages/axum-server/src/signals.rs
    - packages/udp-server/src/server/launcher.rs
    - share/default/config/tracker.development.sqlite3.toml
    - docs/features/shutdown-process/task-inventory.md
    - docs/issues/closed/2370-1488-si-15-define-udp-active-request-policy/ISSUE.md
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
    - "issue #1166"
---

<!-- skill-link: create-issue -->

# Issue #2387 - Make Tracker Startup and Shutdown Logs Coherent for Operators

## Goal

An operator who starts the tracker and then stops it should get startup and shutdown logs that
use one consistent message style, field names, and levels, and that include the facts needed to
see what is running and whether it stopped cleanly.

## Background

The Copilot review of PR #2382 (issue #2370, EPIC #1488 SI-15) found that the new UDP drain logs
named the listener `local_addr`. The rest of the codebase mostly uses `service_binding`. The drain
logs were renamed, but the older UDP receive-loop logs still say `local_addr`, because #2370 must
not change existing log messages. The maintainer asked for a follow-up covering the whole lifecycle,
not just that field: run the tracker, read what it logs at startup, shut it down, read what it logs
then, make everything coherent, and look for missing operator information.

Evidence from one real run: a release build with UDP and health-check services, stopped with
`SIGTERM` after one announce (#2370 manual scenario M1, 2026-09-30). The full baseline with every
service enabled, using the unchanged development configuration, is in
[baseline-lifecycle-logs.md](baseline-lifecycle-logs.md).

| Observation | Example |
| --- | --- |
| The listener is identified four ways | `local_addr`, `service_binding`, `address`, `bind_address` (one run; more may appear with HTTP and REST enabled) |
| Message prefixes mix styles | Human prefixes such as `UDP TRACKER:`, `HEALTH CHECK API:`, `METRICS:` sit beside Rust module paths such as `torrust_tracker_lib::bootstrap::jobs::manager:` |
| Span fields leak internal `Debug` output | `start_job{... cancellation_token=CancellationToken { is_cancelled: false } ...}` on every startup line of a job |
| Some start lines are duplicated | `HEALTH CHECK API: Starting on: ...`, then `Started health check API ...`, then `Started on: ...` |
| A normal startup emits eight `WARN` lines when usage statistics are disabled | `Ignoring UDP server event from an unknown or metrics-disabled listener configuration_instance_id=ConfigurationInstanceId { service_role: UdpTracker, instance_index: 0 }`; the development configuration, which enables statistics, emits none |
| Metrics registration floods `INFO` | 26 `METRICS: type=... name=... description=Some(MetricDescription(...))` lines, with `Debug` formatting |
| Shutdown messages vary | `!! Shutting down health check API server ... !!`, `Received cancellation request, shutting down ...`, `Stopping ... job ...`, `Job completed after cooperative cancellation job=...` |

Open issue #1166 already discusses how log levels, especially `ERROR`, are used for request
handling. This issue covers only the lifecycle (startup and shutdown). It should apply any level
policy #1166 decides, not redefine it.

## Scope

### In Scope

- Capture the lifecycle logs of real tracker runs with every service enabled (UDP, HTTP, REST API,
  health check, periodic jobs), from process start to exit after `SIGTERM`, at `info` and `debug`.
- Inventory every lifecycle log line: level, target, message style, fields and their names, and
  value formatting.
- Propose a lifecycle logging convention for maintainer approval. It should cover message style,
  the component-name prefix, field names for recurring concepts (listener identity, job name,
  service role, instance, deadlines, elapsed time, counts), and value formatting (`Display` rather
  than `Debug`).
- Apply the approved convention to lifecycle logs, including the UDP `local_addr` fields left over
  from #2370.
- Review what an operator cannot currently see, then add the approved items. Candidates:
  - effective configuration summary and enabled services at startup;
  - tracker version and instance identity;
  - which shutdown deadline applies;
  - per-component stop outcome and duration;
  - the final process outcome.
- Fix or reclassify misleading lifecycle warnings, such as the eight `Ignoring ... event` warnings
  on a normal startup.

### Out of Scope

- Per-request and hot-path logs (announce, scrape, and connect handling); they are request
  observability, and #1166 covers their levels.
- A general tracing level policy (#1166).
- Metrics, domain events, and log output format or transport (JSON, syslog; see closed #387).
- Changing shutdown behavior or deadlines (EPIC #1488 SI-20).

## Dependencies

Postponed by the maintainer until EPIC #1488 (tracker shutdown overhaul) is complete. Its remaining
subissues (SI-16 to SI-21) still change shutdown behavior and logs, so this work must start from
the final shutdown design. Before T2, recapture the baseline: the capture in
`baseline-lifecycle-logs.md` predates those subissues. The GitHub issue carries the `Postponed`
label, and this spec uses `status: blocked` until the EPIC closes.

## Architectural Decisions

- No existing ADR governs log field naming. If the approved convention is meant to bind future
  code, record it as an ADR or a documented convention in the same change.
- Renamed fields are a visible change for operators whose queries or alerts filter on the old
  names. The convention decision must say whether renames need release notes or a deprecation
  period.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| -- | ------ | ---- | ----------------------- |
| T1 | TODO | Capture baseline lifecycle logs | Initial `info` baseline with the development configuration is in `baseline-lifecycle-logs.md`. Still to capture: `debug` level and a statistics-disabled configuration. |
| T2 | TODO | Inventory lifecycle log lines | One table of every startup and shutdown line: location, level, target, message, fields, formatting, and whether it is duplicated or misleading. |
| T3 | TODO | Propose the lifecycle logging convention and missing-information list | Maintainer decision on names, style, formatting, levels (aligned with #1166), additions, and whether renames need release notes. |
| T4 | TODO | Apply the convention per component | One reviewable change per component (bootstrap and job manager, UDP, HTTP, REST API, health check, jobs). Tests that assert log output are updated. |
| T5 | TODO | Add approved missing information | Only the items the maintainer approved in T3. |
| T6 | TODO | Recapture and compare | Same runs as T1; the before/after comparison shows coherent logs. |
| T7 | TODO | Acceptance and completion review | Independent review; retrospective decision recorded. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T1-T3 | Evidence, inventory, and approved convention | Documentation commits; implementation starts only after the T3 decision. |
| T4 | One component's lifecycle logs | One commit per component, so each rename is reviewable and revertible on its own. |
| T5 | Each approved addition | One commit per addition. |
| T6-T7 | Final evidence and review | Commit with the final evidence. |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/review-tracker-lifecycle-logs/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md`
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-09-30 10:44 UTC - GitHub Copilot - Drafted from the PR #2382 Copilot finding on mixed UDP
  listener field names, at the maintainer's request to review the whole startup and shutdown log
  for coherence and missing operator information. Evidence is from #2370 manual scenario M1.
- 2026-09-30 10:48 UTC - GitHub Copilot - Captured the `info` baseline with the unchanged
  development configuration (every service enabled): exit 0, 215 lines, no `WARN`. Stored verbatim
  in `baseline-lifecycle-logs.md`.
- 2026-09-30 11:03 UTC - GitHub Copilot - Maintainer approved the spec as a standalone issue,
  postponed until EPIC #1488 is complete. Created GitHub issue #2387 with the `task` and
  `Postponed` labels, and moved the spec to `docs/issues/open/`. It merges with PR #2382.

## Acceptance Criteria

- [ ] AC1: A documented, maintainer-approved lifecycle logging convention exists for message style,
  component prefixes, recurring field names, and value formatting.
- [ ] AC2: Every startup and shutdown log line follows the convention; the listener is identified
  by one field name in every component.
- [ ] AC3: Lifecycle logs format values with `Display`, not `Debug`, so no internal type dumps such
  as `CancellationToken { is_cancelled: false }` appear.
- [ ] AC4: A normal startup and a clean shutdown emit no `WARN` or `ERROR` lines, unless the
  convention records why a specific one is correct.
- [ ] AC5: The operator information approved in T3 is logged.
- [ ] AC6: Field renames that affect operators are listed for the release notes.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

### Automatic Checks

- `linter all`
- Tests for every changed component, including tests that assert log output
- Pre-push checks

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| -- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1 | Full lifecycle at `info` | Start the release tracker with every service enabled; confirm UDP and HTTP announces, a REST API call, and the health check; send `SIGTERM` to the binary PID; capture the whole log | Coherent startup and shutdown logs; no unexplained `WARN`; exit 0 | TODO | `manual-verification-evidence.md` M1 |
| M2 | Full lifecycle at `debug` | Same as M1 with `trace_filter = "debug"` | Same field names and style at the higher verbosity | TODO | `manual-verification-evidence.md` M2 |
| M3 | Before/after comparison | Diff the T1 and T6 captures by line category | Every inventory finding is resolved or has a recorded decision | TODO | `manual-verification-evidence.md` M3 |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | TODO | Convention document or ADR |
| AC2 | TODO | T6 capture; inventory |
| AC3 | TODO | T6 capture |
| AC4 | TODO | M1 log |
| AC5 | TODO | M1 log |
| AC6 | TODO | Release-notes list |

## Risks and Trade-offs

- **Operator breakage.** Renamed fields silently break existing queries and alerts. Mitigation:
  list renames for the release notes, as AC6 requires.
- **Scope creep into request logs.** Mitigation: keep to startup and shutdown lines; request logs
  follow #1166.
- **Log-output tests.** Some tests assert exact log text. Mitigation: update them in the same
  per-component commit.

## Implementation Completion Review

- Retrospective: `Not yet assessed`
- Create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` for
  material discoveries; otherwise record why none was needed in the progress log.

## References

- Origin: PR #2382 Copilot review finding F4; issue #2370 (EPIC #1488 SI-15)
- Related: #1166 (tracing level usage); closed #387 (syslog format)
- Evidence source: `docs/issues/closed/2370-1488-si-15-define-udp-active-request-policy/manual-verification-evidence.md` (M1)
