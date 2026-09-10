---
doc-type: issue
issue-type: enhancement
status: draft
priority: p2
epic: 2003
github-issue: null
spec-path: docs/issues/drafts/2003-run-developer-tool-test-suites-in-ci/ISSUE.md
branch: null
related-pr: null
last-updated-utc: 2026-09-10 09:10
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - .github/workflows/testing.yaml
    - contrib/dev-tools/git/tests/test-merge-pull-request.sh
    - contrib/dev-tools/checks/tests/test-format-project-words.sh
    - contrib/dev-tools/checks/tests/test-agent-review-report-contract.sh
    - contrib/dev-tools/checks/format-project-words.sh
    - contrib/dev-tools/checks/lint-containerfile.sh
    - contrib/dev-tools/git/hooks/pre-commit.sh
    - docs/issues/open/2003-overhaul-guardrails-and-automation/EPIC.md
    - docs/issues/open/2190-maintenance-frictions-cleanup/EPIC.md
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Run the developer-tool test suites in CI

**Parent EPIC:** #2003 - Overhaul: Automation Tools and AI Agent Guardrails

## Goal

Have one lightweight CI step run the three existing developer-tool test suites and fail the workflow when any of them fails, and remove the notes in the tree that will then be untrue.

## Background

Three developer-tool test suites exist and no orchestrator runs any of them, verified at revision `f6b73e29` on 2026-09-09: `contrib/dev-tools/git/tests/test-merge-pull-request.sh`, `contrib/dev-tools/checks/tests/test-format-project-words.sh`, and `contrib/dev-tools/checks/tests/test-agent-review-report-contract.sh`. Searching the workflows for `contrib/dev-tools` returns only the container persistence test and the hook installer, and none of the suites appears in a `STEPS` entry of `contrib/dev-tools/git/hooks/pre-commit.sh` (lines 51-58) or `pre-push.sh`.

Two scripts state the gap themselves, at `contrib/dev-tools/checks/tests/test-format-project-words.sh:6` and `contrib/dev-tools/checks/format-project-words.sh:6`. A third note, at `contrib/dev-tools/checks/lint-containerfile.sh:4`, records that the sensor has no automated tests yet and names this EPIC. That note stops being true in the same change that makes it so, because the sibling subissue provisions the sensor in the formatter suite's fixture and this subissue runs that fixture in CI.

The three suites together complete in roughly one second on a warm host, so the cost of running them is negligible next to the cost of a suite silently rotting.

## Scope

### In Scope

- Add one lightweight CI step that runs the three suites and fails the workflow when any of them fails.
- Keep that step outside the expensive test matrix, so a failure is easy to read and does not consume matrix capacity.
- Delete the two notes that claim the tests are not automatically run, once they are.
- Correct or delete the dead note at `contrib/dev-tools/checks/lint-containerfile.sh:4`, in the same change, so the tree is never internally inconsistent.

### Out of Scope

- Repairing the formatter suite. That is the sibling subissue under this EPIC, and it must merge first.
- Selecting the long-term check-harness shape, runner, cache, or execution tier. Those belong to this EPIC's architecture decision, and this step remains replaceable by it.
- Adding the suites to a git hook, which would change local commit latency without evidence.
- Writing new suites for sensors that have none.

## Architectural Decisions

No architectural decision is expected. The step adds one invocation of checks that already exist and selects no runner, cache, or enforcement platform, which is why this work qualifies under the parent EPIC's exception for additive, independently verifiable subissues that may proceed before its architecture decision.

- Related ADRs: `None`
- ADRs to create: `None known`

## Design and Ownership Review

If the CI placement turns out to be a real choice about where developer-tool tests belong rather than a one-step addition, stop and raise it against the parent EPIC rather than settling it here. That stop condition is the reason this subissue sits under EPIC #2003 rather than in a clean-up issue.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task | Notes / Expected Output |
| --- | ------ | ---- | ----------------------- |
| T1 | BLOCKED | Wait for the formatter suite repair to merge | The sibling subissue is merged and the formatter suite exits 0 on `develop`. This step must not wire a red suite into CI. |
| T2 | TODO | Add the CI step that runs the three suites | One step invokes all three suites; a locally broken suite makes the step exit non-zero. |
| T3 | TODO | Remove the two now-untrue notes | Neither `test-format-project-words.sh:6` nor `format-project-words.sh:6` still claims the tests are not automatically run. |
| T4 | TODO | Correct the dead note on the Containerfile lint sensor | `lint-containerfile.sh:4` names the fixture that covers the sensor, or the note is gone. |
| T5 | TODO | Final verification and acceptance review | `linter all` exits 0, the CI step is observed failing on a deliberately broken suite, and every acceptance criterion is re-reviewed against observed behaviour. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T2 | The CI step. | Commit on its own after the workflow file is validated. Depends on the sibling subissue being merged. |
| T3, T4 | The three notes. | Commit together with, or immediately after, the step, so no revision of the tree both runs the suites and denies that it does. |
| T5 | Completion evidence. | Keep separate from the change so the verification record is reviewable on its own. |

Record a justified no-change decision in the task's evidence without creating an empty commit. Use a Conventional Commit message with the narrow affected scope.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2003-run-developer-tool-test-suites-in-ci/ISSUE.md`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] GitHub issue created, linked as a subissue of the parent EPIC, and issue number added to this spec
- [ ] Specification moved from `docs/issues/drafts/` to `docs/issues/open/` under its assigned number
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-09-10 08:43 UTC - Specification author - Drafted from the verified evidence in EPIC #2190; inventory items F1 and F7 - https://github.com/torrust/torrust-tracker/issues/2190

## Acceptance Criteria

- [ ] AC1: All three developer-tool suites run in CI on every pull request.
- [ ] AC2: A deliberately broken suite fails the workflow and the failure names the broken suite.
- [ ] AC3: No script in `contrib/dev-tools/` still claims its tests are not automatically run when they are.
- [ ] AC4: The note at `contrib/dev-tools/checks/lint-containerfile.sh:4` is true of the tree it ships in.
- [ ] AC5: The step runs outside the test matrix and adds no measurable time to the matrix jobs.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

### Automatic Checks

- `linter all`
- The three suites, individually and through the new CI step
- `bash contrib/dev-tools/git/hooks/pre-commit.sh`

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1 | The new CI step actually catches a failure | On a scratch branch, break one assertion in one suite, push, and read the workflow run | The workflow fails and names the broken suite. Revert the scratch branch afterwards | TODO | `manual-verification-evidence.md` section V1 |
| M2 | The step runs all three suites | Read the run log of the new step on a green build | All three suite names appear in the step output with a passing result each | TODO | `manual-verification-evidence.md` section V2 |
| M3 | No note contradicts the tree | Read the three note lines on the merged revision | Each note describes what the tree actually does | TODO | `manual-verification-evidence.md` section V3 |

Notes:

- Manual verification is mandatory even when automated tests pass. It is a real human-oriented use of the repository's tooling, not a simulated result and not merely running automated tests.
- Create `manual-verification-evidence.md` from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md` when executing these scenarios. Record actual prerequisites, actions, commands, program output, relevant logs, and outcomes there.
- If a scenario fails, record the failure and diagnosis in the progress log before proceeding.

### Disposable Verification Scripts

None is planned. Every scenario reads a real workflow run or a file in the tree.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | TODO | M2 |
| AC2 | TODO | M1 |
| AC3 | TODO | M3 |
| AC4 | TODO | M3 |
| AC5 | TODO | M2 |

## Risks and Trade-offs

- Wiring previously invisible failures into a blocking check makes a future breakage stop a merge. That is the point, but it is a real cost. Mitigation: the suites are fast and hermetic, and the step is separate and lightweight, so a failure is cheap to read and cheap to fix.
- The step could be superseded by this EPIC's later harness design. Mitigation: it is one workflow step invoking existing scripts, with no shared runner or cache to unpick.
- Merging before the formatter suite repair would make CI red on `develop`. Mitigation: T1 is a blocking dependency, stated as such in the implementation plan.

## Implementation Completion Review

After implementation, compare the result with this specification. Record invalidated assumptions, material design changes, unexpected validation findings, and reusable lessons.

- Retrospective: `Not yet assessed`
- Create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this directory if the CI placement becomes a design decision rather than a one-step addition, or a suite proves unstable once it actually runs.
- If none of that occurs, add a concise progress-log entry explaining why the work had no material discovery.
- When an independent reviewer receives this folder-style specification, it records its result in `agent-review-reports.md` using `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- Related issues: #2003 (parent EPIC), #2190 (source inventory)
- Related PRs: #2193 (the EPIC specification that produced this draft)
- Related ADRs: `None`
