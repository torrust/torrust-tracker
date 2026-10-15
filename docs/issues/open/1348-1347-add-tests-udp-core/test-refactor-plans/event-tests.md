---
doc-type: file-test-plan
issue: 1348
package: torrust-tracker-udp-core
target-file: packages/udp-core/src/event.rs
status: done
semantic-links:
  related-artifacts:
    - packages/udp-core/src/event.rs
    - packages/udp-core/src/statistics/event/handler.rs
    - packages/udp-server/src/banning/event/handler.rs
    - docs/issues/open/1348-1347-add-tests-udp-core/coverage-evidence.md
---

# UDP Core Event File Test Plan

Follow the shared [guidance](README.md). This plan covers only `packages/udp-core/src/event.rs`.

## Current State

- **Unit-only coverage:** 131 / 137 lines (95.62%) at T1 and unchanged since.
- **Uncovered lines:** 79-81, `ConnectionContext::client_socket_addr()`; 84-86,
  `ConnectionContext::server_socket_addr()`.
- **Existing tests:** 7: client IP family (IPv4, IPv6), client IP type (plain IPv4, plain IPv6,
  IPv4-mapped IPv6), public URL retained, and the `public_url` label present only when configured.
- **Module-owned decisions:** the event variants; the `ConnectionContext` accessors; the client IP
  family and type classification (including IPv4-mapped IPv6); the projection of a context into the
  metric label set (seven labels, plus `public_url` when configured).
- **Consumers:** `statistics/event/handler.rs` turns every event into labels; `udp-server` uses
  `ConnectionContext` in its events, and its banning handler keys bans on
  `context.client_socket_addr().ip()`.
- **T1 hypothesis:** probably no change. The analysis below falsifies it.

## Current Tests Review

Conclusion: `refactor`.

- No visible Arrange/Act/Assert sections.
- All seven tests build the context with the same three-line `ConnectionContext::new(...)` and
  `ServiceBinding::new(...)` call, and several repeat the same local instance-ID binding. Extract
  one fixture, `context_for_client(client_socket_addr)`, with a fixed UDP server binding, so each
  test shows only the client address that matters.
- Names are clear; keep them.

## Coverage Analysis

| Behavior group | Classification | Decision |
| --- | --- | --- |
| Label projection (seven labels and their values) | 3A unit | E1: a context with a known client and server binding projects to the exact expected `LabelSet`. Today only `statistics/event/handler.rs` pins the full set, as a collaboration test; this module owns the projection, so it should own the test. The handler's duplicate is reconsidered in its own plan (ledger order 10). |
| `client_socket_addr()` | 3A unit | E2: returns the client address, not the server's. It is a getter, but `udp-server` bans by its IP, so a swap would ban the wrong address. |
| `server_socket_addr()` | Not tested; finding | No caller in the repository. Recorded in `lessons.md` as a removal candidate instead of adding a test for dead API. |
| Client IP family and type | 3A unit | Existing tests, kept after R1. |
| Public URL retention and label | 3A unit | Existing tests, kept after R1. |
| Event bus, sender, receiver type aliases | Collaborator-owned | `torrust-tracker-events` owns them. |
| Property-based testing | Not selected | The classifications are enumerable cases. |

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | DONE | Add the `context_for_client` fixture and AAA sections. | Focused `event` tests passed; nightly formatting and `git diff --check` clean. |
| R2 | DONE | Add E1 and E2. | 70 package unit tests passed; pedantic Clippy clean. With `client_socket_addr()` temporarily returning the server address, only E2 failed; reverted by hand. |
| R3 | SKIPPED | No integration increment: plain data and pure conversions. | Explicit rationale. |
| R4 | DONE | Re-run unit-only coverage; record before/after. | 131 / 137 to 144 / 147 lines; see Results. |

## Results

- **Unit-only coverage (test-inclusive file totals):** 131 / 137 (95.62%) to 144 / 147 (97.96%)
  lines; 229 / 233 regions; 21 / 22 functions. The three uncovered lines are the dead
  `server_socket_addr()`.
- **Tests refactored:** 7 (shared `context_for_client` fixture, AAA sections).
- **Tests added:** 2: E1 (full label projection) and E2 (client address getter).
- **Findings:** `server_socket_addr()` recorded in [lessons.md](../lessons.md) as a removal
  candidate.
- **Integration selected:** No.
- **Completed-file review:** Approved by Jose Celano on 2026-10-13.

## Prose-First AAA Review

Recorded on 2026-10-15, late: the PR #2538 Task Reviewer found no record of it (AC7). For each
test, temporary Arrange, Act, and Assert prose was compared with the code and then removed.

| Test | Arrange → Act → Assert | Outcome |
| --- | --- | --- |
| E2 `..._return_the_client_socket_address_not_the_server_one` | IPv4 client → `client_socket_addr` → that client | No change |
| `it_should_report_the_inet_family_when_the_client_is_ipv4` | IPv4 client → `client_address_ip_family` → `Inet` | Changed: `it_should_` name |
| `it_should_report_the_inet6_family_when_the_client_is_ipv6` | IPv6 client → `client_address_ip_family` → `Inet6` | Changed: `it_should_` name |
| `it_should_report_a_plain_ip_type_when_the_client_is_ipv4` | IPv4 client → `client_address_ip_type` → `Plain` | Changed: `it_should_` name |
| `it_should_report_a_plain_ip_type_when_the_client_is_native_ipv6` | Native IPv6 client → `client_address_ip_type` → `Plain` | Changed: `it_should_` name |
| `it_should_report_a_v4_mapped_v6_ip_type_when_the_client_is_ipv4_mapped_ipv6` | Mapped client → `client_address_ip_type` → `V4MappedV6` | Changed: `it_should_` name; the address is built with `to_ipv6_mapped()` instead of hex and a comment |
| `..._retain_an_optional_configured_public_url` | Context with a URL → `public_url` → that URL | No change |
| E1 `..._project_the_server_binding_and_the_client_address_into_metric_labels` | IPv4 client → `LabelSet::from` → literal seven labels | No change; the server values come from the one helper that owns them |
| `it_should_include_the_public_url_label_only_when_a_public_url_is_configured` | With and without a URL → `LabelSet::from` ×2 → label only in the first | Changed: `it_should_` name |

## Approval

Approved by Jose Celano on 2026-10-13: R1, E1, E2, and recording `server_socket_addr()` as a
removal candidate.

## Progress Log

- 2026-10-13 16:57 UTC - GitHub Copilot - Created the plan from `event.rs`, its in-package and
  `udp-server` consumers, and the latest unit-only missing-line report. `develop` was 106 commits
  ahead with no `udp-core` change. No Rust tests or production code changed. R1 and R2 await
  maintainer approval.
- 2026-10-13 17:06 UTC - GitHub Copilot - Plan approved. Implemented R1 and R2 and measured R4
  (stable Rust 1.99.0, nightly 1.101.0, `cargo-llvm-cov 0.6.16`). The plan awaits completed-file
  review.
- 2026-10-13 17:12 UTC - Jose Celano - Approved completed-file review. The file is ready for
  pre-commit and its signed per-file commit.
- 2026-10-15 10:30 UTC - GitHub Copilot - Recorded the prose-first AAA review late, after the
  PR #2538 Task Reviewer found no record (AC7). Six tests renamed to the `it_should_` form; the
  mapped address is built with `to_ipv6_mapped()`.
