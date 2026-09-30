---
doc-type: file-test-plan
issue: 2283
package: torrust-tracker-udp-server
target-file: packages/udp-server/src/server/bound_socket.rs
status: done
semantic-links:
  related-artifacts:
    - packages/udp-server/src/server/bound_socket.rs
    - packages/udp-server/src/server/states.rs
    - packages/udp-server/tests/server/contract.rs
    - docs/issues/closed/2149-1347-add-focused-udp-server-package-tests/test-refactor-plans/bound-socket-tests.md
    - docs/issues/closed/2283-1347-increase-udp-server-package-coverage/coverage-evidence.md
---

# UDP Bound Socket File Test Plan

Follow the shared [guidance](README.md). This plan covers only
`packages/udp-server/src/server/bound_socket.rs`.

## Current State

- **Fresh unit-only coverage:** 55 / 65 lines (84.62%), 10 / 11 functions, and 109 / 135 regions
  from `cargo llvm-cov -p torrust-tracker-udp-server --all-features --lib --json` (stable Rust
  toolchain).
- **Uncovered:** the `IPV6_V6ONLY` branch in `create_socket`; the `?` error exits of socket
  creation, option setting, binding, Tokio conversion, and `local_addr`; the post-bind port-zero
  error; and the whole `Debug` implementation.
- **Module-owned decisions:**
  1. Every returned socket has a non-zero port (the type invariant).
  2. An IPv6 socket is restricted to IPv6 only when `ipv6_v6only` is requested; otherwise the OS
     default is deliberately preserved.
  3. Binding failures are returned as errors, not panics.
  4. `address`, `url`, and `service_binding` describe the same bound endpoint.
- **Existing tests:** two tests added by #2149: the port-zero invariant and metadata consistency.
- **T1 hypothesis:** platform boundary. Mostly confirmed; two decisions have a portable seam.

## Current Tests Review

Both tests are clean: `it_should_*` names, visible AAA, one causal input (an IPv4 loopback
port-zero request), and independently constructed expectations. The metadata test's two
assertions describe one observable result (endpoint consistency), so it has one reason to fail.
No change.

## Coverage Analysis

| Behavior group | Classification | Decision |
| --- | --- | --- |
| Port-zero invariant, endpoint metadata | 3A unit | Covered by the existing tests. |
| `IPV6_V6ONLY` when requested | 3A unit | R1 candidate: bind `[::]:0` through the public `BoundSocket::bind` with `ipv6_v6only = true` and read the option back through `socket2::SockRef` on the dereferenced Tokio socket. It is the same address the existing `using_ipv6_v6only` integration contract binds in CI and the container build, so it adds no new IPv6-availability assumption. |
| `IPV6_V6ONLY` left unset when not requested | Not selected | The resulting value is the OS default (dual-stack on Linux, IPv6-only elsewhere). Asserting it would test the platform, not the package. #2149 P3 decision retained. |
| Bind failure is returned, not panicked | 3A unit | R2 candidate: hold a bound socket, then bind the same address again and assert an error with kind `AddrInUse`. Deterministic: the held socket guarantees contention and `socket2` does not set `SO_REUSEADDR`. #2149 deferred bind errors in `states.rs` because they need socket contention; at this direct seam contention is a one-line visible Arrange. |
| Other `?` exits, post-bind port-zero error | Not reachable | Socket creation, option setting, Tokio conversion, and `local_addr` fail only under OS fault injection or a production-only seam; the port-zero error needs the OS to return port 0 after a successful bind (inventory rationale retained). |
| `tracing::debug!` field after bind | Not reachable in tests | The field is evaluated only when a subscriber enables `debug` for this target; unit tests install none. It is a log detail, not a decision. |
| `Debug` output | Not selected (choice) | The `Ok` branch is testable by formatting a bound socket, but the text has no stable operator contract, so a test would fail on harmless wording changes. The `Err` branch needs a broken socket (#2149 R5 retained). |
| `Deref` | Not selected | Thin standard trait implementation. |

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | DONE | Added `it_should_restrict_an_ipv6_socket_to_ipv6_when_ipv6_v6only_is_requested`. | Passes; fails with "the socket should accept IPv6 traffic only" when `set_only_v6(true)` is removed. |
| R2 | DONE | Added `it_should_return_an_error_when_the_address_is_already_in_use`. | Passes; fails with `left: Other, right: AddrInUse` when the bind error is ignored (the unbound socket then hits the port-zero check). |
| R3 | SKIPPED | No integration increment: `using_ipv6_v6only` already covers the real-listener boundary. | Explicit rationale above. |
| R4 | DONE | Re-ran unit-only coverage and recorded before/after file results. | 55 / 65 (84.62%) to 77 / 86 (89.53%) lines; 10 / 11 to 14 / 15 functions; 109 / 135 to 159 / 179 regions. |

## Results

- **Unit-only coverage:** 55 / 65 (84.62%) to 77 / 86 (89.53%) lines; 10 / 11 to 14 / 15
  functions; 109 / 135 to 159 / 179 regions.
- **Tests refactored:** none. **Tests added:** two.
- **Residual:** lines 36, 38, 84, 87, and 90 (`?` exits) and 42 (post-bind port-zero error) are
  not reachable without fault injection; line 39 is a `debug`-level log field; lines 132-139
  (`Debug`) are left untested by choice. See Coverage Analysis.
- **Mutation evidence:** each new test was run against a hand-applied mutation of the production
  line it guards, failed with the message above, and the mutation was reverted by hand.

## Completed-File Review

Approved by Jose Celano on 2026-09-24.

## Progress Log

- 2026-09-24 - GitHub Copilot - Created the plan from `bound_socket.rs`, the #2149 plan, the
  IPv6-only integration contract, the container test exclusion list, and fresh unit-only coverage.
  No Rust tests or production code changed. R1 and R2 await maintainer approval.
- 2026-09-24 - Jose Celano - Approved R1 and R2.
- 2026-09-24 - GitHub Copilot - Completed R1, R2, and R4. `cargo test -p torrust-tracker-udp-server
  bound_socket::tests` passed four tests (stable Rust toolchain); nightly Rust formatting, Clippy,
  and `git diff --check` passed. While proving R1, restoring with `git checkout --` also discarded
  the uncommitted new tests in the same file; they were re-applied unchanged and later mutations
  were reverted by hand. Completed-file review is requested.
- 2026-09-24 - Jose Celano - Asked whether the residual coverage limits were documented; the
  Coverage Analysis now separates unreachable exits, the `debug`-level log field, and the
  deliberate `Debug` choice. Approved the completed-file result.
