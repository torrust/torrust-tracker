---
doc-type: issue
issue-type: task
status: done
priority: p3
github-issue: 1810
spec-path: docs/issues/open/1810-add-frontmatter-to-docs-markdown-files.md
branch: "1810-add-frontmatter-to-docs-markdown-files"
related-pr: null
last-updated-utc: 2026-05-20 15:45
semantic-links:
  skill-links:
    - create-issue
    - write-markdown-docs
  related-artifacts:
    - docs/skills/semantic-skill-link-convention.md
    - .github/skills/dev/planning/write-markdown-docs/SKILL.md
    - docs/AGENTS.md
    - docs/templates/ISSUE.md
    - docs/templates/EPIC.md
    - docs/templates/ADR.md
    - docs/templates/REFACTOR-PLAN.md
    - docs/templates/COPILOT-SUGGESTIONS-TEMPLATE.md
---

# Issue #1810 — Add YAML frontmatter and semantic links to all `docs/` Markdown files

## Goal

Add YAML frontmatter to every Markdown file under `docs/` that currently lacks it, populate
`related-artifacts` based on semantic analysis of each file, and apply bidirectional links
between Markdown files so that when file A references file B, file B also references file A.
Follow the convention defined in `docs/skills/semantic-skill-link-convention.md` and
summarized in `docs/AGENTS.md`.

## Background

The project defines a lightweight YAML frontmatter convention (see
`docs/skills/semantic-skill-link-convention.md`) to keep document metadata machine-readable
and to couple artifacts to Agent Skills via `semantic-links`.

Usage varies by document type:

- **Required** — issue specs and EPIC specs must include `doc-type`, status, issue tracking
  fields, and `semantic-links`.
- **Recommended** — ADRs, refactor plans, PR-review docs, and skills docs should include at
  minimum `semantic-links` (following their respective templates).
- **Optional** — short reference pages, README/index files.

Despite the convention being established, a large number of existing `docs/` files predate it
and have no frontmatter at all. This means agents and tooling cannot reliably query document
metadata, and several issue/EPIC specs violate the "required" rule.

A scan of `docs/` on 2026-05-20 found **67 files** without frontmatter. This issue
tracks adding the appropriate frontmatter to every one of them.

Beyond structural compliance, the `related-artifacts` field is the key mechanism for coupling
documentation to the code and other artifacts it describes. Without it, agents cannot discover
which source files, packages, skills, or other documents a given doc governs — and cannot
travel the graph in either direction. Bidirectionality between Markdown files is achievable
purely within `docs/` frontmatter and has a high signal-to-noise ratio: it makes the
relationship explicit, machine-queryable, and maintainable without touching source code.

## Scope

### In Scope

- Add YAML frontmatter to every `docs/` Markdown file listed in the [File Inventory](#file-inventory).
- Use the correct frontmatter shape for each document type (see [Frontmatter Guidance](#frontmatter-guidance)).
- Perform semantic analysis of each file (see [Semantic Analysis Guidance](#semantic-analysis-guidance))
  to identify meaningful related artifacts and populate `related-artifacts` accordingly.
- Apply bidirectional links between Markdown files within `docs/`: when file A lists file B in
  `related-artifacts`, file B must also list file A (see bidirectionality rules).
- Reference source code paths (packages, modules, key files) in `related-artifacts` of the
  Markdown file that documents them.
- Clarify inline `<!-- skill-link: ... -->` versus frontmatter `skill-links` guidance in
  `docs/skills/semantic-skill-link-convention.md` (T15): when frontmatter is present,
  frontmatter is the canonical machine-readable source; inline top-of-file comments are
  redundant and should be omitted.
- Do not change body content, headings, or links in any file — only the frontmatter block
  (exception: T15 updates targeted convention guidance in
  `docs/skills/semantic-skill-link-convention.md`).
- Inline `<!-- skill-link: ... -->` body markers are **not** being added to any file;
  frontmatter is the canonical source when present.

### Out of Scope

- Changing the content, structure, or headings of any file (exception: T15 targeted content
  update in `docs/skills/semantic-skill-link-convention.md`).
- Restructuring or renaming subdirectories under `docs/`.
- Updating `docs/templates/` content.
- Updating `docs/skills/semantic-skill-link-convention.md` beyond the targeted inline-marker
  clarification in T15.
- Adding frontmatter to Markdown files outside `docs/` (covered by separate work if needed).
- Adding back-reference annotations inside source code files (Rust, TOML, shell): no
  convention for doc back-references in source code is defined; that is a follow-up issue.
- Updating `related-artifacts` in `.github/skills/` SKILL.md files that are referenced by
  `docs/` files: those files already have frontmatter and a separate concern governs them.

## Frontmatter Guidance

Use the following shapes as the canonical reference for each document type.
See the full spec in `docs/skills/semantic-skill-link-convention.md`.

### Issue specs (`doc-type: issue`)

```yaml
---
doc-type: issue
issue-type: <task|bug|feature|enhancement>
status: done
priority: <p0|p1|p2|p3>
github-issue: <number>
spec-path: <repo-relative-path>
branch: "<branch-name>"
related-pr: <number|null>
last-updated-utc: YYYY-MM-DD HH:MM
semantic-links:
  skill-links:
    - create-issue
  related-artifacts: []
---
```

For closed issue specs, use `status: done`. Derive `github-issue`, `branch`, and `last-updated-utc`
from the file content or git history. Use `null` for fields that cannot be determined.

### EPIC specs (`doc-type: epic`)

```yaml
---
doc-type: epic
status: done
github-issue: <number>
spec-path: <repo-relative-path>
epic-owner: null
last-updated-utc: YYYY-MM-DD HH:MM
semantic-links:
  skill-links:
    - create-issue
  related-artifacts: []
---
```

### Refactor plans (`doc-type: refactor-plan`)

```yaml
---
doc-type: refactor-plan
status: done
related-issue: <number|null>
spec-path: <repo-relative-path>
last-updated-utc: YYYY-MM-DD HH:MM
semantic-links:
  skill-links:
    - create-refactor-plan
  related-artifacts: []
---
```

### ADR files

```yaml
---
semantic-links:
  skill-links:
    - create-adr
  related-artifacts:
    - .github/skills/dev/planning/create-adr/SKILL.md
---
```

### PR review files

```yaml
---
semantic-links:
  skill-links:
    - process-copilot-suggestions
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-copilot-suggestions/SKILL.md
---
```

### General docs and README/index files

For files that do not fall into a named document type (guides, AGENTS.md, index files,
README files), add a minimal frontmatter block with `semantic-links` where a relevant
skill-link exists; otherwise use an empty block:

```yaml
---
semantic-links:
  skill-links:
    - write-markdown-docs
  related-artifacts: []
---
```

README/index navigation files may use an empty frontmatter block if no skill-link applies:

```yaml
---
# navigation index — no semantic skill links
---
```

## Semantic Analysis Guidance

### What to analyze per file

For each file, read the full content and identify:

1. **Packages or crates** explicitly mentioned (e.g., `torrust-tracker-core`, `packages/tracker-core/`).
2. **Source files or modules** referenced (e.g., `src/app.rs`, `packages/*/src/lib.rs`).
3. **Other `docs/` Markdown files** explicitly linked or discussed.
4. **Agent Skills** (`.github/skills/`) the file is governed by or relies on.
5. **GitHub issues or PRs** (use `github-issue` / `related-pr` metadata fields for these,
   not `related-artifacts` — `related-artifacts` holds repo-relative file paths only).

Keep `related-artifacts` high-signal: list only artifacts with a clear, direct relationship.
Do not list every file incidentally mentioned; focus on structural coupling.

### Bidirectionality rules

| Relationship type                         | Rule                                                                                                                          |
| ----------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| `docs/` file A → `docs/` file B           | Bidirectional: add A to B's `related-artifacts` and B to A's.                                                                 |
| `docs/` file → `.github/skills/` SKILL.md | One-directional: add the skill path to the doc's `related-artifacts` only. The reverse update is out of scope for this issue. |
| `docs/` file → source code path           | One-directional: add the source path to the doc's `related-artifacts` only. Source code back-references are out of scope.     |
| `docs/` file → GitHub issue/PR URL        | Not a `related-artifacts` entry. Use `github-issue` / `related-pr` metadata fields in issue/EPIC specs.                       |

### Handling the bidirectionality backlog

When semantic analysis of a file (say file A) identifies that file B should reference file A
but file B is in a **later task batch**, note the pending back-reference in the Notes column
of the implementation plan. Apply it when that later batch is processed.

When file B is **already in a completed batch**, apply the back-reference to file B
immediately (a small additive change to its frontmatter).

### Priority guidance

- Prioritize `related-artifacts` accuracy for **top-level docs**, **ADRs**, and **open issue
  specs** — these are most frequently queried by agents.
- For **closed issue specs**, a minimal frontmatter (required fields + obvious direct links)
  is sufficient; exhaustive semantic research is not required.
- For **README/navigation files**, `related-artifacts` may be omitted or list only the most
  architecturally significant entries.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

Each task covers a logical batch of files and includes both semantic research and frontmatter
application. The detailed per-file checklist is in the [File Inventory](#file-inventory) section.

| ID  | Status | Task                                                                                                 | Files in batch |
| --- | ------ | ---------------------------------------------------------------------------------------------------- | -------------- |
| T0  | DONE   | Semantic research pre-pass: analyze all 67 files, build relationship map                             | all 67         |
| T1  | DONE   | Add frontmatter + semantic links to top-level `docs/` files                                          | 7              |
| T2  | DONE   | Add frontmatter + semantic links to `docs/adrs/` ADR files                                           | 5              |
| T3  | DONE   | Add frontmatter + semantic links to `docs/adrs/` navigation files                                    | 2              |
| T4  | DONE   | Add frontmatter + semantic links to `docs/issues/` README/nav files                                  | 4              |
| T5  | DONE   | Add frontmatter + semantic links to `docs/issues/closed/` ≤ 672 specs                                | 4              |
| T6  | DONE   | Add frontmatter + semantic links to `docs/issues/closed/` 1525–1563                                  | 6              |
| T7  | DONE   | Add frontmatter + semantic links to `docs/issues/closed/` 1582 group                                 | 5              |
| T8  | DONE   | Add frontmatter + semantic links to `docs/issues/closed/` 1697–1723                                  | 10             |
| T9  | DONE   | Add frontmatter + semantic links to `docs/issues/closed/` 1732 group                                 | 6              |
| T10 | DONE   | Add frontmatter + semantic links to `docs/issues/closed/` 1740–1750                                  | 6              |
| T11 | DONE   | Add frontmatter + semantic links to `docs/issues/open/` supplementary                                | 4              |
| T12 | DONE   | Add frontmatter + semantic links to `docs/copilot-pr-reviews/` files                                 | 2              |
| T13 | DONE   | Add frontmatter + semantic links to `docs/refactor-plans/` files                                     | 5              |
| T14 | DONE   | Add frontmatter + semantic links to `docs/skills/` files                                             | 1              |
| T15 | DONE   | Clarify inline marker vs. frontmatter skill-links in `docs/skills/semantic-skill-link-convention.md` | 1              |

## File Inventory

Per-file progress checklist. Check each file when its frontmatter has been added and verified.

### T1 — Top-level `docs/` files (7)

- [x] `docs/AGENTS.md`
- [x] `docs/benchmarking.md`
- [x] `docs/containers.md`
- [x] `docs/index.md`
- [x] `docs/packages.md`
- [x] `docs/profiling.md`
- [x] `docs/release_process.md`

### T2 — `docs/adrs/` ADR files (5)

- [x] `docs/adrs/20240227164834_use_plural_for_modules_containing_collections.md`
- [x] `docs/adrs/20260420200013_adopt_custom_github_copilot_aligned_agent_framework.md`
- [x] `docs/adrs/20260429000000_keep_database_as_aggregate_supertrait.md`
- [x] `docs/adrs/20260512102000_define_tracker_client_peer_id_convention.md`
- [x] `docs/adrs/20260519000000_define_global_cli_output_contract.md`

### T3 — `docs/adrs/` navigation files (2)

- [x] `docs/adrs/README.md`
- [x] `docs/adrs/index.md`

### T4 — `docs/issues/` README/navigation files (4)

- [x] `docs/issues/README.md`
- [x] `docs/issues/closed/README.md`
- [x] `docs/issues/drafts/README.md`
- [x] `docs/issues/open/README.md`

### T5 — `docs/issues/closed/` — very old specs ≤ 672 (4)

- [x] `docs/issues/closed/523-internal-linting-tool.md`
- [x] `docs/issues/closed/669-overhaul-clients.md`
- [x] `docs/issues/closed/671-udp-tracker-client-print-unrecognized-responses.md`
- [x] `docs/issues/closed/672-http-tracker-client-print-unrecognized-responses.md`

### T6 — `docs/issues/closed/` — 1525–1563 specs (6)

- [x] `docs/issues/closed/1525-overhaul-persistence.md`
- [x] `docs/issues/closed/1532-http-tracker-client-add-optional-announce-params.md`
- [x] `docs/issues/closed/1533-udp-tracker-client-add-optional-announce-params.md`
- [x] `docs/issues/closed/1561-http-tracker-client-avoid-duplicating-announce-suffix.md`
- [x] `docs/issues/closed/1562-http-tracker-client-add-option-show-response-pretty-json.md`
- [x] `docs/issues/closed/1563-udp-tracker-client-add-option-show-response-pretty-json.md`

### T7 — `docs/issues/closed/` — 1582 group (5)

- [x] `docs/issues/closed/1582-add-prometheus-deserialization-metrics/ISSUE.md`
- [x] `docs/issues/closed/1582-add-prometheus-deserialization-metrics/increase-unit-test-coverage.md`
- [x] `docs/issues/closed/1582-add-prometheus-deserialization-metrics/metric-collection-module-split.md`
- [x] `docs/issues/closed/1582-add-prometheus-deserialization-metrics/mutation-testing.md`
- [x] `docs/issues/closed/1582-add-prometheus-deserialization-metrics/refactoring-proposals.md`

### T8 — `docs/issues/closed/` — 1697–1723 group (10)

- [x] `docs/issues/closed/1697-ai-agent-configuration.md`
- [x] `docs/issues/closed/1703-1525-01-persistence-test-coverage.md`
- [x] `docs/issues/closed/1706-1525-02-qbittorrent-e2e.md`
- [x] `docs/issues/closed/1710-1525-03-persistence-benchmarking.md`
- [x] `docs/issues/closed/1713-1525-04-split-persistence-traits.md`
- [x] `docs/issues/closed/1715-1525-04b-migrate-consumers-to-narrow-traits.md`
- [x] `docs/issues/closed/1717-1525-05-migrate-sqlite-and-mysql-to-sqlx.md`
- [x] `docs/issues/closed/1719-1525-06-introduce-schema-migrations.md`
- [x] `docs/issues/closed/1721-1525-07-align-rust-and-db-types.md`
- [x] `docs/issues/closed/1723-1525-08-add-postgresql-driver.md`

### T9 — `docs/issues/closed/` — 1732 group (6)

- [x] `docs/issues/closed/1732-replace-aquatic-udp-protocol/ISSUE.md`
- [x] `docs/issues/closed/1732-replace-aquatic-udp-protocol/step-2-analysis.md`
- [x] `docs/issues/closed/1732-replace-aquatic-udp-protocol/step-3-bittorrent-primitives-problem.md`
- [x] `docs/issues/closed/1732-replace-aquatic-udp-protocol/step-5-udp-protocol-module-refactor-plan.md`
- [x] `docs/issues/closed/1732-replace-aquatic-udp-protocol/step-6-primitives-module-refactor-plan.md`
- [x] `docs/issues/closed/1732-replace-aquatic-udp-protocol/step-7-peer-id-extraction-plan.md`

### T10 — `docs/issues/closed/` — 1740–1750 group (6)

- [x] `docs/issues/closed/1740-fix-container-workflow-caching.md`
- [x] `docs/issues/closed/1742-ci-change-aware-workflows-epic.md`
- [x] `docs/issues/closed/1743-docs-only-ci-fast-path.md`
- [x] `docs/issues/closed/1744-scope-persistence-workflows-by-path.md`
- [x] `docs/issues/closed/1748-remove-redundant-compose-step-from-container-workflow.md`
- [x] `docs/issues/closed/1750-refactor-run-tracker-skill-semantic-coupling.md`

### T11 — `docs/issues/open/` supplementary files (4)

- [x] `docs/issues/open/1669-overhaul-packages/readme-audit.md`
- [x] `docs/issues/open/1669-overhaul-packages/workspace-coupling-report.md`
- [x] `docs/issues/open/1726-reduce-build-times-sccache/ISSUE.md`
- [x] `docs/issues/open/1726-reduce-build-times-sccache/benchmark-results.md`

### T12 — `docs/copilot-pr-reviews/` files (2)

- [x] `docs/copilot-pr-reviews/README.md`
- [x] `docs/copilot-pr-reviews/pr-1733-copilot-suggestions.md`

### T13 — `docs/refactor-plans/` files (5)

- [x] `docs/refactor-plans/closed/1178-monitor-udp-post-implementation-improvements.md`
- [x] `docs/refactor-plans/closed/README.md`
- [x] `docs/refactor-plans/closed/agent-docs-refactor-plan.md`
- [x] `docs/refactor-plans/drafts/README.md`
- [x] `docs/refactor-plans/open/README.md`

### T14 — `docs/skills/` files (1)

- [x] `docs/skills/semantic-skill-link-convention.md`

### T15 — Convention doc content update (1)

- [x] Update `docs/skills/semantic-skill-link-convention.md` to clarify that when frontmatter
      is present with `semantic-links.skill-links`, inline `<!-- skill-link: ... -->` top-of-file
      comments are redundant. Body-level inline markers placed near a specific section remain
      valuable for navigation but are not required.

## Progress Tracking

### Workflow Checkpoints

- [ ] Spec drafted in `docs/issues/drafts/`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [ ] (Optional) Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, pre-push checks)
- [ ] Manual verification scenarios executed and recorded (status + evidence)
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

Append one line per meaningful update.

- 2026-05-20 14:00 UTC - Agent - Spec drafted; 67 files identified missing frontmatter across 14 logical batches
- 2026-05-20 15:00 UTC - Agent - Scope expanded: semantic analysis + bidirectional Markdown linking added per user request; new T0 pre-pass task added; AC7/AC8 added
- 2026-05-20 15:30 UTC - Agent - T15 added: clarify inline markers vs. frontmatter in convention doc (Option A); redundant top-of-file inline comments removed from this spec; AC9 added
- 2026-05-20 15:45 UTC - Agent - GitHub issue #1810 created; spec moved to docs/issues/open/

## Acceptance Criteria

- [ ] AC1: All 67 files listed in the [File Inventory](#file-inventory) have a valid YAML frontmatter block at the top of the file.
- [ ] AC2: Each file's frontmatter follows the correct shape for its document type (as defined in [Frontmatter Guidance](#frontmatter-guidance)).
- [ ] AC3: Issue and EPIC specs include all required metadata fields (`doc-type`, `status`, `github-issue`, `spec-path`, `last-updated-utc`).
- [ ] AC4: `linter all` exits with code `0` (markdownlint must pass for all modified files).
- [ ] AC5: No body content, headings, or links are changed in any file — only the frontmatter block is added at the top.
- [ ] AC6: `docs/skills/semantic-skill-link-convention.md` itself has frontmatter consistent with a skills convention document.
- [ ] AC7: Every `docs/` Markdown file that is listed in another file's `related-artifacts` also lists the referencing file in its own `related-artifacts` (bidirectionality rule for Markdown-to-Markdown links within `docs/`).
- [ ] AC8: Top-level docs files (`benchmarking.md`, `containers.md`, `packages.md`, `profiling.md`, `release_process.md`) have at least one `related-artifacts` entry pointing to a relevant source package or module.
- [ ] AC9: `docs/skills/semantic-skill-link-convention.md` guidance (T15) clarifies that when a
      Markdown file has frontmatter with `semantic-links.skill-links`, inline `<!-- skill-link: ... -->`
      top-of-file markers are redundant; frontmatter is the canonical machine-readable source.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented (status + evidence)
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior

## Verification

### Automatic checks

After each task batch:

```bash
linter markdown
linter cspell
```

After all batches:

```bash
linter all
```

Verify no file is missing frontmatter:

```bash
for f in $(find docs -name "*.md" | sort); do
  first_line=$(head -1 "$f")
  if [ "$first_line" != "---" ]; then
    echo "MISSING: $f"
  fi
done
```

The command should produce no output when all files have frontmatter.

### Manual scenarios

| Scenario                                                                                                 | Status | Evidence |
| -------------------------------------------------------------------------------------------------------- | ------ | -------- |
| Run the frontmatter check script above; verify zero output                                               | TODO   | —        |
| Spot-check 3 closed issue specs to confirm required fields are present and correct                       | TODO   | —        |
| Spot-check 2 ADR files to confirm `semantic-links` shape matches the ADR template                        | TODO   | —        |
| Pick 3 top-level docs files; verify each `related-artifacts` entry resolves to a real path in the repo   | TODO   | —        |
| Pick 2 pairs of `docs/` files that reference each other; verify the `related-artifacts` bidirectionality | TODO   | —        |
| Confirm `linter all` passes on the final state of all modified files                                     | TODO   | —        |
