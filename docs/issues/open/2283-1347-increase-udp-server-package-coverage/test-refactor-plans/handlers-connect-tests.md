---
doc-type: file-test-plan
issue: 2283
package: torrust-tracker-udp-server
target-file: packages/udp-server/src/handlers/connect.rs
status: done
semantic-links:
  related-artifacts:
    - packages/udp-server/src/handlers/connect.rs
    - packages/udp-server/src/handlers/mod.rs
    - packages/udp-core/src/services/connect.rs
    - docs/issues/open/2283-1347-increase-udp-server-package-coverage/coverage-evidence.md
---

# UDP Server Connect Handler File Test Plan

Follow the shared [guidance](README.md). This plan covers only
`packages/udp-server/src/handlers/connect.rs`.

## Current State

- **Baseline unit-only coverage:** 251 / 251 lines, 285 / 285 regions, and 18 / 18 functions
  (100.00%).
- **R4 unit-only coverage:** 212 / 212 lines, 234 / 234 regions, and 16 / 16 functions (100.00%)
  from `cargo llvm-cov -p torrust-tracker-udp-server --all-features --lib --json`. The smaller
  denominator reflects removal of the duplicate test body.
- **Module-owned decisions:** publish a `UdpRequestAccepted` fact when enabled, delegate cookie
  creation to `udp-core`, and return the request transaction ID with the generated connection ID.
- **Existing tests:** three response tests and two event-publication tests cover IPv4 and IPv6.
- **T1 hypothesis:** no change. Coverage is complete, but R1 identifies test-quality duplication.

## Current Tests Review

`a_connect_response_should_contain_the_same_transaction_id_as_the_connect_request` and
`a_connect_response_should_contain_a_new_connection_id` have the same IPv4 state, Act, and full
expected response. The first already proves both response fields, so the second is duplicate and
should be removed. The retained response tests and both event tests should receive visible AAA
sections; event-test names should correct `upd4`/`upd6` and `ip4`/`ip6` to `udp4`/`udp6` and
`IPv4`/`IPv6`.

## Coverage Analysis

| Behavior group | Classification | Decision |
| --- | --- | --- |
| Connect response adaptation | 3A unit | R1 retain one IPv4 and one IPv6 response scenario; remove the duplicate IPv4 scenario. |
| Accepted-request event publication | 3A unit | R1 preserve the existing IPv4/IPv6 event contracts while improving AAA readability and names. |
| Connection-cookie construction | Collaborator-owned | `udp-core` owns cookie generation semantics. |
| UDP socket transport | Integration | Not selected: this adapter has no socket Act. |

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | DONE | Removed the duplicate IPv4 response test; added visible AAA sections and corrected event-test terminology. | Focused `handlers::connect` tests, nightly formatting, and prose-first review passed. |
| R2 | DONE | No additional unit test: every executable line, region, and function already has direct unit coverage. | Fresh unit-only 100.00% coverage. |
| R3 | SKIPPED | No integration increment: this handler controls no UDP socket boundary. | Explicit no-integration rationale. |
| R4 | DONE | Re-ran unit-only coverage after R1 and recorded full coverage. | 212 / 212 lines, 234 / 234 regions, and 16 / 16 functions (100.00%). |

## Approval Requested

Approve the completed-file result before its test and documentation commit. R1 removes redundant
coverage while making each retained scenario and event contract easier to read; it adds no behavior
and no integration test.

## Progress Log

- 2026-09-22 17:16 UTC - GitHub Copilot - Created the plan from `handlers/connect.rs`, its
  current tests, direct dispatch caller, and fresh unit-only coverage. The 100.00% report selects
  no new unit test; R1 proposes removal of one duplicate IPv4 response test plus readability-only
  cleanup. No Rust tests or production code changed. R1 awaits maintainer approval.
- 2026-09-22 17:21 UTC - User/maintainer - Approved R1.
- 2026-09-22 17:21 UTC - GitHub Copilot - Completed R1-R4. Removed the duplicate IPv4 response
  test, made AAA boundaries visible, and corrected event-test terminology to `udp4`/`udp6` and
  `IPv4`/`IPv6`. `cargo test -p torrust-tracker-udp-server handlers::connect::tests` passed four
  tests; nightly Rust formatting and `git diff --check` passed. Fresh unit-only coverage remains
  complete at 212 / 212 lines, 234 / 234 regions, and 16 / 16 functions (100.00%). The reduced
  denominator is solely from removing the duplicate test body. Completed-file review is requested.
- 2026-09-22 17:23 UTC - User/maintainer - Approved the completed-file result. The plan is done;
  its test and documentation increment awaits the mandatory validation and signed commit.
