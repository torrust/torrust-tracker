---
doc-type: issue
issue-type: enhancement
status: open
priority: p2
github-issue: 2019
spec-path: docs/issues/open/2019-automatically-format-project-dictionary.md
branch: "2019-automatically-format-project-dictionary"
related-pr: null
last-updated-utc: 2026-07-22 00:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - contrib/dev-tools/git/format-project-words.sh
    - contrib/dev-tools/git/hooks/pre-commit.sh
    - docs/issues/open/2003-overhaul-guardrails-and-automation/EPIC.md
    - project-words.txt
---

<!-- skill-link: create-issue -->

# Issue #2019 - Automatically format the project dictionary

## Goal

Make `project-words.txt` consistently sorted and free of exact duplicate entries without requiring contributors or AI agents to edit its ordering manually.

## Background

`project-words.txt` is the custom cspell dictionary. Its intended alphabetical ordering is documented but not enforced by `linter all` or the pre-commit hook, so pull-request reviews repeatedly identify unsorted entries. This issue delivers a small, immediately useful interim formatter while EPIC #2003 evaluates the long-term automation and guardrail architecture. It must not constrain that future design: the EPIC may replace or refactor this implementation after its design decision.

## Scope

### In Scope

- Add `contrib/dev-tools/git/format-project-words.sh`, an independently runnable formatter that applies `LC_ALL=C sort --unique` to `project-words.txt`.
- Invoke the formatter from the pre-commit hook.
- Detect when formatting changes the dictionary and abort the commit with clear restaging instructions.
- Document the automatic behavior and manual formatting command in the relevant pre-commit workflow guidance.
- Ensure the committed dictionary is formatted by the new command.

### Out of Scope

- Changing the cspell configuration or its accepted dictionaries.
- Case-insensitive de-duplication or normalization of dictionary entries.
- Reordering unrelated project files.
- Selecting the long-term repository automation or guardrail architecture; that decision belongs to EPIC #2003.
- Treating this interim script as a constraint on EPIC #2003's future implementation.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                          | Notes / Expected Output                                                                                                                                                   |
| --- | ------ | ------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | DONE   | Add an independently runnable dictionary formatter            | `contrib/dev-tools/git/format-project-words.sh` applies `LC_ALL=C sort --unique` to `project-words.txt` and reports whether it changed the file.                          |
| T2  | DONE   | Invoke the formatter from the pre-commit hook                 | The hook calls the formatter before verification steps and retains its role as orchestration scaffolding.                                                                 |
| T3  | DONE   | Abort when the formatter changes the dictionary               | The commit stops and tells the contributor to stage `project-words.txt` and retry, preventing a stale index from being committed.                                         |
| T4  | DONE   | Update workflow documentation                                 | The documentation describes automatic formatting, the helper command, and the interim relationship to EPIC #2003; it no longer requires manual alphabetical-order review. |
| T5  | DONE   | Add or update automated tests for formatter and hook behavior | `contrib/dev-tools/git/tests/test-format-project-words.sh` covers formatter and hook behavior for changed and unchanged dictionaries.                                     |
| T6  | DONE   | Format and verify the dictionary                              | The checked-in file is formatted; focused tests and the required pre-commit validation gate pass.                                                                         |

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted in `docs/issues/drafts/`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [ ] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [x] Manual verification scenarios executed and recorded (status + evidence)
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

Append one line per meaningful update.

- 2026-07-22 00:00 UTC - GitHub Copilot - Created draft specification for review - `docs/issues/drafts/automatically-format-project-dictionary.md`
- 2026-07-22 00:00 UTC - josecelano - Approved an interim standalone formatter and hook integration while EPIC #2003 determines the long-term automation design - draft updated
- 2026-07-22 00:00 UTC - GitHub Operator - Created issue #2019 - https://github.com/torrust/torrust-tracker/issues/2019
- 2026-07-22 00:00 UTC - GitHub Copilot - Implemented the standalone formatter, pre-commit orchestration, focused shell tests, and synchronized workflow guidance; reviewed the linked `create-issue` skill with no process change required
- 2026-07-22 00:00 UTC - GitHub Copilot - Verified focused formatter and hook tests, the standalone formatter, and `TORRUST_GIT_HOOKS_LOG_DIR=.tmp ./contrib/dev-tools/git/hooks/pre-commit.sh --format=json`; all passed
- 2026-07-22 00:00 UTC - GitHub Copilot - Verified `TORRUST_GIT_HOOKS_LOG_DIR=.tmp ./contrib/dev-tools/git/hooks/pre-push.sh --format=json`; all nightly checks, documentation build, and stable workspace tests passed

## Acceptance Criteria

- [ ] AC1: `contrib/dev-tools/git/format-project-words.sh` applies `LC_ALL=C sort --unique` to `project-words.txt`, preserving distinct entries that differ only by case.
- [ ] AC2: If formatting modifies `project-words.txt`, the pre-commit hook exits non-zero and clearly instructs the contributor to stage the modified file and retry the commit.
- [ ] AC3: If formatting does not modify `project-words.txt`, the pre-commit hook continues with its existing verification steps.
- [ ] AC4: Automated coverage verifies both unchanged and changed formatter and hook behavior.
- [ ] AC5: The workflow documentation describes the automatic behavior and standalone formatter command.
- [ ] AC6: The implementation is documented as an interim measure related to EPIC #2003 and can be replaced or refactored by its future design.
- [ ] `linter all` exits with code `0`.
- [ ] Relevant tests pass.
- [ ] Manual verification scenarios are executed and documented (status + evidence).
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior.
- [ ] Documentation is updated when behavior/workflow changes.

## Verification Plan

Define verification before implementation starts and execute it before closing the issue.

### Automatic Checks

- `linter all`
- Focused tests for the pre-commit hook behavior
- `./contrib/dev-tools/git/format-project-words.sh`
- `TORRUST_GIT_HOOKS_LOG_DIR=.tmp ./contrib/dev-tools/git/hooks/pre-commit.sh`
- Pre-push checks when applicable

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                      | Command/Steps                                                                                                   | Expected Result                                                                                            | Status | Evidence                                                                                                                                         |
| --- | ----------------------------- | --------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------- | ------ | ------------------------------------------------------------------------------------------------------------------------------------------------ |
| M1  | Dictionary needs formatting   | Temporarily add unsorted and duplicate exact entries in an isolated Git checkout, then run the pre-commit hook. | The hook rewrites `project-words.txt`, exits non-zero, and instructs the user to stage the file and retry. | DONE   | `test-format-project-words.sh`: `it_should_abort_pre_commit_and_request_restaging_when_dictionary_is_formatted`.                                 |
| M2  | Dictionary already formatted  | Run the pre-commit hook with the formatted tracked dictionary.                                                  | The formatter leaves the file unchanged and the hook continues to its existing checks.                     | DONE   | `TORRUST_GIT_HOOKS_LOG_DIR=.tmp ./contrib/dev-tools/git/hooks/pre-commit.sh --format=json` passed its formatter and all four verification steps. |
| M3  | Case variants remain distinct | Run the standalone formatter against a disposable dictionary containing otherwise identical case variants.      | Both variants remain; only exact duplicate lines are removed.                                              | DONE   | `test-format-project-words.sh`: `it_should_sort_and_remove_exact_duplicates_when_dictionary_requires_formatting`.                                |

Notes:

- Manual verification is mandatory even when automated tests pass.
- If a scenario fails, record the failure and diagnosis in the progress log before proceeding.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                |
| ----- | ---------------------- | ----------------------- |
| AC1   | TODO                   | Pending implementation. |
| AC2   | TODO                   | Pending implementation. |
| AC3   | TODO                   | Pending implementation. |
| AC4   | TODO                   | Pending implementation. |
| AC5   | TODO                   | Pending implementation. |
| AC6   | TODO                   | Pending implementation. |

## Risks and Trade-offs

- A hook that changes a working-tree file after Git has prepared the index could otherwise allow the unsorted staged version to be committed. The hook must abort after a formatting change so the corrected file can be staged deliberately.
- Locale-sensitive sorting would yield inconsistent output across machines. Setting `LC_ALL=C` makes the ordering deterministic.
- Case-insensitive de-duplication could delete meaningful proper-name or acronym variants. Exact duplicate removal only avoids that data loss.

## References

- `project-words.txt`
- `cspell.json`
- `contrib/dev-tools/git/format-project-words.sh`
- `contrib/dev-tools/git/hooks/pre-commit.sh`
- `docs/issues/open/2003-overhaul-guardrails-and-automation/EPIC.md`
- `.github/skills/dev/git-workflow/run-pre-commit-checks/SKILL.md`
