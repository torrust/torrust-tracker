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

| Marker       | Value                  | Meaning                                                                                |
| ------------ | ---------------------- | -------------------------------------------------------------------------------------- |
| `skill-link` | `<skill-name>`         | This artifact affects the linked skill and should trigger a skill review when changed. |
| `issue-spec` | `<repo-relative-path>` | This artifact is affected by a draft issue specification at the given temporary path.  |
| `issue`      | `#<number>`            | This artifact is affected by the GitHub issue with the given number.                   |

Add new markers only when there is a concrete recurring maintenance problem that the current marker set cannot represent.

### Issue-spec lifecycle

Use `issue-spec` only while an issue specification is still a draft. The value must be
the repository-relative path to the draft spec:

```text
issue-spec: docs/issues/drafts/simplify-udp-server-main-loop.md
```

When the draft becomes a GitHub issue, replace every corresponding `issue-spec`
marker with the stable issue-number marker:

```text
issue: #1234
```

Do not retain the draft file path after the issue is created: issue specs move from
`drafts/` to `open/` and later to `closed/`, while the issue number remains stable.

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

> **Stability warning**: Issue spec documents can move between `docs/issues/open/` and
> `docs/issues/closed/` as their state changes. When linking to an issue spec from a
> long-lived artifact like an ADR or workflow, prefer the issue number (`issue #NNNN`)
> over a file path, because the path may become stale after the issue is closed.
>
> For in-flight linking within the same issue (e.g., experiment results linking to the
> issue spec), file paths are acceptable because the artifacts move together.

## Cross-Referencing ADRs

ADRs are long-lived records that outlast individual issues and task branches. They should
be linked bidirectionally to the artifacts they affect.

### From ADR to workflows and issue specs

Use the `semantic-links.related-artifacts` frontmatter list:

```yaml
---
semantic-links:
  skill-links:
    - create-adr
  related-artifacts:
    - issue #1726                    # Use issue number, not file path
    - .github/workflows/testing.yaml # Workflow files are stable paths
    - contrib/dev-tools/experiments/ # Canonical experiment directory
---
```

Guidelines:

- Use `issue #NNNN` for issue specs (paths can change when moved to `closed/`).
- Use repository-relative paths for files that do not move (workflows, config files,
  experiment directories).
- Do **not** duplicate the `related-artifacts` in a body section like `## References` —
  the frontmatter is the canonical source.

### From workflow files to ADR

GitHub Actions YAML workflows do not support YAML frontmatter — the `---` document
separator would create a second YAML document, causing a parse error. The workflow
schema only recognizes documented keys (`name`, `on`, `jobs`, etc.) and rejects
unknown top-level keys.

Use a `# adr:` comment at the top of the file, near the `name:` line:

```yaml
name: Testing

# adr: docs/adrs/20260612000000_adopt_sccache_for_ci_bare_builds.md
# Brief one-liner about what the ADR decided for this workflow.

# Path policy: ...
```

Guidelines:

- Use the full repository-relative path to the ADR (ADRs are never renamed or moved).
- Add a brief comment explaining the relevance.
- Multiple ADR references can be stacked as separate `# adr:` lines.
- Keep links high-signal; avoid noisy or speculative links.
- For issue and EPIC specs, include both metadata and `semantic-links` in frontmatter.

## Where to Place Markers

Use language-appropriate syntax:

- Rust: `// skill-link: <skill-name>`
- TOML: `# skill-link: <skill-name>`
- Markdown: `<!-- skill-link: <skill-name> -->`

Use the same language-appropriate comment syntax for issue references:

- Rust: `// issue-spec: docs/issues/drafts/<name>.md` or `// issue: #<number>`
- TOML: `# issue-spec: docs/issues/drafts/<name>.md` or `# issue: #<number>`
- Markdown: `<!-- issue-spec: docs/issues/drafts/<name>.md -->` or `<!-- issue: #<number> -->`

For Markdown files with frontmatter `semantic-links.skill-links`, top-of-file inline markers are
redundant and need not be added. Inline markers placed near specific workflow-defining sections
within the body remain useful for navigation but are not required when frontmatter links are present.

Place a `skill-link`, `issue-spec`, or `issue` marker near:

- constants that encode default behavior,
- configuration blocks consumed by the workflow,
- documentation sections that define the operational procedure.

For issue references in source code, prefer the declaration of the function, type,
or module whose behavior the issue plans to change. Keep these links high-signal:
do not add a marker merely because a file is mentioned incidentally in an issue.

## Maintenance Workflow

1. Add or update `skill-link`, `issue-spec`, or `issue` markers in touched artifacts.
2. When moving a draft spec to an issue, replace all of its `issue-spec` markers
   with `issue: #<number>` markers.
3. Update the skill instructions if semantics changed.
4. Validate links and markers.

## Ontology-Lite Categories

This repository currently uses these minimal categories:

- Skill: instruction protocol with stable `name`
- Artifact: code, config, or documentation file
- Relation: `skill-link` from artifact to skill
- Validator: script that verifies relation integrity
