---
name: create-issue
description: Guide for creating GitHub issues in the torrust-tracker project. Covers the full workflow from specification drafting, user review, to GitHub issue creation with proper documentation and file naming. Supports task, bug, feature, and epic issue types. Use when creating issues, opening tickets, filing bugs, proposing tasks, or adding features. Triggers on "create issue", "open issue", "new issue", "file bug", "add task", "create epic", or "open ticket".
metadata:
  author: torrust
  version: "1.1"
  semantic-links:
    related-artifacts:
      - docs/templates/ISSUE.md
      - docs/templates/EPIC.md
---

# Creating Issues

## Issue Types

| Type        | Label     | When to Use                                  |
| ----------- | --------- | -------------------------------------------- |
| **Task**    | `task`    | Single implementable unit of work            |
| **Bug**     | `bug`     | Something broken that needs fixing           |
| **Feature** | `feature` | New capability or enhancement                |
| **Epic**    | `epic`    | Major feature area containing multiple tasks |

## Workflow Overview

The process is **spec-first**: write and review a specification before creating the GitHub issue.

Lifecycle docs:

- Open issue specs: [`docs/issues/open/README.md`](../../../../../docs/issues/open/README.md)
- Closed issue buffer: [`docs/issues/closed/README.md`](../../../../../docs/issues/closed/README.md)

1. **Draft specification** document in `docs/issues/drafts/` using the repository templates
   appropriate to the issue type (`docs/templates/ISSUE.md` for Task/Bug/Feature,
   `docs/templates/EPIC.md` for Epic). Use a folder-style specification when the issue needs
   supporting artifacts that belong exclusively to that specification.
2. **User reviews** the draft specification
3. **Create GitHub issue**
4. **Move spec file to `docs/issues/open/`** and include the issue number
5. **Pre-commit checks** and commit the spec

For complex or high-impact issues, a **spec-first PR** is recommended:

- Open a branch containing only issue-spec/EPIC documentation changes
- Submit and merge that PR into `develop` first
- Start implementation only after the specification PR has been reviewed and merged
- Use `Related to #<number>` (not `Closes #<number>`) in the spec-only PR body to avoid
  auto-closing the issue on merge (see the `open-pull-request` skill)

This improves visibility and allows maintainers/contributors to review scope and acceptance
criteria before code changes begin.

**Never create the GitHub issue before the user reviews and approves the specification.**

## Step-by-Step Process

### Step 1: Draft Issue Specification

Create a specification with a **temporary name** (no subissue number yet). When the proposed
subissue has a known parent EPIC, prefix the draft name with that EPIC's GitHub issue number:

```text
docs/issues/drafts/{epic-issue-number}-{short-description}.md
```

This prefix identifies the known parent EPIC; it is not a placeholder for the future subissue's
own GitHub issue number. Do not infer a parent EPIC from related issues, ADRs, or topic overlap.
If the parent is not explicitly established, use an unnumbered descriptive draft name:

```bash
touch docs/issues/drafts/{short-description}.md
```

Use a folder-style specification when it needs issue-local supporting artifacts, such as an
immutable source snapshot, evidence, or design input. Place the main specification in `ISSUE.md`:

```bash
mkdir -p docs/issues/drafts/{short-description}
touch docs/issues/drafts/{short-description}/ISSUE.md
```

For a known parent EPIC, apply the same prefix to a folder-style draft:

```bash
mkdir -p docs/issues/drafts/{epic-issue-number}-{short-description}
touch docs/issues/drafts/{epic-issue-number}-{short-description}/ISSUE.md
```

Select the template by issue type:

- Task/Bug/Feature: [docs/templates/ISSUE.md](../../../../docs/templates/ISSUE.md)
- Epic: [docs/templates/EPIC.md](../../../../docs/templates/EPIC.md)

Before presenting the draft for review, initialize these sections so progress can be tracked
explicitly during implementation:

- YAML frontmatter metadata (including `status`, `epic`, `github-issue`, `spec-path`, and `last-updated-utc`)
- `Implementation Plan` (or `Subissues` for epics) with explicit status values
- `Architectural Decisions`, linking relevant ADRs and listing any ADRs expected from the work
- `Progress Tracking` (`Workflow Checkpoints` and first `Progress Log` entry)
- `Acceptance Criteria` and `Acceptance Verification`

The draft must also include a verification policy that is explicit and enforceable:

- Automatic checks to run after implementation (`linter all`, relevant tests, pre-push checks when applicable)
- Manual verification scenarios with status + evidence tracking (mandatory)
- A post-implementation acceptance criteria review step

During implementation, create an ADR when an important architectural decision
emerges, even if the issue draft did not anticipate it. Link the ADR from the
issue specification and update the architectural-decisions section.

Use **placeholders** for the issue number until after creation (for example `github-issue: null`
or `[To be assigned]` in the heading/body content).

Set `epic: {epic-issue-number}` only when the draft is an explicitly established subissue; otherwise
set `epic: null`. An EPIC subissue draft must also identify the parent directly below its title.

After drafting, run linters:

```bash
linter markdown
linter cspell
```

### Step 2: User Reviews the Draft

**STOP HERE** — present the draft to the user. Iterate until approved.

### Step 3: Create the GitHub Issue

After user approval, format the issue body and create the issue.

#### Format Body Text for GitHub

Before calling the GitHub API or CLI, review and reformat the issue body following the
`write-markdown-docs` checklist for GitHub surfaces:

- Write each paragraph as a **single continuous line** — do not hard-wrap at any fixed column width
- Use GitHub Flavored Markdown (GFM) conventions
- Check for accidental `#NUMBER` autolinks (only use `#NUMBER` for intentional issue/PR references)

#### Create the Issue

**GitHub CLI:**

```bash
gh issue create \
  --repo torrust/torrust-tracker \
  --title "{title}" \
  --body "{body}" \
  --label "{label}"
```

### Step 4: Move the Specification to Open Issues

Move from `drafts/` to `open/` using the assigned issue number. Preserve the chosen layout:

**Single-file specification:**

```bash
git mv docs/issues/drafts/{short-description}.md \
  docs/issues/open/{number}-{short-description}.md
```

For a subissue of a known EPIC, replace the draft's parent-only prefix with both GitHub issue
numbers:

```bash
git mv docs/issues/drafts/{epic-issue-number}-{short-description}.md \
  docs/issues/open/{number}-{epic-issue-number}-{short-description}.md
```

**Folder-style specification:**

```bash
git mv docs/issues/drafts/{short-description} \
  docs/issues/open/{number}-{short-description}
```

For a folder-style subissue, use
`docs/issues/open/{number}-{epic-issue-number}-{short-description}`.

For folder-style specifications, the main document is
`docs/issues/open/{number}-{short-description}/ISSUE.md`. Keep all issue-local artifacts in the
same directory. Update the `spec-path` and all internal artifact references after the move.

Update any issue number placeholders inside the file.

### Step 5: Commit and Push

```bash
linter all    # Must pass

git add docs/issues/
git commit -S -m "docs(issues): add issue specification for #{number}"
git push {your-fork-remote} {branch}
```

### Optional Step 6 (Recommended for Complex Issues): Spec-Only PR

When the issue is complex, cross-cutting, or likely to need scope negotiation, open a PR that
contains only the issue specification changes:

1. Branch from `develop`
2. Commit only spec changes (`docs/issues/`, and if needed templates/skills)
3. Push branch to your fork remote (for example `josecelano`)
4. Open PR in the **upstream repository** (`torrust/torrust-tracker`) targeting `develop`
5. If using fork-based workflow, set head as `{fork-owner}:{branch}` (for example
   `josecelano:1771-spec-first-pr-workflow`)
6. Do not open the PR in the fork repository unless explicitly requested
7. Merge PR after review
8. Start implementation work in a separate branch/PR

> **Important — do NOT auto-close the issue from a spec-only PR.**
> Use `Related to #<number>` in the PR body, never `Closes #<number>` / `Fixes #<number>` /
> `Resolves #<number>`. Those keywords trigger GitHub auto-close on merge.
> The issue must remain open until the implementation is merged.
> See the `open-pull-request` skill for the full issue-linking rules.

Policy notes:

- Never push directly to `develop` or `main`.
- To merge into `develop` or `main`, open a PR in `torrust/torrust-tracker` from a fork branch (`<fork-owner>:<branch>`).
- Remote names are contributor-specific (`josecelano`, `origin`, `torrust`, etc.); use your configured fork remote.

Recommended GitHub CLI command for fork-based PRs:

```bash
gh pr create \
  --repo torrust/torrust-tracker \
  --base develop \
  --head {fork-owner}:{branch} \
  --title "{title}" \
  --body-file {body-file}
```

## Verification Requirements for Issue Specs

When creating or updating issue/epic specs, ensure these requirements are present in the spec
before implementation starts:

1. **Automatic verification**: list required automated checks.
2. **Manual verification**: define concrete manual scenarios with commands/steps and expected results.
3. **Evidence tracking**: include status/evidence fields for manual scenarios.
4. **Post-implementation AC review**: explicitly require acceptance criteria to be re-reviewed
   against observed behavior before closing the issue.

Do not treat an issue as complete only because automated tests pass; manual validation is required.

## Naming Convention

Use one of these layouts:

| Layout      | Use when                                                  | Main specification path                 |
| ----------- | --------------------------------------------------------- | --------------------------------------- |
| Single file | The specification has no issue-local supporting artifacts | `{number}-{short-description}.md`       |
| Folder      | The specification has issue-local artifacts               | `{number}-{short-description}/ISSUE.md` |

Examples:

- `1697-ai-agent-configuration.md`
- `42-add-peer-expiry-grace-period.md`
- `523-internal-linting-tool.md`
- `2022-vendor-and-document-maintainer-merge-workflow/ISSUE.md`
