# Agent Skills Specification Reference

This document provides a reference to the Agent Skills specification from [agentskills.io](https://agentskills.io).

## What is Agent Skills?

Agent Skills is an open format for extending AI agent capabilities with specialized knowledge and
workflows. It's vendor-neutral and works with Claude Code, VS Code Copilot, Cursor, and Windsurf.

## Core Concepts

### Progressive Disclosure

```text
Level 1: Metadata (name + description) - ~100 tokens - Loaded at startup for ALL skills
Level 2: SKILL.md body - <5000 tokens - Loaded when skill matches task
Level 3: Bundled resources - On-demand - Loaded only when referenced
```

### Directory Structure

```text
.github/
└── skills/
    └── skill-name/
        ├── SKILL.md          # Required: frontmatter + instructions
        ├── README.md         # Optional: human-readable documentation
        ├── scripts/          # Optional: executable code
        ├── references/       # Optional: detailed docs loaded on-demand
        └── assets/           # Optional: templates, images, data
```

## SKILL.md Format

### Frontmatter (YAML)

```yaml
---
name: skill-name
description: |
  What the skill does and when to use it. Include trigger phrases.
metadata:
  author: torrust
  version: "1.0"
---
```

### Frontmatter Validation Rules

**name**:

- Required; max 64 characters
- Lowercase letters, numbers, hyphens only
- Cannot contain consecutive hyphens or XML tags

**description**:

- Required; max 1024 characters
- Should describe WHAT the skill does AND WHEN to use it
- Include trigger phrases/keywords

## References

- Official spec: <https://agentskills.io/specification>
- GitHub Copilot skills docs: <https://docs.github.com/en/copilot/concepts/agents/about-agent-skills>
