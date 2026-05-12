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
6. Report pass/fail with remediation for any gaps.

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

## Output

Return:

1. Scope reviewed
2. Acceptance criteria matrix (`PASS`/`FAIL`/`PENDING` + evidence)
3. Blocking findings
4. Issue spec updates made
5. Overall result (`REVIEW PASSED` or `REVIEW FAILED`)

## Not In Scope

- Reviewing an open pull request (use `review-pr` for that).
- Publishing review comments to a PR.
- Merging or closing PRs.
