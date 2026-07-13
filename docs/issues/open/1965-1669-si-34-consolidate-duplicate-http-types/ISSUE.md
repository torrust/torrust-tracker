---
doc-type: issue
issue-type: task
status: planned
priority: p1
github-issue: 1965
spec-path: docs/issues/open/1965-1669-si-34-consolidate-duplicate-http-types/ISSUE.md
issue-folder: docs/issues/open/1965-1669-si-34-consolidate-duplicate-http-types/
branch: "1965-1669-si-34-consolidate-duplicate-http-types"
related-pr: "https://github.com/torrust/torrust-tracker/pull/1974"
last-updated-utc: 2026-07-13 12:00
semantic-links:
  skill-links:
    - create-issue
    - run-tracker-locally
  related-artifacts:
    - docs/issues/open/1669-overhaul-packages/EPIC.md
    - docs/issues/open/1669-overhaul-packages/DECISIONS.md
    - packages/http-protocol/src/v1/requests/
    - packages/http-protocol/src/v1/responses/
    - packages/axum-http-server/tests/server/requests/
    - packages/axum-http-server/tests/server/responses/
    - packages/tracker-client/src/http/client/requests/
    - packages/tracker-client/src/http/client/responses/
    - .github/skills/dev/environment-setup/run-tracker-locally/SKILL.md
---

<!-- skill-link: create-issue -->

# Issue #1965 - EPIC 1669 SI-34: Consolidate Duplicate HTTP Types into `http-protocol`

> **Parent EPIC**: [#1669 — Overhaul: Packages](https://github.com/torrust/torrust-tracker/issues/1669)
> **EPIC Reference**: `docs/issues/open/1669-overhaul-packages/EPIC.md`
>
> **Issue type**: Folder issue — manual verification evidence and command logs will be documented
> in a separate `manual-verification.md` file alongside this spec inside the issue folder at
> `docs/issues/open/1965-1669-si-34-consolidate-duplicate-http-types/`.

## Goal

Eliminate duplicate HTTP request/response type definitions across the workspace by consolidating
them into `packages/http-protocol`, and add the `http-protocol` dependency to `tracker-client` so
both consumers import from a single source of truth.

## Background

Three crate locations define overlapping HTTP request and response types:

1. **`packages/http-protocol/src/v1/{requests,responses}/`** — server-side protocol parsing (production library)
2. **`packages/axum-http-server/tests/server/{requests,responses}/`** — test helpers (test-only code)
3. **`packages/tracker-client/src/http/client/{requests,responses}/`** — tracker client library (production library)

Locations (2) and (3) define their own copies of types that semantically belong in (1):

- `axum-http-server` **has** `http-protocol` as a dependency, but its tests define their own types instead of using it
- `tracker-client` does **not** depend on `http-protocol` at all

The duplication creates maintenance burden: any change to these types must be replicated in two
or three places. Several types (especially `Error`, `Compact`, `CompactPeer`, `CompactPeerList`,
scrape `Query`/`QueryBuilder`/`QueryParams`, `ByteArray20`, `InfoHash`, `percent_encode_byte_array`)
are byte-for-byte identical between locations (2) and (3).

The `http-protocol` crate is the canonical home for HTTP tracker protocol types. Client-side
parsing/serialization types are a natural extension of this crate, not a separate concern.

## Design Decisions

The following decisions were made during implementation planning (2026-07-13):

### DD1: Merge Strategy — Add Builder Types Alongside Parsers (Iteration 1)

**Decision**: In the first iteration, add builder types to `http-protocol` alongside the existing
parser types. After consolidation, a second iteration can evaluate whether a unified data model
for both parsing and building makes sense.

**Rationale**: The existing parser types (`TryFrom<Query>`) and builder types (`QueryBuilder`/`QueryParams`)
serve different purposes. Moving them into the same crate first makes it easier to detect
unification opportunities later.

### DD2: Use Domain Types (InfoHash/PeerId) in Consolidated Types

**Decision**: The consolidated types in `http-protocol` will use the domain types `InfoHash` and
`PeerId` from their dedicated crates, rather than raw `ByteArray20`.

**Rationale**: `http-protocol` already depends on `torrust-info-hash` and `torrust-peer-id`.
Client code can convert at the boundary.

### DD3: Consolidate Error Response Type into http-protocol

**Decision**: The `Error { failure_reason: String }` response type will be consolidated into
`http-protocol` and both consumers will import from there.

**Rationale**: The type is identical in all three locations. `http-protocol` already has the
canonical version.

### DD4: Use Full Event Enum from http-protocol

**Decision**: The consolidated `Event` enum will use the full set from `http-protocol`:
`Started`, `Stopped`, `Completed`, `Empty`.

**Rationale**: This is the most complete variant set and covers all use cases.

### DD5: Move percent_encode_byte_array to http-protocol

**Decision**: The `percent_encode_byte_array` helper will be moved into `http-protocol`'s
existing `percent_encoding` module.

**Rationale**: It's used by both consumers and belongs with the protocol crate.

## Scope

### In Scope

- Add client-side request construction and response deserialization types to `packages/http-protocol`
  (e.g., query builders, response structs with `serde_bencode` derives)
- Replace duplicate types in `packages/axum-http-server/tests/server/` with imports from `http-protocol`
- Replace duplicate types in `packages/tracker-client/src/http/client/` with imports from `http-protocol`
- Add `http-protocol` as a dependency of `tracker-client`
- Create a `use-tracker-client` skill in `.github/skills/usage/` capturing the manual verification learnings
- Verify all tests pass and no functionality regresses

### Out of Scope

- Merging `packages/http-protocol` with other protocol crates
- Changing the public API of `http-protocol` beyond what's needed for consolidation
- Removing or refactoring the server-side types in `http-protocol`
- Changing how `axum-http-server` production code uses `http-protocol`

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                | Notes / Expected Output                                                                         |
| --- | ------ | --------------------------------------------------- | ----------------------------------------------------------------------------------------------- |
| T1  | DONE   | Survey duplicate types and decide merge strategy    | Catalog exact types to move; identify which location has the "best" version                     |
| T2  | DONE   | Add client-side types to `http-protocol`            | Move query builders, response deserialization structs, and shared helpers                       |
| T3  | DONE   | Add `http-protocol` dependency to `tracker-client`  | Update `Cargo.toml`, verify dependency tree                                                     |
| T4  | DONE   | Replace duplicate types in `tracker-client`         | Delete local copies, update imports to `http-protocol`                                          |
| T5  | DONE   | Replace duplicate types in `axum-http-server` tests | Delete local copies, update imports to `http-protocol`                                          |
| T6  | DONE   | Run full verification                               | `linter all`, `cargo test --workspace`, pre-commit, pre-push                                    |
| T7  | TODO   | Create `use-tracker-client` skill                   | New skill in `.github/skills/usage/use-tracker-client/` with learnings from manual verification |

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
- 2026-07-13 10:00 UTC - Copilot - Spec reviewed and approved by user; design decisions recorded
- 2026-07-13 12:00 UTC - Copilot - Implementation completed, PR #1974 opened

## Acceptance Criteria

- [ ] AC1: No HTTP request/response types are duplicated between `http-protocol`, `axum-http-server` tests, and `tracker-client`
- [ ] AC2: `tracker-client` depends on `http-protocol` and imports types from it instead of defining its own
- [ ] AC3: `axum-http-server` tests import types from `http-protocol` instead of defining their own
- [ ] AC4: All existing tests pass (`cargo test --workspace`)
- [ ] AC5: `linter all` exits with code `0`
- [ ] AC6: Pre-commit and pre-push checks pass
- [ ] AC7: No `deps.rs` or layer-violation regressions
- [ ] AC8: `use-tracker-client` skill is created in `.github/skills/usage/` with proper YAML frontmatter and instructions
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

All manual verification evidence — including full command output, troubleshooting notes, and
step-by-step logs — will be recorded in a separate `manual-verification.md` file inside the
issue folder. The Evidence column below links to the relevant section of that file.

**Skills used during manual verification**:

- **Run tracker locally**: [`../../../../.github/skills/dev/environment-setup/run-tracker-locally/SKILL.md`](../../../../.github/skills/dev/environment-setup/run-tracker-locally/SKILL.md) — start the tracker with default development configuration
- **Tracker client**: No dedicated skill exists yet. A `use-tracker-client` skill will be created
  in `../../../../.github/skills/usage/` as the final step of this issue, capturing the learnings from the
  manual verification process.

| ID  | Scenario                                        | Command/Steps                                                                            | Expected Result                                     | Status | Evidence                             |
| --- | ----------------------------------------------- | ---------------------------------------------------------------------------------------- | --------------------------------------------------- | ------ | ------------------------------------ |
| M1  | HTTP tracker announces work with tracker-client | Run `tracker_client http announce` against a local tracker; verify request/response flow | Same behavior as before the consolidation           | TODO   | `manual-verification.md#m1-announce` |
| M2  | HTTP scrape works with tracker-client           | Run `tracker_client http scrape` against a local tracker                                 | Same behavior as before                             | TODO   | `manual-verification.md#m2-scrape`   |
| M3  | axum-http-server integration tests pass         | `cargo test -p torrust-tracker-axum-http-server --test integration`                      | All tests pass                                      | TODO   | `manual-verification.md#m3-tests`    |
| M4  | No duplicate type definitions remain            | `grep` for key struct names (e.g., `struct Query`, `struct CompactPeer`) in old paths    | Only imports, no local definitions for merged types | TODO   | `manual-verification.md#m4-grep`     |

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
| AC8   | TODO                   | {test/log/PR link} |

## Risks and Trade-offs

- **Risk**: Client-side types differ subtly between `tracker-client` and `axum-http-server` tests
  (e.g., `Event` default variant, `numwant` field presence). **Mitigation**: The implementer must
  survey both versions and ensure the consolidated type in `http-protocol` accommodates both use
  cases. Where differences are intentional, use configuration (e.g., builder methods, `Option`
  fields) rather than separate types.
- **Risk**: Adding `http-protocol` as a dependency of `tracker-client` increases compile time for
  the client. **Mitigation**: `http-protocol` is already a lightweight crate with few transitive
  dependencies; the impact should be negligible.
- **Risk**: The consolidation might change the public API of `http-protocol`, potentially breaking
  external consumers. **Mitigation**: Review all existing `pub` exports and ensure backward
  compatibility, or bump the version appropriately with clear changelog entries.

## References

- Parent EPIC: [#1669](https://github.com/torrust/torrust-tracker/issues/1669)
- EPIC spec: `docs/issues/open/1669-overhaul-packages/EPIC.md`
- Decisions log: `docs/issues/open/1669-overhaul-packages/DECISIONS.md`
- Duplicate analysis: exploration performed 2026-06-30 by Copilot
- Related ADR: `docs/adrs/20260527175600_keep_protocol_and_domain_types_decoupled.md`
