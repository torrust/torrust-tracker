---
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/closed/1732-replace-aquatic-udp-protocol/ISSUE.md
    - packages/udp-protocol/
---

# Step 2 Analysis: Unused Code in Internal Forks

## Objective

Identify and remove any code paths, feature flags, or types from the internal forks
(`packages/aquatic-peer-id`, `packages/aquatic-udp-protocol`) that no package in this workspace
uses.

## Approach

For each public item exported by the two fork packages, we searched the entire workspace for
import or use sites outside the fork packages themselves.

## Findings

### `packages/aquatic-udp-protocol`

#### Public types used outside the fork

All of the following types are referenced by at least one workspace package outside of the fork:

| Type                        | Used by                                                                                             |
| --------------------------- | --------------------------------------------------------------------------------------------------- |
| `Request`                   | `udp-tracker-core`, `udp-tracker-server`, `tracker-client` (console), `bittorrent-tracker-client`   |
| `ConnectRequest`            | `udp-tracker-core`, `udp-tracker-server`, `tracker-client` (console), `bittorrent-tracker-client`   |
| `AnnounceRequest`           | `udp-tracker-core`, `http-tracker-core`, `tracker-core`, `axum-rest-tracker-api-server`, and others |
| `ScrapeRequest`             | `udp-tracker-core`, `udp-tracker-server`, `tracker-client` (console), `bittorrent-tracker-client`   |
| `Response`                  | `udp-tracker-core`, `udp-tracker-server`, `tracker-client` (console), `bittorrent-tracker-client`   |
| `ConnectResponse`           | `udp-tracker-core`, `udp-tracker-server`, `tracker-client` (console), `bittorrent-tracker-client`   |
| `AnnounceResponse`          | `udp-tracker-core`, `udp-tracker-server`, `tracker-client` (console), `bittorrent-tracker-client`   |
| `ScrapeResponse`            | `udp-tracker-core`, `udp-tracker-server`, `tracker-client` (console), `bittorrent-tracker-client`   |
| `ErrorResponse`             | `udp-tracker-core`, `udp-tracker-server`, `tracker-client` (console), `bittorrent-tracker-client`   |
| `TransactionId`             | `udp-tracker-core`, `udp-tracker-server`, `tracker-client` (console), `bittorrent-tracker-client`   |
| `ConnectionId`              | `udp-tracker-core`, `udp-tracker-server`, `tracker-client` (console), `bittorrent-tracker-client`   |
| `InfoHash`                  | `udp-protocol`, `udp-tracker-core`, `tracker-core`, `swarm-coordination-registry`, and others       |
| `PeerId` (re-export)        | `udp-protocol`, `udp-tracker-core`, `tracker-core`, and others (via `aquatic_peer_id`)              |
| `AnnounceEvent`             | `udp-tracker-core`, `http-tracker-core`, `tracker-core`, `axum-rest-tracker-api-server`, and others |
| `AnnounceActionPlaceholder` | `udp-tracker-core`, `udp-tracker-server`                                                            |
| `Port`                      | `udp-tracker-core`, `udp-tracker-server`, `tracker-client` (console), and others                    |
| `PeerKey`                   | `udp-tracker-core`, `udp-tracker-server`                                                            |
| `NumberOfBytes`             | `udp-tracker-core`, `udp-tracker-server`, `tracker-client` (console), and others                    |
| `NumberOfPeers`             | `udp-tracker-core`, `udp-tracker-server`, `tracker-client` (console), and others                    |
| `NumberOfDownloads`         | `udp-tracker-core`, `udp-tracker-server`, and others                                                |
| `TorrentScrapeStatistics`   | `udp-tracker-core`, `udp-tracker-server`, `tracker-client` (console), and others                    |
| `Ipv4AddrBytes`             | `udp-tracker-core`, `udp-tracker-server`                                                            |
| `Ipv6AddrBytes`             | `udp-tracker-core`, `udp-tracker-server`                                                            |
| `RequestParseError`         | `udp-tracker-core`, `udp-tracker-server`                                                            |
| `ResponsePeer`              | `udp-tracker-core`, `udp-tracker-server`                                                            |

#### Internal-only types

`AnnounceEventBytes` is not exported from the fork's public API and has no uses outside the fork.
It exists solely as an intermediate wire-format representation inside `AnnounceRequest`
(a `#[repr(C, packed)]` struct used during zero-copy deserialization). Removing it would break
the deserialization logic for `AnnounceRequest`. It cannot be removed.

#### Feature flags

The upstream crate has no optional feature flags. No feature stripping is possible.

#### Conclusion

Every public type exported by `packages/aquatic-udp-protocol` is used by at least one other
workspace package. The only internal-only item (`AnnounceEventBytes`) is structurally required
and cannot be removed. **There is no dead code to strip.**

---

### `packages/aquatic-peer-id`

#### Public types used outside the fork

| Type         | Used by                                                                                                      |
| ------------ | ------------------------------------------------------------------------------------------------------------ |
| `PeerId`     | Re-exported through `aquatic-udp-protocol`; used by `udp-protocol`, `tracker-core`, `primitives`, and others |
| `PeerClient` | `udp-tracker-core`                                                                                           |

#### Feature flags

The upstream crate exposed an optional `quickcheck` feature (for property-based testing helpers).
At the time of this analysis in the original migration, the feature was retained to preserve
upstream test-oriented behavior rather than to optimize release dependency footprint.

#### Conclusion

Both public types (`PeerId`, `PeerClient`) are actively used in the workspace. **There is no dead
code to strip.**

---

## Overall Conclusion

After a thorough search of all 26 source files across 13 packages that depend on the two forks,
**no unused public types, functions, or feature-enabled code paths were found** that could be
safely removed at this stage. Step 2 is complete with no changes to the fork source.

The migration continues at Step 3: upgrading `zerocopy` from 0.7 to 0.8.
