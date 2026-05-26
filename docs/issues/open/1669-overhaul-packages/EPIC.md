---
doc-type: epic
issue-type: task
status: planned
priority: p1
github-issue: 1669
spec-path: docs/issues/open/1669-overhaul-packages/EPIC.md
epic-owner: josecelano
last-updated-utc: 2026-05-18 00:00
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
| No                     | `torrust-tracker-axum-http-server`                | `axum-http-tracker-server`        |
| No                     | `torrust-tracker-axum-rest-api-server`            | `axum-rest-tracker-api-server`    |
| No                     | `torrust-tracker-axum-server`                     | `axum-server`                     |
| No                     | `torrust-tracker-client`                          | `console/tracker-client`          |
| Yes                    | `torrust-tracker-configuration`                   | `configuration`                   |
| Yes                    | `torrust-tracker-contrib-bencode`                 | `contrib/bencode`                 |
| No                     | `torrust-tracker-events`                          | `events`                          |
| Yes                    | `torrust-tracker-primitives`                      | `primitives`                      |
| No                     | `torrust-tracker-rest-api-client`                 | `rest-tracker-api-client`         |
| No                     | `torrust-tracker-rest-api-core`                   | `rest-tracker-api-core`           |
| No                     | `torrust-tracker-swarm-coordination-registry`     | `swarm-coordination-registry`     |
| Yes                    | `torrust-tracker-test-helpers`                    | `test-helpers`                    |
| No                     | `torrust-tracker-torrent-repository-benchmarking` | `torrent-repository-benchmarking` |
| No                     | `torrust-tracker-udp-server`                      | `udp-tracker-server`              |

### `bittorrent-` prefix

| Published on crates.io | Crate Name                         | Folder              |
| ---------------------- | ---------------------------------- | ------------------- |
| No                     | `bittorrent-http-tracker-protocol` | `http-protocol`     |
| No                     | `bittorrent-http-tracker-core`     | `http-tracker-core` |
| No                     | `bittorrent-peer-id`               | `peer-id`           |
| No                     | `bittorrent-tracker-client`        | `tracker-client`    |
| No                     | `bittorrent-tracker-core`          | `tracker-core`      |
| No                     | `bittorrent-udp-tracker-protocol`  | `udp-protocol`      |
| No                     | `bittorrent-udp-tracker-core`      | `udp-tracker-core`  |

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
workspace (`bittorrent-peer-id`, `bittorrent-udp-tracker-protocol`,
`bittorrent-http-tracker-protocol`, and the `bittorrent-tracker-*` crates once their
upstream workspace deps are all published).

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

Each table shows the **final crate name** after all planned changes. Packages are grouped by
destination: those remaining in this workspace, those migrating to
[`torrust/torrust-bittorrent`](https://github.com/torrust/torrust-bittorrent), and those
moving to their own standalone repository.

### Packages remaining in this workspace

These packages will remain in the `torrust-tracker` workspace long-term.

#### `torrust-` prefix

| Published on crates.io | Crate Name               | Folder           | Change                                               |
| ---------------------- | ------------------------ | ---------------- | ---------------------------------------------------- |
| Yes                    | `torrust-clock`          | `clock`          | Renamed from `torrust-tracker-clock` ✓ (SI-09 #1821) |
| Yes                    | `torrust-located-error`  | `located-error`  | Renamed from `torrust-tracker-located-error`         |
| Yes                    | `torrust-net-primitives` | `net-primitives` | New package (created by SI-05)                       |
| No                     | `torrust-metrics`        | `metrics`        | —                                                    |

#### `torrust-tracker-` prefix

| Published on crates.io | Crate Name                                        | Folder                            | Change |
| ---------------------- | ------------------------------------------------- | --------------------------------- | ------ |
| No                     | `torrust-tracker-axum-health-check-api-server`    | `axum-health-check-api-server`    | —      |
| No                     | `torrust-tracker-axum-http-server`                | `axum-http-tracker-server`        | —      |
| No                     | `torrust-tracker-axum-rest-api-server`            | `axum-rest-tracker-api-server`    | —      |
| No                     | `torrust-tracker-axum-server`                     | `axum-server`                     | —      |
| Yes                    | `torrust-tracker-configuration`                   | `configuration`                   | —      |
| No                     | `torrust-tracker-events`                          | `events`                          | —      |
| Yes                    | `torrust-tracker-primitives`                      | `primitives`                      | —      |
| No                     | `torrust-tracker-rest-api-client`                 | `rest-tracker-api-client`         | —      |
| No                     | `torrust-tracker-rest-api-core`                   | `rest-tracker-api-core`           | —      |
| No                     | `torrust-tracker-swarm-coordination-registry`     | `swarm-coordination-registry`     | —      |
| Yes                    | `torrust-tracker-test-helpers`                    | `test-helpers`                    | —      |
| No                     | `torrust-tracker-torrent-repository-benchmarking` | `torrent-repository-benchmarking` | —      |
| No                     | `torrust-tracker-udp-server`                      | `udp-tracker-server`              | —      |

> **Note on `torrust-tracker-axum-server`**: This package is classified as `torrust-tracker-` because `tsl.rs` imports `TslConfig` from `torrust-tracker-configuration` and `LocatedError`/`DynError` from `torrust-located-error` (renamed in SI-10, #1823). `TslConfig` remains the temporary tracker-specific dependency: it is a small two-field struct with no tracker-specific logic and could be moved to a generic package. Once that change lands, the package could move to the `torrust-` group as a generic `torrust-axum-server` reusable across the Torrust organisation. A near-identical module already exists in [torrust-index](https://github.com/torrust/torrust-index/blob/develop/src/web/api/server/custom_axum.rs).

#### `bittorrent-` prefix

| Published on crates.io | Crate Name                     | Folder              | Change |
| ---------------------- | ------------------------------ | ------------------- | ------ |
| No                     | `bittorrent-http-tracker-core` | `http-tracker-core` | —      |
| No                     | `bittorrent-tracker-client`    | `tracker-client`    | —      |
| No                     | `bittorrent-udp-tracker-core`  | `udp-tracker-core`  | —      |

### `torrust/torrust-bittorrent` workspace

This section covers both the existing packages in that workspace (all pending a naming and
publishing decision) and the packages coming in from this tracker workspace.

#### Existing packages — renaming pending

All 9 existing packages use generic unprefixed working names and have `publish = false`. A
naming prefix must be chosen before any can be published (see README
[issue #64](https://github.com/torrust/torrust-bittorrent/issues/64)). No decision has been
made here; the table records the current state for analysis.

| Current crate name | Folder               | Proposed final name | Notes                                                                                  |
| ------------------ | -------------------- | ------------------- | -------------------------------------------------------------------------------------- |
| `bencode`          | `packages/bencode`   | TBD                 | Will be replaced by the newer `contrib/bencode` code from tracker (same crate lineage) |
| `dht`              | `packages/dht`       | TBD                 |                                                                                        |
| `disk`             | `packages/disk`      | TBD                 |                                                                                        |
| `handshake`        | `packages/handshake` | TBD                 |                                                                                        |
| `magnet`           | `packages/magnet`    | TBD                 |                                                                                        |
| `metainfo`         | `packages/metainfo`  | TBD                 |                                                                                        |
| `peer`             | `packages/peer`      | TBD                 |                                                                                        |
| `select`           | `packages/select`    | TBD                 |                                                                                        |
| `util`             | `packages/util`      | TBD                 | May be inlined into consumers rather than published independently                      |

> **Prefix options and implications**: a single naming policy is still required for the
> merged `bencode` lineage. Whether the final published name is `torrust-bencode` or
> `torrust-bittorrent-bencode` depends on the prefix decision for this workspace.

#### Incoming packages — extracted from tracker workspace

These packages are planned for extraction from this workspace into
[`torrust/torrust-bittorrent`](https://github.com/torrust/torrust-bittorrent).

| Final crate name                   | Extracted from                  | Blocked by                                                                                                                                                   | Notes                                                                   |
| ---------------------------------- | ------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------ | ----------------------------------------------------------------------- |
| TBD (replaces current `bencode`)   | `contrib/bencode`               | SI-12                                                                                                                                                        | Migrate newer tracker implementation and replace old `packages/bencode` |
| `bittorrent-peer-id`               | `packages/peer-id`              | —                                                                                                                                                            | No workspace deps; first in the `bittorrent-*` extraction sequence      |
| `bittorrent-udp-tracker-protocol`  | `packages/udp-protocol`         | `bittorrent-peer-id` publication                                                                                                                             |                                                                         |
| `bittorrent-http-tracker-protocol` | `packages/http-protocol`        | `bittorrent-udp-tracker-protocol` and `bittorrent-tracker-core` publication                                                                                  |                                                                         |
| `bittorrent-tracker-core`          | `packages/tracker-core`         | Deep dep chain; requires `torrust-tracker-events`, `torrust-metrics`, `swarm-coordination-registry`, `torrust-tracker-rest-api-client` to be published first |                                                                         |
| _(new package for `InfoHash`)_     | `torrust/bittorrent-primitives` | —                                                                                                                                                            | Migrate `InfoHash` here; then archive `torrust/bittorrent-primitives`   |

### Packages moving to standalone repositories

These packages are extracted to their own repositories under the Torrust organisation.

| Final crate name         | Extracted from            | Blocked by                                    | Notes                                                |
| ------------------------ | ------------------------- | --------------------------------------------- | ---------------------------------------------------- |
| `torrust-clock`          | `torrust-tracker-clock`   | SI-02 + SI-09 (rename first)                  | Rule P; published; 11 workspace consumers to migrate |
| `torrust-metrics`        | `torrust-tracker-metrics` | SI-08 (rename first)                          | 7 workspace consumers to migrate                     |
| `torrust-tracker-client` | `console/tracker-client`  | `bittorrent-*` publication (external to EPIC) | Standalone CLI tool; LGPL-3.0                        |

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

### Quick list

Status: TODO unless noted. `SI-XX` = recommended implementation sequence number.

- [ ] SI-01 — Establish baseline: dependency graph + README audit _(analysis; no blockers; informs all other subissues)_
- [x] SI-02 — Move `DurationSinceUnixEpoch` from `torrust-tracker-primitives` to `torrust-tracker-clock` _(Rule M; no hard blockers)_
- [ ] SI-03 — Define per-package default timeout constants and remove `DEFAULT_TIMEOUT` from `torrust-tracker-configuration` _(Rule M; no blockers)_
- [ ] SI-04 — [#1795](https://github.com/torrust/torrust-tracker/issues/1795) Move `AnnouncePolicy` from `torrust-tracker-configuration` to `torrust-tracker-primitives` _(Rule M; no blockers)_
- [x] SI-05 — [#1797](https://github.com/torrust/torrust-tracker/issues/1797) Create `torrust-net-primitives` and move `ServiceBinding` from `torrust-tracker-primitives` _(Rule M + new package; no blockers)_
- [x] SI-06 — [#1813](https://github.com/torrust/torrust-tracker/issues/1813) Resolve `bittorrent-tracker-core` ↔ `torrust-tracker-rest-api-client` layer violation _(Rule M; prerequisite for `bittorrent-tracker-core` extraction)_
- [x] SI-07 — [#1816](https://github.com/torrust/torrust-tracker/issues/1816) Align `torrust-` prefix: rename 7 tracker-specific packages to `torrust-tracker-` _(Rule U; no blockers)_
- [x] SI-08 — [#1819](https://github.com/torrust/torrust-tracker/issues/1819) Rename `torrust-tracker-metrics` to `torrust-metrics` _(Rule U; no blockers)_
- [x] SI-09 — [#1821](https://github.com/torrust/torrust-tracker/issues/1821) Rename `torrust-tracker-clock` to `torrust-clock` _(Rule P; no blockers)_
- [x] SI-10 — [#1823](https://github.com/torrust/torrust-tracker/issues/1823) Rename `torrust-tracker-located-error` to `torrust-located-error` _(Rule P; no blockers)_
- [ ] SI-11 — Update all package READMEs _(documentation; after SI-07–SI-10; before SI-12)_
- [ ] SI-12 — Migrate `contrib/bencode` back to `torrust/torrust-bittorrent`, replacing legacy `packages/bencode` _(Rule E; no blockers within this EPIC)_
- [ ] SI-13 — Extract `torrust-clock` to standalone repository _(Rule E; requires SI-02 + SI-09)_
- [ ] SI-14 — Extract `torrust-metrics` to standalone repository _(Rule E; requires SI-08)_
- [ ] SI-15 — Extract `torrust-tracker-client` to standalone repository _(Rule E; blocked by `bittorrent-*` publication — external to this EPIC)_

Details:

| SI    | Issue                                                                                                                                                                            | Local Spec                                                                                                                                                                                     | Status | Notes                                                                                           |
| ----- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------ | ----------------------------------------------------------------------------------------------- |
| SI-01 | #TBD — Establish baseline: dependency graph + README audit                                                                                                                       | [docs/issues/drafts/1669-01-establish-baseline-analysis.md](../../drafts/1669-01-establish-baseline-analysis.md)                                                                               | TODO   | No blockers; informs extraction decisions                                                       |
| SI-02 | [#1790](https://github.com/torrust/torrust-tracker/issues/1790) — Move `DurationSinceUnixEpoch` from `torrust-tracker-primitives` to `torrust-tracker-clock`                     | [docs/issues/open/1790-move-duration-since-unix-epoch-to-torrust-tracker-clock.md](../../open/1790-move-duration-since-unix-epoch-to-torrust-tracker-clock.md)                                 | DONE   | Rule M; no hard blockers; prerequisite for SI-13                                                |
| SI-03 | [#1793](https://github.com/torrust/torrust-tracker/issues/1793) — Define per-package default timeout constants and remove `DEFAULT_TIMEOUT` from `torrust-tracker-configuration` | [docs/issues/open/1793-1669-03-define-per-package-default-timeout-constants.md](../../open/1793-1669-03-define-per-package-default-timeout-constants.md)                                       | TODO   | Rule M; no blockers; SI-09 no longer depends on this                                            |
| SI-04 | [#1795](https://github.com/torrust/torrust-tracker/issues/1795) — Move `AnnouncePolicy` from `torrust-tracker-configuration` to `torrust-tracker-primitives`                     | [docs/issues/open/1795-1669-04-move-announce-policy-to-torrust-tracker-primitives.md](../../open/1795-1669-04-move-announce-policy-to-torrust-tracker-primitives.md)                           | TODO   | Rule M; fixes inverted dep (primitives → configuration); no blockers                            |
| SI-05 | [#1797](https://github.com/torrust/torrust-tracker/issues/1797) — Create `torrust-net-primitives` and move `ServiceBinding` from `torrust-tracker-primitives`                    | [docs/issues/open/1797-1669-05-create-torrust-net-primitives-and-move-service-binding.md](../../open/1797-1669-05-create-torrust-net-primitives-and-move-service-binding.md)                   | TODO   | Rule M + new package; generic networking type; breaks server-lib → tracker-primitives dep       |
| SI-06 | [#1813](https://github.com/torrust/torrust-tracker/issues/1813) — Resolve `bittorrent-tracker-core` ↔ `torrust-tracker-rest-api-client` layer violation                          | [docs/issues/closed/1813-1669-06-resolve-bittorrent-tracker-core-rest-api-layer-violation.md](../../closed/1813-1669-06-resolve-bittorrent-tracker-core-rest-api-layer-violation.md)           | DONE   | Rule M; stale unused dev dep removed in PR #1804; unblocks `bittorrent-tracker-core` extraction |
| SI-07 | [#1816](https://github.com/torrust/torrust-tracker/issues/1816) — Align `torrust-` prefix: rename 7 tracker-specific packages to `torrust-tracker-`                              | [docs/issues/open/1816-1669-07-align-torrust-prefix-rename-tracker-specific-packages.md](../../open/1816-1669-07-align-torrust-prefix-rename-tracker-specific-packages.md)                     | DONE   | Rule U; none of the 7 are published; pure workspace rename; no blockers                         |
| SI-08 | [#1819](https://github.com/torrust/torrust-tracker/issues/1819) — Rename `torrust-tracker-metrics` to `torrust-metrics`                                                          | [docs/issues/open/1819-1669-08-rename-torrust-tracker-metrics-to-torrust-metrics.md](../../open/1819-1669-08-rename-torrust-tracker-metrics-to-torrust-metrics.md)                             | DONE   | Rule U; not yet published; no blockers; prerequisite for SI-14                                  |
| SI-09 | [#1821](https://github.com/torrust/torrust-tracker/issues/1821) — Rename `torrust-tracker-clock` to `torrust-clock`                                                              | [docs/issues/open/1821-1669-09-rename-torrust-tracker-clock-to-torrust-clock.md](../../open/1821-1669-09-rename-torrust-tracker-clock-to-torrust-clock.md)                                     | DONE   | Rule P; published on crates.io; no blockers; prerequisite for SI-13                             |
| SI-10 | [#1823](https://github.com/torrust/torrust-tracker/issues/1823) — Rename `torrust-tracker-located-error` to `torrust-located-error`                                              | [docs/issues/closed/1823-1669-10-rename-torrust-tracker-located-error-to-torrust-located-error.md](../../closed/1823-1669-10-rename-torrust-tracker-located-error-to-torrust-located-error.md) | DONE   | Rule P; completed                                                                               |
| SI-11 | #TBD — Update all package READMEs                                                                                                                                                | [docs/issues/drafts/1669-11-update-all-package-readmes.md](../../drafts/1669-11-update-all-package-readmes.md)                                                                                 | TODO   | Documentation; requires SI-07–SI-10; before SI-12                                               |
| SI-12 | #TBD — Migrate `contrib/bencode` to `torrust/torrust-bittorrent` and replace legacy `packages/bencode`                                                                           | [docs/issues/drafts/1669-12-extract-torrust-tracker-contrib-bencode-to-torrust-bencode.md](../../drafts/1669-12-extract-torrust-tracker-contrib-bencode-to-torrust-bencode.md)                 | TODO   | Rule E; replaces old `torrust-bittorrent` implementation with newer tracker lineage             |
| SI-13 | #TBD — Extract `torrust-clock` to standalone repository                                                                                                                          | [docs/issues/drafts/1669-13-extract-torrust-clock-to-standalone-repo.md](../../drafts/1669-13-extract-torrust-clock-to-standalone-repo.md)                                                     | TODO   | Rule E; requires SI-02 + SI-09; 11 workspace consumers to migrate                               |
| SI-14 | #TBD — Extract `torrust-metrics` to standalone repository                                                                                                                        | [docs/issues/drafts/1669-14-extract-torrust-metrics-to-standalone-repo.md](../../drafts/1669-14-extract-torrust-metrics-to-standalone-repo.md)                                                 | TODO   | Rule E; requires SI-08; 7 workspace consumers to migrate                                        |
| SI-15 | #TBD — Extract `torrust-tracker-client` to standalone repository                                                                                                                 | [docs/issues/drafts/1669-15-extract-torrust-tracker-client-to-standalone-repo.md](../../drafts/1669-15-extract-torrust-tracker-client-to-standalone-repo.md)                                   | TODO   | Rule E; blocked by `bittorrent-udp-tracker-protocol` publication (external to this EPIC)        |

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

- **`bittorrent-*` protocol crates** (`bittorrent-http-tracker-protocol`,
  `bittorrent-udp-tracker-protocol`, `bittorrent-peer-id`) — implement BEP specs with no
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

### Extraction ordering: crates.io publication constraints

When a package is extracted to a standalone repository, all its **runtime** workspace
dependencies must already be published on crates.io (path deps become version deps after
extraction). The table below analyses every current or near-term extraction candidate
against this constraint (verified May 2026).

| Package                                         | Crates.io status | Unpublished runtime workspace deps                                                                                                                      | Can be published independently? | Ordering constraint                                                                                                                     |
| ----------------------------------------------- | ---------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------- |
| `torrust-tracker-contrib-bencode`               | Yes              | None                                                                                                                                                    | ✅ Now                          | SI-12 can migrate it into `torrust/torrust-bittorrent` and replace legacy `packages/bencode`                                            |
| `bittorrent-peer-id`                            | No               | None                                                                                                                                                    | ✅ Now                          | No spec yet; can be extracted first in the `bittorrent-*` sequence                                                                      |
| `torrust-located-error`                         | Yes              | None                                                                                                                                                    | ✅ Already published            | No extraction spec yet                                                                                                                  |
| `torrust-tracker-clock` (→ `torrust-clock`)     | Yes              | None (✅ `torrust-tracker-primitives` dep removed by SI-02 #1790)                                                                                       | ✅ After rename                 | See [extract clock subissue](../../drafts/1669-13-extract-torrust-clock-to-standalone-repo.md)                                          |
| `torrust-tracker-metrics` (→ `torrust-metrics`) | No               | `torrust-tracker-clock` (published ✅; was `torrust-tracker-primitives` — removed by SI-02 #1790)                                                       | ✅ After rename                 | See [extract metrics subissue](../../drafts/1669-14-extract-torrust-metrics-to-standalone-repo.md)                                      |
| `bittorrent-udp-tracker-protocol`               | No               | `bittorrent-peer-id` (not published)                                                                                                                    | ❌                              | After `bittorrent-peer-id`                                                                                                              |
| `bittorrent-tracker-core`                       | No               | `torrust-tracker-events`, `torrust-tracker-metrics`, `torrust-tracker-swarm-coordination-registry`, `torrust-tracker-rest-api-client` (all unpublished) | ❌ Very deep chain              | After all four above; also has `torrust-tracker-rest-api-client` as a runtime dep — a layer violation worth resolving before extraction |
| `bittorrent-http-tracker-protocol`              | No               | `bittorrent-udp-tracker-protocol`, `bittorrent-tracker-core` (both unpublished)                                                                         | ❌                              | After `bittorrent-udp-tracker-protocol` and `bittorrent-tracker-core`                                                                   |

**Practical extraction order for `bittorrent-*` crates** (once decided):

1. `bittorrent-peer-id` — no workspace deps; extract first.
2. `bittorrent-udp-tracker-protocol` — only blocked by #1.
3. `bittorrent-tracker-core` — needs the four unpublished deps above + clock rename; complex
   chain; the layer violation (`torrust-tracker-rest-api-client` runtime dep) should be
   resolved before or during this step.
4. `bittorrent-http-tracker-protocol` — needs #2 and #3 done.

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
