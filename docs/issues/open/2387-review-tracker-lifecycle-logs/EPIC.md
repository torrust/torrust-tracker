---
schema-version: 1
doc-type: epic
status: planned
epic: null
github-issue: 2387
spec-path: docs/issues/open/2387-review-tracker-lifecycle-logs/EPIC.md
epic-owner: null
last-updated-utc: "2026-11-23 16:54"
semantic-links:
  skill-links:
    - create-issue
    - structured-runtime-logging
  related-artifacts:
    - .claude/skills/create-issue/SKILL.md
    - .claude/skills/structured-runtime-logging/SKILL.md
    - docs/issues/open/2387-review-tracker-lifecycle-logs/decisions.md
    - docs/issues/open/2387-review-tracker-lifecycle-logs/baseline-lifecycle-logs.md
    - docs/issues/open/2387-review-tracker-lifecycle-logs/baseline-lifecycle-logs-debug.md
    - docs/issues/open/2387-review-tracker-lifecycle-logs/baseline-lifecycle-logs-statistics-disabled.md
    - docs/issues/open/2766-2758-daemon-json-logging/ISSUE.md
    - docs/issues/open/2758-cli-output-contract-tools-migration/EPIC.md
    - docs/issues/closed/1488-overhaul-tracker-shutdown/EPIC.md
    - src/app.rs
    - src/bootstrap/jobs/manager.rs
    - src/console/ci/e2e/logs_parser.rs
    - tests/common/native_tracker/health.rs
    - packages/axum-server/src/signals.rs
    - packages/udp-server/src/server/launcher.rs
    - "issue #1166"
---

<!-- skill-link: create-issue -->

# EPIC #2387 - Make Tracker Startup and Shutdown Logs Coherent for Operators

## Goal

An operator who starts the tracker and then stops it gets startup and shutdown logs that use one set of targets, one set of field names, one way to write values and one rule for levels, and that say what is running, how each service started, and whether the tracker stopped cleanly. The rules are written down and an automated check keeps the code from drifting.

This EPIC has no parent EPIC. It started as a single issue and became an EPIC on 2026-11-23, when the maintainer answered its open questions and the work proved too large for one pull request.

## Why This Is Needed

The Copilot review of PR #2382 (issue #2370, EPIC #1488 SI-15) found that the new UDP drain logs named the listener `local_addr`, while most of the code uses `service_binding`. The maintainer asked for a review of the whole lifecycle instead of that one field: run the tracker, read what it logs at startup, stop it, read what it logs then, make everything coherent, and look for information an operator is missing.

The captures of a real run at the current `develop` ([baseline-lifecycle-logs.md](baseline-lifecycle-logs.md), [debug](baseline-lifecycle-logs-debug.md), [statistics disabled](baseline-lifecycle-logs-statistics-disabled.md)) show:

| Observation | Example |
| --- | --- |
| The listener is identified five ways | `local_addr`, `service_binding`, `address`, `bind_address`, `bind_addr` |
| Targets mix two styles | `UDP TRACKER:`, `HEALTH CHECK API:`, `METRICS:` beside Rust module paths such as `torrust_tracker_lib::bootstrap::jobs::manager:` |
| Spans print internal `Debug` output on every line of a job | `start_job{… cancellation_token=CancellationToken { is_cancelled: false } request_drain_deadline=5s …}` |
| A start line is false when printed | `Starting on: <url>` is logged after the socket is bound, next to `Started on: <url>` |
| `Started` is logged before the service is registered | The health check cannot see the service yet |
| Durations have four forms | `timeout_seconds=25`, `drain_timeout=15s`, `request_drain_deadline=5s`, `in 15s` inside a message |
| A supported configuration logs `WARN` for every request | With usage statistics disabled, three statistics listeners log `Ignoring … event from an unknown or metrics-disabled listener` for every event |
| Metrics registration floods `INFO` | 28 `METRICS:` lines with `description=Some(MetricDescription(…))` |
| One event spans about 110 lines | `Configuration:` followed by indented JSON |
| Shutdown messages vary | `!! Shutting down … !!`, `Received cancellation request, shutting down …`, `Job completed after cooperative cancellation job=…` |
| Facts are missing | Version and commit, how long the start took, how long each job took to stop, one final line with the outcome |
| Other programs parse the message text | The E2E runner and the test fixture read service URLs from `Started on: …` |

Version 4.0.0 is the next release. It is the moment to change log lines that operators and tools read, because a major release may break them.

## Maintainer Decisions

All content decisions (D1 to D12), with their alternatives, pros, cons and reasons, are in [decisions.md](decisions.md). Summary:

| ID | Decision |
| --- | --- |
| D1 | This EPIC goes before the JSON log format of #2766. The convention does not depend on the output format |
| D2 | Named targets on every lifecycle line, in uppercase with underscores. Adding targets and renaming targets are two tasks. New: `JOB_MANAGER`, `APPLICATION`. `API` becomes `REST_API` |
| D3 | `service_binding` is the only name of a bound listener, as a URL without trailing slash; `bind_address` only before the bind. A local display helper now and a fix in `torrust-net-primitives`. Service jobs are named by role and index |
| D4 | Three state lines for each service: `Starting`, `Bound … socket`, `Started` (after registration). Lines are merged only when nothing can fail between them. The text `Started on:` is removed and its readers use fields |
| D5 | Lifecycle spans carry only `service_role` and `instance_index`. Settings are logged once, on the `Starting` line |
| D6 | A duration is a number with the unit in the field name (`_in_secs`, `_in_ms`); a configured duration uses its configuration key name; a measured one is `elapsed_in_ms` |
| D7 | Metrics registration is fixed in `torrust-metrics` and summarized in one line here. The configuration is one single-line event plus an `Enabled services` summary |
| D8 | The per-request statistics-disabled `WARN` is a bug: silent when disabled, `ERROR` when the listener is unknown, state reported once at startup |
| D9 | One meaning for each level. A clean run has no `WARN` or `ERROR`. Job panic, failure and abort are `ERROR`. Private mode with UDP trackers configured is rejected at startup |
| D10 | New lines: build and run identity at start (`version`, `git_commit`, `build_profile`, `pid`, `run_id`), `Tracker started`, `elapsed_in_ms` on job outcomes, `Tracker stopped` with the outcome. `run_id` on every record comes after #2766 |
| D11 | The rules live in an ADR, in `docs/logging.md`, in the `structured-runtime-logging` skill and in rustdoc, and an automated check enforces them |
| D12 | Clean break in 4.0.0, no deprecation period. `docs/logging.md` carries the list of operator-visible changes |

Process decisions (2026-11-23):

1. **Issue #2387 is the EPIC.** No other GitHub issue is created now. The sub-issues exist as drafts in `docs/issues/drafts/2387-si-*`.
2. **One pull request merges this specification**: the EPIC, the decisions, the baseline and the sub-issue drafts.
3. **A coordinator session runs the EPIC**, started by the `project-liaison` session at the maintainer's request.
4. **One sub-issue at a time.** The coordinator starts one session for one sub-issue and waits for it to finish before starting the next. Sub-issues do not run in parallel.
5. **Each sub-issue session**, in this order: completes its draft, creates its GitHub issue, moves the specification to `docs/issues/open/`, opens a specification-only pull request, and after that is merged implements the issue, opens the implementation pull request and does the close-out.

## Scope

### In Scope

- A logging convention for lifecycle lines: targets, messages, field names, value formats, durations, spans and levels, recorded in an ADR and in `docs/logging.md`.
- Applying the convention to every startup and shutdown line of the tracker: the application layer, the job manager, the UDP trackers, the HTTP trackers, the REST API, the health check API, the event listeners and the periodic jobs.
- The fix of the per-request statistics-disabled warning.
- Moving the programs that parse log text (E2E runner, test fixture, lifecycle tests) to structured fields.
- The new lines of D10.
- The rejection of private mode with UDP trackers configured.
- Two changes outside lifecycle lines that the duration rule needs to be coherent: the `latency_ms` field of request logs and three configuration keys without a unit.
- An automated check of the convention.
- The list of operator-visible changes for 4.0.0.

### Out of Scope

- Levels of request-handling logs (announce, scrape, connect). Issue #1166 covers them.
- The log output format and stream (JSON on stderr). Issue #2766 covers them, after this EPIC.
- Metrics and domain events as such; only their startup log lines are in scope.
- Changing shutdown behavior or deadlines. EPIC #1488 did that and is closed.
- Changes inside `torrust-net-primitives` and `torrust-metrics`. They are other repositories; this EPIC files an issue in each and adapts when they release.
- A project-wide `CHANGELOG`. The maintainer plans it after 4.0.0.

## Subissues

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

The sub-issues are drafts. Each gets its GitHub issue number when its session starts. The `SI` number is the working order, which is what the `si-{N}` segment of a sub-issue folder means in `docs/issues/open/AGENTS.md`. The reasons for the order are in Delivery Strategy.

| SI | Issue | Local Spec | Status | Decisions | Depends on |
| --- | --- | --- | --- | --- | --- |
| 1 | #[To be assigned] - Bug: statistics listeners log a `WARN` for every event when usage statistics are disabled | `docs/issues/drafts/2387-si-1-statistics-disabled-warning-per-event/ISSUE.md` | TODO | D8 | - |
| 2 | #[To be assigned] - Record the logging convention: ADR, `docs/logging.md`, inventory and skill | `docs/issues/drafts/2387-si-2-logging-convention-adr-and-docs/ISSUE.md` | TODO | D9, D11, D12 | - |
| 3 | #[To be assigned] - Reject private mode with UDP trackers configured at startup | `docs/issues/drafts/2387-si-3-reject-private-mode-with-udp-trackers/ISSUE.md` | TODO | D9 | - |
| 4 | #[To be assigned] - Add the unit to the three duration configuration keys that have none | `docs/issues/drafts/2387-si-4-configuration-keys-with-units/ISSUE.md` | TODO | D6 | - |
| 5 | #[To be assigned] - Add named targets to lifecycle lines that have none | `docs/issues/drafts/2387-si-5-add-missing-named-log-targets/ISSUE.md` | TODO | D2 | SI-2 |
| 6 | #[To be assigned] - Rename log targets to underscore style and `API` to `REST_API` | `docs/issues/drafts/2387-si-6-rename-log-targets-to-underscore-style/ISSUE.md` | TODO | D2 | SI-5 |
| 7 | #[To be assigned] - Print `service_binding` without a trailing slash in every log line | `docs/issues/drafts/2387-si-7-service-binding-without-trailing-slash/ISSUE.md` | TODO | D3b, D3c | SI-2 |
| 8 | #[To be assigned] - Identify a listener with one field: remove `local_addr`, `address` and `bind_addr` | `docs/issues/drafts/2387-si-8-single-listener-identity-field/ISSUE.md` | TODO | D3a | SI-7 |
| 9 | #[To be assigned] - Write durations in lifecycle lines as numbers with the unit in the name | `docs/issues/drafts/2387-si-9-duration-fields-in-lifecycle-lines/ISSUE.md` | TODO | D6 | SI-2 |
| 10 | #[To be assigned] - Log three state lines when a service starts | `docs/issues/drafts/2387-si-10-three-state-service-start-lines/ISSUE.md` | TODO | D4 | SI-6, SI-7 |
| 11 | #[To be assigned] - Read service URLs from structured fields and remove `Starting on:` and `Started on:` | `docs/issues/drafts/2387-si-11-read-service-urls-from-structured-fields/ISSUE.md` | TODO | D4 | SI-10 |
| 12 | #[To be assigned] - Keep only the service identity in lifecycle spans | `docs/issues/drafts/2387-si-12-identity-only-lifecycle-spans/ISSUE.md` | TODO | D5 | SI-10 |
| 13 | #[To be assigned] - Name service jobs by role and index and add identity fields to job manager lines | `docs/issues/drafts/2387-si-13-job-names-from-role-and-index/ISSUE.md` | TODO | D3d | SI-5 |
| 14 | #[To be assigned] - Apply the level rule to job manager and shutdown lines | `docs/issues/drafts/2387-si-14-lifecycle-log-levels/ISSUE.md` | TODO | D9 | SI-5 |
| 15 | #[To be assigned] - Log the configuration as one single-line event and add an `Enabled services` line | `docs/issues/drafts/2387-si-15-configuration-as-single-line-event/ISSUE.md` | TODO | D7b | SI-5 |
| 16 | #[To be assigned] - Log the build and run identity as the first line | `docs/issues/drafts/2387-si-16-build-and-run-identity-start-line/ISSUE.md` | TODO | D10 | SI-5 |
| 17 | #[To be assigned] - Log `Tracker started` when every job has started | `docs/issues/drafts/2387-si-17-tracker-started-line/ISSUE.md` | TODO | D10 | SI-16 |
| 18 | #[To be assigned] - Log how long each job took to stop and one final `Tracker stopped` line | `docs/issues/drafts/2387-si-18-job-stop-duration-and-tracker-stopped-line/ISSUE.md` | TODO | D10 | SI-9, SI-14 |
| 19 | #[To be assigned] - Rename `latency_ms` to `latency_in_ms` in request logs | `docs/issues/drafts/2387-si-19-request-log-latency-field/ISSUE.md` | TODO | D6 | SI-9 |
| 20 | #[To be assigned] - Check the lifecycle log convention automatically | `docs/issues/drafts/2387-si-20-automated-lifecycle-log-check/ISSUE.md` | TODO | D11 | SI-1 to SI-19 |
| 21 | #[To be assigned] - Summarize metrics registration in one line | `docs/issues/drafts/2387-si-21-metrics-registration-summary-line/ISSUE.md` | BLOCKED | D7a | A `torrust-metrics` release with the fix |
| 22 | #[To be assigned] - Add `run_id` to every log record | `docs/issues/drafts/2387-si-22-run-id-on-every-record/ISSUE.md` | BLOCKED | D10 | #2766, SI-16 |
| 23 | #[To be assigned] - Close out: recapture, compare, and review the acceptance criteria | `docs/issues/drafts/2387-si-23-close-out/ISSUE.md` | TODO | - | All others |

### External follow-ups (other repositories)

| Repository | Change | Used by | Status |
| --- | --- | --- | --- |
| `torrust/torrust-net-primitives` | `ServiceBinding`'s `Display` prints no trailing slash for any protocol | SI-7 deletes its helper when the fixed crate is used | Filed: [torrust-net-primitives issue 1](https://github.com/torrust/torrust-net-primitives/issues/1) |
| `torrust/torrust-metrics` | `describe_counter` and `describe_gauge` log at `DEBUG`, with `Display` values and no `Option` wrapper | SI-21 | Filed: [torrust-metrics issue 4](https://github.com/torrust/torrust-metrics/issues/4) |

## Delivery Strategy

One sub-issue at a time, each in its own session, each with a specification pull request and then an implementation pull request into `develop`.

### Working order

The sub-issues run in the order of their numbers: `SI-1` to `SI-20`, then `SI-23`. `SI-21` and `SI-22` are blocked; each runs when its blocker is gone.

Reasons for the order:

- **SI-1 first.** It is the only defect that harms a production tracker today, and bugs go before other work. It depends on nothing: the fields it needs already exist.
- **SI-2 second.** Every later sub-issue adds its lines to `docs/logging.md` and its entries to the list of changes, so the documents must exist.
- **SI-3 and SI-4 early.** They change behavior and the configuration schema. An early merge gives the most time before 4.0.0 to find a problem.
- **Targets (SI-5, SI-6) before lines.** The lines that later sub-issues rewrite then already have their final target.
- **SI-7 before SI-8 and SI-10.** The value of `service_binding` has its final form before it is added to more lines.
- **SI-10 then SI-11.** SI-10 only adds the new lines, so no reader breaks. SI-11 moves the readers and deletes the old lines. Between the two, `develop` has both old and new start lines.
- **SI-20 near the end.** A check written before the lines are final would need an exception list that changes in every pull request.

The coordinator may change the order when a sub-issue's specification shows a dependency this table does not have. It records the change in the progress log.

### Blocked sub-issues and the close-out

`SI-21` waits for a release of `torrust-metrics` and `SI-22` waits for #2766. Neither is needed for the others. If one of them is still blocked when `SI-20` is done, the coordinator asks the maintainer whether the EPIC stays open for it or the sub-issue leaves the EPIC and continues alone. `SI-23` runs after that answer.

### What every sub-issue does

Besides its own change, a sub-issue that changes a log line:

1. updates the readers of that line in the same pull request (see the table of readers in [decisions.md](decisions.md));
2. updates the catalogue of lifecycle lines in `docs/logging.md`;
3. adds its operator-visible changes to "Changes in 4.0.0" in `docs/logging.md`;
4. records a capture of the changed lines in its manual verification evidence.

A sub-issue changes only what its decision says. A problem it finds outside that is reported to the coordinator and becomes a new sub-issue or a separate issue.

For each subissue implementation in this EPIC, the default completion policy is:

1. Run automatic checks (`linter all`, relevant tests, pre-push checks when applicable).
2. Run manual verification scenarios and record evidence.
3. Re-review acceptance criteria after implementation and update verification evidence.
4. Complete an evidence-based implementation review. Create or update an issue-local retrospective for reusable lessons, material design changes, or meaningful deviations from the plan; otherwise record why one was unnecessary in the issue progress log.

### Phase 1: the defect and the rules

- Outcome: SI-1 to SI-4 merged.
- Exit criteria: a tracker with statistics disabled logs no `WARN` for a request; the ADR is accepted; `docs/logging.md` exists with the glossary, the targets, the fields, the levels and an empty catalogue ready to fill.

### Phase 2: targets and identity

- Outcome: SI-5 to SI-8 merged.
- Exit criteria: no lifecycle line has a module path as target; a bound listener is named only by `service_binding`, without a trailing slash.

### Phase 3: the lines

- Outcome: SI-9 to SI-19 merged.
- Exit criteria: no program reads a service URL from message text; a clean run has no `WARN`, no `ERROR` and no `Debug` output of an internal type.

### Phase 4: enforcement and acceptance

- Outcome: SI-20 merged; SI-21 and SI-22 merged or detached by the maintainer; SI-23 done.
- Exit criteria: the check runs in CI; the recapture is compared with the baseline line by line and every difference is in "Changes in 4.0.0".

## Progress Tracking

### Workflow Checkpoints

- [x] Issue spec drafted in `docs/issues/drafts/review-tracker-lifecycle-logs/ISSUE.md` and approved as a standalone issue (2026-09-30)
- [x] GitHub issue #2387 created and issue number added to this spec
- [x] Open questions answered by the maintainer and recorded in `decisions.md` (2026-11-23)
- [x] Converted to an EPIC; sub-issue drafts written in `docs/issues/drafts/`
- [ ] EPIC specification pull request reviewed and merged
- [ ] Subissues created and linked in this spec, one at a time as each starts
- [ ] Subissue statuses kept up to date in the `Subissues` table
- [ ] For each implemented subissue: automatic checks completed and recorded
- [ ] For each implemented subissue: manual verification completed and recorded
- [ ] For each implemented subissue: acceptance criteria reviewed post-implementation
- [ ] For each implemented subissue: implementation completion review recorded
- [ ] Epic acceptance criteria reviewed and checked off
- [ ] Epic issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

Append one line per meaningful update. When an AI agent did the work, name its model after the agent, for example `GitHub Copilot (Claude Opus 5.5)`, so later reviews can relate outcomes to the model that produced them.

- 2026-09-30 10:44 UTC - GitHub Copilot - Drafted from the PR #2382 Copilot finding on mixed UDP listener field names, at the maintainer's request to review the whole startup and shutdown log for coherence and missing operator information. Evidence is from #2370 manual scenario M1.
- 2026-09-30 10:48 UTC - GitHub Copilot - Captured the `info` baseline with the unchanged development configuration (every service enabled): exit 0, 215 lines, no `WARN`. Stored verbatim in `baseline-lifecycle-logs.md`.
- 2026-09-30 11:03 UTC - GitHub Copilot - Maintainer approved the spec as a standalone issue, postponed until EPIC #1488 is complete. Created GitHub issue #2387 with the `task` and `Postponed` labels, and moved the spec to `docs/issues/open/`. It merges with PR #2382.
- 2026-11-21 13:29 UTC - Claude Code (Fable 5.2) - EPIC #1488 closed on 2026-11-19 and the maintainer removed the `Postponed` label. Captured the lifecycle logs at `develop` commit `4be22ee66` at `info`, at `debug` and with statistics disabled, and found the programs that parse log text.
- 2026-11-23 16:05 UTC - Claude Code (Fable 5.2) - The maintainer answered the twelve open questions (2026-11-21 and 2026-11-23); recorded them in `decisions.md` with alternatives, pros, cons and reasons. Recaptured the three logs at `develop` commit `ff3600f10` (same line counts as on 2026-11-21) and replaced the 2026-09-30 baseline.
- 2026-11-23 16:20 UTC - Claude Code (Fable 5.2) - The maintainer approved the split into 23 sub-issues and the process (issue #2387 is the EPIC; drafts only for the sub-issues; a coordinator runs one sub-issue session at a time). Converted the specification from `ISSUE.md` to `EPIC.md`, set the status from `blocked` to `planned`, and wrote the 23 drafts. Updated the specification of #2766 and its row in EPIC #2758 to say that #2766 starts after this EPIC.
- 2026-11-23 16:22 UTC - Claude Code (Fable 5.2) - Filed the two external issues as `josecelano-bot`: `torrust/torrust-net-primitives` issue 1 (`ServiceBinding` display without trailing slash) and `torrust/torrust-metrics` issue 4 (level and format of the metric description logs).
- 2026-11-23 16:52 UTC - Claude Code (Fable 5.2) - Renumbered the sub-issues by working order (review finding F4 of PR 3010): the `si-{N}` segment of a sub-issue folder means the implementation order. The old numbers 2, 1, 15, 12, 3, 4, 5, 6, 10, 7, 8, 9, 13, 14, 17, 18, 19, 20, 11, 21, 16, 22, 23 are now 1 to 23 in that sequence. The draft folders, the decisions and the specification of #2766 use the new numbers.

## Acceptance Criteria

- [ ] AC1: All required subissues are created and linked.
- [ ] AC2: Implementation order is explicit and justified.
- [ ] AC3: Dependencies and blockers are documented and current.
- [ ] AC4: Epic status reflects actual state of linked subissues.
- [ ] AC5: Every completed subissue includes automated verification evidence.
- [ ] AC6: Every completed subissue includes manual verification evidence.
- [ ] AC7: Every completed subissue includes post-implementation acceptance criteria review.
- [ ] AC8: Every completed subissue includes an implementation completion review.
- [ ] AC9: Documentation and governance updates are included when required.
- [ ] AC10: An accepted ADR and `docs/logging.md` state the lifecycle logging convention: targets, messages, field names, value formats, durations, spans and levels.
- [ ] AC11: Every startup and shutdown line follows the convention. A bound listener is identified by `service_binding` in every component, and every lifecycle line has a named target.
- [ ] AC12: Lifecycle lines print no `Debug` output of an internal type, such as `CancellationToken { is_cancelled: false }` or `Some(…)`.
- [ ] AC13: A clean start and a clean stop log no `WARN` and no `ERROR`, with usage statistics enabled and disabled, except the warnings the convention lists.
- [ ] AC14: The lines of D10 are logged: build and run identity, `Tracker started`, the stop duration of each job, and `Tracker stopped` with the outcome.
- [ ] AC15: No program in the repository reads a fact from the message text of a log line.
- [ ] AC16: "Changes in 4.0.0" in `docs/logging.md` lists every operator-visible change, and the comparison of SI-23 finds no change that is missing from it.
- [ ] AC17: The automated check of SI-20 runs in CI and fails for a line that breaks the convention.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| --- | --- | --- |
| AC1 | TODO | {issue/spec/PR links} |
| AC2 | TODO | {issue/spec/PR links} |
| AC3 | TODO | {issue/spec/PR links} |
| AC4 | TODO | {issue/spec/PR links} |
| AC5 | TODO | {issue/spec/PR links} |
| AC6 | TODO | {issue/spec/PR links} |
| AC7 | TODO | {issue/spec/PR links} |
| AC8 | TODO | {issue/spec/PR links} |
| AC9 | TODO | {issue/spec/PR links} |
| AC10 | TODO | ADR and `docs/logging.md` (SI-2) |
| AC11 | TODO | SI-23 recapture; SI-20 check |
| AC12 | TODO | SI-23 recapture; SI-20 check |
| AC13 | TODO | SI-23 recapture; SI-1 regression test; SI-20 check |
| AC14 | TODO | SI-23 recapture |
| AC15 | TODO | SI-11; search recorded in SI-23 |
| AC16 | TODO | SI-23 comparison |
| AC17 | TODO | SI-20 pull request and CI run |

## Risks and Trade-offs

- **Operators' parsers break.** Renamed targets and fields silently break queries and alerts. Mitigation: the break is in a major release, and "Changes in 4.0.0" lists each change (D12).
- **The repository's own readers break.** The E2E runner, the test fixture and the lifecycle tests read log text. Mitigation: each sub-issue updates its readers in the same pull request, and SI-10 and SI-11 are split so the readers move in a pull request of their own.
- **Many small pull requests touch the same files.** Mitigation: one sub-issue at a time, each cut from the current `develop`.
- **Conflict with #2766.** Both rewrite the log parsers. Mitigation: D1 orders them, and both specifications say so.
- **Other repositories release late.** SI-21 and SI-22 may wait a long time. Mitigation: they block nothing else, and the maintainer decides at the end whether they stay in the EPIC.
- **Scope grows into request logs.** Mitigation: only `latency_ms` is in scope (SI-19); levels of request logs stay in #1166.
- **A decision does not survive the code.** The start sequences of the REST API and the health check API were not traced line by line. Mitigation: SI-2's inventory verifies them before SI-10, and a sub-issue stops and asks when a decision does not hold.
- **Behaviour changes hidden in a log EPIC.** SI-4 changes the configuration schema and SI-3 rejects a configuration that starts today. Mitigation: each is its own sub-issue with its own specification review, and SI-3 has its own ADR.

## References

- Origin: PR #2382 Copilot review finding F4; issue #2370 (EPIC #1488 SI-15)
- Related issues: #1166 (tracing level usage in request handling), #2766 (daemon JSON logs, after this EPIC), #2758 (EPIC of #2766), #1488 (shutdown overhaul, closed), closed #387 (syslog format)
- Decisions: [decisions.md](decisions.md)
- Baseline: [baseline-lifecycle-logs.md](baseline-lifecycle-logs.md), [baseline-lifecycle-logs-debug.md](baseline-lifecycle-logs-debug.md), [baseline-lifecycle-logs-statistics-disabled.md](baseline-lifecycle-logs-statistics-disabled.md)
- Related ADRs: the logging ADR created by SI-2; the private-mode ADR created by SI-3
- Skill: `.claude/skills/structured-runtime-logging/SKILL.md`
