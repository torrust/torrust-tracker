---
schema-version: 1
doc-type: issue
issue-type: bug
status: in-progress
priority: p2
epic: 1669
github-issue: 2582
spec-path: docs/issues/open/2582-1669-remove-health-check-self-dev-dependency/ISSUE.md
branch: "2582-1669-remove-health-check-self-dev-dependency"
related-pr: null
last-updated-utc: "2026-10-21 09:51"
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
- The same check as a `pre-push.sh` step, and the documentation that lists the pre-push steps
  (maintainer decision, 2026-10-21; see [Maintainer Decisions](#maintainer-decisions)).

### Out of Scope

- Any other packaging or publishing work (the publish tracking issue).
- The crate's API, tests, README or other metadata (#2482, #2576).

## Architectural Decisions

- Related ADRs:
  [`20260629000000_adopt_independent_package_versioning.md`](../../../adrs/20260629000000_adopt_independent_package_versioning.md).
- ADRs to create: none known.

## Maintainer Decisions

Asked before implementation on 2026-10-21; answers from the maintainer.

1. **Where the CI check runs:** a new independent job `workspace-packaging` in
   `.github/workflows/testing.yaml`, stable toolchain, no `sccache` (it compiles nothing). The
   maintainer asked to record the pros and cons of every option considered:

   | Option | Pros | Cons |
   | ------ | ---- | ---- |
   | **New job in `testing.yaml` (chosen)** | Runs on every non-docs push and pull request, so a regression is caught before merge; reuses the workflow's path policy and concurrency group; a separate job fails with its own name, starts in parallel and does not lengthen the `unit` jobs; can become a required status check on its own. | One more runner per run (checkout, toolchain setup and index update, about a minute of runner time; relevant to the CI runner-time work of #2525). |
   | Extra step in the `unit` job (stable entry only) | No extra runner or toolchain setup; reuses the job's `cargo fetch`. | Needs a matrix flag; a packaging failure shows up as a `Unit (stable)` failure; runs after the slow steps of a 90-minute job or adds to its critical path; cannot be required independently. |
   | A new dedicated workflow file | Clear ownership; own triggers and path filters. | Duplicates triggers, path policy and concurrency settings already in `testing.yaml`; one more workflow to keep in sync. |
   | `deployment-packages.yaml` | Next to the publish step it protects. | Runs only on `releases/pkg/**` pushes and manual dispatch, so a regression is found only when someone tries to publish, which is the failure this issue fixes. |

2. **Command flags:** exactly `cargo package --workspace --no-verify` in CI. `--locked` and a
   full verify (which compiles every packaged crate in isolation) were not chosen.
3. **CI red run:** push the branch with only the CI commit to the fork
   (`josecelano/torrust-tracker`), record the failing fork `Testing` run, then add the fix.
4. **Pre-push:** also add the check to `contrib/dev-tools/git/hooks/pre-push.sh`, as
   `cargo +stable package --workspace --no-verify --allow-dirty`, placed before the
   "Running all tests" step (about 20 seconds, fails early). `--allow-dirty` keeps the hook usable
   with work in progress in the tree and matches the other pre-push steps, which also check the
   working tree rather than the pushed commits. CI keeps the strict form without `--allow-dirty`,
   since its checkout is always clean. The step needs the crates.io index, so the hook fails when
   offline.
5. **Required check:** the maintainer will make the new CI job a required status check in the
   ruleset after the job exists on `develop`; the pull request notes it.

## Design and Ownership Review

Not applicable: a one-line manifest change plus one CI job and one pre-push step that run an
existing Cargo command with no repository logic, so the `implement-workflow` rule about moving logic
into a repository tool does not apply.

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

The maintainer decided on 2026-10-20 to run it as a CI step, and on 2026-10-21 to run it in
`pre-push.sh` too. The implementation adds the CI job with the `implement-workflow` skill, records
its red run on the unfixed tree (locally and in the fork's CI), adds the pre-push step and records
its red run, then fixes the manifest.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| -- | ------ | ---- | ----------------------- |
| T1 | DONE | Reproduce | Evidence V1: exit `101`, "no matching package named `torrust-tracker-axum-health-check-api-server`". |
| T0 | DONE | Record the maintainer's answers | [Maintainer Decisions](#maintainer-decisions). |
| T2 | DONE | Add the CI packaging check | Job `workspace-packaging` in `testing.yaml` running `cargo package --workspace --no-verify`; it fails on the unfixed tree (red run, locally and in the fork's CI, recorded in evidence). Red: evidence V3 (local) and V4 (fork CI). |
| T2b | DONE | Add the pre-push packaging step | `cargo +stable package --workspace --no-verify --allow-dirty` before "Running all tests"; the pre-push step lists in the docs updated; the step fails on the unfixed tree (red run recorded in evidence). Red: evidence V5. |
| T3 | DONE | Remove the self dev-dependency | One-line manifest change; `Cargo.lock` drops the crate's edge to itself. |
| T4 | DONE | Green and recheck | The regression checks and the crate's tests pass; V1 repeated like-for-like and passes; recorded in evidence V6. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T0 | Maintainer decisions | One `docs(issues)` commit. |
| T2 | CI regression check (red evidence recorded) | One `ci(...)` commit, plus a `docs(issues)` commit for the hosted red run. |
| T2b | Pre-push step and its docs | One `chore(git-hooks)` commit. |
| T3 | Manifest fix | One `fix(axum-health-check-api-server)` commit. |
| T4 | Green and recheck evidence | One `docs(issues)` commit. |

Every commit is a signed Conventional Commit.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/1669-remove-health-check-self-dev-dependency/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, regression check, crate tests, pre-push checks)
- [x] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria reviewed after implementation and updated with evidence (by the implementing agent; not yet by a reviewer)
- [x] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
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
- 2026-10-21 09:27 UTC - GitHub Copilot - Started the implementation on branch
  `2582-1669-remove-health-check-self-dev-dependency` from `develop` at `afbd5dbf1`. Asked the
  maintainer the open questions and recorded his answers in
  [Maintainer Decisions](#maintainer-decisions): a new `testing.yaml` job, `--no-verify` only, the
  CI red run from the fork, and the same check in `pre-push.sh` with `--allow-dirty`. Added T0, T2b
  and AC5 for the pre-push step.
- 2026-10-21 09:39 UTC - GitHub Copilot - T2 and T2b done. Added the `Workspace Packaging` job to
  `testing.yaml` and the packaging step to `pre-push.sh` (with the documented step lists). Recorded
  the red runs on the unfixed tree: the CI command locally (V3), the job in the fork's CI (V4), and
  the pre-push step with a dirty tree (V5); all fail with the error of V1.
- 2026-10-21 09:51 UTC - GitHub Copilot - T3 and T4 done. Removed the self dev-dependency (`Cargo.lock`
  drops the self edge). On the fix commit the V1 command packages all 32 members, the crate's tests
  pass, the pre-push hook passes, and the fork's `Workspace Packaging` job is green (V6). No
  retrospective is needed: the work followed the approved plan, and the only additions (the
  pre-push step and its documentation) came from the maintainer's answers recorded before the
  work started. Seven `publish = false` dev-tool crates warn about missing manifest metadata
  when packaged; that is out of scope.

## Acceptance Criteria

- [x] AC1: `packages/axum-health-check-api-server/Cargo.toml` has no dependency on its own crate.
- [x] AC2: `cargo package --workspace --no-verify` exits `0` on the fixed tree (it exited `101`
      before the fix, evidence V1).
- [x] AC3: `cargo test -p torrust-tracker-axum-health-check-api-server` passes.
- [x] AC4: CI runs `cargo package --workspace --no-verify`; the step failed on the unfixed tree and
      passes on the fixed tree.
- [x] AC5: `pre-push.sh` runs `cargo +stable package --workspace --no-verify --allow-dirty` before
      the full test suite; the step failed on the unfixed tree and passes on the fixed tree, and
      the documented pre-push step lists name it.
- [x] `linter all` exits with code `0`
- [x] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [x] Documentation is updated when behavior/workflow changes (the pre-push step lists and gate summaries)

## Verification Plan

### Automatic Checks

- `linter all`
- The CI packaging check and `cargo test -p torrust-tracker-axum-health-check-api-server`
- The pre-push packaging step (red on the unfixed tree, green after the fix)
- `./contrib/dev-tools/git/hooks/pre-push.sh` before opening the pull request

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| -- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1 | Reproduce on the unfixed tree | `cargo package --workspace --no-verify` | Exit `101` at the health check crate | DONE | `manual-verification-evidence.md` section V1 |
| M2 | Like-for-like recheck after the fix | Same command on the fixed tree | Exit `0`; every member packaged | DONE | `manual-verification-evidence.md` section V6 |

### Disposable Verification Scripts

None.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | DONE | The fix commit removes the line; `grep -n axum-health-check-api-server packages/axum-health-check-api-server/Cargo.toml` finds only the `name`. |
| AC2 | DONE | Evidence V6 step 1: exit `0`, 32 members packaged (V1 and V3: exit `101`). |
| AC3 | DONE | Evidence V6 step 3: 3 unit and 8 integration tests pass. |
| AC4 | DONE | Job `Workspace Packaging` in `testing.yaml`: fork run red on the CI commit (V4), green on the fix commit (V6 step 4). |
| AC5 | DONE | Red with a dirty tree (V5), green (V6 step 2); listed in the `run-pre-push-checks` and `push-changes` skills. |

## Risks and Trade-offs

- **CI time.** A packaging step updates the crates.io index on every run. Mitigation: `--no-verify`
  skips compilation; the step can share the existing Cargo cache.
- **Offline pushes.** The pre-push step needs the crates.io index, so the hook fails without
  network access. Accepted by the maintainer.
- **Pre-push checks the working tree.** With `--allow-dirty` the hook packages uncommitted edits,
  so it can pass or fail differently from the pushed commits; the other pre-push steps behave the
  same way, and CI checks the commits strictly.

## Implementation Completion Review

- Retrospective: not needed; see the progress log entry of the implementation's completion.
- If no retrospective is needed, add a concise progress-log entry explaining why.

## References

- Parent EPIC: #1669, [`EPIC.md`](../../open/1669-overhaul-packages/EPIC.md)
- Publish tracking issue: #2583 ([`ISSUE.md`](../2583-1669-first-publish-of-workspace-crates/ISSUE.md))
- Rename that carried the line over: #1816
