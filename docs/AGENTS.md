---
semantic-links:
  skill-links:
    - write-markdown-docs
  related-artifacts:
    - docs/index.md
    - docs/architecture/README.md
    - docs/adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md
    - docs/skills/semantic-skill-link-convention.md
---

# `docs/` — Documentation Directory

This directory contains all project documentation: operational guides, architectural decision
records, issue and refactor-plan specifications, templates, and supporting media.

For the full project context see the [root AGENTS.md](../AGENTS.md).

## Directory Map

| Path                                                                                      | Purpose                                                                            |
| ----------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------- |
| `index.md`                                                                                | Entry point — structured index of every document and subdirectory                  |
| `architecture/`                                                                           | Runtime-composition guides: tracker instances, shared services, and event topology |
| `benchmarking.md`                                                                         | How to run and interpret torrent-repository benchmarks                             |
| `containers.md`                                                                           | Running the tracker with Docker / Podman                                           |
| `packages.md`                                                                             | Workspace package catalog, architecture layers, dependency rules                   |
| `profiling.md`                                                                            | CPU and memory profiling with Valgrind / kcachegrind                               |
| `release_process.md`                                                                      | Branch strategy, versioning, and the release pipeline                              |
| `adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md` | Authority and portability governance for AI-agent workflows and retained context   |
| `adrs/`                                                                                   | Architectural Decision Records (ADRs)                                              |
| `issues/`                                                                                 | Issue specification documents linked to GitHub issues                              |
| `refactor-plans/`                                                                         | Refactor plan specifications (same lifecycle as issue specs)                       |
| `pr-reviews/`                                                                             | Notable PR review records and Copilot suggestion threads                           |
| `skills/`                                                                                 | Internal conventions used by humans and AI agents                                  |
| `templates/`                                                                              | Canonical document templates (ADR, EPIC, issue, refactor plan)                     |
| `media/`                                                                                  | Images, diagrams, flamegraphs, benchmark reports, sample torrents                  |
| `licenses/`                                                                               | Full license texts (AGPL-3.0, MIT-0)                                               |

### Where to place a new artifact

| Artifact type                                  | Target location                                                                                                                     |
| ---------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| New ADR                                        | `docs/adrs/` — filename format: `YYYYMMDDHHMMSS_<short-slug>.md`                                                                    |
| New issue spec (before GitHub issue exists)    | `docs/issues/drafts/`                                                                                                               |
| New issue spec (after GitHub issue created)    | `docs/issues/open/<number>-<short-slug>.md`, or `docs/issues/open/<number>-<short-slug>/ISSUE.md` when it has issue-local artifacts |
| New refactor plan (before GitHub issue exists) | `docs/refactor-plans/drafts/`                                                                                                       |
| New refactor plan (after GitHub issue created) | `docs/refactor-plans/open/<number>-<short-slug>.md`                                                                                 |
| New document template                          | `docs/templates/`                                                                                                                   |
| New diagram or screenshot                      | `docs/media/` (or the relevant subdirectory)                                                                                        |

## Markdown Frontmatter

Frontmatter use varies by document type:

- **Required** for issue specs and EPIC specs — see the required field schema in
  [`docs/skills/semantic-skill-link-convention.md`](skills/semantic-skill-link-convention.md).
- **Recommended** for ADRs, refactor plans, and other specification documents.
- **Optional** for short reference pages and README files.

Use `semantic-links` in frontmatter to couple a document to the Agent Skills it affects:

```yaml
---
semantic-links:
  skill-links:
    - <skill-name>
  related-artifacts:
    - <repo-relative-path>
---
```

## Markdown Linting

Repository `.md` files are linted by markdownlint using the configuration in
[`.markdownlint.json`](../.markdownlint.json).

**GitHub surfaces are a different context.** Issue descriptions, PR descriptions, and review
comments are rendered by GitHub and are **not** governed by `.markdownlint.json`. In
particular:

- Do **not** hard-wrap lines in GitHub issue or PR body text. Wrapping produces broken
  paragraphs on GitHub's web UI. Write each paragraph as a single continuous line.
- The `MD013` line-length rule is disabled in the repo config, but repo files should still be
  kept readable. GitHub surfaces have no such constraint at all.

See the `write-markdown-docs` skill for the full checklist and GFM pitfalls.

## Key Skills

| Skill                                                                                | When to use                                                                             |
| ------------------------------------------------------------------------------------ | --------------------------------------------------------------------------------------- |
| [`write-markdown-docs`](../.github/skills/dev/planning/write-markdown-docs/SKILL.md) | Writing or editing any `.md` file — covers GFM pitfalls, frontmatter, and linting scope |
| [`create-issue`](../.github/skills/dev/planning/create-issue/SKILL.md)               | Drafting and creating issue specifications                                              |
