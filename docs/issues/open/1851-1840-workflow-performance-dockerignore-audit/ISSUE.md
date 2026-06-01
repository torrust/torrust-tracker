---
doc-type: issue
issue-type: task
status: open
priority: p2
github-issue: 1851
spec-path: docs/issues/open/1851-1840-workflow-performance-dockerignore-audit/ISSUE.md
branch: "1851-workflow-performance-dockerignore-audit"
related-pr: null
last-updated-utc: 2026-05-29 00:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .dockerignore
    - .gitignore
    - Containerfile
    - .github/workflows/container.yaml
    - docs/issues/open/1840-improve-pr-workflow-performance-epic/EPIC.md
    - docs/issues/open/1841-1840-workflow-performance-baseline-analysis/benchmark-results-baseline.md
---

<!-- skill-link: create-issue -->

# Issue #1851 - Audit .dockerignore to minimize Docker build context

## Goal

Ensure the Docker build context sent to BuildKit is as small as possible by
auditing `.dockerignore` against `.gitignore` and the actual container
contents, then adding any paths that are tracked by git but not needed in any
Containerfile stage.

## Background

Every file **not** excluded from the build context is transferred to the
BuildKit daemon before the build starts. Large contexts increase transfer time,
create unnecessary cache invalidation when unrelated files change (e.g. docs,
CI config, dev tools), and add noise to layer diffs.

The baseline analysis (`#1841`) already identified one concrete case: the
`.tmp/` directory (AI agent hook logs + benchmark cargo isolation dirs) was
included in the build context and triggering cache misses. That entry was added
to `.dockerignore` as a quick fix. A systematic audit may reveal further
candidates.

Additionally, the `Containerfile` stages that perform a full source copy
(`COPY . /build/src`) are particularly sensitive to context size: any file not
excluded will invalidate those layers' cache whenever it changes, even if the
change is irrelevant to the build (e.g. updating a doc or a YAML config file).

## Scope

### In Scope

- Compare `.dockerignore` with `.gitignore` and identify paths present in the
  repo that are not needed inside any Containerfile stage.
- Inspect the actual build context size (before and after) and the contents
  transferred using `docker build --progress=plain` or `docker buildx du`.
- Optionally build a local image and inspect the filesystem at each stage to
  verify no needed files are accidentally excluded.
- Add all safe exclusions to `.dockerignore` and measure the reduction in
  context size and any improvement in layer cache hit rate.
- Document which files are **intentionally** kept (e.g. `share/`, `contrib/`)
  and why.

### Out of Scope

- Restructuring the `COPY` instructions in the Containerfile to copy only
  subsets of the source tree (that belongs to a separate issue).
- Changes to the build stages or caching strategy beyond `.dockerignore` edits.
- Changes to `.gitignore`.

## Known Candidates

Based on an initial comparison of `.dockerignore` and `.gitignore`, the
following tracked paths are not currently excluded from the Docker build context
and appear unlikely to be needed in any Containerfile stage:

| Path                                                      | Reason likely safe to exclude                  |
| --------------------------------------------------------- | ---------------------------------------------- |
| `.github/`                                                | CI config — not referenced by any stage        |
| `.vscode/`                                                | Editor config — not referenced by any stage    |
| `.gitignore`                                              | Git metadata — not referenced by any stage     |
| `.git-blame-ignore`                                       | Git metadata — not referenced by any stage     |
| `docs/`                                                   | Documentation — not referenced by any stage    |
| `codecov.yaml`                                            | CI config — not referenced by any stage        |
| `compose.*.yaml`                                          | Compose files — not referenced by any stage    |
| `cspell.json` / `project-words.txt`                       | Spell-check config — not used inside container |
| `rustfmt.toml`                                            | Formatter config — not used inside container   |
| `.markdownlint.json` / `.taplo.toml` / `.yamllint-ci.yml` | Linter config — not used inside container      |
| `AGENTS.md`                                               | Agent instructions — not used inside container |
| `README.md` / `NOTICE` / `SECURITY.md` / `LICENSE`        | Project docs — not used inside container       |
| `contrib/dev-tools/`                                      | Dev tooling — not used inside container        |

> These are candidates only. Each must be confirmed safe before being added.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                            | Notes / Expected Output                                                                                                                                                                               |
| --- | ------ | ----------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | DONE   | Measure current build context size              | `printf 'FROM scratch\nCOPY . /ctx' \| docker buildx build --progress=plain --no-cache -f - .` → **4.75 MB**                                                                                          |
| T2  | DONE   | Cross-reference `.dockerignore` vs `.gitignore` | All tracked root-level paths classified; new exclusions: `SECURITY.md`, `LICENSE`, `packages/AGENTS.md`, `src/AGENTS.md`, `contrib/dev-tools/` (minus `su-exec/`).                                    |
| T3  | DONE   | Inspect container stage contents                | Containerfile reviewed stage-by-stage; `contrib/dev-tools/su-exec/` retained via `!` negation rule; all other `COPY` targets verified included.                                                       |
| T4  | DONE   | Add safe exclusions to `.dockerignore`          | `.dockerignore` reorganized into labeled sections; intentionally included paths documented in header comment block.                                                                                   |
| T5  | DONE   | Measure context size and cache behaviour after  | Same command as T1 after clean `docker buildx prune -f` → **4.64 MB** (−110 kB, −2.3%). Cache invalidation surface reduced: `contrib/dev-tools/` changes no longer trigger source-stage cache misses. |

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted in `docs/issues/drafts/`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [ ] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation
- [x] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded (status + evidence)
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

Append one line per meaningful update.

- 2026-05-29 00:00 UTC - GitHub Copilot - Drafted .dockerignore audit issue from baseline analysis findings - draft file created
- 2026-06-01 00:00 UTC - GitHub Copilot - GitHub issue #1851 created; spec moved from drafts/ to open/
- 2026-06-01 00:00 UTC - GitHub Copilot - Implemented on branch 1851-workflow-performance-dockerignore-audit: reorganized .dockerignore with section comments, added SECURITY.md, LICENSE, packages/AGENTS.md, src/AGENTS.md, contrib/dev-tools/ (su-exec/ retained). Context: 4.75 MB → 4.64 MB (−110 kB).

## Acceptance Criteria

- [x] AC1: Current Docker build context size is measured and recorded.
- [x] AC2: All tracked repo paths are classified as needed / excluded / intentionally kept with a rationale.
- [x] AC3: `.dockerignore` is updated with all confirmed-safe exclusions.
- [ ] AC4: No Containerfile stage is broken by the new exclusions (all CI checks pass).
- [x] AC5: Build context size is re-measured and the reduction is documented.
- [x] AC6: Intentionally included paths are documented with inline comments in `.dockerignore`.
- [ ] `linter all` exits with code `0`
- [ ] All CI checks pass for changed files
- [ ] Manual verification scenarios are executed and documented (status + evidence)
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behaviour

## Verification Plan

Define verification before implementation starts and execute it before closing the issue.

### Automatic Checks

- `linter all`
- All CI checks pass for changed `.dockerignore` and Containerfile

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario               | Command/Steps                                                                                  | Expected Result                                                                                  | Status | Evidence                                           |
| --- | ---------------------- | ---------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------ | ------ | -------------------------------------------------- |
| M1  | Measure context before | `printf 'FROM scratch\nCOPY . /ctx' \| docker buildx build --progress=plain --no-cache -f - .` | Baseline context size recorded.                                                                  | DONE   | `#3 transferring context: 4.75MB`                  |
| M2  | Verify no stage breaks | Full cold `docker build --target release .`                                                    | Build completes successfully; all stages produce expected artifacts.                             | TODO   | pending CI                                         |
| M3  | Measure context after  | Same command as M1 after `docker buildx prune -f` and `.dockerignore` update                   | Context size smaller than baseline; reduction documented.                                        | DONE   | `#3 transferring context: 4.64MB` (−110 kB, −2.3%) |
| M4  | Cache stability check  | Run warm baseline twice: `run-container-baseline.sh` without `--cold`                          | Layer cache hit rates are stable or improved; no unexpected misses due to excluded file changes. | TODO   | pending full build                                 |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                                                                                                                                               |
| ----- | ---------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------ |
| AC1   | DONE                   | `printf 'FROM scratch\nCOPY . /ctx' \| docker buildx build --progress=plain --no-cache -f - .` → `#3 transferring context: 4.75MB`                     |
| AC2   | DONE                   | All 32 root-level tracked paths reviewed; classification documented in T2 row above and in `.dockerignore` header block                                |
| AC3   | DONE                   | `.dockerignore` updated: added `SECURITY.md`, `LICENSE`, `packages/AGENTS.md`, `src/AGENTS.md`, `/contrib/dev-tools/` + `!/contrib/dev-tools/su-exec/` |
| AC4   | TODO                   | Pending CI run                                                                                                                                         |
| AC5   | DONE                   | Same command after `docker buildx prune -f` → `#3 transferring context: 4.64MB` (−110 kB, −2.3%)                                                       |
| AC6   | DONE                   | `.dockerignore` header block lists all intentionally included paths with rationale                                                                     |
