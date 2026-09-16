---
name: review-pr
description: Review an existing pull request for the torrust-tracker project. Covers checklist-based PR quality verification, code style standards, test requirements, documentation, and review feedback. Use only when a PR already exists.
metadata:
  author: torrust
  version: "1.1"
---

# Reviewing a Pull Request

Use this skill only when a pull request exists (PR number or URL is available).

If there is no PR yet and you need to validate task completion on a local branch, use:
`.github/skills/dev/task-reviews/review-task/SKILL.md`.

## Quick Overview Approach

1. Read the PR title and description for context
2. Check the diff for scope of change
3. Identify the affected packages and components
4. Apply the checklist below

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

## Providing Feedback

Finding formatting is advisory. Do not reject a review because it omits the format; the PR author
normalizes all feedback under
`.github/skills/dev/pr-reviews/process-pr-review/SKILL.md`.

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
