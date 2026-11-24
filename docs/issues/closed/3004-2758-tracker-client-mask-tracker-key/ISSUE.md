---
schema-version: 1
doc-type: issue
issue-type: bug
status: done
priority: p1
epic: 2758
github-issue: 3004
spec-path: docs/issues/closed/3004-2758-tracker-client-mask-tracker-key/ISSUE.md
branch: "3004-tracker-client-mask-tracker-key"
related-pr: 3012
last-updated-utc: "2026-11-24 13:02"
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
  URL in its `Display` text: reqwest 0.13.5 builds that error with the request URL
  (`crate::error::decode(err).with_url(*self.url)` in `Response::do_bytes`,
  `reqwest-0.13.5/src/async_impl/response.rs:435`). The checker calls `bytes()` too
  (`checks/http.rs:76,94`) and writes its `Debug` text.
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

## Output Path Audit (T1)

Done on `develop` `6bc417424`, by reading every file under `console/tracker-client/src` and
`packages/tracker-client/src/http/client`, and by running the built binary against a closed port
and malformed input (commands and output in `manual-verification-evidence.md`, R0 repeat and T1
probes).

Paths that write a tracker URL or `reqwest` error text, and how this issue treats them:

| ID | Path | Output | What leaks today | Treatment |
| -- | ---- | ------ | ---------------- | --------- |
| P1 | Library `Error::ResponseError`, `Error::ClientBuildingError` (`packages/tracker-client/src/http/client/mod.rs`) | stderr (`http announce`, `http scrape`), stdout (`check`) | `Debug` of `reqwest::Error`: full URL with the key and the query | Kind-based text and the masked URL, no `Debug` text |
| P2 | Library `Error::UnsuccessfulResponse` (same file) | as P1 | `Debug` of `reqwest::Response`: the request URL | Status code and the masked URL |
| P3 | `response.bytes().await?` in `unified/http.rs` (announce and scrape) | stderr | `Display` of the body-read `reqwest::Error`: the URL (`for url (...)`) | Kind-based text and the masked URL |
| P4 | Checker `Checks.url` (`checker/checks/http.rs`) | stdout (`check`) | The configured URL with the key | Masked URL |
| P5 | Checker per-request errors (`clients/http/mod.rs`: `HttpClientError`, `ResponseError`, `{err:?}`) | stdout (`check`) | `Debug` of the library error and of `reqwest::Error`; no key today, because the checker drops the path | `Display` of the fixed library error; kind-based text for the body-read error |
| P6 | Health check errors (`checker/checks/health.rs`, `{err:?}` and `{response:?}`) | stdout (`check`) | `Debug` of `reqwest::Error` and `reqwest::Response`; health URLs carry no key | Kind-based text, no `Debug` text (AC2) |
| P7 | `report_runtime_failure` in `bin/tracker_client.rs` | stderr | Writes P1 to P3 unchanged; it reports `anyhow` errors with `to_string()`, the outermost message only | No change needed once P1 to P3 are fixed |
| P8 | Checker configuration parse error (`checker/config.rs`, `JsonParseError`) | stderr (`check`) | `serde_json` echoes a value of the wrong type: `invalid type: string "http://…/announce/<key>", expected a sequence` | Mask the key of every tracker URL inside the record's text (see below) |
| P9 | `clap` usage error (`AppError::from_clap`) | stderr (any command) | `clap` echoes a misplaced argument: `unexpected argument 'http://…/announce/<key>' found` | As P8 |

Checked and not leaking:

- UDP commands (`udp announce`, `udp scrape`), the UDP checker and `check monitor`: UDP URLs
  carry no key. Their `tracing::debug!` lines are below the fixed `INFO` level of the subscriber
  (`unified/app.rs`), so they are not written.
- `tracing` records of `reqwest` and `hyper`: they log at `debug` and `trace`, below `INFO`.
- The URL validation error (`parse_and_validate_tracker_url`) and the infohash error do not echo the
  URL.
- Invalid URL errors of the checker configuration (`Invalid URL: <url::ParseError>`) do not echo
  the URL.
- The panic hook record: the panics found (`checker/checks/udp.rs:107`, #2984) carry no URL.
- `CheckerOutput` (`checks/structs.rs`) is unused.
- The e2e runner configuration log is out of scope (see Out of Scope).
- Other users of the library (`packages/udp-server`, `packages/axum-http-server`, dev-dependency
  tests): no code matches the old error texts.

P8 and P9 echo text the caller typed, at the place where it was wrong, through a third-party
formatter (`serde_json`, `clap`). The URL is inside free text, so the URL-masking function cannot
reach them alone. The maintainer decided, when shown the audit, to fix them in this issue: a second
small function masks the key of every tracker URL it finds in a text, and the usage-error and
configuration-error records go through it. Each gets a red regression test. Review round 1 of the
implementation PR (3012) showed that the first version, which parsed each URL it found, missed a
scheme in upper or mixed case, a URL cut at a `)` or `'` inside it, and a URL that does not parse;
the function now finds the text after `/announce/` and `/scrape/` without parsing. Rounds 2 to 5
found more forms a URL parser normalizes to the same key (a `\` separator, a delimiter at the start
of the segment, a nested endpoint, a dot segment); since round 5 the function masks the whole rest
of the path after the endpoint, up to the query, so no such form moves the key out of the mask.

## Architectural Decisions

- Related ADRs: `docs/adrs/20260519000000_define_global_cli_output_contract.md` (section 7,
  redaction policy).
- ADRs to create: none known. Masking a URL key segment applies the existing rule.

## Design and Ownership Review

Interface: one small pure function owns the masking of a URL (input: a URL; output: the same URL
with the key segment replaced). Every output path calls it; no output path builds its own masked
text. A second pure function (input: a text; output: the same text with the rest of the path
after every `/announce/` and `/scrape/` and the password of every URL user info masked, without parsing the
URLs) covers the records whose text comes from a third-party formatter (P8, P9 in the audit). An
authenticated library client also masks its own key wherever its path puts it (a custom endpoint
path, the health-check path).

The production change has no child process, readiness wait or shared fixture.

The regression tests do, so they follow these rules:

- **Closed port.** A test that needs a refused connection binds a listener to `127.0.0.1:0`, reads
  its port and drops the listener before the request, so nothing listens there. The test owns the
  port only for that moment; no shared fixture.
- **Local HTTP responder** (the non-2xx case at the library seam). The test binds a
  `tokio` listener to `127.0.0.1:0` and spawns one task that accepts one connection, writes a fixed
  reply and closes it. The test owns the task's `JoinHandle` and aborts it at the end, so a failing
  assertion (a panic, which drops the handle) does not leave it serving: the runtime of
  `#[tokio::test]` ends with the test. No readiness wait is needed: the listener is bound before the
  client connects, and the kernel queues the connection.
- **Fake HTTP tracker** (the body-read case, a black-box test). It reuses
  `console/tracker-client/tests/common/fake_trackers/http.rs`, which already owns a fake HTTP
  server with bounded reads and a stop-and-join thread, extended with a fixed-reply mode instead of
  a second fake.
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
- A unit test at the same library seam for a non-2xx reply, with a keyed URL and the local HTTP
  responder: the text of `UnsuccessfulResponse` (today it prints the `reqwest::Response` with its
  URL) does not contain the key.
- A black-box test for a reply whose body read fails, where that error is built: in the console
  client (`bytes()` at `unified/http.rs:187,215` and `checks/http.rs:76,94`), not in the library.
  `tracker_client http announce` runs against the fake HTTP tracker in a fixed-reply mode whose
  headers promise more bytes than it sends before it closes, with a keyed URL. Stderr does not
  contain the key. On the unchanged code it fails on the key, because the body-read `reqwest::Error`
  carries the URL (`response.rs:435`, see Background). The same test also runs `tracker_client check`
  against the same fake, the only command that reaches the checker's `bytes()` calls, and asserts
  that the output holds no `reqwest::Error` `Debug` text. The checker clears the URL path
  (`checks/http.rs:34`), so that run can be red only on AC2, not on the key.
- A black-box test of `tracker_client check` with a keyed HTTP URL on a closed local port (R0c),
  asserting that neither stdout nor stderr contains the key, and that the result's `url` field
  shows `****`. This path copies the configured URL into `Checks.url` and never goes through the
  library error, so the tests above do not cover it.
- Unit tests of the text-masking function (P8, P9): a text with one keyed URL, two keyed URLs,
  a keyed URL inside quotes, and a text with no URL.
- Black-box tests of the two third-party echoes: `tracker_client check` with a configuration whose
  `http_trackers` is a keyed URL string instead of a list (P8), and `tracker_client http announce`
  with a misplaced keyed URL argument (P9). Stderr does not contain the key and shows `****`.
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
| T1  | DONE   | Audit the output paths | The list of paths that write a URL or request error text, recorded in this spec. |
| T2  | DONE   | Red regression tests | The tests of the Regression Test Strategy, failing on the unchanged code; output and the design review of each increment in the evidence file. |
| T3  | DONE   | Masking function and library error text | The key segment is masked; no `Debug` text of `reqwest::Error` is written. |
| T4  | DONE   | Apply it on every output path | Every path from T1 writes masked URLs only. |
| T5  | DONE   | Maintainer stop | After the final test increment is green and reviewed, report to the maintainer and wait for his review before T6, T7 and the pull request. |
| T6  | DONE   | Docs | Contract doc redaction rule for tracker URLs; README examples checked. Done: a "No secrets" rule in `tracker-cli-io-contract.md`, a bullet in the console README, the `redaction` module in the library README. Sweep: `grep -rn 'announce/[A-Za-z0-9]\|scrape/[A-Za-z0-9]' console/tracker-client packages/tracker-client docs/*.md` and a search of all Markdown files for `reqwest::Error` and `Failed to get a response`: no live doc shows a key or the old error text (the matches are in other issues' records, which stay as written). |
| T7  | DONE   | Green run and recheck | The regression tests pass (output in the evidence file) and R0 is repeated like for like as M1 to M4; the key does not appear. |

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
- [x] Spec-only PR merged into `develop` before implementation
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [x] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [x] Evidence-based implementation completion review recorded
- [x] Reviewer validated acceptance criteria and updated checkboxes (PR #3012 merged after five review rounds)
- [x] Committer verified spec progress is up to date before commit
- [x] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-11-19 16:09 UTC - Claude Code (Claude Opus 5.5) - Drafted this spec as the EPIC #2758 coordinator from the #2768 leftover; reproduced the leak (R0).
- 2026-11-23 13:47 UTC - Claude Code (Claude Opus 5.5) - Re-read the draft against `develop` `ff3600f10`: added the `Display` text of `reqwest::Error` (it also carries the URL), the `Debug` of `reqwest::Response` in `UnsuccessfulResponse`, the body-read error in `unified/http.rs` and the `{err:?}` texts in `http/mod.rs` and `health.rs`; recorded the relation to EPIC #2993. Created issue #3004 (maintainer approved the spec content and a public issue) and moved the spec to `docs/issues/open/`.
- 2026-11-23 14:19 UTC - Claude Code (Claude Opus 5.5) - Addressed review round 1 of PR 3005 (da2ce7 F1 to F10, Greptile): `Checks.url` named as the `check` source, black-box tests for `http scrape` and `check`, M1 keeps the exit status, the progressive test loop and the maintainer stop (new T5), #2984 and #3006 named, the e2e runner configuration log recorded as out of scope, the public-issue decision stated. The EPIC row is added by the coordinator in PR 3007.
- 2026-11-23 14:22 UTC - Claude Code (Claude Opus 5.5) - Addressed the Copilot review of round 1 of PR 3005: ownership and deadlines of the regression-test resources, library-seam tests for a non-2xx reply and a failed body read, and how the completion review is recorded.
- 2026-11-23 14:41 UTC - Claude Code (Claude Opus 5.5) - Addressed review round 2 of PR 3005 (da2ce7): the body-read test is a black-box test of the console client against the existing fake HTTP tracker, and the URL in a body-read error is confirmed in the reqwest 0.13.5 source (`response.rs:435`).
- 2026-11-23 14:55 UTC - Claude Code (Claude Opus 5.5) - Addressed review round 3 of PR 3005 (da2ce7): the body-read test also runs `check` against the fake, the only way to reach the checker's `bytes()` calls, and asserts no `Debug` text (AC2).
- 2026-11-23 16:52 UTC - Claude Code (Claude Opus 5.5) - Spec PR 3005 merged. Repeated R0 on the implementation branch (same output) and did the T1 audit: two more leaks found, the `serde_json` echo of a wrong-typed checker configuration value (P8) and the `clap` echo of a misplaced argument (P9). The maintainer decided to fix both here; added a text-masking function and their tests to the spec.
- 2026-11-23 17:09 UTC - Claude Code (Claude Opus 5.5) - Implemented T2 to T7: masking functions in the library, the library error and every console output path fixed; 13 library tests (10 of them red before the fix) and 8 black-box tests, red before the fix and green after; each console fix guarded by a mutation; the maintainer reviewed the work at the T5 stop. Docs updated; M1 to M4 rechecked. Material discoveries are in `implementation-retrospective.md`.
- 2026-11-23 18:03 UTC - Claude Code (Claude Opus 5.5) - Addressed review round 1 of PR 3012 (da2ce7 F1 to F6, Copilot, Greptile): the text masker no longer parses URLs, so it masks echoed URLs with an upper-case scheme, a `)` or `'` inside, or that do not parse; an authenticated client masks its own key; the error text keeps the chain of causes; contract, README and evidence counts corrected. New tests were red first; the recheck was repeated on the last commit with code.
- 2026-11-23 18:43 UTC - Claude Code (Claude Opus 5.5) - Addressed review round 2 of PR 3012 (Greptile, CodeRabbit): the text masker takes `\` as a separator, masks the whole segment after an endpoint and the first non-empty one after extra separators, finds nested endpoints, and masks every password; an `http scrape` body-read test. The recheck found the `serde_json`-escaped backslash leak, fixed with its own red tests; the final recheck ran on the last commit with code.
- 2026-11-23 19:03 UTC - Claude Code (Claude Opus 5.5) - Addressed review round 3 of PR 3012 (Copilot): `HTTP` in upper case in the checker errors, a percent-encoding test, the library README paragraph wrapped; the recheck repeated on the last commit with code.
- 2026-11-23 19:42 UTC - Claude Code (Claude Opus 5.5) - Review round 4 of PR 3012 (Copilot): the tokio test features are listed. Round 5 (CodeRabbit): a dot segment moved the key past the masked segment; the text masker now hides the whole rest of the path after the endpoint; red tests first, the recheck repeated on the last commit with code.

## Acceptance Criteria

- [x] AC1: No output of `tracker_client` (stdout or stderr, any subcommand, success or failure)
  contains the key of a tracker URL; the key segment is written as `****`.
- [x] AC2: No output contains the `Debug` text of a `reqwest::Error`; a failed request is reported
  with an error kind and the masked URL.
- [x] AC3: The library error text of `packages/tracker-client` does not contain the key.
- [x] AC4: Regression tests fail on the unchanged code and pass after the fix.
- [x] `linter all` exits with code `0`
- [x] Relevant tests pass
- [x] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [x] Documentation is updated when behavior/workflow changes

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
| M1 | Failed announce with a key | `tracker_client http announce http://127.0.0.1:1/announce/SECRETKEYabc123 <infohash> > out.txt 2> err.txt; echo "exit=$?"` (stdout to a file, as R0 does: it keeps the exit status and avoids the terminal refusal), then search `err.txt` for `SECRETKEYabc123` | Exit `1`; one JSON record; no match; the URL shows `/announce/****` | DONE | `manual-verification-evidence.md` section V1 |
| M2 | Failed scrape with a key | The same with `http scrape` and `/scrape/<key>` | As M1 | DONE | `manual-verification-evidence.md` section V2 |
| M3 | `check` with a keyed HTTP tracker | A `check` configuration with a keyed URL on a closed port | The result and stderr contain no key | DONE | `manual-verification-evidence.md` section V3 |
| M4 | Success with a key | Announce to a local private tracker with a valid key | The result contains no key | DONE | `manual-verification-evidence.md` section V4 |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | DONE | V1 to V5 of `manual-verification-evidence.md`; black-box tests in `console/tracker-client/tests/key_redaction.rs` |
| AC2 | DONE | V1 to V3 (kind-based texts); tests `..._check_response_body_is_cut_short` and `..._health_check_gets_no_response` |
| AC3 | DONE | `error_text` tests in `packages/tracker-client/src/http/client/mod.rs` (`Display` and `Debug`) |
| AC4 | DONE | Red runs and mutations in `manual-verification-evidence.md` and `evidence/`; green run T7 |

## Risks and Trade-offs

- Masking too much hides which tracker failed: mask only the key segment, keep host, port and
  path prefix.
- Changing the library error text can break callers that match it; search the workspace for the
  old text before changing it.
- Public issue: decided by the maintainer (a public issue, not `SECURITY.md`). The key is the
  caller's own and is written to the caller's own output; it is not a server vulnerability.

## Implementation Completion Review

- Retrospective: [`implementation-retrospective.md`](implementation-retrospective.md)

After implementation, compare the result with this specification: invalidated assumptions,
material design changes, unexpected validation findings and reusable lessons. If there is any,
create `implementation-retrospective.md` in this folder from
`docs/templates/IMPLEMENTATION-RETROSPECTIVE.md`. If there is none, add a progress-log entry that
says why the work had no material discovery. The completion-review checkbox is ticked only with one
of the two.

- 2026-11-24 11:54 UTC - Claude Code (Claude Opus 5.5) - PR #3012 merged and issue #3004 closed; archived this spec to `docs/issues/closed/` (`status: done`, `related-pr: 3012`).

## References

- Related issues: #2758 (EPIC), #2768 (leftover found there)
- Related ADRs: `docs/adrs/20260519000000_define_global_cli_output_contract.md`
