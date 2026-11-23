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
last-updated-utc: "2026-11-23 14:22"
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

<!-- cspell:ignore SECRETKEY Greptile -->

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
- `console/tracker-client/src/console/clients/checker/`: `check` results carry the tracker URL in
  `Checks.url` (`checks/http.rs:17`, filled from the configured URL at `:37`); the per-request
  `Err` texts of `Checks.results` come from the errors of `clients/http/mod.rs:11,15`, which carry
  the same `reqwest` text. The health-check results carry their URL in `Checks.url`
  (`checks/health.rs:31`). The monitor records carry a `url` field (`monitor/udp.rs`, UDP only,
  which has no key). `CheckerOutput` in `checks/structs.rs` is not used anywhere and does not
  reach the output.

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

- The panic of `check` for a `udp://` URL without a port (`checker/checks/udp.rs:107`); #2984, a
  bug issue outside this EPIC, covers it.
- A schema version for the results; #3006 of this EPIC covers it.
- Two behaviors seen in R0c: the checker drops the key from the request path, so it cannot check a
  private tracker, and it reports a refused connection as a timeout. Record them as separate
  issues if the maintainer wants them fixed.
- Other tools of the EPIC; UDP has no key in the URL.
- The e2e runner logs the whole checker configuration before it runs the checker
  (`src/console/ci/e2e/tracker_checker.rs:88`, `Tracker Checker config:`). A keyed HTTP URL there
  would reach the same logs, but the runner builds that configuration from the running services,
  which carry no key (`src/console/ci/e2e/runner.rs:117-118`), and the file is not part of
  `tracker_client`. The T1 audit records it as checked and out of scope.

Files this issue may touch (a sibling issue in the same wave must not touch them):

- `console/tracker-client/**`
- `packages/tracker-client/src/http/client/**` and its tests

## Architectural Decisions

- Related ADRs: `docs/adrs/20260519000000_define_global_cli_output_contract.md` (section 7,
  redaction policy).
- ADRs to create: none known. Masking a URL key segment applies the existing rule.

## Design and Ownership Review

Interface: one small pure function owns the masking of a URL (input: a URL; output: the same URL
with the key segment replaced). Every output path calls it; no output path builds its own masked
text. The production change has no child process, readiness wait or shared fixture.

The regression tests do, so they follow these rules:

- **Closed port.** A test that needs a refused connection binds a listener to `127.0.0.1:0`, reads
  its port and drops the listener before the request, so nothing listens there. The test owns the
  port only for that moment; no shared fixture.
- **Local HTTP responder** (the non-2xx and body-read cases at the library seam). The test binds a
  `tokio` listener to `127.0.0.1:0` and spawns one task that accepts one connection, writes a fixed
  reply and closes it. The test owns the task's `JoinHandle` and aborts it at the end, so a failing
  assertion (a panic, which drops the handle) does not leave it serving: the runtime of
  `#[tokio::test]` ends with the test. No readiness wait is needed: the listener is bound before the
  client connects, and the kernel queues the connection.
- **Child process** (the black-box tests). The test runs `tracker_client` with
  `std::process::Command::output()`, which waits for the exit and closes both pipes; stdout goes to a
  pipe, so the terminal refusal does not apply. The child is never left running: it ends on its own
  request failure.
- **Deadlines.** Every awaited request is bounded: the client gets a short timeout (the library
  tests pass one to `Client::new`; the tool uses its own default network timeout), and every await
  in a library test is wrapped in `tokio::time::timeout` with a fixed bound, so a hang fails the test
  instead of blocking the run.
- **Checkpoint.** After the first vertical slice (the masking function used by the library error,
  with its unit test and one black-box test green), review this ownership map against the code and
  record the result in the evidence file before adding the next increment.

## Bug-Fix Process

Follow `.claude/skills/fix-bug/SKILL.md`:

1. Analysis: list every output path that can write a URL or `reqwest` error text (In Scope).
2. Reproduction: R0 is recorded in this folder; repeat it on the implementation branch before the
   fix.
3. Regression boundary: a unit test of the masking function, a unit test at the library error
   seam that formats a failed request (the error text must not contain the key), and black-box
   tests of `http announce`, `http scrape` and `check` (see Regression Test Strategy).
4. Red: run the tests against the unchanged code and record the failure.
5. Fix.
6. Green: record the passing tests and repeat R0 like for like (expected: the key is masked).

## Regression Test Strategy

- Unit test of the masking function: URLs with and without a key, `/announce/<key>`,
  `/scrape/<key>`, a trailing slash, a query string, and a URL that is not an announce or scrape
  URL.
- Unit test at the library error seam in `packages/tracker-client`: a request to a closed local
  port with a URL that carries a key; the error's `Display` text does not contain the key.
- Black-box tests of `tracker_client http announce` and `tracker_client http scrape` against a
  closed local port with a keyed URL (R0a and R0b), asserting that stderr is one JSON record, that
  it does not contain the key, and that the URL shows `****`. Announce and scrape build their URLs
  and errors on separate paths (`build_announce_url`, `build_scrape_url`), so each gets its own
  test.
- Unit tests at the same library seam for the other two leak paths, each with a keyed URL and the
  local HTTP responder: a non-2xx reply (`UnsuccessfulResponse`, whose text today prints the
  `reqwest::Response` with its URL), and a reply whose body read fails (the headers promise more
  bytes than the responder sends before it closes, so `Response::bytes()` returns a
  `reqwest::Error` that carries the URL). The error text of each does not contain the key. The
  body-read error is built by the console client (`unified/http.rs`, `clients/http/mod.rs`), so
  its test sits where that text is built.
- A black-box test of `tracker_client check` with a keyed HTTP URL on a closed local port (R0c),
  asserting that neither stdout nor stderr contains the key, and that the result's `url` field
  shows `****`. This path copies the configured URL into `Checks.url` and never goes through the
  library error, so the tests above do not cover it.
- Tests that run the binary look up `NEXTEST_BIN_EXE_tracker_client` before
  `CARGO_BIN_EXE_tracker_client`.

Progressive test development: use the `write-unit-test` skill and add one behavior-focused test
increment at a time. After each increment passes focused validation, and before the next one
starts, do the design review and record it in the evidence file: the prose-first
Arrange-Act-Assert comparison (write temporary prose for each section, refactor until the code says
it, remove the redundant prose), confirming that the test shows the one causal difference (a keyed
URL), that its fixture owns only incidental mechanics, and that the production Act and the
independently stated expected result stay visible. Judge helpers by named actions and abstraction
level, not caller count. After the final increment, stop for maintainer review before the final
verification, the commit of the fix and the pull request.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task | Notes / Expected Output |
| --- | ------ | ---- | ----------------------- |
| T1  | TODO   | Audit the output paths | The list of paths that write a URL or request error text, recorded in this spec. |
| T2  | TODO   | Red regression tests | The tests of the Regression Test Strategy, failing on the unchanged code; output and the design review of each increment in the evidence file. |
| T3  | TODO   | Masking function and library error text | The key segment is masked; no `Debug` text of `reqwest::Error` is written. |
| T4  | TODO   | Apply it on every output path | Every path from T1 writes masked URLs only. |
| T5  | TODO   | Maintainer stop | After the final test increment is green and reviewed, report to the maintainer and wait for his review before T6, T7 and the pull request. |
| T6  | TODO   | Docs | Contract doc redaction rule for tracker URLs; README examples checked. |
| T7  | TODO   | Green run and recheck | The regression tests pass (output in the evidence file) and R0 is repeated like for like as M1 to M4; the key does not appear. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T1 | Audit list in the spec | Commit after focused validation and required review. |
| T2 | Red tests | Commit after focused validation and required review. |
| T3 | Masking function and library error text | Commit after focused validation and required review. |
| T4 | Output paths | Commit after focused validation and required review. |
| T5 | No change: a checkpoint | No commit. |
| T6 | Docs | Commit after focused validation and required review. |
| T7 | Green run and recheck evidence | Commit after focused validation and required review. |

Each test-producing task (T2, T3, T4) follows the progressive test development of the Regression
Test Strategy: one increment, focused validation, recorded design review, then the next. Sign every
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
- 2026-11-23 14:19 UTC - Claude Code (Claude Opus 5.5) - Addressed review round 1 of PR 3005 (da2ce7 F1 to F10, Greptile): `Checks.url` named as the `check` source, black-box tests for `http scrape` and `check`, M1 keeps the exit status, the progressive test loop and the maintainer stop (new T5), #2984 and #3006 named, the e2e runner configuration log recorded as out of scope, the public-issue decision stated. The EPIC row is added by the coordinator in PR 3007.
- 2026-11-23 14:22 UTC - Claude Code (Claude Opus 5.5) - Addressed the Copilot review of round 1 of PR 3005: ownership and deadlines of the regression-test resources, library-seam tests for a non-2xx reply and a failed body read, and how the completion review is recorded.

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
| M1 | Failed announce with a key | `tracker_client http announce http://127.0.0.1:1/announce/SECRETKEYabc123 <infohash> > out.txt 2> err.txt; echo "exit=$?"` (stdout to a file, as R0 does: it keeps the exit status and avoids the terminal refusal), then search `err.txt` for `SECRETKEYabc123` | Exit `1`; one JSON record; no match; the URL shows `/announce/****` | TODO | `manual-verification-evidence.md` section V1 |
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
- Public issue: decided by the maintainer (a public issue, not `SECURITY.md`). The key is the
  caller's own and is written to the caller's own output; it is not a server vulnerability.

## Implementation Completion Review

- Retrospective: `Not yet assessed`

After implementation, compare the result with this specification: invalidated assumptions,
material design changes, unexpected validation findings and reusable lessons. If there is any,
create `implementation-retrospective.md` in this folder from
`docs/templates/IMPLEMENTATION-RETROSPECTIVE.md`. If there is none, add a progress-log entry that
says why the work had no material discovery. The completion-review checkbox is ticked only with one
of the two.

## References

- Related issues: #2758 (EPIC), #2768 (leftover found there)
- Related ADRs: `docs/adrs/20260519000000_define_global_cli_output_contract.md`
