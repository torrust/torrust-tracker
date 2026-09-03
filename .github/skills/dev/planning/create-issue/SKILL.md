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
      - docs/templates/IMPLEMENTATION-RETROSPECTIVE.md
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

1. **Draft a folder-style specification** in `docs/issues/drafts/` using the repository templates
   appropriate to the issue type (`docs/templates/ISSUE.md` for Task/Bug/Feature,
   `docs/templates/EPIC.md` for Epic). The template filenames are uppercase, but every concrete
   specification filename is lowercase: `issue.md` or `epic.md`.
2. **User reviews** the draft specification
3. **Create GitHub issue**
4. **Move the spec directory to `docs/issues/open/`** and include the issue number
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
docs/issues/drafts/{epic-issue-number}-{short-description}/issue.md
```

This prefix identifies the known parent EPIC; it is not a placeholder for the future subissue's
own GitHub issue number. Do not infer a parent EPIC from related issues, ADRs, or topic overlap.
If the parent is not explicitly established, use an unnumbered descriptive draft folder:

```bash
mkdir -p docs/issues/drafts/{short-description}
touch docs/issues/drafts/{short-description}/issue.md
```

All new specifications use a folder-style layout. It keeps issue-local artifacts, such as an
immutable source snapshot, evidence, design input, or an implementation retrospective with the
main specification. Place the main specification in lowercase `issue.md`, or `epic.md` for an
EPIC:

```bash
mkdir -p docs/issues/drafts/{short-description}
touch docs/issues/drafts/{short-description}/issue.md
```

For an EPIC, use:

```bash
mkdir -p docs/issues/drafts/{short-description}
touch docs/issues/drafts/{short-description}/epic.md
```

For a known parent EPIC, apply the same prefix to a folder-style draft:

```bash
mkdir -p docs/issues/drafts/{epic-issue-number}-{short-description}
touch docs/issues/drafts/{epic-issue-number}-{short-description}/issue.md
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
- `Implementation Completion Review`, with the conditions for creating an
  issue-local `implementation-retrospective.md` or recording why one is
  unnecessary

The draft must also include a verification policy that is explicit and enforceable:

- Automatic checks to run after implementation (`linter all`, relevant tests, pre-push checks when applicable)
- Manual verification scenarios with status + evidence tracking (mandatory)
- A post-implementation acceptance criteria review step
- An evidence-based implementation completion review that records reusable
  lessons, material design changes, or deviations from the plan. Use
  `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` when a separate
  retrospective is warranted; otherwise require a concise progress-log entry
  explaining why it is not.

For work involving child processes, asynchronous I/O, network readiness,
resource cleanup, or reusable test fixtures, the draft must additionally define:

- a responsibility and ownership map;
- normal and failure/drop-path resource lifetime invariants;
- absolute deadline coverage for every awaited readiness operation; and
- a design-review checkpoint after the first passing vertical slice.

During implementation, create an ADR when an important architectural decision
emerges, even if the issue draft did not anticipate it. Link the ADR from the
issue specification and update the architectural-decisions section. For each
planned ADR, identify its expected root or package-local collection by decision
scope: use `docs/adrs/` for repository-wide, multi-package, and inter-package
decisions, and `packages/<package>/docs/adrs/` only for decisions owned solely
by an extractable package. Do not choose placement only from the implementation
paths expected to change.

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

Move the folder-style specification from `drafts/` to `open/` using its assigned issue number:

```bash
git mv docs/issues/drafts/{short-description} \
  docs/issues/open/{number}-{short-description}
```

For a subissue of a known EPIC, replace the draft's parent-only prefix with both GitHub issue
numbers:

```bash
git mv docs/issues/drafts/{epic-issue-number}-{short-description} \
  docs/issues/open/{number}-{epic-issue-number}-{short-description}
```

For folder-style specifications, the main document is
`docs/issues/open/{number}-{short-description}/issue.md`, or `epic.md` for an EPIC. Keep all
issue-local artifacts in the same directory. Update the `spec-path` and all internal artifact
references after the move.

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
2. Name the branch `{issue-number}-{short-description}-spec`; reserve the base
   `{issue-number}-{short-description}` name for the later implementation branch. Set the issue
   specification frontmatter `branch:` value to this same `-spec` branch name.
3. Commit only spec changes (`docs/issues/`, and if needed templates/skills)
4. Push branch to your fork remote (for example `josecelano`)
5. Open PR in the **upstream repository** (`torrust/torrust-tracker`) targeting `develop`
6. If using fork-based workflow, set head as `{fork-owner}:{branch}` (for example
   `josecelano:1771-spec-first-pr-workflow-spec`)
7. Do not open the PR in the fork repository unless explicitly requested
8. Merge PR after review
9. Start implementation work in the reserved base branch and open a separate implementation PR

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
5. **Implementation completion review**: require an evidence-based review after
   implementation. Create an issue-local retrospective for reusable lessons,
   material design changes, or meaningful deviations from the plan; otherwise
   record why none was needed in the issue progress log.

Do not treat an issue as complete only because automated tests pass; manual validation is required.

## Naming Convention

Use one of these layouts:

| Layout      | Status                                                                                  | Main specification path                 |
| ----------- | --------------------------------------------------------------------------------------- | --------------------------------------- |
| Folder      | Required for all new specifications                                                     | `{number}-{short-description}/issue.md` |
| Single file | Legacy only; migrate when materially updating it or when adding an issue-local artifact | `{number}-{short-description}.md`       |

### Migrating a Legacy Specification

Migrate a legacy single-file specification, or an older folder whose primary
file is uppercase, before adding an issue-local artifact or when materially
updating its planning or completion-review content. Do not migrate unrelated
legacy specifications opportunistically.

1. Move the existing primary document into a folder with its current issue
   prefix and lowercase primary filename: `issue.md` or `epic.md`.
2. Update the moved document's `spec-path`, `semantic-links.related-artifacts`,
   and relative links to issue-local documents.
3. Search for live references to the former path and repair them. Retain paths
   in immutable historical records only when they accurately describe the path
   at that time.
4. Add any new issue-local artifact after the move, then validate Markdown
   links and frontmatter.

For example, migrate an issue specification with:

```bash
mkdir docs/issues/open/42-short-description
git mv docs/issues/open/42-short-description.md \
  docs/issues/open/42-short-description/issue.md
```

Examples:

- `1697-ai-agent-configuration/issue.md`
- `42-add-peer-expiry-grace-period/issue.md`
- `523-internal-linting-tool/issue.md`
- `2022-vendor-and-document-maintainer-merge-workflow/issue.md`
- `1669-overhaul-packages/epic.md`
