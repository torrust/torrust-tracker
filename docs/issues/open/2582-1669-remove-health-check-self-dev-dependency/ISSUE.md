---
schema-version: 1
doc-type: issue
issue-type: bug
status: planned
priority: p2
epic: 1669
github-issue: 2582
spec-path: docs/issues/open/2582-1669-remove-health-check-self-dev-dependency/ISSUE.md
branch: "2582-1669-remove-health-check-self-dev-dependency"
related-pr: null
last-updated-utc: "2026-10-20 17:41"
semantic-links:
  skill-links:
    - create-issue
    - fix-bug
  related-artifacts:
    - .github/skills/dev/debugging/fix-bug/SKILL.md
    - .github/skills/dev/planning/create-issue/SKILL.md
    - "issue #1669"
    - docs/issues/open/1669-overhaul-packages/EPIC.md
    - "issue #2583"
    - docs/issues/open/2583-1669-first-publish-of-workspace-crates/ISSUE.md
    - packages/axum-health-check-api-server/Cargo.toml
---

<!-- skill-link: create-issue -->

# Issue #2582 - Remove the self dev-dependency that stops the health check API server crate from being packaged

Parent EPIC: #1669 (Overhaul: Packages). Pre-publish sub-issue P1 of the first-publish tracking
issue #2583 ([`ISSUE.md`](../2583-1669-first-publish-of-workspace-crates/ISSUE.md)).

## Goal

`torrust-tracker-axum-health-check-api-server` can be packaged and published, so
`cargo package --workspace` succeeds.

## Background

What happens, step by step:

1. `packages/axum-health-check-api-server/Cargo.toml` lists the crate itself under
   `[dev-dependencies]`:
   `torrust-tracker-axum-health-check-api-server = { version = "0.1.0", path = "../axum-health-check-api-server" }`.
   The line dates from before the #1816 rename and serves no purpose: Cargo already lets the
   crate's integration tests in `tests/` import the package under test.
2. `cargo package` keeps a dev-dependency that has a `version` and resolves it against the
   registry. `cargo package --workspace` also sees the crates it has already packaged in the same
   run, but a crate is never packaged before itself.
3. So the requirement `torrust-tracker-axum-health-check-api-server ^0.1.0` cannot be met, and
   packaging fails with `no matching package named torrust-tracker-axum-health-check-api-server found`.
   `cargo publish` fails the same way.

Expectation violated: the
[independent versioning ADR](../../../adrs/20260629000000_adopt_independent_package_versioning.md)
says every crate without `publish = false` is publishable through `deployment-packages.yaml`.

Impact: nothing is wrong at runtime or in tests today. The crate cannot be published, and
`cargo package --workspace` stops at it, so the publish tracking issue cannot check the workspace
in one command. Found while drafting that issue on 2026-10-20.

Reproduction: **Reproduced** on `develop` (`5b54bb0f4`) with the stable Rust toolchain (cargo
1.99.0); see [manual-verification-evidence.md](manual-verification-evidence.md) V1. A probe (V2)
showed that removing the line keeps the crate's tests green and lets all 32 workspace members
package.

## Scope

### In Scope

- Remove the self dev-dependency from `packages/axum-health-check-api-server/Cargo.toml`.
- A CI step that runs `cargo package --workspace --no-verify` (maintainer decision, 2026-10-20).

### Out of Scope

- Any other packaging or publishing work (the publish tracking issue).
- The crate's API, tests, README or other metadata (#2482, #2576).

## Architectural Decisions

- Related ADRs:
  [`20260629000000_adopt_independent_package_versioning.md`](../../../adrs/20260629000000_adopt_independent_package_versioning.md).
- ADRs to create: none known.

## Design and Ownership Review

Not applicable: a one-line manifest change.

## Bug-Fix Process

Follows [fix-bug](../../../../.github/skills/dev/debugging/fix-bug/SKILL.md):

1. Analysis: done above (self dev-dependency with a `version` requirement).
2. Real-artifact reproduction: done, evidence V1.
3. Regression boundary: see below.
4. Red: run the regression check on the unfixed tree and record the failure.
5. Fix: delete the line; `cargo` updates `Cargo.lock` if needed.
6. Green and recheck: the regression check passes; `cargo test -p
   torrust-tracker-axum-health-check-api-server` passes; repeat V1 like-for-like.

## Regression Test Strategy

There is no unit or integration seam: the defect lives in the manifest and shows only when Cargo
packages the crate. The smallest maintained check is `cargo package --workspace --no-verify`, which
fails for this defect and also for any other crate that cannot be packaged (a path dependency
without `version`, a dependency on a `publish = false` crate). It does not compile anything; on the
machine above it took about 20 seconds with a warm index.

The maintainer decided on 2026-10-20 to run it as a CI step. The implementation adds it with the
`implement-workflow` skill, records its red run on the unfixed tree, then fixes the manifest.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| -- | ------ | ---- | ----------------------- |
| T1 | DONE | Reproduce | Evidence V1: exit `101`, "no matching package named `torrust-tracker-axum-health-check-api-server`". |
| T2 | TODO | Add the CI packaging check | `cargo package --workspace --no-verify` in a CI workflow; it fails on the unfixed tree (red run, locally and in CI, recorded in evidence). |
| T3 | TODO | Remove the self dev-dependency | One-line manifest change (and `Cargo.lock` if Cargo changes it). |
| T4 | TODO | Green and recheck | The regression check and the crate's tests pass; V1 repeated like-for-like and passes; recorded in evidence. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T2 | Regression check (red evidence recorded) | One `ci(...)` commit. |
| T3 | Manifest fix | One `fix(axum-health-check-api-server)` commit. |
| T4 | Green and recheck evidence | One `docs(issues)` commit. |

Every commit is a signed Conventional Commit.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/1669-remove-health-check-self-dev-dependency/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, regression check, crate tests, pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-10-20 17:35 UTC - GitHub Copilot (Claude Opus 5.5) - Drafted from the `cargo package`
  failure found while drafting the first-publish tracking issue; reproduced it and probed the fix
  ([evidence](manual-verification-evidence.md) V1, V2).
- 2026-10-20 17:41 UTC - GitHub Copilot (Claude Opus 5.5) - The maintainer approved the spec and chose a CI
  step running `cargo package --workspace --no-verify` as the regression check. Created #2582
  (labels `Bug`, `packages`), linked it as a subissue of #1669, and moved the spec to
  `docs/issues/open/`. It lands through the spec-only PR of #2583; `branch` names the
  implementation branch.

## Acceptance Criteria

- [ ] AC1: `packages/axum-health-check-api-server/Cargo.toml` has no dependency on its own crate.
- [ ] AC2: `cargo package --workspace --no-verify` exits `0` on the fixed tree (it exited `101`
      before the fix, evidence V1).
- [ ] AC3: `cargo test -p torrust-tracker-axum-health-check-api-server` passes.
- [ ] AC4: CI runs `cargo package --workspace --no-verify`; the step failed on the unfixed tree and
      passes on the fixed tree.
- [ ] `linter all` exits with code `0`
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

### Automatic Checks

- `linter all`
- The CI packaging check and `cargo test -p torrust-tracker-axum-health-check-api-server`
- `./contrib/dev-tools/git/hooks/pre-push.sh` before opening the pull request

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| -- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1 | Reproduce on the unfixed tree | `cargo package --workspace --no-verify` | Exit `101` at the health check crate | DONE | `manual-verification-evidence.md` section V1 |
| M2 | Like-for-like recheck after the fix | Same command on the fixed tree | Exit `0`; every member packaged | TODO | `manual-verification-evidence.md` section V3 |

### Disposable Verification Scripts

None.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | TODO | |
| AC2 | TODO | |
| AC3 | TODO | |
| AC4 | TODO | |

## Risks and Trade-offs

- **CI time.** A packaging step updates the crates.io index on every run. Mitigation: `--no-verify`
  skips compilation; the step can share the existing Cargo cache.

## Implementation Completion Review

- Retrospective: `Not yet assessed`
- If no retrospective is needed, add a concise progress-log entry explaining why.

## References

- Parent EPIC: #1669, [`EPIC.md`](../../open/1669-overhaul-packages/EPIC.md)
- Publish tracking issue: #2583 ([`ISSUE.md`](../2583-1669-first-publish-of-workspace-crates/ISSUE.md))
- Rename that carried the line over: #1816
