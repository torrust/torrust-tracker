---
schema-version: 1
doc-type: issue
issue-type: bug
status: planned
priority: p3
epic: null
github-issue: 2345
spec-path: docs/issues/open/2345-keep-request-kind-in-udp-error-response-event/ISSUE.md
branch: "2345-keep-request-kind-in-udp-error-response-event-spec"
related-pr: null
last-updated-utc: "2026-09-26 11:01"
semantic-links:
  skill-links:
    - create-issue
    - fix-bug
    - write-unit-test
  related-artifacts:
    - .github/skills/dev/debugging/fix-bug/SKILL.md
    - .github/skills/dev/planning/create-issue/SKILL.md
    - .github/skills/dev/testing/write-unit-test/SKILL.md
    - docs/adrs/20260727000000_events_are_objective_facts.md
    - docs/issues/open/2345-keep-request-kind-in-udp-error-response-event/manual-verification-evidence.md
    - packages/udp-server/src/server/processor.rs
    - packages/udp-server/src/handlers/mod.rs
    - packages/udp-server/src/event.rs
    - packages/udp-server/src/statistics/event/handler/response_sent.rs
---

<!-- skill-link: create-issue -->

# Issue #2345 - Keep the Request Kind in the UDP Error-Response Event

## Goal

When the UDP tracker answers a parsed request with an error response, the published
`Event::UdpResponseSent` must retain the request kind, as the event contract documents.

## Background

### The Bug in Plain Terms

When the UDP tracker answers a request it **could parse** with an error response, the internal
event it publishes about that response forgets what kind of request it was.

1. A client sends a UDP request, for example a scrape with an invalid connection ID.
2. `handlers::handle_packet` parses it, so it knows it is a **scrape**. The scrape handler rejects
   the connection ID, and `handle_packet` returns the error response together with
   `Some(UdpRequestKind::Scrape)`.
3. `Processor::send_response` sends the error response to the client and publishes
   `Event::UdpResponseSent`. For every error response it hard-codes the request kind to `None`,
   discarding the kind it was just given:

   ```rust
   Response::Error(_e) => event::UdpResponseKind::Error { opt_req_kind: None },
   ```

The event contract in `packages/udp-server/src/event.rs` says the opposite:

```rust
/// There was an error handling the request. The error contains the request
/// kind if the request was parsed successfully.
Error {
    opt_req_kind: Option<UdpRequestKind>,
},
```

`None` should mean only "the payload could not be parsed". Today it also covers parsed requests, so
the event drops a fact it is documented to carry, contrary to the "events are objective facts"
ADR. The other events for the same request (`UdpRequestAccepted`, `UdpError`) do carry the kind.

This looks like an incomplete change, not a design decision. Commit `27e2db4b`
(`refactor: [#1382] include req kin in UDP error response if it's known`) threaded the value
through `handle_packet` into `send_response` for this purpose, but left this match arm hard-coded.

The fix is one line: pass the received value through, `Error { opt_req_kind }`.

### Reproduction

Reproduced on 2026-09-26 against `develop`. The exact steps, code, commands, and output are in
[manual-verification-evidence.md](manual-verification-evidence.md). In short:

- **V1, Trigger only (real tracker):** the existing strict-mode loopback contract sends eleven announces
  with `ConnectionId::new(0)`, each answered with an error response through the affected path,
  then a twelfth that is banned before handling. The defective field is not visible there:
  clients, metrics, and logs do not expose published events.
- **V2, Reproduced (event bus):** a temporary test on the processor's event bus sent a parsed scrape
  with `ConnectionId::new(0)` from a real loopback client socket. It observed
  `UdpRequestAccepted { kind: Scrape }`, `UdpError { kind: Some(Scrape), .. }`, and then
  `UdpResponseSent { kind: Error { opt_req_kind: None }, .. }`. The temporary test was reverted;
  B2 turns it into the maintained regression test.

### Impact

- The published event contradicts its documented contract and drops known information.
- The current statistics consumer intentionally ignores the optional request kind for errors, so no
  current metric is wrong. A future consumer that trusts the event contract would receive `None`.

## Scope

### In Scope

- Preserve `opt_req_kind` in `Processor::send_response` for `Response::Error`.
- Add deterministic regression coverage for a parsed handler failure and an unparsable payload.
- Record real-artifact reproduction, red/green test evidence, and the final recheck in the
  issue-local `manual-verification-evidence.md`.

### Out of Scope

- Adding a `request_kind` label to error-response metrics. That changes the metrics contract and
  requires a separate proposal.
- Changing the defensive non-error/absent-kind classification branch in `send_response`.
- Continuing the paused #2283 processor-test refactor beyond the narrow regression coverage.

## Architectural Decisions

- Related ADRs: `docs/adrs/20260727000000_events_are_objective_facts.md`.
- ADRs to create: None known. The one-line data-preservation correction does not introduce a new
  architectural decision.

## Design and Ownership Review

`Processor` owns the response classification and event publication. `handle_packet` owns request
parsing and already supplies the optional request kind. `BoundSocket` owns response transmission.
The regression test owns its ephemeral loopback socket and direct event-bus receiver; every receive
must use the existing absolute `EVENT_PUBLICATION_TIMEOUT`. The test must consume the processor
event stream until `UdpResponseSent` because `UdpRequestAccepted` and `UdpError` are published
first (observed in V2). Review the first passing test increment before adding the unparsable-payload complement.

## Bug-Fix Process

Follow `.github/skills/dev/debugging/fix-bug/SKILL.md`:

1. **Analyze** (done at spec time): the decision point is the `Response::Error` arm in
   `Processor::send_response`; see "The Bug in Plain Terms".
2. **Reproduce** (done at spec time): V1 and V2 in `manual-verification-evidence.md`.
3. **Select the boundary and prove red** (B2): turn V2 into a maintained unit test and record
   its failing output before changing production code; complete the prose-first test review.
4. **Fix** (B3): pass `opt_req_kind` into `UdpResponseKind::Error`.
5. **Green and recheck** (B4): rerun the regression tests and package tests, then repeat V1
   unchanged and V2 through the maintained test.

## Regression Test Strategy

Add unit tests in `packages/udp-server/src/server/processor.rs`, the causal event-publication seam.
Use the existing `Processor` fixture, a real loopback client socket with a nonzero source port, and
the direct event receiver.

1. Send a parsable scrape with `ConnectionId::new(0)` under strict validation (the V2 request).
   Receive events until `Event::UdpResponseSent`, then assert
   `UdpResponseKind::Error { opt_req_kind: Some(UdpRequestKind::Scrape) }`. This is red before
   the fix. One parsed kind is enough: the defective arm does not depend on the kind, and scrape
   avoids the announce request's many incidental fields.
2. Send an unparsable payload and assert the corresponding response event has
   `UdpResponseKind::Error { opt_req_kind: None }`. This preserves the valid unknown-kind case.

The unit boundary is preferred over a package integration test because it directly observes the
processor-owned fact without adding a statistics listener or depending on a consumer that currently
discards the field. Do not assert processing duration. Record the test's red and green output and
the final artifact recheck in `manual-verification-evidence.md`.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| --- | ------ | ---- | ----------------------- |
| B1 | DONE | Confirm and record reproduction evidence | V1 and V2 recorded in `manual-verification-evidence.md` at spec time. |
| B2 | TODO | Add the regression tests and prove them red | Parsed-scrape failure test fails with `None` before the fix; unparsable payload keeps `None`; record red output and prose-first review. |
| B3 | TODO | Preserve the error-response request kind | `Response::Error` constructs `UdpResponseKind::Error { opt_req_kind }`. |
| B4 | TODO | Verify and recheck | Run focused/package checks and repeat the B1 artifact and event-bus observations. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| B1 | Reproduction evidence | Included in the spec-only PR. |
| B2 | Processor regression tests and red-test evidence | Commit after focused red validation and test-design review. |
| B3-B4 | One-line fix with green and recheck evidence | Commit after focused and package validation. |

Use signed Conventional Commits. Cite in-progress commits by their unique subject rather than SHA
until merge.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/udp-error-response-request-kind/ISSUE.md`.
- [x] Bug reproduced and recorded in issue-local `manual-verification-evidence.md` (V1, V2).
- [x] Spec reviewed and approved by user/maintainer.
- [x] GitHub issue created and issue number added to this spec.
- [x] Spec moved to `docs/issues/open/` with the assigned issue number.
- [ ] Spec-only PR merged into `develop` before implementation.
- [ ] Implementation completed.
- [ ] Automatic verification completed (`linter all`, relevant tests, and pre-push checks when applicable).
- [ ] Manual verification and recheck recorded in issue-local `manual-verification-evidence.md`.
- [ ] Acceptance criteria re-reviewed after implementation and updated with evidence.
- [ ] Evidence-based implementation completion review recorded.
- [ ] Reviewer validated acceptance criteria and updated checkboxes.
- [ ] Committer verified spec progress is up to date before commit.
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`.

### Progress Log

- 2026-09-26 10:37 UTC - GitHub Copilot - Created this draft from the external analysis, confirmed
  the source data-loss path, and ran the strict-mode real-loopback trigger plus the existing direct
  processor event-bus fixture. The public artifact does not expose the defective event field; the
  planned unit regression test is the maintained observation boundary. - This specification
- 2026-09-26 10:52 UTC - GitHub Copilot - Reproduced the defect with a temporary processor
  event-bus test (V2): a parsed scrape failure published `Error { opt_req_kind: None }`. Reverted
  the temporary test, recorded steps and output, added the plain-language explanation, and aligned
  the regression test and AC1 with the reproduced scrape case. - `manual-verification-evidence.md`
- 2026-09-26 11:01 UTC - josecelano and GitHub Copilot - Approved the specification, created
  GitHub issue #2345, and moved the specification to `docs/issues/open/`. Next step: spec-only PR,
  then implementation on branch `2345-keep-request-kind-in-udp-error-response-event`. -
  https://github.com/torrust/torrust-tracker/issues/2345

## Acceptance Criteria

- [ ] AC1: A parsed request whose handler fails (a scrape with an invalid connection ID) publishes
      `UdpResponseKind::Error { opt_req_kind: Some(UdpRequestKind::Scrape) }`.
- [ ] AC2: An unparsable payload still publishes
      `UdpResponseKind::Error { opt_req_kind: None }`.
- [ ] AC3: The parsed-failure regression test was observed red before the fix and green after.
- [ ] AC4: The post-fix rechecks M3 and M4 are recorded in `manual-verification-evidence.md`
      next to the pre-fix reproduction (M1, M2).
- [ ] `cargo test -p torrust-tracker-udp-server` and `linter all` exit with code `0`.

## Verification Plan

### Automatic Checks

- `cargo test -p torrust-tracker-udp-server <new parsed-error regression test> --lib`
- `cargo test -p torrust-tracker-udp-server <new unparsable-payload regression test> --lib`
- `cargo test -p torrust-tracker-udp-server`
- `linter all`

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| M1 | Pre-fix: real tracker reaches the parsed-request error path | Run the strict-mode loopback contract listed in V1. | Error responses for the parsed announces (Trigger only). | DONE | `manual-verification-evidence.md` V1 |
| M2 | Pre-fix: published event drops the request kind | Run the temporary event-bus test recorded in V2: a parsed scrape with `ConnectionId::new(0)`. | `UdpResponseSent` reports `Error { opt_req_kind: None }` (Reproduced). | DONE | `manual-verification-evidence.md` V2 |
| M3 | Post-fix recheck of M1 | Rerun the V1 contract unchanged. | Error responses for the parsed announces, as before. | TODO | `manual-verification-evidence.md` Post-Fix Recheck |
| M4 | Post-fix recheck of M2 | Observe the same parsed scrape on the event bus through the maintained regression test. | `UdpResponseSent` reports `Error { opt_req_kind: Some(Scrape) }`. | TODO | `manual-verification-evidence.md` Post-Fix Recheck |

The tracker/client artifact in M1 and M3 cannot expose the event field, because clients, metrics,
and logs do not show published events. M2 and M4 therefore observe it at the processor's
event-publishing boundary.

### Disposable Verification Scripts

No disposable script is planned. The pre-fix reproduction used a temporary Rust test (recorded
verbatim in V2 and reverted); B2 replaces it with the maintained regression test.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | TODO | Focused processor regression test and `manual-verification-evidence.md`. |
| AC2 | TODO | Focused processor regression test. |
| AC3 | TODO | Recorded red and green command output. |
| AC4 | TODO | `manual-verification-evidence.md`. |

## Risks and Trade-offs

- A test that assumes `UdpResponseSent` is the first event would fail, because
  `UdpRequestAccepted` and `UdpError` come first. Consume events until the response event arrives.
- The current metrics consumer ignores the field. Do not mistake passing metrics tests for proof
  that the event contract is preserved.
- #2283 has processor-test work paused on this bug fix. Keep the test fixture extension narrow to
  minimize rebase conflict.

## Implementation Completion Review

After implementation, compare the result with this specification and record material discoveries,
deviations, and reusable lessons.

- Retrospective: Not yet assessed.
- Create `implementation-retrospective.md` only if the fix reveals a material design or workflow
  lesson; otherwise record why none is needed in the progress log.

## References

- Commit `27e2db4b`: `refactor: [#1382] include req kin in UDP error response if it's known`.
- Related issue: #2283 (processor test coverage work paused to avoid a conflict).
- `.github/skills/dev/debugging/fix-bug/SKILL.md`.
