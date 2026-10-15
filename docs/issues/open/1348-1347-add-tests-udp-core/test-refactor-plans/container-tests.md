---
doc-type: file-test-plan
issue: 1348
package: torrust-tracker-udp-core
target-file: packages/udp-core/src/container.rs
status: done
semantic-links:
  related-artifacts:
    - packages/udp-core/src/container.rs
    - packages/udp-core/src/services/connect.rs
    - packages/udp-core/src/services/announce.rs
    - packages/udp-core/src/services/scrape.rs
    - packages/udp-core/src/services/banning.rs
    - src/container.rs
    - docs/issues/open/1348-1347-add-tests-udp-core/coverage-evidence.md
---

# UDP Core Container File Test Plan

Follow the shared [guidance](README.md). This plan covers only `packages/udp-core/src/container.rs`
(ledger order 9). It was deferred until #2505 merged; the branch is rebased onto it.

## Current State

- **Unit-only coverage:** 183 / 183 lines (100.00%), 191 / 191 regions, 16 / 16 functions. T1
  measured 0 / 97; #2458 added the two shared-key tests.
- **Existing tests:** two, both through `UdpTrackerCoreContainer::initialize` (public tracker, no
  database): a connection ID issued by the container's connect service is accepted by its announce
  service and by its scrape service. A compile-fail doctest keeps the cookie key private.
- **Module-owned decisions:**
  1. One cookie key per `UdpTrackerCoreServices`, shared by every service built from it (tested).
  2. The configured `public_url` reaches the connect, announce, and scrape services.
  3. The `ConfigurationInstanceId` reaches the container and the three services.
  4. `max_connection_id_errors_per_ip` becomes the ban service threshold.
  5. The configured `network.external_ip` reaches the announce service.
  6. Several containers built from one `UdpTrackerCoreServices` share its key, ban service,
     statistics, and event bus. Production does exactly this: `src/container.rs` builds one
     container per configured UDP tracker from one set of services.
- **Gap:** line coverage is complete, but decisions 2-6 are wiring that no test observes. Passing
  `None` instead of the public URL, a fixed ban limit, or dropping the external IP keeps every test
  green.
- **T1 hypothesis:** test candidate.

## Current Tests Review

| Test | Review |
| --- | --- |
| `it_should_accept_in_announce_a_connection_id_issued_by_the_connect_service_of_the_same_container` | Clean: state-named fixture, visible Act, failure message. |
| `it_should_accept_in_scrape_a_connection_id_issued_by_the_connect_service_of_the_same_container` | Clean. |

No R1 refactor.

## Coverage Analysis

| Decision | Classification | Decision |
| --- | --- | --- |
| 2. Public URL propagation | 3A unit | C1: with a configured public URL, `public_url()` of the connect, announce, and scrape services all return it. |
| 3. Instance ID propagation | 3A unit | C2: with instance ID 7, the container and the three services' `configuration_instance_id()` all return it. |
| 4. Ban threshold | 3A unit | C3: with a limit of 3, the container's ban service does not ban an IP after 3 errors and bans it after the 4th. A fixed or off-by-one limit fails it. |
| 5. External IP propagation | 3A unit | C4: with `network.external_ip` configured, a loopback client's announce through the container's announce service is stored with the external IP. Mirrors the announce service's own test, but through the container. |
| 6. Containers share one set of services | 3A unit | C5: two containers built with `initialize_from_services` from the same services (two instance IDs): a connection ID issued by the first container's connect service is accepted by the second container's announce service. This is the production multi-tracker shape. |
| 1. Shared key within a container | Covered | No change. |
| Property-based testing | Not selected | Fixed wiring. |

Test data: literals (instance ID 7, limit 3, a documentation-range external IP, a public URL such
as `udp://tracker.example.test:6969/announce`). Construction goes through the container's own
constructors, which is the unit under test; tracker-core comes from `Core::default()` as in the
existing fixture. The announce fixture for C4 needs a tracker-core handle to read the stored peer;
`initialize_from_tracker_core` gives the test that handle.

**Finding (recorded in [lessons.md](../lessons.md); no production change in this issue):**
`UdpTrackerCoreServices::initialize_from` takes a `_tracker_core_container` parameter it never uses.
Two callers pass it (`src/container.rs` and `initialize_from_tracker_core`).

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | SKIPPED | Existing tests are clean. | Recorded above. |
| R2 | DONE | Add C1-C5. | 98 package unit tests passed (stable Rust 1.99.0); pedantic Clippy, nightly formatting (nightly 1.101.0), and `git diff --check` clean. Five mutations applied together (public URL and external IP dropped from the announce service, a fixed scrape instance ID, a fixed ban limit of 10, and a per-container cookie key): exactly C1-C5 failed and the two existing tests passed; reverted by hand. Production code unchanged. |
| R3 | SKIPPED | No integration increment: the containers run in-process with an in-memory tracker core. | Explicit rationale. |
| R4 | DONE | Re-run unit-only coverage; record before/after. | 183 / 183 to 317 / 317 lines; see Results. |

## Results

- **Unit-only coverage (test-inclusive file totals, `cargo-llvm-cov 0.6.16`):** 183 / 183
  (100.00%) to 317 / 317 (100.00%) lines; 333 / 333 regions; 30 / 30 functions. The gain is the
  wiring decisions now observed, not coverage.
- **Tests added:** 5 (C1-C5); the fixture now takes the configuration, limit, and instance ID.
- **Finding:** recorded in [lessons.md](../lessons.md).
- **Integration selected:** No.
- **Completed-file review:** Approved by Jose Celano on 2026-10-14.

## Prose-First AAA Review

Recorded on 2026-10-15, late: the PR #2538 Task Reviewer found no record of it (AC7). For each
test this plan added, temporary Arrange, Act, and Assert prose was compared with the code and then
removed. #2458's two tests are its own.

| Test | Arrange → Act → Assert | Outcome |
| --- | --- | --- |
| C1 `..._give_every_service_the_configured_public_url` | Configured URL → build the container → three services report it | No change |
| C2 `..._give_the_container_and_every_service_its_configuration_instance_id` | Instance ID 7 → build the container → container and services report it | No change |
| C3 `..._ban_an_ip_only_after_more_than_the_configured_connection_id_errors` | Limit 3, three errors → `is_banned`, one more error, `is_banned` → `(false, true)` | No change |
| C4 `..._announce_a_loopback_client_with_the_configured_external_ip` | External IP, loopback client → `handle_announce` → peer at external IP and request port | Changed: info hash and port are read from the request instead of repeated literals, and the loopback client port now differs from the request port, so a port mix-up fails |
| C5 `..._accept_in_one_container_a_connection_id_issued_by_another_container_built_from_the_same_services` | ID from the first container → second container's `authenticate` → issue time | No change |

## Approval

Approved by Jose Celano on 2026-10-14: C1-C5 and recording the finding.

## Progress Log

- 2026-10-14 14:55 UTC - GitHub Copilot - Created the plan from `container.rs` after the #2505
  rebase, the services' getters, the announce service's external-IP test, and the production
  multi-tracker composition in `src/container.rs`. No Rust tests or production code changed. R2
  awaits maintainer approval.
- 2026-10-14 14:58 UTC - Jose Celano - Approved C1-C5 and the finding.
- 2026-10-14 15:02 UTC - GitHub Copilot - Implemented R2 and measured R4 (stable Rust 1.99.0,
  nightly 1.101.0, `cargo-llvm-cov 0.6.16`). The plan awaits completed-file review.
- 2026-10-14 15:05 UTC - Jose Celano - Approved completed-file review. The file is ready for
  pre-commit and its signed per-file commit.
- 2026-10-15 10:30 UTC - GitHub Copilot - Recorded the prose-first AAA review late, after the
  PR #2538 Task Reviewer found no record (AC7). One change, in C4. Unit-only file totals:
  322 / 322 lines, 338 / 338 regions, 30 / 30 functions.
