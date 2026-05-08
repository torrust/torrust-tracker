---
name: add-new-skill
description: Guide for creating effective Agent Skills for the torrust-tracker project. Use when you need to create a new skill (or update an existing skill) that extends AI agent capabilities with specialized knowledge, workflows, or tool integrations. Triggers on "create skill", "add new skill", "how to add skill", or "skill creation".
metadata:
  author: torrust
  version: "1.0"
---

# Creating New Agent Skills

This skill guides you through creating effective Agent Skills for the Torrust Tracker project.

## About Skills

**What are Agent Skills?**

Agent Skills are specialized instruction sets that extend AI agent capabilities with domain-specific
knowledge, workflows, and tool integrations. They follow the [agentskills.io](https://agentskills.io)
open format and work with multiple AI coding agents (Claude Code, VS Code Copilot, Cursor, Windsurf).

### Progressive Disclosure

Skills use a three-level loading strategy to minimize context window usage:

1. **Metadata** (~100 tokens): `name` and `description` loaded at startup for all skills
2. **SKILL.md Body** (<5000 tokens): Loaded when a task matches the skill's description
3. **Bundled Resources**: Loaded on-demand only when referenced (scripts, references, assets)

### When to Create a Skill vs Updating AGENTS.md

| Use AGENTS.md for...            | Use Skills for...               |
| ------------------------------- | ------------------------------- |
| Always-on rules and constraints | On-demand workflows             |
| "Always do X, never do Y"       | Multi-step repeatable processes |
| Baseline conventions            | Specialist domain knowledge     |
| Rarely changes                  | Can be added/refined frequently |

**Example**: "Use lowercase for skill filenames" → AGENTS.md rule.
"How to run pre-commit checks" → Skill.

## Core Principles

### 1. Concise is Key

**Context window is shared** between system prompt, conversation history, other skills,
and your actual request. Only add context the agent doesn't already have.

### 2. Set Appropriate Degrees of Freedom

Match specificity to task fragility:

- **High freedom** (text-based instructions): multiple approaches valid, context-dependent
- **Medium freedom** (pseudocode): preferred pattern exists, some variation acceptable
- **Low freedom** (specific scripts): operations are fragile, sequence must be followed

### 3. Anatomy of a Skill

A skill consists of:

- **SKILL.md**: Frontmatter (metadata) + body (instructions)
- **Optional bundled resources**: `scripts/`, `references/`, `assets/`

Keep SKILL.md concise (<500 lines). Move detailed content to reference files.

### 4. Progressive Disclosure

Split detailed content into reference files loaded on-demand:

```markdown
## Advanced Features

See [specification.md](references/specification.md) for Agent Skills spec.
See [patterns.md](references/patterns.md) for workflow patterns.
```

### 5. Content Strategy

- **Include in SKILL.md**: essential commands and step-by-step workflows
- **Put in `references/`**: detailed descriptions, config options, troubleshooting
- **Link to official docs**: architecture docs, ADRs, contributing guides

## Skill Creation Process

### Step 1: Plan the Skill

Answer:

- What specific queries should trigger this skill?
- What tasks does it help accomplish?
- Does a similar skill already exist?

### Step 2: Choose the Location

Follow the directory layout:

```text
.github/skills/
  add-new-skill/
  dev/
    git-workflow/
    maintenance/
    planning/
    rust-code-quality/
    testing/
```

### Step 3: Write the SKILL.md

Frontmatter rules:

- `name`: lowercase letters, numbers, hyphens only; max 64 chars; no consecutive hyphens
- `description`: max 1024 chars; include trigger phrases; describe WHAT and WHEN
- `metadata.author`: `torrust`
- `metadata.version`: `"1.0"`

Semantic coupling rules:

- Identify critical project artifacts that the skill depends on.
- Add a `skill-link: <skill-name>` marker in each linked artifact using language-appropriate comments.
- Add a short "Skill Links" section in `SKILL.md` listing those artifacts.
- Prefer a small validation script in `scripts/` to verify linked files and markers.
- Follow the canonical convention in `docs/skills/semantic-skill-link-convention.md`.
- Keep marker usage aligned with the marker catalog in `docs/skills/semantic-skill-link-convention.md`.

### Step 4: Validate and Commit

```bash
# Check spelling and markdown
linter cspell
linter markdown

# Run all linters
linter all

# Commit
git add .github/skills/
git commit -S -m "docs(skills): add {skill-name} skill"
```

## Directory Layout

```text
.github/skills/
  <skill-name>/
    SKILL.md            ← Required
    references/         ← Optional: detailed docs
    scripts/            ← Optional: executable scripts
    assets/             ← Optional: templates, data
```

## Skill Link Convention

Use a lightweight marker convention for cross-artifact maintenance links:

- Marker format: `skill-link: <skill-name>`
- Put markers near constants, configuration blocks, or documentation lines that define behavior used by the skill.
- Keep links minimal and high signal: only link artifacts that can make the skill stale when they change.
- Validate links with a script when practical.

## References

- Agent Skills specification: [references/specification.md](references/specification.md)
- Skill patterns: [references/patterns.md](references/patterns.md)
- Real examples: [references/examples.md](references/examples.md)
- Semantic link convention: [`docs/skills/semantic-skill-link-convention.md`](../../../docs/skills/semantic-skill-link-convention.md)
