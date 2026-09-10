---
doc-type: issue
issue-type: bug
status: draft
priority: p2
epic: 2003
github-issue: null
spec-path: docs/issues/drafts/2003-repair-project-dictionary-formatter-test-suite/ISSUE.md
branch: null
related-pr: null
last-updated-utc: 2026-09-10 09:30
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - contrib/dev-tools/checks/tests/test-format-project-words.sh
    - contrib/dev-tools/checks/format-project-words.sh
    - contrib/dev-tools/checks/lint-containerfile.sh
    - contrib/dev-tools/git/tests/test-merge-pull-request.sh
    - contrib/dev-tools/git/hooks/pre-commit.sh
    - docs/issues/open/2003-overhaul-guardrails-and-automation/EPIC.md
    - docs/issues/open/2190-maintenance-frictions-cleanup/EPIC.md
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Repair the project dictionary formatter test suite

**Parent EPIC:** #2003 - Overhaul: Automation Tools and AI Agent Guardrails

## Goal

Make `contrib/dev-tools/checks/tests/test-format-project-words.sh` exit 0 from a clean checkout on a host with no container runtime, so the suite can be trusted and, in a following subissue, invoked automatically.

## Background

The suite has been red on `develop` since commit `ffa2aa2c5` moved it beside its sensor. It fails for two independent reasons, verified at revision `f6b73e29` on 2026-09-09.

First, `PROJECT_ROOT` at line 13 climbs three directory levels from `contrib/dev-tools/checks/tests/` and lands on `<repo>/contrib`, so every fixture copy reads `<repo>/contrib/contrib/dev-tools/...` and fails. The sibling suite at `contrib/dev-tools/git/tests/test-merge-pull-request.sh:6` climbs four levels from an equally deep directory and is correct, which is the reference for the right depth.

Second, with the depth corrected the suite still fails. `create_fixture` at lines 21-28 never provisions `contrib/dev-tools/checks/lint-containerfile.sh`, so pre-commit step 5 of 6 reports `No such file or directory`, `commands.log` holds three entries where line 205 asserts four, and the success banner asserted at line 206 is never printed.

Nobody has noticed because no orchestrator runs the suite. That gap is the sibling subissue; this one makes the suite worth running.

## Scope

### In Scope

- Correct the `PROJECT_ROOT` depth so the fixture resolves against the repository root.
- Provision the Containerfile lint sensor in the fixture with a stub that keeps the suite hermetic and free of any container runtime.
- Re-check the `commands.log` count assertion and the success-banner assertion against whichever stub is chosen.
- Leave the suite runnable by a developer with a single `bash` invocation and no environment setup.

### Out of Scope

- Invoking this suite, or its two siblings, from CI or a git hook. That is the sibling subissue under this EPIC.
- Changing the behaviour of `contrib/dev-tools/checks/format-project-words.sh` itself.
- Redesigning the fixture, the pre-commit step list, or the check-harness architecture. Those belong to this EPIC's design decision.
- Adding a real Containerfile lint to the fixture, which would reintroduce a container-runtime dependency.

## Architectural Decisions

No architectural decision is expected. Both defects are mechanical: a wrong path depth and a fixture that does not provision one file the code under test invokes. The stub choice is a test-fixture detail, not an architecture decision; if it turns into a question about what the fixture is allowed to simulate, stop and raise it against this EPIC.

- Related ADRs: `None`
- ADRs to create: `None known`

## Design and Ownership Review

The suite creates and removes its own temporary directory under an `EXIT` trap that already works, so no new resource-ownership question arises. The one ownership point to preserve is that the stub sensor lives inside the fixture directory and is removed with it, never written into the working tree.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task | Notes / Expected Output |
| --- | ------ | ---- | ----------------------- |
| T1 | TODO | Correct the fixture root resolution | `PROJECT_ROOT` climbs four levels; a debug echo of the copied paths shows `<repo>/contrib/dev-tools/...` rather than a doubled `contrib`. |
| T2 | TODO | Provision the Containerfile lint sensor in the fixture | Pre-commit step 5 of 6 runs against a stub; no `No such file or directory` appears in the run output. |
| T3 | TODO | Reconcile the log-count and banner assertions | `commands.log` count and the success banner assertion match what the corrected fixture actually produces. |
| T4 | TODO | Final verification and acceptance review | `bash contrib/dev-tools/checks/tests/test-format-project-words.sh` exits 0 from a clean checkout; `linter all` exits 0; every acceptance criterion re-reviewed against observed behaviour. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T1 | The `PROJECT_ROOT` depth correction. | Commit on its own. It is the smallest independently reviewable step and it changes the failure mode visibly. |
| T2, T3 | The fixture stub plus the assertions it forces. | Commit together once the suite exits 0. The assertions are only correct in the presence of the stub, so splitting them would commit a red tree. |
| T4 | Completion evidence. | Keep separate from the fix so the verification record is reviewable on its own. |

Record a justified no-change decision in the task's evidence without creating an empty commit. Use a Conventional Commit message with the narrow affected scope.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2003-repair-project-dictionary-formatter-test-suite/ISSUE.md`
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

- 2026-09-10 09:30 UTC - Specification author - Drafted from the verified evidence in EPIC #2190; cluster C1, friction F5 - https://github.com/torrust/torrust-tracker/issues/2190

## Acceptance Criteria

- [ ] AC1: `bash contrib/dev-tools/checks/tests/test-format-project-words.sh` exits 0 from a clean checkout, on a host with no container runtime available.
- [ ] AC2: The suite prints its success banner, and the assertion that checks it is exercised rather than skipped.
- [ ] AC3: A deliberately broken assertion in the suite makes it exit non-zero, so the pass is not vacuous.
- [ ] AC4: The sibling suite `contrib/dev-tools/git/tests/test-merge-pull-request.sh` is unchanged and still passes.
- [ ] AC5: No file outside `contrib/dev-tools/checks/` changes.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

### Automatic Checks

- `linter all`
- `bash contrib/dev-tools/checks/tests/test-format-project-words.sh`
- `bash contrib/dev-tools/git/tests/test-merge-pull-request.sh`
- `bash contrib/dev-tools/git/hooks/pre-commit.sh`

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1 | Repaired suite passes from a clean checkout | Clone the branch into a fresh directory on a host with no container runtime, then run `bash contrib/dev-tools/checks/tests/test-format-project-words.sh` and echo `$?` | Exit code 0 and the line `All formatter and pre-commit hook tests passed.` | TODO | `manual-verification-evidence.md` section V1 |
| M2 | The suite is not vacuously green | Change one assertion in the suite to an expectation the fixture cannot satisfy, run it, then revert the change | The suite exits non-zero and names the failed assertion | TODO | `manual-verification-evidence.md` section V2 |
| M3 | No container runtime is reached | Run the suite on a host where no container runtime is installed and read the full output | No step attempts to invoke a container runtime, and no step is silently skipped | TODO | `manual-verification-evidence.md` section V3 |

Notes:

- Manual verification is mandatory even when automated tests pass. It is a real human-oriented use of the repository's tooling, not a simulated result and not merely running automated tests.
- Create `manual-verification-evidence.md` from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md` when executing these scenarios. Record actual prerequisites, actions, commands, program output, relevant logs, and outcomes there.
- If a scenario fails, record the failure and diagnosis in the progress log before proceeding.

### Disposable Verification Scripts

None is planned. Every scenario is a direct invocation of the suite under repair.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | TODO | M1 |
| AC2 | TODO | M1 |
| AC3 | TODO | M2 |
| AC4 | TODO | Automatic checks |
| AC5 | TODO | {PR link} |

## Risks and Trade-offs

- A stub can make the suite pass while hiding a real integration defect in the Containerfile lint sensor. Mitigation: the stub records its invocation in `commands.log`, so the suite still proves the step was reached, and M3 confirms nothing is skipped.
- Correcting the depth may expose further assertions that were never reached. Mitigation: T3 reconciles the assertions against observed output rather than against the values written before the suite was moved.
- Repairing a suite that nothing runs leaves it free to rot again. Mitigation: the sibling subissue wires it into CI, and it depends on this one.

## Implementation Completion Review

After implementation, compare the result with this specification. Record invalidated assumptions, material design changes, unexpected validation findings, and reusable lessons.

- Retrospective: `Not yet assessed`
- Create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this directory if either defect turns out to be larger than a fixture correction, or the stub choice raises a question about what the fixture may simulate.
- If none of that occurs, add a concise progress-log entry explaining why the work had no material discovery.
- When an independent reviewer receives this folder-style specification, it records its result in `agent-review-reports.md` using `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- Related issues: #2003 (parent EPIC), #2190 (source inventory), #2019 (the interim dictionary formatter this suite covers)
- Related PRs: #2193 (the EPIC specification that produced this draft)
- Related ADRs: `None`
