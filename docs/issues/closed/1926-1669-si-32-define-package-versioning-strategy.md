---
doc-type: issue
issue-type: task
status: done
priority: p1
github-issue: 1926
spec-path: docs/issues/closed/1926-1669-si-32-define-package-versioning-strategy.md
branch: 1926-1669-si-32-define-package-versioning-strategy
related-pr: null
last-updated-utc: 2026-07-15
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - Cargo.toml
    - docs/issues/open/1669-overhaul-packages/EPIC.md
    - docs/issues/open/1669-overhaul-packages/DECISIONS.md
    - docs/packages.md
    - AGENTS.md
    - docs/adrs/20260629000000_adopt_independent_package_versioning.md
    - .github/workflows/deployment.yaml
    - .github/workflows/deployment-packages.yaml
    - docs/release_process.md
---

<!-- skill-link: create-issue -->

# Issue #1926 — Define and implement package versioning strategy for EPIC #1669

## Goal

Define an explicit and maintainable SemVer policy for workspace packages, replacing
the implicit "everything shares one workspace version" rule with independent versioning
for every package — and implement all resulting changes (version migration, release process,
CI automation).

This issue is a subissue of EPIC [#1669](../open/1669-overhaul-packages/EPIC.md)
(Overhaul: Packages).

All work happens on a single branch and is merged together into `develop`.

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

### Four-Tier Versioning Model

While all packages version independently, the workspace has four distinct **versioning semantics**
tiers. These describe **what a version bump signals** for external consumers — they do **not**
determine how publishing works. All publishable packages are published **independently** via
`deployment-packages.yaml` as they evolve. The tracker release (`deployment.yaml`) only
publishes the root `torrust-tracker` binary crate.

| Tier                    | Description                                     | What a version bump signals                                 | Packages                                                                                                                                                                                                                                                                                             |
| ----------------------- | ----------------------------------------------- | ----------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Tracker runtime**     | Binary + tightly-coupled runtime crates         | The tracker application behaviour or feature set changed    | `torrust-tracker`, `tracker-core`, `udp-core`, `http-core`, `udp-server`, `axum-http-server`, `axum-server`, `axum-health-check-api-server`, `swarm-coordination-registry`, `tracker-client-lib`, `torrust-tracker-client` (console binary), `events`, `http-protocol`, `udp-protocol`, `primitives` |
| **API contract**        | Packages sharing a wire protocol with consumers | The REST API or config schema changed                       | `rest-api-protocol`, `rest-api-client`, `axum-rest-api-server`, `rest-api-application`, `rest-api-runtime-adapter`, `rest-api-core`, `configuration`                                                                                                                                                 |
| **Platform/utility**    | Generic reusable crates, test infrastructure    | The crate's own library API changed                         | `test-helpers`                                                                                                                                                                                                                                                                                       |
| **Unpublished tooling** | Workspace members with no external consumers    | Version changes only when internal API changes meaningfully | `e2e-tools`, `persistence-benchmark`, `torrent-repository-benchmarking`, `workspace-coupling`                                                                                                                                                                                                        |

> **Note on `torrust-tracker-client`** (console binary): this package is planned for extraction
> to a standalone repository (see
> [`docs/issues/drafts/1669-extract-torrust-tracker-client-to-standalone-repo.md`](../drafts/1669-extract-torrust-tracker-client-to-standalone-repo.md)).
> Key points:

1. **All publishable workspace crates are published independently** via `deployment-packages.yaml` whenever a
   crate's version changes. By the time a tracker release happens, all dependency crates are
   already on crates.io — `deployment.yaml` only publishes `torrust-tracker` itself.

2. **For API contract packages**, a major/minor bump should be coordinated across server and
   client (a human convention, not a mechanical link or separate workflow). If you release
   `axum-rest-api-server` v2.0.0, you should also bump `rest-api-client` to v2.0.0 and publish
   it independently at the same time via `deployment-packages.yaml`.

3. **`rest-api-protocol`** sits at the root of the REST API contract tree. Its version is
   the canonical API version. Server and client implementations carry matching major.minor
   as a convention.

### Version by Namespace for Public Contracts

> **Also known as**: **version by namespace convention** (the official term from ASP.NET API
> Versioning's `VersionByNamespaceConvention`), **namespace-based versioning**, **co-located
> versioning**.
>
> The opposite approach (separate Git branches per version) is called **branch-based versioning**
> or **version branches**.
>
> **Naming decision**: the project adopts **"version by namespace"** as the preferred term
> because it:
>
> - Has a direct, well-known analogue in the ASP.NET ecosystem (`VersionByNamespaceConvention`)
> - Describes exactly what we do (derive versions from namespace/directory names)
> - Is unambiguous ("in-code versioning" could be confused with runtime version negotiation)
> - Is concise enough for ADR titles and commit messages

The project already uses a **version by namespace** pattern for public contracts.
Multiple versions of the same contract coexist in the codebase under versioned namespace modules:

```text
# REST API — all versions live in the same repository
packages/rest-api-protocol/src/v1/           # protocol DTOs for API v1
packages/rest-api-client/src/v1/             # client implementation for API v1
packages/axum-rest-api-server/src/v1/        # server implementation for API v1

# Configuration schema — all versions live in the same repository
packages/configuration/src/v2_0_0/           # schema v2.0.0
```

The latest version of `develop` and `main` defaults to the latest API/config version,
but the code for older versions is retained alongside. This was chosen over maintaining
separate Git branches per version because:

**Pros of version by namespace:**

- Multiple API versions coexist during long migration periods (consumers may take months
  or years to migrate)
- Consumers can use multiple API versions simultaneously during incremental migration
- Configuration schema migrations can read/write both old and new schemas in the same
  codebase, enabling zero-downtime schema migration scripts
- No branch management overhead (cherry-pick conflicts, stale branches, merge hell)
- CI always tests all supported versions together
- A single `develop` → `main` flow is easier to reason about

**Cons of version by namespace:**

- Source tree is larger (older versions accumulate)
- Removing an old version requires a deliberate code removal commit (not just branch deletion)
- Risk of accidental changes to old versions if tests are not careful
- Can encourage "keep everything forever" if there is no deprecation policy

**Pros of Git-branch-per-version:**

- Clean separation of concerns — each branch has only the code it needs
- Removing an old version is as simple as deleting a branch
- No risk of accidentally modifying old version code

**Cons of Git-branch-per-version:**

- Cherry-pick fixes across N active version branches is painful and error-prone
- Branches diverge over time — hotfixes may not apply cleanly
- Consumers on older versions cannot easily see what the new API looks like
- CI must be configured to test N branches instead of one
- Configuration schema migrations require two branches (or complex cross-branch coordination)

**Decision**: version by namespace is the right approach for this project. The ability to
support long-lived parallel versions, seamless configuration migration, and a single
CI pipeline outweighs the source tree size cost. A deprecation policy should be
defined separately to prevent unbounded accumulation of old versions.

### Rationale

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

## Release Process Implications

Independent versioning splits the current unified release model into two distinct concepts:

| Concept                         | Description                                 | Branch convention                     | Tag convention                        | CI                                                        |
| ------------------------------- | ------------------------------------------- | ------------------------------------- | ------------------------------------- | --------------------------------------------------------- |
| **Tracker application release** | Root binary `torrust-tracker`               | `releases/v<semver>`                  | `v<semver>` (signed)                  | `deployment.yaml` triggered by `releases/v*`              |
| **Individual package publish**  | Any workspace crate published independently | `releases/pkg/<crate-name>/v<semver>` | `pkg/<crate-name>/v<semver>` (signed) | `deployment-packages.yaml` triggered by `releases/pkg/**` |

The glob `releases/v*` does **not** match `releases/pkg/...` because `*` does not cross `/` boundaries in GitHub Actions pattern matching. This keeps triggers mutually exclusive.

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

- Split the current monolithic process into two sections:
  - "Tracker Application Release" — the existing process, now publishing only `torrust-tracker`.
  - "Publishing a Workspace Package" — the **primary** publishing path for all packages.
    Includes branch/tag conventions, CI trigger, manual fallback, and a
    [real-world example](../../release_process.md#real-world-example-a-full-release-cycle) showing how package
    publishing works over a full release cycle.
- Remove stale crate entries from the tracker release checklist.

**`.github/workflows/deployment.yaml`**:

- Refined to publish **only** `torrust-tracker` (the root binary crate).
- All dependency crates are published independently via `deployment-packages.yaml` before
  the tracker release.

**`.github/workflows/deployment-packages.yaml`**:

- **Created** — the primary publishing path for all workspace packages.
- Trigger: `on.push.branches: "releases/pkg/**"` or `workflow_dispatch`.
- Extracts the package name from the branch ref and runs `cargo publish -p <name>`.

> **Design decision**: `deployment.yaml` publishes only the root binary crate.
> All publishable dependency crates are published independently via `deployment-packages.yaml` as they
> evolve. This avoids conflating versioning semantics (four-tier model) with publish
> mechanics (single workflow per package).

### GitHub Releases

GitHub Releases (with release notes, assets, etc.) are used **only for the tracker
application binary**. Workspace packages are published to crates.io only — they do not
get GitHub Releases. The crate's README and `Cargo.toml` metadata serve as their
documentation surface.

### What Does Not Change

- The existing **tracker application release process** continues to work as before — tagged releases now publish only `torrust-tracker` (all dependency crates are independently published beforehand).
- Path dependencies within the workspace are unaffected — Cargo always resolves the local copy.

## Implementation

The work is organised into three phases, all executed within this single branch.
Each phase produces its own commit(s).

### Phase 1 — Policy Definition (already done)

1. Define the policy contract: all packages version independently. ✓
2. Create an ADR in `docs/adrs/` documenting the decision. ✓
3. Update EPIC documentation with the ADR reference. ✓

### Phase 2 — Version Migration

1. Remove `version.workspace = true` from all workspace `Cargo.toml` package manifests.
   This includes:
   - All packages under `packages/*/Cargo.toml` (24 crates)
   - `console/tracker-client/Cargo.toml` — the console binary crate, planned for
     extraction to a standalone repository (see
     [`docs/issues/drafts/1669-extract-torrust-tracker-client-to-standalone-repo.md`](../drafts/1669-extract-torrust-tracker-client-to-standalone-repo.md))
   - `contrib/dev-tools/analysis/workspace-coupling/Cargo.toml` — the workspace
     coupling analysis tool
2. Set appropriate initial versions for each package:
   - `0.1.0` for unpublished tool crates (axum-\*, events, etc.).
   - Matching existing published versions for crates already on crates.io.
3. Remove the `version` key from `[workspace.package]` in the root `Cargo.toml`.
   The `torrust-tracker` binary crate gets its own explicit `version = "3.0.0-develop"` field.
   The `[workspace.package]` section keeps all metadata fields (authors, description,
   edition, etc.) but no longer carries a shared version for other packages to inherit.
4. Update all `version` fields in `[dependencies]` and `[dev-dependencies]` in the root
   `Cargo.toml` to match each workspace package's new explicit version. Without this,
   `cargo publish` for `torrust-tracker` would declare a wrong required version range
   (e.g., `>= 3.0.0-develop` for a crate actually published as `0.1.0`), causing publish
   failures.
5. Validate that `cargo publish -p <crate>` (dry-run) succeeds for a representative
   subset of packages.
6. Update package READMEs where they reference the shared version.

### Phase 3 — Release Process and CI Automation

1. Update `docs/release_process.md` with both release paths:
   - "Tracker Application Release" — existing process, now publishes only `torrust-tracker`.
   - "Publishing a Workspace Package" — the **primary** publishing path for all packages,
     with branch/tag conventions, CI automation, manual fallback, and a real-world example.
2. Update `.github/workflows/deployment.yaml`:
   - Refine trigger to `releases/v*` (tracker only).
   - Reduce publish step to only `cargo publish -p torrust-tracker`.
3. Create `.github/workflows/deployment-packages.yaml`:
   - Trigger: `releases/pkg/**` and `workflow_dispatch` (manual crate name input).
   - Single publish job that extracts the crate name from branch name or input.
   - Tests the specific crate before publishing.
   - Add a `Verify explicit version` step that checks the crate has its own `version`
     field (not `version.workspace = true`) before attempting to publish. This prevents
     confusing Cargo errors if someone pushes a branch for a crate still using
     `version.workspace = true`.
4. Document the branch and tag naming convention:
   - Tracker: `releases/v<semver>` / `v<semver>`.
   - Package: `releases/pkg/<crate-name>/v<semver>` / `pkg/<crate-name>/v<semver>`.
5. Verify that `releases/v*` does NOT match `releases/pkg/...` (glob safety).

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

### Alternative C - Link versions for API contract packages only (discarded)

Why considered:

- The REST API server and client share a wire protocol — bumping the API version
  on the server without a matching client bump would confuse consumers.
- The same reasoning applies to configuration schema consumers.
- A "semi-independent" model seemed simpler than the three-tier model above.

Why discarded:

- The coupling is already handled by **version by namespace** (the `v1/` modules):
  the server and client both implement `v1` of the protocol. They are always in
  sync because they live in the same branch at the same protocol version.
- The `Cargo.toml` version is a **distribution/packaging concern**, not a protocol
  version indicator. The protocol version is tracked by the `v1/` namespace.
- Linking `Cargo.toml` versions across API packages would reintroduce the same
  churn problem that independent versioning solves: a bugfix in the client's HTTP
  transport layer would force a version bump on the server crate.
- The convention "major.minor tracks the API contract; patches are independent" is
  sufficient without mechanical enforcement. If the `cargo publish` workflow for
  the REST API server bumps its version, it's a human responsibility to also bump
  the client if the API contract changed.
- Proving that linking is unnecessary: if `axum-rest-api-server` v2.1.0 adds a new
  endpoint and `rest-api-client` v2.0.3 doesn't support it yet, the consumer simply
  knows they need client ≥ v2.1.0 — the crates.io solver handles this naturally via
  version constraints. No mechanical link needed.

### Alternative D - Automated CI check to prevent `version.workspace = true` regression (discarded)

Why considered:

- A CI check could catch accidental reintroduction of `version.workspace = true`
  in a crate's `Cargo.toml` before a publish attempt.
- Would provide a clear error message instead of a confusing Cargo failure.

Why discarded:

- The existing `deployment-packages.yaml` already has a `Verify explicit version`
  step that catches this before publishing — the check was moved to the point of
  use (the publish workflow) rather than a standalone CI gate.
- Adding a separate CI check on every `push`/`pull_request` would add noise for
  little benefit: the publish workflow check is sufficient.
- Pre-commit hooks are team-local and cannot be enforced in CI without duplicating
  the publish workflow logic.
- If a crate accidentally uses `version.workspace = true`, it will be caught at
  publish time with a clear message. No intermediate gate needed.

## Scope

### In Scope

- Define and document the independent versioning policy. ✓
- Create an ADR documenting the policy decision for permanent reference in `docs/adrs/`. ✓
- Update EPIC documentation with the ADR reference. ✓
- Remove `version.workspace = true` from all workspace `Cargo.toml` package manifests. ✓
- Set appropriate initial versions for each package. ✓
- Remove `version` from `[workspace.package]` in root `Cargo.toml` (tracker crate gets its own). ✓
- Add CI checks to prevent reintroducing `version.workspace = true`. (Discarded — see Alternative D)
- Split `docs/release_process.md` into two release paths (tracker + packages). ✓
- Refine `.github/workflows/deployment.yaml` trigger and publish list. ✓
- Create `.github/workflows/deployment-packages.yaml`. ✓

### Out of Scope

- Publishing any crate to crates.io (that is the release process itself).
- Renaming packages or restructuring the workspace.
- Changes to packages extracted to standalone repositories (they publish from their own CI).

## Acceptance Criteria

- [x] The policy explicitly states that all packages version independently.
- [x] The rationale explains why linked versions are unnecessary (path deps guarantee compatibility).
- [x] An ADR is created in `docs/adrs/` documenting the independent versioning decision.
- [x] ADR is linked from EPIC #1669 documentation.
- [x] At least two alternatives are documented with discard reasons.
- [x] EPIC #1669 references the approved versioning policy.
- [x] No package uses `version.workspace = true`.
- [x] Each package has an explicit `version` field appropriate to its maturity and publication status.
- [x] `[workspace.package]` in root `Cargo.toml` no longer has a `version` key.
      `torrust-tracker` has its own explicit `version` field.
- [x] All `version` fields in root `Cargo.toml` `[dependencies]` and `[dev-dependencies]` match
      each package's new explicit version.
- [ ] `cargo publish -p <crate>` (dry-run) succeeds for representative packages.
- [x] All existing tests and linters pass.
- [x] `docs/release_process.md` documents both publishing paths (tracker release + per-package).
- [x] `.github/workflows/deployment.yaml` no longer lists extracted crates.
- [x] `.github/workflows/deployment-packages.yaml` is created and documents the package release path.
- [x] Crate dependency publish order is documented (or validated by CI).
- [x] Branch/tag naming conventions are documented and verified to not conflict.

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
- ADR: [docs/adrs/20260629000000_adopt_independent_package_versioning.md](../../adrs/20260629000000_adopt_independent_package_versioning.md)
- Workspace manifest: [Cargo.toml](../../../Cargo.toml)
- Package catalog: [docs/packages.md](../../packages.md)
- Tracker release workflow: [.github/workflows/deployment.yaml](../../../.github/workflows/deployment.yaml)
- Package release workflow: [.github/workflows/deployment-packages.yaml](../../../.github/workflows/deployment-packages.yaml)
- Release process: [docs/release_process.md](../../release_process.md)

## Appendix A — Version Assignment Table

Crates.io status verified 2026-06-29. This table is the authoritative source for
Phase 2 version migration.

### Published on crates.io (carry forward existing version)

| Package                         | Crate Name                      | crates.io Version | Proposed Initial Version            |
| ------------------------------- | ------------------------------- | ----------------- | ----------------------------------- |
| `torrust-tracker` (root binary) | `torrust-tracker`               | `3.0.0`           | `3.0.0-develop` (retain dev suffix) |
| `primitives`                    | `torrust-tracker-primitives`    | `3.0.0`           | `3.0.0`                             |
| `configuration`                 | `torrust-tracker-configuration` | `3.0.0`           | `3.0.0`                             |
| `test-helpers`                  | `torrust-tracker-test-helpers`  | `3.0.0`           | `3.0.0`                             |

### Extracted to standalone repos (not in workspace — out of scope)

| Package          | Crate Name               | crates.io Version | Repository                       |
| ---------------- | ------------------------ | ----------------- | -------------------------------- |
| `clock`          | `torrust-clock`          | `3.0.0`           | `torrust/torrust-clock`          |
| `located-error`  | `torrust-located-error`  | `3.0.0`           | `torrust/torrust-located-error`  |
| `metrics`        | `torrust-metrics`        | `0.1.0`           | `torrust/torrust-metrics`        |
| `net-primitives` | `torrust-net-primitives` | `0.1.0`           | `torrust/torrust-net-primitives` |
| `server-lib`     | `torrust-server-lib`     | `0.1.0`           | `torrust/torrust-server-lib`     |

### Not on crates.io (unpublished — initial version `0.1.0`)

| Package                                                      | Crate Name                                        | Tier                                   |
| ------------------------------------------------------------ | ------------------------------------------------- | -------------------------------------- |
| `tracker-core`                                               | `torrust-tracker-core`                            | Tracker runtime                        |
| `udp-core`                                                   | `torrust-tracker-udp-core`                        | Tracker runtime                        |
| `http-core`                                                  | `torrust-tracker-http-core`                       | Tracker runtime                        |
| `udp-server`                                                 | `torrust-tracker-udp-server`                      | Tracker runtime                        |
| `udp-protocol`                                               | `torrust-tracker-udp-protocol`                    | Tracker runtime                        |
| `http-protocol`                                              | `torrust-tracker-http-protocol`                   | Tracker runtime                        |
| `events`                                                     | `torrust-tracker-events`                          | Tracker runtime                        |
| `swarm-coordination-registry`                                | `torrust-tracker-swarm-coordination-registry`     | Tracker runtime                        |
| `axum-health-check-api-server`                               | `torrust-tracker-axum-health-check-api-server`    | Tracker runtime                        |
| `axum-http-server`                                           | `torrust-tracker-axum-http-server`                | Tracker runtime                        |
| `axum-server`                                                | `torrust-tracker-axum-server`                     | Tracker runtime                        |
| `tracker-client` (lib, `packages/tracker-client/`)           | `torrust-tracker-client-lib`                      | Tracker runtime                        |
| `tracker-client` (console binary, `console/tracker-client/`) | `torrust-tracker-client`                          | Tracker runtime (extraction candidate) |
| `rest-api-protocol`                                          | `torrust-tracker-rest-api-protocol`               | API contract                           |
| `rest-api-core`                                              | `torrust-tracker-rest-api-core`                   | API contract                           |
| `rest-api-client`                                            | `torrust-tracker-rest-api-client`                 | API contract                           |
| `rest-api-application`                                       | `torrust-tracker-rest-api-application`            | API contract                           |
| `rest-api-runtime-adapter`                                   | `torrust-tracker-rest-api-runtime-adapter`        | API contract                           |
| `axum-rest-api-server`                                       | `torrust-tracker-axum-rest-api-server`            | API contract                           |
| `e2e-tools`                                                  | `torrust-tracker-e2e-tools`                       | Unpublished tooling                    |
| `persistence-benchmark`                                      | `torrust-tracker-persistence-benchmark`           | Unpublished tooling                    |
| `torrent-repository-benchmarking`                            | `torrust-tracker-torrent-repository-benchmarking` | Unpublished tooling                    |
| `workspace-coupling` (contrib)                               | `torrust-tracker-workspace-coupling`              | Unpublished tooling                    |

**Summary**: 4 crates keep `3.0.0`, 23 crates start at `0.1.0`, the root binary
keeps `3.0.0-develop`. 5 extracted crates are out of scope.

> **Publishability**: The "Unpublished tooling" tier crates (`e2e-tools`,
> `persistence-benchmark`, `torrent-repository-benchmarking`, `workspace-coupling`) are internal
> testing, benchmarking, and analysis tools with no external consumers. They are never published to
> crates.io. All other workspace crates are publishable via `deployment-packages.yaml`.

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted
- [x] Implementation completed (PR #1961 merged)
- [x] Automatic verification completed (`linter all`, relevant tests, pre-push checks)
- [x] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-06-20 11:26 UTC - PR #1927 merged - Archival of initial subissue spec
- 2026-07-13 08:50 UTC - PR #1961 merged - Implementation completed (independent package versioning)
- 2026-07-15 UTC - Spec archived to `docs/issues/closed/`
