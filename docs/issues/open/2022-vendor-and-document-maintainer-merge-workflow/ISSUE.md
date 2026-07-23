---
doc-type: issue
issue-type: task
status: open
priority: p2
github-issue: 2022
spec-path: docs/issues/open/2022-vendor-and-document-maintainer-merge-workflow/ISSUE.md
branch: "2022-vendor-and-document-maintainer-merge-workflow"
related-pr: null
last-updated-utc: 2026-07-22 15:30
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - .github/skills/dev/git-workflow/
    - .github/skills/dev/git-workflow/merge-pull-request/SKILL.md
    - contrib/dev-tools/git/
    - cspell.json
    - docs/issues/open/2022-vendor-and-document-maintainer-merge-workflow/github-merge.py
    - docs/issues/open/2003-overhaul-guardrails-and-automation/EPIC.md
    - project-words.txt
---

<!-- skill-link: create-issue -->

# Issue #2022 - Vendor and document the maintainer merge workflow

## Goal

Bring the currently external, maintainer-operated pull-request merge workflow into this repository and document it as an agent-aware, reproducible process.

## Background

Maintainers currently invoke `/home/josecelano/Bin/github-merge.py` through `gh-merge {PR-NUMBER}` to construct, inspect, sign, and optionally push local merge commits. The script is not versioned with this repository and its required configuration, temporary branches, hook behavior, validation flow, and recovery process are undocumented here.

The exact current script is preserved with this folder-style specification as [`github-merge.py`](github-merge.py). It has SHA-256 `e390eb014131f3183a2cba642134974a6b09b19a65322d17dd7c81cf4ffbaad2` and is a planning snapshot only; implementation must audit and vendor it under `contrib/dev-tools/git/` with its copyright and MIT license notice intact. Its source-derived identifiers are excluded by one precise `cspell.json` ignore pattern so that the snapshot remains byte-for-byte reviewable without expanding the project dictionary.

During the merge of PR #2020, the merge tool ran `git merge --commit`, which invoked the repository pre-commit hook. The hook's dictionary formatter rewrote `project-words.txt` and aborted the temporary merge commit. The incident showed that an external, undocumented merge tool leaves both maintainers and agents without a repository-local procedure for understanding side effects, recovering safely, and preparing a mergeable tree.

This task is related to EPIC #2003. It provides a concrete, immediately useful merge-workflow integration without selecting the EPIC's eventual automation architecture. The EPIC may replace or refactor the result after its design decision, including a potential migration to Rust or replacement by another approved automation architecture. This issue must preserve that migration path without committing to it.

## Scope

### In Scope

- Vendor the current merge script under `contrib/dev-tools/git/` with its existing license and provenance preserved.
- Provide a repository-local entry point or documented invocation equivalent to the current `gh-merge {PR-NUMBER}` workflow.
- Add the dedicated AI-agent merge skill at `.github/skills/dev/git-workflow/merge-pull-request/SKILL.md`.
- Require the AI-agent merge skill to direct agents to verify the target branch and clean working tree; run the repository-local tool; inspect the temporary merge; run validation; recognize hook side effects; recover safely; and never sign or push without explicit maintainer confirmation.
- Document required Git configuration, credentials, signing prerequisites, temporary branches, merge inspection, testing, signing, and push confirmation.
- Document how Git hooks run during the tool's temporary `git merge --commit` operation, including the requirement that mutating hook actions leave the merge tree unchanged.
- Define a safe recovery procedure for a failed merge attempt, including how to return to the target branch and remove temporary state.
- Add maintainable automated coverage or a deterministic dry-run strategy for the repository-owned wrapper and any repository-specific behavior.
- Record the relationship to EPIC #2003 without treating this implementation as its final automation design; preserve a potential future migration to Rust or replacement by another approved automation architecture without committing to either.

### Out of Scope

- Changing GitHub's server-side merge behavior or repository branch-protection policy.
- Replacing the repository's existing pre-commit or pre-push framework.
- Designing the final common action/check/policy runner proposed by EPIC #2003.
- Automating maintainer judgment, PR review, or the final decision to sign and push a merge.
- Rewriting the vendored merge algorithm beyond necessary repository integration, security, portability, or correctness changes.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status      | Task                              | Notes / Expected Output                                                                                                                                                                                                                                  |
| --- | ----------- | --------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | DONE        | Audit the external merge workflow | Verified the planning snapshot and external source SHA-256 as `e390eb014131f3183a2cba642134974a6b09b19a65322d17dd7c81cf4ffbaad2`; audited its Python standard-library dependencies, configuration keys, entry point, copyright, and MIT license.         |
| T2  | DONE        | Vendor the merge tool             | Added byte-identical `contrib/dev-tools/git/github-merge.py` and `github-merge-COPYING`; the vendor source preserves its upstream header and SHA-256.                                                                                                    |
| T3  | DONE        | Add repository integration        | Added `contrib/dev-tools/git/merge-pull-request.sh`, which validates a clean tree, fixed upstream repository, `develop`, and signing-key setup; `--dry-run` is non-destructive.                                                                          |
| T4  | DONE        | Write the AI-agent merge skill    | Added `.github/skills/dev/git-workflow/merge-pull-request/SKILL.md` with the required preflight, temporary-branch, hook, validation, signing, push-confirmation, abort, and recovery guidance.                                                           |
| T5  | DONE        | Add verification coverage         | Added deterministic wrapper coverage for argument/configuration validation and delegation; documented interactive, network, GPG, merge, and push test boundaries.                                                                                        |
| T6  | DONE        | Document automation relationship  | Documented the interim relationship to EPIC #2003 and preserved a future Rust migration or approved replacement path without selecting either.                                                                                                           |
| T7  | IN_PROGRESS | Validate and review               | Focused tests, vendor SHA-256 and license comparisons, pre-commit, and pre-push checks passed. Manual M1-M3 evidence is recorded; M4 remains blocked pending an authorized disposable merge. Complexity audit and independent review are still required. |

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted in `docs/issues/drafts/`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [x] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded (status + evidence)
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

Append one line per meaningful update.

- 2026-07-22 00:00 UTC - GitHub Copilot - Created folder-style draft specification after the PR #2020 merge-hook failure exposed the undocumented external merge workflow - `docs/issues/drafts/vendor-and-document-maintainer-merge-workflow/`
- 2026-07-22 13:00 UTC - GitHub Copilot - User approved the specification; created GitHub issue #2022 with the `task`, `Documentation`, and `Automation` labels - `https://github.com/torrust/torrust-tracker/issues/2022`
- 2026-07-22 15:30 UTC - GitHub Copilot - Corrected reviewed specification wording and added the MIT license text referenced by the immutable planning snapshot - PR #2024
- 2026-07-23 00:00 UTC - GitHub Copilot - Verified the planning snapshot and external source against the recorded SHA-256, then vendored the byte-identical MIT-licensed tool with a repository-local wrapper, deterministic dry-run coverage, and maintainer merge skill - implementation branch `2022-vendor-and-document-maintainer-merge-workflow`

## Acceptance Criteria

- [ ] AC1: The merge tool is versioned under `contrib/dev-tools/git/` with its provenance, copyright, and license preserved.
- [ ] AC2: A maintainer can discover and invoke the repository-local merge workflow without depending on an undocumented path outside the repository.
- [ ] AC3: `.github/skills/dev/git-workflow/merge-pull-request/SKILL.md` provides AI agents with explicit instructions for preflight, temporary branches, merge inspection, validation, hook side effects, recovery, signing, and explicit push confirmation.
- [ ] AC4: The merge workflow skill describes configuration, credentials, signing, temporary branches, inspection, validation, signing, push confirmation, abort, and recovery steps.
- [ ] AC5: Documentation explicitly states that the tool creates a temporary merge commit with `git merge --commit`, which invokes installed pre-commit hooks.
- [ ] AC6: Documentation explains how a mutating hook action can block a merge and gives a safe recovery path that does not discard unrelated work.
- [ ] AC7: Automated coverage or a documented deterministic dry-run strategy validates repository-specific, non-destructive behavior; unsupported interactive or networked paths have an explicit test-boundary rationale.
- [ ] AC8: The implementation's interim relationship to EPIC #2003 is documented, preserves a potential future migration to Rust or replacement by another approved automation architecture, and does not claim to choose either.
- [ ] `linter all` exits with code `0`.
- [ ] Relevant tests pass.
- [ ] Manual verification scenarios are executed and documented (status + evidence).
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior.
- [ ] Documentation is updated when behavior/workflow changes.

## Verification Plan

Define verification before implementation starts and execute it before closing the issue.

### Automatic Checks

- `linter all`
- Focused tests for the repository-local merge tool, wrapper, or dry-run behavior
- `TORRUST_GIT_HOOKS_LOG_DIR=.tmp ./contrib/dev-tools/git/hooks/pre-commit.sh`
- Pre-push checks when applicable

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                     | Command/Steps                                                                                                                                                                    | Expected Result                                                                                                                         | Status  | Evidence                                                                                                                                                                                                                                                                             |
| --- | ---------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------- | ------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| M1  | Prerequisite discovery       | Follow only repository-local documentation from a clean checkout to identify required Git configuration, credentials, and signing setup.                                         | A maintainer or agent can identify every prerequisite without relying on an external personal script path.                              | DONE    | Reviewed `README-github-merge.md` and the `merge-pull-request` skill; both enumerate local command, Git configuration, credentials, hooks, and GPG prerequisites.                                                                                                                    |
| M2  | Supported dry-run validation | Run the explicitly supported dry-run fixture for the repository-local merge tool.                                                                                                | The fixture verifies that `--dry-run` succeeds without invoking the vendor tool or modifying repository state.                          | DONE    | `bash contrib/dev-tools/git/tests/test-merge-pull-request.sh` exercised the supported `--dry-run` fixture and verified no vendor invocation; a live GitHub inspection was intentionally not run against a production PR.                                                             |
| M3  | Hook-side-effect recovery    | Use an isolated Git checkout with a deliberately unsorted dictionary, run the merge inspection path until the pre-commit hook aborts, then follow the documented recovery steps. | The recovery returns to the target branch, preserves unrelated work, and explains how to make the merge tree canonical before retrying. | DONE    | `bash contrib/dev-tools/git/tests/test-format-project-words.sh` exercised an isolated fixture where the hook formats and aborts; the merge skill documents automatic abort, temporary-branch cleanup, preservation of pre-existing work, and a separate canonical dictionary commit. |
| M4  | Signed merge completion      | In an authorized disposable or maintainer-reviewed context, inspect the merge, run required validation, sign, and confirm the push.                                              | The final merge commit is signed, has the documented tree verification, and is pushed only after explicit confirmation.                 | BLOCKED | Not run: it requires an authorized disposable or maintainer-reviewed PR plus explicit authorization to sign and push; this implementation task must not create an unreviewed production merge.                                                                                       |

Notes:

- Manual verification is mandatory even when automated tests pass.
- Do not use a production branch or unreviewed PR for destructive verification.
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
| AC7   | TODO                   | Pending implementation. |
| AC8   | TODO                   | Pending implementation. |

## Risks and Trade-offs

- Vendoring a script preserves a known workflow but creates an ownership obligation. Preserve provenance and license, minimize local divergence, and document the update policy.
- The merge workflow is interactive and can push protected branches. The skill must preserve explicit human confirmation rather than making signing or pushing automatic.
- Git hooks can mutate the temporary merge tree. The workflow must make this visible, require a clean canonical tree before retrying, and document recovery that protects unrelated work.
- The tool's network, credential, GPG, and interactive-shell paths are difficult to unit test completely. Cover deterministic local behavior and document manual verification boundaries explicitly.
- EPIC #2003 may choose a different long-term architecture, including a migration to Rust or another approved replacement. Keep this task narrowly focused on making the existing workflow reproducible and agent-aware without blocking that migration path.

## References

- Related issues: #2003, #2022
- Related PRs: #2020
- External source before vendoring: `/home/josecelano/Bin/github-merge.py`
- Current source snapshot: `docs/issues/open/2022-vendor-and-document-maintainer-merge-workflow/github-merge.py` (SHA-256 `e390eb014131f3183a2cba642134974a6b09b19a65322d17dd7c81cf4ffbaad2`)
- `cspell.json`
- `contrib/dev-tools/git/hooks/pre-commit.sh`
- `contrib/dev-tools/git/format-project-words.sh`
- `docs/issues/open/2003-overhaul-guardrails-and-automation/EPIC.md`
