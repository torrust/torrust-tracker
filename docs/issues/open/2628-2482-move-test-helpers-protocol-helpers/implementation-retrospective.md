---
semantic-links:
  skill-links:
    - write-markdown-docs
  related-artifacts:
    - docs/issues/open/2628-2482-move-test-helpers-protocol-helpers/ISSUE.md
    - docs/issues/open/2628-2482-move-test-helpers-protocol-helpers/manual-verification-evidence.md
    - docs/adrs/20261021190318_share_workspace_test_support_through_testing_features.md
---

# Implementation Retrospective - Move the `test-helpers` protocol helpers to their consumer

## Purpose

Record evidence-based process improvements discovered while implementing issue #2628 (subissue 2
of #2482). This is a blameless review of the implementation approach; it does not replace the
acceptance-criteria verification.

## Outcome

`torrust-tracker-test-helpers` is a generic leaf crate: its direct dependencies fell from 11 to
`rand`, `configuration`, `tracing` and `tracing-subscriber`. The `http` and `udp` helpers live in
the root crate's `tests/common/` (`http_requests.rs`, `udp_requests.rs`) with the names the root
tests already used, and the root tests give the same 194 passing results in 15 binaries as before.
`deny.toml` bans `client-lib`, `tracker-core`, `swarm-coordination-registry` and `events` outside
their dependents, and the new bans were shown to fire. Evidence:
[`manual-verification-evidence.md`](manual-verification-evidence.md).

## What Went Well

1. The spec's "copy the code, do not redesign it" rule kept each move small: `git mv` kept the
   history, the root tests needed no edit, and the per-binary test results were identical before
   and after.
2. Checking the new bans by adding the forbidden dependency back (V3) showed that the entries
   work, instead of only showing that the tree passes.

## What Changed During Implementation

1. **The commit points could not hold.** The spec planned the dependency pruning and the
   `deny.toml` changes for T4. The pre-commit gate runs `cargo machete --with-metadata` and
   `cargo deny check bans` on every commit, so the `http` move failed machete until `test-helpers`
   dropped `torrust-info-hash`, `http-protocol` and `url`, and the `udp` move failed `cargo deny`
   until the root joined the `udp-protocol` wrappers. Each move commit carries the changes it makes
   necessary; T4 shrank to the four new ban entries.
2. **The spec's wrapper list for `client-lib` named a crate the check never reaches.** `cargo deny
   check bans` runs on the graph rooted at the root crate, so the console `torrust-tracker-client`
   (and `persistence-benchmark`, for `tracker-core`) printed an unmatched-wrapper warning. The
   spec derived the lists from `cargo metadata` over the whole workspace. `cargo deny --workspace
   check bans` fails for the two protocol crates on this branch, because the console client depends
   on them and is in neither wrapper list, so the root-graph check is the supported one. The entries list only the dependents inside that graph, and a comment in
   `deny.toml` says so.
3. **The moved helpers raised `dead_code`.** In `test-helpers` they were `pub`, so no binary
   warned. In a private test module each integration-test binary compiles its own copy and uses
   only some helpers, which fails the workspace's `-D warnings`. Each moved file carries a
   module-level `#![allow(dead_code, reason = ...)]`. This departs on purpose from the rule of the
   closed #2238 spec (`docs/issues/closed/2238-refactor-native-tracker-test-fixture/ISSUE.md`):
   module-level allows only where a whole module is unused by one binary, item-level elsewhere. Most
   binaries leave one of the two modules wholly unused (the banning tests use no `http` helper, the
   completed-downloads persistence test no `udp` helper, `scaffold` neither), and the item-level form would need an
   allow on all 16 functions of the two files, because each private helper becomes dead when its
   public callers do. The existing `tests/common/configuration.rs` and `native_tracker/` keep their
   item-level allows, which this change leaves as they are.
4. **`develop` moved under the spec.** #2623 added `http_complete_download` after the spec was
   refreshed; the reviewer found it in the spec PR. No code was affected, because the move carries
   whatever the module holds.
5. **AC1 reads as stronger than it is.** "No dependency on `primitives`" is true for direct
   dependencies only: `configuration` depends on `primitives`, so `test-helpers` still reaches it
   transitively, which rule B.1 allows.

## Root Cause

The spec planned the work as if each task could be committed in isolation, but the repository's
pre-commit gate checks dependencies and layer bans on every commit, so a move and the manifest and
`deny.toml` changes it forces are one change. It also took the wrapper lists from the whole
workspace graph when the check uses the root crate's graph, and it phrased AC1 without saying
"direct".

## Improvements for Future Work

1. When a spec splits a move from its manifest and `deny.toml` follow-ups, say that every commit
   must pass the pre-commit gate and that a dependency or wrapper change belongs to the commit that
   makes it necessary. The #2482 rows that move code between crates (3 to 7) should do the same.
2. Derive `deny.toml` wrapper lists by running `cargo deny check bans` and removing what it reports
   as unmatched, not from `cargo metadata` alone, because the check only sees the root crate's
   graph.
3. Write dependency acceptance criteria as "no direct dependency" and name the command
   (`cargo tree -p <crate> -e normal --depth 1`).

## Avoiding Overcorrection

The evidence does not justify a workspace-wide `dead_code` rule for test modules or a change to how
`cargo deny` is invoked. The first is a per-module choice with a stated reason, and the second
would fail on `develop` for reasons outside this issue.

## Evidence

- [`ISSUE.md`](ISSUE.md) and [`manual-verification-evidence.md`](manual-verification-evidence.md)
- Commits `refactor(test-helpers): [#2628] move the http helpers to the root tests`,
  `refactor(test-helpers): [#2628] move the udp helpers to the root tests` and
  `chore(deny): [#2628] ban the crates test-helpers must not depend on`
- Spec PR #2629 review audit: [`PR-REVIEW.md`](../../../pr-reviews/pr-2629-review/PR-REVIEW.md)
