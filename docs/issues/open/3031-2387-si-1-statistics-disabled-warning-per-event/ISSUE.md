---
schema-version: 1
doc-type: issue
issue-type: bug
status: planned
priority: p1
epic: 2387
github-issue: 3031
spec-path: docs/issues/open/3031-2387-si-1-statistics-disabled-warning-per-event/ISSUE.md
branch: "3031-2387-si-1-statistics-disabled-warning-per-event-spec"
related-pr: null
last-updated-utc: "2026-11-24 16:20"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/open/2387-review-tracker-lifecycle-logs/EPIC.md
    - docs/issues/open/2387-review-tracker-lifecycle-logs/decisions.md
    - .claude/skills/create-issue/SKILL.md
    - packages/udp-server/src/statistics/event/listener.rs
    - packages/udp-core/src/statistics/event/listener.rs
    - packages/http-core/src/statistics/event/listener.rs
    - .claude/skills/fix-bug/SKILL.md
---

<!-- skill-link: create-issue -->

# Issue #3031 - Bug: statistics listeners log a `WARN` for every event when usage statistics are disabled

**Parent EPIC:** #2387 - Make Tracker Startup and Shutdown Logs Coherent for Operators. Sub-issue SI-1 of 23.

**Approved by the maintainer on 2026-11-24.** The GitHub issue is #3031. This specification is added by a spec-only pull request; the implementation follows in a separate pull request after it is merged.

- Implements: D8, in [decisions.md](../2387-review-tracker-lifecycle-logs/decisions.md).
- Depends on: Nothing.
- Working order and rules common to all sub-issues: [EPIC.md](../2387-review-tracker-lifecycle-logs/EPIC.md), section Delivery Strategy.
- Evidence of today's behavior: [baseline-lifecycle-logs.md](../2387-review-tracker-lifecycle-logs/baseline-lifecycle-logs.md) and the two captures beside it.

## Goal

A tracker whose UDP or HTTP tracker sections have `tracker_usage_statistics = false` (the default of that per-tracker key) logs nothing for each event it does not count. An event from a listener the statistics policy does not know is an `ERROR`. The operator sees once, at startup, for which services statistics are disabled.

## Background

Three statistics event listeners share one branch for two cases: the service has statistics disabled (a supported configuration) and the service is unknown (a bug). Both log `WARN … Ignoring … event from an unknown or metrics-disabled listener configuration_instance_id=ConfigurationInstanceId { … }`. A UDP request publishes one to four events and an HTTP announce or scrape publishes one, so a tracker that serves traffic with statistics disabled writes a `WARN` for every request. The baseline capture with statistics disabled shows eight of them for one health check request. The sites are `packages/udp-server/src/statistics/event/listener.rs`, `packages/udp-core/src/statistics/event/listener.rs` and `packages/http-core/src/statistics/event/listener.rs`.

## Scope

### In Scope

- In the three listeners, split the policy lookup into its three results: enabled (count the event), disabled (no log; `TRACE` at most), unknown (`ERROR` with `service_role` and `instance_index` as fields, not a `Debug` struct).
- Report the state once at startup: the statistics listener's start line says how many service instances have statistics enabled and disabled (for example `enabled_instances=0 disabled_instances=2`). The `Starting` line of each UDP and HTTP tracker already has `tracker_usage_statistics` (`src/bootstrap/jobs/udp_tracker.rs:65-69`, `src/bootstrap/jobs/http_tracker.rs:70-74`).
- A regression test: a tracker with statistics disabled handles a UDP request and an HTTP request and logs no `WARN` and no `ERROR`.
- A test for the unknown-listener case at the unit boundary of each listener (see Regression Test Strategy: the code is the same in three packages).
- This issue runs before SI-2, which creates `docs/logging.md`. It lists its operator-visible changes in its own specification, and SI-2 copies them into "Changes in 4.0.0". If SI-2 is already merged when this issue runs, update `docs/logging.md` directly.

### Out of Scope

- Renaming the targets of these lines (SI-6).
- Stopping the publication of events when no listener counts them. The draft `docs/issues/drafts/optimize-event-publication-without-consumers` covers that.
- Any other `WARN` line.

## Findings of the coordinator's check (2026-11-24)

The EPIC coordinator compared this draft with the code at `develop` `6a058dd4d` on 2026-11-24. Nothing changed since the draft was written; these are cases the draft did not name. The session that completes the draft checks each one and decides how its specification uses it.

- The per-tracker `tracker_usage_statistics` defaults to `false` (`packages/configuration/src/v3_0_0/udp_tracker.rs:63`, `http_tracker.rs:74`). `share/default/config/tracker.container.no-persistence.toml` and `tracker.e2e.container.sqlite3.toml` omit the key in their tracker sections, so they log the `WARN` for every request today. `core.tracker_usage_statistics` (default `true`) is a different key and does not feed this policy.
- Sites: `packages/udp-server/src/statistics/event/listener.rs:126-133`, `packages/udp-core/src/statistics/event/listener.rs:126-132`, `packages/http-core/src/statistics/event/listener.rs:129-135`. No other listener has a policy branch. The drain path calls the same function.
- The native tracker fixture configures one HTTP tracker and no UDP tracker (`tests/common/native_tracker/command.rs:29-31`) and exposes only the HTTP tracker address (`mod.rs:175`). The UDP half of the regression test needs fixture work, or another harness that captures the log.
- Under order B′ (D1) #2766 comes after this issue: the regression test reads today's text output.

### Result of the check (2026-11-24, `torrust/develop` `23095e3a6`)

| Case | Result |
| --- | --- |
| Default `false` of the per-tracker key | Confirmed. `tracker.container.no-persistence.toml` sets only `core.tracker_usage_statistics = true` (a different key, not the policy) and declares no tracker section, so it runs no UDP or HTTP tracker. `tracker.e2e.container.sqlite3.toml` declares one UDP and one HTTP tracker without the key: both log the `WARN` per request. `tracker.container.sqlite3.toml` has its tracker sections commented out. `tracker.udp.benchmarking.toml` sets `false` explicitly. |
| Sites | Confirmed, with one difference: the three branches are not alike. `udp-server` tests `== Some(true)`; `udp-core` and `http-core` use `.copied().unwrap_or(false)`. Only the last two cannot tell a missing entry from `false`, and all three log the same `WARN` with `configuration_instance_id` as a `Debug` struct. The drain path (`drain_events`) calls the same function as the live path: one fix covers both. |
| Who builds the policy | `src/bootstrap/jobs/{udp_tracker_server,udp_tracker_core,http_tracker_core}.rs` build a `BTreeMap<ConfigurationInstanceId, bool>` from the instance containers, one entry per configured instance of the role. An event from a configured instance therefore always finds its entry: `None` is reachable only by a wiring bug. No other listener has a policy branch. |
| Test fixture | Confirmed. `tests/common/native_tracker/command.rs` already sets `tracker_usage_statistics = false` for its one HTTP tracker and has no UDP tracker. The fixture finds the HTTP address by parsing the `Started on:` line (`health.rs`, `parse_started_address`); SI-11 will move readers to structured fields, so the UDP addition must reuse the same parser and be easy to switch. |
| Order B′ | Understood. The test reads today's text output, with ANSI codes stripped (`NativeTrackerExit::plain_output`). |

## Detailed Design

### Behavior after the fix, in each of the three listeners

| Policy lookup | Event counted | Log |
| --- | --- | --- |
| `Some(true)` | yes | none (unchanged) |
| `Some(false)` | no | none at `INFO` or above; `TRACE` only |
| `None` | no | one `ERROR` per event with `service_role` and `instance_index` as fields |

Exact messages (the three keep their existing log targets; target renames are SI-6):

- `TRACE`, disabled: `Ignoring event: usage statistics are disabled for this instance`, fields `service_role`, `instance_index`. Variant per listener: `UDP server event`, `UDP tracker event`, `HTTP tracker event`, as today.
- `ERROR`, unknown: `Received an event from an instance that has no statistics policy entry`, with the same fields. The `Debug` struct `configuration_instance_id=ConfigurationInstanceId { … }` goes away. `service_role` is written with `Display` (`udp_tracker`, `http_tracker`), `instance_index` as an integer.
- Start line of each of the three listeners (today `Starting UDP tracker server event listener`, `Starting UDP tracker core event listener`, `Starting HTTP tracker core event listener`) keeps its text and gains the fields `enabled_instances=N disabled_instances=M`, counted from the policy map the listener receives. D8 names "the statistics listener's start line"; there are three statistics listeners with three policy maps, so the proposal is to put the counts on all three (a UDP-server and a UDP-core line give the same numbers; that is the cost of reporting where the policy is held). The maintainer may prefer only the `udp-server` one: say so at review.
- `tracker_usage_statistics` on each service's `Starting` line already exists; no change.

The three-way decision is written once per listener as a `match` on `metrics_policy.get(&id).copied()`; no shared type. A shared helper would need a new dependency edge between the three packages or a type in `primitives`, for about ten lines.

### Files

| File | Change |
| --- | --- |
| `packages/udp-server/src/statistics/event/listener.rs` | `handle_event_if_enabled`: three arms; start line counts; unit tests |
| `packages/udp-core/src/statistics/event/listener.rs` | `handle_received_event`: three arms; start line counts; unit tests |
| `packages/http-core/src/statistics/event/listener.rs` | `handle_received_event`: three arms; start line counts; unit tests |
| `tests/common/native_tracker/command.rs`, `mod.rs` | A `[[udp_trackers]]` entry with `tracker_usage_statistics = false` and discovery of its address; a `plain_output` accessor if missing |
| `tests/metrics/` (new file) | The tracker-level regression test, new test binary for the statistics-disabled case |
| `docs/issues/.../manual-verification-evidence.md` | M1 to M3 evidence |

Production code outside the three listeners does not change.

### Operator-visible changes (for SI-2, "Changes in 4.0.0")

1. With per-tracker `tracker_usage_statistics = false` (the default), the tracker no longer logs `WARN … Ignoring … event from an unknown or metrics-disabled listener` for every request.
2. An event from a listener with no policy entry is now an `ERROR` with `service_role` and `instance_index` fields, not a `WARN` with a `Debug` struct.
3. The start line of the UDP server, UDP core and HTTP core statistics listeners carries `enabled_instances` and `disabled_instances`.

If SI-2 has merged when this issue is implemented, `docs/logging.md` is updated directly instead.

### Test boundaries

- **Unit, per listener** (`write-unit-test` skill): capture log records with a local `tracing_subscriber` `MakeWriter` set by `tracing::subscriber::set_default` (the `packages/*` crates depend on `tracing` only, so `tracing-subscriber` becomes a `dev-dependency` of the three crates; `test-helpers` is not used, its buffer is global and level `WARN` only). Cases: enabled counts and logs nothing; disabled counts nothing and logs nothing at `INFO` or above; unknown counts nothing and logs one `ERROR` with both fields; start line shows the counts for a map with enabled, disabled and no entries. The existing test `it_should_ignore_disabled_and_unknown_events_before_processing_an_enabled_event` stays.
- **Tracker level** (the real binary through `tests/common/native_tracker/`): configuration with one UDP and one HTTP tracker, both statistics disabled; one UDP announce (`tests/common/udp_requests.rs`) and one HTTP announce (`http_requests.rs`); graceful shutdown; assert that the plain output has no line with `WARN` or `ERROR` and that the start lines show `disabled_instances=1` (HTTP) and `disabled_instances=1` (UDP). Red run on `develop`: the `WARN` count is greater than zero, recorded in T2. In-process `TrackerApplicationFixture` is not used: `test-helpers` captures `WARN` and above into one global buffer shared by tests of the same binary, which makes "no `WARN`" unreliable.

### Risks added by the check

- The `ERROR` fires per event, so a real wiring bug floods the log. D8 asks for `ERROR` per event; the bug is rare and must be loud. A once-per-instance limiter is not added; say so at review if the maintainer wants it.
- `udp-core`'s and `http-core`'s `.unwrap_or(false)` hid the difference for as long as the code existed; the fix makes the unknown case reachable in unit tests only (the real wiring never produces it).

### Follow-ups found (not fixed here)

- `tracing::debug!(target = ...)` in `packages/udp-core/src/services/announce.rs:178` and `scrape.rs:140`: has its own issue (maintainer's decision).
- The `Lagged` `WARN` of the three listeners is a different case (a real loss of events) and is untouched.

## Architectural Decisions

- Related decisions: D8 of the EPIC. After SI-2 is merged, the logging ADR it creates governs this work.
- ADRs to create: none known. The level rule itself is recorded by SI-2.

During implementation, stop and create an ADR when a decision affects project architecture or design patterns, selects an approach among meaningful alternatives, or has consequences future contributors need to understand. If a decision of the EPIC does not hold in the code, stop, record the finding and ask the maintainer.

## Design and Ownership Review

Not applicable.

## Bug-Fix Process

Use `.claude/skills/fix-bug/SKILL.md`. Sequence: analysis (done in the EPIC: one branch for two cases); reproduction with the real binary (T1); regression-test boundary selection; red test (T2); production fix (T3, T4); green test and the like-for-like recheck (T6).

## Regression Test Strategy

Two boundaries. A unit test in each listener for the three results of the policy lookup, because the defect is in that branch. One test at the tracker level that starts a statistics-disabled tracker, sends a request and asserts on the captured log, because the defect an operator sees is the log line; the specification pull request names the existing harness it uses (the native tracker fixture in `tests/common/native_tracker/`).

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | TODO | Reproduce with the real binary | Capture of a statistics-disabled tracker that serves one UDP announce and one HTTP announce, showing the `WARN` lines, in `manual-verification-evidence.md` |
| T2 | TODO | Write the failing regression test | Fixture gains a UDP tracker; tracker-level test and unit tests fail on `develop`; red output recorded |
| T3 | TODO | Fix the three listeners | Three results, each handled in its own branch |
| T4 | TODO | Report the state at startup | Start line of the three listeners carries `enabled_instances` and `disabled_instances` |
| T5 | TODO | Test the unknown-listener case | A unit test asserts the `ERROR` and its fields |
| T6 | TODO | Recheck with the real binary | Same scenario as T1: no `WARN`, no `ERROR` |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T1 | Reproduce with the real binary | Commit after focused validation and review. |
| T2 | Write the failing regression test | Commit after focused validation and review. |
| T3 | Fix the three listeners | Commit after focused validation and review. |
| T4 | Report the state at startup | Commit after focused validation and review. |
| T5 | Test the unknown-listener case | Commit after focused validation and review. |
| T6 | Recheck with the real binary | Commit after focused validation and review. |

For test-producing work, use the `write-unit-test` skill and complete the design review it requires after each passing test increment. Use a Conventional Commit message with the narrow affected scope, and sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/open/3031-2387-si-1-statistics-disabled-warning-per-event/ISSUE.md`
- [x] Draft completed by the sub-issue session
- [x] Draft approved by the maintainer
- [x] GitHub issue created and issue number added to this spec
- [ ] Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

Append one line per meaningful update. When an AI agent did the work, name its model after the agent.

- 2026-11-23 16:16 UTC - Claude Code (Fable 5.2) - Drafted with EPIC #2387 from the maintainer's decisions of 2026-11-21 and 2026-11-23. Not approved as a final specification; no GitHub issue created.
- 2026-11-23 16:53 UTC - Claude Code (Fable 5.2) - Renumbered the sub-issues by working order after review round 1 of PR 3010 (finding F4); this draft's number and its references to other sub-issues follow the new numbers.
- 2026-11-23 17:29 UTC - Claude Code (Fable 5.2) - Corrected after the CodeRabbit review of PR 3010 (audit findings F15 to F22): this draft now names the maintainer's approval before the GitHub issue is created; the drafts of SI-4, SI-8, SI-9, SI-13 and SI-23 have one more correction each.
- 2026-11-24 12:46 UTC - Claude Code (Opus 5.5), EPIC coordinator - Compared with the code at `develop` `6a058dd4d`: corrected the claims the code contradicts, recorded the maintainer's answers of 2026-11-24 that concern this draft, and added "Findings of the coordinator's check" (order B′ of D1 places #2766 after SI-1 and before SI-2).
- 2026-11-24 16:00 UTC - Claude Code (Sonnet 5.5), SI-1 session - Completed the draft against `torrust/develop` `23095e3a6`: checked each case of the coordinator, added the check results, the detailed design, the file list, the exact messages, the test boundaries and the operator-visible changes for SI-2. Waiting for the maintainer's approval; no GitHub issue yet.
- 2026-11-24 16:20 UTC - Claude Code (Sonnet 5.5), SI-1 session - The maintainer approved the completed draft in the session. Created issue #3031 (label `bug`, "Part of EPIC #2387" in the body; the bot account cannot add it as a native sub-issue of #2387, the maintainer links it). Moved the specification to `docs/issues/open/3031-2387-si-1-statistics-disabled-warning-per-event/`.

## Acceptance Criteria

- [ ] AC1: With statistics disabled, handling UDP and HTTP requests logs no `WARN` and no `ERROR`.
- [ ] AC2: An event from an unknown listener logs one `ERROR` with `service_role` and `instance_index` fields.
- [ ] AC3: The startup log says for how many instances statistics are enabled and disabled.
- [ ] AC4: With statistics enabled, the counted metrics are the same as before the change.
- [ ] AC5: The regression test fails without the fix and passes with it.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

### Automatic Checks

- `linter all`
- Relevant tests for changed components, including tests that assert log output
- Pre-push checks

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| M1 | Reproduce the defect | Start the release tracker with `tracker_usage_statistics = false` in every tracker; send one UDP announce and one HTTP announce with the tracker client; stop it with `SIGTERM`; count the `WARN` lines | Before the fix: one or more `WARN` for each request | TODO | `manual-verification-evidence.md` section V1 |
| M2 | Recheck after the fix | Same steps on the fixed binary | No `WARN`, no `ERROR`; the start lines show the disabled state | TODO | `manual-verification-evidence.md` section V2 |
| M3 | Statistics enabled | Same steps with the unchanged development configuration; read the metrics from the REST API | Counters increase as before the change | TODO | `manual-verification-evidence.md` section V3 |

Notes:

- Manual verification is mandatory even when automated tests pass. Record the actual commands, output and relevant tracker logs in `manual-verification-evidence.md`, created from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`.
- Every recorded command result names the toolchain or runtime that produced it when one can affect behavior.
- The capture steps are the ones of the baseline: start the release binary, poll the health check, wait one second, send `SIGTERM`, strip ANSI codes.

### Disposable Verification Scripts

None planned. If one is needed, record its path, purpose and removal owner here before creating it.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| --- | --- | --- |
| AC1 | TODO | {test/log/PR link} |
| AC2 | TODO | {test/log/PR link} |
| AC3 | TODO | {test/log/PR link} |
| AC4 | TODO | {test/log/PR link} |
| AC5 | TODO | {test/log/PR link} |

## Risks and Trade-offs

- The unknown-listener `ERROR` fires in a case that is normal and was hidden by the shared branch. Mitigation: the manual scenarios run every service; any `ERROR` found is reported before merging.
- A test that asserts the absence of a log line passes for the wrong reason. Mitigation: the red run of T2 proves the test can fail.

## Implementation Completion Review

After implementation, compare the result with this specification. Record invalidated assumptions, material design changes, unexpected validation findings, and reusable lessons.

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this issue specification's directory.
- If no retrospective is needed, add a concise progress-log entry explaining why the work had no material discovery.
- Send a short final report to the EPIC coordinator: the pull request links and anything that affects another sub-issue.

## References

- Parent EPIC: #2387, [EPIC.md](../2387-review-tracker-lifecycle-logs/EPIC.md)
- Decisions: [decisions.md](../2387-review-tracker-lifecycle-logs/decisions.md) (D8)
- Baseline: [baseline-lifecycle-logs.md](../2387-review-tracker-lifecycle-logs/baseline-lifecycle-logs.md)
