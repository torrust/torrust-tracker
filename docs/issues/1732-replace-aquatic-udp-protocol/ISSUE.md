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
- A `NOTICE` file is included (if the original has one).
- Modifications are clearly marked.

We must include the Apache 2.0 license text and copyright header in the new package.

### Types currently used across the workspace

The following distinct types are imported from `aquatic_udp_protocol` in 26 source files across
13 packages:

| Category            | Types                                                                                        |
| ------------------- | -------------------------------------------------------------------------------------------- |
| Request types       | `Request`, `ConnectRequest`, `AnnounceRequest`, `ScrapeRequest`                              |
| Response types      | `Response`, `ConnectResponse`, `AnnounceResponse<T>`, `ScrapeResponse`, `ErrorResponse`     |
| Identifiers         | `TransactionId`, `ConnectionId`, `InfoHash`, `PeerId`                                        |
| Announce parameters | `AnnounceEvent`, `AnnounceActionPlaceholder`, `Port`, `PeerKey`                              |
| Counters            | `NumberOfBytes`, `NumberOfPeers`, `NumberOfDownloads`                                        |
| Scrape statistics   | `TorrentScrapeStatistics`                                                                    |
| Address types       | `Ipv4AddrBytes`, `Ipv6AddrBytes`                                                             |
| Modules             | `aquatic_udp_protocol::common`                                                               |

### Packages to update

| Package                       | Path                                  |
| ----------------------------- | ------------------------------------- |
| `bittorrent-udp-protocol`     | `packages/udp-protocol`               |
| `bittorrent-http-protocol`    | `packages/http-protocol`              |
| `bittorrent-udp-tracker-core` | `packages/udp-tracker-core`           |
| `bittorrent-tracker-core`     | `packages/tracker-core`               |
| `bittorrent-http-tracker-core`| `packages/http-tracker-core`          |
| `bittorrent-tracker-primitives`| `packages/primitives`                |
| `axum-http-tracker-server`    | `packages/axum-http-tracker-server`   |
| `axum-rest-tracker-api-server`| `packages/axum-rest-tracker-api-server`|
| `swarm-coordination-registry` | `packages/swarm-coordination-registry`|
| `torrent-repository-benchmarking`| `packages/torrent-repository-benchmarking`|
| `bittorrent-tracker-client`   | `packages/tracker-client`             |
| `tracker-client` (console)    | `console/tracker-client`              |
| `udp-tracker-server`          | `packages/udp-tracker-server`         |

## Goals

- [ ] Remove the external `aquatic_udp_protocol` dependency from the entire workspace.
- [ ] Own the BEP 15 implementation in an internal package that we fully control.
- [ ] Apply the `zerocopy` 0.8 migration (unblocking
      [torrust/torrust-tracker#1682](https://github.com/torrust/torrust-tracker/pull/1682)).
- [ ] Keep all existing tests green throughout the migration.
- [ ] Pass `linter all` and `cargo machete` with zero warnings after every step.

## Implementation Plan

### Step 1: Create `packages/aquatic-udp-protocol` (internal fork)

- [ ] Copy the `aquatic_udp_protocol` 0.9.0 source (4 files) into a new workspace package
      `packages/aquatic-udp-protocol`.
- [ ] Add the Apache 2.0 `LICENSE` file and a `NOTICE` file crediting the original author
      (Joakim Frostegård / greatest-ape).
- [ ] Add a `README.md` explaining that this is a temporary internal fork.
- [ ] Register the package in the workspace `Cargo.toml`.
- [ ] Point all 13 packages at the internal fork instead of the crates.io version
      (`aquatic_udp_protocol = { path = "../aquatic-udp-protocol" }`).
- [ ] Verify the build compiles and all tests pass.

### Step 2: Strip unused items from the internal fork

- [ ] Identify and remove any code paths, feature flags, or types from the fork that no
      package in this workspace uses.
- [ ] Confirm no regressions.

### Step 3: Apply the `zerocopy` 0.8 migration

- [ ] Update `zerocopy` to `0.8` in the fork's `Cargo.toml`.
- [ ] Apply the API migration from our PR
      ([aquatic#235](https://github.com/greatest-ape/aquatic/pull/235)).
- [ ] Ensure the build is clean under the workspace `rustflags` (`-D warnings`, etc.).

### Step 4: Migrate `packages/udp-protocol` to own the protocol types

- [ ] Move all BEP 15 types into `packages/udp-protocol`, replacing the internal fork.
- [ ] Update all 13 dependent packages to import from `bittorrent-udp-tracker-protocol`
      instead of `aquatic_udp_protocol`.
- [ ] Remove `packages/aquatic-udp-protocol` from the workspace once no package depends on it.
- [ ] Remove `aquatic_udp_protocol` from every `Cargo.toml`.

### Step 5: Redesign types to fit the Torrust Tracker domain model

- [ ] Review each type and assess whether a domain-specific redesign is warranted.
- [ ] Introduce new types iteratively — keeping the existing API intact until each replacement
      is complete.
- [ ] Document design decisions in an ADR if any significant trade-offs arise.

## Acceptance Criteria

- [ ] `aquatic_udp_protocol` does not appear in any `Cargo.toml` or source file.
- [ ] All workspace tests pass (`cargo test --workspace`).
- [ ] `linter all` exits with code `0`.
- [ ] `cargo machete` reports no unused dependencies.
- [ ] The `zerocopy` version across the workspace is `0.8`.
- [ ] The internal fork (`packages/aquatic-udp-protocol`) has been removed by the end of
      Step 4.

## References

- Upstream crate: <https://crates.io/crates/aquatic_udp_protocol>
- Upstream repository: <https://github.com/greatest-ape/aquatic>
- Upstream `zerocopy` upgrade issue: <https://github.com/greatest-ape/aquatic/issues/224>
- Our unmerged upgrade PR: <https://github.com/greatest-ape/aquatic/pull/235>
- Dependabot PR (blocked): <https://github.com/torrust/torrust-tracker/pull/1682>
- BEP 15 specification: <https://www.bittorrent.org/beps/bep_0015.html>
- Apache 2.0 license: <https://www.apache.org/licenses/LICENSE-2.0>
