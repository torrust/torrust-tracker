# Replace `aquatic_udp_protocol` with an In-House UDP Protocol Crate

## Overview

The Torrust Tracker currently depends on
[`aquatic_udp_protocol`](https://crates.io/crates/aquatic_udp_protocol) (from the
[`aquatic`](https://github.com/greatest-ape/aquatic) project) for BitTorrent UDP tracker
protocol types, serialization, and deserialization (BEP 15).

The upstream project has been inactive since February 2025. An open issue
([aquatic#224](https://github.com/greatest-ape/aquatic/issues/224)) requesting a `zerocopy` 0.8
upgrade has received no response. We contributed a PR
([aquatic#235](https://github.com/greatest-ape/aquatic/pull/235)) to apply the fix ourselves,
but it has also remained unreviewed. This `zerocopy` version mismatch currently blocks
[torrust/torrust-tracker#1682](https://github.com/torrust/torrust-tracker/pull/1682) — a
recurring dependabot PR that cannot be merged.

With **13 packages** in this workspace directly depending on `aquatic_udp_protocol`, continuing
to rely on an apparently unmaintained external crate is a maintenance and security risk.

The proposal is to own the UDP protocol implementation inside this workspace:

1. Copy the current `aquatic_udp_protocol` source into a new internal package
   (`packages/aquatic-udp-protocol`) under the terms of its Apache 2.0 license.
2. Remove everything we do not use.
3. Apply the `zerocopy` 0.8 migration from our unmerged PR.
4. Migrate `packages/udp-protocol` to own all protocol types, absorbing the internal fork.
5. Remove the interim fork once the migration is complete.
6. Progressively redesign the types so they fit the Torrust Tracker domain model — while
   keeping the public surface backward-compatible throughout the transition.

## Background

### Why `aquatic_udp_protocol`?

It provides a complete, correct implementation of the BEP 15 UDP tracker wire protocol.
The crate is small (~785 SLoC, 4 source files: `common.rs`, `lib.rs`, `request.rs`,
`response.rs`), making an in-house replacement feasible.

### License

`aquatic_udp_protocol` is published under **Apache 2.0**, which is fully compatible with the
Torrust Tracker's AGPL-3.0 license. Apache 2.0 permits copying, modification, and
redistribution provided that:

- The original copyright notice is preserved.
- A `NOTICE` file is included (if the original has one — the aquatic repo does not have one).
- Modifications are clearly marked.

We must include the Apache 2.0 `LICENSE` file in each new package and attribute the original
author in the `README.md`.

### No publishing required

The internal fork packages (`packages/aquatic-peer-id`, `packages/aquatic-udp-protocol`) are
**never published to crates.io**. All dependent packages reference them via Cargo path
dependencies (`path = "../aquatic-peer-id"`, `path = "../aquatic-udp-protocol"`), which are
resolved locally by Cargo. The crate names are kept identical to the upstream ones
(`aquatic_peer_id`, `aquatic_udp_protocol`) so that all existing `use` statements in the
codebase compile without changes. Once Step 4 is complete and the packages are removed from the
workspace, the path dependencies are removed along with them.

### Types currently used across the workspace

The following distinct types are imported from `aquatic_udp_protocol` in 26 source files across
13 packages:

| Category            | Types                                                                                   |
| ------------------- | --------------------------------------------------------------------------------------- |
| Request types       | `Request`, `ConnectRequest`, `AnnounceRequest`, `ScrapeRequest`                         |
| Response types      | `Response`, `ConnectResponse`, `AnnounceResponse<T>`, `ScrapeResponse`, `ErrorResponse` |
| Identifiers         | `TransactionId`, `ConnectionId`, `InfoHash`, `PeerId`                                   |
| Announce parameters | `AnnounceEvent`, `AnnounceActionPlaceholder`, `Port`, `PeerKey`                         |
| Counters            | `NumberOfBytes`, `NumberOfPeers`, `NumberOfDownloads`                                   |
| Scrape statistics   | `TorrentScrapeStatistics`                                                               |
| Address types       | `Ipv4AddrBytes`, `Ipv6AddrBytes`                                                        |
| Modules             | `aquatic_udp_protocol::common`                                                          |

### Packages to update

| Package                           | Path                                       |
| --------------------------------- | ------------------------------------------ |
| `bittorrent-udp-protocol`         | `packages/udp-protocol`                    |
| `bittorrent-http-protocol`        | `packages/http-protocol`                   |
| `bittorrent-udp-tracker-core`     | `packages/udp-tracker-core`                |
| `bittorrent-tracker-core`         | `packages/tracker-core`                    |
| `bittorrent-http-tracker-core`    | `packages/http-tracker-core`               |
| `bittorrent-tracker-primitives`   | `packages/primitives`                      |
| `axum-http-tracker-server`        | `packages/axum-http-tracker-server`        |
| `axum-rest-tracker-api-server`    | `packages/axum-rest-tracker-api-server`    |
| `swarm-coordination-registry`     | `packages/swarm-coordination-registry`     |
| `torrent-repository-benchmarking` | `packages/torrent-repository-benchmarking` |
| `bittorrent-tracker-client`       | `packages/tracker-client`                  |
| `tracker-client` (console)        | `console/tracker-client`                   |
| `udp-tracker-server`              | `packages/udp-tracker-server`              |

## Goals

- [ ] Remove the external `aquatic_udp_protocol` dependency from the entire workspace.
- [ ] Own the BEP 15 implementation in an internal package that we fully control.
- [ ] Apply the `zerocopy` 0.8 migration (unblocking
      [torrust/torrust-tracker#1682](https://github.com/torrust/torrust-tracker/pull/1682)).
- [ ] Keep all existing tests green throughout the migration.
- [ ] Pass `linter all` and `cargo machete` with zero warnings after every step.

## Implementation Plan

### Step 1: Create `packages/aquatic-udp-protocol` (internal fork)

#### Step 1a: Add the internal fork packages to the workspace

- [x] Copy the `aquatic_udp_protocol` 0.9.0 source (4 files) into a new workspace package
      `packages/aquatic-udp-protocol`. Also copied `aquatic_peer_id` 0.9.0 into
      `packages/aquatic-peer-id` (needed because `PeerClient` is used in the workspace).
- [x] Add the Apache 2.0 `LICENSE` file to each fork package. The upstream aquatic repo has no
      `NOTICE` file and no per-file copyright headers, so none need to be copied. Each source
      file carries an inline attribution header naming the original author (Joakim Frostegård /
      greatest-ape), linking to the source crate version on crates.io, and stating the Apache
      2.0 license.
- [x] Add a `README.md` to each fork package explaining it is a temporary internal fork.
- [x] Register both packages in the workspace `Cargo.toml`.

#### Step 1b: Switch all dependent packages to the internal fork

- [x] Point all 13 packages at the internal fork instead of the crates.io version
      (`aquatic_udp_protocol = { path = "../aquatic-udp-protocol" }`).
- [x] Verify the build compiles and all tests pass.

### Step 2: Strip unused items from the internal fork

Analysis documented in [step-2-analysis.md](step-2-analysis.md).

- [x] Identify and remove any code paths, feature flags, or types from the fork that no
      package in this workspace uses.
- [x] Confirm no regressions.

After a thorough search of all 26 source files across 13 packages, no unused public types,
functions, or feature-enabled code paths were found that could be safely removed. Every public
type is used by at least one workspace package. The only internal-only item (`AnnounceEventBytes`)
is structurally required for zero-copy deserialization and cannot be removed. No changes to the
fork source were needed.

### Step 3: Apply the `zerocopy` 0.8 migration

- [ ] Update `zerocopy` to `0.8` in the fork's `Cargo.toml`.
- [ ] Apply the API migration from our PR
      ([aquatic#235](https://github.com/greatest-ape/aquatic/pull/235)).
- [ ] Ensure the build is clean under the workspace `rustflags` (`-D warnings`, etc.).

### Step 4: Absorb the internal forks into their permanent homes

`PeerId` and `PeerClient` are domain concepts used across the workspace (UDP tracker, HTTP tracker,
REST API, core logic), not UDP-protocol-specific. They should live in `packages/primitives` with
other peer-related types. UDP protocol types (`Request`, `Response`, etc.) belong in
`packages/udp-protocol`. This split requires two substeps.

#### Step 4a: Migrate UDP protocol types to `packages/udp-protocol`

- [ ] Move all BEP 15 protocol types (`Request`, `Response`, common types) from
      `packages/aquatic-udp-protocol` into `packages/udp-protocol/src/`.
      Add an inline attribution comment to each migrated source file crediting the original
      `aquatic_udp_protocol` 0.9.0 as the starting point.
- [ ] Update all packages that import from `aquatic_udp_protocol` to import from
      `bittorrent-udp-tracker-protocol` instead.
- [ ] Remove `aquatic_udp_protocol` from every `Cargo.toml`.

#### Step 4b: Migrate peer ID types to `packages/primitives`

- [ ] Move `PeerId` and `PeerClient` from `packages/aquatic-peer-id` into
      `packages/primitives/src/` (alongside existing peer-related domain types).
      Add an inline attribution comment crediting the original `aquatic_peer_id` 0.9.0 as the
      starting point.
- [ ] Update all packages that import from `aquatic_peer_id` to import from
      `bittorrent-tracker-primitives` instead.
- [ ] Remove `aquatic_peer_id` from every `Cargo.toml`.
- [ ] Remove both interim forks (`packages/aquatic-udp-protocol` and `packages/aquatic-peer-id`)
      from the workspace `Cargo.toml` once no package depends on them.

### Step 5: Redesign types to fit the Torrust Tracker domain model

- [ ] Review each type and assess whether a domain-specific redesign is warranted.
- [ ] Introduce new types iteratively — keeping the existing API intact until each replacement
      is complete.
- [ ] Document design decisions in an ADR if any significant trade-offs arise.

## Acceptance Criteria

- [ ] `aquatic_udp_protocol` and `aquatic_peer_id` do not appear in any `Cargo.toml` or source file.
- [ ] All workspace tests pass (`cargo test --workspace`).
- [ ] `linter all` exits with code `0`.
- [ ] `cargo machete` reports no unused dependencies.
- [ ] The `zerocopy` version across the workspace is `0.8`.
- [ ] Both interim forks (`packages/aquatic-udp-protocol` and `packages/aquatic-peer-id`) have been
      removed from the workspace by the end of Step 4b.
- [ ] `PeerId` and `PeerClient` live in `packages/primitives`.
- [ ] UDP protocol types live in `packages/udp-protocol`.

## References

- Upstream crate: <https://crates.io/crates/aquatic_udp_protocol>
- Upstream repository: <https://github.com/greatest-ape/aquatic>
- Upstream `zerocopy` upgrade issue: <https://github.com/greatest-ape/aquatic/issues/224>
- Our unmerged upgrade PR: <https://github.com/greatest-ape/aquatic/pull/235>
- Dependabot PR (blocked): <https://github.com/torrust/torrust-tracker/pull/1682>
- BEP 15 specification: <https://www.bittorrent.org/beps/bep_0015.html>
- Apache 2.0 license: <https://www.apache.org/licenses/LICENSE-2.0>
