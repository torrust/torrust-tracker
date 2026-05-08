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

## Where to Place Markers

Use language-appropriate syntax:

- Rust: `// skill-link: <skill-name>`
- TOML: `# skill-link: <skill-name>`
- Markdown: `<!-- skill-link: <skill-name> -->`

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
