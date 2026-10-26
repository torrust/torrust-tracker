---
semantic-links:
  skill-links:
    - write-markdown-docs
  related-artifacts:
    - docs/issues/closed/2645-2482-label-optional-and-self-dependency-edges/ISSUE.md
    - docs/issues/closed/2645-2482-label-optional-and-self-dependency-edges/manual-verification-evidence.md
    - docs/adrs/20261021190318_share_workspace_test_support_through_testing_features.md
---

# Implementation Retrospective - Label optional and self dependency edges

## Purpose

Record evidence-based process improvements discovered while implementing issue #2645 (subissue 8
of #2482). This is a blameless review of the implementation approach; it does not replace the
acceptance-criteria verification.

## Outcome

The coupling tool labels an optional normal dependency with the features that enable it (so an edge
only `testing` enables is `optional (testing)`) and skips a package's dependency on itself. The
report legend and the dependency diagram note explain both. Evidence:
[`manual-verification-evidence.md`](manual-verification-evidence.md).

## What Went Well

1. The spec asked for the features table as the source of truth and for self-edge handling in one
   place, so the logic lives in the library and each case is a small unit test.
2. The manual run on a scratch workspace compared the baseline tool with the new one on the same
   input, which shows what changed instead of what the new tool prints.

## What Changed During Implementation

The first implementation passed all its tests but mislabelled the shorthand `testing = ["y"]` as
`optional (testing, y)`. The spec and the unit test fixtures described the explicit `dep:y` syntax,
and the hand-made `features` table in the test did not contain the implicit feature `y = ["dep:y"]`
that Cargo adds. Running `cargo metadata` on a real scratch workspace, to record the evidence,
showed it. A fix and two tests with the real table shape followed.

Copilot's review of PR #2648 then found a second gap of the same kind: the tool looked only at the
features that list the dependency, so `default = ["testing"]` with `testing = ["dep:c"]` still
showed `optional (testing)` although a default build enables the dependency. The tool now follows
feature references, with a stop on cycles. The spec said "enabled only by `testing`" without saying
what "enabled by" means for a feature that another feature enables.

The fix for that gap hid every feature named like the dependency, not only the one Cargo generates.
A second review round (Copilot and da2ce7) showed that an explicit `c = ["dep:c", "extras"]` beside
`testing = ["dep:c"]` then read `optional (testing)`, the test-only label, although `--features c`
enables the dependency in a normal build. The tool now drops a same-name feature only in the shape
Cargo generates.

## Root Cause

The test fixtures were written from the spec's description of the metadata, not from the output of
`cargo metadata`, and the spec defined "enabled by a feature" only for a feature that lists the
dependency. The spec's Background quoted the dependency lines of the ADR evidence but not the
`features` table, which was the new input.

## Improvements for Future Work

1. When a tool starts to read a new part of `cargo metadata`, copy the real JSON for each
   syntax variant into the test fixture (or record it in the spec) before writing the test.
2. Run the tool on a scratch workspace for each syntax variant the ADR allows, not only the one
   the evidence used. Cargo 1.99.1 still accepts both `testing = ["dep:c"]` and `testing = ["c"]`
   at edition 2024 (the second adds the implicit feature `c = ["dep:c"]`), so the rows 3 to 7 that
   add `testing` features may meet either, and a `default` feature that includes `testing` must be
   treated as a runtime edge.
3. When a spec defines a label by a condition on a graph (here, features enabling features), say
   whether the condition is direct or transitive.
4. When a tool hides a name because Cargo generates it, write down the exact rule that makes the
   name generated (here: the feature is exactly `["dep:key"]`, no other feature names `dep:key`) and
   test the look-alike that the manifest can write.

## Avoiding Overcorrection

A full `cargo metadata` JSON snapshot test is not justified: the fields the tool reads are few,
and the real-shape fixtures of the new tests are enough.

## Evidence

- [`ISSUE.md`](ISSUE.md) and [`manual-verification-evidence.md`](manual-verification-evidence.md)
- Commits `feat(workspace-coupling): [#2645] label optional edges and skip self edges`,
  `fix(workspace-coupling): [#2645] ignore the implicit feature of an optional dependency` and
  `fix(workspace-coupling): [#2645] follow feature references when labelling an optional edge`
- Spec PR #2646: no review findings, so no audit record exists. Implementation PR #2648 review
  audit: [`PR-REVIEW.md`](../../../pr-reviews/pr-2648-review/PR-REVIEW.md)
