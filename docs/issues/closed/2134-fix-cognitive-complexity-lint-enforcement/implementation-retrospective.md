---
semantic-links:
  related-artifacts:
    - ISSUE.md
    - Cargo.toml
    - .github/workflows/testing.yaml
    - contrib/dev-tools/git/hooks/pre-commit.sh
---

# Implementation Retrospective — Issue #2134 cognitive-complexity enforcement

## Purpose

Record reusable lessons from enabling `clippy::cognitive_complexity` for the complete Torrust
Tracker Cargo workspace.

## Outcome

All workspace packages now inherit the workspace lint policy. The root `Cargo.toml` denies
`clippy::cognitive_complexity`, and the existing `linter all` invocation in pre-commit and CI
enforces it. Fourteen functions were structurally simplified without cognitive-complexity
allowances or a threshold change. Focused regression tests cover the refactored event-processing
paths.

## What Went Well

1. Enabling lint inheritance before the new gate exposed the pre-existing baseline explicitly and
   allowed each diagnostic to be fixed without weakening the policy.
2. Repeated policy trials revealed violations incrementally; focused refactors and deterministic
   receiver tests preserved cancellation, closed-channel, lagged-channel, and metrics-policy
   behavior.
3. The existing `linter all` integration made the final CI and hook enforcement path available
   without a redundant workflow step.

## What Changed During Implementation

The approved initial scope covered two violations in `swarm-coordination-registry`. As compilation
proceeded after each remediation, policy trials exposed twelve additional violations in
`tracker-core`, `http-core`, `udp-core`, `udp-server`, and the root profiling runner. The work also
found that Cargo requires the `nursery` lint group to have lower priority than an individual
overriding lint; its priority changed from `-1` to `-2`.

The final pre-push documentation build exposed twelve stale intra-doc links in unrelated existing
documentation. They were corrected in `7b75d393` to restore the required validation gate.

## Root Cause

The original investigation stopped as soon as the compiler encountered the first
cognitive-complexity failures. It therefore could not enumerate later violations that compilation
would only reach after earlier failures were fixed. The original specification also predated the
repository's folder-style issue layout and did not reserve an issue-local retrospective artifact.

## Improvements for Future Work

1. When adding a denied workspace lint, run it with `cargo clippy --workspace --all-targets
--all-features` repeatedly until the complete workspace reaches a fixed point before treating
   the violation inventory as final.
2. Start complex issue specifications in folder-style layout so validation evidence and
   retrospectives can be recorded locally without a mid-implementation migration.

## Avoiding Overcorrection

Do not add a separate CI Clippy workflow step for each new Cargo lint: `testing.yaml` and the
pre-commit hook already run `linter all`, which invokes Clippy. Do not create cross-package event
listener abstractions solely to reduce local complexity; package-local helpers retained clear
protocol and domain boundaries.

## Evidence

- [Issue specification](ISSUE.md)
- `b2733dac` — denied root cognitive-complexity policy
- `9e2a692b`, `e289ab60`, `2f6be923`, `477bd377`, `33387541`, and `0278ae00` — structural refactors
- `7b75d393` — nightly rustdoc link corrections
- 2026-09-05 pre-push JSON output — nightly formatting/check/docs and full stable tests passed
