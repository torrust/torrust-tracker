---
semantic-links:
  skill-links:
    - write-markdown-docs
  related-artifacts:
    - docs/issues/open/2757-2482-gate-axum-health-check-api-server-test-environment/ISSUE.md
    - docs/issues/open/2757-2482-gate-axum-health-check-api-server-test-environment/manual-verification-evidence.md
---

# Implementation Retrospective — Gate the `axum-health-check-api-server` test environment

## Purpose

Record the two material discoveries of #2757, the last row of #2482. A blameless review; the move itself matched the spec.

## Outcome

`src/environment.rs` compiles only with the non-default `testing` feature; `torrust-tracker-configuration` is optional behind it; the package's own targets enable the feature through a path-only self dev-dependency with its `deny.toml` wrapper entry. Plain package tests keep the baseline (15 unit + 11 integration), `cargo check --lib` passes with no features, and the container recipe, Shipped Features, deny, machete and packaging checks pass (evidence V1 to V5).

## What Went Well

1. The row 4 to 6 recipe (gate, optional dependency, self dev-dependency, wrapper in the same commit, chef check) applied unchanged.
2. Running `cargo check --lib` with no features immediately after the gate found the first discovery below before any test run.
3. The AC8 check read the generated report and the diagram, and compared the diagram with the report's `optional (testing)` labels.

## What Changed During Implementation

1. The no-features library build of the package was already broken at `ca8efcd4d`, before this change: `CheckReport` derives `Serialize` and `Deserialize` on a `url::Url`, but the package's own `url = "2.5.4"` had no `serde` feature. Only crates reached through `[dev-dependencies]` (`udp-server`, `rest-api-client`) request the feature, so plain `cargo test -p` passed. It surfaced here only because the first no-features `cargo check --lib` of the package was run after the gate. (The first version of this document, the inventory and the PR description wrongly blamed the dropped `configuration` edge; da2ce7's review corrected it with a measurement at the merge base.) The package now declares `url` with `features = ["serde"]`.
2. The AC8 check found test-only edges still drawn in the dependency diagram that rows 4 and 5 had made optional (`axum-http --> clock`, `info-hash`, `swarm`; `udp-srv --> swarm`), plus six `test-helpers` arrows that no manifest declares. They are removed here, together with the health check's `config` edge.

## Root Cause

1. Rows 4 to 6 and T1's baseline ran `cargo test`, which builds a package with its dev-dependencies' features; nothing built the package alone without features. Feature unification through dev-dependencies hid a missing declaration.
2. The diagram rule ("test-only edges are not drawn") was applied per row by hand, and nothing compared the arrows with the manifests.

## Improvements for Future Work

1. Run `cargo check -p <package> --lib` with no features as part of the baseline (T1), not only after the change, so a pre-existing defect is not confused with one the move causes.
2. Compare the diagram's arrows with `cargo metadata` (the script in this folder's `evidence/` does it) when a row touches the diagram.

## Avoiding Overcorrection

A general lint for undeclared features is not justified by one case. The no-features `cargo check` already catches it.

## Evidence

- [Spec](ISSUE.md) and [manual verification evidence](manual-verification-evidence.md) (V2, V6).
- Commit `refactor(health-check-api): [#2757] gate the test environment behind a testing feature`.
