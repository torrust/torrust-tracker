---
schema-version: 1
doc-type: issue
issue-type: task
status: planned
priority: p2
epic: null
github-issue: 2734
spec-path: docs/issues/open/2734-stats-prometheus-text-help-and-type/ISSUE.md
branch: "2734-stats-prometheus-text-help-and-type-spec"
related-pr: null
last-updated-utc: "2026-11-05 16:32"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - docs/adrs/20261105132341_define_the_stats_and_metrics_endpoints_and_require_help_and_type_in_prometheus_output.md
    - docs/issues/open/2719-stats-prometheus-text-boolean-line/ISSUE.md
---

<!-- skill-link: create-issue -->

# Issue #2734 - Add `# HELP` and `# TYPE` lines to `GET /api/v1/stats?format=prometheus`

## Goal

Make the Prometheus text of `GET /api/v1/stats?format=prometheus` follow the ADR rule that every Prometheus text of the tracker
carries `# HELP` and `# TYPE` for each metric family, without changing any metric name or value.

## Background

The ADR "Define the stats and metrics endpoints and require HELP and TYPE in Prometheus output" decided that `/stats` is not
deprecated and that every Prometheus text the tracker exports must carry `# HELP` and `# TYPE`. It recorded the legacy text of
`/stats` as a known exception and named this issue as the follow-up.

Issue #2719 made the text parseable (numeric values, final line feed). Today the 32 lines are bare `name value` pairs, so every
metric is `untyped` and `promtool check metrics` reports "no help text" for each (exit code 3). The text comes from
`metrics_response` in `packages/axum-rest-api-server/src/v1/context/stats/responses.rs`, which formats the flat `Stats` resource.
Every `Stats` field already has a doc comment that describes it, so the descriptions exist, but only as Rust comments.

The metric names are part of API v1 and do not change, so a counter without the `_total` suffix keeps its name.

## Scope

### In Scope

- Print `# HELP <name> <description>` and `# TYPE <name> <type>` before each sample line of the legacy text, in the order of the
  JSON fields.
- Choose the type of each of the 32 names (see the proposal below) and one description per name.
- Keep one source for the descriptions so the text and the other formats do not drift.
- Update the unit tests and the `stats` module docs example; re-run the Prometheus 3.5.0 and `promtool` check.

### Out of Scope

- Renaming metrics, adding labels, or changing values.
- Changing the JSON `/stats` response or the labelled `/metrics` endpoint.
- Deprecating `/stats` (the ADR says it is not deprecated).

## Proposed types

| Type    | Names                                                                                                                                                                                                                                                                           |
| ------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| gauge   | `torrents`, `seeders`, `leechers`, `completed_persisted_enabled`, `udp_banned_ips_total`, `udp_avg_connect_processing_time_ns`, `udp_avg_announce_processing_time_ns`, `udp_avg_scrape_processing_time_ns`                                                                        |
| counter | `completed`, `completed_in_session`, `completed_persisted`, `tcp4_*_handled`, `tcp6_*_handled`, `udp_requests_discarded`, `udp_requests_aborted`, `udp_requests_banned`, `udp4_*`, `udp6_*` (requests, connections, announces, scrapes, responses and errors handled)         |

To be confirmed in review: `completed_persisted` can be restored from storage at startup, which a counter allows; and
`udp_banned_ips_total` has a `_total` name but counts the IPs currently over the limit, so it is a gauge.

## Options

1. Keep the strings in `metrics_response`, next to each line. Simple, but the descriptions then exist twice (the `Stats` doc comment
   and the string).
2. Build the descriptions and types from one table that both the `Stats` documentation and the renderer read (a constant array
   of `(name, type, help)` in the protocol package, or a derive). One source, a little more code.
3. Render the text from the labelled metric collection with `torrust-metrics`, as `/metrics` does. Gets the exposition for free but
   changes the names and needs a mapping, so it is likely outside API v1 compatibility.

Proposed: option 2. To be decided by the maintainer when the spec is reviewed.

## Architectural Decisions

- Related ADRs: `docs/adrs/20261105132341_define_the_stats_and_metrics_endpoints_and_require_help_and_type_in_prometheus_output.md`
  (the rule this issue implements).
- ADRs to create: None known.

## Design and Ownership Review

Not applicable.

## Regression Test Strategy

Unit tests on `metrics_response` (pure over `Stats`): every sample line is preceded by one `# HELP` and one `# TYPE` line for the
same name; every name has exactly one type from the table; the body still ends with a line feed. Red first, then the change. The
existing pinned test for the exact text changes on purpose in the same PR.

## Implementation Plan

| ID  | Status  | Task                                                  | Notes / Expected Output                                   |
| --- | ------- | ----------------------------------------------------- | --------------------------------------------------------- |
| T1  | PENDING | Maintainer decides the option and the types           | Decision recorded in this spec                            |
| T2  | PENDING | Add the failing tests (red)                           | Fail on the missing `# HELP` and `# TYPE` lines           |
| T3  | PENDING | Implement the single source and the renderer change   | Tests green; JSON `/stats` untouched                      |
| T4  | PENDING | Update the module docs example                        | Text with `# HELP` and `# TYPE`; remove the "known gap"   |
| T5  | PENDING | Like-for-like recheck with real artifacts             | Prometheus 3.5.0 target `up`, 32 samples; `promtool` exit 0 |

## Commit Points

| Task | Coherent change set                  | Commit policy                                        |
| ---- | ------------------------------------ | ---------------------------------------------------- |
| T2   | Failing tests                        | Commit after focused validation and required review. |
| T3   | Single source and renderer change    | Commit after focused validation and required review. |
| T4   | Module docs                          | Commit after focused validation and required review. |
| T5   | Recheck evidence                     | Commit after focused validation and required review. |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/stats-prometheus-text-help-and-type/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [ ] (Optional) Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-11-05 16:25 UTC - Claude Code (Claude Sonnet 5.5) - Drafted this spec after the ADR merged; awaiting maintainer review
- 2026-11-05 16:32 UTC - Claude Code (Claude Sonnet 5.5) - Spec approved by the maintainer; GitHub issue #2734 created; spec moved to `open/`

## Acceptance Criteria

- [ ] AC1: Every metric of `GET /api/v1/stats?format=prometheus` has one `# HELP` and one `# TYPE` line before its sample.
- [ ] AC2: Names and values are unchanged from the current text.
- [ ] AC3: The descriptions have one source shared with the `Stats` documentation.
- [ ] AC4: `promtool check metrics` reports no warnings (exit code `0`) and Prometheus 3.x scrapes 32 samples with the target `up`.
- [ ] AC5: The `stats` module docs show the new text and no longer call the missing lines a known gap.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test -p torrust-tracker-axum-rest-api-server`
- Pre-push checks

### Manual Verification Scenarios

| ID  | Scenario                       | Human-oriented command/steps                                                     | Expected Result                                  | Status  | Evidence |
| --- | ------------------------------ | -------------------------------------------------------------------------------- | ------------------------------------------------ | ------- | -------- |
| M1  | Raw text with persistence off and on | Run the tracker; `curl` `/api/v1/stats?format=prometheus`                    | `# HELP` and `# TYPE` before every sample        | PENDING | -        |
| M2  | Lint the body                  | `promtool check metrics` 3.5.0 on the saved body                                 | Exit code `0`, no warnings                       | PENDING | -        |
| M3  | Real Prometheus scrape         | Prometheus 3.5.0 scraping the tracker                                            | Target `up`, 32 samples, types visible in the UI | PENDING | -        |

### Acceptance Verification

| AC ID | Status  | Evidence |
| ----- | ------- | -------- |
| AC1   | PENDING | -        |
| AC2   | PENDING | -        |
| AC3   | PENDING | -        |
| AC4   | PENDING | -        |
| AC5   | PENDING | -        |

## Risks and Trade-offs

- Naive line-based consumers of the legacy text see new `#` lines. Prometheus ignores them; no consumer in the repository is known.
- A wrong type (for example a gauge declared as a counter) misleads `rate()` users. The table above is the review item for that.

## Implementation Completion Review

- Retrospective: `Not yet assessed`

## References

- Related issues: #2734 (this issue), #2719 (numeric values and final newline), #1126 (original feature)
- Related ADRs: `docs/adrs/20261105132341_define_the_stats_and_metrics_endpoints_and_require_help_and_type_in_prometheus_output.md`
