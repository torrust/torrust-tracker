---
schema-version: 1
doc-type: issue
issue-type: task
status: planned
priority: p2
epic: 2482
github-issue: 2645
spec-path: docs/issues/open/2645-2482-label-optional-and-self-dependency-edges/ISSUE.md
branch: "2645-2482-label-optional-and-self-dependency-edges"
related-pr: 2648
last-updated-utc: "2026-10-25 08:17"
semantic-links:
  skill-links:
    - create-issue
    - write-unit-test
  related-artifacts:
    - docs/adrs/20261021190318_share_workspace_test_support_through_testing_features.md
    - docs/issues/open/2482-1669-reorganize-shared-test-support/EPIC.md
    - docs/issues/closed/2618-2482-decide-test-support-rules/manual-verification-evidence.md
    - contrib/dev-tools/analysis/workspace-coupling/src/main.rs
    - contrib/dev-tools/analysis/workspace-coupling/src/lib.rs
    - docs/media/packages/dependencies-workspace-packages.md
---

<!-- skill-link: create-issue -->

# Issue #2645 - Label optional and self dependency edges in the coupling tool and diagram

Parent EPIC: #2482 (Reorganize test support code shared between workspace packages), row 8 of its
Subissues table. Draft written by subissue 1 (#2618). Execution order: after row 2 and before row
3, the first move that gates test code behind a `testing` feature.

## Goal

Make the workspace coupling tool and the dependency diagrams show an edge that exists only for the
`testing` feature, or only as a dev edge, as test-only, so that no test-only edge appears as a
runtime edge once rows 3 to 7 move test support behind `testing` features.

## Background

The ADR chose the `testing` feature (O1) with a path-only self dev-dependency for the package's own
targets. The verification (evidence V1 and V4) shows what `cargo metadata` reports for that shape:

```text
a -> c kind=None optional=True features=[]           (an optional dependency enabled by `testing`)
b -> a kind=dev  optional=False features=['testing'] (a consumer's dev-dependency)
o1c -> o1c kind=dev features=['testing']             (the self dev-dependency: an edge from a package to itself)
```

The coupling tool (`contrib/dev-tools/analysis/workspace-coupling`) labels an edge by its kind
(normal, dev, build). Its source never reads the `optional` field of a dependency, and no code
handles a self edge, so a self dev-dependency may be listed as a package depending on itself (T1
checks it). Without this change the regenerated coupling report and the diagrams keep showing
test-only edges as runtime edges, so #2482's AC8 cannot be checked.

## Scope

### In Scope

- The tool reads each dependency's `optional` field and each package's `features` table from `cargo
  metadata` (the dependency's own `features` list names what the consumer enables, not what enables
  the dependency, so the table is the source). A normal dependency that is optional and enabled only
  by the `testing` feature (`testing = ["dep:c"]` or `["c"]`) is labelled `optional (testing)` and is
  not counted as a runtime edge. Another optional dependency is labelled `optional (<feature>)`
  (several features are joined with a comma and a space); an optional dependency no feature enables is labelled
  `optional`.
- The tool skips a self edge (a package depending on itself) everywhere: details, leaf detection and
  the workspace dependency counts.
- A unit test per case, written test-first with the `write-unit-test` skill, using small
  `cargo metadata`-shaped fixtures.
- The legend in the report ("How to read this report") and the hand-maintained dependency diagram
  `docs/media/packages/dependencies-workspace-packages.md` (which draws direct normal edges only)
  explain the labels: an `optional (testing)` edge is not drawn as a normal edge, and the diagram
  note says so. `docs/packages.md` and `packages/AGENTS.md` hold layer diagrams without dependency
  edges, so they need no change.

### Out of Scope

- Moving any test support code (rows 3 to 7).
- Regenerating the coupling report (the last subissue, row 7, does it to check AC8).
- Editing existing dated coupling reports.

## Architectural Decisions

- Related ADRs: [`20261021190318`](../../../adrs/20261021190318_share_workspace_test_support_through_testing_features.md)
  (consequence: optional and self edges need a label; this subissue is required because the ADR
  chose O1).
- ADRs to create: None known.

## Design and Ownership Review

Not applicable: no child process, I/O readiness or shared fixture. The tool's input is `cargo
metadata` JSON and its output is Markdown, as today; the change adds fields and labels.

## Bug-Fix Process

Not applicable.

## Regression Test Strategy

Not applicable.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | DONE | Test: a self edge is skipped | Fails first, then passes |
| T2 | DONE | Test: an optional dependency enabled only by `testing` is labelled `optional (testing)` | Same |
| T3 | DONE | Test: another optional dependency is labelled `optional (<feature>)`; a required normal one is unchanged | Same |
| T4 | DONE | Update the report legend and `docs/media/packages/dependencies-workspace-packages.md` | Legend and diagram note explain the labels |
| T5 | DONE | Run the tool on the workspace and review the output | The regenerated output is attached to the evidence, not committed as a dated report |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T1 to T3 | One commit per reviewed test increment with its production change | Commit after the test passes and its design review is recorded. |
| T4 | Legend and diagrams | One commit. |
| T5 | Evidence | One commit. |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/open/2645-2482-label-optional-and-self-dependency-edges/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer (approved by the maintainer through `ask_user` on 2026-10-25)
- [x] GitHub issue created, linked as a sub-issue of #2482 (`parent_issue_url` verified), and its number added to this spec
- [x] Spec-only PR merged into `develop` before implementation (PR #2646)
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests, and pre-push checks) (`linter all` through the pre-commit gate on each commit, `cargo test -p workspace-coupling` and `pre-push.sh` by the AI assistant)
- [x] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [x] Evidence-based implementation completion review recorded
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-10-21 19:16 UTC - AI assistant (Copilot SDK in VS Code, #2482 subissue 1) - Drafted from row 8 of
  the #2482 Subissues table; the ADR chose O1, so the row is required.
- 2026-10-25 06:55 UTC - AI assistant (Copilot SDK in VS Code, #2482 subissue 8) - Refreshed against `develop`, pointed the diagram to `docs/media/packages/dependencies-workspace-packages.md`, created #2645, linked it under #2482 and moved the spec to `open/`. The maintainer approved the spec.
- 2026-10-25 08:15 UTC - AI assistant (Copilot SDK in VS Code, #2482 subissue 8) - Implemented T1 to T5 after the spec PR #2646 merged: the tool labels optional edges and skips self edges, the legend and diagram note explain it, the evidence and a retrospective are recorded (the retrospective is needed: the first version mislabelled the `testing = ["c"]` shorthand). AC2 is read as "never labelled `normal`", because the tool has no runtime-edge counter (see the completion review).

## Acceptance Criteria

- [x] AC1: The tool never reports a self edge.
- [x] AC2: An edge enabled only by a package's `testing` feature carries the label
      `optional (testing)` and is not counted as a runtime edge; a required normal edge is
      unchanged.
- [x] AC3: The report legend and `docs/media/packages/dependencies-workspace-packages.md` describe
      the labels and draw no `optional (testing)` edge as a normal edge.
- [x] AC4: The tool's own tests cover each case; `cargo test -p workspace-coupling` passes.
- [x] `linter all` exits with code `0`; pre-push checks pass (validated by the AI assistant on 2026-10-25: the pre-commit gate on each commit and `pre-push.sh` on the final code).

## Verification Plan

The [#2482 default completion policy](../../open/2482-1669-reorganize-shared-test-support/EPIC.md#how-we-work-in-this-epic)
applies; the O1-specific checks do not (this subissue moves no test code).

### Automatic Checks

- `linter all`, `cargo test -p workspace-coupling`, pre-push checks.

### Manual Verification Scenarios

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| M1 | The tool labels a scratch workspace | Run the tool on a scratch workspace shaped like the evidence's V1 and V4 workspaces (optional dependency behind `testing`, a self dev-dependency) | `optional (testing)` labels, no self edge | DONE | `manual-verification-evidence.md` V1 |
| M2 | The tool still reads this workspace | Run it on the repository | Output generated; no new label appears before rows 3 to 7 land (verified on `develop`: no workspace optional or self edge exists today) | DONE | V2 |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | DONE                   | `tests/edge_labels.rs` (two self-edge tests); V1: `o1c` has no entry and is a leaf |
| AC2   | DONE                   | `tests/edge_labels.rs` (the `optional (testing)` tests, including the `dep:` and implicit-feature syntax and a rename); V1: `a` -> `c` is `optional (testing)`, `b` -> `c` stays `normal` |
| AC3   | DONE                   | The legend lines in `src/main.rs` (V2 diff) and the note in `docs/media/packages/dependencies-workspace-packages.md`; no `optional (testing)` edge exists, so none is drawn |
| AC4   | DONE                   | V3: `cargo test -p workspace-coupling`, 35 tests pass; 14 of the 15 new ones failed to compile before the change and the two implicit-feature ones failed before the fix |

## Risks and Trade-offs

- A feature may enable an optional dependency for a reason other than testing. The label names the
  enabling feature, so a non-`testing` feature is shown, not hidden.
- The diagrams are hand-maintained; a later edge change can leave them stale. Mitigation: the
  last #2482 subissue regenerates the report and re-checks the diagrams.

## Implementation Completion Review

Reviewed by the AI assistant against the acceptance criteria and the evidence on 2026-10-25; the
maintainer reviews it in the pull request.

- Scope: the tool change is in `contrib/dev-tools/analysis/workspace-coupling` only (`src/lib.rs`,
  `src/main.rs`, `tests/edge_labels.rs`), plus the diagram note. No test code moved, no report
  regenerated, no dated report edited.
- Design: the classification lives in the library (`Package::edge_kind`, `EdgeKind`,
  `Package::workspace_dependencies`) so it is unit-tested without running `cargo metadata`; the new
  serde fields default, so the existing fixtures still deserialize (a test covers it).
- Interpretation of "not counted as a runtime edge" (AC2): the tool has no runtime-edge counter,
  only a per-package "Workspace deps" total that already includes dev and build edges. An
  `optional (testing)` edge is therefore never labelled `normal` and sorts after the normal edges;
  the total is unchanged. If row 7 wants a runtime-only count it must add one.
- Known limits: a feature entry `dep:c` or `c` or `c/feature` enables the dependency; the weak form
  `c?/feature` does not. The implicit feature Cargo adds for an optional dependency
  (`c = ["dep:c"]`) is ignored, so a dependency only that feature enables is labelled `optional`.
  The legend lines were committed with the tool change rather than with the diagram note.
- Retrospective: created, [`implementation-retrospective.md`](implementation-retrospective.md): the
  first version mislabelled the `testing = ["c"]` shorthand because the test fixtures lacked the
  implicit feature Cargo adds to the real metadata.

## References

- Parent EPIC: [`#2482`](../../open/2482-1669-reorganize-shared-test-support/EPIC.md); ADR above
- Verification evidence: [`manual-verification-evidence.md`](../../closed/2618-2482-decide-test-support-rules/manual-verification-evidence.md) V1 and V4
