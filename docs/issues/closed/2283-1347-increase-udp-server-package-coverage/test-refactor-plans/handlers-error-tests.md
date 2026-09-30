---
doc-type: file-test-plan
issue: 2283
package: torrust-tracker-udp-server
target-file: packages/udp-server/src/handlers/error.rs
status: done
semantic-links:
  related-artifacts:
    - packages/udp-server/src/handlers/error.rs
    - packages/udp-server/src/handlers/mod.rs
    - packages/udp-server/src/error.rs
    - packages/udp-server/src/event.rs
    - packages/udp-server/src/banning/event/handler.rs
    - docs/issues/closed/2283-1347-increase-udp-server-package-coverage/coverage-evidence.md
---

# UDP Server Error Handler File Test Plan

Follow the shared [guidance](README.md). This plan covers only
`packages/udp-server/src/handlers/error.rs`.

## Current State

- **Fresh unit-only coverage:** 155 / 178 lines (87.08%), 22 / 23 functions, and 172 / 189
  regions from `cargo llvm-cov -p torrust-tracker-udp-server --all-features --lib --json`
  (stable Rust toolchain), measured after the scrape follow-up commits.
- **Uncovered executable lines:** the `log_connection_cookie_error` branch and body (lines 72-77,
  93-107), the `true` outcome of `is_connection_cookie_error` (line 127), and the `panic!` arm in
  the test that extracts the public URL (line 263).
- **Module-owned decisions:**
  1. Answer every failed request with an error response carrying the client's transaction ID,
     or zero when the request could not be parsed far enough to have one.
  2. Publish one objective `UdpError` fact naming the client, listener, configuration instance,
     public URL, request kind, and classified error, when a publisher exists.
  3. Log connection-cookie failures at `warn` (expected client noise) and every other failure at
     `error`.
- **Existing tests:** four tests over the named `handle_error_with_default_context` Act: two
  response tests and two event tests.
- **T1 hypothesis:** no change. Fresh evidence shows decision 3 has no test at all, which
  falsifies it.

## Current Tests Review

The structure is sound and was the precedent for the scrape refactor: AAA markers, `it_should_*`
names, a documented named Act hiding only the wide context no test varies, and event capture via a
`Broadcaster` subscription instead of `mockall`. Remaining smells:

| Test | Smell | Proposed response |
| --- | --- | --- |
| `..._return_an_error_response_with_the_supplied_transaction_id` | Partial assertion: only the transaction ID is checked, not the error message the client sees. | Assert the complete `ErrorResponse`, with the message independently specified from the error's display text. |
| `..._return_a_zero_transaction_id_without_an_event_sender` | Misleading name: the zero ID follows from the missing transaction ID, not the missing sender. | Rename to `..._when_the_request_has_no_transaction_id`; assert the complete response. |
| `..._publish_an_error_event_with_the_supplied_request_kind` / `..._public_url` | Two partial event assertions; the connection context (client, listener, instance) that banning and metrics consume is never asserted. | Merge into one test that asserts the complete `Event::UdpError`, with the context built visibly from the same client and listener values. |
| `handle_error_with_default_context` | Hides the client and listener, which prevents the context assertion above. | Expose the default client and listener as named test values (`sample_client`, `sample_listener`) used by both the helper and the expected event. |

## Coverage Analysis

| Behavior group | Classification | Decision |
| --- | --- | --- |
| Error response | 3A unit | R1 complete the two response assertions. |
| Error-event publication | 3A unit | R1 merge the two partial tests into one complete-event contract. |
| Cookie-error log severity | 3A unit | R2 test the existing pure `is_connection_cookie_error` directly: true for announce and scrape cookie failures, false for another failure. It selects the log level, and it is the only uncovered decision. |
| Logged fields and levels | Not selected | The only capture helper, `torrust_tracker_test_helpers::logging`, is a process-global buffer used by integration contracts. Asserting on `tracing` output here would couple a unit test to subscriber setup for an operator-facing detail. The classifier test protects the decision; the remaining uncovered log statements are deliberately untested. |
| Error classification (`ErrorKind`) | Collaborator-owned | `event.rs` owns the `Error` to `ErrorKind` conversion and its tests. |
| UDP socket transport | Integration | Not selected: this adapter has no socket Act. |

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | DONE | Completed the response assertions, renamed the zero-transaction-ID test, merged the two event tests into one complete-event test, and exposed the default client, listener, and configuration instance as named test values. | Focused `handlers::error` tests, nightly formatting, Clippy, and prose-first review passed. |
| R2 | DONE | Added two unit tests for `is_connection_cookie_error`: announce and scrape cookie failures are cookie errors; an internal failure is not. | Classifier `true` and `false` outcomes covered. |
| R3 | SKIPPED | No integration increment: this handler controls no UDP socket boundary. | Explicit no-integration rationale. |
| R4 | DONE | Re-ran unit-only coverage and recorded before/after file results. | 155 / 178 (87.08%) to 218 / 218 (100.00%) lines; 29 / 29 functions; 215 / 215 regions. |

## Results

- **Unit-only coverage:** 155 / 178 (87.08%) to 218 / 218 (100.00%) lines; 22 / 23 to 29 / 29
  functions; 172 / 189 to 215 / 215 regions.
- **Tests refactored:** three (two response tests completed, two partial event tests merged into
  one complete-event test).
- **Tests added:** two classifier tests.
- **Integration selected:** No. The handler has no socket Act.
- **Log statements:** the response tests now use a cookie failure, so the `warn` branch executes;
  its level is not asserted. The classifier tests protect the severity decision.
- **Response test fixture:** the response tests use a scrape cookie failure because
  `Error::Internal` embeds the source `Location`, which would make the expected message
  line-number dependent.

## Completed-File Review

Approved by Jose Celano on 2026-09-24.

## Progress Log

- 2026-09-24 - GitHub Copilot - Created the plan from `handlers/error.rs`, its four tests, and
  fresh unit-only coverage. The uncovered decision is the cookie-error log-severity classification.
  No Rust tests or production code changed. R1 and R2 await maintainer approval.
- 2026-09-24 - Jose Celano - Approved R1 and R2.
- 2026-09-24 - GitHub Copilot - Completed R1-R4. `cargo test -p torrust-tracker-udp-server
  handlers::error::tests` passed five tests (stable Rust toolchain); nightly Rust formatting,
  Clippy, and `git diff --check` passed. Fresh unit-only coverage is 218 / 218 lines,
  29 / 29 functions, and 215 / 215 regions (100.00%). Completed-file review is requested.
- 2026-09-24 - Jose Celano - Approved the completed-file result. The file is ready for package
  validation, pre-commit, and its signed per-file commit.
