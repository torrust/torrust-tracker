---
semantic-links:
  skill-links:
    - write-markdown-docs
  related-artifacts:
    - docs/AGENTS.md
    - docs/architecture/README.md
    - docs/adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md
    - docs/benchmarking.md
    - docs/application-jobs.md
    - docs/containers.md
    - docs/packages.md
    - docs/profiling.md
    - docs/release_process.md
    - docs/adrs/README.md
    - docs/adrs/index.md
    - docs/issues/README.md
    - docs/copilot-pr-reviews/README.md
    - docs/refactor-plans/closed/README.md
    - docs/refactor-plans/drafts/README.md
    - docs/refactor-plans/open/README.md
---

# Torrust Tracker — Documentation Index

This is the entry point for all project documentation. For API documentation generated from
source code, see the [crate docs on docs.rs][docs].

## Guides

Operational and development guides for working with the tracker.

| Document                                                                                                                                                                           | Description                                                          |
| ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------- |
| [application-jobs.md](application-jobs.md)                                                                                                                                         | Current background-job ownership, lifecycle, and shutdown behavior   |
| [benchmarking.md](benchmarking.md)                                                                                                                                                 | How to run and interpret the torrent-repository benchmarks           |
| [containers.md](containers.md)                                                                                                                                                     | Building and running the tracker with Docker / Podman                |
| [git-hooks.md](git-hooks.md)                                                                                                                                                       | Hook behavior and SSH idle-timeout troubleshooting                   |
| [packages.md](packages.md)                                                                                                                                                         | Workspace package catalog, architecture layers, and dependency rules |
| [Configuration v2-to-v3 migration guide](../packages/configuration/docs/migrate-v2-to-v3.md)                                                                                       | Upgrade tracker configuration files to active schema v3              |
| [profiling.md](profiling.md)                                                                                                                                                       | CPU and memory profiling with Valgrind / kcachegrind                 |
| [release_process.md](release_process.md)                                                                                                                                           | Branch strategy, versioning, and the staging → main release pipeline |
| [adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md](adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md) | Governance for portable AI-agent workflows and retained context      |

## Runtime Architecture

Guides describing the running application's composition and behavior. They
complement ADRs, which record accepted architectural decisions.

| Document                                         | Description                                                                         |
| ------------------------------------------------ | ----------------------------------------------------------------------------------- |
| [architecture/README.md](architecture/README.md) | Runtime architecture index: tracker instances, shared services, and event topology. |

## Architecture Decisions (ADRs)

Records of significant architectural decisions, including context and consequences.

| Document                         | Description                                              |
| -------------------------------- | -------------------------------------------------------- |
| [adrs/README.md](adrs/README.md) | Root ADR guidance, including placement by decision scope |
| [adrs/index.md](adrs/index.md)   | Quick-reference table of repository-level ADRs           |

## Issue Specifications

Structured specification documents linked to GitHub issues. Used for planning and tracking
implementation work before and during development.

| Location                             | Description                                               |
| ------------------------------------ | --------------------------------------------------------- |
| [issues/README.md](issues/README.md) | Overview, folder structure, and workflow skill references |
| [issues/drafts/](issues/drafts/)     | Specs not yet linked to a GitHub issue                    |
| [issues/open/](issues/open/)         | Active specs for open GitHub issues                       |
| [issues/closed/](issues/closed/)     | Recently closed specs kept temporarily for reference      |

## Refactor Plans

Specification documents for larger refactoring efforts, following the same lifecycle as issue
specs (drafts → open → closed).

| Location                                         | Description                                         |
| ------------------------------------------------ | --------------------------------------------------- |
| [refactor-plans/drafts/](refactor-plans/drafts/) | Draft refactor plans not yet tied to a GitHub issue |
| [refactor-plans/open/](refactor-plans/open/)     | Active refactor plan specs                          |
| [refactor-plans/closed/](refactor-plans/closed/) | Completed refactor plans kept for reference         |

## Copilot PR Reviews

Records of Copilot pull request suggestion reviews.

| Document                                                     | Description                               |
| ------------------------------------------------------------ | ----------------------------------------- |
| [copilot-pr-reviews/README.md](copilot-pr-reviews/README.md) | Overview of the Copilot PR review archive |

## Skills and Conventions

Internal documentation on project-specific conventions used by both humans and AI agents.

| Document                                                                             | Description                                                                                |
| ------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------ |
| [skills/semantic-skill-link-convention.md](skills/semantic-skill-link-convention.md) | Frontmatter schema, `skill-link` marker catalog, and machine-readable metadata conventions |

## Templates

Canonical document templates. Copy the appropriate template when creating a new artifact of
that type.

| Template                                                                               | Description                                            |
| -------------------------------------------------------------------------------------- | ------------------------------------------------------ |
| [templates/ADR.md](templates/ADR.md)                                                   | Template for Architectural Decision Records            |
| [templates/EPIC.md](templates/EPIC.md)                                                 | Template for EPIC issue specifications                 |
| [templates/ISSUE.md](templates/ISSUE.md)                                               | Template for task / bug / feature issue specifications |
| [templates/REFACTOR-PLAN.md](templates/REFACTOR-PLAN.md)                               | Template for refactor plan specifications              |
| [templates/COPILOT-SUGGESTIONS-TEMPLATE.md](templates/COPILOT-SUGGESTIONS-TEMPLATE.md) | Template for recording Copilot PR review suggestions   |

## Media

Images, diagrams, flamegraphs, benchmark reports, and sample torrent files used in
documentation.

| Location                           | Description                                                                  |
| ---------------------------------- | ---------------------------------------------------------------------------- |
| [media/](media/)                   | Top-level media assets (flamegraphs, benchmark screenshots, sample torrents) |
| [media/demo/](media/demo/)         | Screenshots and assets used in demo documentation                            |
| [media/packages/](media/packages/) | Package architecture diagrams                                                |

## Licenses

Full license texts referenced by the project.

| Location               | Description                      |
| ---------------------- | -------------------------------- |
| [licenses/](licenses/) | AGPL-3.0 and MIT-0 license files |

[docs]: https://docs.rs/torrust-tracker/latest/torrust_tracker/
