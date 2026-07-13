---
semantic-links:
  skill-links:
    - create-adr
  related-artifacts:
    - docs/issues/open/1926-1669-si-32-define-package-versioning-strategy.md
    - docs/issues/open/1669-overhaul-packages/EPIC.md
    - docs/issues/open/1669-overhaul-packages/DECISIONS.md
    - docs/release_process.md
    - .github/workflows/deployment.yaml
    - .github/workflows/deployment-packages.yaml
---

# Adopt Independent Package Versioning

## Description

All workspace packages previously shared a single lockstep version (`version.workspace = true`
→ `3.0.0-develop`). This coupled unrelated packages to the same release cadence, inflated
SemVer churn on packages with no changes, and gave weak signals to external consumers about
change risk.

The workspace contains packages with very different consumer surfaces: tightly-coupled tracker
runtime crates, protocol crates, utility crates, and tool crates. A single shared version
cannot accurately reflect the maturity and change frequency of all of them.

## Agreement

**All packages in the `torrust-tracker` workspace version independently.**
Publishable packages are published to crates.io via `deployment-packages.yaml` as they evolve.

The tracker release (`deployment.yaml`) publishes **only** the root `torrust-tracker`
binary crate — all dependency crates are already on crates.io from their independent
publishing cycles.

The release model splits into two distinct concepts with dedicated branch/tag conventions
and CI automation:

| Concept                         | Description                                                     | Branch convention                     | Tag convention                        | CI workflow                | Trigger           | Publishes              |
| ------------------------------- | --------------------------------------------------------------- | ------------------------------------- | ------------------------------------- | -------------------------- | ----------------- | ---------------------- |
| **Tracker application release** | Root binary crate `torrust-tracker`                             | `releases/v<semver>`                  | `v<semver>` (signed)                  | `deployment.yaml`          | `releases/v*`     | Only `torrust-tracker` |
| **Individual package publish**  | Any workspace crate published independently (primary mechanism) | `releases/pkg/<crate-name>/v<semver>` | `pkg/<crate-name>/v<semver>` (signed) | `deployment-packages.yaml` | `releases/pkg/**` | Exactly one crate      |

While all packages version independently, the workspace has four distinct **versioning semantics**
tiers. These describe **what a version bump signals** for external consumers — they do **not**
determine how publishing works.

| Tier                    | Version bump signals                                        | Example packages                                                                |
| ----------------------- | ----------------------------------------------------------- | ------------------------------------------------------------------------------- |
| **Tracker runtime**     | Tracker application behaviour or feature set changed        | `tracker-core`, `udp-server`, `primitives`, `http-protocol`, `axum-http-server` |
| **API contract**        | REST API or configuration schema changed                    | `rest-api-protocol`, `configuration`, `axum-rest-api-server`                    |
| **Platform/utility**    | Crate's own library API changed                             | `test-helpers`                                                                  |
| **Unpublished tooling** | Version changes only when internal API changes meaningfully | `e2e-tools`, `persistence-benchmark`, `workspace-coupling`                      |

GitHub Releases are used **only for tracker application releases**. Workspace packages
are published to crates.io only.

### Rationale

1. **Path dependencies guarantee compatibility**: since all inter-package dependencies use
   `path = "..."` within the workspace, Cargo always resolves the local copy regardless of
   the declared version number. Linked version numbers add no safety.
2. **Accurate SemVer signals**: external consumers can infer change risk from version
   numbers because each package's version reflects its own history, not the workspace's.
3. **Avoids unnecessary churn**: a bugfix in one package no longer forces a version bump
   on every unrelated package in the workspace.
4. **Aligns with EPIC #1669 extraction goal**: packages moving to standalone repositories
   already version independently. This formalises the same approach for every package.
5. **Emergent coupling, not imposed coupling**: if packages naturally evolve together over
   time, that coupling can be formalised later when there is evidence, not before.
6. **Glob safety**: `releases/v*` in GitHub Actions does **not** match `releases/pkg/...`
   because `*` does not cross `/` boundaries. This keeps trigger patterns mutually exclusive
   without complex negative matching.
7. **Tag prefixes disambiguate ownership**: `pkg/` prefix in tags makes it immediately
   clear which package a tag refers to, avoiding ambiguity with tracker app tags.

### CI Automation Design

Two separate workflows with complementary responsibilities:

| Aspect     | `deployment.yaml` (tracker) | `deployment-packages.yaml` (packages)      |
| ---------- | --------------------------- | ------------------------------------------ |
| Trigger    | `releases/v*`               | `releases/pkg/**` (or `workflow_dispatch`) |
| Publishes  | Only `torrust-tracker`      | Single crate extracted from branch name    |
| Role       | Tracker application release | Primary publishing path for all packages   |
| Complexity | Low (one crate)             | Low (one shot)                             |

**Why `deployment.yaml` publishes only one crate**: by the time a tracker release happens,
all dependency crates have already been published independently via `deployment-packages.yaml`
as they evolved during the development cycle. The tracker release is the final step that
publishes the binary crate consumers actually download.

### GitHub Releases

GitHub Releases (release notes, downloadable assets, etc.) are used **only for the tracker
application binary**. The tracker binary is the primary deliverable for end-users; workspace
crates are library/tool code consumed via crates.io.

For workspace packages, the crate README and `Cargo.toml` metadata serve as the documentation
surface. crates.io handles distribution and version tracking.

### What Does Not Change

- The existing **tracker application release process** (branch, tag, PR into `main`,
  CI deployment) continues to work — it now only publishes `torrust-tracker` itself.
- Path dependencies within the workspace are unaffected — Cargo always resolves the
  local copy regardless of the declared version number.

### Version by Namespace for Public Contracts

The project uses a **version by namespace** pattern
for public contracts — the REST API and configuration schema. Multiple
protocol/schema versions coexist in the same branch under versioned namespace
modules (`rest-api-protocol/src/v1/`, `configuration/src/v2_0_0/`). This is the
agreed approach; versioning via separate Git branches (branch-based versioning)
was considered and rejected for this project.

From the issue spec's pros/cons analysis, the key reasons are:

- Multiple API/config versions coexist during long migration periods without branch
  management overhead.
- Consumer migration is incremental — old and new code coexist.
- Configuration schema migration scripts can read/write both old and new schemas.
- A single CI pipeline tests all supported versions together.
- hotfixes apply to all supported versions simultaneously without cherry-pick effort.

### Why API Contract Packages Still Version Independently

The REST API server, client, and protocol packages share a wire protocol, but they
still version independently in `Cargo.toml`:

- The API contract version is tracked by the **`v1/` namespace**, not the `Cargo.toml` version.
- `Cargo.toml` versions are a **distribution/packaging concern** — they track the crate's
  release history, not the API contract.
- A bugfix in the client's HTTP transport layer should not force a server version bump.
- The convention "major.minor should reflect the API contract; patches are independent"
  is sufficient without mechanical enforcement.
- The crates.io dependency solver handles compatibility naturally via version constraints
  in downstream `Cargo.toml` files.

### Alternatives Considered

#### A) Keep all crates on one shared workspace version (discarded)

Why considered: minimal tooling complexity, very easy coordinated release process.

Why discarded: over-couples unrelated packages and inflates churn; weak SemVer signal for
external consumers; conflicts with EPIC extraction goals and independent release cadence.

#### B) Hybrid two-tier strategy (discarded)

Why considered: appeared to balance coordination simplicity for tightly-coupled runtime
crates against independent evolution for utility crates.

Why discarded: the linked-tier advantage is illusory — path dependencies already guarantee
compatibility within the workspace, so linked version numbers add no safety. Imposes a
guess about future coupling that may not hold. Adds unnecessary policy complexity over
the simple "all independent" approach.

#### C) Link versions for API contract packages only (discarded)

Why considered: the REST API server and client share a wire protocol — bumping the API
version on the server without a matching client bump would confuse consumers.

Why discarded: the coupling is already handled by version by namespace (`v1/` modules);
the `Cargo.toml` version is a distribution concern, not a protocol version indicator.
Linking them would reintroduce unnecessary churn. See [Version by Namespace](#version-by-namespace-for-public-contracts)
for the full rationale.

## Date

2026-06-29

## References

- Issue: [#1926](https://github.com/torrust/torrust-tracker/issues/1926) — Define package versioning strategy
- Issue spec: [`docs/issues/open/1926-1669-si-32-define-package-versioning-strategy.md`](../../docs/issues/open/1926-1669-si-32-define-package-versioning-strategy.md)
- EPIC: [#1669](https://github.com/torrust/torrust-tracker/issues/1669) — Overhaul: Packages
- ADR: [20260527175600](20260527175600_keep_protocol_and_domain_types_decoupled.md) — related ADR on protocol/domain decoupling
