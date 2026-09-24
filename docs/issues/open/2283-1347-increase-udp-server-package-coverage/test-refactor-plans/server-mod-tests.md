---
doc-type: file-test-plan
issue: 2283
package: torrust-tracker-udp-server
target-file: packages/udp-server/src/server/mod.rs
status: done
semantic-links:
  related-artifacts:
    - packages/udp-server/src/server/mod.rs
    - packages/udp-server/src/server/states.rs
    - docs/issues/closed/2149-1347-add-focused-udp-server-package-tests/test-refactor-plans/server-states-tests.md
    - docs/issues/open/2283-1347-increase-udp-server-package-coverage/coverage-evidence.md
---

# UDP Server Module File Test Plan

Follow the shared [guidance](README.md). This plan covers only
`packages/udp-server/src/server/mod.rs`.

## Current State

- **Fresh unit-only coverage:** 176 / 177 lines (99.44%), 13 / 13 functions, and 289 / 291 regions
  from `cargo llvm-cov -p torrust-tracker-udp-server --all-features --lib --json` (stable Rust
  toolchain).
- **Uncovered executable lines:** both are test code: the `panic!` arm of the registration-failure
  test's `let ... else` (line 245) and one branch inside `test_tokio` (line 280).
- **Module-owned decisions:** none. The file declares `UdpError` (derived `thiserror` messages)
  and the `Server<S>` state holder. `start` and `stop` live in `states.rs`.
- **Existing tests:**
  - `tests`: three public start contracts. Per #2149 D3 they stay here as the public
    `Server` transition tests; `stop` semantics are owned by #1488.
  - `test_tokio::test_barrier_with_aborted_tasks`: tests Tokio's `Barrier`. The package does not
    use `Barrier` anywhere; the module comment says "submit test to tokio documentation".
- **T1 hypothesis:** no change. Confirmed for coverage; the review below finds one misplaced test.

## Current Tests Review

| Test | Smells | Proposed response |
| --- | --- | --- |
| `test_tokio::test_barrier_with_aborted_tasks` | Collaborator-owned: it verifies a Tokio primitive this package never uses, so it cannot fail for a package reason. Timing-dependent (`sleep(50ms)` then `is_finished`), so it can flake under load. | R1 delete it. |
| `it_should_be_able_to_start_and_stop` / `..._with_wait` | No AAA markers; ~25 duplicated Arrange lines each; weak assertion (only `bind_to` survives the round trip); fixed `sleep(1s)` calls add ~3 s per run; the two differ only by a sleep between start and stop. | Not changed here. Their timing and `stop` semantics are #1488 lifecycle scope (SI-14/SI-17 replace this shutdown path); restructuring them now would conflict with that work. The smells are recorded here for that work. |
| `it_should_preserve_registration_error_and_release_listener_when_registration_fails` | Clean: AAA markers, visible causal duplicate registration, complete error and listener-release assertions. | No change. |

Production note: `UdpError::FailedToStartOrStopServer` displayed "sever" instead of "server".
The maintainer approved fixing it here in its own `fix(udp-server)` commit: no test or known
consumer parses the message; the only test matches the variant.

## Coverage Analysis

| Behavior group | Classification | Decision |
| --- | --- | --- |
| `UdpError` messages and `Server` representation | Not selected | Derived `thiserror`/`Display` output; testing it restates macro-generated code (#2149 D2). |
| Public start transition and registration cleanup | Collaboration | Existing tests retained unchanged. |
| `stop` lifecycle and timing | Lifecycle (#1488) | Deferred with owner. |
| Tokio `Barrier` semantics | Collaborator-owned | R1 remove from this package. |

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | DONE | Deleted the `test_tokio` module. | `cargo test -p torrust-tracker-udp-server server::tests` passed three tests; nightly formatting and Clippy passed. |
| R2 | SKIPPED | No new unit test: the file owns no decision. | Explicit rationale above. |
| R3 | SKIPPED | No integration increment: start/stop socket behavior is already covered and lifecycle is #1488. | Explicit rationale above. |
| R4 | DONE | Re-ran unit-only coverage and recorded before/after file results. | 176 / 177 (99.44%) to 137 / 138 (99.28%) lines; 8 / 8 functions; 227 / 228 regions. The smaller denominator is the removed Tokio test; the one uncovered line is the test-only `panic!` arm. |

## Completed-File Review

Approved by Jose Celano on 2026-09-24.

## Progress Log

- 2026-09-24 - GitHub Copilot - Created the plan from `server/mod.rs`, its four tests, the #2149
  `states.rs` decisions, and fresh unit-only coverage. No Rust tests or production code changed.
  R1 awaits maintainer approval.
- 2026-09-24 - Jose Celano - Approved R1, leaving the start/stop tests for #1488, and fixing the
  "sever" typo in this branch.
- 2026-09-24 - GitHub Copilot - Completed R1 and R4 (stable Rust toolchain for tests and
  coverage; nightly Rust for formatting). Fixed the typo in a separate `fix(udp-server)` commit.
  Completed-file review is requested.
- 2026-09-24 - Jose Celano - Approved the completed-file result.
