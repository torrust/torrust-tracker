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

## Checklist Before Committing Docs

- [ ] No `#NUMBER` patterns used for enumeration or step numbering
- [ ] Ordered lists use Markdown syntax (`1.` `2.` `3.`)
- [ ] Any `#NUMBER` present is an intentional issue/PR reference
- [ ] Tables are consistently formatted
- [ ] `linter markdown` and `linter cspell` pass
