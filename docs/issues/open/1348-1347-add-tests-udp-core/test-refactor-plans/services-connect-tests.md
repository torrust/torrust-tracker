---
doc-type: file-test-plan
issue: 1348
package: torrust-tracker-udp-core
target-file: packages/udp-core/src/services/connect.rs
status: done
semantic-links:
  related-artifacts:
    - packages/udp-core/src/services/connect.rs
    - packages/udp-core/src/connection_cookie.rs
    - packages/udp-core/src/container.rs
    - docs/issues/open/1348-1347-add-tests-udp-core/coverage-evidence.md
---

# UDP Core Connect Service File Test Plan

Follow the shared [guidance](README.md). This plan covers only
`packages/udp-core/src/services/connect.rs`.

## Current State

- **Unit-only coverage:** 147 / 157 lines (93.63%) at T1; 195 / 201 after #2458.
- **Uncovered lines:** 28-30 and 56-58, the `configuration_instance_id()` and `public_url()`
  getters, the same trivial getters left uncovered in the announce and scrape services.
- **Existing tests:** 5, all from before this issue, adapted by #2458 to pass the fixed test
  cipher.
- **Module-owned decisions:** issue a connection ID with the injected key, bound to the client
  socket address and the given issue time; publish `UdpConnect` with the connection context
  (instance ID, client address, server binding, public URL) when a sender exists; panic when the
  issue time is not a normal `f64` (documented under `# Panics`).
- **T1 hypothesis:** probably no change. The test review below falsifies it.

## Current Tests Review

Conclusion: `refactor`.

- `a_connect_response_should_contain_the_same_transaction_id_as_the_connect_request` is misnamed:
  this service never sees a transaction ID (echoing it is the `udp-server` handler's job). Its body
  is identical to `a_connect_response_should_contain_a_new_connection_id`. Delete it.
- The two connection-ID tests (IPv4, IPv6) recompute the expected value with `make`, the same
  function the service calls, so they only prove the service calls `make` with these arguments.
  The contract is that the issued ID is accepted by the validators. Replace both with one test
  that checks the issued ID with `check` for the client's fingerprint at exactly the issue time.
  IP family is not a decision here: the fingerprint hashes any `SocketAddr`.
- The two event tests are misnamed (`upd4`/`upd6`) and identical apart from the client address.
  The IP-family labels are computed by `ConnectionContext` (`event.rs`, ledger order 8), not here.
  Merge them into one test that also sets a public URL, as the announce and scrape event tests do.
- No visible Arrange/Act/Assert sections. The non-event tests build a disabled `EventBus` only to
  get no sender; pass `None` directly.

## Coverage Analysis

| Behavior group | Classification | Decision |
| --- | --- | --- |
| Issued connection ID validates for the client at the issue time | 3A unit | R1 replacement test (see above). |
| `UdpConnect` event with the full connection context | 3A unit | R1 merged event test, with a public URL. |
| No event without a sender | 3A unit | Covered by the R1 tests that pass `None`. |
| Panic on a non-normal issue time | 3A unit | C1: `#[should_panic(expected = "it should be a normal value")]` with issue time `0.0`, pinning the documented `# Panics` contract. |
| One shared key across services | Collaboration | Owned by `container.rs` (#2458's container tests); not repeated here. |
| `configuration_instance_id()` and `public_url()` getters | Not selected | Trivial getters, consistent with the announce and scrape plans. |
| Property-based testing | Not selected | The cookie round trip is already a property in `connection_cookie.rs`. |

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | DONE | Delete the duplicate test, replace the two connection-ID tests with one `check`-based test, merge the two event tests (with public URL), add AAA sections, pass `None` as the sender. | 66 package unit tests passed. A temporary mutation that fingerprinted the server address instead of the client failed the new ID test (and #2458's two container tests); reverted by hand. |
| R2 | DONE | Add C1. | Passed as a `should_panic` test; pedantic Clippy, nightly formatting, and `git diff --check` clean. |
| R3 | SKIPPED | No integration increment: the service has no network boundary. | Explicit rationale. |
| R4 | DONE | Re-run unit-only coverage; record before/after. | 195 / 201 to 108 / 114 lines; see Results. |

## Results

- **Unit-only coverage (test-inclusive file totals):** 195 / 201 (97.01%) after #2458 to 108 / 114
  (94.74%) lines; 113 / 120 regions; 13 / 15 functions. The denominator shrank because five long
  tests became three short ones; the only uncovered lines are still the two getters.
- **Tests:** 5 removed (1 duplicate, 2 that recomputed the expected ID with `make`, 2 near-identical
  event tests); 3 added (ID validates with `check`, event with full context, documented panic).
- **Necessary change outside this file:** removing those tests left four shared helpers in
  `services/mod.rs` unused (`sample_ipv4_remote_addr_fingerprint`, `sample_ipv6_remote_addr`,
  `sample_ipv6_remote_addr_fingerprint`, `sample_ipv6_socket_address`), which fails the build under
  `-D warnings`. They are removed in the same commit. `services/mod.rs` keeps its own ledger row
  (order 17); its one uncovered line is inside the `mock!` macro, unchanged since T1.
- **Integration selected:** No.
- **Completed-file review:** Approved by Jose Celano on 2026-10-10, including the `services/mod.rs`
  helper removal in the same commit.

## Prose-First AAA Review

Recorded on 2026-10-15, late: the PR #2538 Task Reviewer found no record of it (AC7). For each
test, temporary Arrange, Act, and Assert prose was compared with the code and then removed.

| Test | Arrange → Act → Assert | Outcome |
| --- | --- | --- |
| `..._issue_a_connection_id_that_validates_for_the_client_at_the_issue_time` | Shared cipher, fixed time → `handle_connect` → `check` returns the issue time | No change |
| `..._publish_a_connect_event_with_the_connection_context` | Mock expects one event → `handle_connect` → mock verifies on drop | No change; the comment on the drop check is context the code cannot show |
| `..._panic_when_the_issue_time_is_not_a_normal_value` | Issue time `0.0` inline → `handle_connect` → panic message | No change |

## Approval

Approved by Jose Celano on 2026-10-10: the R1 refactor (5 tests become 2) and C1.

## Progress Log

- 2026-10-10 11:09 UTC - GitHub Copilot - Created the plan from `services/connect.rs` as changed by
  #2458 and the latest unit-only missing-line report. `develop` was 15 commits ahead with no
  `udp-core` change. No Rust tests or production code changed. R1 and R2 await maintainer approval.
- 2026-10-10 11:22 UTC - GitHub Copilot - Plan approved. Implemented R1 and R2 together and
  measured R4 (stable Rust 1.99.0, nightly 1.101.0, `cargo-llvm-cov 0.6.16`). The plan awaits
  completed-file review.
- 2026-10-10 11:29 UTC - Jose Celano - Approved completed-file review. The file is ready for
  pre-commit and its signed per-file commit.
- 2026-10-15 10:30 UTC - GitHub Copilot - Recorded the prose-first AAA review late, after the
  PR #2538 Task Reviewer found no record (AC7). No change.
