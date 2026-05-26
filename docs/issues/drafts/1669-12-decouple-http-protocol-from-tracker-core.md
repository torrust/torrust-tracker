---
doc-type: issue
issue-type: task
status: draft
priority: p1
github-issue: null
spec-path: docs/issues/drafts/1669-12-decouple-http-protocol-from-tracker-core.md
branch: null
related-pr: null
last-updated-utc: 2026-05-26 00:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/open/1669-overhaul-packages/EPIC.md
    - packages/http-protocol/Cargo.toml
    - packages/http-protocol/src/v1/responses/error.rs
    - packages/http-tracker-core/src/services/announce.rs
    - packages/http-tracker-core/src/services/scrape.rs
    - packages/axum-http-tracker-server/src/v1/handlers/announce.rs
    - packages/axum-http-tracker-server/src/v1/handlers/scrape.rs
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Decouple `http-protocol` from `tracker-core`

Subissue ID: SI-12 (1669-12).

## Goal

Remove the forbidden layer edge `protocol -> tracker-core` by eliminating the
`bittorrent-tracker-core` dependency from `packages/http-protocol`.

This draft is intentionally the first step of a two-step cleanup strategy:

1. Remove forbidden dependency edges with minimal behavior change.
2. Follow with explicit protocol-vs-domain type separation where needed.

This is a subissue of EPIC [#1669](../open/1669-overhaul-packages/EPIC.md).

## Layer Impact Summary

Current edge:

- `http-protocol (protocol layer) -> tracker-core (tracker-core layer)`

Why this is a violation:

- EPIC layer guardrails define `protocol -> tracker-core` as forbidden.
- Protocol crates should contain BEP-defined parsing/encoding only.

Target edge:

- Remove `http-protocol -> tracker-core`.
- Keep tracker-core error mapping in higher layers (`http-tracker-core` and/or
  `axum-http-tracker-server`) where service/domain errors are already handled.

Two-step intent for this subissue:

- This issue performs step 1 only (edge removal and boundary mapping move).
- Any broader type-model cleanup is deferred to a dedicated follow-up so this
  change remains small and low-risk.

## Concrete Dependency Evidence

Manifest-level dependency:

- `packages/http-protocol/Cargo.toml`: `bittorrent-tracker-core = { ... path = "../tracker-core" }`

Symbol-level usage inside protocol:

- `packages/http-protocol/src/v1/responses/error.rs`
  - `impl From<bittorrent_tracker_core::error::AnnounceError> for Error`
  - `impl From<bittorrent_tracker_core::error::ScrapeError> for Error`
  - `impl From<bittorrent_tracker_core::error::WhitelistError> for Error`
  - `impl From<bittorrent_tracker_core::authentication::Error> for Error`

Usage purpose:

- The dependency is used only for stringification/mapping of tracker-core errors
  into HTTP failure reason strings.

## Scope

### In Scope

- Remove tracker-core error conversion implementations from
  `http-protocol` response error module.
- Remove `bittorrent-tracker-core` from `packages/http-protocol/Cargo.toml`.
- Introduce/adjust mapping in higher layer(s) to keep the same HTTP failure
  reason behavior.
- Update tests impacted by the mapping move.
- Update EPIC dependency analysis notes if needed.

### Out of Scope

- Decoupling `http-protocol` from `udp-protocol`.
- Decoupling `http-protocol` from `torrust-tracker-primitives`.
- Any BEP behavior changes in protocol parsing or response formatting.
- Full protocol/domain model split for error types (follow-up issue).

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                                                                                                      | Notes / Expected Output                                                                                                                  |
| --- | ------ | ----------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | TODO   | Confirm all tracker-core usage in `http-protocol` is limited to `responses/error.rs`                                                      | Evidence captured in PR description                                                                                                      |
| T2  | TODO   | Remove `From<tracker-core error>` impls from `packages/http-protocol/src/v1/responses/error.rs`                                           | No direct imports or type references to `bittorrent_tracker_core::*` remain                                                              |
| T3  | TODO   | Remove `bittorrent-tracker-core` from `packages/http-protocol/Cargo.toml`                                                                 | `cargo metadata` shows no `http-protocol -> tracker-core` edge                                                                           |
| T4  | TODO   | Add/adjust mapping at higher layer (`http-tracker-core` and/or `axum-http-tracker-server`) for equivalent client-visible failure messages | Existing behavior preserved for announce/scrape/auth/whitelist errors                                                                    |
| T5  | TODO   | Update or add tests for failure mapping behavior                                                                                          | Coverage in affected crates; assertions on error message fragments                                                                       |
| T6  | TODO   | Run verification commands                                                                                                                 | Build/tests/lints pass                                                                                                                   |
| T7  | TODO   | Update EPIC tracking rows and draft list as needed                                                                                        | Active Subissues remain consistent                                                                                                       |
| T8  | TODO   | Update EPIC after implementation                                                                                                          | Update Active Subissues progress and EPIC sections: Package Inventory, Desired Package State, Torrust Dependency Lists (Direct, Non-dev) |

## Acceptance Criteria

- [ ] `packages/http-protocol/Cargo.toml` has no `bittorrent-tracker-core` dependency.
- [ ] `packages/http-protocol` has no source-level references to `bittorrent_tracker_core::`.
- [ ] Client-visible HTTP error responses still include meaningful failure reasons
      for announce/scrape/auth/whitelist failures.
- [ ] `cargo build --workspace` passes.
- [ ] Relevant tests in HTTP protocol/core/server packages pass.
- [ ] `linter all` exits with code `0`.
- [ ] EPIC tracking is updated to include this subissue.

## Verification Plan

### Automatic Checks

- `cargo build --workspace`
- `cargo test -p bittorrent-http-tracker-protocol`
- `cargo test -p bittorrent-http-tracker-core`
- `cargo test -p torrust-tracker-axum-http-server`
- `linter all`

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                                   | Command / Steps                                              | Expected Result                                        | Status | Evidence |
| --- | ------------------------------------------ | ------------------------------------------------------------ | ------------------------------------------------------ | ------ | -------- |
| M1  | No forbidden edge remains                  | `cargo tree -p bittorrent-http-tracker-protocol --depth 1`   | No dependency on `bittorrent-tracker-core`             | TODO   |          |
| M2  | No tracker-core symbols in protocol source | `rg "bittorrent_tracker_core::" packages/http-protocol`      | No matches                                             | TODO   |          |
| M3  | Error mapping behavior preserved           | Trigger announce/scrape/auth failure cases in existing tests | Error responses still include expected message context | TODO   |          |

## Risks and Trade-offs

- Error text may change slightly when mapping logic moves. Keep message semantics,
  not exact punctuation, unless tests require exact matching.
- If mapping is duplicated in multiple layers, a follow-up refactor may be needed
  to centralize shared conversion helpers.

## Follow-up

- Open a dedicated follow-up subissue to separate protocol-layer error models
  from tracker-domain error models, keeping mapping strictly at layer boundaries.

## References

- EPIC: [docs/issues/open/1669-overhaul-packages/EPIC.md](../open/1669-overhaul-packages/EPIC.md)
- Protocol error mapping: [packages/http-protocol/src/v1/responses/error.rs](../../packages/http-protocol/src/v1/responses/error.rs)
- HTTP core announce service: [packages/http-tracker-core/src/services/announce.rs](../../packages/http-tracker-core/src/services/announce.rs)
- HTTP core scrape service: [packages/http-tracker-core/src/services/scrape.rs](../../packages/http-tracker-core/src/services/scrape.rs)
- Axum announce handler: [packages/axum-http-tracker-server/src/v1/handlers/announce.rs](../../packages/axum-http-tracker-server/src/v1/handlers/announce.rs)
- Axum scrape handler: [packages/axum-http-tracker-server/src/v1/handlers/scrape.rs](../../packages/axum-http-tracker-server/src/v1/handlers/scrape.rs)
