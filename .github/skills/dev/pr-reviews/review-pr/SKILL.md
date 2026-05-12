---
name: review-pr
description: Review a pull request for the torrust-tracker project. Covers checklist-based PR quality verification, code style standards, test requirements, documentation, and how to submit review feedback. Use when asked to review a PR, check a pull request, or provide feedback on code changes. Triggers on "review PR", "review pull request", "check PR quality", or "code review".
metadata:
  author: torrust
  version: "1.0"
---

# Reviewing a Pull Request

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

Categorize comments to help the author prioritize:

- **Blocker** — must fix before merge (correctness, security, breaking changes)
- **Suggestion** — improvement recommended but not blocking
- **Nit** — minor style/readability point

## Standards Reference

All code quality standards are defined in the root `AGENTS.md`. When pointing to a
standard, reference the relevant section of `AGENTS.md`.
