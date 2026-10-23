---
schema-version: 1
doc-type: issue
issue-type: task
status: draft
priority: p2
epic: 2482
github-issue: null
spec-path: docs/issues/drafts/2482-label-optional-and-self-dependency-edges/ISSUE.md
branch: "{issue-number}-2482-label-optional-and-self-dependency-edges"
related-pr: null
last-updated-utc: "2026-10-23 07:32"
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
    - docs/packages.md
    - packages/AGENTS.md
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Label optional and self dependency edges in the coupling tool and diagram

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

- The tool reads `optional` and the features that enable each dependency from `cargo metadata`, and
  labels a normal dependency that only the `testing` feature enables as `optional (testing)`, so it
  is not counted as a runtime edge. Other optional dependencies are labelled `optional (<feature>)`.
- The tool skips a self edge (a package depending on itself).
- A unit test per case, written test-first with the `write-unit-test` skill, using small
  `cargo metadata`-shaped fixtures.
- The legend in the report ("How to read this report") and the dependency diagrams in
  `docs/packages.md` and `packages/AGENTS.md` explain the labels.

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
| T1 | TODO | Test: a self edge is skipped | Fails first, then passes |
| T2 | TODO | Test: an optional dependency enabled only by `testing` is labelled `optional (testing)` | Same |
| T3 | TODO | Test: another optional dependency is labelled `optional (<feature>)`; a required normal one is unchanged | Same |
| T4 | TODO | Update the report legend and the diagrams | Legend and diagrams explain the labels |
| T5 | TODO | Run the tool on the workspace and review the output | The regenerated output is attached to the evidence, not committed as a dated report |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T1 to T3 | One commit per reviewed test increment with its production change | Commit after the test passes and its design review is recorded. |
| T4 | Legend and diagrams | One commit. |
| T5 | Evidence | One commit. |

## Progress Tracking

### Workflow Checkpoints

- [ ] Folder-style spec drafted in `docs/issues/drafts/2482-label-optional-and-self-dependency-edges/ISSUE.md`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] GitHub issue created, linked as a sub-issue of #2482, and its number added to this spec
- [ ] Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-10-21 19:16 UTC - AI assistant (Copilot SDK in VS Code, #2482 subissue 1) - Drafted from row 8 of
  the #2482 Subissues table; the ADR chose O1, so the row is required.

## Acceptance Criteria

- [ ] AC1: The tool never reports a self edge.
- [ ] AC2: An edge enabled only by a package's `testing` feature carries the label
      `optional (testing)` and is not counted as a runtime edge; a required normal edge is
      unchanged.
- [ ] AC3: The report legend and the diagrams in `docs/packages.md` and `packages/AGENTS.md`
      describe the labels.
- [ ] AC4: The tool's own tests cover each case; `cargo test -p workspace-coupling` passes.
- [ ] `linter all` exits with code `0`; pre-push checks pass.

## Verification Plan

The [#2482 default completion policy](../../open/2482-1669-reorganize-shared-test-support/EPIC.md#how-we-work-in-this-epic)
applies; the O1-specific checks do not (this subissue moves no test code).

### Automatic Checks

- `linter all`, `cargo test -p workspace-coupling`, pre-push checks.

### Manual Verification Scenarios

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| M1 | The tool labels a scratch workspace | Run the tool on a scratch workspace shaped like the evidence's V1 and V4 workspaces (optional dependency behind `testing`, a self dev-dependency) | `optional (testing)` labels, no self edge | TODO | `manual-verification-evidence.md` V1 |
| M2 | The tool still reads this workspace | Run it on the repository | Output generated; no new label appears before rows 3 to 7 land | TODO | V2 |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | TODO                   |          |
| AC2   | TODO                   |          |
| AC3   | TODO                   |          |
| AC4   | TODO                   |          |

## Risks and Trade-offs

- A feature may enable an optional dependency for a reason other than testing. The label names the
  enabling feature, so a non-`testing` feature is shown, not hidden.
- The diagrams are hand-maintained; a later edge change can leave them stale. Mitigation: the
  last #2482 subissue regenerates the report and re-checks the diagrams.

## Implementation Completion Review

- Retrospective: `Not yet assessed`
- If no retrospective is needed, add a progress-log entry saying why.

## References

- Parent EPIC: [`#2482`](../../open/2482-1669-reorganize-shared-test-support/EPIC.md); ADR above
- Verification evidence: [`manual-verification-evidence.md`](../../closed/2618-2482-decide-test-support-rules/manual-verification-evidence.md) V1 and V4
