---
semantic-links:
  skill-links:
    - write-markdown-docs
  related-artifacts:
    - docs/issues/closed/2681-2482-gate-tracker-core-test-modules-and-mocks/ISSUE.md
    - docs/issues/closed/2681-2482-gate-tracker-core-test-modules-and-mocks/manual-verification-evidence.md
    - docs/adrs/20261021190318_share_workspace_test_support_through_testing_features.md
---

# Implementation Retrospective — Gate `tracker-core`'s test modules and mocks (#2681)

## Purpose

Record the reusable lessons of #2681, the first row of #2482 that adds a `testing` feature to a real crate, for rows 4 to 7 (the server environments). This is a blameless review of the approach; it does not replace the acceptance review in the [spec](ISSUE.md).

## Outcome

The three helper modules are private `#[cfg(test)]` modules. `MockTorrentMetricsStore`, the one mock other packages use, is generated behind the non-default `testing` feature of `tracker-core` (`testing = ["dep:mockall"]`, optional `mockall`); the other three mocks use `#[cfg_attr(test, automock)]`. `tracker-core`'s own targets use a path-only self dev-dependency, `http-core` and `udp-core` enable the feature from their `[dev-dependencies]`, and `deny.toml` lists `torrust-tracker-core` as its own wrapper. A default `tracker-core` build has no direct `mockall` dependency and no `Mock*` item or helper module in its API; test counts are equal in all 12 commands compared ([evidence V2](manual-verification-evidence.md)); the `Shipped Features` check is green on the real tree and red when the feature reaches a shipped package ([V4](manual-verification-evidence.md)).

## What Went Well

1. A discarded prototype before the spec was approved found the two side effects of the self dev-dependency (the `deny.toml` wrapper and clippy's `redundant_pub_crate`) while changing the spec was still cheap.
2. Counting the consumers of each mock separately, instead of per rule, showed that three of the four mocks are not shared test support. The maintainer narrowed the rule to what other crates need.
3. Test-first for the feature gate: with the mock gated and no consumer entries, both consumers failed with `E0432`; adding their `[dev-dependencies]` entries turned them green ([V3](manual-verification-evidence.md)).
4. The positive control for the "no mock in the shipped binary" check (an `rlib` built with the feature has thousands of matching strings) showed the check can fail.

## What Changed During Implementation

- Rule D.1 of the ADR said `tracker-core` applies the feature to all four traits. The per-mock consumer count contradicted it, so the maintainer chose the narrower scope and the ADR got an amendment note instead of a rewrite.
- `develop` moved between the spec audit and the implementation (`tracker-core` and other packages changed). The facts were re-verified before editing, and nothing in the spec changed.
- The spec's scenario M5 first expected the coupling report to show `mockall`; the report lists only workspace packages. Copilot's review of the spec PR caught it, and it now checks the `[normal]` and `[dev]` entries and `cargo metadata`.
- The body of the spec-only PR #2682 said "The implementation PR follows after this one merges and will close #2681". The merge tool copies the PR body into the merge commit, so GitHub applied the closing keyword and closed #2681 when #2682 merged; the #2482 coordinator reopened it. The `open-pull-request` skill already warns that a keyword closes the issue even in a sentence about the future, and its grep would have flagged the line; it was not run on that body.
- The `create-issue` skill says a spec-only PR's frontmatter `branch:` is the `-spec` name; earlier rows used the base name. A review finding fixed it.

## Root Cause

The ADR was written from the inventory's pattern-level view ("four traits, one shared"), not from a per-item consumer count, and the draft inherited its sentence. The effects of a path-only self dev-dependency on `cargo deny` and clippy were not in the ADR's verified Cargo behaviour (V1 to V4c), which covered Cargo only.

## Improvements for Future Work

1. Rows 4 to 7 add the self dev-dependency to servers: add the package to its own `wrappers` entry in `deny.toml` (if it has one) in the same commit as the manifest change, because the pre-commit gate runs `cargo deny check bans` on every commit.
2. Before applying a rule to a group of items, count each item's consumers; apply the rule to the items that other packages use and the private rule to the rest.
3. Run the dependency-graph side of a spec (`cargo deny`, `cargo machete`, clippy) in a throwaway prototype before the spec is approved.
4. Never write `close`, `closes`, `fixes` or `resolves` followed by an issue number in a spec-only PR body, even in a sentence about the future (say "the implementation PR will follow" without the keyword). Run the `open-pull-request` skill's keyword grep on every spec-PR body before creating or editing it; the implementation PR body is the only one that carries `Closes #N`.
5. For a scenario that inspects a tool's output, read the tool's code or run it first; do not assume what it lists.
6. When a check needs an artifact (a binary) that another command can rebuild, copy the artifact right after building it and test the copy; record its checksum before and after (finding F4 of PR #2688).

## Avoiding Overcorrection

The evidence does not justify new tooling, a rule that every mock gets the feature, deleting the two unused mocks (a separate cleanup), or putting `testing` in `[package.metadata.docs.rs]`.

## Evidence

- [Issue specification](ISSUE.md) and [manual verification evidence](manual-verification-evidence.md) (V0 to V7).
- The ADR amendment of 2026-10-28 and the inventory notes for patterns C and D.
- Spec PR #2682 and its review audit, `docs/pr-reviews/pr-2682-review/PR-REVIEW.md`.
