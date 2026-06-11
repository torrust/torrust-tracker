---
doc-type: epic
issue-type: task
status: planned
priority: p1
github-issue: 1669
spec-path: docs/issues/open/1669-overhaul-packages/EPIC.md
epic-owner: josecelano
last-updated-utc: 2026-06-11 22:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/packages.md
    - docs/issues/open/1669-overhaul-packages/
    - docs/issues/open/1835-1669-14-decouple-http-protocol-from-tracker-primitives.md
    - docs/issues/open/1889-1669-21-migrate-from-bittorrent-primitives-to-torrust-info-hash.md
    - docs/adrs/20260527175600_keep_protocol_and_domain_types_decoupled.md
    - docs/adrs/index.md
    - docs/issues/open/1669-overhaul-packages/DECISIONS.md
    - AGENTS.md
    - packages/AGENTS.md
    - docs/media/packages/dependencies-workspace-packages.md
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
- **Some packages are clearly generic and reusable**: `bencode`, `clock`, `metrics`,
  `located-error`, `net-primitives`, and several utility crates have no tracker-specific
  logic and would be more useful to the wider community as standalone crates in their own
  repositories. Keeping them here adds noise to the workspace and makes their independent
  evolution harder. Protocol packages (`udp-protocol`, `http-protocol`) also have high
  reuse potential but are intentionally kept in the tracker workspace — see the naming
  and ownership policy in the Decision Log (DEC-14).
- **Versioning policy is implicit**: all packages share the workspace version; packages
  extracted to separate repos will need their own release cadence.
- **Only 6 of originally 27 packages were published on crates.io** (as of May 2026);
  the remaining 21 packages were unpublished, in particular every `bittorrent-*` crate.
  As of June 2026, 4 more packages have been published from standalone repositories
  (`torrust-clock`, `torrust-located-error`, `torrust-metrics`, `torrust-net-primitives`),
  bringing the total published across the organisation to 10. Publishing them in-workspace
  conflicted with giving them independent versions; extraction resolved this tension.

The approach is not all-or-nothing. Each small extraction or structural improvement is a
self-contained win. Re-evaluation happens naturally after each change, or when the package
landscape shifts (new packages, splits, significant growth).

## Package Inventory

The workspace currently contains **23 packages** (including the root `torrust-tracker` crate) across three crate-name prefixes.
"Published" means a crate with that name exists on crates.io (verified June 2026).

Packages that have been extracted to standalone repositories are listed as `(extracted)`.

### `torrust-` prefix (non-`torrust-tracker-`)

| Published on crates.io | Crate Name               | Folder       |
| ---------------------- | ------------------------ | ------------ |
| Yes                    | `torrust-clock`          | (extracted)  |
| Yes                    | `torrust-located-error`  | (extracted)  |
| Yes                    | `torrust-metrics`        | (extracted)  |
| Yes                    | `torrust-net-primitives` | (extracted)  |
| No                     | `torrust-server-lib`     | `server-lib` |

### `torrust-tracker-` prefix

| Published on crates.io | Crate Name                                        | Folder                            |
| ---------------------- | ------------------------------------------------- | --------------------------------- |
| No                     | `torrust-tracker-axum-health-check-api-server`    | `axum-health-check-api-server`    |
| No                     | `torrust-tracker-axum-http-server`                | `axum-http-server`                |
| No                     | `torrust-tracker-axum-rest-api-server`            | `axum-rest-api-server`            |
| No                     | `torrust-tracker-axum-server`                     | `axum-server`                     |
| No                     | `torrust-tracker-client`                          | `console/tracker-client`          |
| Yes                    | `torrust-tracker-configuration`                   | `configuration`                   |
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

**Observation**: 10 packages across the organisation (including extracted) are published on crates.io: `torrust-bencode` 3.0.0, `torrust-clock` 3.0.0, `torrust-info-hash` 0.2.0, `torrust-located-error` 3.0.0, `torrust-metrics` 0.1.0, `torrust-net-primitives` 0.1.0, `torrust-peer-id` 0.1.0, `torrust-tracker-configuration`, `torrust-tracker-primitives`, and `torrust-tracker-test-helpers`. Of those still in this workspace, 3 are published. Every `torrust-axum-` crate is
unpublished. This confirms issue #1659's note that "many new crates have not been published
yet after we refactored the packages."

### External repositories in scope

This EPIC covers coordination with the following external repositories. Packages extracted
from this workspace may land in one of these rather than in a brand-new standalone repository.

#### `torrust/torrust-bittorrent` — <https://github.com/torrust/torrust-bittorrent>

A Cargo workspace for BitTorrent protocol implementations (forked from
[bip-rs](https://github.com/GGist/bip-rs), maintained by the Torrust organisation). It has
been restructured with `torrust-` prefixed crate names. Packages migrated from
`torrust/torrust-tracker` have been published on crates.io.

**Packages** (verified June 2026):

| Published on crates.io | Crate Name          | Folder               | Internal workspace deps                                                 | Description                                                             |
| ---------------------- | ------------------- | -------------------- | ----------------------------------------------------------------------- | ----------------------------------------------------------------------- |
| Yes                    | `torrust-bencode`   | `packages/bencode`   | —                                                                       | Efficient decoding and encoding for bencode                             |
| No                     | `torrust-dht`       | `packages/dht`       | `torrust-bencode`, `torrust-handshake`, `torrust-util`                  | Bittorrent Mainline DHT implementation                                  |
| No                     | `torrust-disk`      | `packages/disk`      | `torrust-metainfo`, `torrust-util`                                      | Torrent piece filesystem interface                                      |
| No                     | `torrust-handshake` | `packages/handshake` | `torrust-util`                                                          | BitTorrent handshake trait and implementation                           |
| Yes                    | `torrust-info-hash` | `packages/info-hash` | —                                                                       | BitTorrent InfoHash v1 type (migrated from tracker SI-21)               |
| No                     | `torrust-magnet`    | `packages/magnet`    | `torrust-util`                                                          | Parsing and constructing magnet links                                   |
| No                     | `torrust-metainfo`  | `packages/metainfo`  | `torrust-bencode`, `torrust-util`                                       | Parsing and building `.torrent` metainfo files                          |
| No                     | `torrust-peer`      | `packages/peer`      | `torrust-bencode`, `torrust-handshake`, `torrust-util`                  | Peer wire protocol communication                                        |
| Yes                    | `torrust-peer-id`   | `packages/peer-id`   | —                                                                       | Peer ID parsing and client identification (migrated from tracker SI-19) |
| No                     | `torrust-select`    | `packages/select`    | `torrust-handshake`, `torrust-metainfo`, `torrust-peer`, `torrust-util` | Piece selection algorithm                                               |
| No                     | `torrust-util`      | `packages/util`      | —                                                                       | Shared utilities used across packages                                   |

**Observation**: the workspace has been restructured with `torrust-` prefixed crate names.
Of the 11 packages, 3 have been published on crates.io (`torrust-bencode` 3.0.0,
`torrust-peer-id` 0.1.0, `torrust-info-hash` 0.2.0). The remaining 8 packages
(`torrust-dht`, `torrust-disk`, `torrust-handshake`, `torrust-magnet`, `torrust-metainfo`,
`torrust-peer`, `torrust-select`, `torrust-util`) are not yet published.

**Role in this EPIC**: already received 3 packages migrated from `torrust/torrust-tracker`:
`torrust-bencode` (SI-16), `torrust-peer-id` (SI-19), and `torrust-info-hash` (SI-21).
The protocol and tracker-core crates remain in `torrust/torrust-tracker` for now;
the move will be reconsidered after dependency cleanup.

#### `torrust/bittorrent-primitives` — <https://github.com/torrust/bittorrent-primitives>

A single-package repository containing one crate (`bittorrent-primitives` v0.3.0) whose
sole public type is `InfoHash`. Originally created as the home for foundational BitTorrent
primitive types, it has not grown beyond that single type.

**Packages** (verified June 2026):

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
| Yes                    | `torrust-tracker-primitives`[^fu1]                | `primitives`                      | —                                  | —                              |
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

[^fu1]: FU-1 (#1859): `TrackerPolicy`, `TORRENT_PEERS_LIMIT`, and `PrivateMode` were moved here from `torrust-tracker-configuration` (completed in #1859, PR #1865). See [DECISIONS.md](./DECISIONS.md) DEC-07.

### `torrust/torrust-bittorrent` workspace

All packages now live in this workspace as `torrust-` prefixed crates. The SI-16 (bencode),
SI-19 (peer-id), and SI-21 (info-hash) migrations are complete and the incoming packages
have been merged into the existing set.

| Package status | Final crate name    | Folder               | Source / change             | Notes |
| -------------- | ------------------- | -------------------- | --------------------------- | ----- |
| Existing       | `torrust-bencode`   | `packages/bencode`   | Rename in destination       | [1]   |
| Existing       | `torrust-dht`       | `packages/dht`       | Rename in destination       |       |
| Existing       | `torrust-disk`      | `packages/disk`      | Rename in destination       |       |
| Existing       | `torrust-handshake` | `packages/handshake` | Rename in destination       |       |
| Existing       | `torrust-info-hash` | `packages/info-hash` | Migrated from tracker SI-21 | [4]   |
| Existing       | `torrust-magnet`    | `packages/magnet`    | Rename in destination       |       |
| Existing       | `torrust-metainfo`  | `packages/metainfo`  | Rename in destination       |       |
| Existing       | `torrust-peer`      | `packages/peer`      | Rename in destination       |       |
| Existing       | `torrust-peer-id`   | `packages/peer-id`   | Migrated from tracker SI-19 | [3]   |
| Existing       | `torrust-select`    | `packages/select`    | Rename in destination       |       |
| Existing       | `torrust-util`      | `packages/util`      | Rename in destination       | [2]   |

Notes:

1. Renamed from original `bencode` and replaced by the newer `contrib/bencode` code from tracker via SI-16 (#1881). Published on crates.io as `torrust-bencode` 3.0.0.
2. May be inlined into consumers rather than published independently.
3. Migrated from `packages/peer-id` in the tracker workspace via SI-19 (#1884). Published on crates.io as `torrust-peer-id` 0.1.0.
4. Migrated from `bittorrent-primitives` v0.2.0 via SI-21 (#1889). Published on crates.io as `torrust-info-hash` 0.2.0. The old `torrust/bittorrent-primitives` repository can be archived.

The following crates remain in `torrust/torrust-tracker` (and are expected to stay):

- `torrust-tracker-udp-tracker-protocol`
- `torrust-tracker-http-tracker-protocol`
- `torrust-tracker-core`
- `torrust-tracker-udp-tracker-core`
- `torrust-tracker-http-tracker-core`

These packages are **owned by the tracker workspace** per the naming and ownership
policy (DEC-14). They are not planned for migration to `torrust/torrust-bittorrent`
even though some (particularly the protocol crates) have high reuse potential.
The parent repository groups packages by concern and ownership, not by estimated
reusability.

> **Naming policy**: all Torrust organisation packages use the `torrust-` prefix. The
> `bittorrent-` prefix is not used; it is redundant since most code in this organisation
> relates to BitTorrent. Tracker-owned packages use the `torrust-tracker-` prefix.
> Organisation-level shared crates use `torrust-` alone. Package location in a repository
> is determined by grouping by concern (ownership of the workspace), not by estimated
> reusability. See DEC-14 in the Decision Log for the full rationale.

### Packages moving to standalone repositories

These packages are extracted to their own repositories under the Torrust organisation.

| Final crate name         | Extracted from                  | Blocked by                                    | Notes                                                                                                                                                          |
| ------------------------ | ------------------------------- | --------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `torrust-clock`          | `torrust-tracker-clock`         | SI-02 + SI-09 (rename first)                  | **DONE** — published v3.0.0; standalone repo at [torrust/torrust-clock](https://github.com/torrust/torrust-clock); all 11 consumers migrated                   |
| `torrust-located-error`  | `torrust-tracker-located-error` | SI-10 (rename first)                          | **DONE** — published v3.0.0; standalone repo at [torrust/torrust-located-error](https://github.com/torrust/torrust-located-error); all 5 consumers migrated    |
| `torrust-metrics`        | `torrust-tracker-metrics`       | SI-08 (rename first)                          | **DONE** — published v0.1.0; all 7 consumers migrated                                                                                                          |
| `torrust-net-primitives` | `torrust-net-primitives`        | Extraction issue TBD (SI-20)                  | **DONE** — published v0.1.0; standalone repo at [torrust/torrust-net-primitives](https://github.com/torrust/torrust-net-primitives); all 10 consumers migrated |
| `torrust-server-lib`     | `torrust-server-lib`            | Extraction issue TBD                          | Generic server utility crate; standalone extraction candidate                                                                                                  |
| `torrust-tracker-client` | `console/tracker-client`        | `bittorrent-*` publication (external to EPIC) | Standalone CLI tool; LGPL-3.0                                                                                                                                  |

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
  - `torrust-info-hash`
  - `torrust-net-primitives`
  - `torrust-server-lib`
  - `torrust-tracker-axum-server`
  - `torrust-tracker-configuration`
  - `torrust-tracker-core`
  - `torrust-tracker-http-tracker-core`
  - `torrust-tracker-http-tracker-protocol`
  - `torrust-tracker-primitives`
  - `torrust-tracker-swarm-coordination-registry`
  - `torrust-tracker-udp-tracker-protocol`
- `torrust-tracker-axum-rest-api-server`
  - `torrust-clock`
  - `torrust-info-hash`
  - `torrust-metrics`
  - `torrust-net-primitives`
  - `torrust-server-lib`
  - `torrust-tracker-axum-server`
  - `torrust-tracker-configuration`
  - `torrust-tracker-core`
  - `torrust-tracker-http-tracker-core`
  - `torrust-tracker-primitives`
  - `torrust-tracker-rest-api-client`
  - `torrust-tracker-rest-api-core`
  - `torrust-tracker-swarm-coordination-registry`
  - `torrust-tracker-udp-server`
  - `torrust-tracker-udp-tracker-core`
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
  - `torrust-info-hash`
  - `torrust-metrics`
  - `torrust-net-primitives`
  - `torrust-tracker-configuration`
  - `torrust-tracker-core`
  - `torrust-tracker-events`
  - `torrust-tracker-http-tracker-protocol`
  - `torrust-tracker-primitives`
  - `torrust-tracker-swarm-coordination-registry`
- `torrust-tracker-http-tracker-protocol`
  - `torrust-bencode`
  - `torrust-clock`
  - `torrust-info-hash`
  - `torrust-located-error`
  - `torrust-peer-id`
- `torrust-tracker-primitives`
  - `torrust-clock`
  - `torrust-info-hash`
  - `torrust-net-primitives`
  - `torrust-peer-id`
- `torrust-tracker-rest-api-client`
  - None
- `torrust-tracker-rest-api-core`
  - `torrust-metrics`
  - `torrust-tracker-configuration`
  - `torrust-tracker-core`
  - `torrust-tracker-http-tracker-core`
  - `torrust-tracker-primitives`
  - `torrust-tracker-swarm-coordination-registry`
  - `torrust-tracker-udp-server`
  - `torrust-tracker-udp-tracker-core`
- `torrust-tracker-swarm-coordination-registry`
  - `torrust-clock`
  - `torrust-info-hash`
  - `torrust-metrics`
  - `torrust-tracker-configuration`
  - `torrust-tracker-events`
  - `torrust-tracker-primitives`
- `torrust-tracker-core`
  - `torrust-clock`
  - `torrust-info-hash`
  - `torrust-located-error`
  - `torrust-metrics`
  - `torrust-tracker-configuration`
  - `torrust-tracker-events`
  - `torrust-tracker-primitives`
  - `torrust-tracker-swarm-coordination-registry`
- `torrust-tracker-test-helpers`
  - `torrust-tracker-configuration`
- `torrust-tracker-torrent-repository-benchmarking`
  - `torrust-clock`
  - `torrust-info-hash`
  - `torrust-tracker-configuration`
  - `torrust-tracker-primitives`
- `torrust-tracker-client` (`packages/tracker-client`)
  - `torrust-info-hash`
  - `torrust-located-error`
  - `torrust-net-primitives`
  - `torrust-tracker-primitives`
  - `torrust-tracker-udp-tracker-protocol`
- `torrust-tracker-client` (`console/tracker-client`)
  - `torrust-info-hash`
  - `torrust-tracker-client` (`torrust-tracker-client-lib`)
  - `torrust-tracker-udp-tracker-protocol`
- `torrust-tracker-udp-tracker-protocol`
  - `torrust-peer-id`
- `torrust-tracker-udp-tracker-core`
  - `torrust-clock`
  - `torrust-info-hash`
  - `torrust-metrics`
  - `torrust-net-primitives`
  - `torrust-tracker-configuration`
  - `torrust-tracker-core`
  - `torrust-tracker-events`
  - `torrust-tracker-primitives`
  - `torrust-tracker-swarm-coordination-registry`
  - `torrust-tracker-udp-tracker-protocol`
- `torrust-tracker-udp-server`
  - `torrust-clock`
  - `torrust-info-hash`
  - `torrust-metrics`
  - `torrust-net-primitives`
  - `torrust-server-lib`
  - `torrust-tracker-client` (`torrust-tracker-client-lib`)
  - `torrust-tracker-configuration`
  - `torrust-tracker-core`
  - `torrust-tracker-events`
  - `torrust-tracker-primitives`
  - `torrust-tracker-swarm-coordination-registry`
  - `torrust-tracker-udp-tracker-core`
  - `torrust-tracker-udp-tracker-protocol`

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
- `torrust-info-hash`
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
- Keep the dependency diagram at `docs/media/packages/dependencies-workspace-packages.md`
  in sync with the actual Cargo workspace by regenerating it after every package move,
  rename, or dependency change.
- **Documentation audit**: before closing any subissue, verify that:
  1. `docs/packages.md` lists every actual package (run `cargo metadata --no-deps` and
     cross-check against the File Listing and Package Catalog tables).
  2. `packages/AGENTS.md` matches, with no ghost packages that have been removed or renamed.
  3. The extracted packages table in both docs reflects the current set of extracted crates.
  4. The dependency diagram is consistent with the actual `Cargo.toml` dependencies.
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

Current known smells to prioritize under these rules:

- ~~`http-protocol` depending on `udp-protocol`~~ — **fixed** by SI-13 (#1834).
- `rest-api-core` depending on `udp-server` (core → server violation). Tracked in
  [`docs/issues/drafts/1669-decouple-rest-api-core-from-udp-internals.md`](../../drafts/1669-decouple-rest-api-core-from-udp-internals.md).

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
- [ ] [#1859](https://github.com/torrust/torrust-tracker/issues/1859) Move `TrackerPolicy`, `TORRENT_PEERS_LIMIT`, and `PrivateMode` to `torrust-tracker-primitives` _(Rule M; FU-1 from #1856)_
- [ ] [#1860](https://github.com/torrust/torrust-tracker/issues/1860) Evaluate moving `TslConfig` from `torrust-tracker-configuration` into `torrust-tracker-axum-server` _(Rule M candidate; FU-2 from #1856)_
- [ ] [#1861](https://github.com/torrust/torrust-tracker/issues/1861) Revisit `EnvContainer::initialize` to accept narrower config slices _(design/analysis; FU-3 from #1856)_

#### 3. Numbered Subissues (GitHub Issues Open)

- [x] [#1834](https://github.com/torrust/torrust-tracker/issues/1834) SI-13: Decouple `http-protocol` from `udp-protocol` _(Rule M; remove cross-protocol dependency edge)_
- [x] [#1835](https://github.com/torrust/torrust-tracker/issues/1835) SI-14: Decouple `http-protocol` from `torrust-tracker-primitives` _(Rule M; remove protocol -> domain coupling as step 2)_
- [x] [#1882](https://github.com/torrust/torrust-tracker/issues/1882) SI-18: Extract `torrust-metrics` to standalone repository _(Rule E; requires completed metrics rename work)_
- [x] [#1884](https://github.com/torrust/torrust-tracker/issues/1884) SI-19: Move `bittorrent-peer-id` to `torrust/torrust-bittorrent` as `torrust-peer-id` _(Rule E; no workspace deps; first `bittorrent-*` extraction)_
- [x] [#1885](https://github.com/torrust/torrust-tracker/issues/1885) SI-20: Extract `torrust-net-primitives` to standalone repository _(Rule E; no workspace deps; no prerequisites)_ — **DONE**
- [x] [#1894](https://github.com/torrust/torrust-tracker/issues/1894) SI-22: Extract `torrust-located-error` to standalone repository _(Rule E; no workspace deps; requires completed rename SI-10 #1823)_ — **DONE**

#### 4. Other Tracked Items (Drafts and Promoted Issues)

- [ ] Establish baseline: dependency graph + README audit _(analysis; no blockers; informs all other subissues)_
- [ ] Update all package READMEs _(documentation; after completed rename work; before extractions)_
- [x] [#1881](https://github.com/torrust/torrust-tracker/issues/1881) SI-16: Migrate `contrib/bencode` to `torrust/torrust-bittorrent` as `torrust-bencode` _(Rule E; no blockers within this EPIC)_
- [x] Extract `torrust-clock` to standalone repository — [#1879](https://github.com/torrust/torrust-tracker/issues/1879) _(Rule E; requires completed clock rename and type move work)_
- [x] Extract `torrust-located-error` to standalone repository — [#1894](https://github.com/torrust/torrust-tracker/issues/1894) _(Rule E; requires completed rename SI-10 #1823)_ — **DONE**
- [x] Extract `torrust-metrics` to standalone repository — [#1882](https://github.com/torrust/torrust-tracker/issues/1882) _(Rule E; requires completed metrics rename work)_ — **DONE**
- [x] Move `bittorrent-peer-id` to `torrust/torrust-bittorrent` as `torrust-peer-id` — [#1884](https://github.com/torrust/torrust-tracker/issues/1884) _(Rule E; no workspace deps; first `bittorrent-*` extraction)_ — **DONE**
- [x] Extract `torrust-net-primitives` to standalone repository — [#1885](https://github.com/torrust/torrust-tracker/issues/1885) _(Rule E; no workspace deps; no prerequisites)_ — **DONE**
- [ ] Extract `torrust-tracker-client` to standalone repository _(Rule E; blocked by `bittorrent-*` publication - external to this EPIC)_
- [ ] Remove redundant `-tracker-` from HTTP and UDP crate names _(Rule U; rename 4 unpublished packages to match DEC-15 folder convention)_
- [ ] Configure `cargo deny` for workspace layer boundary enforcement _(tooling; create deny.toml with bans for all forbidden edges)_
- [ ] Define package versioning strategy (linked vs independent SemVer evolution) _(policy; no blockers; informs extraction and publication cadence)_
- [ ] Define REST API contract-first package architecture _(policy reminder; PoC-first and dedicated API EPIC before migration/extraction)_
- [x] [#1856](https://github.com/torrust/torrust-tracker/issues/1856) Analyse configuration package coupling and evaluate splitting strategies _(research; no blockers; informs "build-your-own tracker" goal and versioning strategy)_

Details:

| Item                       | Issue                                                                                                                                                                            | Local Spec                                                                                                                                                                                     | Status | Notes                                                                                                                                         |
| -------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------ | --------------------------------------------------------------------------------------------------------------------------------------------- |
| Baseline analysis          | #TBD — Establish baseline: dependency graph + README audit                                                                                                                       | [docs/issues/drafts/1669-01-establish-baseline-analysis.md](../../drafts/1669-01-establish-baseline-analysis.md)                                                                               | TODO   | No blockers; informs extraction decisions                                                                                                     |
| Duration move              | [#1790](https://github.com/torrust/torrust-tracker/issues/1790) — Move `DurationSinceUnixEpoch` from `torrust-tracker-primitives` to `torrust-tracker-clock`                     | [docs/issues/open/1790-move-duration-since-unix-epoch-to-torrust-tracker-clock.md](../../open/1790-move-duration-since-unix-epoch-to-torrust-tracker-clock.md)                                 | DONE   | Rule M; no hard blockers; prerequisite for clock extraction                                                                                   |
| Timeout constants          | [#1793](https://github.com/torrust/torrust-tracker/issues/1793) — Define per-package default timeout constants and remove `DEFAULT_TIMEOUT` from `torrust-tracker-configuration` | [docs/issues/open/1793-1669-03-define-per-package-default-timeout-constants.md](../../open/1793-1669-03-define-per-package-default-timeout-constants.md)                                       | DONE   | Rule M; completed                                                                                                                             |
| Announce policy move       | [#1795](https://github.com/torrust/torrust-tracker/issues/1795) — Move `AnnouncePolicy` from `torrust-tracker-configuration` to `torrust-tracker-primitives`                     | [docs/issues/open/1795-1669-04-move-announce-policy-to-torrust-tracker-primitives.md](../../open/1795-1669-04-move-announce-policy-to-torrust-tracker-primitives.md)                           | DONE   | Rule M; completed                                                                                                                             |
| Net primitives split       | [#1797](https://github.com/torrust/torrust-tracker/issues/1797) — Create `torrust-net-primitives` and move `ServiceBinding` from `torrust-tracker-primitives`                    | [docs/issues/closed/1797-1669-05-create-torrust-net-primitives-and-move-service-binding.md](../../closed/1797-1669-05-create-torrust-net-primitives-and-move-service-binding.md)               | DONE   | Rule M + new package; generic networking type; completed                                                                                      |
| Layer violation fix        | [#1813](https://github.com/torrust/torrust-tracker/issues/1813) — Resolve `torrust-tracker-core` ↔ `torrust-tracker-rest-api-client` layer violation                             | [docs/issues/closed/1813-1669-06-resolve-torrust-tracker-core-rest-api-layer-violation.md](../../closed/1813-1669-06-resolve-torrust-tracker-core-rest-api-layer-violation.md)                 | DONE   | Rule M; stale unused dev dep removed in PR #1804; unblocks `torrust-tracker-core` extraction                                                  |
| Prefix alignment           | [#1816](https://github.com/torrust/torrust-tracker/issues/1816) — Align `torrust-` prefix: rename 7 tracker-specific packages to `torrust-tracker-`                              | [docs/issues/open/1816-1669-07-align-torrust-prefix-rename-tracker-specific-packages.md](../../open/1816-1669-07-align-torrust-prefix-rename-tracker-specific-packages.md)                     | DONE   | Rule U; none of the 7 are published; pure workspace rename; no blockers                                                                       |
| Metrics rename             | [#1819](https://github.com/torrust/torrust-tracker/issues/1819) — Rename `torrust-tracker-metrics` to `torrust-metrics`                                                          | [docs/issues/open/1819-1669-08-rename-torrust-tracker-metrics-to-torrust-metrics.md](../../open/1819-1669-08-rename-torrust-tracker-metrics-to-torrust-metrics.md)                             | DONE   | Rule U; not yet published; no blockers; prerequisite for metrics extraction                                                                   |
| Clock rename               | [#1821](https://github.com/torrust/torrust-tracker/issues/1821) — Rename `torrust-tracker-clock` to `torrust-clock`                                                              | [docs/issues/open/1821-1669-09-rename-torrust-tracker-clock-to-torrust-clock.md](../../open/1821-1669-09-rename-torrust-tracker-clock-to-torrust-clock.md)                                     | DONE   | Rule P; published on crates.io; no blockers; prerequisite for clock extraction                                                                |
| Located error rename       | [#1823](https://github.com/torrust/torrust-tracker/issues/1823) — Rename `torrust-tracker-located-error` to `torrust-located-error`                                              | [docs/issues/closed/1823-1669-10-rename-torrust-tracker-located-error-to-torrust-located-error.md](../../closed/1823-1669-10-rename-torrust-tracker-located-error-to-torrust-located-error.md) | DONE   | Rule P; completed                                                                                                                             |
| README refresh             | #TBD — Update all package READMEs                                                                                                                                                | [docs/issues/drafts/1669-update-all-package-readmes.md](../../drafts/1669-update-all-package-readmes.md)                                                                                       | TODO   | Documentation; requires completed rename work; before extraction work                                                                         |
| Bencode migration          | [#1881](https://github.com/torrust/torrust-tracker/issues/1881) SI-16: Migrate `contrib/bencode` to `torrust/torrust-bittorrent` as `torrust-bencode`                            | [docs/issues/closed/1881-1669-16-migrate-contrib-bencode-to-torrust-bittorrent/ISSUE.md](../../closed/1881-1669-16-migrate-contrib-bencode-to-torrust-bittorrent/ISSUE.md)                     | DONE   | Rule E; torrust-bencode 3.0.0 published; contrib/bencode removed from tracker workspace                                                       |
| Peer-ID move               | [#1884](https://github.com/torrust/torrust-tracker/issues/1884) — Move `bittorrent-peer-id` to `torrust/torrust-bittorrent` as `torrust-peer-id`                                 | [docs/issues/open/1884-1669-19-move-bittorrent-peer-id-to-torrust-bittorrent.md](../../open/1884-1669-19-move-bittorrent-peer-id-to-torrust-bittorrent.md)                                     | DONE   | Rule E; published as torrust-peer-id 0.1.0 on crates.io; 3 tracker consumers migrated; packages/peer-id removed                               |
| Clock extraction           | [#1879](https://github.com/torrust/torrust-tracker/issues/1879) — Extract `torrust-clock` to standalone repository                                                               | [docs/issues/closed/1879-1669-17-extract-torrust-clock-to-standalone-repo.md](../../closed/1879-1669-17-extract-torrust-clock-to-standalone-repo.md)                                           | DONE   | Rule E; torrust-clock v3.0.0 published; 13 consumers migrated; packages/clock removed                                                         |
| Metrics extraction         | [#1882](https://github.com/torrust/torrust-tracker/issues/1882) — Extract `torrust-metrics` to standalone repository                                                             | [docs/issues/open/1882-1669-18-extract-torrust-metrics-to-standalone-repo.md](../../open/1882-1669-18-extract-torrust-metrics-to-standalone-repo.md)                                           | DONE   | Rule E; torrust-metrics v0.1.0 published; 7 consumers migrated; packages/metrics removed                                                      |
| Located error extraction   | [#1894](https://github.com/torrust/torrust-tracker/issues/1894) — Extract `torrust-located-error` to standalone repository                                                       | [docs/issues/open/1894-1669-22-extract-torrust-located-error-to-standalone-repo.md](../../open/1894-1669-22-extract-torrust-located-error-to-standalone-repo.md)                               | DONE   | Rule E; no workspace deps; requires completed rename SI-10 (#1823); 5 consumers migrated; crate v3.0.0 published                              |
| Net-primitives extraction  | [#1885](https://github.com/torrust/torrust-tracker/issues/1885) — Extract `torrust-net-primitives` to standalone repository                                                      | [docs/issues/open/1885-1669-20-extract-torrust-net-primitives-to-standalone-repo.md](../../open/1885-1669-20-extract-torrust-net-primitives-to-standalone-repo.md)                             | DONE   | Rule E; no workspace deps; no prerequisites; 10 consumers migrated; crate v0.1.0 published                                                    |
| InfoHash migration         | [#1889](https://github.com/torrust/torrust-tracker/issues/1889) — Migrate from `bittorrent-primitives` to `torrust-info-hash`                                                    | [docs/issues/open/1889-1669-21-migrate-from-bittorrent-primitives-to-torrust-info-hash.md](../../open/1889-1669-21-migrate-from-bittorrent-primitives-to-torrust-info-hash.md)                 | DONE   | SI-21; replaces `bittorrent-primitives` deps across 14 Cargo.toml files with `torrust-info-hash`; unblocks `bittorrent-primitives` archiving  |
| Tracker client extraction  | #TBD — Extract `torrust-tracker-client` to standalone repository                                                                                                                 | [docs/issues/drafts/1669-extract-torrust-tracker-client-to-standalone-repo.md](../../drafts/1669-extract-torrust-tracker-client-to-standalone-repo.md)                                         | TODO   | Rule E; blocked by `torrust-tracker-udp-tracker-protocol` publication (external to this EPIC)                                                 |
| Versioning policy          | #TBD — Define package versioning strategy (linked vs independent SemVer evolution)                                                                                               | [docs/issues/drafts/1669-define-package-versioning-strategy.md](../../drafts/1669-define-package-versioning-strategy.md)                                                                       | TODO   | Policy issue; defines release-train vs independent package cadence and migration plan                                                         |
| REST API architecture      | #TBD — Define REST API contract-first package architecture                                                                                                                       | [docs/issues/drafts/1669-define-rest-api-contract-first-package-architecture.md](../../drafts/1669-define-rest-api-contract-first-package-architecture.md)                                     | TODO   | Policy reminder only in this EPIC; validate via PoC, then execute migration in a dedicated API EPIC; defer API package extraction/publication |
| Configuration coupling     | [#1856](https://github.com/torrust/torrust-tracker/issues/1856) — Analyse configuration package coupling and evaluate splitting strategies                                       | [docs/issues/open/1856-1669-analyse-configuration-package-coupling/ISSUE.md](../../open/1856-1669-analyse-configuration-package-coupling/ISSUE.md)                                             | DONE   | DEC-07: keep single package; move TrackerPolicy/TORRENT_PEERS_LIMIT/PrivateMode to primitives (FU-1); see DECISIONS.md                        |
| Move domain primitives     | [#1859](https://github.com/torrust/torrust-tracker/issues/1859) — Move `TrackerPolicy`, `TORRENT_PEERS_LIMIT`, and `PrivateMode` to `torrust-tracker-primitives`                 | [docs/issues/open/1859-1669-move-tracker-policy-and-private-mode-to-primitives/ISSUE.md](../../open/1859-1669-move-tracker-policy-and-private-mode-to-primitives/ISSUE.md)                     | TODO   | Rule M; FU-1 from #1856; removes `swarm-coordination-registry` and `torrent-repository-benchmarking` config dep                               |
| TslConfig evaluation       | [#1860](https://github.com/torrust/torrust-tracker/issues/1860) — Evaluate moving `TslConfig` from `torrust-tracker-configuration` into `torrust-tracker-axum-server`            | [docs/issues/open/1860-1669-evaluate-tslconfig-move-to-axum-server/ISSUE.md](../../open/1860-1669-evaluate-tslconfig-move-to-axum-server/ISSUE.md)                                             | TODO   | Rule M candidate; FU-2 from #1856; may enable `axum-server` → `torrust-axum-server` reclassification                                          |
| Narrow init config slices  | [#1861](https://github.com/torrust/torrust-tracker/issues/1861) — Revisit `EnvContainer::initialize` to accept narrower config slices                                            | [docs/issues/open/1861-1669-narrow-envcontainer-initialize-config-slices/ISSUE.md](../../open/1861-1669-narrow-envcontainer-initialize-config-slices/ISSUE.md)                                 | TODO   | Design/analysis; FU-3 from #1856; addresses root forcing function for full-config compile-in when only one server runs                        |
| Rename-to-desired-state    | [#1829](https://github.com/torrust/torrust-tracker/issues/1829) — Rename crates and folder names to match desired `torrust-tracker` workspace state                              | [docs/issues/closed/1829-1669-11-rename-crates-and-folders-to-match-desired-tracker-workspace.md](../../closed/1829-1669-11-rename-crates-and-folders-to-match-desired-tracker-workspace.md)   | DONE   | SI-11 complete; spec archived to `docs/issues/closed/` after issue closure                                                                    |
| HTTP protocol decoupling   | [#1830](https://github.com/torrust/torrust-tracker/issues/1830) — Decouple `http-protocol` from `tracker-core`                                                                   | [docs/issues/closed/1830-1669-12-decouple-http-protocol-from-tracker-core.md](../../closed/1830-1669-12-decouple-http-protocol-from-tracker-core.md)                                           | DONE   | SI-12 complete; removed `http-protocol -> tracker-core` edge and moved mapping to higher layer                                                |
| HTTP/UDP decoupling        | [#1834](https://github.com/torrust/torrust-tracker/issues/1834) — Decouple `http-protocol` from `udp-protocol`                                                                   | [docs/issues/open/1834-1669-13-decouple-http-protocol-from-udp-protocol.md](../../open/1834-1669-13-decouple-http-protocol-from-udp-protocol.md)                                               | DONE   | SI-13 complete; removed `http-protocol -> udp-protocol` edge                                                                                  |
| HTTP/primitives decoupling | [#1835](https://github.com/torrust/torrust-tracker/issues/1835) — Decouple `http-protocol` from `torrust-tracker-primitives`                                                     | [docs/issues/open/1835-1669-14-decouple-http-protocol-from-tracker-primitives.md](../../open/1835-1669-14-decouple-http-protocol-from-tracker-primitives.md)                                   | DONE   | SI-14 complete; protocol-owned DTOs introduced and boundary mapping moved to core/server layers                                               |

Proposal note:
After SI-14, there is a proposal to evaluate a dedicated repository for protocol crates so protocol packages can evolve with BEP/spec changes while tracker app packages evolve with domain/product changes. This is proposal-only for now (not committed scope) and is tracked in [#1835](https://github.com/torrust/torrust-tracker/issues/1835) and [docs/issues/open/1835-1669-14-decouple-http-protocol-from-tracker-primitives.md](../../open/1835-1669-14-decouple-http-protocol-from-tracker-primitives.md).

### Subissue Specs Index

- [docs/issues/drafts/1669-01-establish-baseline-analysis.md](../../drafts/1669-01-establish-baseline-analysis.md)
- [docs/issues/drafts/1669-update-all-package-readmes.md](../../drafts/1669-update-all-package-readmes.md)
- [docs/issues/drafts/1669-extract-torrust-tracker-contrib-bencode-to-torrust-bencode.md](../../drafts/1669-extract-torrust-tracker-contrib-bencode-to-torrust-bencode.md)
- [docs/issues/open/1882-1669-18-extract-torrust-metrics-to-standalone-repo.md](../../open/1882-1669-18-extract-torrust-metrics-to-standalone-repo.md)
- [docs/issues/open/1884-1669-19-move-bittorrent-peer-id-to-torrust-bittorrent.md](../../open/1884-1669-19-move-bittorrent-peer-id-to-torrust-bittorrent.md)
- [docs/issues/open/1885-1669-20-extract-torrust-net-primitives-to-standalone-repo.md](../../open/1885-1669-20-extract-torrust-net-primitives-to-standalone-repo.md)
- [docs/issues/open/1889-1669-21-migrate-from-bittorrent-primitives-to-torrust-info-hash.md](../../open/1889-1669-21-migrate-from-bittorrent-primitives-to-torrust-info-hash.md)
- [docs/issues/open/1894-1669-22-extract-torrust-located-error-to-standalone-repo.md](../../open/1894-1669-22-extract-torrust-located-error-to-standalone-repo.md)
- [docs/issues/drafts/1669-extract-torrust-tracker-client-to-standalone-repo.md](../../drafts/1669-extract-torrust-tracker-client-to-standalone-repo.md)
- [docs/issues/drafts/1669-define-package-versioning-strategy.md](../../drafts/1669-define-package-versioning-strategy.md)
- [docs/issues/drafts/1669-define-rest-api-contract-first-package-architecture.md](../../drafts/1669-define-rest-api-contract-first-package-architecture.md)
- [docs/issues/open/1910-1669-si-29-rename-udp-and-http-core-protocol-crates-to-remove-redundant-tracker.md](../../open/1910-1669-si-29-rename-udp-and-http-core-protocol-crates-to-remove-redundant-tracker.md)
- [docs/issues/drafts/1669-configure-cargo-deny-for-layer-boundary-enforcement.md](../../drafts/1669-configure-cargo-deny-for-layer-boundary-enforcement.md)
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

### Which packages are extraction candidates?

The following decisions have been made (see DEC-14 for the naming and ownership policy):

- **Protocol crates** (`torrust-tracker-http-tracker-protocol`, `torrust-tracker-udp-tracker-protocol`,
  `torrust-tracker-core`, `torrust-tracker-udp-tracker-core`, `torrust-tracker-http-tracker-core`) —
  remain in the tracker workspace per the ownership policy. Not extraction candidates.
- ~~**`contrib/bencode`** (`torrust-tracker-contrib-bencode`)~~ — migrated to `torrust/torrust-bittorrent`
  as `torrust-bencode` 3.0.0 (#1881 ✅).
- ~~**`bittorrent-peer-id`** (`torrust-tracker-peer-id`)~~ — migrated to `torrust/torrust-bittorrent` as
  `torrust-peer-id` 0.1.0 (#1884 ✅).
- **Utility crates** (`torrust-clock`, `torrust-located-error`, `torrust-metrics`, `torrust-net-primitives`) —
  already extracted to standalone repositories.
- **`torrust-server-lib`** — extraction candidate (depends only on published crates).
- **`torrust-tracker-client`** (console CLI) — extraction candidate (blocked by publication of
  `torrust-tracker-udp-tracker-protocol`).

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
extraction). The table below analyses every extraction candidate against this constraint.

**Already extracted** (completed):

| Package                                                     | Status   |
| ----------------------------------------------------------- | -------- |
| `torrust-tracker-contrib-bencode` → `torrust-bencode` 3.0.0 | ✅ #1881 |
| `bittorrent-peer-id` → `torrust-peer-id` 0.1.0              | ✅ #1884 |
| `torrust-located-error` → `torrust-located-error` 3.0.0     | ✅ #1894 |
| `torrust-tracker-clock` → `torrust-clock` 3.0.0             | ✅ #1879 |
| `torrust-tracker-metrics` → `torrust-metrics` 0.1.0         | ✅ #1882 |
| `torrust-net-primitives` → `torrust-net-primitives` 0.1.0   | ✅ #1885 |

**Current candidates** (under consideration):

| Package                                | Crates.io status | Unpublished runtime workspace deps                                   | Can be extracted? | Blocked by                             |
| -------------------------------------- | ---------------- | -------------------------------------------------------------------- | ----------------- | -------------------------------------- |
| `torrust-server-lib`                   | Yes              | None (dep only on published crates)                                  | ✅                | No blockers                            |
| `torrust-tracker-client` (console CLI) | No               | `torrust-tracker-udp-tracker-protocol`, `torrust-tracker-client-lib` | ❌                | Publication of the two blocking crates |

**Not extraction candidates** (per DEC-14, remain in tracker workspace):

| Package                                      | Reason                                                   |
| -------------------------------------------- | -------------------------------------------------------- |
| `torrust-tracker-udp-tracker-protocol`       | Tracker-owned protocol crate; stays in tracker workspace |
| `torrust-tracker-http-tracker-protocol`      | Tracker-owned protocol crate; stays in tracker workspace |
| `torrust-tracker-core`                       | Tracker-owned core crate; stays in tracker workspace     |
| `torrust-tracker-udp-tracker-core`           | Tracker-owned core crate; stays in tracker workspace     |
| `torrust-tracker-http-tracker-core`          | Tracker-owned core crate; stays in tracker workspace     |
| `torrust-tracker-axum-*` (all server crates) | Tracker-owned server crates; stays in tracker workspace  |

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

> **Future consideration — workspace coupling CI check**: Once the baseline coupling analysis
> tool (`contrib/dev-tools/analysis/workspace-coupling/`) is mature and stable, consider
> adding the coupling report generation or a coupling-regression check to CI (e.g.,
> a pre-commit hook or a GitHub Action that fails when new thin dependencies are introduced).
> This would prevent new coupling regressions as the EPIC progresses. Not in scope for the
> current cycle — revisit after the baseline analysis is complete and the tool has proven
> useful.

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
- 2026-06-09 20:00 UTC - josecelano - Updated Package Inventory, Desired Package State,
  and dependency lists to reflect completion of SI-18, SI-19, SI-20, SI-22 extractions
  and SI-21 InfoHash migration.

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
