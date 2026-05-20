---
semantic-links:
  skill-links:
    - write-markdown-docs
  related-artifacts:
    - docs/AGENTS.md
    - docs/index.md
---

# Semantic Skill Link Convention

## Purpose

Define a lightweight, machine-readable convention to couple Agent Skills and repository artifacts.

This convention is intentionally minimal. It is designed to prevent skill drift without introducing a heavy ontology framework.

## Marker Catalog

The repository keeps a small catalog of marker definitions.

Current markers:

| Marker       | Value          | Meaning                                                                                |
| ------------ | -------------- | -------------------------------------------------------------------------------------- |
| `skill-link` | `<skill-name>` | This artifact affects the linked skill and should trigger a skill review when changed. |

Add new markers only when there is a concrete recurring maintenance problem that the current marker set cannot represent.

## Marker Format

Use this marker in comments or documentation text close to behavior-defining lines:

```text
skill-link: <skill-name>
```

Rules:

- `skill-name` must match the skill frontmatter `name` value.
- Use lowercase letters, numbers, and hyphens.
- Add only high-signal links: artifacts that can make a skill stale when they change.

## Markdown Frontmatter (Required for New or Updated Issue and EPIC Specs)

For new or updated issue and EPIC specification documents, YAML frontmatter is the canonical
metadata source. Existing specs may be migrated incrementally as they are touched.

Use frontmatter to keep machine-readable metadata and semantic links queryable and consistent.

For other Markdown artifacts, frontmatter remains optional but recommended.

Required metadata fields for issue specs:

```yaml
doc-type: issue
issue-type: <task|bug|feature|enhancement>
status: <draft|planned|in-progress|blocked|in-review|done>
priority: <p0|p1|p2|p3>
github-issue: <number|null>
spec-path: <repo-relative-path>
branch: <branch-name>
related-pr: <number|null>
last-updated-utc: YYYY-MM-DD HH:MM
```

Required metadata fields for EPIC specs:

```yaml
doc-type: epic
status: <draft|planned|in-progress|blocked|in-review|done>
github-issue: <number|null>
spec-path: <repo-relative-path>
epic-owner: <owner|null>
last-updated-utc: YYYY-MM-DD HH:MM
```

When frontmatter metadata is present, do not duplicate it in a body section like `## Metadata`.

Recommended shape:

```yaml
---
semantic-links:
  skill-links:
    - <skill-name>
  related-artifacts:
    - <repo-relative-path>
---
```

Guidance:

- For Markdown files with frontmatter `semantic-links.skill-links`, the frontmatter is the
  canonical source; inline `<!-- skill-link: ... -->` top-of-file markers are redundant and need
  not be added.
- For non-Markdown artifacts and Markdown files without frontmatter, inline markers remain the
  primary convention.
- Use frontmatter to express richer relations (for example bidirectional links).
- Keep paths repository-relative and stable.
- Keep links high-signal; avoid noisy or speculative links.
- For issue and EPIC specs, include both metadata and `semantic-links` in frontmatter.

## Where to Place Markers

Use language-appropriate syntax:

- Rust: `// skill-link: <skill-name>`
- TOML: `# skill-link: <skill-name>`
- Markdown: `<!-- skill-link: <skill-name> -->`

For Markdown files with frontmatter `semantic-links.skill-links`, top-of-file inline markers are
redundant and need not be added. Inline markers placed near specific workflow-defining sections
within the body remain useful for navigation but are not required when frontmatter links are present.

Place the marker near:

- constants that encode default behavior,
- configuration blocks consumed by the workflow,
- documentation sections that define the operational procedure.

## Maintenance Workflow

1. Add or update `skill-link` markers in touched artifacts.
2. Update the skill instructions if semantics changed.
3. Validate links and markers.

## Ontology-Lite Categories

This repository currently uses these minimal categories:

- Skill: instruction protocol with stable `name`
- Artifact: code, config, or documentation file
- Relation: `skill-link` from artifact to skill
- Validator: script that verifies relation integrity
