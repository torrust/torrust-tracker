---
semantic-links:
  skill-links:
    - write-markdown-docs
  related-artifacts:
    - docs/issues/closed/2699-2482-gate-udp-server-test-environment/ISSUE.md
    - docs/issues/closed/2699-2482-gate-udp-server-test-environment/manual-verification-evidence.md
    - docs/adrs/20261021190318_share_workspace_test_support_through_testing_features.md
---

# Implementation Retrospective — Gate the `udp-server` test environment (#2699)

## Purpose

Record the reusable lessons of #2699, the first server environment of #2482 to move behind a `testing` feature, for rows 5 to 7. This is a blameless review of the approach; it does not replace the acceptance review in the [spec](ISSUE.md).

## Outcome

`src/testing/` of `udp-server` is compiled only with the non-default `testing` feature (`testing = ["dep:torrust-tracker-swarm-coordination-registry"]`, the registry crate an optional dependency, the only direct dependency only that module used). The package's own targets enable it through a path-only self dev-dependency, `axum-health-check-api-server` enables it from its `[dev-dependencies]`, and `deny.toml` lists `torrust-tracker-udp-server` as its own wrapper. Test counts equal the baseline in every command compared ([evidence V2](manual-verification-evidence.md)); the `Shipped Features` check is green on the real tree and red when the feature reaches a shipped package ([V4](manual-verification-evidence.md)). The existing example stays on the test environment and a new production-style example starts the server like the binary ([V7](manual-verification-evidence.md)).

## What Went Well

1. The row 3 pattern transferred unchanged: manifest, self dev-dependency, `deny.toml` wrapper and consumer entry in two small commits, with no surprise from `cargo deny` or clippy's `redundant_pub_crate`.
2. The maintainer settled the example question (keep it, add a production-style one, label both) before the spec was approved, so the implementation had no open design question.
3. The red/green steps are cheap to script: the consumer without its feature entry fails with two `E0433` errors, and the `Shipped Features` check goes red with the feature on a `[dependencies]` edge or in the `default` chain.
4. The independent pre-PR review compared the new example with the production startup and found three places where it did not match what its header claimed.

## What Changed During Implementation

- The first gating commit left out `Cargo.lock`: the path-only self dev-dependency adds one line there. It showed up as a modified file after the consumer's tests ran, and the unpushed commit was amended before the next one.
- The new example was first written with the listener metrics policy hard-coded and its drain a constant, and it dropped an `io::Error` of the receive loop. The review found it against `src/bootstrap/jobs/udp_tracker_core.rs` and `udp_tracker.rs`; the example now builds both from the configuration and reports every outcome.
- The pedantic clippy `too_many_lines` limit (100) applies to examples; the first `main` had 102 lines and the pre-commit gate rejected it, so the container composition and the listener stop moved to helpers.
- The example's header said it does not use the `testing` feature, but it imports the registry crate, which the package's own targets get through the self dev-dependency. The header now says it does not use the `testing` module.

- The first push failed the `Test (Docker) (release)` job: `cargo chef prepare --external-only` strips workspace path dependencies from every manifest but, in `torrust-cargo-chef` 0.1.78, not the `[features]` entries that name them, so `testing = ["dep:torrust-tracker-swarm-coordination-registry"]` made the skeleton manifest invalid. The maintainer chose to fix the fork (options A to E are in the spec): https://github.com/torrust/cargo-chef/pull/1, version 0.1.79, published and tagged; the `Containerfile` pin was bumped and the job passed. The assistant reproduced the failure locally with the pinned version before choosing, and proved the fix on the real tree (red then green, V9) before the maintainer was asked to release.
- Two Copilot rounds on the PR found five points: two were not defects (the 2024 prelude has `Future`; taplo requires `default = [  ]`), and the checks behind both replies are in the audit. A third (the wording "dev-dependencies only") exposed a real contradiction with the example headers.

## Root Cause

"Simulates production as closely as possible" was a claim written from memory of the application code. The example was made by adapting the test environment, which has its own shortcuts (a fixed metrics policy, a constant drain), so it inherited them.

The container failure has a different cause: the verification plan listed cargo commands only, and the `Containerfile` runs a tool (`cargo chef`) that reads the manifests in its own way. Row 3 passed through it only because its optional dependency is third-party.

## Improvements for Future Work

1. A path-only self dev-dependency changes `Cargo.lock`: stage it with the manifest and the `deny.toml` entry, in the same commit.
2. When an example claims to mirror production code, read that code next to the example before the first commit, list the deliberate differences in the header, and let a reviewer check the list.
3. Count `main`'s lines in examples that compose several containers: the pedantic limit is 100.
4. An example that imports an optional dependency of the package compiles only through the self dev-dependency (as the `src/` unit tests do); a packaged `.crate` strips that edge, so say so instead of claiming the example never needs the feature.
5. The sub-second part of `date -u` stamps from one script can be non-monotonic on this host; cite the order of the script and the seconds.
6. Put the container build in the verification of every row that changes a manifest. When the change makes a workspace path dependency optional, run `cargo chef prepare --external-only` then `cargo chef cook --tests --workspace --all-features` on the tree (V9 has the script) before the first push, and check that the pinned `torrust-cargo-chef` is 0.1.79 or later. Rows 5 to 7 need this.
7. A failing check on the first push is a decision, not only a fix: reproduce it, list the options with their strengths and weaknesses in the spec, let the maintainer choose, and reopen the checkpoints that the failure makes untrue.
8. Check a pending review request before writing that no review exists: the audit entry that said so was wrong by the time it was committed and was corrected before the push.

## Avoiding Overcorrection

The evidence does not justify a shared helper for the production startup of the examples (the other server examples may still choose their own), new tooling, or a `required-features` entry (banned by the ADR).

## Evidence

- [Issue specification](ISSUE.md) and [manual verification evidence](manual-verification-evidence.md) (V1 to V8).
- Spec PR #2700 and its review audit, `docs/pr-reviews/pr-2700-review/PR-REVIEW.md`; this PR's audit, `docs/pr-reviews/pr-2711-review/PR-REVIEW.md`.
- The fork fix: https://github.com/torrust/cargo-chef/pull/1 (evidence V9 and V10).
