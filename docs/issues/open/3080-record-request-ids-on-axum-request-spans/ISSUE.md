---
schema-version: 1
doc-type: issue
issue-type: bug
status: in-review
priority: p2
epic: null
github-issue: 3080
spec-path: docs/issues/open/3080-record-request-ids-on-axum-request-spans/ISSUE.md
branch: "3080-record-request-ids-on-axum-request-spans"
related-pr: 3084
last-updated-utc: "2026-11-28 03:17"
semantic-links:
  skill-links:
    - create-issue
    - fix-bug
  related-artifacts:
    - .claude/skills/fix-bug/SKILL.md
    - packages/axum-server/src/request_log.rs
    - packages/axum-http-server/src/v1/routes.rs
    - packages/axum-rest-api-server/src/routes.rs
    - packages/axum-health-check-api-server/src/server.rs
    - packages/udp-server/src/handlers/mod.rs
---

<!-- skill-link: create-issue -->

# Issue #3080 - Record request IDs on axum request spans

## Goal

All three Axum servers (`axum-http-server`, `axum-rest-api-server`, and `axum-health-check-api-server`) record `request_id` on their `request` tracing spans so every log event emitted inside a request span carries the request ID in its span context.

## Background

**What happens, step by step.** Each of the three Axum servers builds its HTTP `TraceLayer` span with `RedactedMakeSpan` from `packages/axum-server/src/request_log.rs` and calls `span.record("request_id", request_id)` in both `on_request` and `on_response` (six call sites across `packages/axum-http-server/src/v1/routes.rs`, `packages/axum-rest-api-server/src/routes.rs`, and `packages/axum-health-check-api-server/src/server.rs`). However, `RedactedMakeSpan::make_span` only declared `method`, `uri`, and `version` in `tracing::span!` and omitted `request_id`. In `tracing`, `Span::record` silently ignores fields that were not declared when the span was created.

**Expectation violated.** Unlike `packages/udp-server/src/handlers/mod.rs` (which declares `#[instrument(fields(request_id), ...)]` before calling `Span::current().record("request_id", ...)`), the Axum request spans never recorded `request_id`.

**Impact.** The explicit `request` and `response` events emitted by `TraceLayer` already carried `%request_id` as event fields, so existing contract tests checking those events passed. However, `on_failure` events (`"response failed"`) and all log events emitted by downstream handlers or core services inside the request future lacked `request_id` in their `request{...}` span context, preventing correlation by `request_id` across inner handler logs.

## Scope

### In Scope

- Declare `request_id = Empty` (using `tracing::field::Empty`) on the `tracing::span!` in `RedactedMakeSpan::make_span` in `packages/axum-server/src/request_log.rs`.
- Add a maintained unit regression test in `packages/axum-server/src/request_log.rs` verifying that `span.record("request_id", ...)` populates `request_id` in the `request{...}` span context for inner events.
- Record real-artifact reproduction, red/green test runs, and final like-for-like recheck in `manual-verification-evidence.md`.

### Out of Scope

- Refactoring the six existing `span.record("request_id", ...)` call sites across the three Axum server crates (`on_request` and `on_response`).

## Architectural Decisions

- Related ADRs: none known.
- ADRs to create: None known.

## Design and Ownership Review

Not applicable.

## Bug-Fix Process

Follows `.claude/skills/fix-bug/SKILL.md`:

- **Analysis:** `RedactedMakeSpan::make_span` omitted `request_id` from the `tracing::span!` field list, causing all six `span.record("request_id", ...)` calls to be no-ops.
- **Reproduction:** Executed the `torrust-tracker` binary in Docker (`rust:1-bookworm`, `rustc 1.100.0`) with `trace_filter = "debug"` and `x-request-id` headers sent to REST API `/api/v1/torrents`, HTTP Tracker `/announce`, and Health Check API `/health_check`. Outcome: **Reproduced** (`request{...}` span lacked `request_id` on handler `DEBUG` events).
- **Regression-test boundary:** Unit test in `packages/axum-server/src/request_log.rs` (`mod tests`).
- **Red test:** Observed `it_should_record_the_request_id_in_the_span_of_a_request` fail on the unfixed `RedactedMakeSpan`.
- **Fix:** Added `request_id = Empty` to `RedactedMakeSpan::make_span`.
- **Green and recheck:** Re-ran unit, downstream Axum server, Clippy, and `rustfmt` checks, then repeated the real-artifact `torrust-tracker` reproduction like-for-like.

## Regression Test Strategy

- **Boundary:** Unit test `it_should_record_the_request_id_in_the_span_of_a_request` in `packages/axum-server/src/request_log.rs` using `torrust_tracker_test_helpers::logging::capture_logs_in_scope()`.
- **Why this boundary:** `RedactedMakeSpan::make_span` is the causal seam where the request span schema is defined; testing directly against `RedactedMakeSpan` + `span.record("request_id", ...)` is fast, isolated, and deterministic without spawning listeners or databases.
- **Evidence:** Red/green runs and before/after real-artifact outputs are recorded in `manual-verification-evidence.md`.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                                                    | Notes / Expected Output                                                                                                                                                                                                                                   |
| --- | ------ | --------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | DONE   | Reproduce the missing `request_id` on real `torrust-tracker`                            | Recorded in `manual-verification-evidence.md` section V1 (before)                                                                                                                                                                                         |
| T2  | DONE   | Add `request_log` regression test (`write-unit-test` skill) and verify red run          | Added `it_should_record_the_request_id_in_the_span_of_a_request` following `write-unit-test` (causal initial state without unused headers), observed the red run fail on unfixed `RedactedMakeSpan` (re-recorded via mutate-then-restore after PR #3084 review cleanup), and recorded the prose-first Arrange-Act-Assert design review in `manual-verification-evidence.md` (causal-state, fixture-ownership, and visible Act/expected-result checks); pending maintainer review stays open under Workflow Checkpoints |
| T3  | DONE   | Declare `request_id = Empty` in `RedactedMakeSpan::make_span`                           | Regression test and all Axum server tests pass                                                                                                                                                                                                            |
| T4  | DONE   | Repeat real-artifact reproduction like-for-like and run checks                          | Recorded in `manual-verification-evidence.md` section V2 (after)                                                                                                                                                                                          |

## Commit Points

| Commit / Stage  | Coherent change set                                                                                       | Commit policy                                                                                                                       |
| --------------- | --------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| Initial PR      | `fix(axum-server): [#3080] declare request_id field on request spans` (T1–T4 code, test, and issue docs)  | Signed commit on PR #3084; evidence and audit metadata synced in the follow-up commit below.                                        |
| PR #3084 review | `fix(axum-server): [#3080] address PR #3084 review findings` (test cleanup, docs fixes, and `PR-REVIEW.md`) | Commit with GPG signature after `write-unit-test`, validator, and pre-commit checks; maintainer review remains pending on PR #3084. |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec created in `docs/issues/open/3080-record-request-ids-on-axum-request-spans/ISSUE.md`
- [ ] Spec reviewed and approved by user/maintainer
- [x] GitHub issue #3080 linked in specification
- [x] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [x] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-11-27 12:00 UTC - Antigravity (Gemini 4 Hydrogen) - Reproduced missing `request_id` on Axum request spans, observed regression test red on unfixed code, implemented `request_id = Empty` in `RedactedMakeSpan::make_span`, verified Axum server tests/Clippy/fmt, and completed like-for-like real-artifact recheck - `manual-verification-evidence.md`
- 2026-11-28 03:17 UTC - Antigravity - Addressed PR #3084 review findings: dropped unused `x-request-id` header from regression test Arrange, re-recorded mutate-then-restore red run and green `torrust-tracker-axum-server` checks, updated spec metadata/commit references, and added `docs/pr-reviews/pr-3084-review/PR-REVIEW.md` - `manual-verification-evidence.md`

## Acceptance Criteria

- [ ] AC1: `RedactedMakeSpan::make_span` declares `request_id = Empty` so `span.record("request_id", ...)` populates the `request` span.
- [ ] AC2: The regression test `it_should_record_the_request_id_in_the_span_of_a_request` failed before the fix and passes after the fix while existing URI redaction tests remain green.
- [ ] AC3: Real-artifact requests with `x-request-id` to REST API, HTTP Tracker, and Health Check API show `request_id` on `request{...}` spans in handler `DEBUG` log lines.
- [ ] AC4: `torrust-tracker-axum-server`, `torrust-tracker-axum-http-server`, `torrust-tracker-axum-rest-api-server`, and `torrust-tracker-axum-health-check-api-server` test suites pass.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior

## Verification Plan

### Automatic Checks

- `cargo test -p torrust-tracker-axum-server`
- `cargo test -p torrust-tracker-axum-http-server -p torrust-tracker-axum-rest-api-server -p torrust-tracker-axum-health-check-api-server`
- `cargo clippy -p torrust-tracker-axum-server --all-targets --all-features -- -D warnings`
- `cargo fmt --all -- --check`
- `linter all` (in developer environment with full linter toolchain)

### Manual Verification Scenarios

| ID  | Scenario                                           | Human-oriented command/steps                                                                      | Expected Result                                                             | Status | Evidence                                     |
| --- | -------------------------------------------------- | ------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------- | ------ | -------------------------------------------- |
| V1  | Request span before fix with `x-request-id`        | Start `torrust-tracker` with `trace_filter = "debug"` and send requests with `x-request-id`       | `request{...}` span on handler `DEBUG` logs lacks `request_id`              | DONE   | `manual-verification-evidence.md` section V1 |
| V2  | Request span after fix (like-for-like recheck)     | Rebuild `torrust-tracker` and repeat identical `curl` requests with `x-request-id`                | `request{...}` span on handler `DEBUG` logs contains `request_id="..."`     | DONE   | `manual-verification-evidence.md` section V2 |

## Risks and Trade-offs

- Keeping both existing `span.record("request_id", ...)` calls (`on_request` and `on_response`) in the three Axum servers means `tracing_subscriber::fmt` appends `request_id="..."` a second time on `on_response` / `on_eos` lines; leaving those server files untouched keeps the change strictly scoped to `RedactedMakeSpan`, and removing the redundant `on_response` records is tracked as follow-up work (`review-finding:pr-3084-f10`).

## Implementation Completion Review

- Retrospective: Not needed (root cause and fix matched the initial hypothesis directly with no unexpected architectural discoveries).

## References

- GitHub issue: #3080
- Reference pattern: `packages/udp-server/src/handlers/mod.rs`
