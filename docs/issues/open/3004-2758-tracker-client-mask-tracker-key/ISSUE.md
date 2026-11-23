---
schema-version: 1
doc-type: issue
issue-type: bug
status: planned
priority: p1
epic: 2758
github-issue: 3004
spec-path: docs/issues/open/3004-2758-tracker-client-mask-tracker-key/ISSUE.md
branch: "3004-tracker-client-mask-tracker-key-spec"
related-pr: null
last-updated-utc: "2026-11-23 13:47"
semantic-links:
  skill-links:
    - create-issue
    - fix-bug
  related-artifacts:
    - .claude/skills/create-issue/SKILL.md
    - .claude/skills/fix-bug/SKILL.md
    - docs/adrs/20260519000000_define_global_cli_output_contract.md
    - docs/issues/open/2758-cli-output-contract-tools-migration/EPIC.md
    - console/tracker-client/docs/contracts/tracker-cli-io-contract.md
---

<!-- skill-link: create-issue -->

<!-- cspell:ignore SECRETKEY -->

# Issue #3004 - `tracker_client`: do not write the tracker key in diagnostics or results

Parent EPIC: #2758 - Migrate the CLI tools to the global output contract

## Goal

`tracker_client` never writes the key of a private HTTP tracker (the `<key>` in
`/announce/<key>` and `/scrape/<key>`) to stdout or stderr. Where a tracker URL is needed to
identify a target, the key segment is masked with a fixed string (`****`), as section 7 of the
global CLI output contract requires.

## Background

A private Torrust HTTP tracker authenticates a peer with a key in the URL path:
`/announce/{key}` and `/scrape/{key}`. A caller of `tracker_client` passes that URL on the
command line or in the `check` configuration, so the key is a credential the tool holds.

Section 7 of `docs/adrs/20260519000000_define_global_cli_output_contract.md` says JSON
diagnostics and result payloads must not expose secrets or credentials, and that secret values are
masked with a fixed string such as `****`.

The #2768 session saw the request URL in a `runtime_failure` record (recorded as a leftover in
`docs/issues/closed/2768-2758-tracker-client-json-only/manual-verification-evidence.md`). The
coordinator reproduced it on `develop` (record R0 in `manual-verification-evidence.md` of this
folder):

- `http announce` and `http scrape`: when the request fails, the `runtime_failure` record on stderr
  carries the full request URL, including the key and the query, inside the `Debug` text of a
  `reqwest::Error` (R0a, R0b).
- `check`: the result on stdout echoes the configured URL, key included, in its `url` field (R0c).
  Its per-request texts also carry `reqwest::Error` `Debug` text.

Known sources, to confirm and complete during analysis:

- `packages/tracker-client/src/http/client/mod.rs`: the library error formats the `reqwest` error
  with `{err:?}` (`Failed to get a response: {err:?}` and `Failed to Build a Http Client: {err:?}`).
  The `Debug` text of `reqwest::Error` contains the full URL. Its `Display` text does too: reqwest
  0.13.5 appends `for url (<url>)` to it (`reqwest-0.13.5/src/error.rs:301`), so replacing `{err:?}`
  with `{err}` is not enough; the URL must be removed (`reqwest::Error::without_url`) or the text
  built from the error kind. The same file's `UnsuccessfulResponse` formats the
  `reqwest::Response` with `{response:?}`, whose `Debug` prints the request URL
  (`reqwest-0.13.5/src/async_impl/response.rs:443`); a non-2xx reply (for example from a proxy)
  writes the key the same way.
- `console/tracker-client/src/console/clients/unified/http.rs`: `response.bytes().await?` turns a
  body-read `reqwest::Error` into the `anyhow` error that `report_runtime_failure` writes, with the
  URL in its `Display` text.
- `console/tracker-client/src/console/clients/http/mod.rs` and
  `console/tracker-client/src/console/clients/checker/checks/health.rs`: their errors also format a
  `reqwest::Error` (or the library error) with `{err:?}`. Health-check URLs carry no key, but the
  `Debug` text still must not be written (AC2).
- `console/tracker-client/src/bin/tracker_client.rs`: `report_runtime_failure` writes the error
  text unchanged.
- `console/tracker-client/src/console/clients/checker/`: `check` results carry the tracker URL
  (`CheckerOutput.url` in `checks/structs.rs`) and a status message that may carry the same
  `reqwest` text. The monitor records carry a `url` field (`monitor/udp.rs`, UDP only, which has no
  key).

Why it matters: stderr and stdout of `tracker_client` are captured by CI and by AI agents (the e2e
runner logs the checker output), so a key written there can end up in shared logs.

This is a bug, not a regression: it predates #2768. It was out of the scope of #2768's acceptance
criteria.

Relation to EPIC #2993 (keep secrets out of errors, logs and client replies): its sub-issue SI-A
(#2999) fixes the same class of leak on the server side and in `packages/rest-api-client`. It does
not touch `console/tracker-client` or `packages/tracker-client`, so the two issues do not share
files. SI-A masks with `***`, the marker of the configuration redaction; this issue follows
section 7 of the CLI output contract, whose example marker is `****`. The two markers apply to
different outputs (server logs versus CLI output); aligning them is not in this issue's scope.

## Scope

### In Scope

- Find every output path of `tracker_client` that can write a tracker URL or text derived from a
  request error, on stdout and stderr, for `http announce`, `http scrape`, `udp announce`,
  `udp scrape`, `check` and `check monitor`. Record the list in the spec.
- Mask the key path segment of an HTTP tracker URL (`/announce/<key>`, `/scrape/<key>`) wherever a
  URL is written; keep the rest of the URL so a reader can tell which tracker failed.
- Stop writing the `Debug` text of `reqwest::Error`; write a short message with the error kind
  (connect, timeout, status, decode) and the masked URL.
- Fix the source in `packages/tracker-client` when the library builds the text, so other callers of
  the library get the same protection.
- Regression tests at the smallest boundary that fails for the defect (see below).
- Update `console/tracker-client/docs/contracts/tracker-cli-io-contract.md` (redaction rule for
  tracker URLs) and the README if an example shows a key.

### Out of Scope

- The panic of `check` for a `udp://` URL without a port (`checker/checks/udp.rs:107`); a separate
  bug issue outside this EPIC covers it.
- A schema version for the results (separate issue of this EPIC).
- Two behaviors seen in R0c: the checker drops the key from the request path, so it cannot check a
  private tracker, and it reports a refused connection as a timeout. Record them as separate
  issues if the maintainer wants them fixed.
- Other tools of the EPIC; UDP has no key in the URL.

Files this issue may touch (a sibling issue in the same wave must not touch them):

- `console/tracker-client/**`
- `packages/tracker-client/src/http/client/**` and its tests

## Architectural Decisions

- Related ADRs: `docs/adrs/20260519000000_define_global_cli_output_contract.md` (section 7,
  redaction policy).
- ADRs to create: none known. Masking a URL key segment applies the existing rule.

## Design and Ownership Review

Not applicable: no child processes, readiness waits or fixtures. One small pure function owns the
masking of a URL (input: a URL; output: the same URL with the key segment replaced). Every output
path calls it; no output path builds its own masked text.

## Bug-Fix Process

Follow `.claude/skills/fix-bug/SKILL.md`:

1. Analysis: list every output path that can write a URL or `reqwest` error text (In Scope).
2. Reproduction: R0 is recorded in this folder; repeat it on the implementation branch before the
   fix.
3. Regression boundary: a unit test of the masking function, and a unit test at the library error
   seam that formats a failed request (the error text must not contain the key).
4. Red: run the tests against the unchanged code and record the failure.
5. Fix.
6. Green: record the passing tests and repeat R0 like for like (expected: the key is masked).

## Regression Test Strategy

- Unit test of the masking function: URLs with and without a key, `/announce/<key>`,
  `/scrape/<key>`, a trailing slash, a query string, and a URL that is not an announce or scrape
  URL.
- Unit test at the library error seam in `packages/tracker-client`: a request to a closed local
  port with a URL that carries a key; the error's `Display` text does not contain the key.
- A black-box test of `tracker_client http announce` against a closed local port, asserting that
  stderr is one JSON record and does not contain the key. Tests that run the binary look up
  `NEXTEST_BIN_EXE_tracker_client` before `CARGO_BIN_EXE_tracker_client`.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task | Notes / Expected Output |
| --- | ------ | ---- | ----------------------- |
| T1  | TODO   | Audit the output paths | The list of paths that write a URL or request error text, recorded in this spec. |
| T2  | TODO   | Red regression tests | The tests of the Regression Test Strategy, failing on the unchanged code; output in the evidence file. |
| T3  | TODO   | Masking function and library error text | The key segment is masked; no `Debug` text of `reqwest::Error` is written. |
| T4  | TODO   | Apply it on every output path | Every path from T1 writes masked URLs only. |
| T5  | TODO   | Docs | Contract doc redaction rule for tracker URLs; README examples checked. |
| T6  | TODO   | Recheck | R0 repeated like for like; the key does not appear. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T1 | Audit list in the spec | Commit after focused validation and required review. |
| T2 | Red tests | Commit after focused validation and required review. |
| T3 | Masking function and library error text | Commit after focused validation and required review. |
| T4 | Output paths | Commit after focused validation and required review. |
| T5 | Docs | Commit after focused validation and required review. |
| T6 | Evidence | Commit after focused validation and required review. |

Use the `write-unit-test` skill and the design review after each passing test increment. Sign every
commit.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2758-tracker-client-mask-tracker-key/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [ ] Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-11-19 16:09 UTC - Claude Code (Claude Opus 5.5) - Drafted this spec as the EPIC #2758 coordinator from the #2768 leftover; reproduced the leak (R0).
- 2026-11-23 13:47 UTC - Claude Code (Claude Opus 5.5) - Re-read the draft against `develop` `ff3600f10`: added the `Display` text of `reqwest::Error` (it also carries the URL), the `Debug` of `reqwest::Response` in `UnsuccessfulResponse`, the body-read error in `unified/http.rs` and the `{err:?}` texts in `http/mod.rs` and `health.rs`; recorded the relation to EPIC #2993. Created issue #3004 (maintainer approved the spec content and a public issue) and moved the spec to `docs/issues/open/`.

## Acceptance Criteria

- [ ] AC1: No output of `tracker_client` (stdout or stderr, any subcommand, success or failure)
  contains the key of a tracker URL; the key segment is written as `****`.
- [ ] AC2: No output contains the `Debug` text of a `reqwest::Error`; a failed request is reported
  with an error kind and the masked URL.
- [ ] AC3: The library error text of `packages/tracker-client` does not contain the key.
- [ ] AC4: Regression tests fail on the unchanged code and pass after the fix.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test -p torrust-tracker-client` and `cargo test -p torrust-tracker-client-lib` (the
  package of `packages/tracker-client`)
- Pre-push checks

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1 | Failed announce with a key | `tracker_client http announce http://127.0.0.1:1/announce/SECRETKEYabc123 <infohash> 2>err.txt \| cat`, then search `err.txt` for `SECRETKEYabc123` | Exit `1`; one JSON record; no match; the URL shows `/announce/****` | TODO | `manual-verification-evidence.md` section V1 |
| M2 | Failed scrape with a key | The same with `http scrape` and `/scrape/<key>` | As M1 | TODO | `manual-verification-evidence.md` section V2 |
| M3 | `check` with a keyed HTTP tracker | A `check` configuration with a keyed URL on a closed port | The result and stderr contain no key | TODO | `manual-verification-evidence.md` section V3 |
| M4 | Success with a key | Announce to a local private tracker with a valid key | The result contains no key | TODO | `manual-verification-evidence.md` section V4 |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | TODO |  |
| AC2 | TODO |  |
| AC3 | TODO |  |
| AC4 | TODO |  |

## Risks and Trade-offs

- Masking too much hides which tracker failed: mask only the key segment, keep host, port and
  path prefix.
- Changing the library error text can break callers that match it; search the workspace for the
  old text before changing it.
- Public issue: the defect exposes a caller's own key in its own output, not a server
  vulnerability. The maintainer decides whether it must go through `SECURITY.md` instead.

## Implementation Completion Review

- Retrospective: `Not yet assessed`

## References

- Related issues: #2758 (EPIC), #2768 (leftover found there)
- Related ADRs: `docs/adrs/20260519000000_define_global_cli_output_contract.md`
