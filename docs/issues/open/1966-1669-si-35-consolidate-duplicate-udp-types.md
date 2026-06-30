---
doc-type: issue
issue-type: task
status: planned
priority: p2
github-issue: 1966
spec-path: docs/issues/open/1966-1669-si-35-consolidate-duplicate-udp-types.md
branch: "1966-1669-si-35-consolidate-duplicate-udp-types"
related-pr: null
last-updated-utc: 2026-06-30 12:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/open/1669-overhaul-packages/EPIC.md
    - docs/issues/open/1669-overhaul-packages/DECISIONS.md
    - packages/udp-protocol/src/
    - packages/udp-core/src/event.rs
    - packages/udp-server/src/event.rs
    - packages/udp-server/src/lib.rs
    - packages/tracker-client/src/udp/mod.rs
    - docs/adrs/20260527175600_keep_protocol_and_domain_types_decoupled.md
    - packages/primitives/src/announce.rs
    - packages/http-protocol/src/v1/requests/announce.rs
    - packages/http-protocol/src/v1/responses/announce.rs
    - packages/http-protocol/src/v1/responses/scrape.rs
---

<!-- skill-link: create-issue -->

# Issue #1966 - EPIC 1669 SI-35: Consolidate Duplicate UDP Types

> **Parent EPIC**: [#1669 — Overhaul: Packages](https://github.com/torrust/torrust-tracker/issues/1669)
> **EPIC Reference**: `docs/issues/open/1669-overhaul-packages/EPIC.md`

## Goal

Eliminate duplicate type definitions and constants in the UDP tracker packages by consolidating
them into their canonical locations.

## Background

A workspace-wide audit of UDP-related packages found that the UDP layer is significantly cleaner
than the HTTP layer — the core protocol types (`ConnectRequest`, `ConnectResponse`,
`AnnounceRequest`, `AnnounceResponse`, `ScrapeRequest`, `ScrapeResponse`, `Request`, `Response`,
`ErrorResponse`, `ResponsePeer`, `TorrentScrapeStatistics`) are defined exclusively in
`packages/udp-protocol/src/` and imported everywhere else. This is the correct architecture.

However, three duplications were found:

### 🔴 `ConnectionContext` — full copy-paste

The struct and its entire `impl` block are duplicated between:

|                                            | `packages/udp-core/src/event.rs` (line 26)                                                                        | `packages/udp-server/src/event.rs` (line 85)                                                                      |
| ------------------------------------------ | ----------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------- |
| **Fields**                                 | `pub client_socket_addr: SocketAddr`, `pub server_service_binding: ServiceBinding`                                | `client_socket_addr: SocketAddr` (private), `server_service_binding: ServiceBinding` (private)                    |
| **Methods**                                | `new()`, `client_socket_addr()`, `server_socket_addr()`, `client_address_ip_family()`, `client_address_ip_type()` | `new()`, `client_socket_addr()`, `server_socket_addr()`, `client_address_ip_family()`, `client_address_ip_type()` |
| **Derive**                                 | `Debug, PartialEq, Eq, Clone`                                                                                     | `Debug, PartialEq, Eq, Clone`                                                                                     |
| **`From<ConnectionContext> for LabelSet`** | Yes                                                                                                               | Yes                                                                                                               |

The only difference is field visibility (`pub` in core, private in server). The impl blocks are
identical. One should be the canonical definition and the other should import it.

### 🟡 `MAX_PACKET_SIZE` — same constant, two locations

| Package                                            | File                                       | Value  |
| -------------------------------------------------- | ------------------------------------------ | ------ |
| `packages/udp-server/src/lib.rs` (line 651)        | `pub const MAX_PACKET_SIZE: usize = 1496;` | `1496` |
| `packages/tracker-client/src/udp/mod.rs` (line 11) | `pub const MAX_PACKET_SIZE: usize = 1496;` | `1496` |

The `tracker-client` already depends on `udp-protocol`. This constant could live in
`udp-protocol` and be shared by both consumers.

### 🟡 `PROTOCOL_ID` — dead code copy

| Package                                            | Symbol                | Value               | Visibility   |
| -------------------------------------------------- | --------------------- | ------------------- | ------------ |
| `packages/udp-protocol/src/connect.rs` (line 15)   | `PROTOCOL_IDENTIFIER` | `4_497_486_125_440` | `pub(crate)` |
| `packages/tracker-client/src/udp/mod.rs` (line 14) | `PROTOCOL_ID`         | `0x0417_2710_1980`  | `pub`        |

Same magic constant with different names. `PROTOCOL_ID` in `tracker-client` is **unused** — a
grep shows no references to it anywhere. It should be removed.

### 🟢 Intentional duplications (not in scope)

The following are kept separate per
[ADR 20260527175600](docs/adrs/20260527175600_keep_protocol_and_domain_types_decoupled.md)
and are **not** addressed by this issue:

- `AnnounceEvent` — `udp-protocol` vs `primitives` (wire type vs domain type)
- `InfoHash` — `udp-protocol` vs `torrust_info_hash` (wire type vs domain type)
- `NumberOfBytes` — `udp-protocol` vs `primitives` vs `http-protocol` (wire type vs domain type)

These types currently have comments like `// Intentionally kept in...` or `// Intentional boundary duplication` but
do not explicitly reference the ADR. As part of this issue, each location will gain a `// adr:` comment so
future contributors understand the architectural reasoning and do not accidentally re-couple the types.

**Code locations to annotate**:

- `packages/udp-protocol/src/common.rs` — `InfoHash` (line 20) and `NumberOfBytes` (line 46)
- `packages/http-protocol/src/v1/requests/announce.rs` — `NumberOfBytes` (line 28)
- `packages/http-protocol/src/v1/responses/announce.rs` — `Announce` DTO (line 11)
- `packages/http-protocol/src/v1/responses/scrape.rs` — scrape response DTOs (lines 10, 20)
- `packages/primitives/src/announce.rs` — `AnnounceEvent` (line 91)

## Scope

### In Scope

- Consolidate `ConnectionContext` into a single canonical definition (likely in `udp-core`)
- Move `MAX_PACKET_SIZE` to `udp-protocol` and import it in both `udp-server` and `tracker-client`
- Remove the unused `PROTOCOL_ID` constant from `tracker-client`
- Add `adr:` comments to the code locations listed under "Intentional duplications" referencing
  ADR `docs/adrs/20260527175600_keep_protocol_and_domain_types_decoupled.md`, so future
  contributors understand why the duplication exists and do not accidentally re-couple the types
- Verify all tests pass and no functionality regresses

### Out of Scope

- Merging protocol-level types (`AnnounceEvent`, `InfoHash`, `NumberOfBytes`) — governed by ADR
- Changing the public API of `udp-protocol` beyond what's needed for consolidation
- Refactoring the UDP server architecture

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                             | Notes / Expected Output                                                                                                                                                                                                                                            |
| --- | ------ | ------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| T1  | TODO   | Consolidate `ConnectionContext` into `udp-core`  | Make `udp-server` import from `udp-core` instead of defining its own copy                                                                                                                                                                                          |
| T2  | TODO   | Move `MAX_PACKET_SIZE` to `udp-protocol`         | Add `pub const MAX_PACKET_SIZE: usize = 1496;` to `udp-protocol`; update imports                                                                                                                                                                                   |
| T3  | TODO   | Remove dead `PROTOCOL_ID` from `tracker-client`  | Delete the unused constant                                                                                                                                                                                                                                         |
| T4  | TODO   | Add `adr:` comments for intentional duplications | Annotate `udp-protocol/src/common.rs`, `http-protocol/src/v1/requests/announce.rs`, `http-protocol/src/v1/responses/announce.rs`, `http-protocol/src/v1/responses/scrape.rs`, and `primitives/src/announce.rs` with `// adr: docs/adrs/20260527175600...` comments |
| T5  | TODO   | Run full verification                            | `linter all`, `cargo test --workspace`, pre-commit, pre-push                                                                                                                                                                                                       |

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted in `docs/issues/drafts/`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] GitHub issue created and issue number added to this spec
- [ ] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded (status + evidence)
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-06-30 12:00 UTC - Copilot - Spec draft created

## Acceptance Criteria

- [ ] AC1: `ConnectionContext` is defined in exactly one location (imported by the other)
- [ ] AC2: `MAX_PACKET_SIZE` is defined in `udp-protocol` and imported by both `udp-server` and `tracker-client`
- [ ] AC3: `PROTOCOL_ID` no longer exists in `tracker-client`
- [ ] AC4: Each location listed in the "Intentional duplications" section has an `adr:` comment referencing the ADR
- [ ] AC5: All existing tests pass (`cargo test --workspace`)
- [ ] AC5: `linter all` exits with code `0`
- [ ] AC6: Pre-commit and pre-push checks pass
- [ ] Manual verification scenarios are executed and documented (status + evidence)
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test --workspace`
- Pre-commit checks (`./contrib/dev-tools/git/hooks/pre-commit.sh`)
- Pre-push checks (`./contrib/dev-tools/git/hooks/pre-push.sh`)
- `cargo machete` (no unused dependencies introduced)

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                                       | Command/Steps                                                                           | Expected Result                           | Status | Evidence                     |
| --- | ---------------------------------------------- | --------------------------------------------------------------------------------------- | ----------------------------------------- | ------ | ---------------------------- |
| M1  | UDP tracker announces work with tracker-client | Run `tracker_client udp announce` against a local tracker; verify request/response flow | Same behavior as before the consolidation | TODO   | {log/output/screenshot/path} |
| M2  | UDP scrape works with tracker-client           | Run `tracker_client udp scrape` against a local tracker                                 | Same behavior as before                   | TODO   | {log/output/screenshot/path} |
| M3  | udp-server tests pass                          | `cargo test -p torrust-tracker-udp-server`                                              | All tests pass                            | TODO   | {log/output/screenshot/path} |
| M4  | No duplicate definitions remain                | `grep` for `ConnectionContext` and `MAX_PACKET_SIZE` across workspace                   | Only one definition each                  | TODO   | {log/output/screenshot/path} |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence           |
| ----- | ---------------------- | ------------------ |
| AC1   | TODO                   | {test/log/PR link} |
| AC2   | TODO                   | {test/log/PR link} |
| AC3   | TODO                   | {test/log/PR link} |
| AC4   | TODO                   | {test/log/PR link} |
| AC5   | TODO                   | {test/log/PR link} |
| AC6   | TODO                   | {test/log/PR link} |
| AC7   | TODO                   | {test/log/PR link} |

## Risks and Trade-offs

- **Risk**: `ConnectionContext` has different field visibility (`pub` in core, private in server).
  **Mitigation**: The consolidated definition should use `pub` fields (or provide accessor methods)
  so both consumers can use it without friction.
- **Risk**: Moving `MAX_PACKET_SIZE` to `udp-protocol` changes its visibility scope.
  **Mitigation**: Make it `pub` in `udp-protocol`; both consumers already depend on it.
- **Risk**: Removing `PROTOCOL_ID` could break something if it's used via macro or build script.
  **Mitigation**: The grep confirmed zero references; removal is safe.

## References

- Parent EPIC: [#1669](https://github.com/torrust/torrust-tracker/issues/1669)
- EPIC spec: `docs/issues/open/1669-overhaul-packages/EPIC.md`
- Decisions log: `docs/issues/open/1669-overhaul-packages/DECISIONS.md`
- Duplicate analysis: exploration performed 2026-06-30 by Copilot
- Related ADR: `docs/adrs/20260527175600_keep_protocol_and_domain_types_decoupled.md`
- Related HTTP consolidation issue: `docs/issues/drafts/1669-si-34-consolidate-duplicate-http-types-into-http-protocol.md`
