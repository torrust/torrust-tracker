---
schema-version: 1
doc-type: issue
issue-type: bug
status: planned
priority: p2
epic: 2411
github-issue: 2776
spec-path: docs/issues/open/2776-2411-axum-server-0-9-timeouts/ISSUE.md
branch: "2776-axum-server-0-9-timeouts-spec"
related-pr: null
last-updated-utc: "2026-11-06 18:30"
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

The tracker's Axum listeners have had no idle-connection timeout since the Hyper 1.0 migration
removed it from `axum-server`. The history is spread over issues that pre-date issue specs:

- #324 (open): "Denial of Service attack factor". A client that connects (for example with
  `telnet`) and sends nothing is never disconnected. Reopened to consolidate #612 and #613.
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

The tracker's lockfile still resolves `axum-server` 0.8.0, and all four packages depend on
`axum-server = "0"` with `tls-rustls-no-provider`.

Source: facts about `axum-server` come from its 0.9.0 release notes and the two PRs above; they
have not been reproduced against the tracker. Every "confirmed" or "hypothesis" in this spec is
resolved by the manual scenarios below.

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
- ADRs to create: none known. Create one if the tracker keeps a custom acceptor (or drops it)
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
- Reproduction: open a raw TCP and a TLS connection to each listener, send nothing, and observe
  whether the server closes them (see M1 to M3). Record attempted commands.
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

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                                                         | Notes / Expected Output                                                                  |
| --- | ------ | -------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------- |
| T1  | TODO   | Reproduce current behaviour on plaintext, TLS and health check listeners                     | Evidence of which idle connections stay open today (M1 to M3)                             |
| T2  | TODO   | Bump `axum-server` to 0.9 and fix compile or behaviour breakage                              | Workspace builds; `Cargo.lock` resolves 0.9.x                                            |
| T3  | TODO   | Add red tests for idle plaintext and TLS connections                                         | Tests fail on the current code for TLS and health check                                  |
| T4  | TODO   | Apply `first_request_timeout` through one constructor path used by all three server packages | All listeners close idle connections; tests green                                        |
| T5  | TODO   | Verify idle HTTP/1 keep-alive between requests and decide whether `TimeoutAcceptor` goes     | Evidence from M4; wrapper removed, or kept with a recorded reason                         |
| T6  | TODO   | Decide timeout configurability; record the TLS handshake timeout behaviour                   | Progress-log decision; constants or configuration keys documented                         |
| T7  | TODO   | Update EPIC #2411 case A2 and controls, remove stale comments, update docs                   | EPIC rows match the new behaviour; no stale "TLS does not work" comments                  |
| T8  | TODO   | Close or link #324, #612, #613 and #603 checklist                                            | Maintainer action; issue links recorded in the progress log                              |

## Commit Points

| Task | Coherent change set                                           | Commit policy                                        |
| ---- | ------------------------------------------------------------- | ---------------------------------------------------- |
| T2   | Dependency bump and compile fixes only                        | Commit after focused validation and required review. |
| T3   | Red regression tests (may be committed with T4 if they fail)  | Commit after focused validation and required review. |
| T4   | Single constructor path and listener changes                  | Commit after focused validation and required review. |
| T5   | Removal of the custom acceptor, or the no-change decision     | Commit after focused validation and required review. |
| T6   | Constants or configuration plus documentation                 | Commit after focused validation and required review. |
| T7   | EPIC and documentation updates                                | Commit after focused validation and required review. |

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

- 2026-11-06 18:30 UTC - Claude Code (Claude Sonnet 5.5) - Drafted after `axum-server` issue #116 was closed and 0.9.0 was released; analysed #324, #603, #612, #613 and EPIC #2411 case A2. Nothing was run against the tracker.
- 2026-11-06 18:45 UTC - Claude Code (Claude Sonnet 5.5) - Maintainer approved the draft; created issue #2776 and moved the spec to `open/`.

## Acceptance Criteria

- [ ] AC1: A client that connects to the HTTP tracker, REST API or health check API over plaintext and sends nothing is disconnected within the configured first-request timeout.
- [ ] AC2: The same holds for TLS connections of the HTTP tracker and REST API after the handshake completes.
- [ ] AC3: A slow or stalled TLS handshake is bounded, and the bound is recorded in the documentation.
- [ ] AC4: An idle HTTP/1 keep-alive connection between requests is closed in bounded time, or the gap is recorded as a known limitation in EPIC #2411.
- [ ] AC5: `TimeoutAcceptor` and its helper types are removed, or the spec records why they stay.
- [ ] AC6: EPIC #2411 case A2 and the existing-controls rows describe the new behaviour; the stale "TLS does not work" comments are gone.
- [ ] AC7: #324, #612 and #613 are closed or linked to this issue, and the #603 checklist is updated.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test --workspace` for the changed packages, including the new idle-connection tests
- Pre-push checks (when applicable)

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                              | Human-oriented command/steps                                                                  | Expected Result                                                     | Status | Evidence                                     |
| --- | ------------------------------------- | --------------------------------------------------------------------------------------------- | ------------------------------------------------------------------- | ------ | -------------------------------------------- |
| M1  | Idle plaintext connection             | Start the tracker; `telnet 127.0.0.1 <port>` to HTTP tracker, REST API and health check API | `Connection closed by foreign host` within the configured timeout   | TODO   | `manual-verification-evidence.md` section V1 |
| M2  | Idle TLS connection after handshake   | `openssl s_client -connect 127.0.0.1:<tls-port>` and send nothing                             | Connection closed within the configured timeout                     | TODO   | `manual-verification-evidence.md` section V2 |
| M3  | Stalled TLS handshake                 | Open a TCP connection to a TLS port and send nothing, or one byte                             | Connection closed within the handshake bound                        | TODO   | `manual-verification-evidence.md` section V3 |
| M4  | Idle HTTP/1 keep-alive between requests | One request over a kept-alive connection, then wait                                         | Connection closed in bounded time                                   | TODO   | `manual-verification-evidence.md` section V4 |
| M5  | Normal traffic unaffected             | Announce, scrape, REST API call and health check through the e2e runner                       | Same responses as before the change; slow handlers still answer 408 | TODO   | `manual-verification-evidence.md` section V5 |

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
| AC1   | TODO                   | {test/log/PR link} |
| AC2   | TODO                   | {test/log/PR link} |
| AC3   | TODO                   | {test/log/PR link} |
| AC4   | TODO                   | {test/log/PR link} |
| AC5   | TODO                   | {test/log/PR link} |
| AC6   | TODO                   | {test/log/PR link} |
| AC7   | TODO                   | {test/log/PR link} |

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

- Retrospective: `Not yet assessed`
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
