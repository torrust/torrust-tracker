---
schema-version: 1
doc-type: issue
issue-type: bug
status: draft
priority: p3
epic: 1669
github-issue: null
spec-path: docs/issues/drafts/1669-coupling-tool-resolve-lib-names-and-renames/ISSUE.md
branch: "{issue-number}-1669-coupling-tool-resolve-lib-names-and-renames"
related-pr: null
last-updated-utc: "2026-10-06 16:01"
semantic-links:
  skill-links:
    - create-issue
    - fix-bug
  related-artifacts:
    - contrib/dev-tools/analysis/workspace-coupling/src/main.rs
    - contrib/dev-tools/analysis/workspace-coupling/tests/parse_imports.rs
    - docs/issues/open/1669-overhaul-packages/workspace-coupling-report-2026-10-06.md
    - docs/issues/open/1669-overhaul-packages/EPIC.md
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Coupling tool misses renamed dependencies and custom library names

Subissue of EPIC [#1669](../../open/1669-overhaul-packages/EPIC.md) (Overhaul: Packages).

## Goal

Make `contrib/dev-tools/analysis/workspace-coupling/` find the imports of every workspace
dependency, including dependencies renamed in `Cargo.toml` and packages whose library target has
a custom name.

## Background

The tool derives the Rust module of a dependency from its package name
(`name.replace('-', "_")` in `src/main.rs`). The 2026-10-06 workspace coupling report
([finding 3](../../open/1669-overhaul-packages/workspace-coupling-report-2026-10-06.md#3-the-tool-misses-renamed-crates-and-custom-library-names),
issue #2446) shows four edges reported as "No `…::` references found in source" although the
dependency is used:

| Edge                              | Why the module name differs                                                        |
| --------------------------------- | ---------------------------------------------------------------------------------- |
| `client` → `client-lib`           | Dependency renamed to `torrust-tracker-client` in `console/tracker-client/Cargo.toml` |
| `udp-server` → `client-lib`       | Dependency renamed to `torrust-tracker-client` in `packages/udp-server/Cargo.toml` |
| `test-helpers` → `client-lib`     | Library target named `torrust_tracker_client` in `packages/tracker-client/Cargo.toml` |
| `e2e-tools` → `torrust-tracker`   | Root library target named `torrust_tracker_lib`                                    |

`cargo metadata` already provides both facts: each dependency's `rename`, and each package's
library target name (`targets[]` with kind `lib`).

## Scope

### In Scope

- Resolve the module name of each dependency edge from the dependency's `rename` when present,
  otherwise from the dependency package's library target name, falling back to the package name.
- Add tests covering a renamed dependency and a custom library name.
- Regenerate the coupling report as a new dated file and confirm the four edges list imports.

### Out of Scope

- Other scan limitations (macro expansion, inactive `cfg` code) documented in the report header.
- Changing the report format.

## Architectural Decisions

- Related ADRs: None
- ADRs to create: None known

## Design and Ownership Review

Not applicable: a single pure name-resolution step changes; no processes, I/O or fixtures.

## Bug-Fix Process

Follow the `fix-bug` skill: reproduce with a failing test (a fixture edge with a renamed
dependency and one with a custom library name), fix the name resolution, and prove the test
guards the bug by reverting the fix in the working tree.

## Regression Test Strategy

- Unit tests for the module-name resolution: renamed dependency, custom library name, and the
  plain package-name default.
- Prove each test fails without the fix (mutation check described in the `write-unit-test`
  skill), then restore the fix.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                                   | Notes / Expected Output                                          |
| --- | ------ | ---------------------------------------------------------------------- | ---------------------------------------------------------------- |
| T1  | TODO   | Add failing tests for a renamed dependency and a custom library name  | Tests fail on the current tool                                   |
| T2  | TODO   | Resolve module names from `rename` and the library target name        | Tests pass; mutation check recorded                              |
| T3  | TODO   | Regenerate the coupling report                                         | The four edges list their imports; no "no references" for them   |

## Commit Points

| Task | Coherent change set                       | Commit policy                                                   |
| ---- | ----------------------------------------- | --------------------------------------------------------------- |
| T1   | Failing tests                              | Commit together with T2 so every commit passes its tests.       |
| T2   | Name-resolution fix                        | One commit with T1 after the mutation check.                    |
| T3   | New dated coupling report                  | One commit after MV1.                                           |

Use a Conventional Commit message with the issue reference and sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted in `docs/issues/drafts/`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] GitHub issue created and issue number added to this spec
- [ ] Spec moved to `docs/issues/open/` with issue number prefix
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests and pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local
      `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded
- [ ] Issue closed and spec moved to `docs/issues/closed/`

### Progress Log

- 2026-10-06 16:01 UTC - GitHub Copilot - Drafted from finding 3 of the 2026-10-06 workspace
  coupling report (#2446).

## Acceptance Criteria

- [ ] AC1: The tool resolves a dependency's module name from its `rename` or library target name.
- [ ] AC2: Tests cover a renamed dependency, a custom library name and the default, and each
      fails without the fix.
- [ ] AC3: A regenerated report lists imports for the four edges in the Background.
- [ ] AC4: `cargo test -p workspace-coupling` and `linter all` exit with code `0`.
- [ ] AC5: Manual verification scenarios are executed and documented in issue-local
      `manual-verification-evidence.md`.
- [ ] AC6: The acceptance criteria are re-reviewed after implementation.

## Verification Plan

### Automatic Checks

- `cargo test -p workspace-coupling`
- `linter all`
- Pre-push checks

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                            | Human-oriented command/steps                                                         | Expected Result                                   | Status | Evidence                                      |
| --- | ----------------------------------- | ------------------------------------------------------------------------------------ | ------------------------------------------------- | ------ | --------------------------------------------- |
| MV1 | The four edges are found            | `cargo run -p workspace-coupling -- <dated path>` and read the four edge sections    | Each lists at least one import path               | TODO   | `manual-verification-evidence.md` section MV1 |

Create `manual-verification-evidence.md` from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`
when executing these scenarios, and record the toolchain for each `cargo` command.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | TODO                   |          |
| AC2   | TODO                   |          |
| AC3   | TODO                   |          |
| AC4   | TODO                   |          |
| AC5   | TODO                   |          |
| AC6   | TODO                   |          |

## Risks and Trade-offs

- A dependency could be renamed differently by different dependents; resolution must be per
  edge, not per package.

## Implementation Completion Review

- Retrospective: `Not yet assessed`

## References

- EPIC: [`docs/issues/open/1669-overhaul-packages/EPIC.md`](../../open/1669-overhaul-packages/EPIC.md)
- Source finding: [2026-10-06 coupling report](../../open/1669-overhaul-packages/workspace-coupling-report-2026-10-06.md)
- Baseline issue: #2446
