---
name: add-new-skill
description: "Create or update an Agent Skill in .claude/skills: its folder, frontmatter, a short description and its index row. Use when adding a new skill or changing an existing one."
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

1. **Metadata** (~70 tokens at most): `name` and `description` loaded at startup for all skills
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
See the [Agent Skills specification](https://agentskills.io/specification) for workflow patterns.
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

Create the skill at `.claude/skills/<skill-name>/SKILL.md`, flat: Claude Code discovers a project
skill only there, and a skill one folder deeper is never listed. The folder name must equal the
skill's `name`.

Group the skill by area in the index instead: add one row for it to the fitting area table of
[`.claude/skills/README.md`](../README.md), with a one-line purpose, or add an area if none fits.

Do not add anything under `.github/skills/`: that folder holds only the redirect stubs of the skills
that moved in issue #2866. A new skill needs no stub.

### Step 3: Write the SKILL.md

Frontmatter rules:

- `name`: lowercase letters, numbers, hyphens only; max 64 chars; no consecutive hyphens
- `description`: 200 characters or less, as YAML reads it; see [Description rules](#description-rules)
- `metadata.author`: `torrust`
- `metadata.version`: `"1.0"`

#### Description Rules

Claude Code loads every skill's `name` and `description` into every session, within a budget of 1%
of the context window; over budget, the descriptions of the least-used skills are dropped. It
chooses a skill from those two fields, so the words a task uses must be in them. Issue #2873 set
these rules:

- **Length:** 200 characters or less, counted on the value YAML reads. The open format allows
  1,024; this repository's limit is lower to keep the listing small.
- **Shape:** one sentence on what the skill does, then `Use when ...` naming the tasks. Start with
  a verb or the object, not `Guide for` or `This skill`.
- **Trigger words:** keep the two to four words a task is most likely to contain and that set this
  skill apart from its neighbors. Drop words every skill could claim (`documentation`, `project`,
  `torrust-tracker`) and long `Triggers on "..."` lists.
- **No duplication:** details, steps, warnings and option lists go in the body, which loads only
  when the skill does.
- **Quoting:** put the value in double quotes when it contains `#` or a colon followed by a space,
  or starts with a character YAML treats specially. Unquoted, YAML reads white space followed by
  `#` as the start of a comment and drops the rest.
- **Related skills:** when two skills share trigger words, each description says what sets it
  apart (for example `run-pre-commit-checks` and `run-pre-push-checks`).

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

# Check the skill layout: placement, name, and the index row
cargo run --quiet --package frontmatter-validator --bin frontmatter-validator -- --skill-layout

# Check the description: whole as YAML reads it, 200 characters or less
cargo run --quiet --package frontmatter-validator --bin frontmatter-validator -- --skill-descriptions

# Commit
git add .claude/skills/
git commit -S -m "docs(skills): add {skill-name} skill"
```

## Directory Layout

```text
.claude/skills/
  README.md             ← The index: one row per skill, by area
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
- Skill patterns: [Agent Skills specification](https://agentskills.io/specification)
- Real examples: [Agent Skills specification](https://agentskills.io/specification)
- Semantic link convention: [`docs/skills/semantic-skill-link-convention.md`](../../../docs/skills/semantic-skill-link-convention.md)
