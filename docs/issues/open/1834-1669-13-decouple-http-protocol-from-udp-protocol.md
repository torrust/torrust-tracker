---
doc-type: issue
issue-type: task
status: planned
priority: p1
github-issue: 1834
spec-path: docs/issues/open/1834-1669-13-decouple-http-protocol-from-udp-protocol.md
branch: null
related-pr: null
last-updated-utc: 2026-05-27 00:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/open/1669-overhaul-packages/EPIC.md
    - packages/http-protocol/Cargo.toml
    - packages/http-protocol/src/v1/requests/announce.rs
    - packages/primitives/src/announce.rs
---

<!-- skill-link: create-issue -->

# Issue #1834 - Decouple `http-protocol` from `udp-protocol`

Subissue ID: SI-13 (1669-13).

## Goal

Remove the cross-protocol dependency edge `http-protocol -> udp-protocol` by
eliminating the `torrust-tracker-udp-tracker-protocol` dependency from
`packages/http-protocol`.

This draft is intentionally step 1 of a two-step cleanup strategy:

1. Remove concrete forbidden/smelly edges with minimal behavior change.
2. Follow with explicit protocol-level vs domain-level type separation.

This is a subissue of EPIC [#1669](1669-overhaul-packages/EPIC.md).

## Layer Impact Summary

Current edge:

- `http-protocol (protocol layer) -> udp-protocol (protocol layer)`

Why this is a smell:

- Even though both are protocol-layer crates, this creates protocol-to-protocol
  coupling between BEP 3/23 HTTP concerns and BEP 15 UDP concerns.
- It makes extraction/reuse of HTTP protocol logic depend on UDP package details.

Target edge:

- Remove `http-protocol -> udp-protocol`.
- Keep event conversions anchored on local HTTP event types and shared domain
  event types (`torrust-tracker-primitives::AnnounceEvent`) rather than UDP types.

Two-step intent for this subissue:

- This issue performs edge cleanup only.
- A later follow-up should remove protocol dependency on tracker-domain event
  types as well, by introducing/using protocol-owned event DTOs and boundary
  mapping in higher layers.

## Concrete Dependency Evidence

Manifest-level dependency:

- `packages/http-protocol/Cargo.toml`: `torrust-tracker-udp-tracker-protocol = { ... path = "../udp-protocol" }`

Symbol-level usage inside protocol:

- `packages/http-protocol/src/v1/requests/announce.rs`
  - `impl From<torrust_tracker_udp_tracker_protocol::AnnounceEvent> for Event`
  - Match arms on `Started`, `Stopped`, `Completed`, `None`

Additional context:

- `http-protocol` already defines conversion to/from
  `torrust_tracker_primitives::AnnounceEvent` in the same file.
- The current UDP dependency is therefore concentrated in one conversion impl.

## Scope

### In Scope

- Remove `From<torrust_tracker_udp_tracker_protocol::AnnounceEvent> for Event` in
  `packages/http-protocol/src/v1/requests/announce.rs`.
- Remove `torrust-tracker-udp-tracker-protocol` from
  `packages/http-protocol/Cargo.toml`.
- Adjust tests and call sites (if any) to use local `Event` or
  `torrust-tracker-primitives::AnnounceEvent` conversions.
- Update EPIC tracking references if needed.

### Out of Scope

- Decoupling `http-protocol` from `tracker-core`.
- Decoupling `http-protocol` from `torrust-tracker-primitives`.
- Any protocol behavior changes beyond dependency cleanup.
- Full protocol/domain event type split (follow-up issue).

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                                                                   | Notes / Expected Output                                                                                                                  |
| --- | ------ | ------------------------------------------------------------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | TODO   | Confirm all UDP protocol usage in `http-protocol` is limited to one conversion impl                    | Evidence recorded in PR description                                                                                                      |
| T2  | TODO   | Remove UDP `AnnounceEvent` conversion impl from `packages/http-protocol/src/v1/requests/announce.rs`   | No direct references to `torrust_tracker_udp_tracker_protocol::` remain                                                                   |
| T3  | TODO   | Remove `torrust-tracker-udp-tracker-protocol` from `packages/http-protocol/Cargo.toml`                | `cargo tree -p torrust-tracker-http-tracker-protocol --depth 1` shows no UDP protocol edge                                               |
| T4  | TODO   | Update tests to use supported conversion paths (`Event <-> torrust-tracker-primitives::AnnounceEvent`) | Tests compile and pass without UDP protocol types                                                                                        |
| T5  | TODO   | Run verification commands                                                                              | Build/tests/lints pass                                                                                                                   |
| T6  | TODO   | Update EPIC tracking rows and draft list as needed                                                     | Active Subissues remain consistent                                                                                                       |
| T7  | TODO   | Update EPIC after implementation                                                                       | Update Active Subissues progress and EPIC sections: Package Inventory, Desired Package State, Torrust Dependency Lists (Direct, Non-dev) |

## Acceptance Criteria

- [ ] `packages/http-protocol/Cargo.toml` has no `torrust-tracker-udp-tracker-protocol` dependency.
- [ ] `packages/http-protocol` has no source-level references to
      `bittorrent_udp_tracker_protocol::`.
- [ ] HTTP protocol announce event behavior remains unchanged for
      `started/stopped/completed/none` mappings.
- [ ] `cargo build --workspace` passes.
- [ ] `cargo test -p torrust-tracker-http-tracker-protocol` passes.
- [ ] `linter all` exits with code `0`.
- [ ] EPIC tracking includes this subissue.

## Verification Plan

### Automatic Checks

- `cargo build --workspace`
- `cargo test -p torrust-tracker-http-tracker-protocol`
- `cargo test -p torrust-tracker-http-tracker-core`
- `cargo test -p torrust-tracker-axum-http-server`
- `linter all`

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                               | Command / Steps                                                 | Expected Result                                              | Status | Evidence |
| --- | -------------------------------------- | --------------------------------------------------------------- | ------------------------------------------------------------ | ------ | -------- |
| M1  | No cross-protocol edge remains         | `cargo tree -p torrust-tracker-http-tracker-protocol --depth 1` | No dependency on `torrust-tracker-udp-tracker-protocol`      | TODO   |          |
| M2  | No UDP symbols in HTTP protocol source | `rg "torrust_tracker_udp_tracker_protocol::" packages/http-protocol` | No matches                                                   | TODO   |          |
| M3  | Event conversion behavior preserved    | Run existing announce request parsing/unit tests                | Mappings for `started/stopped/completed/none` remain correct | TODO   |          |

## Risks and Trade-offs

- Some tests may implicitly rely on UDP types for fixtures. If so, update them
  to use protocol-local event types or tracker-primitives events.
- If another hidden UDP usage appears, this issue may need to include a small
  compatibility helper in a higher layer.

## Follow-up

- Open a dedicated follow-up subissue to remove
  `http-protocol -> torrust-tracker-primitives` event coupling by separating
  protocol-level event models from tracker-domain event models and mapping at
  boundary layers.

## References

- EPIC: [docs/issues/open/1669-overhaul-packages/EPIC.md](1669-overhaul-packages/EPIC.md)
- HTTP protocol announce request: [packages/http-protocol/src/v1/requests/announce.rs](../../packages/http-protocol/src/v1/requests/announce.rs)
- HTTP protocol manifest: [packages/http-protocol/Cargo.toml](../../packages/http-protocol/Cargo.toml)
- Shared announce event type: [packages/primitives/src/announce.rs](../../packages/primitives/src/announce.rs)
