---
schema-version: 1
doc-type: issue
issue-type: task
status: in-progress
priority: p2
epic: 1347
github-issue: 2824
spec-path: docs/issues/open/2824-1347-add-tests-primitives/ISSUE.md
branch: "2824-1347-add-tests-primitives-fixes"
related-pr: 2834
last-updated-utc: "2026-11-12 18:24"
semantic-links:
  skill-links:
    - create-issue
    - write-unit-test
    - fix-bug
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - .github/skills/dev/testing/write-unit-test/SKILL.md
    - .github/skills/dev/debugging/fix-bug/SKILL.md
    - docs/issues/open/1347-overhaul-packages-testing/EPIC.md
    - docs/issues/open/1347-overhaul-packages-testing/subissue-spec-guidelines.md
    - docs/issues/closed/2637-1347-add-tests-axum-rest-api-server/ISSUE.md
    - docs/issues/closed/1349-1347-add-tests-http-core/ISSUE.md
    - docs/issues/closed/1348-1347-add-tests-udp-core/ISSUE.md
    - docs/issues/open/2824-1347-add-tests-primitives/coverage-evidence.md
    - packages/primitives
---

<!-- skill-link: create-issue -->

# Issue #2824 - Add Tests to the Primitives Package

Parent EPIC: #1347 - Overhaul: Packages Testing

## Goal

Increase the maintainable, package-local test coverage of `torrust-tracker-primitives` by going over
every Rust source file in `packages/primitives/src/`. For each file: review the existing tests
against the `write-unit-test` skill, then analyse coverage and add unit tests for the
package-owned behavior, and record the resulting per-file coverage.

Deliver the work as a short sequence of small pull requests, each merged before the next slice
starts.

## Background

`torrust-tracker-primitives` holds the domain types that every other tracker package depends on:
`Peer` and its seeder/leecher classification, `AnnounceEvent`, `AnnouncePolicy`, `SwarmMetadata`,
`ScrapeData`, `Pagination`, `TrackerPolicy`, `PrivateMode`, `Driver`, `ServiceRole`, the
`AnnounceIntervalInSecs` bounded value, and `Readiness`. It is pure code (no I/O, no async), 2,017
source lines in 16 files, 47 unit tests and no `tests/` directory. Several modules are
deprecated re-exports kept for compatibility (EPIC #1669).

The Phase 3 assessment of EPIC #1347 ranked it first among the remaining packages: lowest unit-only
coverage of the published core crates (60.56% lines), cheap to test, and everything depends on it.
Since the package has no integration tests, the aggregate and unit-only numbers are the same.

## Baseline Coverage

Measured 2026-11-10 at `develop` commit `eaae0e487` with `cargo-llvm-cov 0.6.16` and stable
Rust 1.99.1 from a clean workspace; totals include only `packages/primitives/src/` (in-file test
modules and the `peer::fixture` test support included).

| Scope | Tests | Lines | Regions | Functions |
| --- | ---: | ---: | ---: | ---: |
| Unit-only (`--lib`) = aggregate | 47 unit | 453 / 748 (60.56%) | 712 / 1,048 (67.94%) | 91 / 159 (57.23%) |

The per-file inventory and uncovered-line groups are in [coverage-evidence.md](coverage-evidence.md).
Summary: five small modules have 0% unit-only coverage (`pagination.rs`, `policy.rs`,
`announce.rs`, `mode.rs`, most of `swarm_metadata.rs`: all defaults, constructors and serde
defaults), `peer.rs` is at 35.4% (210 of its 325 lines uncovered), and the rest are fully or
almost fully covered. Indirect coverage from other packages' tests is not accepted as evidence.

## Key Package Responsibilities

| Rank | Responsibility | Source modules | Why it matters |
| ---: | --- | --- | --- |
| 1 | Peer classification and identity | `peer.rs` | `is_seeder`/`is_leecher`/`role` decide the seeder and leecher counts of every swarm and announce response; `peer::Id` and the serializers build the REST API peer output. |
| 2 | Bounded and defaulted configuration values | `announce_interval.rs`, `announce.rs`, `policy.rs`, `mode.rs`, `pagination.rs` | Defaults and serde defaults are operator-visible behavior (a changed default silently changes the tracker); `AnnounceIntervalInSecs` guarantees protocol encodability (ADR on the tightest delivery protocol). |
| 3 | Swarm and scrape data | `swarm_metadata.rs`, `scrape.rs` | Carry the counts returned to clients; `AggregateActiveSwarmMetadata` sums swarm statistics. |
| 4 | Service identity and lifecycle flags | `service_role.rs`, `configuration_instance_id.rs`, `runtime_service_metadata.rs`, `readiness.rs`, `driver.rs` | Stable identifiers used in logs, metrics and health checks. Already well covered. |
| 5 | Wiring | `lib.rs`, `peer_id.rs`, `number_of_bytes.rs` | Declarations, deprecated re-exports, one newtype. |

## Coverage Objectives

EPIC #1347 sets qualitative, risk-based objectives rather than a numeric target.

1. **Highest** - `peer.rs`: the seeder/leecher/role decision table over `left` and `event`
   (including `left < 0` and `Stopped`), `PeerRole` parsing and display, `Id` construction from
   byte vectors and numbers, hex and client-name output, and the JSON shape of `Peer`.
2. **High** - the default values and serde defaults of `AnnouncePolicy`, `TrackerPolicy`,
   `PrivateMode` and `Pagination` (including partial JSON), and the `AnnounceIntervalInSecs`
   bounds and conversions.
3. **Medium** - `SwarmMetadata` accessors (`AggregateActiveSwarmMetadata::add_assign` is not tested because #2823 deletes the impl).
4. **Low** - already covered modules, deprecated re-exports, declaration-only code, and the
   `peer::fixture` test support.

The objective is met when every highest-risk and high package-owned decision is protected by a unit
test or has a recorded owner-based reason, not when a percentage is reached.

## Test-Technique Assessment

See the table in [coverage-evidence.md](coverage-evidence.md#test-technique-assessment-initial).
Unit tests are the only level used (pure types). Property-based testing is assessed per seam;
the proposed candidates are `AnnounceIntervalInSecs` (accepts iff `<= MAX`) and the `peer::Id`
byte round-trip. A property is added only if it protects a state that examples miss, with `quickcheck`
added as in #1348 following the `add-rust-dependency` skill.

## Scope

### In Scope

- The complete per-file inventory and baseline ([coverage-evidence.md](coverage-evidence.md)).
- Process every file through the per-file workflow below. Each file gets one concise decision record
  (about 40 lines) in `test-refactor-plans/`; no-change files may share one record.
- Add focused, deterministic unit tests for package-owned decisions. Review existing tests against
  `write-unit-test` and refactor those that have smells only when a plan says so.
- Add dev-dependencies a plan justifies (`quickcheck`), following `add-rust-dependency` and
  `cargo machete --with-metadata`.
- Documentation-comment corrections in the files under test.
- The F2 production fix (`rename_all = "lowercase"` on `PeerRole`, in `peer.rs`) with its regression
  test, in slice S1, per the `fix-bug` skill.
- Hand off production findings through the [findings procedure](#findings-during-implementation).
- A bounded mutation sample after the last test increment.
- Issue-local `lessons.md` for reusable lessons; promoting them elsewhere is a separate small PR.

### Out of Scope

- Production behavior changes in `packages/primitives/src/`, except the F2 fix (`PeerRole` JSON
  casing) planned for slice S1 and listed in In Scope.
- Tests of other packages that use these types; shared test-support reorganization
  (`peer::fixture`: to be classified as test support in the S2 `peer.rs` record; any move behind a `testing`
  feature belongs to the F5 follow-up, not to this issue); #2301 and #2361.
- Removing the deprecated re-exports (EPIC #1669).
- Coverage-percentage targets and tests added only to move line totals.
- Cross-cutting process, skill or pattern-catalog edits, and a repo-wide serialization-casing ADR (the
  implicit lowercase/snake_case enum convention is recorded as a `lessons.md` candidate, to be audited in a
  separate issue under `create-adr`).

## Architectural Decisions

- Related ADRs: `docs/adrs/20261007082938_bound_protocol_agnostic_values_by_the_tightest_delivery_protocol.md`
  (`AnnounceIntervalInSecs`) and `docs/adrs/20260527175600_keep_protocol_and_domain_types_decoupled.md`
  (`AnnounceEvent` duplicated on purpose).
- No package-local ADRs. None to create.

## Design and Ownership Review

Questions each file's plan must answer. Each was confirmed by a throwaway test or by code reading and is now
recorded in the [Findings Register](#findings-register); each file's plan only needs to check that
its tests do not pin behavior a finding reports as wrong:

- `Peer` `Ord` versus `Eq`: finding F1, fixed upstream (`Ord` and `PartialOrd` were removed from
  `Peer`; a `compile_fail` doctest guards it).
- `PeerRole` JSON casing: finding F2, fixed in slice S1.
- `ParsePeerRoleError` location: finding F3, handed off to #2804.
- `AggregateActiveSwarmMetadata::add_assign` overflow: finding F4, handed off to #2823, which deletes
  the unused impl.
- `Id::new` contract for negative numbers: finding F5, handed off (the behavior is correct and
  lossless; the design question is whether it belongs in a production crate).

## Bug-Fix Process

The issue as a whole is coverage and test-design work, not a known defect, and a defect found is
handed off through the findings procedure and fixed in its own issue under the `fix-bug` skill. The
one exception is F2 (`PeerRole` JSON casing), a confirmed defect the maintainer decided to fix here,
in slice S1: S1 follows the [fix-bug](../../../../.github/skills/dev/debugging/fix-bug/SKILL.md)
workflow and records, in `manual-verification-evidence.md`, the initial reproduction, the red and
green regression-test results and the final like-for-like recheck (or the documented infeasibility
and the strongest substitute evidence).

## Regression Test Strategy

For F2 the regression test fails before the `fix` commit and passes after it (slice S1). For each other increment choose the smallest deterministic test that
protects the package-owned behavior and record it in the plan. Never pin behavior that a handed-off
finding reports as wrong without a recorded maintainer decision: leave it uncovered with the finding
as owner until the fix merges.

## Findings During Implementation

A finding that needs a production change is handled this way (same procedure as #1348, #1349, #2637):

1. Stop the current increment at a clean point.
2. Write a draft bug spec under `docs/issues/drafts/<short-topic>/ISSUE.md` on its own branch cut from
   `develop` (never on a slice branch), with the `Bug-Fix Process` and `Regression Test Strategy`
   sections, and a git-ignored `.tmp/<issue>-<short-topic>/HANDOFF.md`.
3. Stop and tell the maintainer; he decides whether this work pauses.
4. Record a progress-log entry and a `lessons.md` row naming the draft or issue.
5. Resume after a pause: rebase, re-measure the affected rows, re-check their plans.

Non-bug findings (dead code, misleading docs) need only a `lessons.md` row. Small defects the
maintainer decides to fix in this issue are not handed off: they go to the T2a slice (S1).

### Findings Register

| ID | Finding | Evidence | Owner | Status |
| --- | --- | --- | --- | --- |
| F1 | `peer::Peer` derives `Eq`/`Hash` over all fields but implements `Ord` by `peer_id` only, so two unequal peers compare `Ordering::Equal` (violates the `Ord`/`Eq` contract). No production collection orders peers today (the swarm keys by `PeerId`), so the impact is latent. | Throwaway test, 2026-11-10: a seeder and a copy with `left = 5` give `a == b` false and `a.cmp(&b) == Equal`. | Handled by a separate bug issue and session, not fixed here (maintainer decision, 2026-11-10). Bug issue: #2798, spec PR #2800. Fixed by PR #2811 (merged 2026-11-11): `Ord` and `PartialOrd` removed from `Peer` (option A, maintainer's choice), guarded by a `compile_fail` doctest in `peer.rs`. | FIXED (upstream) |
| F3 | `ParsePeerRoleError::InvalidPeerRole` records `Location::caller()` inside `PeerRole::from_str`, which has no `#[track_caller]`, so every error reports the same line inside `peer.rs` (`peer.rs:73:27`) instead of the caller's. Adding `#[track_caller]` fixes only a direct `from_str` call: through `str::parse` the location is a line in Rust's standard library. The pattern is wider than this type: `Location::caller()` appears 65 times across the packages and `#[track_caller]` 3 times, and the located error may reach production output. | Throwaway test, 2026-11-10: `"x".parse::<PeerRole>()` reports `peer.rs:73:27`; with `#[track_caller]`, `parse` reports `core/src/str/mod.rs:2783:9` and a direct `from_str` call reports the caller's line. | Handed off to a follow-up issue that audits every use of `Location::caller()` (maintainer decision, 2026-11-10). Follow-up issue: #2804 (spec PR #2805). Nothing is fixed in this issue. | HANDED OFF |
| F4 | `AggregateActiveSwarmMetadata::add_assign` uses plain `+=` on four `u64` fields (panic in debug, wrap in release on overflow; practically unreachable). The registry and `torrent-repository-benchmarking` sum the fields by hand, so the impl may be dead code. | Code reading, 2026-11-11. | Handed off to a follow-up issue/session `aggregate-addassign` (maintainer decision, 2026-11-11): check whether the impl is used (remove it and run all tests, check git history); delete it if unused, otherwise make overflow explicit (`checked_add` + panic, test, comment). Follow-up issue: #2823. Outcome (maintainer approved, 2026-11-11): the unused `AddAssign` impl is being deleted, so the `AddAssign` lines of `swarm_metadata.rs` are not tested here. Nothing is fixed in this issue. | HANDED OFF |
| F5 | `peer::Id::new` (public, in a production crate) builds a 20-byte id from a little-endian `i128` (4 zero bytes + 16 bytes). Behavior is correct and lossless, including negatives (two's complement). But it is a test-only helper: the only callers found are `torrent-repository-benchmarking` tests, and its ids look like no real client (`get_client_name()` is `None`). It has no doc comment and no tests. | Code reading and caller search, 2026-11-11 (not a behavior defect; a design problem). | Handed off to a follow-up issue/session `peer-id-test-helper` (maintainer decision, 2026-11-11): prove it is unused in production, move it to a testing helper that generates realistic ids, decide with it whether `peer::fixture` (and `Id::new`) move behind a `testing` feature, as the ADR on shared test support asks (EPIC #2482 is closed and did not reorganize `peer::fixture`). Follow-up issue: not yet created when S2 was written (proposal `peer-id-test-helper`); fill in its number. The `Id::new` tests in `peer.rs` wait on its outcome. | HANDED OFF |
| F2 | `PeerRole` derives `Serialize` with `#[serde(rename_all_fields = "lowercase")]`, which does not apply to unit variants, so it serializes as `"Seeder"` and `"Leecher"` while `Display` and the `peer_role` metric label use `seeder` and `leecher`. No production code serializes `PeerRole` today. | Throwaway test, 2026-11-10: `serde_json::to_string(&PeerRole::Seeder)` gives `"Seeder"`. | Fixed in this issue, in slice S1 (maintainer decision, 2026-11-10): `rename_all = "lowercase"` plus a regression test, in its own `fix` commit. | FIXED through #2834 |
| F6 | `Peer::is_seeder` classifies a negative `left` as a seeder because `NumberOfBytes` is signed; the UDP announce accepts a negative `left` while HTTP rejects it. | Found writing the `peer.rs` tests (S2), 2026-11-12. | Handed off to issue #2838; its fix PR #2839 was closed unmerged, and #2850 (non-negative number of bytes by construction, spec PR #2851) will remove the case. The S2 test pins the classification of the signed domain value and is removed by the #2850 implementation. | HANDED OFF |

While F3 is open, no test pins the reported `location` of `ParsePeerRoleError`; the `peer.rs` record tests only that the error carries the rejected input (`raw_param`).

F1 is fixed: `Peer` is no longer `Ord`, so no test orders peers; equality stays over all fields and
the `should_be_comparable` test is re-checked against the new code when `peer.rs` is worked on.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`, `DEFERRED`.

| ID | Status | Task | Notes / Expected Output |
| --- | ------ | ---- | ----------------------- |
| T1 | DONE | Coverage evidence and complete module inventory | [coverage-evidence.md](coverage-evidence.md): baseline, per-file uncovered groups, property candidacy, initial hypothesis for the 16 files. Delivered with this spec (S0). |
| T2a | DONE | Fix the small defects decided in this issue (slice S1) | F2 only: F1 is fixed upstream and F3, F4, F5 are handed off to their own issues. Each fix: failing regression test first, then a separate `fix` commit, per the `fix-bug` skill. Merged before the test slices. Done: the commits `test(primitives): [#2824] pin the lowercase JSON form of PeerRole` (fails before the fix) and `fix(primitives): [#2824] serialize PeerRole in lowercase`. |
| T2 | DONE | Shared plan guidance | `test-refactor-plans/README.md`: two-phase sequence, guardrails, toolchain-pinned commands. First test slice (S2). |
| T3 | TODO | Process every ledger row through the per-file workflow | Each row ends in exactly one of: test added, no-change with reason, deferred with owner. |
| T4 | TODO | Bounded mutation assessment | `mutation-evidence.md`: configuration, survivors, reason for each unviable mutant. |
| T5 | TODO | Reconcile evidence and progress state | Ledger, plans, slice table, acceptance rows and both EPIC coverage tables agree; grep for `status: proposed` and stray `TODO` labels in finished records. |
| T6 | TODO | Final verification, acceptance review, retrospective | Automatic checks, manual verification, acceptance review, `implementation-retrospective.md` (or a log entry saying none was needed). |

After the last test-producing increment, stop for maintainer review before T4-T6 and the final PR.

## Pull-Request Slices

Each slice is one PR, merged before the next slice branch is cut from `develop`. Slicing approved by
the maintainer (2026-11-10):

| Slice | Content | Ledger rows | Branch | PR | Status |
| --- | --- | --- | --- | --- | --- |
| S0 | This specification, the T1 evidence, and the EPIC registration with the baseline | - | `2824-1347-add-tests-primitives-spec` | #2828 | DONE |
| S1 | Defect fixes decided in this issue (T2a): F2 `PeerRole` JSON casing, only (F1 fixed upstream; F3, F4, F5 handed off). Merged first, so the test slices pin the fixed behavior. | - | `2824-1347-add-tests-primitives-fixes` | #2834 | DONE |
| S2 | T2, then `peer.rs` (rank 1, the largest gap) | 1 | `2824-1347-add-tests-primitives-peer` | #2841 | IN_PROGRESS |
| S3 | Configuration defaults and bounded values: `announce.rs`, `policy.rs`, `mode.rs`, `pagination.rs`, `announce_interval.rs` | 2-6 | `2824-1347-add-tests-primitives-defaults` | - | TODO |
| S4 | `swarm_metadata.rs`, no-change decisions for the covered and wiring files, then T4-T6; the clean-up moves the spec to `docs/issues/closed/` and updates the EPIC tables | 7-16 | `2824-1347-add-tests-primitives-final` | - | TODO |

Rules: S0 to S3 use `Related to #2824`; only S4 uses `Closes #2824`. Do not stack
branches; keep at most one implementation PR open. Titles follow the repository convention, for
example `test(primitives): [#2824] cover the peer seeder and leecher classification`. Each PR
updates the ledger, plan index, slice table, the slice measurement in `coverage-evidence.md`, and the
EPIC "Latest" values. The tracker-wide manual scenarios (M1, M2) and the mutation run happen once, in S4 (S2 and S3
change tests and documentation only). S1 changes behavior, so it records its own F2 reproduction,
regression results and recheck in `manual-verification-evidence.md` before it merges. Re-measure the baseline at the start of every slice.

## Commit Points

| Task | Change set | Policy |
| --- | --- | --- |
| Spec (S0) | This specification, evidence, EPIC registration | One signed `docs(issues)` commit after approval, plus follow-ups recording the PR number and review fixes. |
| T3 | One source file: its decision record, tests and recorded coverage | One signed commit per completed file after the completed-file review. Never mix two source files. |
| Slice close | Measurement, EPIC update, records | One documentation commit per slice. |
| Review round | `docs/pr-reviews/pr-<PR>-review/PR-REVIEW.md` and fixes | Audit commit separate from fix commits, as `process-pr-review` requires. |

Conventional Commit messages, for example `test(primitives): cover peer role classification`; all
commits GPG signed. Cite commits in issue-local evidence by their unique subject, never by SHA, until
merged. If work stops mid-task, record `interruption-note.md` (next action, validation state,
uncommitted-work location) and remove it on resume.

## Per-File Workflow

T3 applies to every row, one file at a time, in the ledger order within a slice.

Each file has one decision record `test-refactor-plans/<file>.md` of about 40 lines: current state
(unit-only coverage, existing tests, hypothesis); review of existing tests (`clean` or `refactor`
with smells); uncovered lines grouped by behavior, each classified **unit**, **collaborator-owned**,
**finding**, or **unselected** (with the answer to "hard to test, owned elsewhere, or merely
unselected?"); the property-test decision; the Prose-First AAA review of every added or changed
test; and results (before/after, mutation proofs). No-change rows share one record.

Two maintainer gates per file: approve the decision record before changing tests, then review the
completed result. Intra-file steps proceed without stopping unless the record flags a risk.
Validation per file: focused `cargo test -p torrust-tracker-primitives`, `cargo +nightly fmt --all -- --check`,
`git diff --check`, and the pre-commit gate; then re-run unit-only coverage and record before and after.

## Source File Ledger

States: `PENDING`, `IN_PROGRESS`, `BLOCKED`, `DONE`. Unit-only lines are the baseline measurement.

| Order | Source module | Rank | Slice | Unit-only lines | Initial hypothesis | State |
| ---: | --- | ---: | --- | ---: | --- | --- |
| 1 | `peer.rs` | 1 | S2 | 115 / 325 | Test candidate; property candidate (`Id`) | DONE (S2; the `Id::new` and `peer::fixture` groups wait on the F5 follow-up) |
| 2 | `announce_interval.rs` | 2 | S3 | 79 / 88 | Test candidate (3 gaps); property candidate | PENDING |
| 3 | `announce.rs` | 2 | S3 | 0 / 16 | Test candidate | PENDING |
| 4 | `policy.rs` | 2 | S3 | 0 / 16 | Test candidate | PENDING |
| 5 | `mode.rs` | 2 | S3 | 0 / 8 | Test candidate | PENDING |
| 6 | `pagination.rs` | 2 | S3 | 0 / 21 | Test candidate | PENDING |
| 7 | `swarm_metadata.rs` | 3 | S4 | 3 / 18 | Test candidate; the `AddAssign` lines are not tested because #2823 deletes the impl | PENDING |
| 8 | `scrape.rs` | 3 | S4 | 28 / 28 | No change; review test breadth | PENDING |
| 9 | `service_role.rs` | 4 | S4 | 40 / 40 | No change | PENDING |
| 10 | `configuration_instance_id.rs` | 4 | S4 | 42 / 42 | No change | PENDING |
| 11 | `runtime_service_metadata.rs` | 4 | S4 | 32 / 32 | No change | PENDING |
| 12 | `readiness.rs` | 4 | S4 | 52 / 52 | No change | PENDING |
| 13 | `driver.rs` | 4 | S4 | 59 / 59 | No change; style review | PENDING |
| 14 | `number_of_bytes.rs` | 5 | S4 | 3 / 3 | No change | PENDING |
| 15 | `peer_id.rs` | 5 | S4 | no executable entries | Wiring only (deprecated) | PENDING |
| 16 | `lib.rs` | 5 | S4 | no executable entries | Wiring only | PENDING |

The 16 rows are every `.rs` file under `packages/primitives/src/`, each once.

## Progress Tracking

### Workflow Checkpoints

- [x] Read EPIC #1347, the subissue guidelines, the `create-issue` workflow and the closed specs of
      #1348, #1349 and #2637.
- [x] Clean baseline coverage measured and the complete module inventory recorded (T1).
- [x] Specification and slicing reviewed and approved by the maintainer.
- [x] GitHub issue #2824 created. Linking it as a subissue of #1347 needs the maintainer (the bot account gets 404 from the sub-issues API).
- [x] Spec-only PR (S0) merged before implementation (#2828).
- [ ] Every ledger row `DONE` with a decision record, merged through its slice.
- [ ] Maintainer review after the final test-producing increment.
- [ ] Automatic verification with toolchain-qualified evidence.
- [ ] Manual verification in `manual-verification-evidence.md`.
- [ ] Bounded mutation assessment in `mutation-evidence.md`.
- [ ] Independent reviewer reports in `agent-review-reports.md` (if used).
- [ ] Acceptance criteria and implementation retrospective.
- [ ] Issue closed; spec moved to `docs/issues/closed/`.

### Progress Log

- 2026-11-10 (time not recorded) - Claude Code (session `i1347 primitives tests`) - Draft written; baseline measured at `eaae0e487`; waiting for maintainer answers.
- 2026-11-10 (time not recorded) - Claude Code (session `i1347 primitives tests`) - Maintainer approved slicing, `quickcheck` for two seams, the `peer::fixture` and `driver.rs`/`service_role.rs` decisions. Finding F1 (`Peer` `Ord` vs `Eq`) confirmed and handed to a separate bug session.
- 2026-11-11 11:22 UTC - Claude Code (Sonnet 5.5) - Created issue #2824, moved the spec to `docs/issues/open/`, registered the subissue and baseline in EPIC #1347. S0 branch `2824-1347-add-tests-primitives-spec`.
- 2026-11-11 14:07 UTC - Claude Code (Sonnet 5.5) - S1: two regression tests for the `PeerRole` JSON casing failed before the fix (`"Seeder"`, `"Leecher"`) and pass after `rename_all = "lowercase"`; unit tests 49 passed. No other package serializes `PeerRole` to JSON.
- 2026-11-12 14:10 UTC - Claude Code (Sonnet 5.5) - S2: `peer.rs` tests (32 added, 81 unit tests), decision record `test-refactor-plans/peer.md`, 14 of 14 sampled mutants killed; unit-only lines 85.20%, regions 89.28%, functions 82.99%. F6 (negative `left`) handed to #2838 (fix PR #2839). PR #2841, round 1 fixes (Copilot's `ReadInfo` cases; the reviewer's record and commit-split findings).
- 2026-11-12 16:45 UTC - Claude Code (Sonnet 5.5) - S2 review round 1 timing (for the 14:10 entry above): the reviews arrived at 14:09 (Copilot) and 14:11 and 14:28 (da2ce7); the fixes were pushed at 16:16 after a history rewrite approved by the maintainer; round 2 (da2ce7, 16:41) asked to rebuild the history from the pre-split commits (done, keeping the case table and records as their own commits) and to correct records and the audit.
- 2026-11-12 17:53 UTC - Claude Code (Sonnet 5.5) - Correction of the entry stamped 16:45 UTC above: the rebuilt commits were made between 16:44 and 16:47 UTC and pushed at 17:26 UTC; the rebuild was done at 16:47 UTC, not at the stamp.

## Acceptance Criteria

- [ ] AC1: Complete per-file inventory with unit-only coverage, property candidacy and hypothesis.
- [ ] AC2: Every row ends in test-added, no-change-with-reason, or deferred-with-owner, with its
      decision record and recorded coverage.
- [ ] AC3: Clean, reproducible unit-only coverage evidence with toolchain-qualified results.
- [ ] AC4: Every highest-risk and high objective is protected by a unit test or has a recorded owner.
- [ ] AC5: No-change and deferral decisions enumerate their uncovered lines with per-group reasons.
- [ ] AC6: Each added or changed test has a recorded prose-first AAA review.
- [ ] AC7: Bounded mutation assessment completed, with no score target or CI gate.
- [ ] AC8: Reconciliation confirms plans, evidence, slice table and both EPIC tables agree.
- [ ] AC9: Every finding needing a production change has a draft spec, a log entry and a `lessons.md`
      row, and no test pins its reported behavior without a recorded maintainer decision.
- [ ] AC10: Merged through the planned slices; only the last PR closes the issue.
- [ ] AC11: `cargo +nightly fmt --all -- --check`, `linter all` and
      `cargo test -p torrust-tracker-primitives` pass.
- [ ] AC12: Manual verification executed and recorded.

## Verification Plan

### Automatic Checks

- `cargo llvm-cov clean --workspace`, then
  `cargo llvm-cov -p torrust-tracker-primitives --all-features --lib --json --output-path .tmp/primitives-coverage/unit.json`
  (stable Rust)
- `cargo test -p torrust-tracker-primitives` (stable Rust)
- `cargo +nightly fmt --all -- --check` (nightly rustfmt; the bare stable command is not valid evidence)
- `cargo machete --with-metadata`
- `linter all`
- `TORRUST_GIT_HOOKS_LOG_DIR=.tmp ./contrib/dev-tools/git/hooks/pre-commit.sh --format=json`
- Pre-push checks before each slice PR.

Every evidence row names the toolchain that produced it. Raw JSON stays in git-ignored `.tmp/`.

### Manual Verification

Human-oriented interaction with the built tracker, run once in S4 with an isolated configuration
(automated tests never satisfy it):

| ID | Scenario | Steps | Expected | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| M1 | Default announce interval | Start the tracker with a minimal configuration (no announce policy section); announce once and read the interval from the announce response. | The announce response carries the documented default interval (120 s). The peer cap (74) is not a response field and is protected by the unit test of the default in S3. | TODO | `manual-verification-evidence.md` (S4) |
| M2 | Seeder and leecher counts | Announce one seeder and one leecher with `tracker_client`, then scrape and call the REST API torrent endpoint. | Complete/incomplete counts and the peer JSON match the announces. | TODO | `manual-verification-evidence.md` (S4) |

### Acceptance Verification

Reviewed against observed behavior after the last slice, before the issue is closed.

| AC ID | Status | Evidence |
| ----- | ------ | -------- |
| AC1 | TODO | Inventory and T1 hypotheses in `coverage-evidence.md`. |
| AC2 | TODO | All ledger rows `DONE`; plans in `test-refactor-plans/`; per-file results in `coverage-evidence.md`. |
| AC3 | TODO | Commands, toolchains and figures in the final measurement section of `coverage-evidence.md`. |
| AC4 | TODO | Decision records of the highest and high objectives. |
| AC5 | TODO | No-change and deferral records with per-group reasons. |
| AC6 | TODO | Prose-first AAA review in each decision record. |
| AC7 | TODO | `mutation-evidence.md`. |
| AC8 | TODO | The T5 reconciliation and both EPIC tables. |
| AC9 | TODO | Draft specs, progress-log entries and `lessons.md` rows, or a recorded absence of findings. |
| AC10 | TODO | The slice table and the merged pull requests. |
| AC11 | TODO | `cargo +nightly fmt --all -- --check` (nightly Rust), `linter all`, `cargo test -p torrust-tracker-primitives` (stable Rust) output. |
| AC12 | TODO | `manual-verification-evidence.md`. |

## Risks and Trade-offs

- Types that are only `derive`d produce lines that count as uncovered but carry no decision; mitigate
  by testing documented defaults and behavior, not line totals.
- A test of a suspected defect would pin wrong behavior; mitigate with the findings procedure.
- `quickcheck` is a new dev-dependency; mitigate by requiring a recorded justification per seam.
- `peer::fixture` is compiled unconditionally and imported by six other packages, although the ADR
  `20261021190318_share_workspace_test_support_through_testing_features.md` puts shared test support
  behind a `testing` feature. EPIC #2482 (closed, `status: done`) did not reorganize it. Its lines are
  planned to be classified as test support in the `peer.rs` record (S2): no package-owned behavior, so
  no unit test by decision; the live owner of any move is the F5 follow-up (issue number to be filled in).
- `develop` moves under the work; mitigate by re-measuring at every slice start.

## Implementation Completion Review

After implementation, compare the result with this specification and create
`implementation-retrospective.md` if there were material discoveries or deviations; otherwise add a
progress-log entry saying none was needed.

## References

- Parent EPIC: https://github.com/torrust/torrust-tracker/issues/1347
- Template sub-issues: #2637, #1349, #1348 (`docs/issues/closed/`)
- Package: `packages/primitives/`
- Phase 3 assessment (coordinator, git-ignored): `.tmp/phase3-assessment.md`
