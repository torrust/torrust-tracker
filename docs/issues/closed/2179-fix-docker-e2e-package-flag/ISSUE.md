<!-- markdownlint-disable MD003 -->
doc-type: issue
issue-type: bug
status: done
priority: p2
epic: null
github-issue: 2179
spec-path: docs/issues/closed/2179-fix-docker-e2e-package-flag/ISSUE.md
branch: 2179-fix-docker-e2e-package-flag
related-pr: 2272
last-updated-utc: 2026-09-25
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
| T2 | DONE | Correct four E2E runner invocations | All commands explicitly select `torrust-tracker-e2e-tools`. |
| T3 | DONE | Validate locally and on feature-branch CI | Local checks, feature-branch CI, and PR guard check pass. |
| T4 | DONE | Review acceptance and completion evidence | AC1-AC4 pass; no implementation deviation occurred. |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| Specification | Add the approved local issue specification | Commit before implementation. |
| T2-T4 | Fix the workflow and record verification evidence | Commit after focused validation and review. |

Issue #2179 proposed a spec-only PR followed by the fix. The `create-issue` skill makes that
sequence optional for narrow work, so this four-command workflow correction combines both while
preserving the required spec-first commit order.

No automated test code is planned because the defect is GitHub Actions configuration and the
feature-branch workflow run is the behavior-level regression check.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec created in `docs/issues/open/2179-fix-docker-e2e-package-flag/ISSUE.md`
- [x] Existing GitHub issue and maintainer-authored scope reviewed
- [x] GitHub issue number added to this spec
- [x] Implementation completed
- [x] Automatic verification completed (`linter all` and focused command checks)
- [x] Manual verification scenarios executed and recorded in issue-local evidence
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [x] Evidence-based implementation completion review recorded
- [x] Reviewer validated acceptance criteria and updated checkboxes
- [x] Independent reviewer reports recorded in issue-local `agent-review-reports.md`
- [x] Committer verified spec progress is up to date before commit
- [x] Pull request opened against `develop`
- [x] Issue closed and spec moved to `docs/issues/closed/`

### Progress Log

- 2026-09-25 - GitHub Copilot - Verified GitHub issue #2179 is closed and archived this specification in `docs/issues/closed/`.

- 2026-09-19 11:59 UTC - GitHub Copilot - Reviewed issue #2179; no clarification was required because
  the exact change, exclusions, and verification requirements are specified.
- 2026-09-19 11:59 UTC - GitHub Copilot - Reproduced the root-level Cargo target resolution failure:
  Cargo identified `torrust-tracker-e2e-tools` as the package containing `e2e_tests_runner`.
- 2026-09-19 12:01 UTC - GitHub Copilot - Added explicit package selection to all four commands;
  YAML lint, both package-qualified help commands, and `linter all` pass. Feature-branch CI remains.
- 2026-09-19 13:17 UTC - GitHub Copilot - Feature-branch `Testing` run 35443663968 passed;
  `Docker E2E` ran successfully, including all four E2E runner steps.
- 2026-09-19 13:20 UTC - Task Reviewer - AC1-AC4 pass without scope creep; requested completion
  bookkeeping before re-review.
- 2026-09-19 13:21 UTC - GitHub Copilot - Re-reviewed AC1-AC4 against the workflow diff and CI
  evidence. No retrospective is needed because implementation matched the specified established
  pattern exactly and revealed no reusable design lesson or deviation.
- 2026-09-19 16:14 UTC - GitHub Copilot - Opened PR #2272 targeting `develop`; its `Testing` run
  skipped `Docker E2E` under the unchanged guard, completing M2.
- 2026-09-19 21:40 UTC - GitHub Copilot - Addressed Cameron's three spec-record findings and
  Copilot's two stable semantic-reference findings from PR #2272.

## Acceptance Criteria

- [x] AC1: All four `cargo run` invocations in `docker-e2e` select
  `torrust-tracker-e2e-tools`, with every other argument unchanged.
- [x] AC2: A feature-branch push runs `Docker E2E`; all four runner steps resolve and the job passes.
- [x] AC3: `.github/workflows/container.yaml` is unchanged.
- [x] AC4: The `docker-e2e` comment explains why the package selector is required.
- [x] `linter all` exits with code `0`.
- [x] Acceptance criteria are re-reviewed against observed behavior.

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
| M1 | Feature-branch Docker E2E | Push this branch and inspect the `testing.yaml` workflow run. | `Docker E2E` runs and passes all four runner steps. | DONE | `manual-verification-evidence.md` section V1 |
| M2 | Pull request to `develop` | Open the PR and inspect its `testing.yaml` jobs. | `Docker E2E` remains skipped by the existing guard. | DONE | `manual-verification-evidence.md` section V2 |

Create `manual-verification-evidence.md` from the repository template when executing these
scenarios. Record the workflow URL, triggering ref, relevant step outcomes, and observed result.

### Disposable Verification Scripts

None planned.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| --- | --- | --- |
| AC1 | DONE | Workflow diff and both package-qualified runner help commands |
| AC2 | DONE | <https://github.com/josecelano/torrust-tracker/actions/runs/35443663968> |
| AC3 | DONE | `git diff --exit-code -- .github/workflows/container.yaml` |
| AC4 | DONE | Workflow diff and `linter yaml` |

## Risks and Trade-offs

- The local command can prove Cargo target resolution but not the full container workflow; M1
  supplies the required behavior-level evidence.
- A PR targeting `develop` skips this job by design; both branch-push and PR observations are needed.

## Implementation Completion Review

- Retrospective: Not required. The implementation matched the issue's established
  `container.yaml` pattern exactly, with no material discovery, design change, or deviation.
- Independent review results are recorded in `agent-review-reports.md`.

## References

- GitHub issue: [#2179](https://github.com/torrust/torrust-tracker/issues/2179)
- Related issue: [#1854](https://github.com/torrust/torrust-tracker/issues/1854)
- Related issue: [#1840](https://github.com/torrust/torrust-tracker/issues/1840)
