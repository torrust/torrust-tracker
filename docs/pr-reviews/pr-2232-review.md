---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - review-finding:pr-2232-f1
    - review-finding:pr-2232-f2
    - review-finding:pr-2232-f3
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

<!-- cspell:disable -->

# PR #2232 Review Audit

Source: pull-request reviews and inline review threads for
https://github.com/torrust/torrust-tracker/pull/2232.

## Findings

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2232-f1 | Copilot | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2232-f2 | Copilot | Suggestion (inferred) | maintainability | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2232-f3 | Copilot | Suggestion (inferred) | maintainability | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - PR-template issue metadata could render in a PR body

- PR number: 2232
- Source review ID: PRR_kwDOGp2yqc8AAAABNsfloA
- Source URL: https://github.com/torrust/torrust-tracker/pull/2232#discussion_r4018776876
- Concern: `.github/PULL_REQUEST_TEMPLATE/review-findings.md` used issue-template YAML frontmatter,
  which GitHub renders verbatim inside a pull-request body.
- Solution: removed the issue-form metadata and kept the advisory guidance as plain Markdown.
- Current-tree verification: `linter markdown` and the pre-commit gate passed after removal.
- Resolution reference: fix(pr-reviews): remove PR template metadata
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2232#discussion_r4019117053

### F2 - Nightly formatting ran after aggregate linting

- PR number: 2232
- Source review ID: PRR_kwDOGp2yqc8AAAABNsfloA
- Source URL: https://github.com/torrust/torrust-tracker/pull/2232#discussion_r4018776945
- Concern: the pre-commit hook ran `linter all` before the nightly rustfmt check, so a common
  formatting failure surfaced only after the slower aggregate step.
- Solution: moved the nightly formatting step before aggregate linting so it fails fast; no check
  was removed.
- Current-tree verification: the pre-commit gate reports nightly formatting as step 5 and
  aggregate linting as step 6; all steps passed.
- Resolution reference: ci(hooks): run nightly formatting earlier
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2232#discussion_r4019117216

### F3 - Flow-style semantic-link YAML was less robust than a block list

- PR number: 2232
- Source review ID: PRR_kwDOGp2yqc8AAAABNsfloA
- Source URL: https://github.com/torrust/torrust-tracker/pull/2232#discussion_r4018776990
- Concern: the canonical skill declared `semantic-links.related-artifacts` as a multi-line
  flow-style YAML sequence with a trailing comma, which is easier to break and less portable.
- Solution: converted the sequence to the repository's block-list style with identical values.
- Current-tree verification: YAML and Markdown linting, the focused contract test, and the
  pre-commit gate passed.
- Resolution reference: docs(pr-reviews): use portable semantic link YAML
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2232#discussion_r4019117462

## Processing Log

- 2026-09-15 18:47 UTC - Collected the authoritative GraphQL thread snapshot and the submitted
  Copilot review `PRR_kwDOGp2yqc8AAAABNsfloA`; normalized its three independently actionable
  inline findings as F1-F3 in source order.
- 2026-09-15 18:50 UTC - Applied, focused-validated, and separately GPG-signed fixes for F1-F3.
- 2026-09-15 18:51 UTC - Replied to and resolved all three Copilot threads. A refreshed GraphQL
  snapshot reported no unresolved threads. Future asynchronous reviews append new source rows to
  this audit without changing these immutable finding references.
- 2026-09-15 19:05 UTC - First real use showed the single 15-column findings table was hard to
  read; split the audit into a compact tracking table plus a Finding Details section, updating the
  canonical template, workflow, and contract test in the same change. All required fields remain
  recorded.
