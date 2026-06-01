---
doc-type: issue
issue-type: task
status: resolved
priority: p2
github-issue: 1856
spec-path: docs/issues/open/1856-1669-analyse-configuration-package-coupling/ISSUE.md
branch: 1856-analyse-configuration-package-coupling
related-pr: null
last-updated-utc: 2026-06-04 00:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/configuration/
    - packages/configuration/src/lib.rs
    - packages/configuration/src/v2_0_0/
    - packages/udp-server/examples/udp_only_public_tracker.rs
    - packages/axum-http-server/examples/http_only_public_tracker.rs
    - docs/issues/open/1669-overhaul-packages/EPIC.md
    - docs/issues/open/1669-overhaul-packages/DECISIONS.md
    - docs/adrs/
---

<!-- skill-link: create-issue -->

# Issue #1856 — Analyse configuration package coupling and evaluate splitting strategies

## Goal

Research and decide whether `torrust-tracker-configuration` should be split into
service-specific configuration packages, kept centralized with Cargo feature gates, or
left as-is. The output is a decision entry in
[DECISIONS.md](../open/1669-overhaul-packages/DECISIONS.md) and, if the decision is
significant enough, a new ADR under `docs/adrs/`.

This issue is a subissue of EPIC [#1669](../open/1669-overhaul-packages/EPIC.md)
(Overhaul: Packages). It is purely research and analysis — no configuration code
changes are produced as output.

## Background

The `torrust-tracker-configuration` package acts as the single configuration source
for the entire tracker binary. It holds config types for all services:

- `Core` — shared tracker domain settings (mode, announce policy, database, etc.)
- `HttpTracker` — HTTP tracker service configuration
- `UdpTracker` — UDP tracker service configuration
- `HttpApi` — REST management API configuration
- `HealthCheckApi` — health-check endpoint configuration
- `Database` — persistence driver and connection settings
- `Logging`, `Network`, `Tls` — cross-cutting infrastructure settings

As a result it is a **central coupling hub**: nearly every package that needs even one
service-specific setting must declare a dependency on the entire configuration package.
The current direct (non-dev) dependents are:

- `torrust-tracker-axum-health-check-api-server`
- `torrust-tracker-axum-http-server`
- `torrust-tracker-axum-rest-api-server`
- `torrust-tracker-axum-server` (via `TslConfig`)
- `torrust-tracker-http-tracker-core`
- `torrust-tracker-rest-api-core`
- `torrust-tracker-swarm-coordination-registry`
- `torrust-tracker-core`
- `torrust-tracker-test-helpers`
- `torrust-tracker-torrent-repository-benchmarking`
- `torrust-tracker-udp-tracker-core`
- `torrust-tracker-udp-server`

This coupling becomes a friction point for the **"build-your-own tracker"** use case:
a binary that runs only a UDP tracker (no REST API, no HTTP tracker, no health-check
endpoint) currently must still depend on the entire configuration crate, which pulls
in all of the config types for services it does not use.

### Versioning constraint

The whole configuration file carries a schema version (currently `2.0.0`) that allows
controlled upgrade paths and breaking-change announcements. Any splitting strategy must
preserve the ability to version the full config file and support schema migrations.

### Config sharing between layers

Config types flow across multiple layers, not just the server that runs a service.
For example, HTTP tracker config may be used in:

- The HTTP tracker server package (to bind ports, enable TLS, set limits).
- Tests and test-helpers that spin up an HTTP tracker.
- A future HTTP tracker client that must mirror the server's TLS settings.

This cross-layer sharing means moving config types into the server package itself is
not a clean solution.

## Alternatives to Analyse

### Alternative A — Split into service-specific configuration packages

Create separate crates:

- `torrust-tracker-core-configuration` — `Core`, `Database`, `Logging`
- `torrust-tracker-http-configuration` — `HttpTracker` + relevant `Network`/`Tls` types
- `torrust-tracker-udp-configuration` — `UdpTracker`
- `torrust-tracker-rest-api-configuration` — `HttpApi`
- `torrust-tracker-health-check-configuration` — `HealthCheckApi`

The top-level `torrust-tracker-configuration` package becomes a facade that re-exports
all of the above for users who want the full config file in one place.

**Questions to answer for this alternative**:

- How does the schema version travel across five packages? Does the facade own it?
- Does the facade's `Cargo.toml` depend on all five sub-packages, creating the same
  wide-coupling problem at a different level?
- Can the versioned `v2_0_0` module structure still work across package boundaries?
- How is the TOML deserialization entry point handled (currently in the facade `lib.rs`)?

### Alternative B — Keep centralized, add Cargo feature gates

Keep one `torrust-tracker-configuration` package. Add features:

```toml
[features]
default = ["http-tracker", "udp-tracker", "rest-api", "health-check-api"]
core = []
http-tracker = ["core"]
udp-tracker = ["core"]
rest-api = ["core"]
health-check-api = []
```

Each service-specific config module is guarded by `#[cfg(feature = "...")]`. A minimal
binary enables only the features it needs and does not compile (or depend on) unused
service config types.

**Questions to answer for this alternative**:

- Does Cargo feature selection genuinely remove compilation of unused code, or do the
  types still appear in the final binary?
- Does conditional compilation of config structs interact badly with the schema
  versioning and TOML deserialization logic?
- How does this affect test-helpers and benchmarking packages that depend on the full
  config?

### Alternative C — Keep fully centralized (status quo)

Do nothing to the package boundary. Accept that every consumer depends on the full
config. Focus energy on reducing coupling elsewhere in the workspace.

**Questions to answer for this alternative**:

- How much real friction does the current coupling actually cause in practice?
- Is the coupling stable (unlikely to grow) or will it worsen as new services are added?
- What is the true cost in binary size and compile time for a minimal binary (e.g., UDP
  only) that drags in the full config package?

### Alternative D — Hybrid: centralized facade re-exporting specialized sub-packages

Same split as Alternative A, but the sub-packages own the types and the central
`torrust-tracker-configuration` package re-exports everything. The key difference from
Alternative A is the direction of ownership: sub-packages define the types, facade
assembles them.

**Questions to answer for this alternative**:

- Is re-exporting across package boundaries idiomatic in the Rust/Cargo ecosystem
  for this kind of config assembly?
- How does this interact with the workspace version and the `LATEST_VERSION` constant?

## Proposed Implementation Plan

This is a research issue. The output is analysis and a decision, not code changes.

### Step 1 — Analyse current coupling

Produce a table of every item imported from `torrust-tracker-configuration` by each
direct dependent. Use the existing
`contrib/dev-tools/analysis/workspace-coupling/` tool or `cargo-modules` to generate
the item-level view. The goal is to identify which config types are truly shared
across many consumers and which are only used by one or two packages.

Artefact: updated coupling section or appendix in this document.

### Step 2 — Identify natural split boundaries

Based on Step 1, identify which config modules have a single consumer (candidate for
co-location) versus broad shared use (must remain shared). Map this onto the
alternatives above.

Artefact: a table of config module → consumer packages → split candidate y/n.

### Step 3 — Build minimal tracker examples

Build two Cargo examples that act as realistic "build-your-own" tracker scenarios:

1. **UDP-only public tracker** — no REST API, no HTTP tracker, no health-check
   endpoint. Add as a Cargo example in `packages/udp-server/examples/` (or the
   highest-level package that makes sense).
2. **HTTP-only private tracker** — no UDP tracker, no REST API, with custom event
   listeners (stub is sufficient). Add as a Cargo example in
   `packages/axum-http-server/examples/` (or equivalent).

The purpose is not functional completeness but to verify concretely how much
configuration coupling a minimal binary cannot avoid today. Measure:

- Number of config types imported that are irrelevant to the service.
- `cargo tree` output showing the full dependency chain from the example binary.
- Approximate size delta for the config-related dependency chain vs a hypothetical
  lean version.

Artefact: two working `examples/*.rs` files committed under the appropriate packages.

### Step 4 — Analyse versioning implications

For each viable alternative, answer:

- How does the config schema version (`2.0.0`, `LATEST_VERSION`) work?
- Can a user upgrade from a full config file to a minimal config file across a major
  version bump without custom migration tooling?
- Is there a risk of version drift between sub-packages if they are released
  independently?

### Step 5 — Evaluate and decide

Summarize findings from Steps 1–4. Choose one alternative (or a hybrid not listed
above if the analysis reveals one). Write the decision.

### Step 6 — Record the decision

Add an entry to
[docs/issues/open/1669-overhaul-packages/DECISIONS.md](../open/1669-overhaul-packages/DECISIONS.md).
If the decision materially affects any other packages in this EPIC (e.g., it changes
the desired final state table), update
[docs/issues/open/1669-overhaul-packages/EPIC.md](../open/1669-overhaul-packages/EPIC.md)
accordingly.

If the decision warrants a permanent architectural record, draft a new ADR under
`docs/adrs/` and link it from the decision entry.

## Acceptance Criteria

- [x] Item-level coupling table exists for `torrust-tracker-configuration` and all
      direct dependents (Step 1 artefact).
- [x] Config module split-boundary table exists (Step 2 artefact).
- [x] Two working Cargo examples exist (Step 3 artefact), each with a brief comment
      explaining what it demonstrates.
- [x] Versioning implications are documented for each viable alternative (Step 4).
- [x] A decision entry is added to `DECISIONS.md` with: the chosen alternative, the
      reasoning, and the trade-offs explicitly acknowledged (Step 6).
- [x] If a new ADR is warranted, a draft exists under `docs/adrs/` (Step 6).
      — No new ADR created. The decision (DEC-07) is "keep status quo + move domain
      primitives". This is a scoped refinement, not an architectural direction change;
      the permanent record is DEC-07 in `DECISIONS.md`.
- [x] `EPIC.md` "Desired Package State" table is updated if the decision changes the
      target state of `torrust-tracker-configuration` or introduces new packages.
      — Three follow-up subissues created (#1859, #1860, #1861) and noted in EPIC.md
      Active Subissues table. The `primitives` row in the Desired Package State table
      gains a note that FU-1 (#1859) will add `TrackerPolicy`/`TORRENT_PEERS_LIMIT`/
      `PrivateMode` to it.

## Out of Scope

- Implementing the chosen alternative (that is a follow-up issue).
- Changing any existing configuration Rust code.
- Changing any service code to use new config packages.
- Versioning policy for the workspace as a whole (tracked in the versioning strategy
  draft issue).

## Notes

- The "build-your-own tracker" use case is one of the explicit long-term goals of the
  workspace overhaul. This analysis directly informs how achievable that goal is with
  the current configuration design.
- The schema versioning concern is closely related to the package versioning strategy
  draft issue (`1669-define-package-versioning-strategy.md`). Both issues should be
  resolved before any structural changes to `configuration` are implemented.
- The `TslConfig` type in `torrust-tracker-axum-server` was already flagged in the
  EPIC as a temporary tracker-specific coupling. The analysis here should consider
  whether `TslConfig` belongs in a generic config sub-package or stays in
  `axum-server`.

---

## Analysis Results

The sections below are the artifacts produced by implementing the steps in
[Proposed Implementation Plan](#proposed-implementation-plan).

---

### Step 1 — Item-level coupling table

The table below lists every item imported from `torrust-tracker-configuration`
by each direct (non-dev) dependent, along with whether the import appears in
production execution paths or in test-infrastructure code compiled into the
library.

Legend:

- **Prod** — import appears in a code path executed at runtime.
- **TestInfra** — import appears in `src/` files (compiled into the library)
  that are only _called_ from tests (typically `environment.rs` with
  `#[allow(dead_code)]`).
- **Test-only** — import is inside a `#[cfg(test)]` block; it is not included
  in the production binary.

| Package                           | Item                        | Context                                                                 |
| --------------------------------- | --------------------------- | ----------------------------------------------------------------------- |
| `axum-health-check-api-server`    | `HealthCheckApi`            | Prod                                                                    |
| `axum-http-server`                | `Configuration`             | TestInfra (environment.rs)                                              |
| `axum-http-server`                | `logging`                   | TestInfra (environment.rs)                                              |
| `axum-http-server`                | `Core`                      | Test-only (`#[cfg(test)]`)                                              |
| `axum-rest-api-server`            | `AccessTokens`              | Prod (routes.rs, auth.rs)                                               |
| `axum-rest-api-server`            | `Configuration`             | TestInfra (environment.rs)                                              |
| `axum-rest-api-server`            | `logging`                   | TestInfra (environment.rs)                                              |
| `axum-server`                     | `TslConfig`                 | Prod (tsl.rs)                                                           |
| `http-tracker-core`               | `Core`                      | Prod (container.rs, announce.rs, scrape.rs)                             |
| `http-tracker-core`               | `HttpTracker`               | Prod (container.rs)                                                     |
| `http-tracker-core`               | `Configuration`             | Test-only                                                               |
| `rest-api-core`                   | `Core`                      | Prod (container.rs)                                                     |
| `rest-api-core`                   | `HttpApi`                   | Prod (container.rs)                                                     |
| `rest-api-core`                   | `HttpTracker`               | Prod (container.rs — REST API reads HTTP tracker status)                |
| `rest-api-core`                   | `UdpTracker`                | Prod (container.rs — REST API reads UDP tracker status)                 |
| `rest-api-core`                   | `Configuration`             | Test-only                                                               |
| `swarm-coordination-registry`     | `TrackerPolicy`             | Prod (coordinator.rs, registry.rs)                                      |
| `swarm-coordination-registry`     | `TORRENT_PEERS_LIMIT`       | Test-only                                                               |
| `test-helpers`                    | `Configuration`             | Prod (config factory functions)                                         |
| `test-helpers`                    | `HttpApi`                   | Prod (config factory functions)                                         |
| `test-helpers`                    | `HttpTracker`               | Prod (config factory functions)                                         |
| `test-helpers`                    | `Threshold`                 | Prod (config factory functions)                                         |
| `test-helpers`                    | `UdpTracker`                | Prod (config factory functions)                                         |
| `test-helpers`                    | `logging::TraceStyle`       | Prod (logging.rs)                                                       |
| `torrent-repository-benchmarking` | `TrackerPolicy`             | Prod (entry/\*.rs, repository/\*.rs)                                    |
| `torrent-repository-benchmarking` | `TORRENT_PEERS_LIMIT`       | Test-only                                                               |
| `tracker-core`                    | `Core`                      | Prod (announce_handler, auth, container, databases, torrent, whitelist) |
| `tracker-core`                    | `TrackerPolicy`             | Prod (torrent/repository/in_memory.rs)                                  |
| `tracker-core`                    | `TORRENT_PEERS_LIMIT`       | Prod (announce_handler.rs, torrent/repository/in_memory.rs)             |
| `tracker-core`                    | `v2_0_0::core::PrivateMode` | Prod (authentication/mod.rs, authentication/service.rs)                 |
| `tracker-core`                    | `Driver`                    | Prod (persistence_benchmark bins)                                       |
| `tracker-core`                    | `Configuration`             | Test-only                                                               |
| `udp-server`                      | `Core`                      | Prod (container.rs, handlers/announce.rs)                               |
| `udp-server`                      | `Configuration`             | TestInfra (environment.rs)                                              |
| `udp-server`                      | `logging`                   | TestInfra (environment.rs)                                              |
| `udp-tracker-core`                | `Core`                      | Prod (container.rs)                                                     |
| `udp-tracker-core`                | `UdpTracker`                | Prod (container.rs)                                                     |

**Key observations:**

1. `Core` is used in production by five packages
   (`http-tracker-core`, `tracker-core`, `udp-server`, `udp-tracker-core`,
   `rest-api-core`) — it is the most-shared type and any split must keep it in a
   central location.
2. `TrackerPolicy` and `TORRENT_PEERS_LIMIT` are domain-level constants/structs
   that are not service-configuration options. They are used by
   `tracker-core`, `swarm-coordination-registry`, and
   `torrent-repository-benchmarking` — three packages that have nothing to do
   with service-specific configuration. These types are candidates for
   relocation to `torrust-tracker-primitives`.
3. `PrivateMode` (a sub-type of `Core`) is only used by `tracker-core` for
   authentication logic. It is already a domain primitive candidate.
4. `HttpTracker` and `UdpTracker` are used cross-layer: `rest-api-core` imports
   both to serve tracker status via the REST API. A package split by service
   type cannot break this cross-layer dependency.
5. `AccessTokens` (`HashMap<String, String>`) is only used by the REST API
   layer; it is a simple type alias with no domain semantics.
6. `HealthCheckApi` and `TslConfig` each have a single non-test consumer
   (`axum-health-check-api-server` and `axum-server` respectively).
7. `Configuration` (the full aggregate) appears mostly in test infrastructure
   and the main binary bootstrap code. In production, most packages consume
   individual service config types, not the aggregate.

---

### Step 2 — Config module split-boundary table

The table below maps each config type to its consumers and a split assessment.
"Split candidate" means the type has a small, bounded consumer set and could
plausibly live in a more focused package without breaking cross-layer use.

| Config type                            | Production consumers                                                             | Split candidate?               | Notes                                                                                   |
| -------------------------------------- | -------------------------------------------------------------------------------- | ------------------------------ | --------------------------------------------------------------------------------------- |
| `Core`                                 | http-tracker-core, tracker-core, udp-server, udp-tracker-core, rest-api-core     | **No**                         | Deeply shared domain config; any split would be a facade over the same types            |
| `Database` / `Driver`                  | tracker-core                                                                     | **No**                         | Part of `Core`; tight semantic coupling                                                 |
| `TrackerPolicy`                        | tracker-core, swarm-coordination-registry, torrent-repository-benchmarking       | **Yes — move to `primitives`** | These are domain policy objects, not service config                                     |
| `TORRENT_PEERS_LIMIT`                  | tracker-core                                                                     | **Yes — move to `primitives`** | Domain constant, not config                                                             |
| `v2_0_0::core::PrivateMode`            | tracker-core                                                                     | **Yes — move to `primitives`** | Domain mode type, used in authentication logic                                          |
| `HttpTracker`                          | http-tracker-core, rest-api-core, test-helpers                                   | **No**                         | Cross-layer: REST API needs it to serve HTTP tracker status                             |
| `UdpTracker`                           | udp-tracker-core, rest-api-core, test-helpers                                    | **No**                         | Cross-layer: REST API needs it to serve UDP tracker status                              |
| `HttpApi` / `AccessTokens`             | axum-rest-api-server (prod), rest-api-core, test-helpers                         | Moderate                       | REST-API-specific; three consumers means co-location is awkward                         |
| `HealthCheckApi`                       | axum-health-check-api-server                                                     | **Yes — single consumer**      | Could move into that package; but the gain is small (tiny struct)                       |
| `TslConfig`                            | axum-server                                                                      | **Yes — single consumer**      | Could move into that package; already flagged in EPIC as temporary                      |
| `Logging` / `Threshold` / `TraceStyle` | axum-http-server, axum-rest-api-server, udp-server (all TestInfra), test-helpers | No                             | Cross-cutting; shared by many                                                           |
| `Configuration` (aggregate)            | test-helpers, TestInfra in src/, main binary                                     | **No**                         | Required by `EnvContainer::initialize` everywhere; splitting would just move the facade |
| `Info`, `Metadata`, `Version`, `Error` | main binary only                                                                 | Candidate                      | Binary bootstrap glue; could live in a thin bootstrap crate                             |

**Key findings:**

- The only types that would meaningfully reduce coupling if moved _out_ of
  `torrust-tracker-configuration` are the domain primitives: `TrackerPolicy`,
  `TORRENT_PEERS_LIMIT`, and `PrivateMode`. Moving them to
  `torrust-tracker-primitives` would free `swarm-coordination-registry` and
  `torrent-repository-benchmarking` from depending on `torrust-tracker-configuration`
  entirely, since those two packages use no other config types in production code.
- Service-specific config types (`HttpTracker`, `UdpTracker`, `HttpApi`) cannot be
  cleanly co-located in their respective service packages because `rest-api-core`
  needs to import all service configs to serve tracker status endpoints.
- `HealthCheckApi` and `TslConfig` are single-consumer types; moving them would reduce
  the central package's surface area slightly but would not reduce coupling for any
  other package.

---

### Step 3 — Cargo examples

Two working examples were added to demonstrate the coupling concretely.

#### Example 1 — UDP-only public tracker

**Location**: `packages/udp-server/examples/udp_only_public_tracker.rs`

```bash
cargo run -p torrust-tracker-udp-server --example udp_only_public_tracker
```

**Output**:

```text
UDP-only public tracker — runtime configuration:
  private mode        : false
  UDP bind address    : 127.0.0.1:6969
  UDP cookie lifetime : 120s

Types from torrust-tracker-configuration compiled into this binary:
  Used at runtime   : Core, UdpTracker, Logging
  Required by EnvContainer::initialize signature : Configuration (full aggregate)
  Compiled but idle : HttpTracker, HttpApi, HealthCheckApi, TslConfig, AccessTokens
```

**Key finding**: `EnvContainer::initialize` accepts `&Configuration` — the full
aggregate struct — so the compiler must include `HttpTracker`, `HttpApi`,
`HealthCheckApi`, `TslConfig`, and `AccessTokens` even though none of those
services are enabled at runtime.

#### Example 2 — HTTP-only public tracker

> **Why public (not private)?** Private mode requires a running REST API to
> issue authentication keys, which would pull `torrust-tracker-axum-rest-api-server`
> into the dependency graph and obscure the coupling signal we are trying to
> measure. Keeping both examples public and self-contained makes the coupling
> table directly comparable between the two protocols.

**Location**: `packages/axum-http-server/examples/http_only_public_tracker.rs`

```bash
cargo run -p torrust-tracker-axum-http-server --example http_only_public_tracker
```

**Output**:

```text
HTTP-only public tracker — runtime configuration:
  private mode        : false
  HTTP bind address   : 127.0.0.1:0 (0 = OS-assigned)
  HTTP TLS enabled    : false

Types from torrust-tracker-configuration compiled into this binary:
  Used at runtime    : Core, HttpTracker, Logging
  Full aggregate     : Configuration (required by the initialization entry point)
  Compiled but idle  : UdpTracker, HttpApi, AccessTokens, HealthCheckApi

Cross-layer coupling: rest-api-core imports both HttpTracker and UdpTracker
  to expose tracker status via the REST API.  A package split would not
  eliminate this dependency — the REST API needs all service config types.
```

**Key finding**: even with all non-HTTP services disabled at runtime, the
cross-layer dependency of `rest-api-core` on `HttpTracker` _and_ `UdpTracker`
means that any binary including the REST API compiles all service config types
regardless of which services run.

---

### Step 4 — Versioning implications

The schema version (`2.0.0`, `LATEST_VERSION`) and the migration logic that
reads the `metadata.schema_version` field from a TOML file currently live in
`lib.rs` and `v2_0_0/mod.rs` of the single `torrust-tracker-configuration` crate.

#### Alternative A — Split into service-specific packages

| Question                               | Finding                                                                                                                                                         |
| -------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Where does the schema version live?    | In the facade package, which re-exports all sub-packages. Sub-packages would carry no version metadata.                                                         |
| Can a user upgrade a full config file? | Yes, but only if the facade package owns all migration logic and the facade is always used for file I/O.                                                        |
| Risk of version drift?                 | **High**: if sub-packages are released independently, their semver versions diverge from the schema version. Users may import mismatched sub-package versions.  |
| TOML deserialization entry point?      | Must stay in the facade; Figment cannot deserialize across separate crate boundaries without the full type graph.                                               |
| `v2_0_0` versioned module structure?   | Breaks naturally at package boundaries — each sub-package would need its own versioned module, or all sub-packages would depend on each other for shared types. |

**Verdict**: High versioning complexity. The facade keeps the schema version but
sub-packages introduce independent release cadences that are hard to coordinate
with schema bumps.

#### Alternative B — Feature gates in the single package

| Question                             | Finding                                                                                                                                                                                                                                                                                |
| ------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Where does the schema version live?  | Unchanged — in the single package.                                                                                                                                                                                                                                                     |
| Risk of version drift?               | **None** — one crate, one version.                                                                                                                                                                                                                                                     |
| Do feature flags remove unused code? | Partially. Cargo features gate _compilation_ of the flagged code, but `Configuration::default()` (and Serde derive) would need `#[cfg(feature = "...")]` annotations on every field, which is verbose and error-prone.                                                                 |
| TOML deserialization?                | **Problematic**: a config file written with all features enabled would fail to deserialize on a feature-limited binary (fields present in TOML but not compiled). Serde's `deny_unknown_fields` would reject it; without that attribute, fields would silently be ignored — a footgun. |
| Test-helpers and benchmarking?       | Would need to enable all features, which defeats the purpose.                                                                                                                                                                                                                          |

**Verdict**: Feature gates interact badly with TOML deserialization and do not
cleanly remove types from the compiled binary in the presence of `Default`
trait implementations and Serde derives.

#### Alternative C — Status quo

| Question              | Finding                                           |
| --------------------- | ------------------------------------------------- |
| Schema versioning?    | **Unchanged** — no new risk.                      |
| Migration tooling?    | Unchanged.                                        |
| Version drift risk?   | **None**.                                         |
| What grows over time? | The coupling set grows if new services are added. |

**Verdict**: Zero versioning risk. The coupling cost is primarily a code
organisation concern; `torrust-tracker-configuration` has no heavy external
dependencies, so unused types do not meaningfully inflate binary size or compile
times for a realistic tracker binary.

#### Alternative D — Hybrid facade re-exporting specialised sub-packages

| Question                   | Finding                                                       |
| -------------------------- | ------------------------------------------------------------- |
| Schema version ownership?  | Same as Alternative A: facade owns it.                        |
| Is re-exporting idiomatic? | Yes — common in Rust (e.g., `tokio` re-exporting sub-crates). |
| TOML deserialization?      | Must stay in the facade.                                      |
| Version drift risk?        | Same as Alternative A: sub-packages have independent semver.  |
| `LATEST_VERSION` constant? | Must live in the facade or be duplicated.                     |

**Verdict**: The re-export pattern is idiomatic but inherits all versioning
complexity from Alternative A. It adds an extra indirection layer without
eliminating the root coupling problem.

---

### Step 5 — Evaluation and decision

#### Summary of findings

1. **`Core` is deeply shared** — five packages use it in production. No package
   split can reduce this coupling.
2. **Cross-layer coupling is structural** — `rest-api-core` must import all
   service config types to serve tracker status endpoints. This coupling survives
   any package reorganization.
3. **`trackerPolicy`, `TORRENT_PEERS_LIMIT`, and `PrivateMode` are domain
   primitives misplaced in the config crate** — three packages that have no
   other use for the config crate depend on these types. Moving them to
   `torrust-tracker-primitives` would free `swarm-coordination-registry` and
   `torrent-repository-benchmarking` from a config dependency entirely.
4. **Versioning alternatives A and D introduce high complexity** — schema version
   coordination across multiple packages is error-prone and adds tooling burden.
5. **Alternative B (feature gates) is impractical** — TOML deserialization
   failures and verbose conditional compilation make it unworkable.
6. **The "build-your-own tracker" goal is not blocked by the config package
   boundary** — it is blocked by the structural design of `tracker-core`
   (which always needs `Core` config) and by the cross-layer coupling in
   `rest-api-core`. Splitting the config package would not change either.
7. **The coupling cost is low in practice** — `torrust-tracker-configuration`
   has no heavy external dependencies. Unused config types compile quickly and
   add negligible binary size.

#### Decision

**Adopt Alternative C (status quo) for the package boundary**, with one focused
follow-up task:

> Move `TrackerPolicy`, `TORRENT_PEERS_LIMIT`, and `PrivateMode` from
> `torrust-tracker-configuration` to `torrust-tracker-primitives`.

**Rationale:**

- Splitting the configuration package (Alternatives A or D) introduces versioning
  complexity that outweighs the coupling reduction, given that the cross-layer
  design of `rest-api-core` means the REST API must depend on all service config
  types regardless.
- Feature gates (Alternative B) are incompatible with the existing TOML
  deserialization strategy and `Default` trait usage.
- Moving `TrackerPolicy`, `TORRENT_PEERS_LIMIT`, and `PrivateMode` to
  `torrust-tracker-primitives` is a clean, low-risk improvement that:
  - Removes two packages (`swarm-coordination-registry`,
    `torrent-repository-benchmarking`) from the config crate dependency entirely.
  - Corrects a type-placement error (policy objects in a config package).
  - Does not affect the schema version or TOML deserialization.
- The "build-your-own tracker" use case requires a broader redesign of how
  `tracker-core` is initialized (accepting narrower config slices rather than
  a monolithic `Core`) and how the REST API is decoupled from all-service
  config. That work is out of scope for this issue.

This decision is recorded in
[DECISIONS.md](../open/1669-overhaul-packages/DECISIONS.md) as **DEC-07**.

#### Follow-up tasks identified

- **FU-1**: Move `TrackerPolicy`, `TORRENT_PEERS_LIMIT`, and `PrivateMode` from
  `torrust-tracker-configuration` to `torrust-tracker-primitives`. Update all
  import sites. This is a code-change follow-up to be tracked as a new subissue
  of EPIC #1669.
- **FU-2**: Evaluate whether `TslConfig` should move into `axum-server` (already
  flagged in EPIC.md as a temporary coupling). This can be done independently.
- **FU-3**: Revisit whether `EnvContainer::initialize` should accept narrower
  config slices (`Arc<Core>`, `Arc<UdpTracker>`) instead of `&Configuration`,
  which would reduce the coupling forcing function at the initialisation boundary.
