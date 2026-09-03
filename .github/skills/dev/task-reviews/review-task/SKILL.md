---
name: review-task
description: Review a completed implementation task before push/PR. Validates issue-spec acceptance criteria, scope, tests, docs, and lint readiness on a local branch. Use when asked to verify issue completion without an open PR.
metadata:
  author: torrust
  version: "1.0"
---

# Reviewing A Task (Pre-PR)

Use this skill when there is no pull request yet and the goal is to verify that implementation for
an issue/task is complete and ready to be pushed.

## Preconditions

- An issue spec exists (typically under `docs/issues/open/`).
- Local changes are available on the branch.
- No PR review workflow is required yet.

## Workflow

1. Read the issue spec and extract acceptance criteria.
2. Map each criterion to concrete evidence in changed files/tests.
3. Run relevant validation checks (`linter all` minimum, plus focused tests when applicable).
4. Classify each criterion as `PASS`, `FAIL`, or `PENDING`.
5. Update only verified checklist items in the issue spec.
6. Review the implementation completion evidence. Require an issue-local
   `implementation-retrospective.md` when the work revealed reusable lessons,
   material design changes, or meaningful deviations from the plan. Otherwise,
   require a concise issue progress-log entry explaining why no retrospective
   was needed.
7. Require a folder-style specification before accepting an issue-local
   retrospective. When a touched legacy single-file specification needs one,
   require migration to the documented folder layout first. `ISSUE.md` and
   `EPIC.md` are allowed primary-file exceptions in that layout.
8. Report pass/fail with remediation for any gaps.

## Task Review Checklist

### Scope And Criteria

- [ ] Issue spec path is identified.
- [ ] Acceptance criteria are fully listed.
- [ ] Claimed implementation scope matches actual changes.
- [ ] No scope creep beyond what the issue asks.

### Verification

- [ ] Each acceptance criterion has objective evidence.
- [ ] Required tests/lint checks pass.
- [ ] Docs updates are present when behavior changed.
- [ ] New terms are added to `project-words.txt` when needed.

### Spec Hygiene

- [ ] Only verified checklist items are marked done.
- [ ] Workflow checkpoints reflect pre-PR status correctly.
- [ ] Progress log includes meaningful, factual updates.
- [ ] Completion review records a retrospective or a rationale that one was unnecessary.
- [ ] Any retrospective belongs to a folder-style issue specification; the retrospective is lowercase while `ISSUE.md` and `EPIC.md` are allowed primary-file exceptions.

## Output

Return:

1. Scope reviewed
2. Acceptance criteria matrix (`PASS`/`FAIL`/`PENDING` + evidence)
3. Repository-convention findings
4. Completion-review finding
5. Issue spec updates made
6. Overall result (`REVIEW PASSED` or `REVIEW FAILED`)

## Not In Scope

- Reviewing an open pull request (use `review-pr` for that).
- Publishing review comments to a PR.
- Merging or closing PRs.
