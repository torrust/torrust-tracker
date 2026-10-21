---
schema-version: 1
doc-type: issue
issue-type: task
status: draft
priority: p2
epic: 2482
github-issue: null
spec-path: docs/issues/drafts/2482-move-test-helpers-protocol-helpers/ISSUE.md
branch: "{issue-number}-2482-move-test-helpers-protocol-helpers"
related-pr: null
last-updated-utc: "2026-10-21 19:16"
semantic-links:
  skill-links:
    - create-issue
    - write-unit-test
  related-artifacts:
    - docs/adrs/20261021190318_share_workspace_test_support_through_testing_features.md
    - docs/issues/open/2482-1669-reorganize-shared-test-support/EPIC.md
    - docs/issues/open/2482-1669-reorganize-shared-test-support/test-support-inventory.md
    - docs/issues/open/1669-overhaul-packages/workspace-coupling-report-2026-10-06.md
    - packages/test-helpers/Cargo.toml
    - tests/common/mod.rs
    - deny.toml
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Move the `test-helpers` protocol helpers to their consumer

Parent EPIC: #2482 (Reorganize test support code shared between workspace packages), row 2 of its
Subissues table. Draft written by subissue 1 (#2618); execution order: after #2618 merges, before
row 8.

## Goal

Make `torrust-tracker-test-helpers` a generic leaf crate (`configuration`, `logging`, `random`) by
moving its protocol-specific `http` and `udp` helpers into the only package that uses them, the root
crate's `tests/common/`, as
[rule B of the ADR](../../../adrs/20261021190318_share_workspace_test_support_through_testing_features.md)
requires.

## Background

`test-helpers` is a dev-dependency of seven packages. Its `http` module (68 lines: `http_announce`,
`http_scrape`) and `udp` module (304 lines: `udp_announce`, `udp_scrape`, `udp_complete_download`
and three `send_invalid_connection_id*` helpers) have one consumer, the root crate's
`tests/common/mod.rs`, and pull `client-lib`, `http-protocol`, `udp-protocol`, `primitives`,
`torrust-info-hash`, `torrust-peer-id` and `url` into every dependent. This is finding 1 of the
[2026-10-06 coupling report](../../open/1669-overhaul-packages/workspace-coupling-report-2026-10-06.md).
The published `torrust-tracker-test-helpers` 3.0.0 exports only `configuration` and `random`, so the
move does not change its published API; #2583 decides any version bump.

## Scope

### In Scope

- Move the code of `packages/test-helpers/src/http.rs` and `udp.rs` into the root crate's
  `tests/common/` (module names decided during implementation), keeping the root tests'
  behaviour.
- Add the dependencies the moved code needs to the root crate's `[dev-dependencies]` and remove them
  from `test-helpers`.
- Update `deny.toml`: remove `torrust-tracker-test-helpers` from the `wrappers` lists of
  `torrust-tracker-http-protocol` and `torrust-tracker-udp-protocol`, and add the root crate to the
  lists if it now depends on a protocol crate directly. Run `cargo deny check bans`.
- Update the package descriptions that say `test-helpers` provides "mock servers" (`AGENTS.md`,
  `docs/packages.md`, `packages/AGENTS.md`) and the dependency diagram where it shows the removed
  edges; mark inventory pattern B resolved.

### Out of Scope

- The `configuration`, `logging` and `random` modules, and every other package's tests.
- Any version bump (#2583).
- The `testing` features of rows 3 to 7.

## Architectural Decisions

- Related ADRs: [`20261021190318`](../../../adrs/20261021190318_share_workspace_test_support_through_testing_features.md)
  (rule B: `test-helpers` stays generic; a protocol helper with one consumer goes in that
  consumer's `tests/common/`).
- ADRs to create: None known.

## Design and Ownership Review

The root crate's integration tests own the protocol helpers after the move; `test-helpers` owns no
protocol code. The helpers send real UDP and HTTP requests to a tracker the tests start, so the move
keeps their timeouts, socket lifetimes and ban-test sequencing unchanged: copy the code, do not
redesign it. After the first moved helper compiles and its tests pass, review that the root tests
still read as before (design-review checkpoint).

## Bug-Fix Process

Not applicable.

## Regression Test Strategy

Not applicable. The root crate's existing tests exercise the moved helpers; the move must keep
their count and results.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | TODO | Record the baseline | `cargo test -p torrust-tracker --tests` counts, `cargo tree -p torrust-tracker-test-helpers -e normal`, `cargo deny check bans` result |
| T2 | TODO | Move `http` helpers to `tests/common/` | Root tests that call them still pass |
| T3 | TODO | Move `udp` helpers to `tests/common/` | Same |
| T4 | TODO | Prune `test-helpers`' dependencies and update `deny.toml` | `test-helpers` depends only on `rand`, `configuration`, `tracing`, `tracing-subscriber`; `cargo machete` and `cargo deny check bans` pass |
| T5 | TODO | Update docs, diagram and inventory | Descriptions match; inventory pattern B marked resolved |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T2 | The `http` helpers and their root `[dev-dependencies]` | Commit after `cargo test -p torrust-tracker --tests` passes. |
| T3 | The `udp` helpers | Same. |
| T4 | `test-helpers` manifest and `deny.toml` | Commit after `cargo deny check bans` and `cargo machete` pass. |
| T5 | Docs, diagram, inventory | One commit. |

## Progress Tracking

### Workflow Checkpoints

- [ ] Folder-style spec drafted in `docs/issues/drafts/2482-move-test-helpers-protocol-helpers/ISSUE.md`
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

- 2026-10-21 19:16 UTC - AI assistant (Copilot SDK in VS Code, #2482 subissue 1) - Drafted from row 2 of
  the #2482 Subissues table and rule B of the ADR.

## Acceptance Criteria

- [ ] AC1: `torrust-tracker-test-helpers` has no dependency on `client-lib`, `http-protocol`,
      `udp-protocol`, `primitives`, `torrust-info-hash`, `torrust-peer-id` or `url`.
- [ ] AC2: The root crate's tests that used the helpers pass, with the same test count as the
      baseline.
- [ ] AC3: `cargo deny check bans` passes and `torrust-tracker-test-helpers` is in no protocol
      crate's `wrappers` list.
- [ ] AC4: Descriptions and the dependency diagram match; inventory pattern B is marked resolved.
- [ ] `linter all` exits with code `0`; pre-push checks pass.

## Verification Plan

The [#2482 default completion policy](../../open/2482-1669-reorganize-shared-test-support/EPIC.md#how-we-work-in-this-epic)
applies. This move is not an O1 move, so the without-feature check does not apply.

### Automatic Checks

- `linter all`, `cargo test -p torrust-tracker --tests`, `cargo test -p torrust-tracker-test-helpers`,
  `cargo deny check bans`, `cargo machete --with-metadata`, pre-push checks.

### Manual Verification Scenarios

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| M1 | `test-helpers` has no protocol dependency | `cargo tree -p torrust-tracker-test-helpers -e normal` before and after | After: only `rand`, `configuration`, `tracing`, `tracing-subscriber` and their transitive crates | TODO | `manual-verification-evidence.md` V1 |
| M2 | The root tests keep running | `cargo test -p torrust-tracker --tests` before and after | Same number of tests, all pass | TODO | V2 |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | TODO                   |          |
| AC2   | TODO                   |          |
| AC3   | TODO                   |          |
| AC4   | TODO                   |          |

## Risks and Trade-offs

- The root crate gains dev-dependencies on the protocol and client crates; `cargo deny` may need the
  root in a `wrappers` list. Mitigation: T4 runs the check.
- Copying 372 lines of helpers moves duplication risk to zero (one consumer) but makes the root
  `tests/common/` larger. Accepted by the ADR (rule B).

## Implementation Completion Review

- Retrospective: `Not yet assessed`
- If no retrospective is needed, add a progress-log entry saying why.

## References

- Parent EPIC: [`#2482`](../../open/2482-1669-reorganize-shared-test-support/EPIC.md); ADR above
- Inventory pattern B and the 2026-10-06 coupling report, finding 1
