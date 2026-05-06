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

- [x] Remove the external `aquatic_udp_protocol` dependency from the entire workspace.
- [x] Own the BEP 15 implementation in an internal package that we fully control.
- [x] Apply the `zerocopy` 0.8 migration (unblocking
      [torrust/torrust-tracker#1682](https://github.com/torrust/torrust-tracker/pull/1682)).
- [x] Keep all existing tests green throughout the migration.
- [x] Pass `linter all` and `cargo machete` with zero warnings after every step.

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

Analysis of the transitive dependency problem documented in
[step-3-bittorrent-primitives-problem.md](step-3-bittorrent-primitives-problem.md).

- [x] Update `zerocopy` to `0.8` in `packages/aquatic-udp-protocol/Cargo.toml` and
      `packages/aquatic-peer-id/Cargo.toml`.
- [x] Apply the API migration from our PR
      ([aquatic#235](https://github.com/greatest-ape/aquatic/pull/235)) to all four fork source
      files (`common.rs`, `request.rs`, `response.rs`, `lib.rs` of `aquatic-peer-id`).
- [x] Update `zerocopy` to `0.8` in `packages/primitives/Cargo.toml` and fix the one
      `read_from` → `read_from_bytes` call site in `src/peer.rs`.
- [x] Create an internal fork of `bittorrent-primitives` at `packages/bittorrent-primitives/`
      to fix the transitive API breakage (see
      [step-3-bittorrent-primitives-problem.md](step-3-bittorrent-primitives-problem.md)).
      Add it to `[patch.crates-io]` and to workspace `members`.
- [x] Ensure the build is clean under the workspace `rustflags` (`-D warnings`, etc.) —
      `cargo check --workspace` passes with no errors or warnings.

### Step 4: Absorb the internal forks into their permanent homes

#### Architectural context

Three types currently defined in `packages/aquatic-udp-protocol` are **domain types**, not
protocol wire types:

| Type            | Current location                | Correct home          |
| --------------- | ------------------------------- | --------------------- |
| `PeerId`        | `aquatic-peer-id` (re-exported) | `packages/primitives` |
| `PeerClient`    | `aquatic-peer-id`               | `packages/primitives` |
| `AnnounceEvent` | `aquatic-udp-protocol`          | `packages/primitives` |
| `NumberOfBytes` | `aquatic-udp-protocol`          | `packages/primitives` |

These types ended up in the protocol package only because BEP 15 was where they first appeared.
In practice they are used across protocols without any UDP-specific wire format:

- `PeerId([u8; 20])` — identifies a peer; used in both UDP and HTTP trackers.
- `AnnounceEvent` — a pure domain enum (`Started` / `Stopped` / `Completed` / `None`); carries
  no wire-format information.
- `NumberOfBytes` — represents transfer statistics (`uploaded`, `downloaded`, `left`) inside the
  domain `Peer` struct. The current definition `NumberOfBytes(pub I64)` uses a zerocopy
  network-endian wrapper `I64` only because `AnnounceRequest` needs to derive `FromBytes` /
  `IntoBytes`. That zerocopy detail has no place in a domain type.

The `Peer` struct in `packages/primitives/src/peer.rs` is a domain type, yet it currently
depends on protocol wire-format types for three of its fields. That is the root of the
architectural problem: the **dependency direction is inverted**.

The correct layering is:

```text
packages/bittorrent-primitives   — InfoHash (standalone BitTorrent primitive)
         ↑
packages/primitives              — PeerId, PeerClient, AnnounceEvent, NumberOfBytes(i64), Peer
         ↑
packages/udp-protocol            — wire types (AnnounceRequest, …), converts I64 ↔ NumberOfBytes
         ↑
packages/udp-tracker-core        — handles the UDP request/response lifecycle
```

`packages/primitives` must depend on **nothing** in the protocol layer. UDP protocol packages
must depend **downward** on `primitives` to re-use domain types in conversions.

#### The circular dependency problem

There is a dependency cycle that prevents a direct migration in a single step:

```text
udp-protocol → primitives           (via peer_builder.rs: constructs torrust_tracker_primitives::Peer)
primitives   → aquatic-udp-protocol  (for PeerId, AnnounceEvent, NumberOfBytes)
```

After Step 4a moves all aquatic types into `udp-protocol`, `packages/primitives` would need to
import those types from `udp-protocol` — but `udp-protocol` already depends on `primitives`.
That would create a **direct circular dependency**: `udp-protocol → primitives → udp-protocol`.

#### Breaking the cycle: define domain types natively first (Step 4b)

The cleanest fix avoids the cycle entirely by making `packages/primitives` self-contained:
define `PeerId`, `PeerClient`, `AnnounceEvent`, and `NumberOfBytes` natively in `primitives`
instead of importing them from any protocol package. Once that is done, `primitives` has no
dependency on any protocol package — the cycle never forms — and the correct dependency
direction is established in a single move.

**`NumberOfBytes` representation change**: the domain type becomes `NumberOfBytes(pub i64)` (plain
Rust `i64`, host byte order). The wire-format type `NumberOfBytes(I64)` (big-endian zerocopy) is
retained inside `packages/udp-protocol` only, renamed or clearly scoped as a wire-format type.
The conversion in `peer_builder.rs` calls `.0.get()` to extract the `i64` from the wire `I64`.

**Required step order:**

1. **Step 4b** (domain types to `primitives`): Define `PeerId`, `PeerClient`, `AnnounceEvent`,
   and `NumberOfBytes(i64)` natively in `packages/primitives`. Remove the
   `bittorrent_udp_tracker_protocol` / `aquatic-peer-id` dependencies from
   `packages/primitives/Cargo.toml`. This step severs the architectural inversion and eliminates
   the cycle root cause.

2. **Step 4a-prep** (move `peer_builder`): `peer_builder.rs` is a domain-adapter, not a
   protocol-parsing concern. Move it from `packages/udp-protocol` to `packages/udp-tracker-core`.
   Remove `torrust-tracker-primitives` from `packages/udp-protocol/Cargo.toml`. After this, the
   dependency graph has no cycle and no architectural inversion.

3. **Step 4a** (absorb aquatic fork): With the clean dependency graph in place, inline the
   aquatic fork source files into `packages/udp-protocol` and remove the fork packages.

4. **Step 4c** (standalone `InfoHash`): Make `bittorrent-primitives::InfoHash` self-contained
   by replacing the `aquatic_udp_protocol::InfoHash` inner field with a plain `[u8; 20]`.

#### Step 4b: Define domain types natively in `packages/primitives`

- [x] Copy `PeerId([u8; 20])` and `PeerClient` from `packages/aquatic-peer-id/src/lib.rs` into
      a new file `packages/primitives/src/peer_id.rs`. Add an inline attribution comment
      crediting the original `aquatic_peer_id` 0.9.0.
- [x] Define `AnnounceEvent { Started, Stopped, Completed, None }` natively in
      `packages/primitives/src/` (e.g., `announce_event.rs` or alongside `peer.rs`).
- [x] Define `NumberOfBytes(pub i64)` natively in `packages/primitives/src/`. Implement
      `NumberOfBytes::new(v: i64) -> Self` to match the existing call sites.
- [x] Update `packages/primitives/src/peer.rs` to import `PeerId`, `AnnounceEvent`, and
      `NumberOfBytes` from the local crate rather than from `bittorrent_udp_tracker_protocol`.
- [x] Remove `bittorrent_udp_tracker_protocol` from `packages/primitives/Cargo.toml`.
- [x] Update `packages/udp-protocol/src/peer_builder.rs` to convert the wire `NumberOfBytes(I64)`
      to the domain `primitives::NumberOfBytes(i64)` using `.0.get()`.
- [x] Update all affected packages, tests, benches, and adapters to use the new primitives
      domain types where they actually model tracker-domain state (`Peer`, HTTP announce parsing,
      REST resources, benchmarking fixtures, and tracker-core test helpers).
- [x] Keep compatibility explicit at the protocol/domain boundary instead of re-exporting the
      domain types from `packages/udp-protocol`. Re-exporting `PeerId` / `AnnounceEvent` from the
      protocol crate would shadow the real wire types and break code that still needs the BEP 15
      representation. The current boundary is handled by explicit conversions in adapters such as
      `peer_builder.rs`.
- [x] Verify `cargo check --workspace` and `linter all` pass with no errors.

#### Step 4a-prep: Move `peer_builder` to `packages/udp-tracker-core`

- [x] Copy `packages/udp-protocol/src/peer_builder.rs` into
      `packages/udp-tracker-core/src/peer_builder.rs` (or a suitable submodule).
- [x] Remove `pub mod peer_builder;` from `packages/udp-protocol/src/lib.rs`.
- [x] Update `packages/udp-tracker-core/src/services/announce.rs` to import `peer_builder`
      from the local module instead of `bittorrent_udp_tracker_protocol::peer_builder`.
- [x] Remove `torrust-tracker-primitives` from `packages/udp-protocol/Cargo.toml`
      (it is no longer needed once `peer_builder` is gone).
- [x] Verify `cargo check --workspace` and `linter all` pass with no errors.

#### Step 4a: Migrate UDP protocol types to `packages/udp-protocol`

- [x] Move all BEP 15 protocol types (`Request`, `Response`, common types) from
      `packages/aquatic-udp-protocol` into `packages/udp-protocol/src/`.
      Add an inline attribution comment to each migrated source file crediting the original
      `aquatic_udp_protocol` 0.9.0 as the starting point.
- [x] Retain a wire-format `NumberOfBytes` type (or inline `I64` fields) inside `udp-protocol`
      to keep zero-copy deserialization of `AnnounceRequest`. Do not expose it as a public
      re-export; the public API uses `primitives::NumberOfBytes`.
- [x] Inline the remaining `aquatic_peer_id` fork code needed by the protocol layer into
      `packages/udp-protocol/src/peer_id.rs` so the in-house crate is self-contained.
- [x] Update all packages that import from `aquatic_udp_protocol` to import from
      `bittorrent-udp-tracker-protocol` instead. `packages/primitives` is now safe to migrate
      (its own domain types are native; no cycle can form).
- [x] Remove `aquatic_udp_protocol` from every `Cargo.toml`.
- [x] Remove the no-longer-needed dependency edge from `packages/udp-protocol` to the clock crate.
      That dead edge became visible after moving `peer_builder` and would otherwise reintroduce a
      package cycle through `clock -> primitives -> bittorrent-primitives -> udp-protocol`.
- [x] Remove both interim forks (`packages/aquatic-udp-protocol` and `packages/aquatic-peer-id`)
      from the workspace `Cargo.toml` once no package depends on them.
- [x] Verify `cargo check --workspace` and `linter all` pass with no errors.
- [x] Verify `cargo test --doc --workspace` passes after updating doc tests to use
      domain types where required.
- [x] Verify `contrib/dev-tools/git/hooks/pre-commit.sh` passes end-to-end.

#### Step 4c: Consolidate `InfoHash` into `bittorrent-primitives`

The internal fork at `packages/bittorrent-primitives/` currently delegates `InfoHash` storage to
`aquatic_udp_protocol::InfoHash`. After Step 4a removes the `aquatic_udp_protocol` dependency from
all other packages, this is the last remaining use of that type from the fork.

- [x] Replace the `data: aquatic_udp_protocol::InfoHash` field with a plain `[u8; 20]` array
      directly inside `bittorrent-primitives::InfoHash`.
- [x] Remove the `aquatic_udp_protocol` dependency from `packages/bittorrent-primitives/Cargo.toml`.
- [x] Update all impls in `src/info_hash.rs` that previously delegated to
      `aquatic_udp_protocol::InfoHash` to operate on the inner `[u8; 20]` directly.
- [x] Ensure all existing tests in `bittorrent-primitives` pass.
- [x] Publish a new version of `bittorrent-primitives` to crates.io once the crate is
      self-contained (no external protocol dependencies).
- [x] Remove the `packages/bittorrent-primitives/` fork and the `[patch.crates-io]` entry once
      the published version is available.

> **Note on step ordering**: Step 4c is independent of Steps 4b and 4a-prep. It can be done in
> parallel or in any order relative to those steps. Step 4c only unblocks removal of the
> `bittorrent-primitives` fork from `[patch.crates-io]`.

### Step 5: Redesign types to fit the Torrust Tracker domain model

- [ ] Review each type and assess whether a domain-specific redesign is warranted.
- [ ] Introduce new types iteratively — keeping the existing API intact until each replacement
      is complete.
- [ ] Document design decisions in an ADR if any significant trade-offs arise.

## Acceptance Criteria

- [x] `aquatic_udp_protocol` and `aquatic_peer_id` are removed as dependencies/imports from
      workspace packages (`Cargo.toml` and Rust code imports).
- [x] All workspace tests pass (`cargo test --workspace`).
- [x] `linter all` exits with code `0`.
- [x] `cargo machete` reports no unused dependencies.
- [x] The `zerocopy` version across the workspace is `0.8`.
- [x] Both interim forks (`packages/aquatic-udp-protocol` and `packages/aquatic-peer-id`) have been
      removed from the workspace members by the end of Step 4a. The fork directories still exist
      on disk and will be physically deleted as a follow-up cleanup.
- [x] `PeerId`, `PeerClient`, `AnnounceEvent`, and `NumberOfBytes` live natively in
      `packages/primitives` (no protocol dep).
- [x] `packages/primitives` has no dependency on any UDP or HTTP protocol package.
- [x] UDP wire-format protocol types live in `packages/udp-protocol`.
- [x] `bittorrent-primitives::InfoHash` is self-contained with a plain `[u8; 20]` inner field.

## References

- Upstream crate: <https://crates.io/crates/aquatic_udp_protocol>
- Upstream repository: <https://github.com/greatest-ape/aquatic>
- Upstream `zerocopy` upgrade issue: <https://github.com/greatest-ape/aquatic/issues/224>
- Our unmerged upgrade PR: <https://github.com/greatest-ape/aquatic/pull/235>
- Dependabot PR (blocked): <https://github.com/torrust/torrust-tracker/pull/1682>
- BEP 15 specification: <https://www.bittorrent.org/beps/bep_0015.html>
- Apache 2.0 license: <https://www.apache.org/licenses/LICENSE-2.0>
