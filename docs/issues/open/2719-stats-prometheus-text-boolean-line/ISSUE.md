---
schema-version: 1
doc-type: issue
issue-type: bug
status: in-progress
priority: p2
epic: null
github-issue: 2719
spec-path: docs/issues/open/2719-stats-prometheus-text-boolean-line/ISSUE.md
branch: "2719-stats-prometheus-text-boolean-line-spec"
related-pr: null
last-updated-utc: "2026-11-05 07:13"
semantic-links:
  skill-links:
    - create-issue
    - fix-bug
  related-artifacts:
    - .github/skills/dev/debugging/fix-bug/SKILL.md
    - .github/skills/dev/planning/create-issue/SKILL.md
---

<!-- skill-link: create-issue -->

# Issue #2719 - `GET /api/v1/stats?format=prometheus` emits `completed_persisted_enabled true` and breaks Prometheus scrapes

## Goal

Make the legacy `GET /api/v1/stats?format=prometheus` text parseable by Prometheus again: every sample value numeric, and the body
terminated by a line feed.

## Background

`GET /api/v1/stats?format=prometheus` answers a hand-built `name value` text (`metrics_response` in
`packages/axum-rest-api-server/src/v1/context/stats/responses.rs`). It was added by
`feat: [#1126] add support for prometheus text format on stats API endpoint` to feed Prometheus (issue #1126: "stats in plain
text that are ready to be consumed by Prometheus"; downstream need: torrust-demo issue 20). The original output was numbers only.

`fix(metrics): expose completed download retention` (implementing the closed issue
`docs/issues/closed/2122-expose-unambiguous-download-counter-semantics/` and the ADR
`docs/adrs/20260901113500_define_completed_download_metric_retention_names.md`) added three lines. One of them,
`completed_persisted_enabled`, is a boolean and prints as `true` or `false`. Neither the issue, the ADR nor the manual
verification considered this legacy text; they discuss the JSON field and the labeled `/metrics` output only.

Evidence (details and raw output in `manual-verification-evidence.md`):

| Check                                                              | Result                                                                                                  |
| ------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------- |
| Real Prometheus 3.5.0 scraping the running tracker                 | Target `down`: `strconv.ParseFloat: parsing "true": invalid syntax`; only 5 of 32 samples ingested       |
| `promtool check metrics` 3.5.0 on the body                         | `expected float as value, got "true"` (and `"false"` with persistence off)                              |
| Python `prometheus_client` 0.26.0 parser                           | `could not convert string to float: 'true'`                                                             |
| Same body with the line as `1` (or dropped) and a final `\n`        | All parse; real Prometheus scrapes all 32 samples                                                       |
| Missing trailing newline                                            | Fails `promtool` (`unexpected end of input stream`); accepted by the Prometheus server and by the Python parser |
| Missing `# TYPE` / `# HELP` lines                                  | Allowed by the format. Not a defect (`promtool` only reports lint warnings, exit code 3)                |

So the boolean line is a regression of the endpoint's purpose: since that commit, a Prometheus server scraping it marks the target
down and loses every line after `completed_persisted_enabled` (`leechers` and all the TCP/UDP counters). The missing trailing
newline is a conformance defect that only strict tools notice.

Nothing asserts the legacy text: the contract tests in
`packages/axum-rest-api-server/tests/server/v1/contract/context/stats.rs` cover the JSON `/stats` and the labeled
`/metrics?format=prometheus` only. No consumer of `/stats?format=prometheus` exists in this repository (docs, REST API module
docs, `packages/rest-api-client`, `console/`, Grafana/Prometheus examples). The ecosystem repos were not searched.

## Scope

### In Scope

- Render `completed_persisted_enabled` as `1` / `0` in the legacy text (proposed; see Options).
- End the body with `\n`.
- A regression test that fails for the current output.
- Document the legacy text in the `stats` module docs, including the `1`/`0` rendering and that `/metrics?format=prometheus` is the
  full-fidelity exposition endpoint.

### Out of Scope

- Changing the JSON `/stats` response (the boolean stays a JSON boolean, as the ADR says).
- Adding `# TYPE`/`# HELP` lines (allowed to be absent), or rewriting the endpoint on top of `torrust-metrics`.
- A dedicated configuration/info endpoint or metric for consumers (design follow-up).
- Removing or deprecating the endpoint (maintainer decision, separate issue if wanted).
- The stale `#[allow(deprecated)]` on `metrics_response` (separate minor cleanup, convenient to fold in only if the maintainer agrees).

## Options

1. **Render `1`/`0` and add the trailing newline (proposed).** Smallest change, keeps all 32 lines and the ADR's "authoritative
   boolean" semantics, fixes the scrape. Compatibility cost: a client that parses the text for the literal `true`/`false`
   breaks. None is known; the line exists for only a few weeks (since the commit above) and the text was never valid for
   Prometheus with it.
2. Drop the line. Loses the only way to tell a disabled count from an enabled zero in this text; contradicts the ADR intent.
3. Render a labeled metric (`completed_persisted{enabled="true"} 5`). Mixes the legacy name/value style with labels; closer to what
   `/metrics` already does, so users should use that endpoint instead.
4. Add `# TYPE` lines. Optional by the format; no effect on the failure. Could be done later together with a newline fix.

5. Omit `completed_persisted` and `completed_persisted_enabled` when persistence is off, as `/metrics` does (ADR item 4), optionally
   with a `#` comment explaining the absence. Rejected for the legacy text: absence cannot be told apart from an older tracker or a
   truncated scrape without out-of-band documentation, and Prometheus discards comments, so automation cannot read them.

Design note: the boolean is configuration, not a measurement, and mixing the two is not ideal. It is kept as a `1`/`0` value because
it is the only form that downstream automation can parse. Where configuration metadata for metric consumers should live (a
config or info endpoint, an `_info`-style metric, or a startup log line) is a separate design question, out of scope here and
worth its own issue.

Maintainer decision needed: option 1 vs. "won't fix, document, point to `/metrics`" (the endpoint is a legacy text).

## Architectural Decisions

- Related ADRs: `docs/adrs/20260901113500_define_completed_download_metric_retention_names.md` (unchanged: the JSON boolean remains
  authoritative; this issue only changes how the legacy text renders it).
- ADRs to create: None known.

## Design and Ownership Review

Not applicable.

## Bug-Fix Process

Follows [fix-bug](../../../../.github/skills/dev/debugging/fix-bug/SKILL.md):

- Analysis: `metrics_response` formats the `bool` field with `{}`, which prints `true`/`false`; the join leaves no final newline.
- Reproduction with real artifacts: done (tracker, `curl`, Prometheus 3.5.0, `promtool`, `prometheus_client`), see
  `manual-verification-evidence.md`.
- Regression test: red first, then the fix, then green and a like-for-like recheck of the same real-artifact commands.

## Regression Test Strategy

Unit test at the causal seam, `metrics_response` (the function is pure over `Stats`). With a `Stats` whose
`completed_persisted_enabled` is `true`, assert that every non-empty line has a value that parses as `f64` and that the body ends
with `\n`; add a `false` case. The #2637 work pins the current text in a unit test with a `TODO`; that test must be updated on
purpose in the PR that fixes this. Red and green output go to `manual-verification-evidence.md`.

## Implementation Plan

| ID  | Status | Task                                                     | Notes / Expected Output                                           |
| --- | ------ | -------------------------------------------------------- | ----------------------------------------------------------------- |
| T1  | DONE   | Reproduce with real artifacts and parsers                | `manual-verification-evidence.md` V1-V3                           |
| T2  | DONE   | Maintainer decides the option                            | Maintainer chose `1`/`0` plus the trailing newline                               |
| T3  | DONE   | Add the failing regression test (red)                    | Fails on `true`/`false` and on the missing newline                |
| T4  | DONE   | Fix `metrics_response`; align the #2637 pinning test      | Test green; JSON `/stats` untouched                               |
| T5  | DONE   | Document the legacy text in the `stats` module docs      | Example of the text, `1`/`0` rendering, pointer to `/metrics`     |
| T6  | DONE   | Like-for-like recheck of the reproduction                | Prometheus target `up` with 32 samples; `promtool` exits 0 or 3 |

## Commit Points

| Task | Coherent change set                                  | Commit policy                                        |
| ---- | ---------------------------------------------------- | ---------------------------------------------------- |
| T3   | Failing regression test                              | Commit after focused validation and required review. |
| T4   | `metrics_response` fix and aligned pinning test      | Commit after focused validation and required review. |
| T5   | `stats` module docs                                  | Commit after focused validation and required review. |
| T6   | Recheck evidence                                     | Commit after focused validation and required review. |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/stats-prometheus-text-boolean-line/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [ ] (Optional) Spec-only PR merged into `develop` before implementation
- [x] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [x] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-11-04 13:50 UTC - Claude Code (Claude Sonnet 5.5) - Reproduced the output, investigated history and consumers, ran
  Prometheus 3.5.0, `promtool` and `prometheus_client` against the body; drafted this spec - `manual-verification-evidence.md`
- 2026-11-04 15:34 UTC - Claude Code (Claude Sonnet 5.5) - Spec approved by the maintainer; GitHub issue #2719 created; spec moved to `open/` on branch `2719-stats-prometheus-text-boolean-line-spec`
- 2026-11-05 07:13 UTC - Claude Code (Claude Sonnet 5.5) - Maintainer chose `1`/`0`; red test, fix, module docs and like-for-like recheck done on branch `2719-stats-prometheus-text-boolean-line` - `manual-verification-evidence.md` V4, V5

## Acceptance Criteria

- [x] AC1: Every sample value in `GET /api/v1/stats?format=prometheus` is numeric, with persistence on and off.
- [x] AC2: The body ends with a line feed.
- [x] AC3: A Prometheus 3.x server scraping the endpoint reports the target `up` with all samples ingested.
- [x] AC4: A unit test fails on the old output and passes on the new one.
- [x] AC5: The `stats` module docs describe the legacy text and point to `/metrics?format=prometheus`.
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

| ID  | Scenario                              | Human-oriented command/steps                                                                                   | Expected Result                                   | Status | Evidence                                     |
| --- | ------------------------------------- | -------------------------------------------------------------------------------------------------------------- | ------------------------------------------------- | ------ | -------------------------------------------- |
| M1  | Reproduce the raw text, both modes    | Run the tracker with persistence off and on; `curl` `/api/v1/stats?format=prometheus`; inspect bytes          | `false`/`true` line, no final newline             | DONE   | `manual-verification-evidence.md` section V1 |
| M2  | Parser checks                         | `promtool check metrics`, `prometheus_client` parser                                                           | Errors on the boolean and on the missing newline  | DONE   | `manual-verification-evidence.md` section V2 |
| M3  | Real Prometheus scrape                | Prometheus 3.5.0 scraping the tracker                                                                          | Target `down`, 5 samples                          | DONE   | `manual-verification-evidence.md` section V3 |
| M4  | Like-for-like recheck after the fix   | Repeat M1-M3                                                                                                   | Numeric values, final newline, target `up`, 32 samples | DONE | `manual-verification-evidence.md` section V5 |

### Acceptance Verification

| AC ID | Status | Evidence |
| ----- | ------ | -------- |
| AC1   | DONE   | `manual-verification-evidence.md` V4, V5 |
| AC2   | DONE   | `manual-verification-evidence.md` V4, V5 |
| AC3   | DONE   | `manual-verification-evidence.md` V4, V5 |
| AC4   | DONE   | `manual-verification-evidence.md` V4, V5 |
| AC5   | DONE   | `manual-verification-evidence.md` V4, V5 |

## Risks and Trade-offs

- Changing a public legacy text: a client matching `true`/`false` breaks. Mitigation: none known in the repository; document the
  change in the release notes; ecosystem repos (Index, torrust-demo) to be checked before merging.
- The `#2637` pinning test changes in the same PR as the behavior (intentional, see the Regression Test Strategy).

## Implementation Completion Review

- Retrospective: `Not yet assessed`

## References

- Related issues: #2719 (this issue), #1126 (original feature), #2122 (retention names), #2637 (REST API server test coverage)
- Related PRs: #1127 (original feature), #2127 (retention names)
- Related ADRs: `docs/adrs/20260901113500_define_completed_download_metric_retention_names.md`
