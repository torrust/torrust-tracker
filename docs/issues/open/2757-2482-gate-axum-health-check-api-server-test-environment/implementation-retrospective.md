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

1. Dropping the `configuration` edge broke the build without the feature: `CheckReport` derives `Serialize` and `Deserialize` on a `url::Url`, but the package's own `url = "2.5.4"` had no `serde` feature. The feature had come only through `torrust-tracker-configuration` (Cargo feature unification). The package now declares `url` with `features = ["serde"]`.
2. The AC8 check found four test-only edges still drawn in the dependency diagram that rows 4 and 5 had made optional (`axum-http --> clock`, `info-hash`, `swarm`; `udp-srv --> swarm`). They are removed here, together with the health check's `config` edge.

## Root Cause

1. The audit per row looks at which dependencies production code uses, not at which features of the remaining dependencies another dependency enables for free. A crate that removes a heavy edge can lose a feature it never declared.
2. The diagram rule ("test-only edges are not drawn") was applied per row by hand. Rows 4 and 5 predate the rule's wording and nothing compared the diagram with the labelled edges.

## Improvements for Future Work

1. When an environment move removes a dependency edge, build the package with no features (`cargo check -p <package> --lib`) before the tests; this is already in the spec and found the defect.
2. A check that compares the diagram's edges with the report's `optional (testing)` edges would keep the diagram honest. Not done here (the maintainer chose no counter and no new tool work for this row).

## Avoiding Overcorrection

A general lint for undeclared transitive features is not justified by one case. The no-features `cargo check` already catches it.

## Evidence

- [Spec](ISSUE.md) and [manual verification evidence](manual-verification-evidence.md) (V2, V6).
- Commit `refactor(health-check-api): [#2757] gate the test environment behind a testing feature`.
