---
semantic-links:
  skill-links:
    - create-adr
  related-artifacts:
    - AGENTS.md
    - .github/agents/
    - .github/skills/
    - .github/prompts/
    - .github/workflows/copilot-setup-steps.yml
    - .vscode/
    - docs/adrs/20260420200013_adopt_custom_github_copilot_aligned_agent_framework.md
---

<!-- skill-link: create-adr -->

# Establish AI Agent Context, Capability, and Portability Governance

## Description

ADR `20260420200013_adopt_custom_github_copilot_aligned_agent_framework.md` established the
repository-owned agent framework: `AGENTS.md`, Agent Skills, custom agent profiles, and Copilot
cloud-agent setup. Those artifacts are intentionally Markdown-oriented and portable, but modern
agent environments can also retain project state or provide proprietary profiles, prompts, tools,
indexes, cloud setup, and instruction-discovery behavior outside the Git repository.

If shared repository knowledge or a required workflow exists only in such a facility, contributors
using another agent, model, IDE, or vendor runtime cannot reliably inspect or reproduce it. At the
same time, the repository cannot claim external-runtime behavior that its tracked configuration
does not prove.

## Agreement

This ADR extends ADR `20260420200013` with the following governance rules.

### Authority and terminology

For repository conventions and project decisions, authority is ordered as follows:

1. **Tracked repository knowledge** — Git-tracked documentation, configuration, scripts, tests,
   and standard interfaces are authoritative.
2. **Retained agent state** — session task state, user-local preferences, and runtime-managed
   retained project state are optional, non-authoritative, and disposable.
3. **Vendor/runtime implementation details** — provider-specific behavior is not a repository
   requirement unless its purpose, portability limitation, and practical alternative are tracked.

This hierarchy governs repository-controlled guidance only. It does not override system, security,
legal, platform, or user instructions that govern an agent's execution environment.

A reusable repository convention, decision, workflow, verified project fact, or command that exists
only in retained agent state is undocumented. Promote it to the appropriate tracked artifact before
using retained state as a concise pointer or convenience cache.

### Retained-state rules

| Information type                                                                                       | Required handling                                                                                                                                   |
| ------------------------------------------------------------------------------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------- |
| Shared policy, workflow, convention, architecture decision, verified project fact, or reusable command | Capture or update the appropriate tracked artifact first. Retained state may store only a concise pointer.                                          |
| Temporary task state                                                                                   | Keep it session-scoped or do not persist it.                                                                                                        |
| User-specific working preference                                                                       | Retain it only in user-local state when the runtime supports that state and the preference is safe to retain.                                       |
| Secret, credential, passphrase, token, sensitive personal data, speculation, or unverified fact        | Never retain it in agent memory.                                                                                                                    |
| Temporary environment fact                                                                             | Keep it task-scoped unless it becomes reusable by contributors or relevant beyond the task; then promote a sanitized fact to tracked documentation. |

Existing secret-handling guidance remains authoritative for application secrets and security
reporting. This ADR adds the agent-context retention boundary; it does not duplicate the existing
secret taxonomy.

### Provider-specific adapters

Agent profiles, instruction adapters, skills or custom commands, tool and MCP integrations,
retained context, session histories, semantic indexes, cloud-agent setup, and IDE settings are
optional adapters. They must not be the sole record of a repository workflow, decision, validation
requirement, or project fact.

When a provider-specific adapter is used, document its purpose, canonical tracked workflow or
source, portability risk, practical alternative, review evidence, and limitation. The absence of an
adapter in another runtime must not make repository knowledge or required validation impossible to
discover and reproduce with tracked Markdown, scripts, tests, or documented standard interfaces.

### Capability inventory and evidence

Maintain an evidence-based inventory of the repository's agent-related adapters. Use these states:

- **Tracked**: a repository definition or configuration exists and passes repository documentation
  checks.
- **Reviewed**: a tracked workflow was assessed against a named public runtime or documentation
  source on a stated date.
- **Verified**: a concrete scenario was manually exercised with source/version evidence, result,
  and limitations recorded.

Do not infer an external capability from a profile, prompt, or workflow reference. Record missing or
inaccessible external capability evidence as unavailable or unverified.

The initial tracked inventory is:

| Capability                   | Purpose                                                               | Canonical source                                                                                                                              | Portability risk                                                                              | Practical alternative                                                    | Review evidence                                                                                            | Limitation                                                                                   |
| ---------------------------- | --------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------ | ---------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------- |
| Root and scoped instructions | Provide repository and scoped guidance.                               | Root and scoped `AGENTS.md` files; linked repository docs, scripts, and tests.                                                                | An external runtime may not discover every instruction file or apply the intended precedence. | Navigate the tracked instruction files and linked sources directly.      | Tracked: repository files present on 2026-08-21.                                                           | External discovery and precedence behavior is unverified.                                    |
| Custom profiles              | Package specialised workflows for supported agent runtimes.           | `.github/agents/*.agent.md`; profile bodies point to `AGENTS.md`, skills, scripts, tests, and Git/GitHub interfaces.                          | A runtime may not support profile syntax, declared tools, or subagent behavior.               | Follow the linked Markdown workflows and standard Git/GitHub interfaces. | Tracked: ten profile definitions cataloged on 2026-08-21.                                                  | Fixed model, tool availability, and cross-runtime behavior are unverified.                   |
| Skills                       | Provide repeatable repository procedures.                             | `.github/skills/**/SKILL.md`; procedures remain readable as Markdown and reference repository commands.                                       | A runtime may not discover or automatically invoke skills.                                    | Read `SKILL.md` and run its referenced repository commands.              | Tracked: skill files are version controlled on 2026-08-21.                                                 | Cross-vendor discovery/loading is unverified.                                                |
| Prompt adapter               | Provide a provider-facing shortcut for dependency updates.            | `.github/prompts/update-dependencies.prompt.md` and its referenced dependency-update skill.                                                   | Other runtimes may not discover `.github/prompts/`.                                           | Use the referenced dependency-update skill directly.                     | Tracked: prompt adapter and skill reference reviewed on 2026-08-21.                                        | No cross-runtime prompt-discovery evidence exists.                                           |
| Tool and MCP preference      | Select a structured interface for GitHub operations.                  | `github-operator.agent.md` documents MCP → GitHub CLI → raw API preference.                                                                   | MCP availability or authentication may differ by runtime.                                     | Use GitHub CLI, then documented raw API when necessary.                  | Tracked: preference chain reviewed on 2026-08-21.                                                          | No tracked MCP server, authentication, or capability configuration exists.                   |
| Cloud-agent setup            | Prepare a cloud-agent build and validation environment.               | `.github/workflows/copilot-setup-steps.yml`; its tool installation and checks are tracked commands.                                           | Another provider may not consume the workflow or supply equivalent environment access.        | Reproduce the documented Cargo/tool installation and git-hook commands.  | Tracked: workflow reviewed on 2026-08-21.                                                                  | Cloud-agent consumption, token scope, network, cache, and execution behavior are unverified. |
| IDE settings                 | Provide editor formatting and Rust-check defaults.                    | Tracked `.vscode/settings.json` and `.vscode/extensions.json`; `cargo fmt`, `cargo clippy`, and `linter all` are portable validation sources. | Contributor user settings or another IDE may not apply the same defaults.                     | Run the tracked formatter, linter, and Cargo commands.                   | Tracked: workspace settings reviewed on 2026-08-21.                                                        | User settings and agent-skill discovery behavior are not repository requirements.            |
| Retained state and indexes   | Optionally accelerate an agent without becoming repository knowledge. | Git-tracked documentation, ADRs, skills, tests, and scripts.                                                                                  | Hidden retained state can become an undocumented workflow dependency.                         | Promote reusable knowledge to the appropriate tracked artifact.          | Reviewed: no tracked runtime-memory, session-history, or semantic-index configuration found on 2026-08-21. | Absence of a tracked configuration does not prove a runtime has no retained state.           |

### Review cadence

Review this inventory **each August** and when any of these events occurs:

- a tracked profile, skill, prompt, cloud setup workflow, or repository IDE setting is added, removed,
  or materially changed;
- a provider or runtime migration occurs;
- a portability failure is documented; or
- an adapter's capability, permission, or authentication boundary materially changes.

A review record must include the configuration checked, source/version evidence where available,
scenario, result, limitations, date, and evidence state. Record unavailable evidence explicitly;
do not replace it with a guess.

### Initial review record

| Date       | Configuration                     | Evidence state | Source/version evidence                                                                         | Scenario                                                                                                                             | Result                                                                                                                                                                            | Limitation                                                                                                                                  |
| ---------- | --------------------------------- | -------------- | ----------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------- |
| 2026-08-21 | Tracked repository agent adapters | Reviewed       | Repository files listed in the initial inventory; no external runtime/version source available. | Inspect tracked profiles, skills, prompts, cloud setup, IDE settings, and configuration for retained state, indexes, or MCP servers. | The inventory records all observed adapters and their portable sources; no tracked runtime-memory, semantic-index, MCP-server, or external compatibility configuration was found. | This does not verify external instruction discovery, model availability, retained-state behavior, MCP capability, or cloud-agent execution. |

## Alternatives Considered

### Let agent-local memory define project conventions

Not adopted. It hides reusable knowledge in provider-specific retained state and prevents other
contributors from reviewing or reproducing it.

### Require a provider-neutral replacement for every adapter immediately

Not adopted. Profiles, skills, and cloud setup provide value today. This ADR requires a documented
portable source or practical alternative and creates follow-up work for high-risk dependencies
instead of mandating a speculative replacement project.

### Add a dedicated memory-maintenance skill now

Not adopted. The current policy is an always-on repository invariant. No concrete recurring,
fragile, on-demand procedure has been demonstrated beyond normal documentation maintenance.
Reconsider a skill only when such a workflow is evidenced.

## Consequences

- Contributors can inspect the canonical record of repository knowledge and workflows in Git.
- New provider-specific adapters require explicit portability documentation rather than becoming
  hidden dependencies.
- Compatibility claims remain evidence-bounded and may include unavailable/unverified states.
- Maintaining the inventory adds documentation work during the annual and event-driven reviews.
- This ADR does not guarantee behavior of any external agent, model, IDE, MCP implementation, or
  memory backend.

## Date

2026-08-21

## References

- Issue: #2075
- ADR: `20260420200013_adopt_custom_github_copilot_aligned_agent_framework.md`
- Root instructions: `AGENTS.md`
- Agent catalog: `.github/agents/README.md`
- Agent profiles: `.github/agents/`
- Skills: `.github/skills/`
- Prompt adapters: `.github/prompts/`
- Cloud setup: `.github/workflows/copilot-setup-steps.yml`
- IDE settings: `.vscode/`
- Secret handling: `.github/skills/dev/rust-code-quality/handle-secrets/SKILL.md`
