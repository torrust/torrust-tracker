---
doc-type: refactor-plan
status: draft
related-issue: null
spec-path: docs/refactor-plans/{short-description}.md
last-updated-utc: YYYY-MM-DD HH:MM
semantic-links:
  skill-links:
    - create-refactor-plan
  related-artifacts:
    - .github/skills/dev/planning/create-refactor-plan/SKILL.md
---

<!-- skill-link: create-refactor-plan -->

# Refactor Plan — {Title}

## Goal

State in one or two sentences what the refactor achieves and why it is worthwhile.
Focus on the quality property improved (readability, testability, maintainability, etc.).

Related artifact: `{path/to/related/file-or-issue-spec.md}`

## Items

<!-- Copy and repeat this block for each item. Order from highest impact/lowest effort
     to lowest impact/highest effort. Number items sequentially. -->

### 1. [ ] {Short title} [{IMPACT} impact / {EFFORT} effort]

**Problem**: Describe the current state and why it is a problem. Be specific — name
files, line numbers, or function names where relevant.

**Files**:

- `{path/to/file.rs}`

**Change**: Describe exactly what needs to change. Prefer concrete before/after
examples over abstract descriptions.

---

### 2. [ ] {Short title} [{IMPACT} impact / {EFFORT} effort]

**Problem**: ...

**Files**:

- `{path/to/file.rs}`

**Change**: ...

---

## Order of Execution

| Order | Status | Item                  | Impact | Effort  |
| ----- | ------ | --------------------- | ------ | ------- |
| 1     | [ ]    | {Short title of item} | High   | Trivial |
| 2     | [ ]    | {Short title of item} | Medium | Low     |

<!-- Impact values: High / Medium / Low -->
<!-- Effort values: Trivial / Low / Medium / High -->
