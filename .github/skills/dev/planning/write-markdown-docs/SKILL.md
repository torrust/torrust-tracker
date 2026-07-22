---
name: write-markdown-docs
description: Guide for writing Markdown documentation in this project. Covers GitHub Flavored Markdown pitfalls, especially the critical #NUMBER pattern that auto-links to GitHub issues and PRs (NEVER use #1, #2, #3 as step/list numbers). Use ordered lists or plain numbers instead. Covers intentional vs accidental autolinks for issues, @mentions, and commit SHAs. Use when writing .md files, documentation, issue descriptions, PR descriptions, or README updates. Triggers on "markdown", "write docs", "documentation", "#number", "github markdown", "autolink", "markdown pitfall", or "GFM".
metadata:
  author: torrust
  version: "1.0"
---

# Writing Markdown Documentation

## Critical: #NUMBER Auto-links to GitHub Issues

**GitHub automatically converts `#NUMBER` → link to issue/PR/discussion.**

```markdown
❌ Bad: accidentally links to issues

- Task #1: Set up infrastructure ← links to GitHub issue #1
- Task #2: Configure database ← links to GitHub issue #2

Step #1: Install dependencies ← links to GitHub issue #1
```

The links pollute the referenced issues with unrelated backlinks and confuse readers.

### Fix: Use Ordered Lists or Plain Numbers

```markdown
✅ Solution 1: Ordered list (automatic numbering)

1. Set up infrastructure
2. Configure database
3. Deploy application

✅ Solution 2: Plain numbers (no hash)

- Task 1: Set up infrastructure
- Task 2: Configure database

✅ Solution 3: Alternative formats

- Task (1): Set up infrastructure
- Task [1]: Set up infrastructure
```

## When #NUMBER IS Intentional

Use `#NUMBER` only when you explicitly want to link to that GitHub issue/PR:

```markdown
✅ Intentional: referencing issue
This implements the behavior described in #42.
Closes #1697.
```

## Other GFM Auto-links to Know

```markdown
@username → links to GitHub user profile (use intentionally for mentions)
abc1234 (SHA) → links to commit (useful for references)
owner/repo#42 → cross-repo issue link
```

## Frontmatter

Frontmatter use in `docs/` varies by document type: **required** for issue specs and
EPIC specs, **recommended** for ADRs and refactor plans, and **optional** for short
reference pages and README files.

Follow the frontmatter convention defined in
[`docs/skills/semantic-skill-link-convention.md`](../../../../../docs/skills/semantic-skill-link-convention.md),
which specifies the required fields for each document type and the shape of
`semantic-links` entries.

When a draft issue spec identifies source artifacts it will change, add an
`issue-spec: <repo-relative-draft-path>` marker to those artifacts when the link
is high-signal. Once the GitHub issue is created, replace the draft-path marker
with `issue: #<number>`; do not keep paths that will become stale when the spec
moves from `drafts/` to `open/` or `closed/`.

## Repo Markdown vs. GitHub Markdown

The `.markdownlint.json` configuration at the repository root applies **only to `.md` files
tracked in the repository**. It does not apply to Markdown written on GitHub surfaces such
as issue descriptions, PR descriptions, PR review comments, or discussion posts.

**Do not wrap lines when writing GitHub issue or PR body text.** Hard-wrapping lines in issue
or PR descriptions produces visually broken paragraphs on GitHub's web UI and is harder for
human readers to follow. Write each paragraph as a single continuous line and let GitHub's
rendering handle the wrapping.

| Surface                | Governed by `.markdownlint.json` | Line wrapping                                                |
| ---------------------- | -------------------------------- | ------------------------------------------------------------ |
| `.md` files in repo    | Yes                              | Follow repo config (MD013 disabled, but keep lines readable) |
| GitHub issue / PR body | No                               | Do **not** hard-wrap lines                                   |
| GitHub review comments | No                               | Do **not** hard-wrap lines                                   |

## Checklist Before Committing Docs

- [ ] No `#NUMBER` patterns used for enumeration or step numbering
- [ ] Ordered lists use Markdown syntax (`1.` `2.` `3.`)
- [ ] Any `#NUMBER` present is an intentional issue/PR reference
- [ ] Tables are consistently formatted
- [ ] Frontmatter is present and follows `docs/skills/semantic-skill-link-convention.md`
- [ ] `linter markdown` and `linter cspell` pass

## Checklist Before Submitting to GitHub

Apply this checklist to any Markdown body submitted via the GitHub API or CLI (issues, PR
descriptions, review comments, discussion posts) **before** calling the API:

- [ ] Each paragraph is written as a single continuous line — do **not** hard-wrap at any fixed column width
- [ ] No `#NUMBER` patterns used for enumeration or step numbering
- [ ] Any `#NUMBER` present is an intentional issue/PR reference
- [ ] Ordered lists use Markdown syntax (`1.` `2.` `3.`)
- [ ] Tables are consistently formatted
- [ ] No raw HTML unless GitHub's renderer requires it
