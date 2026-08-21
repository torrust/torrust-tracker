---
semantic-links:
  related-artifacts:
    - AGENTS.md
    - docs/adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md
---

# Repository Agent Profiles

This directory contains repository-defined agent profiles. Each profile's `.agent.md` file is the
authoritative definition of its purpose, workflow, and declared tools. This README is a navigation
catalog only; do not duplicate profile metadata here.

When adding, removing, or renaming a profile, update this link inventory in the same change.
Repository workflow and policy remain authoritative in `AGENTS.md`, `.github/skills/`, tracked
scripts, tests, and documentation. Profiles are optional adapters, as defined by the
[AI agent context, capability, and portability governance ADR](../../docs/adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md).

## Planning and Implementation

- [Planner](planner.agent.md)
- [Implementer](implementer.agent.md)
- [Complexity Auditor](complexity-auditor.agent.md)
- [Task Reviewer](task-reviewer.agent.md)

## Change and Pull Request Workflow

- [Committer](committer.agent.md)
- [PR Reviewer](pr-reviewer.agent.md)
- [Copilot Suggestions Handler](copilot-suggestions-handler.agent.md)

## Research and GitHub Operations

- [Researcher](researcher.agent.md)
- [GitHub Operator](github-operator.agent.md)

## Targeted Maintenance

- [ClippyFixer](clippy-fixer.agent.md)
