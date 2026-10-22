---
schema-version: 1
doc-type: issue
issue-type: task
status: draft
priority: p2
epic: 2482
github-issue: null
spec-path: docs/issues/drafts/2482-gate-tracker-core-test-modules-and-mocks/ISSUE.md
branch: "{issue-number}-2482-gate-tracker-core-test-modules-and-mocks"
related-pr: null
last-updated-utc: "2026-10-21 19:16"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/adrs/20261021190318_share_workspace_test_support_through_testing_features.md
    - docs/issues/open/2482-1669-reorganize-shared-test-support/EPIC.md
    - docs/issues/open/2482-1669-reorganize-shared-test-support/test-support-inventory.md
    - packages/tracker-core/src/lib.rs
    - packages/tracker-core/src/databases/traits/torrent_metrics.rs
    - packages/http-core/src/services/scrape.rs
    - packages/udp-core/src/services/scrape.rs
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Gate `tracker-core`'s test modules and mocks

Parent EPIC: #2482 (Reorganize test support code shared between workspace packages), row 3 of its
Subissues table. Draft written by subissue 1 (#2618). Execution order: after rows 2, 8 and 9. It is
the first O1 move, so it needs row 8's labels and row 9's container build first.

## Goal

Apply rules C and D of the
[ADR](../../../adrs/20261021190318_share_workspace_test_support_through_testing_features.md) to
`tracker-core`: make its crate-private test helpers private, and put its four `#[automock]`
database mocks behind a `testing` feature so `mockall` leaves its normal dependencies and the
`Mock*` types leave its default public API.

## Background

Patterns C and D of the inventory:

- `lib.rs` declares `pub mod peer_tests;` and `pub mod test_helpers;`, and `whitelist/mod.rs`
  declares `pub mod test_helpers;`. Their content is `#[cfg(test)]` (for `peer_tests`, the inner
  attribute `#![cfg(test)]`), so nothing ships, but the declarations add empty public modules to
  the API. Nothing outside the crate imports them.
- The four traits in `databases/traits/` (`whitelist.rs`, `auth_keys.rs`, `torrent_metrics.rs`,
  `schema.rs`) carry an unconditional `#[automock]`. `mockall` is therefore a normal dependency.
  `MockTorrentMetricsStore` is imported by `#[cfg(test)]` modules of `http-core`
  (`services/announce.rs`, `services/scrape.rs`) and `udp-core` (`services/announce.rs`,
  `services/scrape.rs`), so it is shared test support and `cfg_attr(test, automock)` alone would
  break those four modules.

## Scope

### In Scope

- Rule C: declare the three helper modules `#[cfg(test)] mod`, not `pub mod`.
- Rule D: add the `testing` feature (not in `default`) with `testing = ["dep:mockall"]`, make
  `mockall` an optional normal dependency, and generate the mocks with
  `#[cfg_attr(feature = "testing", automock)]` on all four traits, keeping the existing
  lint attributes on `auth_keys.rs` (which cite `automock`) valid.
- Rule A points 3, 4 and 6 for the feature: a path-only self dev-dependency (no `version`), the
  `[dev-dependencies]` entries `torrust-tracker-core = { ..., features = ["testing"] }` in `http-core`
  and `udp-core`, a comment on the feature saying it is test support outside semver, and no
  `testing` in `[package.metadata.docs.rs]`.
- Update the inventory (patterns C and D resolved) and the dependency diagram if it shows
  `mockall`.

### Out of Scope

- The mocks used only inside their own crate by other packages (`events`, `swarm-coordination-registry`),
  which already use `#[cfg_attr(test, automock)]` or a dev-dependency.
- The server environments (rows 4 to 7) and the protocol helpers (row 2).
- Any version bump (#2583).

## Architectural Decisions

- Related ADRs: [`20261021190318`](../../../adrs/20261021190318_share_workspace_test_support_through_testing_features.md)
  (rules C and D, ownership map: `tracker-core` owns its database mocks).
- ADRs to create: None known.

## Design and Ownership Review

`tracker-core` owns the mocks of its traits; `http-core` and `udp-core` use them only through
`[dev-dependencies]` with `features = ["testing"]`. No lifetimes or I/O change. The consumers'
test modules stay as they are.

## Bug-Fix Process

Not applicable.

## Regression Test Strategy

Not applicable. The existing unit tests of the three packages are the regression suite; their
count must not drop.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | TODO | Record the baseline | `cargo tree -p torrust-tracker-core -e normal,features` (shows `mockall`); test counts of `cargo test -p torrust-tracker-core`, `-p torrust-tracker-http-core`, `-p torrust-tracker-udp-core` with `--all-targets --all-features` and without `--all-features` |
| T2 | TODO | Rule C: make the three helper modules private | Build and tests pass |
| T3 | TODO | Rule D: gate the four mocks and add the feature, the self dev-dependency and the consumers' entries | Plain `cargo test -p` passes for the three packages |
| T4 | TODO | Update the inventory, diagram and docs | Patterns C and D marked resolved |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T2 | The three module declarations | Commit after `cargo test -p torrust-tracker-core` passes. |
| T3 | Feature, optional dependency, `cfg_attr`, self dev-dependency, consumers' entries | Commit after the three packages' tests pass without `--all-features`. |
| T4 | Docs, diagram, inventory | One commit. |

## Progress Tracking

### Workflow Checkpoints

- [ ] Folder-style spec drafted in `docs/issues/drafts/2482-gate-tracker-core-test-modules-and-mocks/ISSUE.md`
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

- 2026-10-21 19:16 UTC - AI assistant (Copilot SDK in VS Code, #2482 subissue 1) - Drafted from row 3 of
  the #2482 Subissues table and rules C and D of the ADR. The scope includes the consumers' entries
  because `MockTorrentMetricsStore` is used by `http-core` and `udp-core` (inventory correction).

## Acceptance Criteria

- [ ] AC1: `cargo tree -p torrust-tracker-core -e normal,features` shows no `mockall`, and the
      `Mock*` types and the three helper modules are not in a default build's public API.
- [ ] AC2: `cargo test -p torrust-tracker-core`, `-p torrust-tracker-http-core` and
      `-p torrust-tracker-udp-core` pass without `--all-features`, and each package's test count
      equals the baseline.
- [ ] AC3: `cargo package --workspace --no-verify` (stable toolchain) passes; the self
      dev-dependency has no `version`.
- [ ] AC4: Inventory patterns C and D are marked resolved.
- [ ] `linter all` exits with code `0`; pre-push checks pass.

## Verification Plan

The [#2482 default completion policy](../../open/2482-1669-reorganize-shared-test-support/EPIC.md#how-we-work-in-this-epic)
applies. This is an O1 move: build the package without the feature, run `cargo test -p <package>`
without `--all-features`, and check the test count.

### Automatic Checks

- `linter all`, the three packages' tests with and without `--all-features`,
  `cargo +stable package --workspace --no-verify`, `cargo machete --with-metadata`,
  `cargo deny check bans`, pre-push checks.

### Manual Verification Scenarios

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| M1 | The production build has no test dependency | `cargo tree -p torrust-tracker-core -e normal,features` before and after | `mockall` absent after | TODO | `manual-verification-evidence.md` V1 |
| M2 | Each package's tests run with a plain command | `cargo test -p torrust-tracker-core`, `-p torrust-tracker-http-core`, `-p torrust-tracker-udp-core` (no `--all-features`) | Test counts equal the baseline | TODO | V2 |
| M3 | A consumer still gets the mock | `cargo test -p torrust-tracker-http-core -- scrape` | The tests that use `MockTorrentMetricsStore` run and pass | TODO | V3 |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | TODO                   |          |
| AC2   | TODO                   |          |
| AC3   | TODO                   |          |
| AC4   | TODO                   |          |

## Risks and Trade-offs

- `cfg_attr` ordering with `#[async_trait]` and the lint attributes may change what
  `mockall` generates. Mitigation: run the three packages' tests after each trait.
- Other consumers of the mocks may exist outside `http-core` and `udp-core`; T1 re-greps
  `Mock(Database|WhitelistStore|AuthKeyStore|TorrentMetricsStore|SchemaMigrator)` across the
  workspace before editing.

## Implementation Completion Review

- Retrospective: `Not yet assessed`
- If no retrospective is needed, add a progress-log entry saying why.

## References

- Parent EPIC: [`#2482`](../../open/2482-1669-reorganize-shared-test-support/EPIC.md); ADR above
- Inventory patterns C and D
