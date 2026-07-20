---
doc-type: issue
issue-type: task
status: in-progress
priority: p2
github-issue: 1803
spec-path: docs/issues/open/1803-improve-docs-folder-navigation.md
branch: "1803-improve-docs-folder-navigation"
related-pr: null
last-updated-utc: 2026-05-20 12:00
semantic-links:
  skill-links:
    - create-issue
    - write-markdown-docs
  related-artifacts:
    - docs/index.md
    - docs/AGENTS.md
    - .github/skills/dev/planning/write-markdown-docs/SKILL.md
    - docs/skills/semantic-skill-link-convention.md
    - .markdownlint.json
---


# Issue #1803 - Improve `docs/` folder navigation

## Goal

Make the `docs/` folder easier to navigate for both human readers and AI agents by expanding
the existing `docs/index.md` with structured sections and descriptions, adding a
`docs/AGENTS.md` with AI-agent guidance, and updating the `write-markdown-docs` skill with
missing rules about frontmatter and GitHub-vs-repo markdown.

## Background

The current `docs/index.md` is a minimal flat list of links with no descriptions and no
entries for several subdirectories (`adrs/`, `refactor-plans/`, `pr-reviews/`, `skills/`,
`templates/`, `licenses/`, `media/`). A reader cannot tell from the index alone what each
section covers or where to look for a specific type of artifact.

There is also no `docs/AGENTS.md` file, while the project already has directory-scoped
`AGENTS.md` files for `packages/` and `src/`. Without one, AI agents asked to write, find, or
update documentation artifacts must infer the correct subdirectory and conventions from
context instead of having explicit guidance.

Finally, the `write-markdown-docs` skill does not mention:

- The frontmatter convention described in `docs/skills/semantic-skill-link-convention.md`.
- The difference between repo Markdown files (linted by `.markdownlint.json`) and GitHub
  Markdown surfaces (issues, PRs) that are not subject to the repo lint configuration.

## Scope

### In Scope

- Expand `docs/index.md` with organized sections, short descriptions for each entry, and
  links to all subdirectories currently missing from the index.
- Create `docs/AGENTS.md` covering: directory map, frontmatter convention, Markdown linting
  rules (repo files vs. GitHub surfaces), and a reference to the `write-markdown-docs` skill.
- Update `.github/skills/dev/planning/write-markdown-docs/SKILL.md` to add a frontmatter
  section and a section distinguishing repo Markdown from GitHub Markdown.

### Out of Scope

- Restructuring or renaming any subdirectory under `docs/`.
- Writing or updating content of individual documentation files.
- Changing the `.markdownlint.json` configuration.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                  | Notes / Expected Output                                                                                    |
| --- | ------ | ----------------------------------------------------- | ---------------------------------------------------------------------------------------------------------- |
| T1  | DONE   | Expand `docs/index.md` with sections and descriptions | Organized sections with one-line descriptions for every entry, including previously missing subdirectories |
| T2  | DONE   | Create `docs/AGENTS.md`                               | Directory map, frontmatter rules, linting scope (repo vs. GitHub), skill reference                         |
| T3  | DONE   | Update `write-markdown-docs` skill                    | New "Frontmatter" section and new "Repo Markdown vs. GitHub Markdown" section (see proposed content below) |

### Proposed additions to `write-markdown-docs` skill (T3)

Add the following two sections before the existing "Checklist Before Committing Docs" section:

```markdown
## Frontmatter

All Markdown files in `docs/` should include YAML frontmatter.

It is **required** for issue specs and EPIC specs. It is **recommended** for all other
`.md` files in the repository.

Follow the frontmatter convention defined in
[`docs/skills/semantic-skill-link-convention.md`](../../../../docs/skills/semantic-skill-link-convention.md),
which specifies the required fields for each document type and the shape of
`semantic-links` entries.

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
```

Also add a frontmatter item to the existing checklist:

```markdown
- [ ] Frontmatter is present and follows `docs/skills/semantic-skill-link-convention.md`
```

## Progress Tracking

### Workflow Checkpoints

- [ ] Spec drafted in `docs/issues/drafts/`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] GitHub issue created and issue number added to this spec
- [x] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded (status + evidence)
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [x] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-05-20 10:00 UTC - Agent - Spec drafted based on user discussion
- 2026-05-20 12:00 UTC - Agent - T1, T2, T3 implemented and committed; spec updated to DONE

## Acceptance Criteria

- [ ] AC1: `docs/index.md` contains a section for every subdirectory and top-level file in
      `docs/`, each with a short description of its purpose.
- [ ] AC2: `docs/AGENTS.md` exists and covers the directory map, frontmatter convention,
      Markdown linting scope distinction (repo files vs. GitHub surfaces), and a reference to the
      `write-markdown-docs` skill.
- [ ] AC3: `write-markdown-docs` skill includes a "Frontmatter" section referencing
      `docs/skills/semantic-skill-link-convention.md` and a section explaining that GitHub
      Markdown surfaces (issues, PRs) are not subject to `.markdownlint.json` rules, and that
      line wrapping must not be applied to issue or PR body text.
- [ ] AC4: `linter all` exits with code `0`.
- [ ] AC5: Manual verification scenarios are executed and documented (status + evidence).
- [ ] AC6: Acceptance criteria are re-reviewed after implementation and reflect actual
      behavior.
- [ ] AC7: Documentation is updated when behavior or workflow changes.

## Verification Plan

### Automatic Checks

- `linter all`
- `linter markdown` (targeted check on changed `.md` files)
- `linter cspell`

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                            | Command/Steps                                                                                | Expected Result                                                                                                      | Status | Evidence |
| --- | ----------------------------------- | -------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------- | ------ | -------- |
| M1  | Index covers all subdirectories     | Open `docs/index.md` and compare against `ls docs/`                                          | Every subdirectory and top-level file has an entry with a description                                                | TODO   | —        |
| M2  | AGENTS.md guides an agent correctly | Ask an AI agent "where should I put a new ADR?" and inspect whether it uses `docs/AGENTS.md` | Agent responds with `docs/adrs/` and cites the naming convention                                                     | TODO   | —        |
| M3  | Skill covers frontmatter rule       | Read `write-markdown-docs` skill and verify frontmatter section is present                   | Section exists and references `docs/skills/semantic-skill-link-convention.md`                                        | TODO   | —        |
| M4  | Skill covers GitHub markdown rule   | Read `write-markdown-docs` skill and verify GitHub markdown section is present               | Section states that `.markdownlint.json` does not apply to GitHub issues/PRs and that line wrapping must not be used | TODO   | —        |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | TODO                   | —        |
| AC2   | TODO                   | —        |
| AC3   | TODO                   | —        |
| AC4   | TODO                   | —        |
| AC5   | TODO                   | —        |
| AC6   | TODO                   | —        |
| AC7   | TODO                   | —        |

## Risks and Trade-offs

- `docs/AGENTS.md` may need updating whenever new subdirectories are added to `docs/`. This
  is low risk since changes are infrequent and the file is small.

## References

- Current index: [`docs/index.md`](../../index.md)
- Frontmatter convention: [`docs/skills/semantic-skill-link-convention.md`](../../skills/semantic-skill-link-convention.md)
- Markdown linting configuration: [`.markdownlint.json`](../../../.markdownlint.json)
- Write markdown docs skill: [`.github/skills/dev/planning/write-markdown-docs/SKILL.md`](../../../.github/skills/dev/planning/write-markdown-docs/SKILL.md)
- Existing `packages/AGENTS.md` (pattern reference): [`packages/AGENTS.md`](../../../packages/AGENTS.md)
- Existing `src/AGENTS.md` (pattern reference): [`src/AGENTS.md`](../../../src/AGENTS.md)
