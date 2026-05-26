---
doc-type: issue
issue-type: task
status: draft
priority: p3
github-issue: null
spec-path: docs/issues/drafts/1669-update-all-package-readmes.md
branch: null
related-pr: null
last-updated-utc: 2026-05-18 00:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/open/1669-overhaul-packages/readme-audit.md
    - docs/issues/open/1669-overhaul-packages/EPIC.md
    - packages/
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Update all package READMEs

## Goal

Bring every package's `README.md` up to a consistent quality bar — clear title, short
description, scope summary, and usage or integration notes — so that packages are
well-documented before they are extracted to standalone repositories.

## Background

The baseline README audit (`docs/issues/open/1669-overhaul-packages/readme-audit.md`,
produced in SI-01) rated each of the 26+ packages as **good**, **minimal**, or **stub**.
Several packages have placeholder READMEs with wrong titles or no meaningful content.

This subissue is intentionally ordered **after** the rename subissues (SI-07 through SI-10)
so that all READMEs are written against the final package names, and **before** the extraction
subissues (SI-12 through SI-15) so that extracted standalone repositories launch with
good documentation from day one.

This issue is a subissue of EPIC [#1669](../open/1669-overhaul-packages/EPIC.md)
(Overhaul: Packages).

## Scope

### In Scope

- Review and update `README.md` for every package listed in
  `docs/issues/open/1669-overhaul-packages/readme-audit.md`.
- Minimum quality bar for each README:
  - Correct title (matching the final crate name after renames).
  - One-paragraph description of what the package does and what it does not do.
  - Scope summary: key public types / traits / constants.
  - Dependency context: what it depends on, what depends on it.
  - Quick-start or integration example where meaningful.
- Prioritise packages rated **stub** first, then **minimal**, then **good** (review/polish only).

### Out of Scope

- Updating `AGENTS.md`, `docs/packages.md`, or top-level `README.md` (handled in separate docs
  cleanup work).
- Writing API reference docs (that is `rustdoc`-level work, a separate concern).
- Adding new tests or code changes.

### Prerequisites

- SI-07 (align `torrust-` prefix rename) complete
- SI-08 (rename `torrust-tracker-metrics`) complete
- SI-09 (rename `torrust-tracker-clock`) complete
- SI-10 (rename `torrust-tracker-located-error`) complete

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status  | Task                                                                       | Notes / Expected Output                                                |
| --- | ------- | -------------------------------------------------------------------------- | ---------------------------------------------------------------------- |
| T1  | BLOCKED | Confirm rename subissues SI-07–SI-10 are complete                          | Blocked on SI-07, SI-08, SI-09, SI-10                                  |
| T2  | TODO    | Update all **stub**-rated package READMEs (see audit)                      | Three or more packages; titles and descriptions rewritten from scratch |
| T3  | TODO    | Update all **minimal**-rated package READMEs (see audit)                   | Expand description, add scope and dependency context                   |
| T4  | TODO    | Review all **good**-rated package READMEs for title accuracy after renames | Minor edits only (title, crate name references)                        |
| T5  | TODO    | Run `linter all` (markdownlint, cspell)                                    | Exit code `0`                                                          |

## Progress Tracking

### Workflow Checkpoints

- [ ] Spec drafted in `docs/issues/drafts/`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] Rename prerequisite subissues complete (SI-07 through SI-10)
- [ ] GitHub issue created and issue number added to this spec
- [ ] Spec moved to `docs/issues/open/` with issue number prefix
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`)
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] EPIC #1669 Active Subissues table updated to `DONE`
- [ ] Issue closed and spec moved to `docs/issues/closed/`

### Progress Log

- 2026-05-18 00:00 UTC - josecelano - Spec drafted as subissue of EPIC #1669; uses
  readme-audit.md baseline from SI-01. Ordered after renaming (SI-07–SI-10) and before
  extraction (SI-12+).

## Acceptance Criteria

- [ ] Every package under `packages/` has a `README.md` with a correct title matching its
      final crate name.
- [ ] Every package README contains at minimum a description paragraph, scope summary, and
      dependency context.
- [ ] No package is rated **stub** in a post-implementation re-audit.
- [ ] `linter all` exits with code `0` (markdownlint passes on all package READMEs).

## Verification Plan

### Automatic Checks

- `linter all`

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                                | Command / Steps                                                   | Expected Result                | Status | Evidence |
| --- | --------------------------------------- | ----------------------------------------------------------------- | ------------------------------ | ------ | -------- |
| M1  | All package READMEs have correct titles | Open each `packages/*/README.md`; verify `# <crate-name>` heading | Titles match final crate names | TODO   |          |
| M2  | No stub READMEs remain                  | Re-run readme audit tool from SI-01                               | Zero packages rated stub       | TODO   |          |
