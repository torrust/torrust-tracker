---
schema-version: 1
doc-type: issue
issue-type: task
status: in-review
priority: p2
epic: 2482
github-issue: 2628
spec-path: docs/issues/open/2628-2482-move-test-helpers-protocol-helpers/ISSUE.md
branch: "2628-2482-move-test-helpers-protocol-helpers"
related-pr: null
last-updated-utc: "2026-10-24 19:37"
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

# Issue #2628 - Move the `test-helpers` protocol helpers to their consumer

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

`test-helpers` is a dev-dependency of seven packages. Its `http` module (92 lines: `http_announce`,
`http_complete_download`, `http_scrape`) and `udp` module (304 lines: `udp_announce`, `udp_scrape`, `udp_complete_download`
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
  `torrust-tracker-udp-protocol` list (it is already in the `http-protocol` list) because the moved
  code makes it a direct dev-dependent. `torrust-tracker-client-lib` has no entry today, so add one
  whose `wrappers` are its current dependents (`torrust-tracker` after the move,
  `torrust-tracker-udp-server`, `torrust-tracker-axum-http-server` and the console
  `torrust-tracker-client`), or `test-helpers` could depend on it again unnoticed.
  Maintainer decision (2026-10-23): also add entries for `torrust-tracker-core` and the other
  domain crates `test-helpers` must not depend on (`swarm-coordination-registry`, `events`), so ADR rule B.1 is checked for those crates and `client-lib`; for the other workspace crates (for example `primitives`, `rest-api-client`) B.1 stays a convention, as rule B.3 already says. Each entry's `wrappers` lists the
  crates that depend on it today, taken from `cargo metadata` during T4, never `test-helpers`.
  Run `cargo deny check bans`.
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
| T1 | DONE | Record the baseline | `cargo test -p torrust-tracker --tests` counts, `cargo tree -p torrust-tracker-test-helpers -e normal`, `cargo deny check bans` result. Baseline: 15 binaries, 194 tests; 11 direct dependencies; `bans ok` with two unmatched-wrapper warnings. See evidence V1 to V3. |
| T2 | DONE | Move `http` helpers to `tests/common/` | `http_requests.rs`; root tests pass (194). The commit also drops the `test-helpers` dependencies only `http` used (see Commit Points). |
| T3 | DONE | Move `udp` helpers to `tests/common/` | `udp_requests.rs`; same results. The commit also drops the dependencies only `udp` used and changes the protocol crates' `deny.toml` wrappers. |
| T4 | DONE | Prune `test-helpers`' dependencies and update `deny.toml` | `test-helpers` depends only on `rand`, `configuration`, `tracing`, `tracing-subscriber`; `cargo machete` and `cargo deny check bans` pass. T4's own commit adds the four ban entries. |
| T5 | DONE | Update docs, diagram and inventory | Descriptions match in `AGENTS.md`, `docs/packages.md`, `packages/AGENTS.md` and `docs/testing.md`; the `packages/AGENTS.md` diagram shows layers, not dependency edges, so it needed no change; inventory pattern B marked resolved. |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T2 | The `http` helpers and their root `[dev-dependencies]` | Commit after `cargo test -p torrust-tracker --tests` passes. |
| T3 | The `udp` helpers | Same. |
| T4 | `test-helpers` manifest and `deny.toml` | Commit after `cargo deny check bans` and `cargo machete` pass. |
| T5 | Docs, diagram, inventory | One commit. |

The pre-commit gate runs `cargo machete` and `cargo deny check bans` on every commit, so the T2 and T3
commits also carry the manifest and `deny.toml` changes their move makes necessary; the T4 commit
holds the four new ban entries. See the retrospective.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2482-move-test-helpers-protocol-helpers/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created, linked as a sub-issue of #2482, and its number added to this spec
- [x] Spec-only PR merged into `develop` before implementation
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests, and pre-push checks)
- [x] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [x] Evidence-based implementation completion review recorded
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-10-21 19:16 UTC - AI assistant (Copilot SDK in VS Code, #2482 subissue 1) - Drafted from row 2 of
  the #2482 Subissues table and rule B of the ADR.
- 2026-10-22 18:10 UTC - AI assistant (Copilot SDK in VS Code, #2482 subissue 1) - Changed after PR #2624's review: the deny.toml scope adds a ban entry for `torrust-tracker-client-lib` (it has none) and asks whether the domain crates need entries.
- 2026-10-23 15:50 UTC - AI assistant (Copilot SDK in VS Code, #2482 subissue 2) - Refreshed against `develop` at `64e6783b0`: every fact re-verified (seven dev-dependents; `http.rs` 68 and `udp.rs` 304 lines; root `tests/common/mod.rs` the only consumer; `cargo deny check bans` passes). Made the `deny.toml` scope concrete: the root joins the `udp-protocol` wrappers, and the new `client-lib` entry lists four wrappers.
- 2026-10-23 16:23 UTC - AI assistant (Copilot SDK in VS Code, #2482 subissue 2) - Maintainer (josecelano) approved the spec and decided that the `deny.toml` scope also adds ban entries for `torrust-tracker-core`, `swarm-coordination-registry` and `events`. Created issue #2628, linked it as a subissue of #2482 (`parent_issue_url` verified) and moved the spec to `docs/issues/open/`.
- 2026-10-23 16:49 UTC - AI assistant (Copilot SDK in VS Code, #2482 subissue 2) - Review round 1 on PR #2629 (da2ce7, four non-blocking findings): fixed the Progress Log placement, the check scope wording, the frontmatter branch/status and the drafted-checkpoint path; see `docs/pr-reviews/pr-2629-review/PR-REVIEW.md`.
- 2026-10-24 17:15 UTC - AI assistant (Copilot SDK in VS Code, #2482 subissue 2) - Review round 6 on PR #2629 (da2ce7, finding F14): refreshed the Background against `develop` at `8f8b4d714`. #2623 added the public helper `http_complete_download`, so `http.rs` is 92 lines (`http_announce`, `http_complete_download`, `http_scrape`) and T2 moves all three; the other facts still hold (seven dev-dependents, `udp.rs` 304 lines, the root `tests/common/mod.rs` as the only consumer).
- 2026-10-24 19:37 UTC - AI assistant (Copilot SDK in VS Code, #2482 subissue 2) - Implemented on branch `2628-2482-move-test-helpers-protocol-helpers` after PR #2629 merged: the `http` and `udp` helpers moved to `tests/common/http_requests.rs` and `udp_requests.rs`, `test-helpers` now depends only on `rand`, `configuration`, `tracing` and `tracing-subscriber`, `deny.toml` gained entries for `client-lib`, `tracker-core`, `swarm-coordination-registry` and `events`, and the descriptions and inventory were updated. Baseline and final: 15 binaries, 194 root tests passed; pre-push passed. Evidence: `manual-verification-evidence.md`; lessons: `implementation-retrospective.md`.

## Acceptance Criteria

- [x] AC1: `torrust-tracker-test-helpers` has no dependency on `client-lib`, `http-protocol`,
      `udp-protocol`, `primitives`, `torrust-info-hash`, `torrust-peer-id` or `url`.
- [x] AC2: The root crate's tests that used the helpers pass, with the same test count as the
      baseline.
- [x] AC3: `cargo deny check bans` passes, `torrust-tracker-test-helpers` is in no protocol
      crate's `wrappers` list, and `torrust-tracker-client-lib`, `torrust-tracker-core`,
      `swarm-coordination-registry` and `events` have ban entries that do not name `test-helpers`.
- [x] AC4: Descriptions and the dependency diagram match; inventory pattern B is marked resolved.
- [x] `linter all` exits with code `0`; pre-push checks pass.

## Verification Plan

The [#2482 default completion policy](../../open/2482-1669-reorganize-shared-test-support/EPIC.md#how-we-work-in-this-epic)
applies. This move is not an O1 move, so the without-feature check does not apply.

### Automatic Checks

- `linter all`, `cargo test -p torrust-tracker --tests`, `cargo test -p torrust-tracker-test-helpers`,
  `cargo deny check bans`, `cargo machete --with-metadata`, pre-push checks.

### Manual Verification Scenarios

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| M1 | `test-helpers` has no protocol dependency | `cargo tree -p torrust-tracker-test-helpers -e normal` before and after | After: only `rand`, `configuration`, `tracing`, `tracing-subscriber` and their transitive crates | DONE | `manual-verification-evidence.md` V1 |
| M2 | The root tests keep running | `cargo test -p torrust-tracker --tests` before and after | Same number of tests, all pass | DONE | V2 |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | DONE                   | [`manual-verification-evidence.md`](manual-verification-evidence.md) V1: `cargo tree -p torrust-tracker-test-helpers -e normal --depth 1` lists only `rand`, `configuration`, `tracing`, `tracing-subscriber` (direct dependencies; `primitives` stays reachable through `configuration`, allowed by rule B.1); `cargo machete --with-metadata` clean (run by the implementing agent) |
| AC2   | DONE                   | V2: 15 binaries, 194 tests, per-binary results identical to the baseline; the pre-push full test suite passed (run by the implementing agent) |
| AC3   | DONE                   | V3: `cargo deny check bans` passes on the real tree and fails when `test-helpers` depends on each of the four banned crates; `test-helpers` is in no `wrappers` list (checked by the implementing agent: `grep -n test-helpers deny.toml` prints only the two comment lines of the new section) |
| AC4   | DONE                   | The four descriptions and inventory pattern B are updated in the T5 commit; `linter all` (markdown links) passed in the pre-commit gate (checked by the implementing agent) |

## Risks and Trade-offs

- The root crate gains dev-dependencies on the protocol and client crates; `cargo deny` may need the
  root in a `wrappers` list. Mitigation: T4 runs the check.
- Copying 372 lines of helpers moves duplication risk to zero (one consumer) but makes the root
  `tests/common/` larger. Accepted by the ADR (rule B).

## Implementation Completion Review

- Retrospective: created, [`implementation-retrospective.md`](implementation-retrospective.md): the commit points that the pre-commit gate
  merged into the move commits, the `deny.toml` wrapper lists that must follow the root crate's graph, the `dead_code`
  allowance and the ambiguity of "no dependency" in AC1.
- If no retrospective is needed, add a progress-log entry saying why.

## References

- Parent EPIC: [`#2482`](../../open/2482-1669-reorganize-shared-test-support/EPIC.md); ADR above
- Inventory pattern B and the 2026-10-06 coupling report, finding 1
