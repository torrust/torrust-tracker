---
doc-type: implementation-retrospective
issue-spec: docs/issues/closed/2238-refactor-native-tracker-test-fixture/ISSUE.md
last-updated-utc: 2026-09-17 11:38
semantic-links:
  related-artifacts:
   - docs/issues/closed/2238-refactor-native-tracker-test-fixture/ISSUE.md
   - docs/issues/closed/2238-refactor-native-tracker-test-fixture/manual-verification-evidence.md
   - docs/refactor-plans/closed/2238-refactor-native-tracker-test-fixture/REFACTOR-PLAN.md
---

# Implementation Retrospective - Native Tracker Test Fixture Refactor

## Purpose

Record material process and design findings from implementing issue #2238 without replacing its
acceptance verification.

## Outcome

The 1,272-line fixture became a standard module root plus four cohesive collaborators:
`command.rs`, `failed_start.rs`, `health.rs`, and `output.rs`. Running lifecycle orchestration
remains in the root. Both executable-boundary consumers pass, all maintenance-task-map rows hold,
the complexity audit passed, and the eight-step pre-commit gate passed.

## What Went Well

1. Moving leaf collaborators before the failed-start subsystem kept each extraction compiling and
   made ownership changes reviewable.
2. Running both integration-test binaries after each extraction caught per-binary unused-code
   behavior while the relevant change was still small.
3. Separating pure moves from API cleanup made the final design easier to review than combining
   relocation and renaming in one diff.

## What Changed During Implementation

The repository's documented-Clippy-allow checker attempted to read a deleted Rust path from the
branch diff and failed during the first full gate. The checker was corrected to skip deleted paths,
allowing moved files to be validated normally.

The approved plan expected either a module-level failed-start allowance or a small set of
configuration-builder allowances. Removing the existing broad allowance on the entire
`cli-configuration` fixture exposed a different precise boundary: configuration scenarios do not
use six lifecycle-only fields and accessors. Narrow documented allowances now sit on those members,
while the failed-start module retains its single justified module-level allowance for the signal
binary.

## Root Cause

The planning analysis correctly identified two-binary compilation as a constraint but inferred the
final unused-code boundary from source inspection. Only compiling each binary without its broad
allowance revealed the exact member set. Separately, the allow-check tool assumed every Rust path in
a diff still existed, an assumption invalidated by a file-to-module-directory migration.

## Improvements for Future Work

1. When narrowing broad lint allowances in shared integration fixtures, remove the allowance early
   and compile every consumer to discover the exact boundary before prescribing replacement
   allowances.
2. Repository checks that inspect changed paths must handle Git status explicitly, including
   deleted and renamed files; the corrected allow checker now covers this case.
3. Keep move-only and API-cleanup commits separate when restructuring ownership-sensitive test
   fixtures.

## Avoiding Overcorrection

The evidence does not justify a generic child-process fixture framework, a repository-wide ban on
module-level allowances, or a line-count limit. The useful boundary is cohesive ownership plus
consumer-specific compilation, not file size alone.

## Evidence

- `docs/issues/closed/2238-refactor-native-tracker-test-fixture/ISSUE.md`
- `docs/refactor-plans/closed/2238-refactor-native-tracker-test-fixture/REFACTOR-PLAN.md`
- `docs/issues/closed/2238-refactor-native-tracker-test-fixture/manual-verification-evidence.md`
- `cargo test --test lifecycle-signals --test cli-configuration`
- `TORRUST_GIT_HOOKS_LOG_DIR=.tmp ./contrib/dev-tools/git/hooks/pre-commit.sh`
- Commit subject `fix(dev-tools): handle deleted Rust files in allow check`
- Commit subject `refactor(tests): [#2238] narrow fixture dead-code allowances`
