---
doc-type: research-report
parent-issue: 2482
status: completed
last-updated-utc: 2026-10-24
semantic-links:
  related-artifacts:
    - docs/issues/open/2482-1669-reorganize-shared-test-support/EPIC.md
    - docs/issues/open/1669-overhaul-packages/DECISIONS.md
    - docs/issues/open/1669-overhaul-packages/workspace-coupling-report-2026-10-06.md
    - packages/test-helpers/src/lib.rs
    - packages/axum-http-server/src/testing/environment.rs
    - packages/axum-rest-api-server/src/testing/environment.rs
    - packages/udp-server/src/testing/environment.rs
    - packages/axum-health-check-api-server/src/environment.rs
    - packages/tracker-core/src/test_helpers.rs
    - tests/common/mod.rs
---

# Test Support Code Inventory

Where the workspace keeps code that exists only to support tests, who uses it, and what it costs
the packages that contain it. Input for issue #2482.

Taken on 2026-10-08 against `develop` at `d1388579b`, over the 31 workspace members reported by
`cargo metadata --no-deps`.

Refreshed on 2026-10-21 against `develop` at `9f20a89b8` (32 members). Re-checked:

- the consumers of every module below;
- the nine forced edges;
- `test-helpers`' 7 dependents and its dependencies;
- `tracker-core`'s four unconditional `#[automock]` and `mockall` as a normal dependency;
- the three `pub mod` test declarations.

All are unchanged. Three environment line counts changed (shown below). The new member, `ci-gate`
(#2525), has only inline `#[cfg(test)]` tests; see
[Members without shared test support](#members-without-shared-test-support).

## Method

- Module declarations named `testing`, `test_helpers`, `environment`, `fixtures`, `mock` and
  similar, searched in every member's `src/` with their attributes.
- Consumers: every import of those modules in `src/`, `tests/`, `examples/` and `benches/`.
- Forced dependencies: for each normal workspace dependency of a package, whether production
  source (outside the test module and outside `#[cfg(test)]` blocks) uses it. Each "only the test
  module uses it" result was then checked by hand with `rg`.
- Mocks and test crates: `#[automock]`, `cfg_attr(test, …)`, and the dependency `kind` of
  `mockall`, `testcontainers` and `tempfile` in `cargo metadata`.
- Per-package test folders: `tests/common/`, `tests/fixtures/` and similar.

## Summary

| Pattern | Where | In production builds? | Shared with other packages? | Cost |
| --- | --- | --- | --- | --- |
| A. Public `testing`/`environment` module | `axum-http-server`, `axum-rest-api-server`, `udp-server`, `axum-health-check-api-server` | Yes, unconditionally | Yes | 9 normal dependency edges used only by these modules; test API in the public API |
| B. Shared helper crate | `test-helpers` | No (every dependent uses it as a dev-dependency) | Yes, by 7 packages | 3 unpublished crates pulled in by 2 modules with one consumer |
| C. Crate-private test helpers in `src/` | `tracker-core` | No (`#[cfg(test)]` inside), but declared `pub mod` | No | Empty public modules in the API; not reusable by the crate's own `tests/` |
| D. Unconditional mocks | `tracker-core` (4 database traits) | Yes | Partly: `MockTorrentMetricsStore` is used by `http-core` and `udp-core` unit tests; the other three mocks are not | `mockall` is a normal dependency; `Mock*` types in the public API |
| E. Per-package `tests/` helpers | 7 packages plus the root crate | No | No | None; the expected place for single-package helpers |

## Pattern A: public `testing` and `environment` modules

Each server package exposes an `Environment<S>` generic over its state (`Environment<Stopped>` and
`Environment<Running>`) that builds the services a server needs, starts it on an ephemeral address
and stops it. All four alias `Environment<Running>` as `Started`; only `axum-http-server` and
`udp-server` also alias `Environment<Stopped>` as `Unstarted`. Three packages also expose an
`EnvContainer` that wires those services.

DEC-13 (2026-06-11) moved the three server environments from `src/environment.rs` to
`src/testing/environment.rs`. Each `testing/mod.rs` says the module is "exported unconditionally
from `lib.rs` so that external test packages can import it" and "compiled in all build profiles".
DEC-13 cited `tracker-core/src/test_helpers.rs` as the existing pattern, but that module's
content is `#[cfg(test)]`-gated and crate-private (pattern C; the module is a `pub mod`, empty in a
non-test build), so it never shipped code or dependencies to production.

The production application does not use these environments: the root crate starts the same
servers through its own bootstrap jobs in `src/bootstrap/jobs/`.

| Package | Module (lines) | What `Environment` starts | Consumers |
| --- | --- | --- | --- |
| `axum-http-server` | `src/testing/environment.rs` (484) | HTTP tracker on `Core` + `HttpTracker` config; `add_torrent_peer`, `base_url` | Own `tests/` (9 files), `examples/http_only_public_tracker.rs`, `axum-health-check-api-server` tests |
| `axum-rest-api-server` | `src/testing/environment.rs` (232) | REST API on the full `Configuration`, wiring the HTTP and UDP cores; `add_torrent_peer`, `get_connection_info` | Own `tests/` (6 files), `axum-health-check-api-server` tests |
| `udp-server` | `src/testing/environment.rs` (581) | UDP tracker on `Core` + `UdpTracker` config | Own `tests/server/contract.rs`, own `#[cfg(test)]` unit tests (`handlers/mod.rs`, `server/processor.rs`), `examples/udp_only_public_tracker.rs`, `axum-health-check-api-server` tests |
| `axum-health-check-api-server` | `src/environment.rs` (118) | Health check API on `HealthCheckApi` config and a `Registar` | Own `tests/server/contract.rs` |

Normal dependencies that production source does not use, only the module above:

| Package | Dependencies forced by the test module |
| --- | --- |
| `axum-rest-api-server` | `torrust-tracker-core`, `torrust-tracker-http-core`, `torrust-tracker-udp-core`, `torrust-tracker-udp-server`, `torrust-tracker-swarm-coordination-registry`, `torrust-tracker-rest-api-client` |
| `axum-http-server` | `torrust-tracker-swarm-coordination-registry` |
| `udp-server` | `torrust-tracker-swarm-coordination-registry` |
| `axum-health-check-api-server` | `torrust-tracker-configuration` |

The table lists workspace dependencies only. Third-party normal dependencies that only test code
uses are not counted: the review of #2618 found two in `axum-http-server`, `torrust-clock` (used
only in `#[cfg(test)]` code and `src/testing/environment.rs`) and `torrust-info-hash` (used only in
test modules and `src/testing/environment.rs`). Each environment move audits every normal
dependency of its package.

The 2026-10-06 coupling report (finding 2) listed four of `axum-rest-api-server`'s six; this
inventory adds `tracker-core` and `rest-api-client`, and the health check's `configuration`.

Intra-doc links count as production use here. `axum-http-server`'s only use of
`torrust-tracker-configuration` outside `src/testing/` and `#[cfg(test)]` code is the intra-doc
link at `src/v1/extractors/client_ip_sources.rs:19`, so that edge is not listed. Under O1 the link
would stop resolving once the dependency became optional, so the edge becomes a tenth candidate if
the link is reworded.

Other observations:

- The two examples, which show how to run a single-protocol tracker, start it through the
  test environment. They teach readers to depend on test infrastructure.
- `axum-rest-api-server/src/testing/mod.rs` still says the UDP dependencies are needed at
  runtime until "the prerequisite decoupling in `rest-api-core`". `rest-api-core` no longer
  exists (#1938), and production source no longer uses those dependencies. The subissue that
  moves this environment (#2482 row 6) removes the comment.

## Pattern B: the shared `test-helpers` crate

`torrust-tracker-test-helpers` (published as 3.0.0) is a dev-dependency of 7 packages:
`torrust-tracker`, `tracker-core`, `http-core`, `udp-server`, `axum-http-server`,
`axum-rest-api-server` and `axum-health-check-api-server`.

| Module (lines) | Content | Consumers | Dependencies it brings |
| --- | --- | --- | --- |
| `configuration` (220) | Ephemeral test configurations | All 7 dependents (unit tests, integration tests, benches) | `configuration` |
| `logging` (156) | Test tracing setup and log capture | Tests of the 4 server packages | `configuration`, `tracing`, `tracing-subscriber` |
| `random` (10) | Random strings | The `configuration` module | `rand` |
| `http` (92) | `http_announce`, `http_complete_download`, `http_scrape` | Root `tests/common/mod.rs` only | `client-lib`, `http-protocol`, `primitives`, `torrust-info-hash`, `torrust-peer-id`, `url` |
| `udp` (304) | `udp_announce`, `udp_scrape`, `udp_complete_download`, three `send_invalid_connection_id*` helpers | Root `tests/common/mod.rs` only | `client-lib`, `udp-protocol`, `primitives`, `torrust-peer-id` |

The `http` and `udp` modules arrived on 2026-07-29 with `client-lib` and the two protocol crates,
and the scrape helpers added `primitives` on 2026-10-02, taking the crate from one workspace
dependency (`configuration`) to five. Generic modules and protocol-specific helpers with a single
consumer share one crate, so every dependent compiles the protocol clients.

`test-helpers` is the only crate in patterns A to D that is already published. Its published
3.0.0 exports only `configuration` and `random`; `logging`, `http` and `udp` are unreleased, so
moving `http` and `udp` out does not change the published API. #2583 decides the bump by
comparing the final API with the published one.

**Resolved by #2628.** The `http` and `udp` modules moved to the root crate's `tests/common/`
(`http_requests.rs` and `udp_requests.rs`), and the tables above describe the state before the move.
`test-helpers` now holds `configuration`, `logging` and `random`, and depends only on `rand`,
`configuration`, `tracing` and `tracing-subscriber`. `deny.toml` bans `client-lib`,
`tracker-core`, `swarm-coordination-registry` and `events` for any crate outside their listed
dependents, so `test-helpers` cannot depend on them unnoticed.

## Pattern C: crate-private test helpers in `src/`

| Package | Declaration | Content |
| --- | --- | --- |
| `tracker-core` | `pub mod test_helpers;` (212 lines) | `#[cfg(test)] pub(crate) mod tests { … }`: tracker, handler and database setup for unit tests |
| `tracker-core` | `pub mod whitelist::test_helpers;` (35 lines) | `#[cfg(test)]` whitelist service setup |
| `tracker-core` | `pub mod peer_tests;` (46 lines) | `#![cfg(test)]` unit tests of `Peer` |

Nothing ships, but two of the three `pub mod` declarations (`test_helpers` and
`whitelist::test_helpers`) add empty public modules to the crate's API: their inner modules are
`#[cfg(test)]`, while `peer_tests` carries the inner attribute `#![cfg(test)]`, which removes the
module itself from non-test builds. `pub(crate)` keeps the helpers out of reach of the crate's own
integration tests, which keep separate fixtures in `tests/common/fixtures.rs`.

## Pattern D: unconditional mocks

`tracker-core` annotates four database traits with an unconditional `#[automock]`
(`databases/traits/whitelist.rs`, `auth_keys.rs`, `torrent_metrics.rs`, `schema.rs`). `mockall`
is therefore a normal dependency (it is also a dev-dependency), and the generated `Mock*` types
are part of the public API. Three of them (`MockWhitelistStore`, `MockAuthKeyStore`,
`MockSchemaMigrator`) are used only by `tracker-core`'s own source. `MockTorrentMetricsStore` is
also imported from `torrust_tracker_core::databases` by `#[cfg(test)]` modules of four files in
two other packages: `http-core` (`services/announce.rs`, `services/scrape.rs`) and `udp-core`
(`services/announce.rs`, `services/scrape.rs`). So it is shared test support across packages, not
a crate-private mock, and `#[cfg_attr(test, automock)]` alone would break those four modules.

Corrected on 2026-10-21 by #2618 (the earlier text said only `tracker-core`'s own source uses the
mocks, which was not checked against the other packages).

`events` does this test-only: `#[cfg_attr(test, automock(…))]` with `mockall` as a
dev-dependency. `http-core`, `udp-core`, `udp-server` and `swarm-coordination-registry` also keep
`mockall` as a dev-dependency.

## Pattern E: per-package `tests/` helpers

Helpers used by one package's integration tests live under its `tests/`, compiled only for those
tests:

| Package | Folder |
| --- | --- |
| Root `torrust-tracker` | `tests/common/` (shared by its `[[test]]` targets) |
| `axum-http-server` | `tests/common/` |
| `axum-rest-api-server` | `tests/common/` |
| `udp-server` | `tests/common/` |
| `tracker-core` | `tests/common/` |
| `torrent-repository-benchmarking` | `tests/common/` |
| `torrust-tracker-client` (`console/tracker-client`) | `tests/common/` |
| `axum-health-check-api-server` | `tests/fixtures/` |

This is the right place for helpers with one consumer, but Cargo cannot share it with other
packages.

## Destination of each item

Set by the [ADR](../../../adrs/20261021190318_share_workspace_test_support_through_testing_features.md)
(rules A to E), and drafted as the subissues of the
[EPIC](EPIC.md#subissues). The "Status" column is the state of the item, updated by the subissue
that resolves it.

| Item | Rule and destination | Subissue draft | Status |
| --- | --- | --- | --- |
| A. `udp-server` environment (and its example) | Rule A: behind the `testing` feature of `udp-server` | [Row 4](../../drafts/2482-gate-udp-server-test-environment/ISSUE.md) | Open |
| A. `axum-http-server` environment (and its example) | Rule A: behind the `testing` feature of `axum-http-server` | [Row 5](../../drafts/2482-gate-axum-http-server-test-environment/ISSUE.md) | Open |
| A. `axum-rest-api-server` environment | Rule A: behind the `testing` feature of `axum-rest-api-server` | [Row 6](../../drafts/2482-gate-axum-rest-api-server-test-environment/ISSUE.md) | Open |
| A. `axum-health-check-api-server` environment | Rule A: behind the `testing` feature of `axum-health-check-api-server` | [Row 7](../../drafts/2482-gate-axum-health-check-api-server-test-environment/ISSUE.md) | Open |
| B. `test-helpers` `http` and `udp` modules | Rule B: the root crate's `tests/common/` | [Row 2](../2628-2482-move-test-helpers-protocol-helpers/ISSUE.md) | Resolved by #2628 |
| B. `test-helpers` `configuration`, `logging`, `random` | Rule B: stay in `test-helpers` | None | Kept by the ADR |
| C. `tracker-core` `pub mod` test declarations | Rule C: private `#[cfg(test)] mod` | [Row 3](../../drafts/2482-gate-tracker-core-test-modules-and-mocks/ISSUE.md) | Open |
| D. `tracker-core` database mocks | Rule D: behind the `testing` feature of `tracker-core` | [Row 3](../../drafts/2482-gate-tracker-core-test-modules-and-mocks/ISSUE.md) | Open |
| E. Per-package `tests/common/` | Rule E: unchanged | None | Kept by the ADR |
| Optional and self edges in `cargo metadata` (consequence of rules A and D) | The coupling tool and diagrams label them | [Row 8](../../drafts/2482-label-optional-and-self-dependency-edges/ISSUE.md) | Open |
| Build commands of shipped artifacts (`Containerfile` builds the release binary with `--tests --all-features`; consequence of rules A and D) | Rule A.5: built on their own with explicit features, plus a check | [Row 9](../../drafts/2482-build-shipped-artifacts-without-test-features/ISSUE.md) | Open |

## Members without shared test support

The method found no shared test support in the members not named above. Two of them are recorded
here (added 2026-10-21): `ci-gate` is new since 2026-10-08, and `e2e-tools` holds test tooling:

| Member | Test support |
| --- | --- |
| `ci-gate` (`contrib/dev-tools/checks/ci-gate`, `publish = false`, added 2026-10-20 by #2525) | Inline `#[cfg(test)]` unit tests and its own `tests/cli.rs`; nothing shared, no test-only normal dependency |
| `e2e-tools` (`packages/e2e-tools`, `publish = false`) | Three binaries that call the root crate's `console::ci` runners and `console::profiling`; the runner code stays in the root crate (see below) |

## Related, out of scope

The root crate's CI runners under `src/console/ci/` (E2E and qBittorrent E2E) are test
infrastructure inside the application crate, and `tempfile` is a normal dependency because of
them. The `e2e-tools` binaries call them. The root crate is the application, not a library other
packages depend on, so it does not leak into consumers; it is listed for completeness, and the
EPIC's AC8 scopes it out.
