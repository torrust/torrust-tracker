---
doc-type: issue
issue-type: task
status: draft
priority: p1
github-issue: null
spec-path: docs/issues/drafts/1669-define-package-versioning-strategy.md
branch: null
related-pr: null
last-updated-utc: 2026-05-27 00:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - Cargo.toml
    - docs/issues/open/1669-overhaul-packages/EPIC.md
    - docs/issues/open/1669-overhaul-packages/DECISIONS.md
    - docs/packages.md
    - AGENTS.md
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Define package versioning strategy for EPIC #1669

## Goal

Define an explicit and maintainable SemVer policy for workspace packages, replacing
the implicit "everything shares one workspace version" rule with a policy that
matches package ownership, coupling, and release cadence.

This issue defines policy now, but does not activate the migration immediately.
Policy activation is intentionally deferred until boundary-refactor subissues
have reduced layer coupling and package ownership is clearer.

This issue is a subissue of EPIC [#1669](../open/1669-overhaul-packages/EPIC.md)
(Overhaul: Packages).

## Problem Statement

Current state:

- All workspace crates use `version.workspace = true` and currently resolve to
  `3.0.0-develop`.
- This keeps internal releases simple but couples unrelated packages to the same
  release cadence.

Observed downside:

- Generic crates and tool crates are version-bumped even when no API or behavior
  changed in those crates.
- Consumers cannot infer change risk from version numbers when every crate bumps
  together.
- Extraction and independent publication plans in EPIC #1669 become harder to
  execute cleanly when package identity and version cadence are still mixed.

## Analysis Summary

From current workspace topology:

- There is a tightly-coupled tracker runtime cluster (`tracker-core`, protocol
  cores, servers, configuration, REST API, root binary) that changes together
  frequently.
- There are utility/platform crates (`torrust-clock`, `torrust-metrics`,
  `torrust-located-error`, `torrust-net-primitives`, `torrust-server-lib`) with
  broader reuse potential and slower API churn.
- There are package candidates intended for extraction or broader reuse
  (`bittorrent-peer-id`, `torrust-tracker-contrib-bencode`, tracker client
  library/CLI split).

Conclusion:

- A single lockstep version for every crate is suboptimal long-term.
- Full per-crate independence immediately is also too expensive operationally.
- A hybrid policy is the best fit now.

## Proposed Versioning Policy (Recommended)

Adopt a two-tier strategy.

### Tier A - Linked "tracker release train" versions

These crates stay version-linked and move together per tracker release:

- `torrust-tracker` (root)
- `torrust-tracker-core`
- `torrust-tracker-http-tracker-core`
- `torrust-tracker-udp-tracker-core`
- `torrust-tracker-http-tracker-protocol`
- `torrust-tracker-udp-tracker-protocol`
- `torrust-tracker-axum-server`
- `torrust-tracker-axum-http-server`
- `torrust-tracker-axum-rest-api-server`
- `torrust-tracker-axum-health-check-api-server`
- `torrust-tracker-rest-api-core`
- `torrust-tracker-rest-api-client`
- `torrust-tracker-configuration`
- `torrust-tracker-events`
- `torrust-tracker-primitives`
- `torrust-tracker-swarm-coordination-registry`
- `torrust-tracker-test-helpers`
- `torrust-tracker-udp-server`

Rationale:

- High internal coupling and coordinated behavior changes.
- Reduces coordination overhead for the main tracker artifact.
- Keeps release management simple for the core product.

### Tier B - Independent package versions

These crates should evolve with independent versions:

- `torrust-clock`
- `torrust-metrics`
- `torrust-located-error`
- `torrust-net-primitives`
- `torrust-server-lib`
- `bittorrent-peer-id`
- `torrust-tracker-contrib-bencode`
- `torrust-tracker-client-lib`
- `torrust-tracker-client` (console package)
- `workspace-coupling` (dev tool)
- `torrust-tracker-torrent-repository-benchmarking`

Rationale:

- Distinct consumer surface and release cadence from core tracker runtime.
- Lower risk of unnecessary version churn.
- Better SemVer signaling for external users and extraction targets.

## Policy Activation Gate (Deferred Implementation)

The policy is documented in this issue now, but implementation is deferred.

Activation preconditions:

- SI-13 (`http-protocol` decoupling from `udp-protocol`) is completed.
- SI-14 (`http-protocol` decoupling from `torrust-tracker-primitives`) is completed.
- No unresolved layer-guardrail violations remain for protocol/core/server
  boundaries relevant to package grouping decisions.
- Package ownership boundaries are stable enough that version grouping changes
  are unlikely to be immediately invalidated by follow-up refactors.

Until these conditions are met, the repository keeps the current workspace
version behavior as the operational default.

## Implementation Strategy

Use an incremental transition, not a one-shot migration.

Phase 1 (this issue): policy definition only.

1. Define policy contract in docs (EPIC + this issue + optional ADR).
2. Define activation gate and prerequisites.
3. Open follow-up implementation issues, but do not migrate versions yet.

Phase 2 (follow-up, after activation gate passes): migration.

1. Keep Tier A on workspace-linked version management.
2. Move Tier B crates to explicit per-package `version = "..."` values.
3. Update internal path dependency constraints to reference intended ranges for
   independent crates.
4. Add CI checks to prevent accidental rollback to all-linked versions.
5. Validate publish workflows and changelog discipline for independent crates.

## Alternatives Considered

### Alternative A - Keep all crates on one shared workspace version (discarded)

Why considered:

- Minimal tooling complexity.
- Very easy coordinated release process.

Why discarded:

- Over-couples unrelated packages and inflates churn.
- Weak SemVer signal for external consumers.
- Conflicts with EPIC extraction goals and independent release cadence.

### Alternative B - Make every crate independently versioned now (discarded)

Why considered:

- Maximum SemVer precision and package autonomy.

Why discarded:

- High immediate operational complexity.
- Larger migration surface while layering work (SI-13/SI-14 and follow-ups)
  is still in progress.
- Increases short-term release friction without enough near-term benefit for
  tightly coupled runtime crates.

## Scope

### In Scope

- Define and document the two-tier versioning policy.
- Classify each workspace package into linked vs independent tier.
- Specify migration sequence, activation gate, and validation checks.
- Update EPIC documentation with the adopted proposal once approved.

### Out of Scope

- Activating or executing version migration before boundary-refactor
  preconditions are satisfied.
- Full migration of every package to the new policy in this issue.
- Publishing extracted crates in external repositories.
- Renaming packages as part of this policy issue.

## Acceptance Criteria

- [ ] A documented package-by-package classification exists (linked vs independent).
- [ ] The proposal includes explicit rationale for each tier.
- [ ] At least two alternatives are documented with discard reasons.
- [ ] The policy activation gate is explicit (deferred implementation until
      boundary refactors are completed).
- [ ] EPIC #1669 references the approved versioning policy.
- [ ] Follow-up implementation issues are opened for migration steps.

## Verification Plan

### Automatic Checks

- `cargo metadata --no-deps --format-version 1` (validate package inventory)
- `linter all`

### Manual Verification

| ID  | Scenario                                       | Expected Result                                                       |
| --- | ---------------------------------------------- | --------------------------------------------------------------------- |
| MV1 | Review package table in this spec              | Every workspace package is assigned to one tier                       |
| MV2 | Review alternatives section                    | Discarded options and reasons are explicit                            |
| MV3 | Cross-check policy against EPIC extraction map | Independent tier aligns with extraction/reuse direction in EPIC #1669 |

## References

- EPIC: [docs/issues/open/1669-overhaul-packages/EPIC.md](../open/1669-overhaul-packages/EPIC.md)
- Decisions: [docs/issues/open/1669-overhaul-packages/DECISIONS.md](../open/1669-overhaul-packages/DECISIONS.md)
- Workspace manifest: [Cargo.toml](../../../Cargo.toml)
- Package catalog: [docs/packages.md](../../packages.md)
