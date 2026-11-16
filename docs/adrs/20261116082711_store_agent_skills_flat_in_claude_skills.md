---
semantic-links:
  skill-links:
    - create-adr
    - add-new-skill
  related-artifacts:
    - .claude/skills
    - .claude/skills/README.md
    - .github/skills/README.md
    - contrib/dev-tools/checks/frontmatter-validator/src/skills.rs
    - docs/adrs/20260420200013_adopt_custom_github_copilot_aligned_agent_framework.md
    - docs/adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md
    - docs/issues/open/2866-2003-move-skills-to-claude-skills/ISSUE.md
---

<!-- skill-link: create-adr -->

# Store Agent Skills Flat in `.claude/skills/`

## Scope

Repository-level, in `docs/adrs/`: the skills serve every package and every contributor's agent, and
the decision sets where the whole repository keeps them.

## Description

ADR [`20260420200013`](20260420200013_adopt_custom_github_copilot_aligned_agent_framework.md) put
the Agent Skills under `.github/skills/`, the folder GitHub Copilot reads, grouped by area as
`.github/skills/dev/<area>/<name>/SKILL.md`. The repository has since moved to Claude Code as its
only AI harness.

Claude Code discovers project skills only at `.claude/skills/<name>/SKILL.md`. It never listed the
repository's skills, so an agent followed a skill only when a brief named its file, and steps were
skipped. A fresh clone of `develop` at `abd7a10ff` showed no project skill in `/context`, and
`/create-issue` did not run as a command (issue #2866, evidence V0).

A folder level between `skills/` and the skill folder is not discovered either. With Claude Code
2.1.318, a scratch repository holding `.claude/skills/flat-probe/SKILL.md` and
`.claude/skills/dev/nested-probe/SKILL.md` listed only `flat-probe`; `/nested-probe` and
`/dev:nested-probe` were unknown commands. So the area folders cannot move as they are.

## Agreement

- Each skill lives at `.claude/skills/<name>/SKILL.md`, one flat folder per skill, with its
  `scripts/`, `references/` and other files inside it. The folder name equals the skill's `name`.
- The grouping by area moves to an index, `.claude/skills/README.md`, with one table row per
  skill.
- Each `SKILL.md` path that existed under `.github/skills/` before the move keeps a redirect stub:
  no frontmatter, one link to the moved skill. `.github/skills/README.md` inventories those 53
  paths. Historical records keep their old paths, which the stubs resolve; stubs and inventory
  rows are never deleted. A renamed skill's stub links to the new name, and a removed skill's
  stub says so.
- `frontmatter-validator --skill-layout` enforces this on every commit and in the Docs Lint
  workflow: every skill is at the discovered place, the stubs match their inventory and lead to
  their skills, each skill's `name` is its folder name, and the index lists every skill exactly
  once.
- `AGENTS.md` stays the root instruction file, as an open standard; there is no repository
  `CLAUDE.md`.

This supersedes the skills location of ADR `20260420200013`; its other decisions stand.

### Alternatives Considered

- **Keep `.github/skills/` and add one symlink per skill under `.claude/skills/`.** Rejected: a
  second place to keep in step, and symbolic links are fragile on Windows.
- **One Claude Code plugin per area.** Rejected: every contributor would need an install step.
- **Keep the area folders under `.claude/skills/`.** Not possible: Claude Code does not discover
  a skill one folder deeper.
- **Rewrite every old path in historical records.** Rejected: about 520 records, such as closed
  specs and pull request review audits, would change for no behavior; the stubs keep them
  navigable.

### Consequences

- Claude Code lists the repository's skills and runs them by name.
- Each skill's name and description are loaded into every session. The listing budget is 1% of the
  context window, and the 53 descriptions already exceed it on a 1M window, so the least-used
  descriptions are dropped. A follow-up sub-issue of EPIC #2003 shortens them.
- GitHub Copilot compatibility is not kept.
- The 53 stubs stay as long as historical records name the old paths.
- `linter lychee` (`torrust-linting` 0.2.0) checks `**/*.md` and `.github/**/*.md`, so links
  inside `.claude/skills/` are not link-checked until that tool also covers `.claude/`.

## Date

2026-11-16

## References

- Issue: #2866 (`docs/issues/open/2866-2003-move-skills-to-claude-skills/ISSUE.md`), EPIC #2003
- Partly superseded ADR:
  [`20260420200013`](20260420200013_adopt_custom_github_copilot_aligned_agent_framework.md)
- Capability inventory:
  [`20260821172000`](20260821172000_establish_ai_agent_context_capability_and_portability_governance.md)
- Skills index: [`.claude/skills/README.md`](../../.claude/skills/README.md)
- Stub inventory: [`.github/skills/README.md`](../../.github/skills/README.md)
- Claude Code skills documentation: <https://code.claude.com/docs/en/skills>
