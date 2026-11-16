---
schema-version: 1
doc-type: issue
issue-type: bug
status: done
priority: p2
epic: null
github-issue: 2867
spec-path: docs/issues/closed/2867-udp-health-check-panics-on-invalid-response/ISSUE.md
branch: "2867-udp-health-check-panics-on-invalid-response"
related-pr: 2879
last-updated-utc: "2026-11-16 15:32"
semantic-links:
  skill-links:
    - create-issue
    - fix-bug
  related-artifacts:
    - .github/skills/dev/debugging/fix-bug/SKILL.md
    - .github/skills/dev/planning/create-issue/SKILL.md
    - packages/tracker-client/src/udp/client.rs
    - packages/axum-health-check-api-server/src/handlers.rs
    - packages/udp-server/src/server/launcher.rs
    - packages/axum-health-check-api-server/tests/server/contract.rs
---

<!-- skill-link: create-issue -->

# Issue #2867 - UDP health check panics when the tracker answers with an invalid response

## Goal

When a UDP tracker answers the health check with a datagram that is not a valid tracker response,
`GET /health_check` returns its normal report with an error detail for that service, instead of
panicking and dropping the HTTP connection.

## Background

This was found on 2026-11-15 during the red-then-green check of issue #2393 (test port races,
EPIC #2392). A mutation made the probed UDP address answer with junk bytes, and the health-check
request failed by a panic instead of an error report. It is out of scope for #2393, whose
implementation only touches tests.

### What happens, step by step

1. A client sends `GET /health_check` to the health-check API.
2. The handler (`packages/axum-health-check-api-server/src/handlers.rs`) spawns one task per
   registered service. For a UDP tracker the job is `Launcher::check`
   (`packages/udp-server/src/server/launcher.rs`), which spawns
   `torrust_tracker_client::udp::client::check`.
3. `check` (`packages/tracker-client/src/udp/client.rs`) sends a connect request and races
   `client.receive()` against a 2 s sleep.
4. A datagram arrives that does not parse (for example the 4 bytes `junk`).
   `UdpTrackerClient::receive` correctly returns `Err(Error::UnableToParseResponse { .. })`.
5. `check` calls `process(response.unwrap())`, so the check task **panics**.
6. The per-service report task awaits that job with
   `.expect("it should be able to join into the checking function")`, so it **panics** too.
7. The handler joins the report tasks with `.expect("it should be able to connect to the job")`,
   so the request handler **panics**.
8. The router has no panic-catching layer, so the HTTP connection is dropped. The client gets no
   status, no body, no report (`hyper::Error(IncompleteMessage)` in the reproduction).

### Which contract it violates

- `check` returns `Result<String, String>`: its caller expects an `Err` for a failed check, and it
  already returns `Err("Timed Out")` and `Err("Did not Connect")` for the other failure modes. Its
  doc comment even says "It will return an error if unable to connect to the UDP service", under a
  `# Panics` heading.
- The health-check endpoint reports per-service results: a failed service should give status
  `Error`, message `health check failed`, and a detail whose `result` is `Err(..)`. This is how the
  HTTP, API and stopped-UDP cases already behave (see the contract tests in
  `packages/axum-health-check-api-server/tests/server/contract.rs`).

### Impact

- Any UDP peer at the registered address that answers with a non-tracker datagram (a
  misconfigured port shared with another service, a proxy, a corrupt or truncated reply) makes the
  whole health-check request fail without a report.
- The results of the **other** services are lost too, because the whole request fails.
- The process logs three panics per request. The server keeps running (the panic stays inside the
  request task), and a container health check still sees a failure, but with no detail to
  diagnose it.

### Reproduction

**Reproduced** on 2026-11-15 against `develop` at `abd7a10ff`, with a temporary integration test
that registers a junk-answering UDP socket with the production `Launcher::check`. Commands, output
and the test code are in [`manual-verification-evidence.md`](manual-verification-evidence.md)
(V1) and [`evidence/`](evidence/). A closed UDP port does not panic: it reports exactly `Err("Timed Out")` (V2, with the value
asserted in V3).

## Scope

### In Scope

- `torrust_tracker_client::udp::client::check` returns `Err(..)` instead of panicking when
  `receive()` fails (a parse error, and by the same change any other receive error). The error text
  should name the cause, for example by using the `Display` text of the client `Error`.
- Fix the stale doc comment of `check` (`# Panics` / `# Errors`).
- The health-check handler reports a check job that panicked or was cancelled (`JoinError`) as a
  failed check for that service (an `Err(..)` result with a short description of the join error),
  instead of panicking. The outer join of the report tasks must not panic either. A panicking probe
  then gives a report with status `Error` and the other services' results intact.
- Regression tests for both (see Regression Test Strategy).

### Out of Scope

- Other `unwrap`/`expect` calls in `tracker-client` (for example `local_addr().expect(..)` in
  `UdpClient::bound`), unless they are on the same path and trivially fixed. They can be a
  follow-up.
- Changing the 2 s `select!` sleep against the 5 s client timeout, or the `Timed Out` text.
- Adding a panic-catching layer to the health-check router (a broader decision; the handler fix
  makes it unnecessary for this case).
- The HTTP and REST API check functions, which already return errors.
- The port-race tests of issue #2393.

## Architectural Decisions

- Related ADRs: none found that govern health-check error reporting.
- ADRs to create: none known.

## Design and Ownership Review

Not applicable as a new design: the change replaces panics with error values on an existing
asynchronous path and adds no new resource, process or fixture ownership. The regression test's
junk-answering UDP socket is owned by the test and lives for the test only; its answering thread
ends when the socket is dropped or the test process exits. Every await in the check is already
bounded: the receive races the 2 s `select!` sleep, and the awaits before it (bind and connect in
`UdpTrackerClient::new`, and `client.send`) are bounded by the 5 s `DEFAULT_UDP_TIMEOUT`.

## Bug-Fix Process

Follow [fix-bug](../../../../.github/skills/dev/debugging/fix-bug/SKILL.md):

1. **Analysis** (done): the deciding code is the `process(response.unwrap())` arm in `check`, and
   the two `.expect(..)` calls in `probe_services`. Hypothesis confirmed by V1.
2. **Real-artifact reproduction** (done): V1 in `manual-verification-evidence.md`.
3. **Regression-test boundary**: see Regression Test Strategy.
4. **Red**: add the regression tests first and record their failing output.
5. **Fix**: the smallest change in `check` and in `probe_services`.
6. **Green and recheck**: rerun the regression tests, the health-check contract tests, and the V1
   reproduction like for like.

## Regression Test Strategy

Three maintained tests: one per causal seam, and one for the whole path that AC2 describes.

1. **Unit test in `tracker-client`** (causal seam of the panic): start a UDP socket on
   `127.0.0.1:0` that answers every datagram with junk bytes, call `check` with its
   `ServiceBinding`, and assert the result is `Err(..)` and names the unparsable response. Today it
   panics (red). It is deterministic: the reply arrives well before the 2 s sleep.
2. **Contract test in `axum-health-check-api-server`**: register a service whose probe job panics
   (a plain `ServiceHealthCheckJob` with `tokio::spawn(async { panic!(..) })`, like the counting
   probes in the `readiness` module) next to a passing one, and assert `GET /health_check` returns
   `200` with status `Error`, an `Err(..)` detail for the panicking service, and an `Ok(..)` detail
   for the other. Today the request fails with `IncompleteMessage` (red). This keeps the handler
   robust against any probe that panics, not only the UDP one.

3. **UDP contract test in `axum-health-check-api-server`** (the AC2 case): keep the V1 test,
   `it_should_return_error_when_the_registered_udp_service_answers_the_check_with_junk`, next to
   `it_should_return_error_when_udp_service_was_stopped_after_registration`. It registers a
   junk-answering UDP socket with the production `Launcher::check` and asserts `GET /health_check`
   returns `200`, status `Error`, message `health check failed`, and an `Err(..)` UDP detail. It is
   deterministic: the junk reply arrives long before the 2 s sleep. It must be added before the
   `check` fix, because the `check` fix alone turns it green; its red run is the V1 output
   (`IncompleteMessage`).

Tests 1 and 2 are the smallest deterministic boundaries. Test 3 covers the whole path a client
sees, and the V1 reproduction, rerun on the fixed build, is the like-for-like recheck.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                              | Notes / Expected Output                                                                                         |
| --- | ------ | ------------------------------------------------- | --------------------------------------------------------------------------------------------------------------- |
| T1  | DONE   | Reproduce against the real health-check server     | V1 in `manual-verification-evidence.md`: three panics and `IncompleteMessage`.                                   |
| T2  | DONE   | Add the `tracker-client` unit regression test      | Recorded red run: the test panics at `client.rs` on `UnableToParseResponse`.                                     |
| T3  | DONE   | Add the UDP junk-reply contract test (AC2)         | Recorded red run: `GET /health_check` fails with `IncompleteMessage`, as in V1.                                  |
| T4  | DONE   | Return an error from `check` on a receive failure  | T2 and T3 green; `check` doc comment fixed.                                                                      |
| T5  | DONE   | Add the handler panicking-probe contract test      | Recorded red run: `GET /health_check` fails with `IncompleteMessage` when a probe panics.                        |
| T6  | DONE   | Report a failed check job instead of panicking     | T5 green; report has status `Error`, the panicking service's detail is `Err(..)`, the other service is `Ok(..)`. |
| T7  | DONE   | Final verification and like-for-like recheck       | Health-check and tracker-client tests pass; V1 rerun gives a `200` report with an `Err(..)` UDP detail.          |

## Commit Points

| Task | Coherent change set                                         | Commit policy                                        |
| ---- | ----------------------------------------------------------- | ---------------------------------------------------- |
| T2   | Unit regression test (may land with T4 if it must be green) | Commit after focused validation and required review. |
| T3   | UDP contract test (may land with T4 if it must be green)    | Commit after focused validation and required review. |
| T4   | `check` returns an error on a receive failure               | Commit after focused validation and required review. |
| T5   | Handler contract regression test (may land with T6)         | Commit after focused validation and required review. |
| T6   | Handler reports a failed job as a failed check              | Commit after focused validation and required review. |
| T7   | Evidence and spec progress update                           | Commit after the final recheck.                      |

Record red runs in `manual-verification-evidence.md` before the fix commits. Use the
`write-unit-test` skill, with the prose-first Arrange-Act-Assert review after each test increment,
before maintainer review and commit. Use Conventional Commit messages with the narrow scope
(`tracker-client`, `axum-health-check-api-server`) and sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/udp-health-check-panics-on-invalid-response/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [x] Spec-only PR merged into `develop` before implementation
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [x] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [x] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [x] Reviewer validated acceptance criteria and updated checkboxes (da2ce7's approving reviews of PR 2879, the last at the merged head)
- [x] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification: not applicable; reviewers delivered their findings through GitHub, recorded in `docs/pr-reviews/pr-2868-review/PR-REVIEW.md` and `docs/pr-reviews/pr-2879-review/PR-REVIEW.md`
- [x] Committer verified spec progress is up to date before commit
- [x] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-11-15 12:45 UTC - Claude Code (Claude Opus 5.5) - Drafted the spec and reproduced the
  defect (V1 reproduced, V2 closed-port scope check passes) - `manual-verification-evidence.md`,
  `evidence/`
- 2026-11-15 20:18 UTC - Claude Code (Claude Opus 5.5) - Maintainer approved the draft; created
  issue #2867 and moved the spec to `docs/issues/open/` for a spec-only PR
- 2026-11-15 21:31 UTC - Claude Code (Claude Opus 5.5) - Reran the closed-port check asserting
  `Err("Timed Out")` (V3) for the first da2ce7 review of PR 2868 - `manual-verification-evidence.md`
- 2026-11-16 08:11 UTC - Maintainer - Merged spec-only PR 2868. Two findings of the second da2ce7
  review (F6, F7 of that review) were deferred to this implementation, as recorded on PR 2868
- 2026-11-16 09:44 UTC - Claude Code (Claude Opus 5.5) - Implemented T2 to T7: three regression
  tests proven red, the `check` and handler fixes, and the V4 recheck. Tightened T3 to require the
  cause in the error detail and re-proved it red by mutate-then-restore. Addressed the deferred
  findings in `manual-verification-evidence.md`: the stored V1 to V3 logs start at `Finished` (F6)
  and the artifact line names the V3 diff (F7). No retrospective: nothing material was discovered
  beyond the plan; the one lesson (keep build lines in stored logs) is applied in the evidence
  file - `manual-verification-evidence.md`, `evidence/`
- 2026-11-16 10:19 UTC - Claude Code (Claude Opus 5.5) - Corrected four stamps that were written ahead of the
  commits that record them (PR 2879 review, F1): the entry above and the evidence file's
  `last-updated-utc` to 09:44, and the PR 2868 audit's last processing-log entry to 09:48
- 2026-11-16 14:48 UTC - Claude Code (Claude Opus 5.5) - Merged `develop` into the branch (one
  import conflict in the health-check contract tests). The #2393 rewrite it brings replaced the
  ignored stopped-UDP test; recorded the replacement as AC4's coverage with a new run, tied the
  evidence's line numbers to their commits, and moved T3 and T5 onto the shared test helpers
  (PR 2879 review, da2ce7 F12 to F14)
- 2026-11-16 15:31 UTC - Maintainer - Merged PR 2879, which closed issue #2867
- 2026-11-16 15:32 UTC - Claude Code (Claude Opus 5.5) - Closed out: moved the spec to
  `docs/issues/closed/`, ticked the remaining checkpoints, and marked the deferred PR 2868
  findings F7 and F8 fixed in their audit

## Acceptance Criteria

- [x] AC1: `check` returns `Err(..)`, and does not panic, when the UDP peer answers the connect
      request with bytes that are not a valid tracker response; the error text names the cause
      (test T2).
- [x] AC2: `GET /health_check` returns `200` with status `Error`, message `health check failed`,
      and an `Err(..)` detail for a UDP service that answers with junk bytes (test T3).
- [x] AC3: when a registered probe job panics, `GET /health_check` returns `200` with status
      `Error`, an `Err(..)` detail for that service, and the other services' details intact; the
      handler does not panic (test T5).
- [x] AC4: the existing health-check contract tests still pass, including the stopped-UDP case's
      `Timed Out` result.
- [x] `linter all` exits with code `0`
- [x] Relevant tests pass
- [x] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [x] Documentation is updated when behavior/workflow changes (the `check` doc comment; no user documentation describes the panic)

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test -p torrust-tracker-client-lib` (stable Rust toolchain)
- `cargo test -p torrust-tracker-axum-health-check-api-server` (stable Rust toolchain)
- `cargo test -p torrust-tracker-udp-server` (its launcher uses `check`)
- Pre-push checks (`./contrib/dev-tools/git/hooks/pre-push.sh`)

### Manual Verification Scenarios

| ID  | Scenario                                       | Human-oriented command/steps                                                                                                           | Expected Result                                                       | Status | Evidence                                     |
| --- | ---------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------- | ------ | -------------------------------------------- |
| M1  | Initial reproduction with a junk-answering UDP | Temporary test from `evidence/reproduction-tests.diff`; `cargo test -p torrust-tracker-axum-health-check-api-server --test integration answers_the_check_with_junk` | Before the fix: panics and `IncompleteMessage` (bug observed)         | DONE   | `manual-verification-evidence.md` section V1 |
| M2  | Closed-port scope check                        | Temporary test from `evidence/closed-port-value-check.diff` (asserts the value); `cargo test ... is_a_closed_udp_port`                 | `Err("Timed Out")` result, no panic                                   | DONE   | `manual-verification-evidence.md` sections V2 and V3 |
| M3  | Like-for-like recheck after the fix            | Rerun M1 on the fixed build                                                                                                            | `200` report, status `Error`, UDP detail `Err(..)` naming the cause   | DONE   | `manual-verification-evidence.md` section V4 |

Notes:

- Manual verification is mandatory even when automated tests pass.
- Record the toolchain for every command result.
- If a scenario fails, record the failure and diagnosis in the progress log before proceeding.

### Disposable Verification Scripts

None planned. The reproduction is a temporary Rust test, kept verbatim in
`evidence/reproduction-tests.diff`.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | DONE                   | T2 red and green logs (`evidence/t2-red-run.log`, `evidence/t2-green-run.log`) |
| AC2   | DONE                   | T3 red and green logs; V4 |
| AC3   | DONE                   | T5 red and green logs (`evidence/t5-red-run.log`, `evidence/t5-green-run.log`) |
| AC4   | DONE                   | Full health-check suite passes; the ignored stopped-UDP test passed with `Timed Out` (`evidence/stopped-udp-run.log`); since the `develop` merge, its replacement `it_should_return_error_when_the_registered_udp_service_never_answers_the_check` runs by default and passes (`evidence/never-answering-udp-run.log`) |

## Risks and Trade-offs

- Reporting a `JoinError` as a failed check could hide a programming error in a probe. Mitigation:
  keep the panic message in the `Err(..)` detail and log it at `error` level, so it stays visible.
- The error text of `check` changes from a panic to a message clients may read. Mitigation: the
  field is free text already (`Timed Out`, `Did not Connect`); no client parses it.

## Implementation Completion Review

- Retrospective: not needed; see the 2026-11-16 09:44 progress-log entry
- If needed, create `implementation-retrospective.md` from
  `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this folder; otherwise add a progress-log
  entry explaining why none was needed.

## References

- Related issues: #2393 (where it was found), #2392 (its EPIC)
- Code: `packages/tracker-client/src/udp/client.rs` (`check`),
  `packages/axum-health-check-api-server/src/handlers.rs` (`probe_services`),
  `packages/udp-server/src/server/launcher.rs` (`Launcher::check`)
