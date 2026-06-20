---
doc-type: issue
issue-type: task
status: open
priority: p1
github-issue: 1926
spec-path: docs/issues/open/1926-1669-si-32-define-package-versioning-strategy.md
branch: null
related-pr: null
last-updated-utc: 2026-06-20 00:00
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

# Issue #1926 - Define package versioning strategy for EPIC #1669

## Goal

Define an explicit and maintainable SemVer policy for workspace packages, replacing
the implicit "everything shares one workspace version" rule with independent versioning
for every package.

This issue defines policy only — actual version migration is deferred to a follow-up
implementation issue.

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

- All packages currently share the workspace root version (`version.workspace = true` → `3.0.0-develop`).
- The workspace contains packages with very different consumer surfaces: tightly-coupled tracker runtime crates, utility/platform crates (`torrust-clock`, `torrust-server-lib`, etc.), and extraction candidates.
- Since all dependencies use `path = "..."` within the workspace, there is **no runtime compatibility risk** from independent versions — Cargo always uses the local copy regardless of the version number in `Cargo.toml`.

Conclusion:

- A single lockstep version is suboptimal — it inflates churn on unrelated packages and gives weak SemVer signals.
- A hybrid two-tier split imposes a guess about future coupling instead of letting it emerge naturally.
- **Independent versioning for all packages** is the simplest correct approach: path dependencies make it safe, and individual release cadences can evolve without coordination overhead.

## Proposed Versioning Policy

**All packages version independently**. Each package declares its own `version` field
(not `version.workspace = true`), starting from their current `3.0.0-develop` value
with an appropriate initial release version.

Rationale:

- Path dependencies make linked versions unnecessary — the workspace always resolves
  the local copy regardless of the declared version.
- Avoids unnecessary SemVer churn on unrelated packages when only part of the workspace changes.
- Gives accurate SemVer signals to external consumers of published crates.
- Aligns with the EPIC #1669 extraction goal — packages moving to standalone repos already
  version independently.
- If packages naturally evolve together over time, that coupling can be formalised later
  when there is evidence, not before.

Packages that have been or will be extracted to standalone repositories already follow
independent versioning (e.g. `torrust-clock` 3.0.0, `torrust-metrics` 0.1.0,
`torrust-net-primitives` 0.1.0). This issue formalises the same approach for every
package in the workspace.

Out of scope for this policy issue:

- Migration execution (changing `Cargo.toml` files) is out of scope — this issue
  defines the policy only.
- Setting specific initial versions for each package — that is a follow-up
  implementation concern.

## Release Process Implications

Independent versioning splits the current unified release model into two distinct concepts:

| Concept                         | Description                                                                                                                                                   | Cadence                                                                     |
| ------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------- |
| **Tracker application release** | Root binary `torrust-tracker` + the tightly-coupled runtime crates (`tracker-core`, `udp-server`, `http-core`, `axum-*`, `configuration`, `primitives`, etc.) | Existing process: tag `releases/v*`, CI publishes the whole set as a bundle |
| **Individual package publish**  | Any workspace crate published independently at its own cadence (e.g., `torrust-tracker-udp-protocol` unblocks the client extraction)                          | Per-package decision — no need to wait for a full tracker release           |

### Why This Matters Now

The client extraction draft ([`docs/issues/drafts/1669-extract-torrust-tracker-client-to-standalone-repo.md`](../drafts/1669-extract-torrust-tracker-client-to-standalone-repo.md))
is blocked on two unpublished workspace crates:

| Blocker crate                                            | Published? | Can publish after this policy?  |
| -------------------------------------------------------- | ---------- | ------------------------------- |
| `torrust-tracker-udp-protocol`                           | **No**     | **Yes** — publish independently |
| `torrust-tracker-client-lib` (`packages/tracker-client`) | **No**     | **Yes** — publish independently |

Currently, publishing them requires the full tracker release process (tag, release branch, full bundle).
With independent versioning, each can be published with a single `cargo publish -p <crate>` when ready.

### Affected Artifacts

**`docs/release_process.md`**:

- Rename to "Tracker Application Release Process" (the existing workflow stays as one path).
- Add a second "Publishing an Individual Package" path documenting the per-package workflow.

**`.github/workflows/deployment.yaml`**:

- Remove crates that have been extracted to standalone repos (they publish from their own CI).
- Convert from one monolithic publish list to either:
  - A reusable workflow that accepts a package name parameter, or
  - A split into a "tracker release" workflow for the runtime bundle and individual `workflow_dispatch` triggers per package.
- Current stale entries (still listing extracted crates like `torrust-located-error`, `torrust-clock`, etc.) must be cleaned up in Phase 2.

### What Does Not Change

- The existing **tracker application release process** continues to work as before — tagged releases still publish the runtime bundle together.
- Path dependencies within the workspace are unaffected — Cargo always resolves the local copy.

## Implementation Strategy

### Phase 1 (this issue): policy definition only

1. Define the policy contract: all packages version independently.
2. Create an ADR in `docs/adrs/` documenting the decision.
3. Document release process impact:
   - What constitutes a "release" now (per-package vs unified).
   - How per-package publishing works in CI.
   - Update `docs/release_process.md`.
   - Update `.github/workflows/deployment.yaml`.
4. Document in the EPIC and EPIC references the ADR.
5. Open a follow-up implementation issue for Phase 2 (version migration)
   and another for release process automation changes.

Phase 2 (follow-up implementation issue):

1. Remove `version.workspace = true` from all packages.
2. Set appropriate initial versions (likely `0.1.0` for unpublished tool crates,
   matching existing releases for published ones).
3. Add CI checks to prevent reintroducing `version.workspace = true`.
4. Validate publish workflows for per-package versioning.

## Alternatives Considered

### Alternative A - Keep all crates on one shared workspace version (discarded)

Why considered:

- Minimal tooling complexity.
- Very easy coordinated release process.

Why discarded:

- Over-couples unrelated packages and inflates churn.
- Weak SemVer signal for external consumers.
- Conflicts with EPIC extraction goals and independent release cadence.

### Alternative B - Hybrid two-tier strategy (discarded)

Why considered:

- Appeared to balance coordination simplicity for tightly-coupled runtime crates
  against independent evolution for utility crates.

Why discarded:

- The linked-tier advantage is illusory: path dependencies already guarantee
  compatibility within the workspace, so linked version numbers add no safety.
- Imposes a guess about future coupling that may not hold — better to let
  emergent coupling patterns drive future decisions.
- Adds unnecessary policy complexity over the simple "all independent" approach.

## Scope

### In Scope

- Define and document the independent versioning policy.
- Document the rationale (path deps make linked versions unnecessary).
- Create an ADR documenting the policy decision for permanent reference in `docs/adrs/`.
- Update EPIC documentation with the adopted proposal once approved.
- Open a follow-up implementation issue for Phase 2 (execution).
- Document required changes to the release process and deployment CI to support
  per-package publishing (affects `docs/release_process.md` and
  `.github/workflows/deployment.yaml`).

### Out of Scope

- Migrating any package to independent versions — that is a follow-up
  implementation issue.
- Setting specific initial versions for each package.
- Publishing extracted crates in external repositories.
- Renaming packages as part of this policy issue.

## Acceptance Criteria

- [ ] The policy explicitly states that all packages version independently.
- [ ] The rationale explains why linked versions are unnecessary (path deps guarantee compatibility).
- [ ] An ADR is created in `docs/adrs/` documenting the independent versioning decision.
- [ ] ADR is linked from EPIC #1669 documentation.
- [ ] At least two alternatives are documented with discard reasons.
- [ ] EPIC #1669 references the approved versioning policy.
- [ ] Release process impact is documented in this spec and a follow-up issue is opened for execution.
- [ ] Deployment CI impact is documented in this spec and a follow-up issue is opened for execution.
- [ ] A follow-up implementation issue is opened for Phase 2 (version migration).

## Verification Plan

### Automatic Checks

- `cargo metadata --no-deps --format-version 1` (validate package inventory)
- `linter all`

### Manual Verification

| ID  | Scenario                                       | Expected Result                                                                                  |
| --- | ---------------------------------------------- | ------------------------------------------------------------------------------------------------ |
| MV1 | Review the policy statement                    | Policy says "all packages version independently" with clear rationale                            |
| MV2 | Review alternatives section                    | Discarded options and reasons are explicit                                                       |
| MV3 | Cross-check policy against EPIC extraction map | Independent versioning aligns with extraction direction in EPIC #1669                            |
| MV4 | Review release process implications            | Two-concept split (tracker release vs per-package publish) is documented with affected artifacts |

## References

- EPIC: [docs/issues/open/1669-overhaul-packages/EPIC.md](../open/1669-overhaul-packages/EPIC.md)
- Decisions: [docs/issues/open/1669-overhaul-packages/DECISIONS.md](../open/1669-overhaul-packages/DECISIONS.md)
- Workspace manifest: [Cargo.toml](../../../Cargo.toml)
- Package catalog: [docs/packages.md](../../packages.md)
