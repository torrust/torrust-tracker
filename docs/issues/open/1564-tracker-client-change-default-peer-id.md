# Issue #1564 — Tracker Client: Change the Default `PeerId` Used in Clients

## Overview

The default `PeerId` used in all tracker client requests is `b"-qB00000000000000001"`.
The prefix `-qB` is the registered [Azureus-style](https://www.bittorrent.org/beps/bep_0020.html)
client identifier for [qBittorrent](https://www.qbittorrent.org/). Using another client's
registered prefix is incorrect — it misrepresents the Torrust tooling as qBittorrent traffic.

The goal is to register and use a Torrust-specific prefix so that requests sent by the
Torrust Tracker client (both in production tooling and in test code) are clearly
identifiable.

- GitHub issue: <https://github.com/torrust/torrust-tracker/issues/1564>
- Parent EPIC: <https://github.com/torrust/torrust-tracker/issues/669>
- BEP 20 (peer ID conventions): <https://www.bittorrent.org/beps/bep_0020.html>
- BitTorrent peer_id spec: <https://wiki.theory.org/BitTorrentSpecification#peer_id>

## Background

The Azureus-style peer ID format is:

```text
-<CC><VVVV>-<random-12-bytes>
```

Where `CC` is a two-character client identifier and `VVVV` is a four-character version string.

The current default is:

```rust
peer_id: PeerId(*b"-qB00000000000000001").0,
```

This is the qBittorrent prefix (`qB`). The Torrust Tracker project needs its own identifier.

Proposed candidates:

- `-RC` — Rust Client (for the current Torrust Tracker REST/checker client)
- `-TC` — Torrust Client (if/when a full Torrust BitTorrent client ships)

The GitHub issue suggests `-RC` for now and reserves `-TC` for a future full BitTorrent client.
A concrete example from the issue: `b"-RC53070047639607806"` (the last 12 bytes are random).

## Current Behaviour

The literal `b"-qB00000000000000001"` appears in several places:

| File                                                           | Context                                             |
| -------------------------------------------------------------- | --------------------------------------------------- |
| `packages/tracker-client/src/http/client/requests/announce.rs` | `QueryBuilder::with_default_values()` — HTTP client |
| `console/tracker-client/src/console/clients/udp/checker.rs`    | UDP checker default peer ID                         |
| `packages/http-protocol/src/v1/requests/announce.rs`           | Protocol test fixtures                              |
| `packages/http-protocol/src/v1/responses/announce.rs`          | Protocol test fixtures                              |
| `packages/http-protocol/src/v1/query.rs`                       | Protocol test fixtures                              |
| `src/lib.rs`                                                   | Library doc example URL                             |

## Proposed Behaviour

1. Define a named constant for the Torrust client default `PeerId` in a shared location
   (e.g. `packages/tracker-client/src/`) so all uses reference a single source of truth.

2. Change the default value to a Torrust-specific prefix using `RC` (approved by maintainer),
   with version bytes that reflect the client version. For current v3.0.0, use `3000`.
   Version bytes are hard-coded per release for now.

   Example test default:

   ```rust
   pub const DEFAULT_TEST_PEER_ID: PeerId = PeerId(*b"-RC3000-000000000001");
   ```

3. Use deterministic peer ID values in tests and fixtures, but use a random suffix for production
   defaults while preserving the Azureus-style structure and version bytes.
   The production random suffix is generated once per process run.

4. Update all call sites that hard-code `b"-qB00000000000000001"` to use the new convention
   or an equivalent Torrust-prefixed value.

5. Test fixtures that hard-code `-qB...` for protocol-level assertions should use a clearly named
   local test constant following the convention, without introducing cross-package constant
   coupling.

6. Add an ADR documenting the PeerId convention for Torrust client defaults and test fixtures.

## Goals

- [ ] Replace all hard-coded `b"-qB00000000000000001"` peer IDs with a Torrust-specific prefix
- [ ] Define tracker-client constants for deterministic test PeerId and production default generation
- [ ] Update all affected test fixtures so protocol-level tests still pass
- [ ] Add ADR documenting the PeerId convention for production and tests
- [ ] Version bytes are hard-coded per release in tracker-client defaults
- [ ] Production default PeerId suffix is generated once per process run
- [ ] `linter all` exits with code `0`
- [ ] `cargo machete` reports no unused dependencies
- [ ] Existing tests pass

## Implementation Plan

### Task 1: Choose and define the constant

In `packages/tracker-client/src/` (or the appropriate shared module), define:

```rust
/// Default deterministic Peer ID used in tests and fixtures.
///
/// Uses the Azureus-style format: `-<CC><VVVV>-<random-12-bytes>`.
/// Prefix `RC` stands for "Rust Client".
pub const DEFAULT_TEST_PEER_ID_BYTES: &[u8; 20] = b"-RC3000-000000000001";
```

Also define a helper for production defaults that keeps prefix/version but randomizes suffix.
Use per-process generation (generate once and reuse during process lifetime).

### Task 2: Update `QueryBuilder::with_default_values`

In `packages/tracker-client/src/http/client/requests/announce.rs`:

```rust
peer_id: make_default_production_peer_id().0,
```

### Task 3: Update the UDP checker default

In `console/tracker-client/src/console/clients/udp/checker.rs`:

```rust
peer_id: params.peer_id.map_or(make_default_production_peer_id(), PeerId),
```

### Task 4: Update protocol test fixtures

In `packages/http-protocol/src/v1/requests/announce.rs`,
`packages/http-protocol/src/v1/responses/announce.rs`, and
`packages/http-protocol/src/v1/query.rs`:

Replace the literal `-qB00000000000000001` bytes in test data with the new convention value
or with an explicit local test constant.

> **Note**: Keep packages decoupled. Protocol packages should not import tracker-client constants;
> duplicate the same convention value in local test constants where needed.

### Task 5: Update doc examples

In `src/lib.rs`, update the example announce URL that contains the old peer ID.

### Task 6: Add ADR for PeerId convention

Create an ADR under `docs/adrs/` documenting:

- Approved prefix (`RC`) and rationale
- Version field convention (e.g. `3000` for v3.0.0)
- Version source policy: hard-coded per release for now
- Deterministic test fixtures vs randomized production suffix
- Production random suffix lifecycle: generated once per process run
- Cross-repository convention and package-decoupling rule

## Acceptance Criteria

- [ ] AC1: `b"-qB00000000000000001"` no longer appears as a default in any client or checker code
- [ ] AC2: Tracker-client defines deterministic test PeerId constant(s) and production default generation helper
- [ ] AC3: The HTTP and UDP clients use `RC` + versioned prefix for production default requests
- [ ] AC4: Protocol fixtures adopt the new convention without creating cross-package coupling
- [ ] AC5: ADR for PeerId convention is added under `docs/adrs/`
- [ ] AC6: Version bytes are hard-coded per release in tracker-client defaults
- [ ] AC7: Production random suffix is generated once per process run
- [ ] AC8: All tests that assert on default PeerId behavior pass with the new convention
- [ ] AC9: `linter all` exits with code `0`
- [ ] AC10: `cargo machete` reports no unused dependencies

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | TODO                   |          |
| AC2   | TODO                   |          |
| AC3   | TODO                   |          |
| AC4   | TODO                   |          |
| AC5   | TODO                   |          |
| AC6   | TODO                   |          |
| AC7   | TODO                   |          |
| AC8   | TODO                   |          |
| AC9   | TODO                   |          |
| AC10  | TODO                   |          |

## Risks and Trade-offs

- **Test fixture churn**: Many tests hard-code the qBittorrent peer ID as part of expected
  byte payloads. Changing the default requires updating those fixtures carefully to avoid
  accidentally masking regressions.
- **External compatibility**: The default peer ID is only used by Torrust tooling (client
  binaries and checker). It is not a protocol compatibility concern. Changing it will not
  break interoperability with any tracker.

## Metadata

| Field              | Value                                                            |
| ------------------ | ---------------------------------------------------------------- |
| Type               | Enhancement                                                      |
| Status             | Planned                                                          |
| Priority           | P3                                                               |
| GitHub Issue       | [#1564](https://github.com/torrust/torrust-tracker/issues/1564)  |
| Spec Path          | `docs/issues/open/1564-tracker-client-change-default-peer-id.md` |
| Branch             | `1564-tracker-client-change-default-peer-id`                     |
| Related PR         | To be assigned                                                   |
| Last Updated (UTC) | 2026-05-12 08:00                                                 |

## Progress Tracking

### Workflow Checkpoints

- [ ] Spec drafted in `docs/issues/open/`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] Implementation completed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-05-11 20:00 UTC - Agent - Spec created from GitHub issue #1564 content
- 2026-05-12 00:00 UTC - Agent - Incorporated maintainer decisions: use RC prefix, versioned bytes, deterministic tests + randomized production suffix, tracker-client constant location, no cross-package coupling, add ADR
- 2026-05-12 08:00 UTC - Agent - Incorporated answered follow-ups: hard-coded per-release version bytes and per-process production random suffix lifecycle

## Open Questions

No open questions at this time.

## References

- Parent EPIC: <https://github.com/torrust/torrust-tracker/issues/669>
- BEP 20 — Peer ID Conventions: <https://www.bittorrent.org/beps/bep_0020.html>
- BitTorrent Specification — peer_id: <https://wiki.theory.org/BitTorrentSpecification#peer_id>
