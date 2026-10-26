---
schema-version: 1
doc-type: issue
issue-type: bug
status: done
priority: p3
epic: 2278
github-issue: 2606
spec-path: docs/issues/closed/2606-2278-validate-audit-record-merged-fix-subjects/ISSUE.md
branch: "2606-validate-audit-record-merged-fix-subjects"
related-pr: 2641
last-updated-utc: "2026-10-26 08:30"
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
- **Follow-up PR URL parsing:** extract every `https://github.com/<owner>/<repo>/pull/<n>` URL from
  the field of each row. A URL names its own repository, so commits are fetched from the owner and
  repository in the URL, not only from `--owner` and `--repo`. `N/A`, `NA`, and an empty field
  contribute nothing. Each distinct pull request is fetched once.
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
- real-artifact reproduction: **Reproduced** on 2026-10-21 against the merged audits of PR #2584 and
  PR #2599 (scenarios M1 and M2). Commands, runtime versions, ref states, and output are in
  `manual-verification-evidence.md` sections V1 and V2. The final green recheck is pending;
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

The cases are built in the three increments of the Implementation Plan, each with its recorded red
run and test-design review. The end-to-end recheck is scenarios M1 and M2 on the real audits.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

Tests follow the [`write-unit-test`](../../../../.github/skills/dev/testing/write-unit-test/SKILL.md) skill
and are built in three behavior-sized increments. Each increment adds the smallest test for one
behavior, runs it red against the current code, and keeps that red output. A separate step then makes
the production change that turns it green. After each increment passes focused validation, record a
design review in the issue-local `test-design-review.md` before the next increment. The review
confirms that the test exposes the one causal initial-state difference, that its fixture owns only
incidental mechanics (the faked `git` and `gh` calls), and that the production Act and the
independently specified expected result stay visible. It uses the prose-first Arrange-Act-Assert
comparison: write temporary prose for each section, refactor until the code expresses it, remove
redundant prose, and keep only irreducible context.

| ID  | Status | Task                                                                                                                | Notes / Expected Output                                                  |
| --- | ------ | ------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------ |
| T1  | DONE   | Reproduce the failure on the merged audits (M1, M2, red)                                                            | Reproduced; `manual-verification-evidence.md` sections V1 and V2         |
| T2  | DONE   | Increment 1 test: a subject only in the audited PR's commit list is accepted with an up-to-date base                | Red output kept; design review recorded                                  |
| T3  | DONE   | Production change: add the audited PR's commits (`gh api .../pulls/<n>/commits`) to the valid subjects              | Increment 1 green; `check_row` and the output contract unchanged         |
| T4  | DONE   | Increment 2 test: a subject only in a follow-up PR's commit list is accepted                                        | Red output kept; design review recorded                                  |
| T5  | DONE   | Production change: add the commits of every PR in a `Follow-up PR URL` field                                       | Increment 2 green                                                        |
| T6  | DONE   | Increment 3 tests: an unknown subject is rejected, a `<base>..HEAD`-only subject is accepted, a `gh` failure exits 2 | The `gh` failure test is red; the other two guard existing behavior      |
| T7  | DONE   | Production change: report a `gh` failure while listing commits as a usage error                                     | Increment 3 green                                                        |
| T8  | DONE   | Stop for maintainer review of the test increments                                                                   | Review happens before final verification, the final commit, and the PR  |
| T9  | DONE   | Update the script help text and the `process-pr-review` skill                                                       | The skill describes the three sources and the role of `--base`           |
| T10 | DONE   | Recheck M1 and M2 (green) and validate every existing audit that has a `FIXED` row                                  | Existing audits still pass; any difference is explained in the log       |

## Commit Points

| Task      | Coherent change set                                                         | Commit policy                                                                                      |
| --------- | --------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------- |
| T2        | Increment 1 test module, with its design review                             | Commit after the red output is kept and the design review is recorded.                             |
| T3        | Subject-set change for the audited PR's commits                             | Commit after focused validation and required review, separately from the test.                     |
| T4        | Increment 2 test and its design review                                      | Commit after the red output is kept and the design review is recorded.                             |
| T5        | Subject-set change for follow-up PRs                                        | Commit after focused validation, separately from the test.                                         |
| T6        | Increment 3 tests and their design review                                   | Hold for maintainer review (T8). Do not commit, verify finally, or open a PR before it.            |
| T7        | `gh` failure reported as a usage error                                      | Commit after T8 and focused validation.                                                            |
| T9        | Script help text and skill description                                      | Commit after `linter all` passes.                                                                  |
| T1, T10   | Evidence file entries (kept apart from product changes)                     | Commit after the evidence is recorded.                                                             |

Use Conventional Commits with the `pr-reviews` scope and sign every commit with GPG. Keep each red
test run in `test-design-review.md` or the evidence file even after its production step turns it green.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/validate-audit-record-merged-fix-subjects/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [x] Spec-only PR merged into `develop` before implementation
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, the new unit tests)
- [x] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [x] Implementation completion review recorded: issue-local retrospective created for material discoveries, or the progress log states why none was needed
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [x] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-10-21 11:00 UTC - GitHub Copilot (Claude Sonnet 5.5) - Draft created after the PR #2599
  review round. Reproduced the failure with `--base torrust/develop` and found that the default
  `--base develop` passes only because the local `develop` branch is stale. This corrects an
  earlier statement in PR #2599's description, which said the default base fails.
- 2026-10-21 11:03 UTC - GitHub Copilot (Claude Sonnet 5.5) - The maintainer approved the draft. Created
  issue #2606, linked it as a sub-issue of EPIC #2278 (`parent_issue_url` verified), and moved this
  specification to `docs/issues/open/2606-2278-validate-audit-record-merged-fix-subjects/`.
- 2026-10-21 11:51 UTC - GitHub Copilot (Claude Sonnet 5.5) - Addressed Copilot finding F1 on PR #2607: created
  `manual-verification-evidence.md` with the reproduction runs made at 11:50 UTC (V1, V2; outcome
  Reproduced), and marked T1 done.
- 2026-10-21 11:53 UTC - GitHub Copilot (Claude Sonnet 5.5) - Addressed Copilot finding F2 on PR #2607: the
  Implementation Plan and Commit Points now require `write-unit-test`, three behavior-sized test
  increments with kept red output, a recorded prose-first Arrange-Act-Assert design review per
  increment, separate production steps, and a maintainer review stop (T8) before final verification,
  the last commit, and the PR.
- 2026-10-21 11:55 UTC - GitHub Copilot (Claude Sonnet 5.5) - Addressed Copilot finding F3 on PR #2607: scenario
  M2 now updates local `develop` separately with `git fetch torrust develop:develop` and keeps
  `HEAD` unchanged between the two runs, so equal results can support AC2.
- 2026-10-21 13:56 UTC - GitHub Copilot (Claude Sonnet 5.5) - Processed Copilot finding F4 of PR #2607 (review
  5599559014, submitted before that PR merged): added the Implementation Completion Review section
  and aligned the two completion checkpoints with `docs/templates/ISSUE.md`. Maintainer approval:
  <https://github.com/torrust/torrust-tracker/pull/2607#issuecomment-6317834429>.
- 2026-10-24 19:45 UTC - GitHub Copilot - Started implementation on branch
  `2606-validate-audit-record-merged-fix-subjects`, cut from `torrust/develop` at `abce11ec8`.
  Maintainer decisions before implementing: keep the T8 stop for review of the test increments
  (T2 to T6) before T7, final verification, and the PR; produce `test-design-review.md` as
  specified; commit as the plan's Commit Points describe, with GPG signing; parse follow-up
  pull requests as recorded in Design and Ownership Review.
- 2026-10-24 19:56 UTC - GitHub Copilot - Implemented T2 to T7 and T9 (see the commits on this
  branch) and ran T10. The maintainer reviewed the T2 to T6 tests (T8) and approved them. Evidence is in
  `manual-verification-evidence.md` and `test-design-review.md`. Added two `pulls_to_check`
  tests beyond the planned case list to pin the follow-up URL parsing decision.
- 2026-10-24 20:27 UTC - GitHub Copilot - Implementation completion review: no retrospective
  was needed. The `gh api .../pulls/<n>/commits` source worked, including after merge; the change
  stayed inside the subject-set function and its call (plus one small `pulls_to_check` function);
  T10 matched the expectation that no audit regresses (11 of 157 audits with a `FIXED` row fail
  both before and after, for reasons in their own content). A transient GitHub HTTP 502 seen during
  T10 produced the designed exit 2 with the endpoint named.
- 2026-10-26 08:30 UTC - GitHub Copilot - PR #2641 merged on 2026-10-25 and issue #2606 closed.
  Archived this specification to `docs/issues/closed/` and set `status: done` and `related-pr: 2641`.
  The independent-reviewer checkpoint stays unchecked: the review findings F1 to F6 submitted on
  PR #2641 are not yet processed (see that pull request).

## Acceptance Criteria

- [x] AC1: With an up-to-date base (`--base torrust/develop`), the validator passes on the PR #2584
  and PR #2599 audits.
- [x] AC2: The result does not change with a stale or current local `develop` for pushed fixes.
- [x] AC3: A subject found in none of the three sources is still rejected.
- [x] AC4: A `gh` failure while listing pull-request commits exits with code 2 and names the
  endpoint.
- [x] AC5: The script's help text and the `process-pr-review` skill describe the three sources.
- [x] `linter all` exits with code `0`
- [x] The new unit tests pass
- [x] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria are re-reviewed after implementation and reflect actual behavior

## Verification Plan

### Automatic Checks

- `linter all`
- `python3 -m unittest discover -s .github/skills/dev/pr-reviews/process-pr-review/scripts`
  (Python 3 standard library)

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                                         | Human-oriented command/steps                                                                                                                                | Expected Result                                          | Status | Evidence                                     |
| --- | ------------------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------- | ------ | -------------------------------------------- |
| M1  | Merged audits pass with a current base           | `git fetch torrust develop`, then `validate-audit-record.py --pr-number 2584 --base torrust/develop` and the same for `2599`                                | Red before the fix (4 failures), green after, exit 0     | DONE | Red: `manual-verification-evidence.md` section V1; green: V1 (recheck) |
| M2  | Result is independent of the local `develop`     | On the implementation branch, record `git rev-parse HEAD`, `git rev-parse develop`, and `git rev-parse torrust/develop`. Run the M1 command with `--base develop` while local `develop` is behind. Update local `develop` without touching `HEAD` by running `git fetch torrust develop:develop` (it fast-forwards only, and refuses when `develop` is checked out or has diverged), then confirm `git rev-parse develop` equals `torrust/develop` and rerun the same command. Record `git rev-parse HEAD` again after the last run                                          | Same result in both runs, with `HEAD` unchanged                            | DONE | Dependency on the base: `manual-verification-evidence.md` section V2; recheck: V2 (recheck) |
| M3  | A bogus subject is still rejected                | Copy the PR #2599 audit, change the `Resolution reference` to a subject that exists in no commit, run the validator with `--audit-file`                    | Exit 1 with one `not a commit subject` failure           | DONE   | `manual-verification-evidence.md` section V3 |

Notes:

- Every recorded command result identifies the runtime that produced it (Python 3 version and
  the `gh` version).
- Create `manual-verification-evidence.md` from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`
  when executing these scenarios.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                                           |
| ----- | ---------------------- | -------------------------------------------------- |
| AC1   | DONE                   | Red V1, green V1 (recheck) in `manual-verification-evidence.md` |
| AC2   | DONE                   | `manual-verification-evidence.md` V2               |
| AC3   | DONE                   | Unit test and V3                                   |
| AC4   | DONE                   | Unit test                                          |
| AC5   | DONE                   | Diff of the script help text and the skill section |

## Risks and Trade-offs

- A `gh` rate limit or outage now blocks validation, because the commit lists come from GitHub.
  The validator already needs `gh` for review comments, so no new dependency appears.
- Accepting `<base>..HEAD` keeps a small false-pass window (see Design and Ownership Review).
  Closing it needs the Rust migration to resolve the PR's recorded head, which is out of scope.
- If the script is migrated to Rust before this lands, port the three-source rule instead of the
  current single range.

## Implementation Completion Review

After implementation, compare the result with this specification. Record invalidated assumptions,
material design changes, unexpected validation findings, and reusable lessons.

- Retrospective: `Not yet assessed`
- Create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md`
  in this issue's directory when any of these happened:
  - the `gh api .../pulls/<n>/commits` source proved unusable (for example truncated lists or rate
    limits) and the three-source design changed;
  - the fix needed more than the planned change to `branch_subjects` and its call, or the Rust
    migration became necessary;
  - the rerun of the existing audits (T10) failed or passed differently than expected;
  - the work taught a reusable lesson for the review workflow.
- If none of these happened, add a concise progress-log entry that says why the work had no
  material discovery.
- When an independent reviewer receives this folder-style specification, the reviewer records their
  result in `agent-review-reports.md` using `docs/templates/AGENT-REVIEW-REPORTS.md`.
