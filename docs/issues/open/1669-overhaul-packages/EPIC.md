---
doc-type: epic
issue-type: task
status: planned
priority: p1
github-issue: 1669
spec-path: docs/issues/open/1669-overhaul-packages/EPIC.md
epic-owner: josecelano
last-updated-utc: 2026-05-27 00:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/packages.md
    - docs/issues/open/1669-overhaul-packages/
    - AGENTS.md
---

<!-- skill-link: create-issue -->

# EPIC #1669 - Overhaul: Packages

## Goal

Progressively simplify and clarify the Cargo workspace package structure through a series
of small, focused improvements. The starting point is identifying and extracting packages
that are clearly generic and reusable outside the tracker — doing so reduces complexity for
the remaining packages and makes it easier to see what to do next. This EPIC is intentionally
open-ended: it is re-evaluated whenever packages are added, split, or grown substantially.

## Why This Is Needed

The package structure grew organically over multiple refactoring cycles. As a result, several
concerns are mixed together:

- **Documentation quality is uneven**: package READMEs vary significantly in depth and
  accuracy; some are stubs.
- **Boundary clarity is uncertain**: it is not always obvious whether packages are
  appropriately cohesive, or whether coupling is intentional.
- **Some packages are clearly generic and reusable**: the `bittorrent-*` protocol crates,
  `bencode`, and several utility crates have no tracker-specific logic and would be more
  useful to the wider community as standalone crates in their own repositories. Keeping them
  here adds noise to the workspace and makes their independent evolution harder.
- **Versioning policy is implicit**: all packages share the workspace version; packages
  extracted to separate repos will need their own release cadence.
- **Only 6 of 27 packages are published on crates.io**: all unpublished (confirmed May 2026),
  in particular every `bittorrent-*` crate. Publishing them in-workspace conflicts with
  giving them independent versions; extraction resolves this tension.

The approach is not all-or-nothing. Each small extraction or structural improvement is a
self-contained win. Re-evaluation happens naturally after each change, or when the package
landscape shifts (new packages, splits, significant growth).

## Package Inventory

The workspace currently contains **27 packages** (including the root `torrust-tracker` crate) across three crate-name prefixes.
"Published" means a crate with that name exists on crates.io (verified May 2026).

### `torrust-` prefix (non-`torrust-tracker-`)

| Published on crates.io | Crate Name               | Folder           |
| ---------------------- | ------------------------ | ---------------- |
| No                     | `torrust-clock`          | `clock`          |
| No                     | `torrust-located-error`  | `located-error`  |
| No                     | `torrust-metrics`        | `metrics`        |
| No                     | `torrust-net-primitives` | `net-primitives` |
| No                     | `torrust-server-lib`     | `server-lib`     |

### `torrust-tracker-` prefix

| Published on crates.io | Crate Name                                        | Folder                            |
| ---------------------- | ------------------------------------------------- | --------------------------------- |
| No                     | `torrust-tracker-axum-health-check-api-server`    | `axum-health-check-api-server`    |
| No                     | `torrust-tracker-axum-http-server`                | `axum-http-server`                |
| No                     | `torrust-tracker-axum-rest-api-server`            | `axum-rest-api-server`            |
| No                     | `torrust-tracker-axum-server`                     | `axum-server`                     |
| No                     | `torrust-tracker-client`                          | `console/tracker-client`          |
| Yes                    | `torrust-tracker-configuration`                   | `configuration`                   |
| Yes                    | `torrust-tracker-contrib-bencode`                 | `contrib/bencode`                 |
| No                     | `torrust-tracker-events`                          | `events`                          |
| No                     | `torrust-tracker-http-tracker-core`               | `http-tracker-core`               |
| No                     | `torrust-tracker-http-tracker-protocol`           | `http-protocol`                   |
| Yes                    | `torrust-tracker-primitives`                      | `primitives`                      |
| No                     | `torrust-tracker-rest-api-client`                 | `rest-api-client`                 |
| No                     | `torrust-tracker-rest-api-core`                   | `rest-api-core`                   |
| No                     | `torrust-tracker-swarm-coordination-registry`     | `swarm-coordination-registry`     |
| Yes                    | `torrust-tracker-test-helpers`                    | `test-helpers`                    |
| No                     | `torrust-tracker-core`                            | `tracker-core`                    |
| No                     | `torrust-tracker-client-lib`                      | `tracker-client`                  |
| No                     | `torrust-tracker-torrent-repository-benchmarking` | `torrent-repository-benchmarking` |
| No                     | `torrust-tracker-udp-tracker-core`                | `udp-tracker-core`                |
| No                     | `torrust-tracker-udp-tracker-protocol`            | `udp-protocol`                    |
| No                     | `torrust-tracker-udp-server`                      | `udp-server`                      |

### `bittorrent-` prefix

| Published on crates.io | Crate Name           | Folder    |
| ---------------------- | -------------------- | --------- |
| No                     | `bittorrent-peer-id` | `peer-id` |

**Observation**: only 6 of 27 packages are currently published on crates.io, all of which
carry the `torrust-tracker-` prefix. Every `bittorrent-` and `torrust-axum-` crate is
unpublished. This confirms issue #1659's note that "many new crates have not been published
yet after we refactored the packages."

### External repositories in scope

This EPIC covers coordination with the following external repositories. Packages extracted
from this workspace may land in one of these rather than in a brand-new standalone repository.

#### `torrust/torrust-bittorrent` — <https://github.com/torrust/torrust-bittorrent>

A Cargo workspace for BitTorrent protocol implementations (forked from
[bip-rs](https://github.com/GGist/bip-rs), maintained by the Torrust organisation). It is
actively being cleaned up and is ready to accept new packages. All packages currently have
`publish = false` at the workspace level; a naming prefix must be chosen before any can be
published.

**Packages** (verified May 2026; all `publish = false`):

| Published on crates.io | Crate Name  | Folder               | Internal workspace deps                 | Description                                         |
| ---------------------- | ----------- | -------------------- | --------------------------------------- | --------------------------------------------------- |
| No                     | `bencode`   | `packages/bencode`   | —                                       | Parsing and converting bencoded data                |
| No                     | `util`      | `packages/util`      | —                                       | Shared utilities used across packages               |
| No                     | `handshake` | `packages/handshake` | `util`                                  | BitTorrent handshake trait and implementation       |
| No                     | `magnet`    | `packages/magnet`    | `util`                                  | Parsing and constructing magnet links               |
| No                     | `metainfo`  | `packages/metainfo`  | `bencode`, `util`                       | Parsing and building `.torrent` metainfo files      |
| No                     | `dht`       | `packages/dht`       | `bencode`, `handshake`, `util`          | Bittorrent Mainline DHT implementation              |
| No                     | `peer`      | `packages/peer`      | `bencode`, `handshake`, `util`          | Communication via peer wire protocol (peer-to-peer) |
| No                     | `disk`      | `packages/disk`      | `metainfo`, `util`                      | FileSystem interface for torrent pieces on disk     |
| No                     | `select`    | `packages/select`    | `handshake`, `metainfo`, `peer`, `util` | Piece selection algorithm                           |

**Observation**: all 9 packages use generic unprefixed working names. The README lists two
prefix candidates: `torrust-` (e.g. `torrust-bencode`) and `torrust-bittorrent-`
(e.g. `torrust-bittorrent-bencode`).

For `bencode`, there is one crate lineage: `packages/bencode` in this workspace and
`contrib/bencode` in tracker are the same crate history at different stages. The tracker copy
is the newer implementation and is planned to move back into this workspace, replacing the
older `packages/bencode` code.

**Role in this EPIC**: target destination for `bittorrent-*` packages extracted from this
workspace (`bittorrent-peer-id`). The protocol and tracker-core crates are explicitly
kept in `torrust/torrust-tracker` for now; the move to `torrust/torrust-bittorrent`
will be reconsidered after dependency cleanup.

#### `torrust/bittorrent-primitives` — <https://github.com/torrust/bittorrent-primitives>

A single-package repository containing one crate (`bittorrent-primitives` v0.2.0) whose
sole public type is `InfoHash`. Originally created as the home for foundational BitTorrent
primitive types, it has not grown beyond that single type.

**Packages** (verified May 2026):

| Published on crates.io | Crate Name              | Description                                                |
| ---------------------- | ----------------------- | ---------------------------------------------------------- |
| Yes                    | `bittorrent-primitives` | Core BitTorrent primitive types; currently only `InfoHash` |

**Role in this EPIC**: planned for deprecation. `InfoHash` (and any other BitTorrent
primitive types) will be migrated to a new package inside `torrust/torrust-bittorrent`;
the `torrust/bittorrent-primitives` repository will be archived once the migration is
complete and downstream consumers have updated.

## Desired Package State

This section captures the target package structure as decisions are made. It is updated
progressively — it does **not** represent a complete end-state plan, only the changes that
have been agreed so far.

This section is about the **final state only**. The current state already lives in
`Package Inventory`, so the tables here do not repeat current crate names unless that is
needed to explain a move or rename. Instead, each row focuses on the final crate name and
the change that leads to it.

Packages are grouped by destination: those remaining in this workspace, those migrating to
[`torrust/torrust-bittorrent`](https://github.com/torrust/torrust-bittorrent), and those
moving to their own standalone repository.

### `torrust/torrust-tracker` workspace

These packages will remain in the `torrust-tracker` workspace long-term.

| Published on crates.io | Crate Name                                        | Folder                            | Old crate name                     | Old folder name                |
| ---------------------- | ------------------------------------------------- | --------------------------------- | ---------------------------------- | ------------------------------ |
| No                     | `torrust-tracker-axum-health-check-api-server`    | `axum-health-check-api-server`    | —                                  | —                              |
| No                     | `torrust-tracker-axum-http-server`                | `axum-http-server`                | —                                  | `axum-http-tracker-server`     |
| No                     | `torrust-tracker-axum-rest-api-server`            | `axum-rest-api-server`            | —                                  | `axum-rest-tracker-api-server` |
| No                     | `torrust-tracker-axum-server`                     | `axum-server`                     | —                                  | —                              |
| Yes                    | `torrust-tracker-configuration`                   | `configuration`                   | —                                  | —                              |
| No                     | `torrust-tracker-events`                          | `events`                          | —                                  | —                              |
| No                     | `torrust-tracker-http-tracker-core`               | `http-tracker-core`               | `bittorrent-http-tracker-core`     | —                              |
| Yes                    | `torrust-tracker-primitives`                      | `primitives`                      | —                                  | —                              |
| No                     | `torrust-tracker-rest-api-client`                 | `rest-api-client`                 | —                                  | `rest-tracker-api-client`      |
| No                     | `torrust-tracker-rest-api-core`                   | `rest-api-core`                   | —                                  | `rest-tracker-api-core`        |
| No                     | `torrust-tracker-swarm-coordination-registry`     | `swarm-coordination-registry`     | —                                  | —                              |
| Yes                    | `torrust-tracker-test-helpers`                    | `test-helpers`                    | —                                  | —                              |
| No                     | `torrust-tracker-core`                            | `tracker-core`                    | `bittorrent-tracker-core`          | —                              |
| No                     | `torrust-tracker-torrent-repository-benchmarking` | `torrent-repository-benchmarking` | —                                  | —                              |
| No                     | `torrust-tracker-client`                          | `tracker-client`                  | `bittorrent-tracker-client`        | —                              |
| No                     | `torrust-tracker-udp-tracker-protocol`            | `udp-protocol`                    | `bittorrent-udp-tracker-protocol`  | —                              |
| No                     | `torrust-tracker-http-tracker-protocol`           | `http-protocol`                   | `bittorrent-http-tracker-protocol` | —                              |
| No                     | `torrust-tracker-udp-tracker-core`                | `udp-tracker-core`                | `bittorrent-udp-tracker-core`      | —                              |
| No                     | `torrust-tracker-udp-server`                      | `udp-server`                      | —                                  | `udp-tracker-server`           |

> **Note on `torrust-tracker-axum-server`**: This package is classified as `torrust-tracker-` because `tsl.rs` imports `TslConfig` from `torrust-tracker-configuration` and `LocatedError`/`DynError` from `torrust-located-error` (renamed in SI-10, #1823). `TslConfig` remains the temporary tracker-specific dependency: it is a small two-field struct with no tracker-specific logic and could be moved to a generic package. Once that change lands, the package could move to the `torrust-` group as a generic `torrust-axum-server` reusable across the Torrust organisation. A near-identical module already exists in [torrust-index](https://github.com/torrust/torrust-index/blob/develop/src/web/api/server/custom_axum.rs).

### `torrust/torrust-bittorrent` workspace

This section shows the final state directly. It keeps the current workspace packages and the
packages that will be moved in, while distinguishing the two cases in the table.

| Package status | Final crate name    | Folder               | Source / change       | Notes |
| -------------- | ------------------- | -------------------- | --------------------- | ----- |
| Existing       | `torrust-bencode`   | `packages/bencode`   | Rename in destination | [1]   |
| Existing       | `torrust-dht`       | `packages/dht`       | Rename in destination |       |
| Existing       | `torrust-disk`      | `packages/disk`      | Rename in destination |       |
| Existing       | `torrust-handshake` | `packages/handshake` | Rename in destination |       |
| Existing       | `torrust-magnet`    | `packages/magnet`    | Rename in destination |       |
| Existing       | `torrust-metainfo`  | `packages/metainfo`  | Rename in destination |       |
| Existing       | `torrust-peer`      | `packages/peer`      | Rename in destination |       |
| Existing       | `torrust-select`    | `packages/select`    | Rename in destination |       |
| Existing       | `torrust-util`      | `packages/util`      | Rename in destination | [2]   |
| Incoming       | `torrust-bencode`   | `packages/bencode`   | SI-16                 | [3]   |
| Incoming       | `torrust-peer-id`   | `packages/peer-id`   | Move from tracker     | [4]   |
| Incoming       | `torrust-infohash`  | `packages/infohash`  | Replace old copy      | [5]   |

Notes:

1. Will be replaced by the newer `contrib/bencode` code from tracker.
2. May be inlined into consumers rather than published independently.
3. Migrates newer tracker implementation and replaces old `packages/bencode`.
4. No workspace deps; first in the `bittorrent-*` extraction sequence.
5. Migrate `InfoHash` here; then archive `torrust/bittorrent-primitives`.

The following crates remain in `torrust/torrust-tracker` for now:

- `torrust-tracker-udp-tracker-protocol`
- `torrust-tracker-http-tracker-protocol`
- `torrust-tracker-core`

Rationale: current dependencies indicate unresolved layering/coupling. In particular,
`torrust-http-tracker-protocol` currently depends on
`torrust-tracker-primitives` and `torrust-udp-tracker-protocol`. The move can be
revisited after these dependencies are clarified and reduced.

> **Naming policy**: prefix reflects ownership and release identity, not estimated
> reusability. Tracker-owned packages keep the `torrust-tracker-` prefix even when they
> are reusable by non-Torrust tracker implementations. Organisation-level shared crates use
> `torrust-` by default.

### Packages moving to standalone repositories

These packages are extracted to their own repositories under the Torrust organisation.

| Final crate name         | Extracted from                  | Blocked by                                    | Notes                                                         |
| ------------------------ | ------------------------------- | --------------------------------------------- | ------------------------------------------------------------- |
| `torrust-clock`          | `torrust-tracker-clock`         | SI-02 + SI-09 (rename first)                  | Rule P; published; 11 workspace consumers to migrate          |
| `torrust-located-error`  | `torrust-tracker-located-error` | SI-10 (rename first)                          | Rule P; published; extraction spec TBD                        |
| `torrust-metrics`        | `torrust-tracker-metrics`       | SI-08 (rename first)                          | 7 workspace consumers to migrate                              |
| `torrust-net-primitives` | `torrust-net-primitives`        | Extraction issue TBD                          | Created by SI-05; standalone extraction planned               |
| `torrust-server-lib`     | `torrust-server-lib`            | Extraction issue TBD                          | Generic server utility crate; standalone extraction candidate |
| `torrust-tracker-client` | `console/tracker-client`        | `bittorrent-*` publication (external to EPIC) | Standalone CLI tool; LGPL-3.0                                 |

### Torrust Dependency Lists (Direct, Non-dev)

This section lists direct crate dependencies that have a `torrust*` prefix.

#### `torrust/torrust-tracker` workspace

- `torrust-tracker-axum-health-check-api-server`
  - `torrust-net-primitives`
  - `torrust-server-lib`
  - `torrust-tracker-axum-server`
  - `torrust-tracker-configuration`
- `torrust-tracker-axum-http-server`
  - `torrust-clock`
  - `torrust-net-primitives`
  - `torrust-server-lib`
  - `torrust-tracker-axum-server`
  - `torrust-tracker-configuration`
  - `torrust-tracker-primitives`
  - `torrust-tracker-swarm-coordination-registry`
- `torrust-tracker-axum-rest-api-server`
  - `torrust-clock`
  - `torrust-metrics`
  - `torrust-net-primitives`
  - `torrust-server-lib`
  - `torrust-tracker-axum-server`
  - `torrust-tracker-configuration`
  - `torrust-tracker-primitives`
  - `torrust-tracker-rest-api-client`
  - `torrust-tracker-rest-api-core`
  - `torrust-tracker-swarm-coordination-registry`
  - `torrust-tracker-udp-server`
- `torrust-tracker-axum-server`
  - `torrust-located-error`
  - `torrust-server-lib`
  - `torrust-tracker-configuration`
- `torrust-tracker-configuration`
  - `torrust-located-error`
  - `torrust-tracker-primitives`
- `torrust-tracker-events`
  - None
- `torrust-tracker-http-tracker-core`
  - `torrust-clock`
  - `torrust-metrics`
  - `torrust-net-primitives`
  - `torrust-tracker-configuration`
  - `torrust-tracker-events`
  - `torrust-tracker-primitives`
  - `torrust-tracker-swarm-coordination-registry`
- `torrust-tracker-primitives`
  - `torrust-clock`
  - `torrust-net-primitives`
- `torrust-tracker-rest-api-client`
  - None
- `torrust-tracker-rest-api-core`
  - `torrust-metrics`
  - `torrust-tracker-configuration`
  - `torrust-tracker-primitives`
  - `torrust-tracker-swarm-coordination-registry`
  - `torrust-tracker-udp-server`
- `torrust-tracker-swarm-coordination-registry`
  - `torrust-clock`
  - `torrust-metrics`
  - `torrust-tracker-configuration`
  - `torrust-tracker-events`
  - `torrust-tracker-primitives`
- `torrust-tracker-core`
  - `torrust-clock`
  - `torrust-located-error`
  - `torrust-metrics`
  - `torrust-tracker-configuration`
  - `torrust-tracker-events`
  - `torrust-tracker-primitives`
  - `torrust-tracker-rest-api-client`
  - `torrust-tracker-swarm-coordination-registry`
- `torrust-tracker-test-helpers`
  - `torrust-tracker-configuration`
- `torrust-tracker-torrent-repository-benchmarking`
  - `torrust-clock`
  - `torrust-tracker-configuration`
  - `torrust-tracker-primitives`
- `torrust-tracker-client`
  - `torrust-located-error`
  - `torrust-net-primitives`
  - `torrust-tracker-primitives`
- `torrust-tracker-udp-tracker-protocol`
  - `torrust-peer-id`
- `torrust-tracker-http-tracker-protocol`
  - `torrust-bencode`
  - `torrust-clock`
  - `torrust-located-error`
  - `torrust-tracker-primitives`
  - `torrust-tracker-udp-tracker-protocol`
- `torrust-tracker-udp-tracker-core`
  - `torrust-clock`
  - `torrust-metrics`
  - `torrust-net-primitives`
  - `torrust-tracker-configuration`
  - `torrust-tracker-events`
  - `torrust-tracker-primitives`
  - `torrust-tracker-swarm-coordination-registry`
- `torrust-tracker-udp-server`
  - `torrust-clock`
  - `torrust-metrics`
  - `torrust-net-primitives`
  - `torrust-server-lib`
  - `torrust-tracker-configuration`
  - `torrust-tracker-events`
  - `torrust-tracker-primitives`
  - `torrust-tracker-swarm-coordination-registry`

#### `torrust/torrust-bittorrent` workspace

- `torrust-bencode`
  - None
- `torrust-dht`
  - `torrust-bencode`
  - `torrust-handshake`
  - `torrust-util`
- `torrust-disk`
  - `torrust-metainfo`
  - `torrust-util`
- `torrust-handshake`
  - `torrust-util`
- `torrust-magnet`
  - `torrust-util`
- `torrust-metainfo`
  - `torrust-bencode`
  - `torrust-util`
- `torrust-peer`
  - `torrust-bencode`
  - `torrust-handshake`
  - `torrust-util`
- `torrust-select`
  - `torrust-handshake`
  - `torrust-metainfo`
  - `torrust-peer`
  - `torrust-util`
- `torrust-util`
  - None
- `torrust-peer-id`
  - None
- `torrust-infohash`
  - None

#### Standalone repositories

- `torrust-clock`
  - None
- `torrust-located-error`
  - None
- `torrust-metrics`
  - `torrust-clock`
- `torrust-net-primitives`
  - None
- `torrust-server-lib`
  - `torrust-net-primitives`
- `torrust-tracker-client`
  - None

## Scope

### In Scope

- Establish a baseline: review package READMEs, produce a dependency graph, identify coupling
  issues.
- Identify packages that are clearly generic and independently reusable outside the tracker.
- For each such candidate, create a dedicated subissue and move it to the appropriate
  destination repository when the decision is made.
- Decide and document the versioning strategy for packages that remain in this workspace
  after extractions.
- Update `docs/packages.md` and `AGENTS.md` Package Catalog after each structural change.
- Re-evaluate the workspace after each extraction to find the next improvement.

### Out of Scope

- All-at-once reorganization of all packages.
- Forced extraction of packages whose independence is unclear or disputed.
- Adding new packages or implementing new tracker features.
- Persistence layer redesign (tracked under
  [#1525](https://github.com/torrust/torrust-tracker/issues/1525)).
- MSRV changes (tracked under
  [#1787](https://github.com/torrust/torrust-tracker/issues/1787)).

## Active Subissues

### Subissue priority rules

When no hard dependency forces a different order, implement subissues according to these
priority levels (lower number = implement first). Hard dependencies always override the
rule priority.

| Rule | Priority | Description                                                                                                                                                                              |
| ---- | -------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| M    | 1        | **Move things between packages** — no crates.io impact; only workspace consumers must update imports.                                                                                    |
| U    | 2        | **Rename unpublished packages** — crate is not on crates.io; only workspace consumers affected; no external migration window needed.                                                     |
| P    | 3        | **Rename published packages** — crate is on crates.io; old and new names coexist for a migration window; external consumers must eventually migrate.                                     |
| E    | 4        | **Extract packages to standalone repositories** — highest effort; requires CI setup, history preservation, and migrating all workspace consumers from path dep to crates.io version dep. |

### Layer guardrails

All package moves, splits, and new package proposals in this EPIC must preserve the
layered architecture below.

#### Layer responsibilities

- Server layer:
  - Delivery and framework integration (Axum, transport wiring, HTTP/UDP endpoint handling).
  - Keep business logic minimal.
- Core layer:
  - Protocol-specific tracker behavior independent from delivery frameworks.
  - Place as much reusable tracker behavior here as practical.
- Tracker-core layer:
  - Central tracker domain and persistence-facing logic (whitelist, keys, tracking, repositories).
- Protocol layer:
  - BEP-defined protocol parsing/encoding and protocol value objects.
  - Should change only with BEP changes or protocol-extension decisions.

#### Dependency direction rules

- `server` may depend on `core`, `tracker-core`, `protocol`, and shared utilities.
- `core` may depend on `tracker-core`, `protocol`, and shared primitives/utilities.
- `tracker-core` may depend on shared primitives/utilities.
- `protocol` may depend on protocol-level primitives/utilities only.

Forbidden edges:

- `core -> server`
- `tracker-core -> core`
- `tracker-core -> protocol`
- `tracker-core -> server`
- `protocol -> core`
- `protocol -> tracker-core`
- `protocol -> server`

#### Subissue architecture checklist

Every subissue touching package boundaries should include:

1. Layer impact summary:
   - Current dependency edge(s).
   - Why each edge violates or respects this model.
   - Target dependency edge(s) after the change.
2. Concrete symbol usage evidence for each problematic edge.
3. Acceptance criteria proving forbidden edges are removed.
4. Verification steps showing dependency diff before/after.

Current known smell to prioritize under these rules:

- `http-protocol` depending on `udp-protocol`.

### Quick list

Status: TODO unless noted.

#### 1. Implemented

- [x] Move `DurationSinceUnixEpoch` from `torrust-tracker-primitives` to `torrust-tracker-clock` _(Rule M; no hard blockers)_
- [x] Define per-package default timeout constants and remove `DEFAULT_TIMEOUT` from `torrust-tracker-configuration` _(Rule M; no blockers)_
- [x] [#1795](https://github.com/torrust/torrust-tracker/issues/1795) Move `AnnouncePolicy` from `torrust-tracker-configuration` to `torrust-tracker-primitives` _(Rule M; no blockers)_
- [x] [#1797](https://github.com/torrust/torrust-tracker/issues/1797) Create `torrust-net-primitives` and move `ServiceBinding` from `torrust-tracker-primitives` _(Rule M + new package; no blockers)_
- [x] [#1813](https://github.com/torrust/torrust-tracker/issues/1813) Resolve `torrust-tracker-core` ↔ `torrust-tracker-rest-api-client` layer violation _(Rule M; prerequisite for `torrust-tracker-core` extraction)_
- [x] [#1816](https://github.com/torrust/torrust-tracker/issues/1816) Align `torrust-` prefix: rename 7 tracker-specific packages to `torrust-tracker-` _(Rule U; no blockers)_
- [x] [#1819](https://github.com/torrust/torrust-tracker/issues/1819) Rename `torrust-tracker-metrics` to `torrust-metrics` _(Rule U; no blockers)_
- [x] [#1821](https://github.com/torrust/torrust-tracker/issues/1821) Rename `torrust-tracker-clock` to `torrust-clock` _(Rule P; no blockers)_
- [x] [#1823](https://github.com/torrust/torrust-tracker/issues/1823) Rename `torrust-tracker-located-error` to `torrust-located-error` _(Rule P; no blockers)_

#### 2. Open GitHub Issue

- [x] [#1829](https://github.com/torrust/torrust-tracker/issues/1829) SI-11: Rename crates and folder names to match desired `torrust-tracker` workspace state _(Rule U; one package at a time)_
- [x] [#1830](https://github.com/torrust/torrust-tracker/issues/1830) SI-12: Decouple `http-protocol` from `tracker-core` _(Rule M; remove forbidden `protocol -> tracker-core` edge)_

#### 3. Numbered Subissues (GitHub Issues Open)

- [ ] [#1834](https://github.com/torrust/torrust-tracker/issues/1834) SI-13: Decouple `http-protocol` from `udp-protocol` _(Rule M; remove cross-protocol dependency edge)_
- [ ] [#1835](https://github.com/torrust/torrust-tracker/issues/1835) SI-14: Decouple `http-protocol` from `torrust-tracker-primitives` _(Rule M; remove protocol -> domain coupling as step 2)_

#### 4. Draft Specs (No Subissue Number, No GitHub Issue)

- [ ] Establish baseline: dependency graph + README audit _(analysis; no blockers; informs all other subissues)_
- [ ] Update all package READMEs _(documentation; after completed rename work; before extractions)_
- [ ] Migrate `contrib/bencode` back to `torrust/torrust-bittorrent`, replacing legacy `packages/bencode` _(Rule E; no blockers within this EPIC)_
- [ ] Extract `torrust-clock` to standalone repository _(Rule E; requires completed clock rename and type move work)_
- [ ] Extract `torrust-metrics` to standalone repository _(Rule E; requires completed metrics rename work)_
- [ ] Extract `torrust-tracker-client` to standalone repository _(Rule E; blocked by `bittorrent-*` publication - external to this EPIC)_
- [ ] Define package versioning strategy (linked vs independent SemVer evolution) _(policy; no blockers; informs extraction and publication cadence)_

Details:

| Item                       | Issue                                                                                                                                                                            | Local Spec                                                                                                                                                                                     | Status | Notes                                                                                          |
| -------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------ | ---------------------------------------------------------------------------------------------- |
| Baseline analysis          | #TBD — Establish baseline: dependency graph + README audit                                                                                                                       | [docs/issues/drafts/1669-01-establish-baseline-analysis.md](../../drafts/1669-01-establish-baseline-analysis.md)                                                                               | TODO   | No blockers; informs extraction decisions                                                      |
| Duration move              | [#1790](https://github.com/torrust/torrust-tracker/issues/1790) — Move `DurationSinceUnixEpoch` from `torrust-tracker-primitives` to `torrust-tracker-clock`                     | [docs/issues/open/1790-move-duration-since-unix-epoch-to-torrust-tracker-clock.md](../../open/1790-move-duration-since-unix-epoch-to-torrust-tracker-clock.md)                                 | DONE   | Rule M; no hard blockers; prerequisite for clock extraction                                    |
| Timeout constants          | [#1793](https://github.com/torrust/torrust-tracker/issues/1793) — Define per-package default timeout constants and remove `DEFAULT_TIMEOUT` from `torrust-tracker-configuration` | [docs/issues/open/1793-1669-03-define-per-package-default-timeout-constants.md](../../open/1793-1669-03-define-per-package-default-timeout-constants.md)                                       | DONE   | Rule M; completed                                                                              |
| Announce policy move       | [#1795](https://github.com/torrust/torrust-tracker/issues/1795) — Move `AnnouncePolicy` from `torrust-tracker-configuration` to `torrust-tracker-primitives`                     | [docs/issues/open/1795-1669-04-move-announce-policy-to-torrust-tracker-primitives.md](../../open/1795-1669-04-move-announce-policy-to-torrust-tracker-primitives.md)                           | DONE   | Rule M; completed                                                                              |
| Net primitives split       | [#1797](https://github.com/torrust/torrust-tracker/issues/1797) — Create `torrust-net-primitives` and move `ServiceBinding` from `torrust-tracker-primitives`                    | [docs/issues/closed/1797-1669-05-create-torrust-net-primitives-and-move-service-binding.md](../../closed/1797-1669-05-create-torrust-net-primitives-and-move-service-binding.md)               | DONE   | Rule M + new package; generic networking type; completed                                       |
| Layer violation fix        | [#1813](https://github.com/torrust/torrust-tracker/issues/1813) — Resolve `torrust-tracker-core` ↔ `torrust-tracker-rest-api-client` layer violation                             | [docs/issues/closed/1813-1669-06-resolve-torrust-tracker-core-rest-api-layer-violation.md](../../closed/1813-1669-06-resolve-torrust-tracker-core-rest-api-layer-violation.md)                 | DONE   | Rule M; stale unused dev dep removed in PR #1804; unblocks `torrust-tracker-core` extraction   |
| Prefix alignment           | [#1816](https://github.com/torrust/torrust-tracker/issues/1816) — Align `torrust-` prefix: rename 7 tracker-specific packages to `torrust-tracker-`                              | [docs/issues/open/1816-1669-07-align-torrust-prefix-rename-tracker-specific-packages.md](../../open/1816-1669-07-align-torrust-prefix-rename-tracker-specific-packages.md)                     | DONE   | Rule U; none of the 7 are published; pure workspace rename; no blockers                        |
| Metrics rename             | [#1819](https://github.com/torrust/torrust-tracker/issues/1819) — Rename `torrust-tracker-metrics` to `torrust-metrics`                                                          | [docs/issues/open/1819-1669-08-rename-torrust-tracker-metrics-to-torrust-metrics.md](../../open/1819-1669-08-rename-torrust-tracker-metrics-to-torrust-metrics.md)                             | DONE   | Rule U; not yet published; no blockers; prerequisite for metrics extraction                    |
| Clock rename               | [#1821](https://github.com/torrust/torrust-tracker/issues/1821) — Rename `torrust-tracker-clock` to `torrust-clock`                                                              | [docs/issues/open/1821-1669-09-rename-torrust-tracker-clock-to-torrust-clock.md](../../open/1821-1669-09-rename-torrust-tracker-clock-to-torrust-clock.md)                                     | DONE   | Rule P; published on crates.io; no blockers; prerequisite for clock extraction                 |
| Located error rename       | [#1823](https://github.com/torrust/torrust-tracker/issues/1823) — Rename `torrust-tracker-located-error` to `torrust-located-error`                                              | [docs/issues/closed/1823-1669-10-rename-torrust-tracker-located-error-to-torrust-located-error.md](../../closed/1823-1669-10-rename-torrust-tracker-located-error-to-torrust-located-error.md) | DONE   | Rule P; completed                                                                              |
| README refresh             | #TBD — Update all package READMEs                                                                                                                                                | [docs/issues/drafts/1669-update-all-package-readmes.md](../../drafts/1669-update-all-package-readmes.md)                                                                                       | TODO   | Documentation; requires completed rename work; before extraction work                          |
| Bencode migration          | #TBD — Migrate `contrib/bencode` to `torrust/torrust-bittorrent` and replace legacy `packages/bencode`                                                                           | [docs/issues/drafts/1669-extract-torrust-tracker-contrib-bencode-to-torrust-bencode.md](../../drafts/1669-extract-torrust-tracker-contrib-bencode-to-torrust-bencode.md)                       | TODO   | Rule E; replaces old `torrust-bittorrent` implementation with newer tracker lineage            |
| Clock extraction           | #TBD — Extract `torrust-clock` to standalone repository                                                                                                                          | [docs/issues/drafts/1669-extract-torrust-clock-to-standalone-repo.md](../../drafts/1669-extract-torrust-clock-to-standalone-repo.md)                                                           | TODO   | Rule E; requires completed duration move and clock rename; 11 workspace consumers to migrate   |
| Metrics extraction         | #TBD — Extract `torrust-metrics` to standalone repository                                                                                                                        | [docs/issues/drafts/1669-extract-torrust-metrics-to-standalone-repo.md](../../drafts/1669-extract-torrust-metrics-to-standalone-repo.md)                                                       | TODO   | Rule E; requires completed metrics rename; 7 workspace consumers to migrate                    |
| Tracker client extraction  | #TBD — Extract `torrust-tracker-client` to standalone repository                                                                                                                 | [docs/issues/drafts/1669-extract-torrust-tracker-client-to-standalone-repo.md](../../drafts/1669-extract-torrust-tracker-client-to-standalone-repo.md)                                         | TODO   | Rule E; blocked by `torrust-tracker-udp-tracker-protocol` publication (external to this EPIC)  |
| Versioning policy          | #TBD — Define package versioning strategy (linked vs independent SemVer evolution)                                                                                               | [docs/issues/drafts/1669-define-package-versioning-strategy.md](../../drafts/1669-define-package-versioning-strategy.md)                                                                       | TODO   | Policy issue; defines release-train vs independent package cadence and migration plan          |
| Rename-to-desired-state    | [#1829](https://github.com/torrust/torrust-tracker/issues/1829) — Rename crates and folder names to match desired `torrust-tracker` workspace state                              | [docs/issues/closed/1829-1669-11-rename-crates-and-folders-to-match-desired-tracker-workspace.md](../../closed/1829-1669-11-rename-crates-and-folders-to-match-desired-tracker-workspace.md)   | DONE   | SI-11 complete; spec archived to `docs/issues/closed/` after issue closure                     |
| HTTP protocol decoupling   | [#1830](https://github.com/torrust/torrust-tracker/issues/1830) — Decouple `http-protocol` from `tracker-core`                                                                   | [docs/issues/closed/1830-1669-12-decouple-http-protocol-from-tracker-core.md](../../closed/1830-1669-12-decouple-http-protocol-from-tracker-core.md)                                           | DONE   | SI-12 complete; removed `http-protocol -> tracker-core` edge and moved mapping to higher layer |
| HTTP/UDP decoupling        | [#1834](https://github.com/torrust/torrust-tracker/issues/1834) — Decouple `http-protocol` from `udp-protocol`                                                                   | [docs/issues/open/1834-1669-13-decouple-http-protocol-from-udp-protocol.md](../../open/1834-1669-13-decouple-http-protocol-from-udp-protocol.md)                                               | TODO   | SI-13. Rule M; remove cross-protocol dependency edge                                           |
| HTTP/primitives decoupling | [#1835](https://github.com/torrust/torrust-tracker/issues/1835) — Decouple `http-protocol` from `torrust-tracker-primitives`                                                     | [docs/issues/open/1835-1669-14-decouple-http-protocol-from-tracker-primitives.md](../../open/1835-1669-14-decouple-http-protocol-from-tracker-primitives.md)                                   | TODO   | SI-14. Rule M; execute after SI-13; remove protocol -> domain coupling in step 2               |

### Draft issues

- [docs/issues/drafts/1669-01-establish-baseline-analysis.md](../../drafts/1669-01-establish-baseline-analysis.md)
- [docs/issues/drafts/1669-update-all-package-readmes.md](../../drafts/1669-update-all-package-readmes.md)
- [docs/issues/drafts/1669-extract-torrust-tracker-contrib-bencode-to-torrust-bencode.md](../../drafts/1669-extract-torrust-tracker-contrib-bencode-to-torrust-bencode.md)
- [docs/issues/drafts/1669-extract-torrust-clock-to-standalone-repo.md](../../drafts/1669-extract-torrust-clock-to-standalone-repo.md)
- [docs/issues/drafts/1669-extract-torrust-metrics-to-standalone-repo.md](../../drafts/1669-extract-torrust-metrics-to-standalone-repo.md)
- [docs/issues/drafts/1669-extract-torrust-tracker-client-to-standalone-repo.md](../../drafts/1669-extract-torrust-tracker-client-to-standalone-repo.md)
- [docs/issues/drafts/1669-define-package-versioning-strategy.md](../../drafts/1669-define-package-versioning-strategy.md)

> New subissues are created as analysis reveals the next improvement. The EPIC is never
> fully planned up front.

## Delivery Strategy

This EPIC uses iterative cycles rather than fixed phases. Each cycle is:

1. **Analyse** — look at the current workspace state (coupling, READMEs, usage patterns).
2. **Identify** — find the smallest, clearest improvement (typically: one package that is
   obviously independent and reusable, or one documentation gap).
3. **Act** — open a focused subissue, implement it, merge it.
4. **Re-evaluate** — with the change landed, repeat from step 1.

The EPIC is re-triggered (a new analysis round starts) whenever:

- A new package is added to the workspace.
- An existing package is split into two.
- A package grows substantially in scope or dependency count.
- A downstream project asks to consume a workspace package independently.

### First cycle (current)

- Outcome: Baseline established — dependency graph committed, READMEs audited, initial
  extraction candidates identified and documented.
- Exit criteria: Baseline analysis subissue merged; at least one extraction candidate has
  a scoped subissue ready.

### Subsequent cycles

Each subsequent cycle produces one or more of:

- An extraction subissue for a clearly independent package.
- A documentation update to `docs/packages.md`.
- An ADR or spec decision (e.g. versioning strategy, naming convention).

There is no predetermined end date or total subissue count.

## Open Questions

These questions do not block starting work, but need answers before specific subissues can
be fully scoped.

### Which packages are the first extraction candidates?

Early intuitions (to be confirmed by the baseline analysis):

- **`bittorrent-*` protocol crates** (`torrust-tracker-http-tracker-protocol`,
  `torrust-tracker-udp-tracker-protocol`, `bittorrent-peer-id`) — implement BEP specs with no
  tracker-specific logic; obvious candidates for migration into `torrust/torrust-bittorrent`.
- **`contrib/bencode`** (`torrust-tracker-contrib-bencode`) — already published on crates.io;
  same crate lineage as `packages/bencode` in `torrust/torrust-bittorrent`; planned to
  replace that older implementation there.
- **Utility crates** (`torrust-clock`, `torrust-located-error`) — generic
  enough to be reused outside the tracker; already published.

Decision criteria to apply per candidate:

- Does it have any tracker-specific logic or dependency?
- Would it benefit a downstream user outside this repository?
- Is its API stable enough for independent semver?
- What CI/release overhead does a separate repository introduce?

### Versioning strategy for remaining packages

The proposed policy — to be confirmed in an ADR — is:

- **Extracted packages** (destination repository): independent versioning from the day of
  extraction. Each extracted package gets its own semver starting point.
- **`torrust-tracker-*` workspace packages**: remain on the shared workspace version.
  These packages are tightly coupled to the tracker's server releases and should bump
  together. Known exceptions that will version independently once extracted:
  - `torrust-tracker-client` — CLI tool being extracted to its own repository.
  - `torrust-located-error` — generic utility package, expected to version independently once
    extracted.
- **`torrust-` workspace packages** (e.g., `torrust-server-lib`): currently follow the
  workspace version but are not tightly bound to the tracker release cadence. Versioning
  strategy for these should be reviewed when they are extracted or decoupled.
- **`bittorrent-*` packages**: independent versions once extracted.

This policy needs a formal ADR before it is enforced. The key open question is: should any
`torrust-tracker-*` package be broken out of the shared workspace version before being
extracted to its own repository?

Current intent (tracked in SI-15 draft) is to define the policy now but defer implementation
until boundary-refactor preconditions are met (at minimum SI-13 and SI-14), so version
migration does not run ahead of layer decoupling.

### Extraction ordering: crates.io publication constraints

When a package is extracted to a standalone repository, all its **runtime** workspace
dependencies must already be published on crates.io (path deps become version deps after
extraction). The table below analyses every current or near-term extraction candidate
against this constraint (verified May 2026).

| Package                                         | Crates.io status | Unpublished runtime workspace deps                                                                                                                      | Can be published independently? | Ordering constraint                                                                                                                     |
| ----------------------------------------------- | ---------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------- |
| `torrust-tracker-contrib-bencode`               | Yes              | None                                                                                                                                                    | ✅ Now                          | SI-16 can migrate it into `torrust/torrust-bittorrent` and replace legacy `packages/bencode`                                            |
| `bittorrent-peer-id`                            | No               | None                                                                                                                                                    | ✅ Now                          | No spec yet; can be extracted first in the `bittorrent-*` sequence                                                                      |
| `torrust-located-error`                         | Yes              | None                                                                                                                                                    | ✅ Already published            | No extraction spec yet                                                                                                                  |
| `torrust-tracker-clock` (→ `torrust-clock`)     | Yes              | None (✅ `torrust-tracker-primitives` dep removed by SI-02 #1790)                                                                                       | ✅ After rename                 | See [extract clock subissue](../../drafts/1669-extract-torrust-clock-to-standalone-repo.md)                                             |
| `torrust-tracker-metrics` (→ `torrust-metrics`) | No               | `torrust-tracker-clock` (published ✅; was `torrust-tracker-primitives` — removed by SI-02 #1790)                                                       | ✅ After rename                 | See [extract metrics subissue](../../drafts/1669-extract-torrust-metrics-to-standalone-repo.md)                                         |
| `torrust-tracker-udp-tracker-protocol`          | No               | `bittorrent-peer-id` (not published)                                                                                                                    | ❌                              | After `bittorrent-peer-id`                                                                                                              |
| `torrust-tracker-core`                          | No               | `torrust-tracker-events`, `torrust-tracker-metrics`, `torrust-tracker-swarm-coordination-registry`, `torrust-tracker-rest-api-client` (all unpublished) | ❌ Very deep chain              | After all four above; also has `torrust-tracker-rest-api-client` as a runtime dep — a layer violation worth resolving before extraction |
| `torrust-tracker-http-tracker-protocol`         | No               | `torrust-tracker-udp-tracker-protocol`, `torrust-tracker-core` (both unpublished)                                                                       | ❌                              | After `torrust-tracker-udp-tracker-protocol` and `torrust-tracker-core`                                                                 |

**Practical extraction order for `bittorrent-*` crates** (once decided):

1. `bittorrent-peer-id` — no workspace deps; extract first.
2. `torrust-tracker-udp-tracker-protocol` — only blocked by #1.
3. `torrust-tracker-core` — needs the four unpublished deps above + clock rename; complex
   chain; the layer violation (`torrust-tracker-rest-api-client` runtime dep) should be
   resolved before or during this step.
4. `torrust-tracker-http-tracker-protocol` — needs #2 and #3 done.

> Workspace renames (this EPIC's current subissues) are independent of extraction ordering —
> a crate can be renamed in-workspace before it is published or extracted.

### Analysis tooling

Four complementary analyses are recommended to assess whether the current package structure
represents coherent bounded contexts:

1. **Dependency graph** — structural coupling: which crates depend on which; detect cycles
   and hotspots. Tools: `cargo metadata`, `cargo-depgraph`, `cargo-modules`, `cargo-deps`.

2. **Semantic domain graph** — conceptual mapping: which crates handle which domain concepts
   (Announce, Scrape, Swarm, Peer, …); identify crates that mix unrelated concerns.

3. **Git co-change graph** — historical coupling: which crates have been modified together
   over time; this often reveals the "real architecture" independent of declared dependencies.
   Tools: `git log`, GitNexus.

4. **Bounded context analysis** — ownership clarity: identify crates that mix concerns
   (e.g. peer validation + database + metrics + protocol parsing in one package).

Recommended pragmatic stack for the baseline analysis:

```text
cargo metadata   →  workspace structure + declared deps
cargo-modules    →  module-level dependency graph
git log          →  co-change history
Graphviz         →  visualization of the above
```

The baseline analysis subissue (SI-01) should pick the tool(s), run them, and commit their
output as artifacts under `docs/issues/open/1669-overhaul-packages/`.

Previously referenced tools (screenshots from CodeScene already in the issue comment):

- [`cargo-depgraph`](https://sr.ht/~jplatte/cargo-depgraph/) — Rust dependency graphs
- [GitNexus](https://github.com/abhigyanpatwari/GitNexus) — Git relationship visualizer
- [CodeScene](https://codescene.io/) — Code quality and hotspot analysis

## Progress Tracking

### Workflow Checkpoints

- [x] Epic spec drafted in `docs/issues/open/`
- [ ] Epic spec reviewed and approved by user/maintainer
- [ ] GitHub epic issue already exists (#1669); issue number added to this spec
- [ ] Baseline analysis subissue created and linked
- [ ] Subissue statuses kept up to date in the `Active Subissues` table
- [ ] For each implemented subissue: automatic checks completed and recorded
- [ ] For each implemented subissue: manual verification completed and recorded
- [ ] For each implemented subissue: acceptance criteria reviewed post-implementation
- [ ] Epic periodically re-evaluated after structural changes (ongoing)

### Progress Log

- 2026-05-15 12:00 UTC - GitHub Copilot - Initial epic spec drafted from issue #1669 body and
  comments.
- 2026-05-15 13:00 UTC - GitHub Copilot - Revised strategy: progressive/iterative approach,
  extraction as first-class action from the start, no fixed phase plan.

## Acceptance Criteria

Because this EPIC is ongoing, acceptance criteria are defined per cycle, not for the
entire EPIC at once. The EPIC is considered healthy (not stale) when:

- [ ] The baseline analysis is merged and the dependency graph is up to date.
- [ ] Every clearly independent package either has an open extraction subissue or a recorded
      decision explaining why extraction was deferred.
- [ ] `docs/packages.md` and `AGENTS.md` Package Catalog are accurate after each change.
- [ ] Every completed subissue includes automated and manual verification evidence.
- [ ] The EPIC spec is reviewed and updated after each significant structural change.

### Acceptance Verification

| AC ID | Status | Evidence                                 |
| ----- | ------ | ---------------------------------------- |
| AC1   | TODO   | {baseline analysis PR link}              |
| AC2   | TODO   | {per-candidate issue or decision record} |
| AC3   | TODO   | {PR link per structural change}          |
| AC4   | TODO   | {per-subissue links}                     |
| AC5   | TODO   | {spec PR link per re-evaluation}         |

## Risks and Trade-offs

- **Extraction execution cost**: Deciding to extract a package is easy; the actual work
  (new repo, CI, publish pipeline, downstream dependency updates) is non-trivial. Scope each
  extraction subissue carefully and do not start one without a clear owner.
- **Documentation drift**: READMEs and `docs/packages.md` updated early may drift if
  structural changes follow. Accept this; a quick second-pass update is cheaper than waiting
  for all decisions to be made before writing any docs.
- **Extraction paralysis**: The progressive approach works only if extractions actually
  happen. Avoid endless analysis — if a package is obviously independent, open the subissue.
- **Tooling lock-in**: CodeScene is a third-party SaaS. Prefer capturing its insights in
  committed documents rather than creating a workflow dependency on external tooling.
- **EPIC staleness**: An open-ended EPIC can quietly go stale. The re-evaluation triggers
  (new package added, package split, etc.) defined in the Delivery Strategy are the
  safeguard against this.

## References

- Design decisions log: [`DECISIONS.md`](DECISIONS.md) — considered-and-discarded options; source material for a future repo-level ADR
- EPIC issue: <https://github.com/torrust/torrust-tracker/issues/1669>
- Relates to: <https://github.com/torrust/torrust-tracker/issues/1659> (Release v4.0.0-rc.1)
- Package architecture: [`docs/packages.md`](../../../packages.md)
- Package diagrams: [`docs/media/packages/`](../../../media/packages/)
- CodeScene screenshots: <https://github.com/torrust/torrust-tracker/issues/1669#issuecomment-4010991467>
- `cargo-depgraph`: <https://sr.ht/~jplatte/cargo-depgraph/>
- GitNexus: <https://github.com/abhigyanpatwari/GitNexus>
- CodeScene: <https://codescene.io/>
