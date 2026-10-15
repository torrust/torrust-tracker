---
doc-type: file-test-plan
issue: 1348
package: torrust-tracker-udp-core
target-file: packages/udp-core/src/services/mod.rs
status: done
semantic-links:
  related-artifacts:
    - packages/udp-core/src/services/mod.rs
    - packages/udp-core/src/services/connect.rs
    - packages/udp-core/src/services/announce.rs
    - packages/udp-core/src/services/scrape.rs
    - docs/issues/open/1348-1347-add-tests-udp-core/coverage-evidence.md
---

# UDP Core Services Module File Test Plan

Follow the shared [guidance](README.md). This plan covers only
`packages/udp-core/src/services/mod.rs` (ledger order 17).

## Current State

- **Unit-only coverage:** 10 / 12 lines (83.33%), 11 / 13 regions, 4 / 5 functions. T1
  measured 22 / 24; the connect plan removed four unused helpers here (approved exception).
- **Uncovered:** line 29, inside the `mock!` expansion (mockall-generated code that no test
  calls). Not a package decision.
- **Contents:** module declarations and a test-only `tests` module shared by the service tests:

  | Helper | Callers |
  | --- | --- |
  | `MockUdpCoreStatsEventSender` | `announce.rs`, `connect.rs`, `scrape.rs` tests |
  | `sample_ipv4_remote_addr()` | `connect.rs` tests only |
  | `sample_ipv4_socket_address()` | only `sample_ipv4_remote_addr()`, which returns it unchanged |
  | `sample_issue_time()` | `connect.rs` tests only |

- **Module-owned decisions:** none at runtime; this file is test support and wiring.
- **T1 hypothesis:** test support only.

## Current Tests Review

No tests in this file. Judged as shared test support against the guardrail "Shared helpers require
an approved cohesive responsibility; do not create generic fixtures for a one-file seam":

- **Keep shared:** the event-sender mock serves all three services.
- **Smell: one-file helpers in the shared module.** `sample_ipv4_remote_addr()` and
  `sample_issue_time()` serve only `connect.rs`.
- **Smell: an alias with no meaning of its own.** `sample_ipv4_socket_address()` exists only to be
  returned by `sample_ipv4_remote_addr()`.

## Coverage Analysis

| Item | Classification | Decision |
| --- | --- | --- |
| Event-sender mock | Shared test support | Keep. |
| `sample_ipv4_remote_addr()`, `sample_issue_time()`, `sample_ipv4_socket_address()` | One-file test support | R1: move the two used helpers into `connect.rs`'s test module, folding the alias into `sample_ipv4_remote_addr()`. No test behavior changes. |
| Line 29 (`mock!` expansion) | Not tested | Generated mockall code; testing it would test mockall. |
| Property-based testing | Not selected | No runtime code. |

R1 touches `connect.rs`'s test module (a `DONE` file) only to receive the helpers; its tests stay
as they are. Line 29 stays uncovered; the moved helper lines are counted in `connect.rs` instead.

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | DONE | Move the connect-only helpers into `connect.rs`; fold the alias. | 83 package unit tests passed (stable Rust 1.99.0); pedantic Clippy, nightly formatting (nightly 1.101.0), and `git diff --check` clean. No test changed behavior. |
| R2 | SKIPPED | No tests: no runtime decision. | Recorded above. |
| R3 | SKIPPED | No integration increment. | Explicit rationale. |
| R4 | DONE | Re-run unit-only coverage; record before/after. | 10 / 12 to 1 / 3 lines; see Results. |

## Results

- **Unit-only coverage (test-inclusive file totals, `cargo-llvm-cov 0.6.16`):** 10 / 12 (83.33%) to
  1 / 3 (33.33%) lines; 1 / 3 regions; 1 / 2 functions. The file now holds only the shared mock;
  its uncovered lines are mockall-generated. The moved helpers are counted in `connect.rs`, now
  114 / 120 lines (95.00%), 120 / 127 regions, 15 / 17 functions.
- **Tests:** none added or changed; two helpers moved, one alias removed.
- **Integration selected:** No.
- **Completed-file review:** Approved by Jose Celano on 2026-10-14.

## Approval

Approved by Jose Celano on 2026-10-14: R1.

## Progress Log

- 2026-10-14 11:43 UTC - GitHub Copilot - Created the plan from `services/mod.rs` and the callers of
  each shared helper. No Rust tests or production code changed. R1 awaits maintainer approval.
- 2026-10-14 11:52 UTC - Jose Celano - Approved R1.
- 2026-10-14 12:00 UTC - GitHub Copilot - Implemented R1 and measured R4 (stable Rust 1.99.0,
  nightly 1.101.0, `cargo-llvm-cov 0.6.16`). The plan awaits completed-file review.
- 2026-10-14 12:22 UTC - Jose Celano - Approved completed-file review. The file is ready for
  pre-commit and its signed per-file commit.
