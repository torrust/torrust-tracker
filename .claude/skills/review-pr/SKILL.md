---
name: review-pr
description: "Review an existing pull request against the review checklist: code style, tests, docs. Use when the PR exists; review-task checks a branch without one."
metadata:
  author: torrust
  version: "1.1"
---

# Reviewing a Pull Request

Use this skill only when a pull request exists (PR number or URL is available).

If there is no PR yet and you need to validate task completion on a local branch, use:
`.claude/skills/review-task/SKILL.md`.

## Quick Overview Approach

1. Confirm that the pull request exists: a number or a URL is required.
2. Read the PR title and description for context, and gather its metadata: the linked issue, the
   base branch, the merge state and time, and the checks when they are available.
3. Check the diff for scope of change. Review the real diff and the checks of the pull request, not
   the intent of the caller.
4. Identify the affected packages and components
5. Apply the checklist below and classify the findings by severity

## PR Review Checklist

### PR Metadata

- [ ] Title follows Conventional Commits format
- [ ] Description clearly explains what changes were made and why
- [ ] Issue is linked (`Closes #<number>` or `Refs #<number>`)
- [ ] Target branch is `develop` (not `main`)

### Code Quality

- [ ] Code follows existing patterns in affected packages
- [ ] No unused imports, variables, or functions
- [ ] No `#[allow(...)]` suppressions unless clearly justified with a comment
- [ ] Errors handled properly (use `thiserror` for structured errors, avoid `.unwrap()`)
- [ ] No security vulnerabilities (OWASP Top 10 awareness)

### Tests

- [ ] New functionality has unit tests
- [ ] Integration tests added if applicable
- [ ] All existing tests still pass
- [ ] Test code is clean, readable, and maintainable

### Documentation

- [ ] Public API items have doc comments
- [ ] `AGENTS.md` updated if architecture changed
- [ ] Markdown docs updated if user-facing behavior changed
- [ ] Spell check: new technical terms added to `project-words.txt`

### Rust-Specific

- [ ] Imports grouped: std → external → internal
- [ ] Line length within `max_width = 130`
- [ ] GPG-signed commits

## Output

Return:

1. Scope reviewed (the pull-request number and the key files)
2. Findings by severity (see [Providing Feedback](#providing-feedback))
3. Checklist gaps
4. Overall verdict: `APPROVE`, `REQUEST_CHANGES`, `COMMENT` or `POST_MERGE_FINDINGS`

Do not approve while a blocker is unresolved: a `COMMENT` verdict is for a review whose findings
are not blockers, and an unresolved blocker gives `REQUEST_CHANGES`.

## A Merged Pull Request

When the pull request is already merged, label the result `POST_MERGE_FINDINGS` and give no
merge-readiness verdict. Do not implement the findings and do not create a follow-up branch.
Direct the caller to the post-merge workflow of
[`process-pr-review`](../process-pr-review/SKILL.md), which needs the maintainer's explicit
approval before any action that changes something.

## Recording The Review

The task you were given tells you which case applies. The review is not a review of a task of a
branch (use [`review-task`](../review-task/SKILL.md) for that): do not run the acceptance review
of a task, and do not mark the workflow checkpoints of an issue specification unless the caller
asks for it and the evidence is there.

### A Reply File

When the task carries the line that names a reply file (decision D17 of issue #2904), write the
review there as one JSON file before you finish. The path is the one the line gives:
`agent-replies/<agent>-<subject>-r<round>.json` in the folder of the issue specification, with
`pr-reviewer` as the agent, `pr<number>` as the subject, and the round counted from 1 (decision
D18). The rounds go from 1 to 3, and the validator refuses a name with a higher round: when the
line names a round above 3, write no file, say so in your final message and leave it to the caller
to stop and ask the maintainer. The fields:

- `schema-version`: 1; `agent`: `pr-reviewer`; `model`: the full model ID you ran on; `subject`
  and `round`: the same as in the file name; `created-utc`: the time, from `date -u`.
- `verdict`, from the verdict of the Output: `REVIEW_PASSED` for `APPROVE` and for `COMMENT`
  (which has no blocker); `REVIEW_FAILED` for `REQUEST_CHANGES`; `DONE` for
  `POST_MERGE_FINDINGS`, with the label at the start of the `summary`; `QUESTION` when only the
  contributor can answer something; `BLOCKED` when you cannot review.
- `summary`: the verdict in a few sentences.
- `findings`: one for each finding, with a `severity` (`Blocker`, `Major`, `Minor`, `Nit` or
  `Suggestion`), a `location` and a `description` that carries the remediation.
- `questions`: what the contributor must answer, one text each, or an empty list.
- `evidence`: the commands you ran and what they showed, the checks you read, and the checklist
  gaps, one text each.

Check the file with the validator before you finish:

```sh
cargo run --quiet --package agent-reply-validator --bin agent-reply-validator -- <reply file>
```

A reply file is committed by the caller with the work it describes. It is independent of the
handling of the review threads of the pull request, and does not replace the audit record of
`process-pr-review`.

### No Folder-Style Specification

When the task names no reply file, because no folder-style issue specification was supplied (a
direct diff or package review, or a legacy standalone specification), write no file. Do not create,
migrate or modify an issue-local report. State in the result: `Issue-local report skipped: no
folder-style issue specification was supplied.`

### A Branch Without The `pr-reviewer` Agent

A review made on a branch that does not have the `pr-reviewer` agent yet is recorded in
`agent-review-reports.md` in the specification's folder, by the procedure of the section "A Branch
Without The `task-reviewer` Agent" of [`review-task`](../review-task/SKILL.md#recording-the-review).
The entry records the pull-request number, the base branch, the reviewed files, the checks, the
findings by severity, the checklist gaps, the verdict and the follow-up owner and action.

## Providing Feedback

Finding formatting is advisory. Do not reject a review because it omits the format; the PR author
normalizes all feedback under
`.claude/skills/process-pr-review/SKILL.md`.

Use one independently actionable finding per inline review thread. When formatting a finding, start
its first line with:

```text
[<Severity>][<FindingId>] <summary>
```

`<Severity>` is one of `Blocker`, `Major`, `Minor`, `Nit`, or `Suggestion`. Use the original
`<FindingId>` for a re-raised finding and state that it is a re-raise in the thread body. Keep
review bodies to the round verdict or summary; do not repeat detailed inline findings there. See
`docs/templates/REVIEW-FINDINGS.md` for the advisory format.

## Re-pushed Heads and Checklists

After a re-push, scope the next review round to the current PR head. Do not re-raise an earlier
finding unless the current tree still has the concern or the re-push regressed it; retain its
original finding ID when you do re-raise it.

Mark checklist items `N/A` only after determining that they do not apply to the PR. Leave an item
unchecked when it was not assessed; do not use `N/A` to represent an unreviewed item.

## Standards Reference

All code quality standards are defined in the root `AGENTS.md`. When pointing to a
standard, reference the relevant section of `AGENTS.md`.
