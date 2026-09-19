---
doc-type: issue
issue-type: bug
status: in-progress
priority: p2
epic: null
github-issue: 2179
spec-path: docs/issues/open/2179-fix-docker-e2e-package-flag/ISSUE.md
branch: 2179-fix-docker-e2e-package-flag
related-pr: null
last-updated-utc: 2026-09-19 00:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - .github/workflows/testing.yaml
    - .github/workflows/container.yaml
---

<!-- skill-link: create-issue -->

# Issue #2179 - Fix Docker E2E Package Selection

## Goal

Restore the `Docker E2E` job in `.github/workflows/testing.yaml` so its four runner steps resolve
the binaries from the `torrust-tracker-e2e-tools` package and execute on feature-branch pushes.

## Background

The workspace root is the `torrust-tracker` package and has no `e2e_tests_runner` or
`qbittorrent_e2e_runner` binary. The `docker-e2e` job invokes those binaries without selecting
their owning `torrust-tracker-e2e-tools` package, so Cargo exits with code 101 before the tests run.

The equivalent invocations in `.github/workflows/container.yaml` already select the package. That
workflow does not run for feature-branch pushes, making the broken job the only intended Docker E2E
coverage for those events.

## Scope

### In Scope

- Add `-p torrust-tracker-e2e-tools` to all four `cargo run` commands in the `docker-e2e` job.
- Preserve every other runner argument.
- Wrap the longest command in a folded YAML block scalar to satisfy the 200-character limit.
- Explain in the job comment why explicit package selection is required.
- Record feature-branch CI evidence.

### Out of Scope

- Changing workflow triggers or the `docker-e2e` job guard.
- Deduplicating `.github/workflows/testing.yaml` and `.github/workflows/container.yaml`.
- Changing runners, images, package layout, or `.github/workflows/container.yaml`.
- Correcting the stale `src/bin/` inventory in `AGENTS.md`.

## Architectural Decisions

- Related ADRs: None.
- ADRs to create: None expected; this restores an established workflow invocation pattern.

## Design and Ownership Review

Not applicable. The change only corrects package selection in GitHub Actions commands.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | DONE | Reproduce target resolution failure | Root command fails and identifies the owning package. |
| T2 | TODO | Correct four E2E runner invocations | All commands explicitly select `torrust-tracker-e2e-tools`. |
| T3 | TODO | Validate locally and on feature-branch CI | Linters pass and the `Docker E2E` job succeeds. |
| T4 | TODO | Review acceptance and completion evidence | Spec records observed results and any deviations. |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| Specification | Add the approved local issue specification | Commit before implementation. |
| T2-T4 | Fix the workflow and record verification evidence | Commit after focused validation and review. |

No automated test code is planned because the defect is GitHub Actions configuration and the
feature-branch workflow run is the behavior-level regression check.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec created in `docs/issues/open/2179-fix-docker-e2e-package-flag/ISSUE.md`
- [x] Existing GitHub issue and maintainer-authored scope reviewed
- [x] GitHub issue number added to this spec
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all` and focused command checks)
- [ ] Manual verification scenarios executed and recorded in issue-local evidence
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md`
- [ ] Committer verified spec progress is up to date before commit
- [ ] Pull request opened against `develop`
- [ ] Issue closed and spec moved to `docs/issues/closed/`

### Progress Log

- 2026-09-19 00:00 UTC - GitHub Copilot - Reviewed issue #2179; no clarification was required because
  the exact change, exclusions, and verification requirements are specified.
- 2026-09-19 00:00 UTC - GitHub Copilot - Reproduced the root-level Cargo target resolution failure:
  Cargo identified `torrust-tracker-e2e-tools` as the package containing `e2e_tests_runner`.

## Acceptance Criteria

- [ ] AC1: All four `cargo run` invocations in `docker-e2e` select
  `torrust-tracker-e2e-tools`, with every other argument unchanged.
- [ ] AC2: A feature-branch push runs `Docker E2E`; all four runner steps resolve and the job passes.
- [ ] AC3: `.github/workflows/container.yaml` is unchanged.
- [ ] AC4: The `docker-e2e` comment explains why the package selector is required.
- [ ] `linter all` exits with code `0`.
- [ ] Acceptance criteria are re-reviewed against observed behavior.

## Verification Plan

### Automatic Checks

- Before the fix, `cargo run --bin e2e_tests_runner -- --help` fails with Cargo exit code 101.
- After the fix, `cargo run -p torrust-tracker-e2e-tools --bin e2e_tests_runner -- --help` resolves
  the binary using the stable Rust toolchain.
- `linter all`.
- Confirm `.github/workflows/container.yaml` has no diff.

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented steps | Expected Result | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| M1 | Feature-branch Docker E2E | Push this branch and inspect the `testing.yaml` workflow run. | `Docker E2E` runs and passes all four runner steps. | TODO | `manual-verification-evidence.md` section V1 |
| M2 | Pull request to `develop` | Open the PR and inspect its `testing.yaml` jobs. | `Docker E2E` remains skipped by the existing guard. | TODO | `manual-verification-evidence.md` section V2 |

Create `manual-verification-evidence.md` from the repository template when executing these
scenarios. Record the workflow URL, triggering ref, relevant step outcomes, and observed result.

### Disposable Verification Scripts

None planned.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| --- | --- | --- |
| AC1 | TODO | Workflow diff and local target-resolution check |
| AC2 | TODO | Feature-branch GitHub Actions run URL |
| AC3 | TODO | Git diff |
| AC4 | TODO | Workflow diff |

## Risks and Trade-offs

- The local command can prove Cargo target resolution but not the full container workflow; M1
  supplies the required behavior-level evidence.
- A PR targeting `develop` skips this job by design; both branch-push and PR observations are needed.

## Implementation Completion Review

- Retrospective: Not yet assessed.
- Create `implementation-retrospective.md` only if implementation reveals a reusable lesson,
  material design change, or deviation; otherwise record why none is needed in the progress log.
- Record the independent Task Reviewer result in `agent-review-reports.md`.

## References

- GitHub issue: [#2179](https://github.com/torrust/torrust-tracker/issues/2179)
- Related issue: [#1854](https://github.com/torrust/torrust-tracker/issues/1854)
- Related issue: [#1840](https://github.com/torrust/torrust-tracker/issues/1840)
