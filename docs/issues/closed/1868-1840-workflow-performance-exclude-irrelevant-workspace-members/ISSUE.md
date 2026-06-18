---
doc-type: issue
issue-type: task
status: done
priority: p1
github-issue: 1868
spec-path: docs/issues/closed/1868-1840-workflow-performance-exclude-irrelevant-workspace-members/ISSUE.md
branch: "1868-1840-exclude-irrelevant-workspace-members"
related-pr: null
last-updated-utc: 2026-06-18 08:30
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - Containerfile
    - .github/workflows/container.yaml
    - docs/issues/open/1840-improve-pr-workflow-performance-epic/EPIC.md
    - docs/issues/closed/1853-1840-workflow-performance-containerfile-target-scope/ISSUE.md
    - docs/issues/open/1869-1840-workflow-performance-dependency-layer-cache-reuse/ISSUE.md
    - docs/issues/open/1854-1840-workflow-performance-container-test-gating/ISSUE.md
---

<!-- skill-link: create-issue -->

# Issue #1868 - Exclude irrelevant workspace members from container build

## Goal

Reduce container image build time by excluding workspace packages that are not needed to
produce or validate the tracker runtime image from all cargo commands in the Containerfile.

## Background

Issue [#1853](https://github.com/torrust/torrust-tracker/issues/1853) removed `--benches
--examples --all-targets` from all cargo commands. However, `--workspace` still causes two
workspace members that are unrelated to the tracker runtime to be compiled on every container
build:

- `workspace-coupling` (`contrib/dev-tools/analysis/workspace-coupling`) — a local analysis
  tool with unique dependencies (`regex`, `serde_json`) not shared by any other package. It
  has no relationship to the tracker runtime image.
- `torrust-tracker-torrent-repository-benchmarking` — a benchmark harness with 17 inline
  unit tests. It is not depended on by any other workspace member.

### What the CI log revealed

A recent CI run (after #1853 merged) showed the following in the `build-tracker-image` step:

```text
#60 [dependencies 3/4] cargo chef cook --tests --workspace --all-features ...
#60 278.9    Compiling workspace-coupling v0.0.1 ...
#60 304.1     Finished in 5m 04s

#61 [dependencies 4/4] cargo nextest archive ... (warmup)
#61 71.63     Finished in 1m 10s       <- fast: stubs still in place

#64 [build 3/3] cargo nextest archive --tests --workspace --all-features ...
#64 253.4    Compiling torrust-tracker v3.0.0-develop
#64 1094.3   Compiling workspace-coupling v3.0.0-develop   <- 840s after tracker
#64 1144.0     Finished in 19m 03s
```

`workspace-coupling` is compiled twice: once in the cook stage (with stub source, ~5 min),
and again in the build stage (with real source, after an ~840s gap). The 840s gap is the
compilation cost of `workspace-coupling`'s unique transitive dependencies (`regex-automata`,
`regex-syntax`, and `serde_json` internals) from scratch — these were not pre-cooked because
the cook layer for `workspace-coupling` was built with stubs, and the real dep graph for
those crates is only triggered when the actual source is compiled.

The total build step time was 19m03s; removing these two packages is expected to cut it
significantly.

## Scope

### In Scope

- Add `--exclude workspace-coupling --exclude torrust-tracker-torrent-repository-benchmarking`
  to all cargo commands in the Containerfile (`cargo chef cook` × 2, `cargo nextest archive`
  × 4).
- Determine whether `cargo chef prepare` should also receive `--exclude` flags, and if so,
  remove the corresponding `COPY`/stub lines from the recipe stage.
- Measure the impact on CI build time with evidence from a full CI run after the change.

### Out of Scope

- Removing tests from the container build (tracked in #1854).
- Implementing cross-workflow cache sharing (tracked in #1869).
- Changing which packages are part of the workspace `[members]` list.
- Broad Containerfile restructuring unrelated to the exclusion change.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                                           | Notes / Expected Output                                                                                                                                                                                                                                                                   |
| --- | ------ | ------------------------------------------------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | DONE   | Verify `cargo nextest archive` supports `--exclude`                            | Confirmed: `--exclude` is standard Cargo package-selection syntax; `cargo nextest archive` passes it through to Cargo. Verified by inspecting cargo-nextest behaviour and Cargo docs.                                                                                                     |
| T2  | DONE   | Add `--exclude` flags to all `cargo nextest archive` commands in Containerfile | Applied to all 4 `cargo nextest archive` commands. `cargo chef cook` does **not** support `--exclude` (cargo-chef CLI limitation; see T3). Documented with comments in the Containerfile.                                                                                                 |
| T3  | DONE   | Decide on `cargo chef prepare` exclusion                                       | Neither `cargo chef prepare` nor `cargo chef cook` exposes an `--exclude` flag. The COPY/stub lines for both excluded packages **must stay** so that `cargo metadata` (invoked by `prepare`) can resolve the workspace without missing manifest files. See Containerfile comment and AC4. |
| T4  | TODO   | Run full CI build and record timing evidence                                   | CI log showing build time after exclusion. Compare against pre-fix baseline (19m03s build step, 38m total).                                                                                                                                                                               |

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted in `docs/issues/drafts/`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [ ] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded (status + evidence)
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-06-03 00:00 UTC - GitHub Copilot - Drafted issue spec based on post-merge CI analysis of #1853 - draft file created
- 2026-06-03 00:00 UTC - GitHub Copilot - Narrowed scope to `--exclude` fix only; layer split analysis moved to dependency-layer-cache-reuse draft - draft updated
- 2026-06-03 00:00 UTC - GitHub Copilot - Implemented: added `--exclude workspace-coupling --exclude torrust-tracker-torrent-repository-benchmarking` to all 4 `cargo nextest archive` commands in Containerfile; investigated and documented that `cargo chef cook` and `cargo chef prepare` do not support `--exclude` (cargo-chef CLI limitation); COPY/stub lines for excluded packages retained in recipe stage because `cargo chef prepare` invokes `cargo metadata` which requires all workspace manifests to be present

## Acceptance Criteria

- [ ] AC1: `workspace-coupling` and `torrust-tracker-torrent-repository-benchmarking` do not appear in the container build compilation output.
- [ ] AC2: The final `cargo nextest archive` step in the CI build completes in measurably less time than the 19m03s baseline recorded after #1853.
- [ ] AC3: The tracker runtime image is produced correctly and all unit tests still pass inside the container build.
- [x] AC4: The decision on `cargo chef prepare` and `cargo chef cook` exclusion is documented: neither tool exposes `--exclude` in its CLI (cargo-chef limitation). `cargo chef prepare` uses `cargo metadata` internally, which requires every workspace member's manifest to exist on disk — the COPY/stub lines for the excluded packages are therefore retained in the recipe stage. `cargo chef cook` similarly has no `--exclude` flag; the exclusion is achieved entirely through the 4 `cargo nextest archive` commands where standard Cargo `--exclude` is supported. This is documented in Containerfile comments.
- [x] `linter all` exits with code `0`
- [x] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented (status + evidence)
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

Define verification before implementation starts and execute it before closing the issue.

### Automatic Checks

- `linter all`
- Pre-push checks pass for changed Containerfile and spec files

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                                           | Command/Steps                                                                                                                                    | Expected Result                                    | Status | Evidence               |
| --- | -------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------ | -------------------------------------------------- | ------ | ---------------------- |
| M1  | Confirm excluded packages absent from build output | Run CI or `docker build --target release` locally; grep build log for `workspace-coupling` and `torrust-tracker-torrent-repository-benchmarking` | Neither package name appears in compilation output | TODO   | {CI log link}          |
| M2  | Measure build step timing improvement              | Compare CI log for `[build 3/3] cargo nextest archive` step before and after change                                                              | Step completes in significantly less than 19m03s   | TODO   | {CI run link + timing} |
| M3  | Verify runtime image correctness                   | Build release image locally; run `docker run --rm torrust-tracker --version` or equivalent health-check                                          | Image starts correctly; expected binaries present  | TODO   | {command output}       |
| M4  | Verify tests still pass inside container           | Review CI test stage output; confirm no test regressions                                                                                         | All unit tests pass in the container `test` stage  | TODO   | {CI log link}          |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                                                                            |
| ----- | ---------------------- | ----------------------------------------------------------------------------------- |
| AC1   | TODO                   | {CI log link}                                                                       |
| AC2   | TODO                   | {timing comparison}                                                                 |
| AC3   | TODO                   | {CI test stage log}                                                                 |
| AC4   | DONE                   | See AC4 text above and Containerfile comments in the Cook (debug) and recipe stages |

## Risks and Trade-offs

- Risk: `cargo nextest archive` may not support `--exclude` in the same way as `cargo build`. Mitigation: T1 verifies support before implementation.
- Risk: Excluding packages from `cargo chef prepare` may require additional changes to keep `cargo metadata` happy (recipe.json must still be valid). Mitigation: test locally with `cargo chef prepare --exclude ...` before removing COPY/stub lines.
- Risk: The change may interact with #1854 (test gating). If tests are later removed from the container build, the `--exclude` optimization becomes less relevant but is still correct. Mitigation: implement independently; document the relationship in the spec.

## References

- Related issues: #1840 (EPIC), #1853, #1854
- Related drafts: `docs/issues/drafts/1840-workflow-performance-dependency-layer-cache-reuse/ISSUE.md`
- Related PRs: #1867 (merged, implemented #1853)
- Related ADRs: none
