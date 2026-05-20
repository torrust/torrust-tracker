---
semantic-links:
  skill-links:
    - create-adr
  related-artifacts:
    - .github/skills/dev/planning/create-adr/SKILL.md
    - AGENTS.md
    - .github/skills/
    - .github/agents/
---

# Adopt a Custom, GitHub-Copilot-Aligned Agent Framework

## Description

As AI coding agents become a more common part of the development workflow, the project needs a
clear strategy for how agents should interact with the codebase. Several third-party "agent
frameworks" exist that promise to give agents structure and purpose, but they each come with
trade-offs that may not fit the tracker's needs.

This ADR records the decision to build a lightweight, first-party agent framework using the
open standards that GitHub Copilot already supports natively: `AGENTS.md`, Agent Skills, and
Custom Agent profiles.

## Agreement

We adopt a custom, GitHub-Copilot-aligned agent framework consisting of:

- **`AGENTS.md`** at the repository root (and in key subdirectories) — following the
  [agents.md](https://agents.md/) open standard stewarded by the Agentic AI Foundation under the
  Linux Foundation. Provides AI coding agents with project context, build steps, test commands,
  conventions, and essential rules.
- **Agent Skills** under `.github/skills/` — following the
  [Agent Skills specification](https://agentskills.io/specification). Each skill is a directory
  containing a `SKILL.md` file with YAML frontmatter and Markdown instructions, covering
  repeatable tasks such as committing changes, running linters, creating ADRs, or setting up the
  development environment.
- **Custom Agent profiles** under `.github/agents/` — Markdown files with YAML frontmatter
  defining specialised Copilot agents (e.g. `committer`, `implementer`, `complexity-auditor`)
  that can be invoked directly or as subagents.
- **`copilot-setup-steps.yml`** workflow — prepares the GitHub Copilot cloud agent environment
  before it starts working on any task.

### Alternatives Considered

**[obra/superpowers](https://github.com/obra/superpowers)**

A framework that adds "superpowers" to coding agents through a set of conventions and tools.
Not adopted for the following reasons:

1. **Complexity mismatch** — introduces abstractions heavier than what tracker development needs.
1. **Precision requirements** — the tracker involves low-level Rust programming where agent work
   must be reviewed carefully; generic productivity frameworks are not designed for that
   constraint.
1. **Tooling churn risk** — depending on a third-party framework risks forced refactoring if
   that framework is deprecated or pivots.

**[gsd-build/get-shit-done](https://github.com/gsd-build/get-shit-done)**

A productivity-oriented agent framework with opinionated workflows.
Not adopted for the same reasons as above, plus:

1. **GitHub-first ecosystem** — the tracker is hosted on GitHub and makes intensive use of
   GitHub resources (Actions, Copilot, MCP tools). Staying aligned with GitHub Copilot avoids
   unnecessary integration friction.

### Why the Custom Approach

1. **Tailored fit** — shaped precisely to Torrust conventions, commit style, linting gates, and
   package structure from day one.
1. **Proven in practice** — the same approach has already been validated during the development
   of `torrust-tracker-deployer`.
1. **Agent-agnostic by design** — expressed as plain Markdown files (`AGENTS.md`, `SKILL.md`,
   agent profiles), decoupled from any single agent product. Migration or multi-agent use is
   straightforward.
1. **Incremental adoption** — individual skills, custom agents, or patterns from evaluated
   frameworks can still be cherry-picked and integrated progressively if specific value is
   identified.
1. **Stability** — a first-party approach is more stable than depending on a third-party
   framework whose roadmap we do not control.

## Date

2026-04-20

## References

- Issue: https://github.com/torrust/torrust-tracker/issues/1697
- PR: https://github.com/torrust/torrust-tracker/pull/1699
- AGENTS.md specification: https://agents.md/
- Agent Skills specification: https://agentskills.io/specification
- GitHub Copilot — About agent skills: https://docs.github.com/en/copilot/concepts/agents/about-agent-skills
- GitHub Copilot — About custom agents: https://docs.github.com/en/copilot/concepts/agents/copilot-cli/about-custom-agents
- Customize the Copilot cloud agent environment: https://docs.github.com/en/copilot/how-tos/use-copilot-agents/cloud-agent/customize-the-agent-environment
- obra/superpowers: https://github.com/obra/superpowers
- gsd-build/get-shit-done: https://github.com/gsd-build/get-shit-done
- torrust-tracker-deployer (validated reference implementation): https://github.com/torrust/torrust-tracker-deployer
