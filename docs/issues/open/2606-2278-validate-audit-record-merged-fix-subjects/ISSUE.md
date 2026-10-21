---
schema-version: 1
doc-type: issue
issue-type: bug
status: planned
priority: p3
epic: 2278
github-issue: 2606
spec-path: docs/issues/open/2606-2278-validate-audit-record-merged-fix-subjects/ISSUE.md
branch: "2606-validate-audit-record-merged-fix-subjects-spec"
related-pr: null
last-updated-utc: "2026-10-21 11:03"
semantic-links:
  skill-links:
    - create-issue
    - fix-bug
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - .github/skills/dev/pr-reviews/process-pr-review/scripts/validate-audit-record.py
    - .github/skills/dev/debugging/fix-bug/SKILL.md
    - .github/skills/dev/planning/create-issue/SKILL.md
    - docs/pr-reviews/pr-2584-review/PR-REVIEW.md
    - docs/pr-reviews/pr-2599-review/PR-REVIEW.md
---

<!-- skill-link: create-issue -->

# Issue #2606 - `validate-audit-record.py` Rejects Fixes Merged Outside Its Commit Range

**Parent EPIC:** #2278 - Strengthen PR review author self-audit and evidence generation

## Goal

Make `validate-audit-record.py` check a `FIXED` resolution reference against the commits that
belong to the audited pull request and its recorded follow-up pull requests, so the result no
longer depends on the local `--base` ref or on whether the fix is already merged.

## Background

The script checks that every `Resolution reference` commit subject exists in `<base>..HEAD`
(`--base` defaults to `develop`). That range is correct while the fix commit is still on the
author's branch, and wrong in two other cases:

1. **Merged fix, current base.** Once the fix is merged, a current `develop` (or
   `torrust/develop`) contains it, so `<base>..HEAD` is empty and the script reports
   `resolution reference not a commit subject on branch`. Reproduced on 2026-10-21 at
   `torrust/develop` = `6da62f0ee`:

   ```sh
   python3 .github/skills/dev/pr-reviews/process-pr-review/scripts/validate-audit-record.py \
     --pr-number 2584 --base torrust/develop
   # F1, F2, WAIT-001: resolution reference not a commit subject on branch ... failures: 3
   python3 ... --pr-number 2599 --base torrust/develop
   # AUDIT-001: resolution reference not a commit subject on branch ... failures: 1
   ```

2. **Stale local base, silent pass.** The default `develop` is the local branch. When it is behind
   `torrust/develop`, the range still contains the merged fix, so the same audits pass with
   `--base develop` (the default) and fail with `--base torrust/develop`. The result depends on how
   recently the contributor updated a local branch, not on the audit.

A fix can also live in a pull request other than the audited one. The WAIT-001 row of the
PR #2584 audit cites a commit that merged in PR #2596 and records PR #2596 in its
`Follow-up PR URL` field. Neither `<base>..HEAD` nor the audited PR's own commit list contains it.

Workaround used so far: pass a base older than every fix (`--base fcd78975c^1`). That is not a
maintainable instruction for agents.

Sizing: the script is 227 lines and the faulty check is one function (`branch_subjects`) plus its
call. The fix is a small change, so this issue does not migrate the script to Rust. The script
stays a Python prototype whose behaviour is the reference for that later migration (see
`process-pr-review` skill, "Validation Script").

## Scope

### In Scope

- Build the set of valid commit subjects from three sources: `<base>..HEAD` (unpushed and
  in-review commits), the commits of the audited pull request, and the commits of every pull
  request named in a `Follow-up PR URL` field of the audit.
- Fetch pull-request commits with `gh api repos/<owner>/<repo>/pulls/<n>/commits --paginate`.
  This works after merge and independently of local refs.
- Add a deterministic regression test that fails before the fix and passes after it.
- Update the script's help text and the "Validation Script" section of the `process-pr-review`
  skill to describe the new rule.

### Out of Scope

- Migrating the script to Rust.
- Verifying prose claims such as `Current-tree verification` sentences, or log stamps against
  commit times (the skill already says these are manual checks).
- Changing the audit template, its field names, or the resolution-reference rules.
- Adding the test to CI or a pre-push hook. The test is run explicitly (see Verification Plan).

## Architectural Decisions

- Related ADRs: `None known`.
- ADRs to create: `None known`.

## Design and Ownership Review

The script runs short synchronous child processes (`git`, `gh`) and holds no resources.

- **Narrow interface:** one function returns the set of valid subjects given the base ref, the
  audited PR number, and the follow-up PR numbers. `check_row` keeps receiving a plain set.
- **Failure path:** a `gh` failure while listing commits is a usage error (exit 2) with the
  failing endpoint on stderr. It must not silently fall back to `<base>..HEAD`, because that
  would hide the defect this issue fixes.
- **Accepted trade-off:** a subject from the `<base>..HEAD` range is accepted even when it
  belongs to an unrelated commit, which a stale base can enlarge. Pull-request commit lists are
  the authoritative source for pushed fixes; the local range only supplements them for
  unpushed work.

## Bug-Fix Process

Follows [`fix-bug`](../../../../.github/skills/dev/debugging/fix-bug/SKILL.md):

- analysis: the defect and hypothesis are in Background;
- real-artifact reproduction: the commands above against the merged audits of PR #2584 and
  PR #2599 (scenario M1), recorded in `manual-verification-evidence.md`;
- regression-test boundary: a unit test at the subject-set function (see below);
- red test evidence, production fix, green evidence, and a like-for-like recheck of M1.

## Regression Test Strategy

Smallest deterministic boundary: a Python `unittest` module next to the script that builds the
subject set with `git` and `gh` faked at the process boundary. Following the repository
precedent of `contrib/dev-tools/git/tests/test-github-merge-symlinks.py`, it uses only the
standard library.

Cases (each fails on the current code):

- a subject present only in the audited PR's commit list is accepted with an up-to-date base;
- a subject present only in a follow-up PR's commit list is accepted;
- a subject present in none of the sources is rejected;
- a subject present only in `<base>..HEAD` is still accepted (unpushed commit);
- a `gh` failure while listing commits yields a usage error, not a silent fallback.

The end-to-end recheck is scenario M1 on the real audits.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                                     | Notes / Expected Output                                           |
| --- | ------ | ------------------------------------------------------------------------ | ----------------------------------------------------------------- |
| T1  | TODO   | Reproduce the failure on the merged audits (M1, red)                     | Commands and output in `manual-verification-evidence.md`          |
| T2  | TODO   | Add the failing unit tests with `git` and `gh` faked                     | Tests fail for the current code                                   |
| T3  | TODO   | Build the subject set from the three sources                             | Tests pass; `check_row` and the output contract are unchanged     |
| T4  | TODO   | Update the script help text and the `process-pr-review` skill            | The skill describes the three sources and the `--base` role       |
| T5  | TODO   | Recheck M1 (green) and validate every existing audit that has a `FIXED` row | Existing audits still pass; any difference is explained in the log |

## Commit Points

| Task  | Coherent change set                                    | Commit policy                                        |
| ----- | ------------------------------------------------------ | ---------------------------------------------------- |
| T2    | New failing test module                                | Commit after the red run is recorded.                |
| T3    | Subject-set change in the script                       | Commit after focused validation and required review. |
| T4    | Script help text and skill description                 | Commit after `linter all` passes.                    |
| T1/T5 | Evidence file entries (kept apart from product change) | Commit after the evidence is recorded.               |

Use Conventional Commits with the `pr-reviews` scope and sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/validate-audit-record-merged-fix-subjects/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [ ] Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, the new unit tests)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Implementation completion review recorded in the progress log
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-10-21 11:00 UTC - GitHub Copilot (Claude Sonnet 5.5) - Draft created after the PR #2599
  review round. Reproduced the failure with `--base torrust/develop` and found that the default
  `--base develop` passes only because the local `develop` branch is stale. This corrects an
  earlier statement in PR #2599's description, which said the default base fails.
- 2026-10-21 11:03 UTC - GitHub Copilot (Claude Sonnet 5.5) - The maintainer approved the draft. Created
  issue #2606, linked it as a sub-issue of EPIC #2278 (`parent_issue_url` verified), and moved this
  specification to `docs/issues/open/2606-2278-validate-audit-record-merged-fix-subjects/`.

## Acceptance Criteria

- [ ] AC1: With an up-to-date base (`--base torrust/develop`), the validator passes on the PR #2584
  and PR #2599 audits.
- [ ] AC2: The result does not change with a stale or current local `develop` for pushed fixes.
- [ ] AC3: A subject found in none of the three sources is still rejected.
- [ ] AC4: A `gh` failure while listing pull-request commits exits with code 2 and names the
  endpoint.
- [ ] AC5: The script's help text and the `process-pr-review` skill describe the three sources.
- [ ] `linter all` exits with code `0`
- [ ] The new unit tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior

## Verification Plan

### Automatic Checks

- `linter all`
- `python3 -m unittest discover -s .github/skills/dev/pr-reviews/process-pr-review/scripts`
  (Python 3 standard library)

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                                         | Human-oriented command/steps                                                                                                                                | Expected Result                                          | Status | Evidence                                     |
| --- | ------------------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------- | ------ | -------------------------------------------- |
| M1  | Merged audits pass with a current base           | `git fetch torrust develop`, then `validate-audit-record.py --pr-number 2584 --base torrust/develop` and the same for `2599`                                | Red before the fix (4 failures), green after, exit 0     | TODO   | `manual-verification-evidence.md` section V1 |
| M2  | Result is independent of the local `develop`     | Run M1 with `--base develop` while local `develop` is behind and again after `git pull --ff-only torrust develop`                                          | Same result in both states                               | TODO   | `manual-verification-evidence.md` section V2 |
| M3  | A bogus subject is still rejected                | Copy the PR #2599 audit, change the `Resolution reference` to a subject that exists in no commit, run the validator with `--audit-file`                    | Exit 1 with one `not a commit subject` failure           | TODO   | `manual-verification-evidence.md` section V3 |

Notes:

- Every recorded command result identifies the runtime that produced it (Python 3 version and
  the `gh` version).
- Create `manual-verification-evidence.md` from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`
  when executing these scenarios.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                                           |
| ----- | ---------------------- | -------------------------------------------------- |
| AC1   | TODO                   | `manual-verification-evidence.md` V1               |
| AC2   | TODO                   | `manual-verification-evidence.md` V2               |
| AC3   | TODO                   | Unit test and V3                                   |
| AC4   | TODO                   | Unit test                                          |
| AC5   | TODO                   | Diff of the script help text and the skill section |

## Risks and Trade-offs

- A `gh` rate limit or outage now blocks validation, because the commit lists come from GitHub.
  The validator already needs `gh` for review comments, so no new dependency appears.
- Accepting `<base>..HEAD` keeps a small false-pass window (see Design and Ownership Review).
  Closing it needs the Rust migration to resolve the PR's recorded head, which is out of scope.
- If the script is migrated to Rust before this lands, port the three-source rule instead of the
  current single range.
