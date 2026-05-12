---
name: create-refactor-plan
description: Guide for creating refactor plans in the torrust-tracker project. Covers identifying quality gaps, decomposing them into trackable items ordered by impact vs effort, writing the plan document, and committing it. Use when planning improvements to readability, testability, maintainability, modularity, or documentation quality. Triggers on "create refactor plan", "refactor plan", "plan refactor", "post-implementation improvements", "code quality plan", or "technical debt plan".
metadata:
  author: torrust
  version: "1.0"
  semantic-links:
    related-artifacts:
      - docs/templates/REFACTOR-PLAN.md
---

# Creating Refactor Plans

## When to Write a Refactor Plan

Write a refactor plan when:

- A completed implementation has known quality gaps that are not blocking but worth tracking.
- A code review, post-implementation audit, or routine quality check identifies improvements
  across multiple dimensions (readability, testability, maintainability, modularity, docs).
- The improvements are too numerous or varied to address in a single commit but collectively
  deserve a structured approach.

Do **not** write a refactor plan for:

- A single trivial fix — just fix it in place.
- Bug fixes — those belong in issue specs (`docs/templates/ISSUE.md`).
- Architectural decisions — those belong in ADRs (`docs/templates/ADR.md`).

## Workflow Overview

1. **Identify quality gaps** by auditing the code, spec, and tests.
2. **Decompose** gaps into discrete, independently completable items.
3. **Order** items by impact vs effort (highest impact / lowest effort first).
4. **Draft the plan** using the template.
5. **Run linters** and fix any issues.
6. **Commit** the plan.
7. **Implement** items one at a time, ticking checkboxes as each is done.
8. **Revisit** the plan after implementation to evaluate whether the template and skill
   need improvements.

## Step-by-Step Process

### Step 1: Identify and Categorize Quality Gaps

Review the following dimensions systematically:

| Dimension       | Questions to Ask                                                                                      |
| --------------- | ----------------------------------------------------------------------------------------------------- |
| Correctness     | Are there edge cases not tested? Does documentation match actual behaviour?                           |
| Readability     | Is intent clear at a glance? Are names self-explanatory? Are surprising choices explained?            |
| Testability     | Can behaviour be verified without spawning a process? Are unit and integration paths both covered?    |
| Maintainability | Are concerns separated? Is any function too long or doing too many things?                            |
| Modularity      | Are abstractions reusable? Are conversions done in idiomatic places (e.g. `From` impls)?              |
| Documentation   | Are public APIs documented? Are non-obvious invariants or contract details captured in spec and code? |

### Step 2: Write Each Item

Each item in the plan must contain:

- **Problem**: what is wrong and why it matters — be specific (name files, functions, line ranges).
- **Files**: the files affected.
- **Change**: what exactly changes — prefer concrete before/after examples over vague descriptions.

Use the effort and impact labels consistently:

| Impact | Meaning                                                   |
| ------ | --------------------------------------------------------- |
| High   | Correctness, observability, or user-facing contract issue |
| Medium | Developer experience, maintainability, clarity            |
| Low    | Nice-to-have polish or future-proofing                    |

| Effort  | Meaning                                             |
| ------- | --------------------------------------------------- |
| Trivial | One-liner or wording change, no logic involved      |
| Low     | Small, self-contained code or doc change (< 1 hour) |
| Medium  | Moderate refactor or new abstraction (1–4 hours)    |
| High    | Significant new code, e.g. mock server (> 4 hours)  |

### Step 3: Order Items

Sort items in the plan and in the execution table by:

1. Highest impact first.
2. Lowest effort first within the same impact band.

This ensures the most valuable, cheapest improvements are visible and tackled first.

### Step 4: Create the Plan File

Plans follow the same `drafts/` → `open/` → `closed/` lifecycle as issue specs.

```bash
touch docs/refactor-plans/drafts/{short-description}.md
```

Use the template at [docs/templates/REFACTOR-PLAN.md](../../../../docs/templates/REFACTOR-PLAN.md).

Naming convention: `{related-artifact-short-description}.md`

Example: `1178-monitor-udp-post-implementation-improvements.md`

Each item heading uses a checkbox and an impact/effort label:

```markdown
### 1. [ ] {Title} [HIGH impact / TRIVIAL effort]
```

The execution table also has a `Status` column with `[ ]`:

```markdown
| 1 | [ ] | {Item} | High | Trivial |
```

To mark an item done, flip `[ ]` → `[x]` in **both** the heading and the table row.

### Step 5: Validate and Commit

Move the plan from `drafts/` to `open/` when implementation starts:

```bash
git mv docs/refactor-plans/drafts/{filename}.md docs/refactor-plans/open/{filename}.md
```

```bash
linter all    # Must pass

git add docs/refactor-plans/
git commit -S -m "docs({scope}): add refactor plan for {description}"
```

### Step 6: Implement and Track Progress

Work through items in order. After completing each item:

1. Flip `[ ]` → `[x]` in the item heading.
2. Flip `[ ]` → `[x]` in the execution table row.
3. Run `linter all` and fix any new issues.
4. Commit the implementation and the updated plan together.

When all items are done, move the plan to `closed/`:

```bash
git mv docs/refactor-plans/open/{filename}.md docs/refactor-plans/closed/{filename}.md
git commit -S -m "docs({scope}): close refactor plan for {description}"
```

### Step 7: Revisit the Template and Skill

After implementing all items, evaluate:

- Did the template structure make items easy to write and track?
- Were the impact/effort labels consistently interpreted?
- Is anything missing that would have made the plan more useful?

Update `docs/templates/REFACTOR-PLAN.md` and this skill file if improvements are identified.

## Naming Convention

File name format: `{related-artifact-short-description}.md`

| Lifecycle stage | Folder                        |
| --------------- | ----------------------------- |
| Being written   | `docs/refactor-plans/drafts/` |
| In progress     | `docs/refactor-plans/open/`   |
| All done        | `docs/refactor-plans/closed/` |

## Relationship to Other Artifacts

| Artifact      | When to Use Instead                                               |
| ------------- | ----------------------------------------------------------------- |
| Issue spec    | When the improvement is a bug fix or new feature                  |
| ADR           | When the improvement requires documenting an architectural choice |
| Refactor plan | When improvements are quality gaps with no new functionality      |
