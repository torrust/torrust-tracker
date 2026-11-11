---
schema-version: 1
doc-type: issue
issue-type: bug
status: in-progress
priority: p2
epic: 2411
github-issue: 2776
spec-path: docs/issues/open/2776-2411-axum-server-0-9-timeouts/ISSUE.md
branch: "2776-axum-server-0-9-timeouts-spec"
related-pr: null
last-updated-utc: "2026-11-11 10:00"
semantic-links:
  skill-links:
    - create-issue
    - fix-bug
  related-artifacts:
    - .github/skills/dev/debugging/fix-bug/SKILL.md
    - .github/skills/dev/planning/create-issue/SKILL.md
    - docs/issues/open/2411-spam-and-abuse-resistance/EPIC.md
    - packages/axum-server/src/custom_axum_server.rs
---

<!-- skill-link: create-issue -->

# Issue #2776 - Adopt axum-server 0.9 timeouts and close idle-connection gaps

Parent EPIC: #2411

## Goal

Use the connection timeouts that `axum-server` 0.9 provides so that every Axum listener (HTTP
tracker, REST API, health check API; plaintext and TLS) closes connections that never send a
request, and retire the tracker's copy of the gist-based `TimeoutAcceptor` if the evidence shows
it is no longer needed.

## Background

The Hyper 1.0 migration removed the idle-connection timeout from `axum-server`. The tracker
restored it for plaintext listeners with a copy of a gist, `TimeoutAcceptor`, but not for TLS
listeners and not for the health check API. A connection that is never closed holds a Tokio task
and a file descriptor for as long as the client keeps it open, so many silent connections can
exhaust the process's descriptors. The baseline reproduction recorded in
[manual-verification-evidence.md](manual-verification-evidence.md) shows the current state on
`develop` (`axum-server` 0.8.0):

| Listener                                | Silent connection closed?              |
| --------------------------------------- | -------------------------------------- |
| HTTP tracker, plaintext                 | Yes, after 5 s                         |
| REST API, plaintext                     | Yes, after 5 s                         |
| HTTP tracker, TLS, after the handshake  | No, still open after 25 s              |
| Health check API                        | No, still open after 25 s              |
| TLS port, no handshake bytes            | Yes, after 10 s (fixed upstream value) |

The history is spread over issues that pre-date issue specs:

- #324 (open): "Denial of Service attack factor". The original report: a client that connects
  (for example with `telnet`) and sends nothing is never disconnected. It is not true of every
  current listener (see the table). Reopened to consolidate #612 and #613.
- #612 and #613 (open): the same problem for the Tracker API and the HTTP tracker (TLS handshake,
  waiting for a first request, waiting for headers). Children of #603 "Review missing timeouts".
- `axum-server` issue #116, opened by the maintainer: "How to set up a timeout for TLS handshake".
  It tracked the removed timeout knobs, the `header_read_timeout` panic ("timeout set, but no
  timer set") and the missing first-request timeout. It was closed as completed on 2026-11-06.

The workaround in the tracker is `TimeoutAcceptor` in
`packages/axum-server/src/custom_axum_server.rs`, a copy of
[programatik29's gist](https://gist.github.com/programatik29/36d371c657392fd7f322e7342957b6d1).
It closes the connection but does not answer `408`. It does not work with TLS, so TLS listeners
omit it (`packages/axum-rest-api-server/src/server.rs`, the comment on the TLS branch, and
`packages/axum-http-server/src/server.rs`). The EPIC #2411 case A2 records the resulting gap:
idle TLS connections and health check API connections are not bounded by the tracker.

`axum-server` 0.9.0 (released 2026-11-06) contains the upstream fixes:

- PR 192: a `TokioTimer` is installed by default, so `header_read_timeout` and
  `keep_alive_timeout` no longer panic. The TLS handshake is cancelled on shutdown.
- PR 195: `Server::first_request_timeout` closes a connection that has not sent a complete first
  request in time, including the protocol-detection read Hyper performs before any HTTP timeout
  applies. It is opt-in, and it starts after the connection acceptor finishes, so TLS handshakes
  stay bounded by the acceptor's own handshake timeout. It also adds `Server::accept_error_handler`
  and a `timeouts` example.
- Breaking: TLS acceptors now return a `TlsService` wrapper service.
- `accept::TcpConfigAcceptor` applies TCP socket options to accepted connections.

The tracker's lockfile still resolves `axum-server` 0.8.0. Five manifests declare it as
`axum-server = "0"` with `tls-rustls-no-provider`: the root `Cargo.toml` and the
`axum-server`, `axum-http-server`, `axum-rest-api-server` and `axum-health-check-api-server`
packages. The `"0"` requirement permits 0.8, which has no `first_request_timeout`.

Source: facts about `axum-server` 0.9 come from its release notes and the two PRs above and
have not been exercised against the tracker yet. The current-state table was observed on a
`develop` build and is recorded in the evidence file. Everything about 0.9 behaviour is resolved
by the scenarios below.

## Scope

### In Scope

- Bump `axum-server` to 0.9 and fix any breakage from the `TlsService` wrapper.
- Set `first_request_timeout` on the HTTP tracker, REST API and health check API listeners, for
  both plaintext and TLS.
- Decide, with evidence, whether `TimeoutAcceptor`, `TimeoutStream`, `TimeoutService` and
  `TimeoutBody` can be deleted, leaving `add_timeouts` as the only wrapper.
- Make the timeout values named constants or configuration, per the decision recorded in the
  progress log (they are fixed at 5 s today).
- Check the TLS handshake timeout behaviour for the tracker's TLS listeners and record it.
- Add automatic tests for the idle-connection behaviour and manual evidence for real clients.
- Update EPIC #2411 case A2 and its existing-controls rows, remove the stale TLS comments, and
  tick #612 and #613 in #603 when they are closed as duplicates or resolved.

### Out of Scope

- Rate limiting and connection caps (EPIC #2411 case A1).
- Sending a `408` for a silent connection. The wrapper closes the connection; upstream does the
  same. Record this as a known limitation.
- UDP tracker timeouts and client-side (Tracker Checker) timeouts, already handled in #603.
- Per-request handler timeouts (`TimeoutLayer`), which already answer `408`.

## Architectural Decisions

- Related ADRs: none known for timeouts.
- ADRs created: [Close idle connections without a 408 response](../../../../packages/axum-server/docs/adrs/20261111095222_close_idle_connections_without_a_408_response.md) (package-local, 2026-11-11). None other known. Create another if the tracker keeps a custom acceptor (or drops it)
  with consequences worth recording, or if timeouts become configuration keys.

## Design and Ownership Review

- Ownership: `packages/axum-server` owns listener construction and the timeout policy; the three
  server packages only call it. The preferred outcome is one constructor path for all three, so
  the health check API stops calling `axum_server::from_tcp` directly
  (`packages/axum-health-check-api-server/src/server.rs`).
- Lifetime invariant: an idle connection holds a task and a file descriptor until it is closed;
  after the change the bound is the configured first-request time (plaintext and TLS), and no
  connection survives graceful shutdown beyond the existing drain timeout.
- Deadlines: tests must use a short, injected or constant timeout and an absolute test deadline,
  never wait on the 5 s production value without a bound.
- Checkpoint: review the design after the first passing vertical slice (the HTTP tracker on
  plaintext) before applying it to the remaining listeners.

## Bug-Fix Process

Follow `.github/skills/dev/debugging/fix-bug/SKILL.md`:

- Analysis: idle TLS connections and health check API connections are not closed by the tracker.
  Local hypothesis: only the plaintext listeners use `TimeoutAcceptor`; the health check API sets
  no timeout; TLS listeners omit the acceptor.
- Reproduction: **Reproduced** at baseline for TLS and the health check API; see the evidence
  file. The probes are repeated unchanged for the final recheck.
- Regression boundary: see below.
- Red evidence before the fix when feasible, production change, green evidence, and a
  like-for-like recheck.

## Regression Test Strategy

Prefer a test at the listener-construction seam in `packages/axum-server` that starts a server
with a short first-request timeout, connects with a plain `TcpStream` that sends nothing, and
asserts the server closes it within a bounded time. Repeat for a TLS connection that completes
the handshake and then stays silent. Run the same assertion against the health check API through
its package test support. If a TLS test needs fixture certificates, use the existing test
certificates in the repository. Record red and green output in `manual-verification-evidence.md`.

Also add bounded automated tests for the stalled-handshake case (a TCP connection to a TLS
listener that sends no handshake bytes is closed within the handshake bound) and for the
post-response idle case (a persistent HTTP/1 connection is closed after its response within the
idle bound, and an HTTP/2 connection is covered the same way). These cases decide whether the
wrapper can be removed, so manual checks alone are not enough. If a maintained test is
impractical for one of them (for example a handshake bound fixed inside `axum-server`), record
why in the progress log, and what follow-up would make it testable, as `fix-bug` requires.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                                                         | Notes / Expected Output                                                                  |
| --- | ------ | -------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------- |
| T1  | DONE   | Reproduce current behaviour on plaintext, TLS and health check listeners                     | Baseline recorded in `manual-verification-evidence.md`: TLS and health check reproduced   |
| T2  | DONE   | Bump `axum-server` to 0.9 and fix compile or behaviour breakage                              | `"0.9"` required in the root and all four Axum package manifests; `Cargo.lock` 0.9.x    |
| T3  | DONE   | Add red regression tests: idle plaintext and TLS, stalled handshake, post-response idle      | Recorded red run before the fix; TLS and health check tests fail on the current code     |
| T4  | DONE   | Apply `first_request_timeout` through one constructor path used by all three server packages | All listeners close idle connections; tests green                                        |
| T5  | DONE   | Compare idle bounds with and without `TimeoutAcceptor`, HTTP/1 and HTTP/2, then decide      | Evidence from M4 for both builds and both protocols; wrapper removed, or kept with reason |
| T6  | DONE   | Decide timeout configurability; record the TLS handshake timeout behaviour                   | Progress-log decision; constants or configuration keys documented                         |
| T7  | DONE   | Update EPIC #2411 case A2 and controls, remove stale comments, update docs                   | EPIC rows match the new behaviour; no stale "TLS does not work" comments                  |
| T8  | TODO   | Close or link #324, #612, #613 and #603 checklist                                            | Maintainer action; issue links recorded in the progress log                              |
| T9  | DONE   | Green run and final like-for-like recheck after the last production change                  | T1 probes repeated on the final build, named by its Conventional Commit subject          |

## Commit Points

| Task | Coherent change set                                           | Commit policy                                        |
| ---- | ------------------------------------------------------------- | ---------------------------------------------------- |
| T2   | Dependency bump and compile fixes only                        | Commit after focused validation and required review. |
| T3   | Red regression tests (may be committed with T4 if they fail)  | Commit after focused validation and required review. |
| T4   | Single constructor path and listener changes                  | Commit after focused validation and required review. |
| T5   | Removal of the custom acceptor, or the no-change decision     | Commit after focused validation and required review. |
| T6   | Constants or configuration plus documentation                 | Commit after focused validation and required review. |
| T7   | EPIC and documentation updates                                | Commit after focused validation and required review. |
| T9   | Final recheck evidence (and fixes from review findings)       | Repeat the recheck after any later production change. |

Record a justified no-change decision in the task's evidence without creating an empty commit.
For test-producing work, use the `write-unit-test` skill and complete the prose-first
Arrange-Act-Assert design review after each passing test increment, before maintainer review and
commit. Use a Conventional Commit message with the narrow affected scope, and sign every commit
with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/open/2776-2411-axum-server-0-9-timeouts/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec (#2776)
- [ ] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [x] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-11-06 18:30 UTC - Claude Code (Claude Sonnet 5.5) - Drafted after `axum-server` issue #116 was closed and 0.9.0 was released; analysed #324, #603, #612, #613 and EPIC #2411 case A2. Nothing was run against the tracker at that point.
- 2026-11-06 18:45 UTC - Claude Code (Claude Sonnet 5.5) - Maintainer approved the draft; created issue #2776 and moved the spec to `open/`.
- 2026-11-06 21:10 UTC - Claude Code (Claude Sonnet 5.5) - Answered the Copilot review on PR #2777: ran the baseline reproduction (Reproduced for TLS and the health check API; plaintext HTTP tracker and REST API close at 5 s), added the evidence file, required `"0.9"` in five manifests, added stalled-handshake and post-response regression tests, a final recheck task (T9), with/without-wrapper comparison for HTTP/1 and HTTP/2, and split M5. - [evidence](manual-verification-evidence.md)
- 2026-11-10 17:30 UTC - Claude Code (Claude Sonnet 5.5) - Implemented. T2: `axum-server` 0.9.0 required in five manifests (no source change needed; the tracker does not wrap a TLS acceptor, so `TlsService` did not matter). T3/T4: `custom_axum_server::Timeouts` (injectable) with `from_tcp_with`/`from_tcp_rustls_with`; `first_request_timeout` applied in the shared constructor path, and the health check API now uses it; red run recorded, tests green. Checkpoint after the plaintext HTTP slice: the design held for all listeners without change. T5: with/without-wrapper comparison identical for HTTP/1 and HTTP/2, so `TimeoutAcceptor`, `TimeoutStream`, `TimeoutService`, `TimeoutBody` were removed (no ADR: the replacement is the upstream feature and the behaviour is recorded in the EPIC controls). T6 decision: the values stay constants in `Timeouts::default` (5 s first request, header read and HTTP/2 keep-alive; 10 s TLS handshake), injectable in code for tests, not configuration keys, because nothing in the evidence asks operators to tune them; revisit under EPIC #2411 case A1. Known limitations: the server closes without `408`; HTTP/2 idle bound is 10 s; a client that answers HTTP/2 pings is not time-bounded. T7: EPIC case A2 and controls updated. T9: final recheck on `refactor(axum-server): [#2776] remove the TimeoutAcceptor wrapper`. T8 is the maintainer's. No retrospective needed: no discovery changed the design beyond the HTTP/2 measurement already recorded in the evidence. - [evidence](manual-verification-evidence.md)
- 2026-11-10 21:40 UTC - Claude Code (Claude Sonnet 5.5) - Review round 1 (PR 2813: Copilot lite review and da2ce7, 14 findings). F1: reran the HTTP/2 probes with the wrapper; the wrapper does not shorten idle HTTP/2 (10.0 s, probable cause in the evidence), so the removal stands, but AC4 and AC5 are reopened (F5) for the maintainer's ruling and the unmeasured "already 10 s on develop" claim is dropped. Also: lockfile restored to develop's resolution for the two unrelated edges (F4), citations by symbol in the EPIC (F3), commit ids dropped (F2), red and green output with commands recorded (F6), HTTP/2 tests now require a response and a close no earlier than the injected timeout (F7), probe note replaced (F8), plus the Copilot suggestions (probe script trap, drop guard in the health check test, longer test timeouts, fixture README, doc note on `from_tcp_with_timeouts`). Audit: `docs/pr-reviews/pr-2813-review/PR-REVIEW.md`.
- 2026-11-11 09:00 UTC - Claude Code (Claude Sonnet 5.5) - Maintainer ruling on AC4: an idle HTTP/2 connection closed after about 10 s is accepted (the extra 5 s changes the cost of a descriptor-exhaustion attack little; the real bound is a connection limit, EPIC #2411 case A1). AC4 reworded to match what ships, AC4 and AC5 ticked.
- 2026-11-11 10:00 UTC - Claude Code (Claude Sonnet 5.5) - Added the package ADR on closing idle connections without a `408` (maintainer discussion: silent close kept; revisit via an upstream `axum-server` feature if proxies report confusing logs). Develop merged into the branch to resolve conflicts with #2801 and #2812.

## Acceptance Criteria

- [x] AC1: A client that connects to the HTTP tracker, REST API or health check API over plaintext and sends nothing is disconnected within the configured first-request timeout.
- [x] AC2: The same holds for TLS connections of the HTTP tracker and REST API after the handshake completes.
- [x] AC3: A slow or stalled TLS handshake is bounded, and the bound is recorded in the documentation.
- [x] AC4: An idle persistent connection after a response is closed in bounded time, no worse than before the change, measured on the replacement build: HTTP/1 after 5 s, HTTP/2 after about 10 s (5 s keep-alive interval plus 5 s ping reply wait). The maintainer accepted the HTTP/2 figure on 2026-11-11; a client that answers pings is not time-bounded (EPIC #2411 case A1).
- [x] AC5: `TimeoutAcceptor` and its helper types are removed only after AC4 is shown by a comparison with and without the wrapper for HTTP/1 and HTTP/2; otherwise the spec records why they stay.
- [x] AC6: EPIC #2411 case A2 and the existing-controls rows describe the new behaviour; the stale "TLS does not work" comments are gone.
- [ ] AC7: #324, #612 and #613 are closed or linked to this issue, and the #603 checklist is updated.
- [x] `linter all` exits with code `0`
- [x] Relevant tests pass
- [x] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [x] Documentation is updated when behavior/workflow changes

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test --workspace` for the changed packages, including the new idle-connection tests
- Pre-push checks (when applicable)

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                              | Human-oriented command/steps                                                                  | Expected Result                                                     | Status | Evidence                                     |
| --- | ------------------------------------- | --------------------------------------------------------------------------------------------- | ------------------------------------------------------------------- | ------ | -------------------------------------------- |
| M1  | Idle plaintext connection             | Start the tracker; `telnet 127.0.0.1 <port>` to HTTP tracker, REST API and health check API | `Connection closed by foreign host` within the configured timeout   | DONE | `manual-verification-evidence.md` V8, V9 |
| M2  | Idle TLS connection after handshake   | `openssl s_client -connect 127.0.0.1:<tls-port>` and send nothing                             | Connection closed within the configured timeout                     | DONE | `manual-verification-evidence.md` V8, V9 |
| M3  | Stalled TLS handshake                 | Open a TCP connection to a TLS port and send nothing, or one byte                             | Connection closed within the handshake bound                        | DONE | `manual-verification-evidence.md` V8, V9 |
| M4  | Idle persistent connection after a response | Send one request, keep the connection open and send nothing more, once over HTTP/1 and once over HTTP/2 (`curl --http2-prior-knowledge` with a held connection, or the probe in `evidence/baseline-probe.py`); run on the build with the wrapper and on the build without it | Closed in bounded time, no worse than 5 s, on both builds and both protocols | DONE (HTTP/1 and HTTP/2) | `manual-verification-evidence.md` V8, V9 |
| M5a | Tracker traffic unaffected            | Run the e2e runner, which exercises the HTTP and UDP trackers and the health check API only    | Same results as before the change                                   | NOT RUN (needs Docker; CI) | `manual-verification-evidence.md` V5 to V7 |
| M5b | REST API call unaffected              | An authenticated REST API request with `curl` and the admin token, separately from the runner | Same response as before the change                                  | DONE   | `manual-verification-evidence.md` V5 to V7 |
| M5c | Slow handler still answers 408        | A deliberately slow request against a test listener (for example a test route that sleeps past the handler timeout) | `408` after the handler timeout                                      | DONE (existing unit test) | `manual-verification-evidence.md` V5 to V7 |

Notes:

- Manual verification is mandatory even when automated tests pass.
- Record toolchain-qualified validation commands (for example the nightly Rust toolchain for
  `cargo +nightly fmt --all -- --check`) in the evidence file.
- Create `manual-verification-evidence.md` from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`.

### Disposable Verification Scripts

None planned.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence           |
| ----- | ---------------------- | ------------------ |
| AC1   | DONE                    | Tests `it_should_close_a_plaintext_connection_that_sends_nothing` (axum-server), `it_should_close_a_connection_that_sends_nothing` (health check API); evidence V8, V9: 5.0 s on all three listeners |
| AC2   | DONE                    | Test `it_should_close_a_tls_connection_that_completes_the_handshake_and_sends_nothing`; red run in `evidence/red-run.txt`; V9: TLS silent closed after 5.0 s |
| AC3   | DONE                    | Tests for no handshake bytes and one handshake byte; V9 M3: 10.0 s (`Timeouts::tls_handshake`); recorded in evidence "TLS handshake timeout" |
| AC4   | DONE                    | V8: HTTP/1 idle after response 5.0 s; HTTP/2 10.0 s with and without the wrapper; maintainer accepted HTTP/2 about 10 s on 2026-11-11 |
| AC5   | DONE                    | V8 comparison; wrapper and helper types removed in `refactor(axum-server): [#2776] remove the TimeoutAcceptor wrapper` |
| AC6   | DONE                    | EPIC #2411 case A2 and Existing Controls rows rewritten; the stale "TLS does not work" comment was removed with the wrapper |
| AC7   | TODO                    | Maintainer action (T8): #324, #612, #613 stay open until the maintainer closes them after the fix; #603 checklist then updated |

## Risks and Trade-offs

- `axum-server` 0.9.0 was released the day this spec was written. Mitigation: pin the exact
  minor in the lockfile, read the diff against 0.8.0, and run the full e2e suite before merging.
- The `TlsService` wrapper is a breaking change. Mitigation: the tracker does not wrap a TLS
  acceptor today; confirm that when bumping.
- `first_request_timeout` covers only the first request. Mitigation: M4 decides whether Hyper's
  `header_read_timeout` already covers the idle gap between keep-alive requests; if not, keep the
  wrapper for that case.
- A silent close instead of `408` may confuse some proxies. The tracker is normally not behind
  one that depends on it; record as a known limitation.
- Making timeouts configurable adds configuration surface. Mitigation: decide in T6 whether
  constants are enough.

## Implementation Completion Review

- Retrospective: Not needed (see the 2026-11-10 progress-log entry).
- If needed, create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md`.
- If none is needed, add a progress-log entry explaining why.

## References

- Related issues: #324, #603, #612, #613, #2411
- Upstream: `axum-server` issue 116, PRs 192 and 195, release 0.9.0
  (<https://github.com/programatik29/axum-server/issues/116>)
- Hyper: <https://github.com/hyperium/hyper/pull/3185> (header read timeout starts immediately, 1.4.0)
- Axum: <https://github.com/tokio-rs/axum/issues/2741>, <https://github.com/tokio-rs/axum/discussions/1383>
- Reproduction repository: <https://github.com/josecelano/axum-server-timeout>
- EPIC: `docs/issues/open/2411-spam-and-abuse-resistance/EPIC.md` (case A2)
