---
doc-type: issue
issue-type: task
status: done
priority: p3
github-issue: 1898
spec-path: docs/issues/closed/1898-document-security-analysis-process.md
branch: "1898-document-security-analysis-process"
related-pr: 1899
last-updated-utc: 2026-06-10 00:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/security/analysis/README.md
    - docs/security/analysis/non-affecting/2026-06-10_containerfile-trixie-cves.md
    - docs/issues/README.md
    - Containerfile
    - docs/adrs/20260603000000_keep_unit_tests_inside_container_build.md
    - docs/skills/semantic-skill-link-convention.md
    - https://github.com/torrust/torrust-tracker/issues/1457
    - https://github.com/torrust/torrust-tracker/issues/1460
    - https://github.com/torrust/torrust-tracker/issues/1463
---

<!-- skill-link: create-issue -->

# Issue #1898 - Document security analysis process and catalog non-affecting Containerfile CVEs

## Goal

Establish a structured process for evaluating security warnings and create the initial
catalog entry documenting why the trixie-based Containerfile image CVEs do not affect us.

## Background

The VS Code Docker DX extension flags vulnerabilities in the Containerfile's three
trixie-based `FROM` images (`rust:trixie`, `rust:slim-trixie`, `gcc:trixie`). These are
upstream CVEs in Docker Official Images. Before this issue, there was no documented process
or central catalog to record such analyses, meaning every contributor seeing these warnings
would need to re-do the same investigation.

### Related prior work

This issue builds on the **Docker Security Overhaul** EPIC
([#1457](https://github.com/torrust/torrust-tracker/issues/1457)), which established
a security baseline for the Containerfile and container workflows. Previous sub-issues
include adding hadolint linting to CI
([#1460](https://github.com/torrust/torrust-tracker/issues/1460)) and evaluating the
`rust:slim-trixie` vs `rust:trixie` trade-off
([#1463](https://github.com/torrust/torrust-tracker/issues/1463)). Issue #1463 already
includes a Trivy scan of both the trixie build images and the distroless runtime,
confirming the runtime has 0 critical/high CVEs.

The current VS Code Docker DX warnings are a new signal that needs to be systematically
analyzed and cataloged, which this issue addresses by creating a permanent analysis
process and catalog.

We need:

1. A `docs/security/analysis/` folder structure with a process document.
2. The initial analysis cataloging these CVEs as non-affecting, with rationale.
3. A subfolder for non-affecting vulnerabilities so they can be looked up quickly.
4. A `.github/skills/dev/maintenance/catalog-security-vulnerabilities/` skill so AI agents
   auto-discover this process.

## Scope

### In Scope

- Create `docs/security/analysis/README.md` — index and process description.
- Create `docs/security/analysis/non-affecting/` — subfolder for non-affecting vulnerabilities.
- Create `docs/security/analysis/non-affecting/2026-06-10_containerfile-trixie-cves.md` — actual analysis.
- Create `.github/skills/dev/maintenance/catalog-security-vulnerabilities/SKILL.md` — AI agent skill.
- Update `docs/adrs/20260603000000_keep_unit_tests_inside_container_build.md` — add Security Rationale section.
- Add semantic links between ADR, security analysis, and skill convention.
- List notable CVEs, explain why non-affecting, define review cadence.

### Out of Scope

- Changing the Containerfile base images (separate concern if needed).
- Fixing the upstream CVEs (they are in Docker Official Images, not our code).
- Creating automation for vulnerability scanning (future enhancement).
- Documenting affecting vulnerabilities (none found yet).

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                    | Notes / Expected Output                                                                                  |
| --- | ------ | ------------------------------------------------------- | -------------------------------------------------------------------------------------------------------- |
| T1  | DONE   | Create `docs/security/analysis/` folder structure       | `README.md` + `non-affecting/` subfolder                                                                 |
| T2  | DONE   | Analyze trixie Containerfile CVEs                       | Document showing why they don't affect us                                                                |
| T3  | DONE   | Write the non-affecting analysis document               | `2026-06-10_containerfile-trixie-cves.md` with full rationale                                            |
| T4  | DONE   | Update ADR with security rationale                      | Added Security Rationale section to `docs/adrs/20260603000000_keep_unit_tests_inside_container_build.md` |
| T5  | DONE   | Add semantic links between all related docs             | `semantic-links` updated in ADR, security analysis, and README                                           |
| T6  | DONE   | Create security analysis skill                          | `.github/skills/dev/maintenance/catalog-security-vulnerabilities/SKILL.md`                               |
| T7  | DONE   | User reviews the draft issue spec                       | Approval before creating GitHub issue                                                                    |
| T8  | DONE   | Create GitHub issue                                     | Issue #1898 created with task+security labels                                                            |
| T9  | DONE   | Rename spec from `drafts/` to `open/` with issue number | Moved to `docs/issues/open/1898-document-security-analysis-process.md`                                   |
| T10 | DONE   | Commit and push                                         | Commit `fdee528f`, pushed, PR #1899 opened                                                               |

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted in `docs/issues/drafts/`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests)
- [ ] Manual verification scenarios executed and recorded
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-06-10 16:30 UTC - GitHub Copilot - Created security analysis skill in `.github/skills/dev/maintenance/catalog-security-vulnerabilities/`
- 2026-06-10 16:00 UTC - GitHub Copilot - Drafted issue spec and created analysis documents in `docs/security/analysis/`

## Acceptance Criteria

- [ ] AC1: `docs/security/analysis/README.md` exists with process description and template
- [ ] AC2: `docs/security/analysis/non-affecting/2026-06-10_containerfile-trixie-cves.md` exists with full analysis
- [ ] AC3: The analysis document includes: vulnerability summary, rationale for non-affecting status, future actions, and references
- [ ] AC4: `docs/adrs/20260603000000_keep_unit_tests_inside_container_build.md` has a Security Rationale section
- [ ] AC5: Semantic links are consistent between ADR, security analysis documents, and related artifacts
- [ ] AC6: `linter all` exits with code `0`
- [ ] AC7: New documents are spell-checked (no false positives)
- [ ] AC8: Documentation is updated when behavior/workflow changes
- [ ] AC9: `.github/skills/dev/maintenance/catalog-security-vulnerabilities/SKILL.md` exists with process description and semantic-links

## Verification Plan

### Automatic Checks

- `linter all`
- Spell check on new documents

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                                   | Command/Steps                                                 | Expected Result                   | Status | Evidence |
| --- | ------------------------------------------ | ------------------------------------------------------------- | --------------------------------- | ------ | -------- |
| M1  | Verify README renders correctly            | Open `docs/security/analysis/README.md` in VS Code preview    | All sections readable, links work | TODO   |          |
| M2  | Verify analysis document renders correctly | Open analysis doc in VS Code preview                          | Tables render, rationale clear    | TODO   |          |
| M3  | Verify no broken internal links            | Check all `semantic-links` and references point to real files | All refs resolve                  | TODO   |          |
| M4  | Run linters                                | `linter all`                                                  | Exit code 0                       | TODO   |          |
