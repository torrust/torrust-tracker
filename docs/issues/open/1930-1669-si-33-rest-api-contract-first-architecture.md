---
doc-type: spec
issue-type: task
status: open
priority: p1
epic: 1669
github-issue: 1930
spec-path: docs/issues/open/1930-1669-si-33-rest-api-contract-first-architecture.md
last-updated-utc: 2026-06-22
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/rest-api-core/Cargo.toml
    - packages/rest-api-core/src/container.rs
    - packages/axum-rest-api-server/Cargo.toml
    - packages/axum-rest-api-server/src/v1/context/stats/routes.rs
    - packages/axum-rest-api-server/src/v1/middlewares/auth.rs
    - packages/rest-api-client/src/v1/client.rs
    - docs/issues/open/1669-overhaul-packages/EPIC.md
    - docs/issues/open/1669-overhaul-packages/DECISIONS.md
    - docs/packages.md
    - docs/issues/closed/1924-1669-si-30-decouple-rest-api-core-from-udp-internals.md
---

# Issue #1930 - Define REST API contract-first package architecture for EPIC #1669

## Subissue of EPIC #1669 — Overhaul: Packages

This issue defines and documents a contract-first package architecture for the
tracker REST API, so the REST API can evolve toward a reusable standard in
future versions while remaining compatible with the current tracker
implementation during migration.

This issue defines architecture and migration policy now, but does not
implement full API v2 behavior changes yet. It establishes package boundaries
and dependency rules that make v2 and standardization feasible.

The full API package refactor is expected to be handled by a dedicated EPIC,
separate from EPIC #1669.

## Prerequisites

This issue depends on [SI-30 (#1924)](../closed/1924-1669-si-30-decouple-rest-api-core-from-udp-internals.md),
which delivers the UDP-side trait abstractions (`BanningStats`,
`UdpCoreStatsRepository`, `UdpServerStatsRepository`) that the future
`TrackerStatsAdapter` will implement.

## Problem Statement

Current state:

- The REST API has server and client packages, but no dedicated, reusable
  protocol/contract package.
- `rest-api-core` is currently an integration container around tracker internals
  (`tracker-core`, `http-tracker-core`, `udp-tracker-core`, `udp-server`) rather
  than a transport-agnostic API contract layer.
- The Axum server package still owns request/response contract details and is
  wired directly to tracker internal repositories/services in multiple contexts.
- The client package is tightly bound to current v1 URL shape and mostly exposes
  raw `reqwest::Response` values.

Observed downside:

- API contract and implementation concerns are mixed, making package boundaries
  hard to enforce.
- Defining a future tracker-agnostic REST API standard is harder because there is
  no single package that owns protocol semantics.
- Generic clients for multiple tracker implementations are harder to build while
  contract types and behavior mapping remain implementation-local.

## Analysis Summary

From current package dependencies and source structure:

- `rest-api-core` directly depends on tracker internals and composes containers,
  so it behaves as integration glue, not as protocol/contract.
- `axum-rest-api-server` depends both on `rest-api-core` and directly on tracker
  internals, indicating incomplete boundary separation.
- V1 behavior includes known legacy constraints (for example unstructured
  rejection responses and command-style endpoints) tracked by API v2 issue #144.

Conclusion:

- REST API layering should not copy UDP/HTTP tracker layering mechanically.
- The right target is a contract-first architecture with explicit boundaries:
  protocol contract, application/use-cases, and transport adapters.

## Proposed Architecture (Recommended)

Adopt the following package-role model.

### 1. REST API protocol contract package

Create a dedicated package for versioned REST contract artifacts.

Responsibilities:

- Versioned endpoint contract modules (`v1`, `v2`, ...).
- Request/response DTOs, error schemas, and status mapping contracts.
- Auth contract surface (transport-agnostic semantics).
- Optional API capability/introspection structures for future interoperability.

Non-responsibilities:

- No Axum, no runtime server wiring, no tracker database logic.

### 2. REST API application package (use-case layer)

Refactor current `rest-api-core` into an application/use-case layer (or replace
it with a new package and keep `rest-api-core` as compatibility shim during
migration).

Responsibilities:

- Use-case services and ports (traits) for torrents, whitelist, auth keys,
  stats/metrics, health, and administrative commands.
- Deterministic mapping of domain errors to protocol-level error categories.
- Independent from Axum and HTTP transport details.

### 3. REST API server adapter package (Axum)

Keep `axum-rest-api-server` as HTTP transport adapter.

Responsibilities:

- HTTP routing, request extraction, response serialization, middleware,
  observability hooks.
- Binding protocol contract DTOs to application layer calls.

Non-responsibilities:

- No direct business logic or domain orchestration.

### 4. REST API client adapter package

Refactor `rest-api-client` to be a typed client adapter over protocol contracts.

Responsibilities:

- Typed request/response APIs by version.
- Transport error handling and retries/timeouts policy surface.
- Optional raw mode for compatibility, but typed mode should be primary.

## Desired Package and Main Type Map

The following map describes the desired package structure and the main types each
package should own.

Notes:

- Names below are target-oriented. Exact crate names can be finalized during
  implementation.
- Crate and folder names follow EPIC #1669 final-state style for tracker-specific
  packages (`torrust-tracker-*` crates with short folder names).
- `rest-api-core` may be kept temporarily as a compatibility shim while types
  are migrated to the new boundaries.

### `torrust-tracker-rest-api-protocol` in `rest-api-protocol` (new; contract)

Main type groups (examples):

- `v1`, `v2` modules
- endpoint request/response DTOs: `StatsResponse`, `TorrentResponse`, `AddKeyRequest`, `ApiErrorBody`
- contract enums: `ApiVersion`, `ErrorCode`, `AuthScheme`
- query/path DTOs: `TorrentsQuery`, `InfoHashPath`

### `torrust-tracker-rest-api-application` in `rest-api-application` (new or refactored from `rest-api-core`)

Main type groups (examples):

- port traits: `TorrentQueryPort`, `WhitelistCommandPort`, `AuthKeyCommandPort`,
  `StatsQueryPort`, `HealthQueryPort`
- use-case services: `TorrentApiService`, `WhitelistApiService`, `StatsApiService`
- app-level errors and mappers: `ApiUseCaseError` and mapping to contract errors

### `torrust-tracker-rest-api-runtime-adapter` in `rest-api-runtime-adapter` (new; tracker-specific bridge)

Main type groups (examples):

- adapter implementations for ports: `TrackerTorrentQueryAdapter`,
  `TrackerWhitelistAdapter`, `TrackerStatsAdapter`
- dependency composition container: `TrackerRestApiRuntimeContainer`
- tracker internal integrations for `tracker-core`, `http-tracker-core`,
  `udp-tracker-core`, and `udp-server`

Note: the underlying UDP traits (`BanningStats`, `UdpCoreStatsRepository`,
`UdpServerStatsRepository`) are delivered by [SI-30 (#1924)](../closed/1924-1669-si-30-decouple-rest-api-core-from-udp-internals.md).
This issue wires them through the `TrackerStatsAdapter`.

### `torrust-tracker-axum-rest-api-server` in `axum-rest-api-server` (existing; transport adapter)

Main type groups (examples):

- HTTP-only types: `RouterConfig`, middleware state, extractor wrappers
- thin endpoint handlers over application services
- HTTP <-> protocol DTO serialization/deserialization types

### `torrust-tracker-rest-api-client` in `rest-api-client` (existing; client adapter)

Main type groups (examples):

- typed clients per version: `V1Client`, `V2Client`
- transport abstraction: `HttpTransport`
- typed client errors: `ClientError`, `ApiErrorResponse`
- optional raw-response compatibility entrypoints

### Type Ownership Rules

- Contract DTOs and protocol error bodies belong only to the protocol package.
- Application use-cases and ports belong only to the application package.
- Tracker-internal wiring and repository/service adaptation belong only to the
  runtime adapter package.
- Axum-specific request extractors and middleware state belong only to the Axum
  server package.
- Client transport and retries/timeouts belong only to the client package.

### Transitional Mapping from Current Types

- `TrackerHttpApiCoreContainer` moves out of `rest-api-core` ownership and
  becomes a runtime adapter concern.
- `v1/context/*/resources` DTOs in Axum server migrate to protocol package
  version modules.
- `rest-api-client` request/response types align to protocol DTOs (instead of
  primarily returning raw `reqwest::Response`).

## Execution Strategy

To reduce risk and avoid overloading EPIC #1669, implementation should proceed
in two stages.

### Stage 1 - Proof-of-concept branch (single endpoint)

Create a dedicated PoC branch to validate the architecture with one endpoint
only (recommended: torrent detail endpoint).

Expected PoC outcomes:

- Confirm package boundaries are practical.
- Confirm adapters add value without excessive complexity.
- Confirm handler/application/adapter contract can be tested cleanly.
- Document what should be adjusted before large-scale migration.

### Stage 2 - Dedicated API package-refactor EPIC

After PoC validation, open a new EPIC focused exclusively on API package
restructuring and progressive migration.

That EPIC should own:

- Incremental endpoint migration plan.
- Contract evolution governance.
- Migration checkpoints and rollout sequencing.

### Policy during EPIC #1669

Until the dedicated API refactor EPIC is opened and executed:

- Do not extract REST API packages to standalone repositories.
- Do not publish REST API packages as stable external contracts.
- Treat this spec as a planning reminder and architecture direction only.

Rationale:

- API packages are expected to change significantly soon.
- Extraction/publication now would increase churn and migration cost.
- Simpler EPIC #1669 subissues can continue in parallel while API refactor is deferred.

## Example - Single Endpoint Through Target Layers

The PoC can use the current torrent detail endpoint
`get_torrent_handler` (`GET /api/v1/torrent/{info_hash}`) as reference.

Current handler location:

- [packages/axum-rest-api-server/src/v1/context/torrent/handlers.rs](../../../packages/axum-rest-api-server/src/v1/context/torrent/handlers.rs)

### Before (current coupling)

- Axum handler parses path parameter.
- Axum handler calls tracker-core service directly.
- Axum handler maps domain result to HTTP response.

### After (target layering)

1. Protocol package (`rest-api-protocol`):
   request/response DTOs and error contract.
2. Application package (`rest-api-application`):
   use case + port trait (`TorrentQueryPort`).
3. Runtime adapter package (`rest-api-runtime-adapter`):
   tracker-specific implementation of `TorrentQueryPort`.
4. Axum package (`axum-rest-api-server`):
   HTTP extraction + call use case + map use-case error to HTTP response.

Illustrative flow:

`HTTP request -> Axum handler -> GetTorrentUseCase -> TorrentQueryPort -> TrackerTorrentQueryAdapter -> tracker-core`

Benefits validated by this PoC:

- Tracker internals can change behind adapter boundary.
- Use case can be unit-tested without Axum.
- Handler remains transport-focused and thin.
- Same use case can be reused by non-Axum transports if needed.

## Dependency Rules (Target)

Allowed edges:

- `torrust-tracker-axum-rest-api-server -> torrust-tracker-rest-api-application`
- `torrust-tracker-axum-rest-api-server -> torrust-tracker-rest-api-protocol`
- `torrust-tracker-rest-api-client -> torrust-tracker-rest-api-protocol`
- `torrust-tracker-rest-api-application -> torrust-tracker-rest-api-protocol`
- `torrust-tracker-rest-api-runtime-adapter -> tracker internals + torrust-tracker-rest-api-application`

Forbidden edges (once migration is complete):

- `torrust-tracker-axum-rest-api-server -> torrust-tracker-core` (direct)
- `torrust-tracker-axum-rest-api-server -> torrust-tracker-http-tracker-core` (direct)
- `torrust-tracker-axum-rest-api-server -> torrust-tracker-udp-tracker-core` (direct)
- `torrust-tracker-axum-rest-api-server -> torrust-tracker-udp-server` (direct)

The forbidden edges are currently present and represent the coupling that this
issue resolves by introducing the application and adapter layers.

### Rationale for Forbidden Edges

The direction `axum-rest-api-server → tracker-core` is **structurally allowed**
(higher-level package depending on a lower-level one). The edge is forbidden
anyway because of **separation of concerns**:

1. **Prevents domain types from leaking into the API contract.** When the Axum
   handler imports `tracker_core::whitelist::WhitelistManager` directly, changes
   to `tracker-core` internals could ripple into the wire format. The protocol
   package should be the sole source of truth for API types.

2. **Enables testability without the tracker stack.** An Axum handler that takes
   `State<Arc<WhitelistManager>>` can only be tested by spinning up real tracker
   infrastructure. The same handler taking `State<Arc<WhitelistApiService>>`
   (which depends on a port trait from `rest-api-application`) can be tested
   against a mock adapter.

3. **Keeps the Axum server thin — it is a transport adapter only.** Its job is:
   extract HTTP request → call a use-case → serialize to HTTP response. Not:
   construct a `KeysHandler`, call `WhitelistManager::add_torrent_to_whitelist`,
   or map `PeerKeyError` variants.

4. **Enables a tracker-agnostic API in the future.** If `axum-rest-api-server`
   depends on `tracker-core`, the REST API is permanently tied to Torrust's
   tracker implementation. With the contract-first architecture, the same
   protocol and application layers could serve as the REST API for any
   BitTorrent tracker that implements the port traits from
   `rest-api-application`.

In short: `axum-rest-api-server` **can** depend on lower-level packages, but
the correct lower-level package is `rest-api-application` (port traits and
use-cases), not `tracker-core` (domain internals). The bridge between the two
is `rest-api-runtime-adapter`, which is the **only** layer that should import
tracker-internal crates directly.

## Migration Strategy

Use incremental migration to avoid destabilizing running APIs.

Phase 1: Define contract package and freeze v1 contract.

1. Extract current v1 wire contract types into `torrust-tracker-rest-api-protocol`
   (`rest-api-protocol`).
2. Keep v1 behavior parity (including legacy semantics where required).
3. Add compatibility tests to ensure no unintentional v1 break.

Phase 2: Introduce application ports and adapters.

1. Define ports/traits for API use-cases in application layer.
2. Implement tracker runtime adapters using current internals. The UDP-side
   traits (`BanningStats`, `UdpCoreStatsRepository`, `UdpServerStatsRepository`)
   delivered by SI-30 (#1924) are consumed here by `TrackerStatsAdapter`.
3. Switch Axum handlers to application ports, remove direct internal wiring.

Phase 3: Enable v2 on top of the same architecture.

> Scope note: this phase is intentionally out of scope for EPIC #1669
> (Overhaul: Packages). EPIC #1669 should deliver package boundaries and
> dependency cleanup only. API v2 behavior rollout is tracked separately under
> issue #144 and related follow-up work.

1. Implement v2 contract module and status/error semantics (issue #144 scope).
2. Serve v1 and v2 in parallel for migration period.
3. Add conformance tests per API version.

## Alignment with API v2 (#144)

This architecture supports API v2 without coupling v2 rollout to immediate
large-scale internal refactors.

In particular, it creates a safe path for:

- Correct status code behavior per endpoint.
- Cleaner command and resource boundaries.
- Better authorization/error semantics.
- Future tracker-agnostic API standardization.

## Alternatives Considered

### Alternative A - Keep current packages and only refactor endpoints in place (discarded)

Why considered:

- Lower short-term change cost.
- Fastest path for isolated endpoint fixes.

Why discarded:

- Contract and implementation remain coupled.
- Reuse by other trackers and generic clients remains weak.
- Repeated endpoint fixes will keep accumulating architecture debt.

### Alternative B - Mirror UDP/HTTP tracker layering exactly (discarded)

Why considered:

- Symmetry with existing tracker package model.

Why discarded:

- REST protocol concerns are broader than parser/codec concerns (status codes,
  auth semantics, error schema, resource and command modeling).
- A strict clone of UDP/HTTP layering does not naturally represent REST contract
  governance needs.

### Alternative C - Jump directly to v2 redesign before package boundary refactor (discarded)

Why considered:

- Delivers visible API improvements quickly.

Why discarded:

- High rework risk while boundaries are unclear.
- Harder to keep v1 compatibility and to extract reusable contract assets.

## Scope

### In Scope

- Define target package architecture for REST API contract/application/adapters.
- Define allowed and forbidden dependency edges.
- Define migration phases and compatibility approach for v1/v2.
- PoC branch with one endpoint (torrent detail recommended).
- Consume UDP-side traits from SI-30 (#1924).

### Out of Scope

- Full API v2 behavior changes (tracked in issue #144).
- Extracting any package to a standalone repository during EPIC #1669.
- Publishing any REST API package as a stable external contract.
- Changing the HTTP tracker or UDP tracker layers.

## Verification / Progress

- [x] PoC branch `1930-rest-api-contract-first-poc` created. Draft PR: [#1936](https://github.com/torrust/torrust-tracker/pull/1936).
- [x] `torrust-tracker-rest-api-protocol` package scaffolded with v1 DTOs (Torrent, Peer, ListItem, ActionStatus).
- [x] README, AGPL-3.0 LICENSE, Containerfile stubs added.
- [x] Pre-commit checks pass (machete, deny, linter, doc tests).
- [x] `axum-rest-api-server` depends on protocol DTOs instead of owning them locally.
- [x] Pre-push checks pass (nightly fmt + check + doc, `cargo test --tests --benches --examples --workspace --all-targets --all-features`). **Results**: pre-push passed on push, waiting for CI confirmation.
- [x] PoC torrent detail endpoint (`GET /api/v1/torrent/{info_hash}`) migrated through all four target layers:
  - `rest-api-protocol`: Torrent/Peer/ListItem DTOs
  - `rest-api-application`: `TorrentQueryPort` + `TorrentApiService` use case
  - `rest-api-runtime-adapter`: `TrackerTorrentQueryAdapter` + conversion functions
  - `axum-rest-api-server`: handler dispatches via use case instead of direct `tracker-core`
- [x] Target architecture documented in `docs/packages.md` and `docs/adrs/`. Verdict: ADR 20260623200526 + packages.md REST API section.

## Follow-up Tasks

### Rename `updated_milliseconds_ago` to clarify wire semantics

The `Peer.updated_milliseconds_ago` field was introduced in commit `bc3d246f` (Nov 2022) as a rename of the original `updated` field. Both fields hold the **same value**: a Unix timestamp in milliseconds (from `DurationSinceUnixEpoch::as_millis()`). The `_ago` suffix is misleading — it suggests a relative duration, not an absolute timestamp.

The original intent was to add the unit "milliseconds" to the field name (hypothesis #2), not to introduce a new duration-based field.

**Proposed fix:** Rename `updated_milliseconds_ago` to `updated_milliseconds` in the v1 protocol DTO, and remove the deprecated `updated` field. This is a breaking change for v3.0.0.

**Scope:**

- `rest-api-protocol`: rename field in `Peer` DTO
- `rest-api-runtime-adapter`: update `from_domain_peer` conversion
- `axum-rest-api-server` test assertions that reference the old name
- REST API client if parsing the field by name
- Documentation / API docs
