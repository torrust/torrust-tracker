---
semantic-links:
  related-artifacts:
    - docs/issues/open/1669-overhaul-packages/EPIC.md
    - docs/adrs/
---

# EPIC #1669 — Design Decisions Log

This file records structural options that were **considered and discarded** during the
overhaul of the Cargo workspace package structure (EPIC #1669). Its purpose is to
prevent re-litigating settled decisions and to preserve the reasoning for future
contributors.

At the end of the refactor this log is intended to serve as the primary source material
for a new repo-level ADR documenting why the workspace ended up in its final shape.

**Format**: newest entry first. Each entry has a short title, the date it was decided,
the proposal, the reasoning, and a reference to any supporting artifact.

---

## DEC-10 — Move peer-count cap from a global constant to `AnnouncePolicy::max_peers_per_announce`

**Date**: 2026-06-09
**Status**: Adopted
**Related issue**: [#1864](https://github.com/torrust/torrust-tracker/issues/1864)

### Proposal considered

The hardcoded constant `TORRENT_PEERS_LIMIT = 74` in `torrust-tracker-primitives` was
the sole compile-time control over how many peers the tracker returns per announce
response. The options evaluated were:

1. **Keep the constant** but expose it as a public API so callers can pass it explicitly.
2. **Add a runtime field** `max_peers_per_announce: usize` to `AnnouncePolicy` and remove
   the constant entirely.
3. **Move the cap to `TrackerPolicy`** alongside the existing cleanup policy fields.

### Alternative chosen

Option 2: add `max_peers_per_announce: usize` (default `74`) to `AnnouncePolicy` and
remove `TORRENT_PEERS_LIMIT`. The cap is applied inside `AnnounceHandler::build_announce_data`
via `PeersWanted::limit(max_peers)` at call time, not at `PeersWanted` construction time.

### Why this alternative was adopted

1. **Semantic fit**: `AnnouncePolicy` already governs announce-response behaviour
   (`interval`, `interval_min`). The peer count cap belongs in the same bucket.
2. **Runtime configurability**: operators can tune the cap per deployment without
   recompiling. The previous constant made that impossible.
3. **Cleaner type boundaries**: `PeersWanted` no longer needs to know about a global
   limit when constructed; the limit is injected once at the point of use
   (`build_announce_data`), keeping the type simple and context-free.
4. **Avoiding `TrackerPolicy` scope creep**: `TrackerPolicy` is about data-retention
   behaviour (persistence, ghost peers, etc.). Mixing in a response-size limit there
   would blur its responsibility.
5. **No `From<i32/u32>` impls**: the old `From` impls baked in the compile-time constant.
   Replacing them with `PeersWanted::from_client_request(i32)` makes the cap injection
   point explicit and removes hidden global state from the type system.

### Tradeoffs accepted

- `AnnouncePolicy::new()` now takes a third argument; callers were updated.
- A small scope increase to `AnnouncePolicy` (previously two fields, now three).

---

## DEC-08 — Keep `TslConfig` in tracker configuration and keep `torrust-tracker-axum-server` tracker-scoped

**Date**: 2026-06-03
**Status**: Adopted

### Proposal considered

Move `TslConfig` out of `torrust-tracker-configuration` and either:

- place it in `torrust-tracker-axum-server`, or
- extract it into a new generic package such as `torrust-server-lib` or a dedicated TLS
  DTO crate.

### Alternative chosen

Keep `TslConfig` in `torrust-tracker-configuration`, keep `torrust-tracker-axum-server`
tracker-scoped, and avoid creating a new package just for the TLS DTO.

### Why this alternative was adopted

1. **The configuration type is already the public DTO**: `HttpTracker` is now used as the
   public configuration object for custom tracker composition, so `TslConfig` remains part
   of the tracker-facing configuration contract.
2. **Moving to `axum-server` would worsen the dependency story**: the configuration crate
   would need to import a delivery-layer package to deserialize `HttpTracker.tsl_config`,
   which inverts the desired layering.
3. **A separate DTO/internal-type split is overkill here**: `TslConfig` is a two-field
   struct with no business logic. Treating it like `SocketAddr` is reasonable and avoids
   needless mapping boilerplate.
4. **A generic home is premature**: `server-lib` is broader infrastructure for all Torrust
   HTTP servers, and there is no current cross-project reuse requirement that justifies a
   new TLS-specific package.
5. **Tracker-scoped naming matches reality**: the package is now explicitly scoped to the
   Torrust tracker HTTP services, so depending on tracker configuration types is acceptable
   when it keeps the service API cohesive.

### Trade-offs acknowledged

- `TslConfig` remains coupled to the tracker supervisor configuration schema.
- If the same TLS DTO is ever reused across other Torrust projects, a generic package can
  be reconsidered then.
- The current choice favors simplicity and cohesive tracker APIs over early abstraction.

### Supporting artifacts

- [Issue #1860 spec](../../open/1860-1669-evaluate-tslconfig-move-to-axum-server/ISSUE.md)
- `packages/axum-server/README.md`
- `packages/configuration/src/lib.rs`

---

## DEC-09 — Narrow `EnvContainer::initialize` and `Environment::new` to accept per-service config slices

**Date**: 2026-06-02
**Status**: Adopted

### Proposal considered

Change `EnvContainer::initialize` (and the wrapping `Environment::new`) for the
UDP and HTTP server packages so that they accept the specific config types they
actually need instead of the full `&Arc<Configuration>` aggregate:

- `UdpTrackerEnvironment::new(core_config: &Arc<Core>, udp_tracker_config: &Arc<UdpTracker>)`
- `HttpTrackerEnvironment::new(core_config: &Arc<Core>, http_tracker_config: &Arc<HttpTracker>)`

### Alternative chosen

Adopt narrowing for the UDP tracker server and the HTTP tracker server environment
constructors. The REST API server environment is **not narrowed** in this issue
because it legitimately depends on all service config types to expose tracker status
via the REST API (see DEC-07 trade-offs).

### Why this alternative was adopted

1. **Eliminates the root forcing function**: the `&Arc<Configuration>` parameter was
   the primary reason a UDP-only binary compiled `HttpTracker`, `HttpApi`,
   `HealthCheckApi`, `TslConfig`, and `AccessTokens` types at all. Narrowing the
   constructor signature removes that dependency at the package boundary.

2. **Explicit contracts**: the narrowed signatures document exactly which config
   types each server environment actually uses, making unintentional coupling visible
   at compile time.

3. **Low migration cost**: all existing test call sites extract the narrower slices
   with two lines (`Arc::new(cfg.core.clone())` and
   `Arc::new(cfg.udp_trackers.unwrap()[0].clone())`). Logging setup, which was
   previously bundled in `initialize_global_services`, was already called separately
   by every test and is not a concern of the server environment constructor.

4. **Main binary unaffected**: `AppContainer::initialize` (in `src/container.rs`)
   does not use `Environment::new`; it initializes containers directly. No change
   needed for the production startup path.

### Trade-offs acknowledged

- Every test call site that used `Started::new(&configuration)` must be updated to
  extract the narrower slices first. The update is mechanical and consistent.
- Logging setup (`logging::setup`) is no longer called inside `Environment::new`.
  Callers that need logging must set it up independently (as tests already did).
- The REST API server environment (`axum-rest-api-server`) still takes
  `&Arc<Configuration>` because it needs `Core`, `HttpTracker`, `UdpTracker`, and
  `HttpApi` — narrowing would provide no benefit there.

### Supporting artifacts

- [Issue #1861 spec](../../open/1861-1669-narrow-envcontainer-initialize-config-slices/ISSUE.md)
- `packages/udp-server/src/environment.rs`
- `packages/axum-http-server/src/environment.rs`
- `packages/udp-server/examples/udp_only_public_tracker.rs` — now compiles without
  `HttpTracker`, `HttpApi`, `HealthCheckApi`, `TslConfig`, `AccessTokens`
- `packages/axum-http-server/examples/http_only_public_tracker.rs` — now compiles
  without `UdpTracker`, `HttpApi`, `AccessTokens`, `HealthCheckApi`

---

## DEC-07 — Keep `torrust-tracker-configuration` as a single central package; move domain primitives to `torrust-tracker-primitives`

**Date**: 2026-06-03
**Status**: Adopted

### Proposal considered

Split `torrust-tracker-configuration` into service-specific sub-packages (Alternatives A
and D from issue #1856) or add Cargo feature gates (Alternative B) to allow binaries
that only need a subset of services to avoid compiling irrelevant config types.

### Alternative chosen

Keep the configuration package as a single central package (Alternative C — status quo),
and separately move the three domain primitives that are misplaced in it to
`torrust-tracker-primitives`:

- `TrackerPolicy`
- `TORRENT_PEERS_LIMIT`
- `v2_0_0::core::PrivateMode`

### Why this alternative was adopted

1. **Cross-layer coupling cannot be broken by package splitting**: `rest-api-core`
   imports both `HttpTracker` and `UdpTracker` config types to expose tracker status
   via the REST API endpoints. Even if those types lived in separate packages,
   `rest-api-core` would still depend on all of them. A package split would rename
   the dependencies, not reduce them.

2. **`Core` is deeply shared**: five packages use `Core` in production code paths.
   Any split that included `Core` would be a thin facade over the same type and would
   not reduce coupling.

3. **Versioning complexity of a split is high**: the schema version (`2.0.0`,
   `LATEST_VERSION`) and TOML deserialization entry point (`Figment`) must stay in a
   single facade that owns all types. If sub-packages carry independent semver
   releases, users risk importing mismatched sub-package versions that are not
   aligned with the schema version. Migration tooling complexity increases.

4. **Feature gates are incompatible with TOML deserialization**: `#[cfg(feature)]`
   on struct fields in `Configuration` would cause TOML deserialization failures when
   a config file written with all features enabled is loaded by a feature-limited
   binary. `Configuration::default()` and Serde derive macros compound this problem.

5. **The coupling cost is low in practice**: `torrust-tracker-configuration` has no
   heavy external dependencies (serde, figment, camino, thiserror). Unused config
   types compile in milliseconds and add negligible binary size.

6. **Domain primitives belong in `primitives`**: `TrackerPolicy`, `TORRENT_PEERS_LIMIT`,
   and `PrivateMode` are domain policy objects, not service configuration options.
   Moving them to `torrust-tracker-primitives` frees two packages
   (`swarm-coordination-registry`, `torrent-repository-benchmarking`) from depending
   on `torrust-tracker-configuration` at all, since those two packages use no other
   config types in production code.

### Trade-offs acknowledged

- `swarm-coordination-registry` and `torrent-repository-benchmarking` no longer
  depend on `torrust-tracker-configuration` after FU-1 (#1859, PR #1865) moved the
  domain primitives to `torrust-tracker-primitives`.
- The "build-your-own tracker" use case remains blocked not by the config package
  boundary but by the structural design of `tracker-core` (always needing `Core` config)
  and the cross-layer coupling in `rest-api-core`. Enabling true service-level
  composability requires a broader redesign of how `tracker-core` and `rest-api-core`
  are initialized — out of scope for this issue.

### Follow-up tasks

- **FU-1** ✅ (#1859, PR #1865): Moved `TrackerPolicy`, `TORRENT_PEERS_LIMIT`, and
  `PrivateMode` from `torrust-tracker-configuration` to `torrust-tracker-primitives`.
  All import sites updated; `swarm-coordination-registry` and
  `torrent-repository-benchmarking` no longer depend on the configuration crate.
  Follow-up issue #1864 tracks whether `TORRENT_PEERS_LIMIT` should become a
  runtime config option.
- **FU-2**: Evaluate moving `TslConfig` into `axum-server` (already flagged in EPIC.md
  as a temporary coupling).
- **FU-3**: Evaluate whether `EnvContainer::initialize` should accept narrower config
  slices (`Arc<Core>`, `Arc<UdpTracker>`) instead of `&Configuration` to reduce the
  coupling forcing function at the initialisation boundary.

### Supporting artifacts

- [Issue #1856 spec](../../open/1856-1669-analyse-configuration-package-coupling/ISSUE.md) —
  full analysis including item-level coupling table, split-boundary table, two Cargo
  examples, and versioning implications for all four alternatives.
- `packages/udp-server/examples/udp_only_public_tracker.rs` — UDP-only coupling demo.
- `packages/axum-http-server/examples/http_only_public_tracker.rs` — HTTP-only
  coupling and cross-layer REST API coupling demo.

---

## DEC-06 - Keep domain AnnounceEvent in primitives; map at boundaries

**Date**: 2026-05-26
**Status**: Adopted

### Proposal considered

Move `torrust_tracker_primitives::AnnounceEvent` to a new shared package for
protocol-facing event types, then reuse that type in both HTTP and UDP protocol
crates.

### Alternative chosen

Keep `torrust_tracker_primitives::AnnounceEvent` in the domain primitives
package, keep protocol-local event types inside each protocol crate, and perform
protocol-to-domain mapping only in boundary layers (`http-tracker-core` and/or
`axum-http-tracker-server`).

### Why this alternative was adopted

1. **Layer clarity**: protocol crates should expose protocol DTOs/types, while
   domain event types stay in domain primitives.
2. **Smaller change scope**: SI-14 is a focused decoupling task; moving the
   domain type itself is broader redesign work.
3. **Current code reality**: UDP protocol already has its own announce event
   type; HTTP can follow the same protocol-local pattern.
4. **Lower migration risk**: `torrust_tracker_primitives::AnnounceEvent` is
   heavily used by tracker-core/domain code, so relocating it now would create a
   large compatibility and migration surface.

### Supporting artifacts

- [EPIC.md](EPIC.md) Layer guardrails and Active Subissues
- [1669-14-decouple-http-protocol-from-tracker-primitives.md](../../drafts/1669-14-decouple-http-protocol-from-tracker-primitives.md)

---

## DEC-05 — Keep protocol and tracker-core crates in tracker workspace for now

**Date**: 2026-05-26
**Status**: Adopted

### Proposal

Do not move the following crates to `torrust/torrust-bittorrent` yet:

- `torrust-udp-tracker-protocol`
- `torrust-http-tracker-protocol`
- `torrust-tracker-core`

Keep them in `torrust/torrust-tracker` until coupling and layering are clarified.

### Why it was adopted

1. **Current move value is unclear**: extraction now would likely shift complexity rather than reduce it.
2. **Dependency knot remains unresolved**: `torrust-http-tracker-protocol` currently depends on:
   - `torrust-tracker-core`
   - `torrust-tracker-primitives`
   - `torrust-udp-tracker-protocol`
3. **Prefix policy consistency**: ownership/subdomain prefixes should follow real package boundaries; keep tracker-owned crates in tracker workspace while boundaries remain mixed.

### Revisit trigger

Reconsider moving `torrust-udp-tracker-protocol` and `torrust-http-tracker-protocol` to
`torrust/torrust-bittorrent` after:

1. Protocol crates no longer require tracker-core dependencies for core protocol behavior.
2. The `torrust-http-tracker-protocol` dependency chain above is removed or justified by a cleaner boundary design.
3. The resulting split reduces coupling and maintenance overhead in practice.

### Supporting artifact

[EPIC.md](EPIC.md) Desired Package State and Torrust Dependency Lists sections.

---

## DEC-04 — Match package folder names to crate names without prefix

**Date**: 2026-05-26
**Status**: Adopted

### Proposal

Use package folder names that match the crate name with the ownership prefix removed.
Examples:

- `torrust-tracker-rest-api-client` -> `rest-api-client`
- `torrust-tracker-udp-server` -> `udp-server`

### Why it was adopted

1. **Lower navigation friction**: the folder name can be inferred directly from crate name.
2. **Consistent workspace layout**: the same naming rule applies across packages.
3. **Cleaner documentation tables**: desired-state tables can show old vs new folder names
   explicitly with less ambiguity.

### Supporting artifact

[EPIC.md](EPIC.md) Desired Package State section.

---

## DEC-03 — Prefix indicates ownership/subdomain, not expected reusability

**Date**: 2026-05-26
**Status**: Adopted

### Proposal

Treat crate prefixes as ownership and release-identity markers. Reusability potential is not
encoded in the prefix. Tracker-domain crates use `torrust-tracker-` while organisation-level
shared crates use `torrust-`.

### Why it was adopted

1. **Clear ownership semantics**: prefixes map to workspace/product area rather than guesses
   about future external reuse.
2. **Stable naming over time**: avoids churn from renaming crates whenever perceived
   reusability changes.
3. **Consistent release identity**: tracker-owned crates remain identifiable as tracker crates
   even if reused outside this repository.

### Supporting artifact

[EPIC.md](EPIC.md) naming policy and Desired Package State tables.

---

## DEC-02 — Use `torrust-` as the default prefix for Torrust organisation crates

**Date**: 2026-05-26
**Status**: Adopted

### Proposal

Use `torrust-` as the default prefix for crates published by Torrust organisation
repositories. In practice, that means preferring names such as `torrust-bencode`,
`torrust-dht`, and `torrust-metainfo` rather than extending the prefix to
`torrust-bittorrent-` for every crate in the BitTorrent sub-project.

### Why it was adopted

1. **Shorter crate names**: the extra `bittorrent` segment adds length without adding
   enough value for the common case.
2. **Consistent organisation-level naming**: `torrust-` already scopes the crate to the
   Torrust organisation, which is the most important part for discoverability.
3. **Avoids redundant repetition**: the BitTorrent context is already obvious from the
   surrounding repository and package documentation.
4. **Leaves room for exceptions**: if a future crate really needs a more specific prefix,
   that can be recorded explicitly as an exception rather than becoming the default.

### Supporting discussion

[torrust/bittorrent#64](https://github.com/torrust/torrust-bittorrent/issues/64)
and its comments.

---

## DEC-01 — Do not merge protocol and core packages into feature-gated crates

**Date**: 2026-05-21
**Status**: Discarded

### Proposal

Merge the two protocol crates and the two protocol-specific core crates into single
crates controlled by Cargo features (`udp` and `http`, both disabled by default):

| Before                             | After                                                         |
| ---------------------------------- | ------------------------------------------------------------- |
| `packages/udp-protocol`            | _(removed)_                                                   |
| `packages/http-protocol`           | _(removed)_                                                   |
| `packages/udp-tracker-core`        | _(removed)_                                                   |
| `packages/http-tracker-core`       | _(removed)_                                                   |
| _(new)_                            | `packages/protocol`                                           |
| `packages/tracker-core` (existing) | `packages/tracker-core` (expanded with `udp`/`http` features) |

Crate renames implied:
`bittorrent-udp-tracker-protocol` + `bittorrent-http-tracker-protocol`
→ `bittorrent-tracker-protocol`

`bittorrent-udp-tracker-core` + `bittorrent-http-tracker-core` absorbed into
`bittorrent-tracker-core` as `udp` and `http` features.

### Why it was discarded

1. **Circular dependency blocker**: `bittorrent-http-tracker-protocol` already depends on
   `bittorrent-tracker-core` for four error types. After the merge the chain would be
   `bittorrent-tracker-core[http] → bittorrent-tracker-protocol[http] → bittorrent-tracker-core`,
   which Cargo refuses to compile. Resolving it requires a non-trivial prerequisite
   refactor (relocating error types) not present in the current plan.

2. **Coupling hidden, not removed**: the logical coupling between the packages does not
   decrease. Inter-crate edges (visible to `cargo tree`, enforceable with `cargo deny`)
   become intra-crate feature coupling (invisible by default, no equivalent tooling).

3. **Worse isolation for protocol-specification changes**: a BEP update currently has a
   clean, single-crate blast radius. After the merge a UDP-only change lives in a file
   that also contains HTTP protocol code; reviewers must filter irrelevant context and
   contributors must maintain `#[cfg(feature)]` discipline permanently.

4. **No benefit for cross-protocol same-layer changes**: the genuinely shared
   announce/scrape/whitelist logic already lives in the base `bittorrent-tracker-core`.
   The protocol-specific code in the core packages is not shared — it just sits at the
   same architectural layer.

5. **Extraction becomes harder**: the EPIC's stated direction is to eventually extract
   `bittorrent-*` crates to standalone repositories. A feature-gated merged crate is
   harder to publish with clean SemVer than two independent crates.

6. **Incremental compilation and test isolation degraded**: any change to the merged crate
   invalidates the compiled artifact for all features; per-feature test suites risk
   unintended cross-feature interactions.

### Supporting artifact

[workspace-coupling-report-proposed-merge.md](workspace-coupling-report-proposed-merge.md)
— full "as-if" coupling graph and three-dimension pros/cons analysis.
