---
doc-type: issue
issue-type: task
status: open
priority: p2
github-issue: 1856
spec-path: docs/issues/open/1856-1669-analyse-configuration-package-coupling.md
branch: 1669-analyse-configuration-package-coupling
related-pr: null
last-updated-utc: 2026-06-01 00:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/configuration/
    - packages/configuration/src/lib.rs
    - packages/configuration/src/v2_0_0/
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

- [ ] Item-level coupling table exists for `torrust-tracker-configuration` and all
      direct dependents (Step 1 artefact).
- [ ] Config module split-boundary table exists (Step 2 artefact).
- [ ] Two working Cargo examples exist (Step 3 artefact), each with a brief comment
      explaining what it demonstrates.
- [ ] Versioning implications are documented for each viable alternative (Step 4).
- [ ] A decision entry is added to `DECISIONS.md` with: the chosen alternative, the
      reasoning, and the trade-offs explicitly acknowledged (Step 6).
- [ ] If a new ADR is warranted, a draft exists under `docs/adrs/` (Step 6).
- [ ] `EPIC.md` "Desired Package State" table is updated if the decision changes the
      target state of `torrust-tracker-configuration` or introduces new packages.

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
