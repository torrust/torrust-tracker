---
schema-version: 1
doc-type: issue
issue-type: task
status: draft
priority: p2
epic: 2482
github-issue: null
spec-path: docs/issues/drafts/2482-build-shipped-artifacts-without-test-features/ISSUE.md
branch: "{issue-number}-2482-build-shipped-artifacts-without-test-features"
related-pr: null
last-updated-utc: "2026-10-22 17:20"
semantic-links:
  skill-links:
    - create-issue
    - implement-workflow
  related-artifacts:
    - docs/adrs/20261021190318_share_workspace_test_support_through_testing_features.md
    - docs/issues/open/2482-1669-reorganize-shared-test-support/EPIC.md
    - docs/issues/open/2618-2482-decide-test-support-rules/manual-verification-evidence.md
    - Containerfile
    - .github/workflows/testing.yaml
    - .github/workflows/deployment.yaml
    - .github/workflows/deployment-packages.yaml
    - .github/workflows/container.yaml
    - docs/containers.md
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Build the shipped container binary without test features

Parent EPIC: #2482 (Reorganize test support code shared between workspace packages), row 9 of its
Subissues table. Draft written by subissue 1 (#2618) after its pre-PR review and the maintainer's
decision of 2026-10-22. Execution order: after row 8 and before row 3, the first move that adds a
`testing` feature.

## Goal

Make every build that produces a shipped artifact (the container image's binaries first) enable only
the features that are meant for production, by building it on its own with explicit features, and
fail CI if a `testing` feature ever reaches it. This keeps the guarantee of
[rule A.5 of the ADR](../../../adrs/20261021190318_share_workspace_test_support_through_testing_features.md)
true for the main shipped artifact.

## Background

The release container image gets `torrust-tracker` and `http_health_check` from the extracted test
archive: `Containerfile` ("Build Archive (release)") runs
`cargo nextest archive --tests --all-features ... --release`, and the `test` stage copies
`target/release/torrust-tracker` and `target/release/http_health_check` out of it. The debug image
does the same with `target/debug/torrust-tracker`. The archive is built over the root
`[workspace] default-members`, which include the four server packages and `tracker-core`.

The verification (evidence V1 and V4c) shows that an invocation that also builds tests
(`--tests`, `--all-targets`) enables dev-dependency features for the package's binary even without
`--all-features`, and `--all-features` enables every feature of the selected packages. After rows 3
to 7, that invocation would build every `testing` feature, its optional dependencies and `mockall`
into the shipped binaries. Before row 3 nothing changes, because no `testing` feature exists yet
(the root crate has no `[features]` table), which is why this row runs first. The maintainer
decided on 2026-10-22 that test support is production code that must not ship unless asked for,
that the container tests need it but the release image does not, that dropping `--all-features` in
favour of explicit features is acceptable, and that a CI error for a missing required feature is
preferable to shipping an unintended one.

## Scope

### In Scope

- Audit every command that produces a shipped artifact for `--all-features`, `--all-targets`,
  `--tests`, `--examples` and `--benches`: `Containerfile` (both archive stages and both cook
  stages), `.github/workflows/deployment.yaml`, `deployment-packages.yaml` and `container.yaml`, and
  any release script. Record each command and its flags.
- Change the `Containerfile` so the shipped binaries (`torrust-tracker`, `http_health_check`) are
  built by an invocation of their own (`cargo build --release` for the binaries, without test flags
  and with explicit features), while the nextest test archive keeps `--tests` and names the features
  it needs instead of `--all-features` where practical. Cover the debug image too.
- Add a check that fails the build if a `testing` feature is enabled in a shipped binary's
  dependency tree. Shell for simple orchestration; Rust if the logic grows (the repository's
  shell-versus-Rust threshold).
- Update `docs/containers.md` if it describes these builds.

### Out of Scope

- Moving any test support code (rows 2 to 7) and the coupling tool (row 8).
- Changing what the container tests verify, or the runtime image layout.
- Test-only workflows that use `--all-features` on purpose (`testing.yaml`, coverage): they build
  and run tests, not shipped artifacts.

## Architectural Decisions

- Related ADRs: [`20261021190318`](../../../adrs/20261021190318_share_workspace_test_support_through_testing_features.md)
  (rule A.5: a shipped artifact is built by an invocation of its own, with explicit features and a
  check that no `testing` feature is enabled).
- ADRs to create: None known.

## Design and Ownership Review

The `Containerfile` owns how shipped binaries are built; the root crate's manifest owns which
features they enable. The cache layers (`cargo chef cook`) currently warm dependencies with test
features; the new build changes the feature set of workspace crates, so the layering must keep the
test archive's cache and may rebuild third-party layers once. No process or I/O ownership changes.
After the first working image build, review the cache layering and the check before the
workflows are touched.

## Bug-Fix Process

Not applicable.

## Regression Test Strategy

Not applicable. The container build and its test stage are the regression suite; the new check has
its own negative test (see the verification plan).

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | TODO | Audit the build commands of shipped artifacts | A table in the evidence: command, file, flags, whether it ships |
| T2 | TODO | Build the shipped binaries on their own in the `Containerfile` (release and debug images) | Image builds; its tests still run in the test stage |
| T3 | TODO | Replace `--all-features` in the test archive by explicit features where practical | Same test count in the image as before |
| T4 | TODO | Add the check that no `testing` feature reaches a shipped binary | Passes now; fails when a `testing` feature is added to the shipped feature set |
| T5 | TODO | Update docs | `docs/containers.md` describes the two builds if it describes builds |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T1 | Audit evidence | One commit. |
| T2 | `Containerfile` stages | Commit after a local image build succeeds. |
| T3 | Test archive flags | Commit after the image's test stage passes. |
| T4 | The check and its wiring | Commit after the negative test fails as expected and the positive passes. |
| T5 | Docs | One commit. |

## Progress Tracking

### Workflow Checkpoints

- [ ] Folder-style spec drafted in `docs/issues/drafts/2482-build-shipped-artifacts-without-test-features/ISSUE.md`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] GitHub issue created, linked as a sub-issue of #2482, and its number added to this spec
- [ ] Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-10-22 17:20 UTC - AI assistant (Copilot SDK in VS Code, #2482 subissue 1) - Drafted after the
  pre-PR review of #2618 found that the release image's binary is built by
  `cargo nextest archive --tests --all-features --release`, and after the maintainer's decision of
  2026-10-22 to build shipped artifacts with explicit features and fail CI on an unintended one.

## Acceptance Criteria

- [ ] AC1: Every command that produces a shipped artifact is listed with its flags, and none uses
      `--all-features`, `--all-targets`, `--tests`, `--examples` or `--benches`.
- [ ] AC2: The container image's shipped binaries are built by an invocation of their own with
      explicit features; the image's test stage still runs the same tests.
- [ ] AC3: A check fails the build when a `testing` feature is enabled in a shipped binary's
      dependency tree, and passes otherwise.
- [ ] AC4: `docs/containers.md` describes the builds if it describes builds today.
- [ ] `linter all` exits with code `0`; pre-push checks pass; the container workflow is green.

## Verification Plan

The [#2482 default completion policy](../../open/2482-1669-reorganize-shared-test-support/EPIC.md#how-we-work-in-this-epic)
applies, except that this subissue moves no test code. The workflow changes follow the
`implement-workflow` skill.

### Automatic Checks

- `linter all`, `hadolint` on the `Containerfile` (part of pre-commit), pre-push checks, and the
  container workflow on the pull request.

### Manual Verification Scenarios

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| M1 | The image builds and its tests run | Build the `release` and `debug` targets of the `Containerfile` locally (`docker build --target release .`, `docker build --target debug .`) | Both build; the test stage runs the baseline test count | TODO | `manual-verification-evidence.md` V1 |
| M2 | The shipped binary has no `testing` feature | Run the check, and `cargo tree -p torrust-tracker -e normal,features` for the shipped build | No `testing` feature | TODO | V2 |
| M3 | The check fails when it should | Temporarily add a `testing` feature to the shipped feature set (a scratch change, not committed) and run the check | The check fails and names the feature | TODO | V3 |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | TODO                   |          |
| AC2   | TODO                   |          |
| AC3   | TODO                   |          |
| AC4   | TODO                   |          |

## Risks and Trade-offs

- Building the shipped binary separately can invalidate cached layers and lengthen the container
  workflow. Mitigation: keep the test archive's cache, measure the build time before and after, and
  record it.
- Without the check, a later change could re-enable a `testing` feature unnoticed. Mitigation: the
  check is part of this subissue, with a negative test.
- Rows 3 to 7 assume this row landed first. Mitigation: the execution order puts it before row 3.

## Implementation Completion Review

- Retrospective: `Not yet assessed`
- If no retrospective is needed, add a progress-log entry saying why.

## References

- Parent EPIC: [`#2482`](../../open/2482-1669-reorganize-shared-test-support/EPIC.md); ADR above
- Verification evidence V1 and V4c: [`manual-verification-evidence.md`](../../open/2618-2482-decide-test-support-rules/manual-verification-evidence.md)
