---
doc-type: issue
issue-type: task
status: open
priority: p2
epic: null
github-issue: 2075
spec-path: docs/issues/open/2075-ai-agent-context-capability-and-portability-governance.md
branch: "2075-ai-agent-context-capability-and-portability-governance"
related-pr: null
last-updated-utc: 2026-08-21 17:25
semantic-links:
  skill-links:
    - create-issue
    - create-adr
    - write-markdown-docs
  related-artifacts:
    - AGENTS.md
    - docs/index.md
    - docs/AGENTS.md
    - docs/adrs/20260420200013_adopt_custom_github_copilot_aligned_agent_framework.md
    - docs/skills/semantic-skill-link-convention.md
    - .github/agents/
    - .github/skills/add-new-skill/SKILL.md
    - .github/skills/dev/rust-code-quality/handle-secrets/SKILL.md
    - .github/workflows/copilot-setup-steps.yml
    - .vscode/
---

<!-- skill-link: create-issue -->
<!-- skill-link: create-adr -->
<!-- skill-link: write-markdown-docs -->

# Issue #2075 - Establish AI Agent Context, Capability, and Portability Governance

## Goal

Establish a repository-wide governance policy ensuring that repository conventions, decisions, and
agent-assisted workflows remain visible, version-controlled, reviewable, and portable across AI
agents, models, IDEs, and vendor runtimes.

## Background

Some AI-agent environments retain context or memory outside the Git repository. Such retention can
be useful as a convenience cache, but it creates a collaboration risk when an agent retains project
knowledge that other contributors, agent profiles, or runtimes cannot inspect. Other provider
facilities can create the same risk: proprietary agent profiles, instruction discovery/precedence,
skills or custom commands, tool and MCP integrations, semantic indexes, session histories,
cloud-agent setup workflows, and undocumented IDE settings.

The repository already adopted a custom GitHub-Copilot-aligned agent framework in ADR
`20260420200013_adopt_custom_github_copilot_aligned_agent_framework.md`. This issue extends that
framework; it does not repeat or replace its decision. The new policy must distinguish repository
conventions from external runtime implementation details and keep shared project knowledge in
tracked artifacts. Provider-specific configurations remain useful adapters, but they must not be
the only record of a repository workflow, decision, capability requirement, or project fact.

The tracked profiles under `.github/agents/` evidence profile names, purposes, and declared tools.
They do not evidence a fixed model, memory capability, context window, vendor runtime version, or
cross-runtime behavior. Any compatibility record must therefore state its source, review date,
scenario, result, and limitations without claiming vendor guarantees.

## Scope

### In Scope

- Create an ADR extending the existing agent-framework decision with an authority model for
  repository knowledge, agent context, and optional memory.
- Define that Git-tracked repository artifacts are authoritative for repository conventions and
  project decisions; agent-local retained state is a non-authoritative convenience cache.
- Define provider-specific agent profiles, skills/custom commands, tool and MCP integrations,
  session history, semantic indexes, cloud-agent setup, and IDE settings as optional adapters rather
  than sources of truth for repository workflows or knowledge.
- Inventory the repository's agent-related capabilities and configurations, documenting each
  capability's purpose, canonical tracked workflow/source, portability risk, practical alternative,
  evidence, and limitations.
- Define an instruction-precedence and discovery record for repository-controlled instructions so
  contributors can understand which tracked artifacts an agent is expected to load.
- Define a memory-write decision rule that promotes reusable repository knowledge to an appropriate
  tracked artifact before it is cached locally.
- Define prohibited memory content, including credentials, passphrases, tokens, sensitive personal
  data, speculation, and unverified facts.
- Define functional terms for tracked content, session state, user-local retained preferences, and
  runtime-managed retained project state without relying on vendor-specific storage paths.
- Define a bounded exception for temporary environment facts and a promotion rule for facts that
  become reusable by contributors.
- Add an AI-agent implementation-independence engineering principle to `AGENTS.md`, with concise
  operational rules and links to the canonical policy.
- Make repository-defined agents discoverable without duplicating their frontmatter; add a minimal
  `.github/agents/README.md` catalog only if it provides a clear navigational benefit.
- Define a support-matrix evidence format and a deterministic review trigger/cadence with recorded
  findings.
- Register any new long-lived documentation in `docs/index.md` and update `docs/AGENTS.md` when its
  directory guidance changes.

### Out of Scope

- Requiring contributors to use a particular AI agent, vendor, IDE, model, or memory backend.
- Implementing cross-vendor context or memory storage.
- Replacing every provider-specific agent feature or integration during this issue.
- Guaranteeing that every external provider supports the same capabilities.
- Treating inaccessible or runtime-managed memory as authoritative repository documentation.
- Recording secrets, passphrases, credentials, tokens, or personal sensitive data.
- Claiming compatibility, model availability, or runtime behavior without reproducible evidence.
- Creating a dedicated memory-maintenance skill unless implementation reveals a concrete, repeatable
  on-demand workflow that exceeds an always-on rule and canonical documentation.

## Proposed Policy

### Authority model

For repository conventions and project decisions, authority is ordered as follows:

1. Git-tracked repository documents and configuration: `AGENTS.md`, ADRs, `.github/skills/`,
   `.github/agents/`, templates, and canonical documents under `docs/`.
2. Agent-local or runtime-managed retained state, which is optional, non-authoritative, and
   disposable.
3. External vendor/runtime implementation details, which are not repository requirements.

This hierarchy applies only within repository-controlled guidance. It does not override system,
security, legal, platform, or user instructions that govern an agent's execution environment.

A reusable repository convention or decision that exists only in agent-local retained state is
considered undocumented and must be promoted to a tracked source of truth.

Provider-specific profiles, instruction adapters, skills, tool integrations, indexes, cloud setup,
and IDE settings must similarly point to or implement a documented canonical workflow. Their
absence from another runtime must not make repository knowledge or required validation impossible
to discover and reproduce with standard tools.

### Engineering principle

Add this principle to the **Engineering Policies** section of `AGENTS.md`:

> **AI-agent implementation independence**: Keep repository knowledge, decisions, workflows, and
> validation reproducible from Git-tracked documentation, scripts, tests, and documented standard
> interfaces. Treat provider-specific agent profiles, memory, indexes, tools, and cloud setup as
> optional adapters, not sources of truth. Do not make a provider-specific capability a required
> repository workflow unless its purpose, portability limitation, and practical alternative are
> documented.

The implementation should refine the wording for consistency with `AGENTS.md`, retain a concise
rule there, and link to the ADR or canonical operational policy for the complete procedure.

### Capability inventory and portability assessment

The policy must inventory these capability categories where they are used by the repository:

| Capability category                           | Inventory requirement                                                                                                                 |
| --------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------- |
| Agent profiles and instruction precedence     | Record the tracked profile/instruction adapter, its purpose, discovery/precedence evidence, and the canonical portable workflow.      |
| Skills and custom commands                    | Record the tracked procedure or source document, provider-specific invocation mechanism, and a plain-Markdown/standard-tool fallback. |
| Tool and MCP integrations                     | Record the required capability, authentication boundary, standard interface or alternative, and any runtime limitation.               |
| Memory, session history, and semantic indexes | Record retention/visibility assumptions functionally, not by vendor path; require promotion of reusable knowledge to tracked sources. |
| Cloud-agent and CI setup                      | Record required toolchain, Git access, and validation capabilities separately from a provider-specific setup workflow.                |
| IDE and workspace settings                    | Record repository-required settings in tracked configuration or documentation; do not rely on undocumented user settings.             |
| Provider-managed secrets or context           | Keep non-secret configuration tracked and use documented secret-management mechanisms; never retain secret values in agent context.   |

For each inventoried provider-specific integration, document its purpose, canonical repository
workflow or source, portability risk, practical alternative, review evidence, and limitations. The
inventory must identify high-risk dependencies as follow-up work rather than silently assuming they
are portable.

### Memory-write decision rule

| Information type                                                                                       | Required handling                                                                                                                               |
| ------------------------------------------------------------------------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| Shared policy, workflow, convention, architecture decision, verified project fact, or reusable command | Capture or update it in the appropriate tracked artifact first. Local memory may retain only a concise pointer to that source.                  |
| User-specific working preference                                                                       | Retain only in user-scoped state when supported by the runtime and safe to retain.                                                              |
| Temporary task state                                                                                   | Keep it session-scoped or do not persist it.                                                                                                    |
| Secret, credential, passphrase, token, sensitive personal data, speculation, or unverified fact        | Never retain it in agent memory.                                                                                                                |
| Agent, vendor, or runtime implementation detail                                                        | Document it only as optional compatibility evidence with source, date, scenario, result, and limitations. Do not make it a project requirement. |

### Compatibility evidence and review

A support matrix must distinguish the following states:

- **Tracked**: a repository-defined profile exists and its tracked definition passes repository
  documentation checks.
- **Reviewed**: the profile or workflow was assessed against a named public runtime/documentation
  source on a stated date.
- **Verified**: a concrete scenario was manually exercised, with source/version evidence, result,
  and limitations recorded.

The policy must define one deterministic review cadence and event-driven triggers. Review records
must name the configuration checked, source/version evidence where available, scenario, result,
limitations, and date. A missing external-runtime capability must be recorded as unavailable rather
than inferred.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                                     | Notes / Expected Output                                                                                                                                   |
| --- | ------ | ------------------------------------------------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | DONE   | Decide ADR scope and relationship to ADR 20260420200013                  | New ADR extends the existing framework decision; it does not supersede it.                                                                                |
| T2  | DONE   | Create the governance ADR                                                | Added `20260821172000_establish_ai_agent_context_capability_and_portability_governance.md`.                                                               |
| T3  | DONE   | Inventory agent capabilities and portability risks                       | ADR records observed profiles, instructions, skills, prompts, tools/MCP preference, cloud setup, IDE settings, retained state, evidence, and limitations. |
| T4  | DONE   | Create an operational companion only if necessary                        | Not added: the ADR and concise `AGENTS.md` rule provide one source of truth without duplicating procedure.                                                |
| T5  | DONE   | Add the implementation-independence engineering principle and navigation | Added Engineering Policy 7 and ADR links from `docs/index.md` and `docs/AGENTS.md`.                                                                       |
| T6  | DONE   | Add a minimal agent catalog if it improves discovery                     | Added `.github/agents/README.md` as a link-only catalog; each `.agent.md` remains authoritative.                                                          |
| T7  | DONE   | Define support-matrix and portability-review records                     | ADR defines `Tracked`, `Reviewed`, and `Verified` evidence states, annual August review, and event triggers.                                              |
| T8  | DONE   | Evaluate dedicated maintenance skills                                    | Not added: no concrete recurring on-demand workflow justified a new skill.                                                                                |
| T9  | DONE   | Verify documentation and links                                           | Full pre-commit checks passed; manual verification results are recorded below.                                                                            |

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted in `docs/issues/drafts/`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [x] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [x] Manual verification scenarios executed and recorded (status + evidence)
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-08-21 16:30 UTC - GitHub Copilot - Created formal draft from the governance proposal and repository exploration; awaiting maintainer review before creating a GitHub issue.
- 2026-08-21 16:45 UTC - GitHub Copilot - Expanded the draft before approval to cover provider-specific capabilities and portability risks beyond retained memory.
- 2026-08-21 16:50 UTC - GitHub Copilot - GitHub issue #2075 created; spec moved from `docs/issues/drafts/` to `docs/issues/open/`.
- 2026-08-21 17:10 UTC - GitHub Copilot - Spec-only PR #2076 opened against `develop`.
- 2026-08-21 17:25 UTC - GitHub Copilot - Implemented the governance ADR, agent catalog, Engineering Policy, and documentation navigation; validation remains in progress.
- 2026-08-21 17:30 UTC - GitHub Copilot - Full pre-commit checks passed; recorded manual verification, including unavailable external-runtime evidence.

## Acceptance Criteria

- [x] AC1: A tracked ADR defines the authority model and explicitly states that agent-local retained
      state cannot be the sole record of repository conventions or project decisions.
- [x] AC2: The policy contains a memory-write decision rule, prohibited-content rule, and bounded
      promotion rule for reusable environment facts.
- [x] AC3: The policy uses functional retention/visibility terminology and does not require a
      vendor-specific memory path or implementation.
- [x] AC4: The policy inventories used provider-specific capability categories and records each
      integration's purpose, canonical workflow/source, portability risk, practical alternative,
      evidence, and limitation.
- [x] AC5: Repository workflows and knowledge remain discoverable and reproducible with tracked
      Markdown, scripts, tests, or documented standard interfaces when provider-specific adapters are
      unavailable.
- [x] AC6: `AGENTS.md` Engineering Policies contains a concise AI-agent implementation-independence
      principle and links to the canonical policy.
- [x] AC7: Repository-defined agent profiles are discoverable through tracked navigation without
      duplicating their authoritative frontmatter.
- [x] AC8: Compatibility/support records distinguish tracked, reviewed, and verified states and
      include evidence, date, scenario, result, and limitations.
- [x] AC9: The policy defines a deterministic review cadence, event-driven triggers, and a durable
      review-record format.
- [x] AC10: Existing secret-handling guidance is linked rather than contradicted or duplicated.
- [x] `linter all` exits with code `0`.
- [x] Relevant documentation tests pass.
- [x] Manual verification scenarios are executed and documented (status + evidence).
- [x] Acceptance criteria are re-reviewed after implementation and reflect actual behavior.
- [x] Documentation is updated when behavior/workflow changes.

## Verification Plan

Define verification before implementation starts and execute it before closing the issue.

### Automatic Checks

- `linter all`
- `cargo test --doc --workspace`
- Link checks or documentation-specific validation available in the repository
- Pre-push checks when applicable

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                                    | Command/Steps                                                                                                                          | Expected Result                                                                                 | Status | Evidence                                                                                                                                  |
| --- | ------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------- | ------ | ----------------------------------------------------------------------------------------------------------------------------------------- |
| M1  | Trace authority for a shared convention     | Select a representative project convention and confirm its tracked source is discoverable from `AGENTS.md` or canonical documentation. | The convention is not dependent on retained agent state.                                        | DONE   | `AGENTS.md` Engineering Policy 7 links to the ADR; the ADR defines tracked artifacts as authoritative.                                    |
| M2  | Apply memory-write decision rule            | Classify one shared repository fact, one temporary task fact, one user preference, and one prohibited secret-like value.               | Each classification selects the required storage/promotion outcome.                             | DONE   | ADR retained-state rules define all four outcomes.                                                                                        |
| M3  | Trace provider-specific capability fallback | Select one profile/skill/tool or cloud setup adapter and follow its canonical source or documented standard-tool alternative.          | Required repository workflow remains discoverable without relying solely on the adapter.        | DONE   | `github-operator.agent.md` documents MCP → GitHub CLI → raw API preference; ADR records the GitHub CLI/raw API alternative.               |
| M4  | Review agent catalog and support record     | Compare catalog links to tracked `.github/agents/*.agent.md` definitions and inspect one evidence record.                              | The catalog does not duplicate profile metadata or claim unverified runtime guarantees.         | DONE   | `.github/agents/README.md` links all ten profile definitions; the ADR labels unsupported runtime behavior unverified.                     |
| M5  | Validate optional external-runtime evidence | Where an accessible runtime exposes a public version/capability source, record the review source, scenario, result, and limitation.    | Any unavailable evidence is explicitly marked unavailable; the policy remains valid without it. | DONE   | ADR initial review record states that no reproducible external runtime/version source was available and records the resulting limitation. |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                                                                                        |
| ----- | ---------------------- | ----------------------------------------------------------------------------------------------- |
| AC1   | DONE                   | `docs/adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md`  |
| AC2   | DONE                   | ADR retained-state rules and promotion requirement.                                             |
| AC3   | DONE                   | ADR uses functional retained-state terminology and avoids vendor paths.                         |
| AC4   | DONE                   | ADR capability inventory records tracked evidence and limitations for all scope categories.     |
| AC5   | DONE                   | ADR requires tracked canonical workflows or practical alternatives for provider adapters.       |
| AC6   | DONE                   | `AGENTS.md` Engineering Policy 7 links to the ADR.                                              |
| AC7   | DONE                   | `.github/agents/README.md` is a link-only catalog of the ten authoritative profile definitions. |
| AC8   | DONE                   | ADR defines `Tracked`, `Reviewed`, and `Verified` with evidence requirements.                   |
| AC9   | DONE                   | ADR requires annual August review and event-driven reviews with durable records.                |
| AC10  | DONE                   | ADR links to existing secret-handling guidance and defines only the agent-retention boundary.   |

## Risks and Trade-offs

- **Policy duplication**: An ADR, guide, `AGENTS.md`, and catalog could drift. Mitigation: make the
  ADR the decision record, keep `AGENTS.md` concise, and add an operational companion only if it
  cannot be expressed without duplication.
- **Overstating compatibility**: A support matrix may imply vendor guarantees. Mitigation: define
  tracked, reviewed, and verified states; require dated evidence and limitations.
- **Hidden capability lock-in**: A proprietary profile, skill, tool, index, setup workflow, or IDE
  setting may become the only way to discover or execute required work. Mitigation: inventory
  provider-specific adapters and require a tracked canonical workflow or practical alternative.
- **Memory loopholes**: A broad exception for environment facts could hide project knowledge.
  Mitigation: require promotion to a tracked artifact when the fact is reusable by contributors or
  relevant beyond the task.
- **Unenforceable runtime controls**: Some runtimes cannot expose or delete retained state.
  Mitigation: treat that as a documented runtime limitation and never rely on inaccessible state as
  authoritative knowledge.
- **Scope expansion**: A dedicated skill or detailed compatibility catalog may exceed the initial
  governance need. Mitigation: add each only when a concrete, repeatable maintenance workflow or
  navigational gap is demonstrated.

## References

- Existing agent framework ADR: `docs/adrs/20260420200013_adopt_custom_github_copilot_aligned_agent_framework.md`
- Agent profile definitions: `.github/agents/`
- Agent setup workflow: `.github/workflows/copilot-setup-steps.yml`
- Agent portability topics: profiles, instruction precedence, skills/custom commands, tools/MCP,
  retained context, session history, semantic indexes, cloud setup, and IDE settings
- Historical configuration issue: #1697
- Semantic skill-link convention: `docs/skills/semantic-skill-link-convention.md`
- Skill-creation guidance: `.github/skills/add-new-skill/SKILL.md`
- Secret-handling guidance: `.github/skills/dev/rust-code-quality/handle-secrets/SKILL.md`
