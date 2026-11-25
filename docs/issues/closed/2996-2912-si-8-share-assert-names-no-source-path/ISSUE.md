---
schema-version: 1
doc-type: issue
issue-type: task
status: done
priority: p3
epic: 2912
github-issue: 2996
spec-path: docs/issues/closed/2996-2912-si-8-share-assert-names-no-source-path/ISSUE.md
branch: "2996-share-assert-names-no-source-path-spec"
related-pr: 3041
last-updated-utc: "2026-11-25 16:55"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .claude/skills/create-issue/SKILL.md
    - docs/issues/open/2912-remove-source-locations-from-errors/EPIC.md
    - docs/adrs/20261118045443_errors_do_not_record_source_locations.md
    - docs/adrs/20261021190318_share_workspace_test_support_through_testing_features.md
    - packages/test-helpers/src/lib.rs
    - packages/http-protocol/src/test_helpers.rs
    - packages/primitives/src/peer.rs
    - packages/axum-http-server/src/lib.rs
    - packages/axum-http-server/tests/server/asserts.rs
    - packages/tracker-core/src/test_helpers.rs
    - packages/axum-rest-api-server/tests/server/v1/asserts.rs
    - packages/udp-server/tests/server/asserts.rs
---

<!-- skill-link: create-issue -->

# Issue #2996 - Share the `assert_names_no_source_path` test helper

Parent EPIC: [Remove source locations from errors](../../open/2912-remove-source-locations-from-errors/EPIC.md) (#2912). Sub-issue SI-8 (#2996) of the EPIC (after SI-6 in the table, but independent of it).

## Goal

Reduce the nine copies of the test helper `assert_names_no_source_path` to five, by defining it once in `torrust-tracker-test-helpers` for the four packages that already depend on that package in tests, and by linking the five remaining definitions to each other so they do not drift. No package gains a dependency, and no behavior changes.

## Background

Sub-issues SI-2 (#2914), SI-3 (#2915) and SI-4 (#2916) each added a regression test for the rule of the [ADR](../../../adrs/20261118045443_errors_do_not_record_source_locations.md) that an error does not record a source location. Each needed the same assertion helper, so each added a copy. The maintainer's reviewer (da2ce7) raised the duplication as finding F5 of PR #2991 (SI-4) and suggested merging the copies into `torrust-tracker-test-helpers`, which is the home of protocol-agnostic test utilities (rule B of the [test-support ADR](../../../adrs/20261021190318_share_workspace_test_support_through_testing_features.md)). On 2026-11-22 the maintainer decided to do it as a new sub-issue of the EPIC, without increasing package coupling: reduce the duplication now, add links between the remaining implementations to avoid drift, and reconsider removing the rest later.

The helper has the same body everywhere. Only the parameter name and the panic wording differ.

```rust
#[track_caller]
fn assert_names_no_source_path(message: &str) {
    // fails when `message` contains any of:
    //   ".rs:", "packages/", "packages\\", "src/", "src\\", ".cargo/registry", "rustlib/src"
}
```

Copies, verified on the branch of PR #2991 (`2916-tracker-core`; the SI-4 copies reached `develop` with that PR). Copies 8 and 9 were added by SI-5 (#2917, PR #3001):

| # | File | Kind | Becomes |
| --- | --- | --- | --- |
| 1 | `packages/http-protocol/src/test_helpers.rs` | `pub fn` in a `#[cfg(test)]`-only module | Stays; cross-linked |
| 2 | `packages/primitives/src/peer.rs` | private fn in the unit-test module | Stays; cross-linked |
| 3 | `packages/axum-http-server/src/lib.rs` | `pub fn` in `#[cfg(test)] mod tests::helpers` | Removed; uses the shared one |
| 4 | `packages/axum-http-server/tests/server/asserts.rs` | integration tests, `pub fn` | Removed; uses the shared one |
| 5 | `packages/tracker-core/src/test_helpers.rs` | `pub fn` in `#[cfg(test)] pub mod tests` | Removed; uses the shared one |
| 6 | `packages/axum-rest-api-server/tests/server/v1/asserts.rs` | integration tests, `pub fn` | Removed; uses the shared one |
| 7 | `packages/udp-server/tests/server/asserts.rs` | integration tests, `pub fn` | Removed; uses the shared one |
| 8 | `packages/configuration/src/lib.rs` | private fn in the unit-test module | Stays; cross-linked (`test-helpers` depends on `configuration`, a dev-dependency would be a cycle, as for copy 2) |
| 9 | `packages/axum-server/src/tls.rs` | private fn in the unit-test module | Stays; cross-linked (`axum-server` has no `test-helpers` dev-dependency, and none is added) |

About 50 call sites in 21 files call the helper (`grep -rn assert_names_no_source_path packages`).

Dependency facts that decide the design (re-verified):

- `tracker-core`, `axum-http-server`, `axum-rest-api-server` and `udp-server` already list `torrust-tracker-test-helpers` under `[dev-dependencies]`. Using it adds no dependency edge.
- `primitives` cannot use it: `torrust-tracker-test-helpers` depends on `torrust-tracker-configuration`, which depends on `torrust-tracker-primitives`, so a dev-dependency would be a cycle.
- `http-protocol` has no `[dev-dependencies]` entry for it. Adding one would pull a package that builds tracker configurations and sets up logging into a protocol crate's tests.

### Options considered

| Option | Pros | Cons |
| --- | --- | --- |
| A. Merge all nine into `test-helpers` | One definition, no drift | The `primitives` dev-dependency cycle. `http-protocol` gains a dev-dependency on a package that configures and logs trackers |
| B. A new dependency-free dev-only crate | One definition usable everywhere, no cycle | One more workspace package (`docs/packages.md` and `deny.toml` change) for a ten-line function |
| C. Leave all nine | No change, no risk | Nine copies of the fragment list that drift if it ever changes |
| D. Put it in `torrust-info-hash` or `torrust-clock` | `primitives` and `http-protocol` already depend on them | Ties an external crate's API to this workspace's test needs |
| **E (chosen).** Merge for the four packages that already use `test-helpers`; cross-link the four that cannot | Duplication 9 to 5, no new coupling, a small mechanical change, follows rule B, easy to undo or to finish later | Drift between the shared function and the four copies is still possible, and comments enforce nothing. A small test comparing the fragment lists is a possible later step if drift happens |

E is chosen because it removes most of the duplication at no coupling cost, while A and B pay a structural price (a cycle, a new package) for the last four copies and C does not address the finding.

### Why the rest of the workspace policy is not affected

- `deny.toml` bans a set of crates from being dependencies of `test-helpers` (rule B.1). The shared function uses only `core`, so `test-helpers` gains no dependency and no ban entry changes. The four consumers already depend on `test-helpers`, so no `wrappers` list changes either.
- The layer rules of `packages/AGENTS.md` and `docs/packages.md` list `test-helpers` as the lowest layer that other packages may use in tests. No new edge is added, so no layer rule is touched.
- Rule B.1 of the test-support ADR keeps `test-helpers` generic. The helper is protocol-agnostic (it inspects text), so it fits next to `configuration`, `logging` and `random`.

### Relation to SI-6 (#2918)

SI-6 is a separate issue and does not depend on this one. Its enforcement check fails on `#[track_caller]` outside `#[cfg(test)]` modules, `tests/` directories and an explicit allow-list. The shared function is `#[track_caller]` and lives in `packages/test-helpers/src/`, which is neither a `#[cfg(test)]` module nor a `tests/` directory, so that check would flag it. ADR rule 6 (`#[track_caller]` stays allowed in tests) already covers it by intent. The two issues relate as follows: whichever lands second makes the check accept `packages/test-helpers/` (as an allow-list entry with a reason, or as test-support code). The EPIC coordinator puts the exemption in the SI-6 brief, so this issue does not edit the SI-6 specification.

## Scope

### In Scope

- Add `assert_names_no_source_path` to `torrust-tracker-test-helpers` (a small module such as `assertions`), `#[track_caller]`, with a doc comment that names the four other definitions by path.
- Replace copies 3 to 7 by uses of the shared function and delete them. Update the roughly 50 call sites (imports only where the path changes).
- Keep copies 1, 2, 8 and 9. Give each of the five remaining definitions (the shared one, `http-protocol`, `primitives`, `configuration`, `axum-server`) a doc comment that names the other four by path, says why this one is separate, and says to change all five together.
- Unify the panic wording on the one already used by four copies: `the error message must not hold a source path, but it contains {fragment:?}: {message:?}`. The two `axum-http-server` copies currently say "the failure reason"; that wording goes away.

### Out of Scope

- Removing copies 1, 2, 8 and 9 (they stay because of the dependency cycle and the coupling cost of option A). This may be reconsidered later, for example with a new dependency-free crate (option B) if more helpers need it.
- A test that compares the fragment lists of the five definitions. Add it only if drift happens.
- Changing the fragment list, the helper's behavior or any production code.
- SI-6 itself (#2918): the check, the removal of `torrust-located-error` and the exemption of `packages/test-helpers/` (carried by the SI-6 brief).

## Architectural Decisions

- Related ADRs: [`docs/adrs/20261021190318_share_workspace_test_support_through_testing_features.md`](../../../adrs/20261021190318_share_workspace_test_support_through_testing_features.md) (rule B), [`docs/adrs/20261118045443_errors_do_not_record_source_locations.md`](../../../adrs/20261118045443_errors_do_not_record_source_locations.md) (`#[track_caller]` stays allowed in tests).
- ADRs to create: None known. Option E applies rule B as written.

During implementation, stop and create an ADR when a decision affects project
architecture or design patterns, selects an approach among meaningful
alternatives, or has consequences future contributors need to understand. Do not
create ADRs for routine implementation details or style choices already governed
by project conventions.

## Design and Ownership Review

Not applicable: a pure function moves between test-support locations. It adds no child process, asynchronous I/O, network readiness, resource cleanup or reusable test fixture.

## Bug-Fix Process

Not applicable: this issue removes duplication; it fixes no defect.

## Regression Test Strategy

Not applicable: no behavior changes. The existing tests that call the helper keep passing, and a unit test of the shared function (T1) proves that it still rejects each of the seven fragments and accepts a clean message.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | DONE | Add the shared function with a unit test | In `packages/test-helpers`; one test per fragment and one for a clean message; doc comment names the other four definitions by path (copies 1, 2, 8 and 9) |
| T2 | DONE | Switch `tracker-core` (copy 5) | Calls use the shared function; copy deleted |
| T3 | DONE | Switch `axum-http-server` (copies 3 and 4) | Unit tests and integration tests use the shared function; copies deleted |
| T4 | DONE | Switch `axum-rest-api-server` and `udp-server` (copies 6 and 7) | Copies deleted |
| T5 | DONE | Cross-link the remaining copies | Doc comments on copies 1, 2, 8 and 9 name the other four by path and say to change all five together |
| T6 | DONE | Verify | `grep -rn "fn assert_names_no_source_path" packages` finds exactly five definitions; no `Cargo.toml` or `deny.toml` change |

## Commit Points

Map implementation-plan tasks that change code, tests, configuration, or documentation to small,
coherent commit opportunities. A commit point completes one independently reviewable behavior,
refactor, or evidence increment; do not group unrelated changes merely to reduce commit count.

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T1 | The shared function and its test | Commit after focused validation and required review. |
| T2, T3, T4 | One commit per package switch | Commit after focused validation and required review. |
| T5 | The cross-link comments | Commit after focused validation and required review. |
| T6 | Verification evidence | Commit after focused validation and required review. |

Record a justified no-change decision in the task's evidence without creating an empty commit. For
test-producing work, use the `write-unit-test` skill and complete an explicit design review after
each passing test increment, before maintainer review and commit. Confirm that the test exposes the
one causal initial-state difference; its fixture owns only incidental mechanics; and the production
Act plus independently specified expected result remain visible. The review must use the mandatory
prose-first Arrange-Act-Assert comparison: write temporary prose for each section, refactor until
the code expresses it, remove redundant prose, and retain only irreducible context. Record this
review in task evidence or a file-local test plan. Assess helper boundaries by meaningful named
actions and abstraction-level alignment, not caller count: a single-use helper is valid when it
keeps the test readable and hides only incidental mechanics. Commit each reviewed test-design
increment before starting the next planned file or behavior area. Keep final verification and
completion evidence separate when it improves reviewability. Use a Conventional Commit message
with the narrow affected scope, and sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/open/2996-2912-si-8-share-assert-names-no-source-path/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [x] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [x] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [x] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [x] Reviewer validated acceptance criteria and updated checkboxes
- [x] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification (not applicable: no independent reviewer agent was asked to review this folder; the pull request reviews of PR 3041, by Greptile, da2ce7, CodeRabbit and the Claude app, are not independent-reviewer reports and are recorded in `docs/pr-reviews/pr-3041-review/PR-REVIEW.md`)
- [x] Committer verified spec progress is up to date before commit
- [x] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

Append one line per meaningful update. When an AI agent did the work, name its model after the
agent, for example `GitHub Copilot (Claude Opus 5.5)`, so later reviews can relate outcomes to
the model that produced them.

- 2026-11-22 14:06 UTC - Claude Code (Claude Sonnet 5.5) - Drafted from finding F5 of PR #2991 and the maintainer's decision of 2026-11-22 (option E). Not yet approved; no GitHub issue created.
- 2026-11-22 14:24 UTC - Claude Code (Claude Sonnet 5.5) - The maintainer approved the specification; issue #2996 created as `josecelano-bot`; the specification moved from `docs/issues/drafts/` to `docs/issues/open/`; row 8 added to the EPIC table.
- 2026-11-23 13:31 UTC - Claude Code (Claude Sonnet 5.5) - Recorded copies 8 and 9, added by SI-5 (#2917, PR #3001) in `configuration` and `axum-server`: neither can use the shared function (`test-helpers` depends on `configuration`; `axum-server` has no `test-helpers` dev-dependency), so both stay and are cross-linked. Counts now read nine copies to five definitions (review finding F1 of PR #3001).
- 2026-11-24 17:07 UTC - Claude Code (Claude Sonnet 5.5) - Implemented T1 to T6: shared function in `test-helpers`, five copies removed, four remaining copies cross-linked; manual verification V1 and V2 recorded; no Cargo or deny changes.
- 2026-11-25 11:37 UTC - Claude Code (Claude Opus 5.5) - Review round 2 of PR 3041 (da2ce7): the shared test now pins each fragment (V4); four long cross-link doc lines wrapped; V1 re-run and V3 (identical wording) added; each acceptance row cites its own evidence; `last-updated-utc` refreshed.
- 2026-11-25 15:59 UTC - Claude Code (Claude Sonnet 5.5) - PR #3041 merged on 2026-11-25 (merge commit message `Merge torrust/torrust-tracker#3041: test(test-helpers): [#2996] share the assert_names_no_source_path test helper`) and issue #2996 closed on 2026-11-25 15:58 UTC; four rounds of review (Greptile, da2ce7, CodeRabbit; the Claude app found no issue) recorded 11 findings (10 fixed, one declined) in `docs/pr-reviews/pr-3041-review/PR-REVIEW.md`; archived the specification to `docs/issues/closed/` and marked row 8 DONE in the EPIC.

## Acceptance Criteria

- [x] AC1: `torrust-tracker-test-helpers` defines `assert_names_no_source_path` once, with a unit test for each of the seven fragments and for a clean message.
- [x] AC2: `tracker-core`, `axum-http-server`, `axum-rest-api-server` and `udp-server` use the shared function and hold no local definition.
- [x] AC3: Exactly five definitions remain (`test-helpers`, `http-protocol`, `primitives`, `configuration`, `axum-server`), each with a doc comment that names the other four and says to change them together.
- [x] AC4: The panic wording is the same in the five definitions.
- [x] AC5: No `Cargo.toml`, `Cargo.lock` or `deny.toml` changes, and no behavior change besides the unified failure wording.
- [x] `linter all` exits with code `0`
- [x] Relevant tests pass
- [x] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [x] Documentation is updated when behavior/workflow changes

## Verification Plan

Define verification before implementation starts and execute it before closing the issue.

### Automatic Checks

- `linter all`
- `cargo test` for `torrust-tracker-test-helpers`, `torrust-tracker-core`, `torrust-tracker-axum-http-server`, `torrust-tracker-axum-rest-api-server` and `torrust-tracker-udp-server` (unit and integration tests), and for `torrust-tracker-http-protocol` and `torrust-tracker-primitives` for the comment changes
- `cargo deny check bans` (must be unchanged)
- Pre-push checks (when applicable)

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| M1 | Count the definitions | `grep -rn "fn assert_names_no_source_path" packages` | Exactly five definitions, in `test-helpers`, `http-protocol`, `primitives`, `configuration` and `axum-server` | DONE | `manual-verification-evidence.md` section V1 |
| M2 | The shared helper still catches a path | On a scratch change, make one test in a switched package pass a message containing `packages/x.rs:1` to the helper | The test fails with the unified message at the test line (`#[track_caller]`); the scratch change is reverted | DONE | `manual-verification-evidence.md` section V2 |

Notes:

- Manual verification is mandatory even when automated tests pass. It is a
  real human-oriented use of the feature or reproduction of the bug fix, not a
  simulated result and not merely running automated tests.
- Every recorded validation command result must identify the toolchain or
  runtime that produced it when one can affect behavior. For example, record
  `cargo +nightly fmt --all -- --check` as `nightly Rust toolchain`, rather than
  the ambiguous `cargo fmt --all -- --check`.
- Create `manual-verification-evidence.md` from
  `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md` when executing these
  scenarios. Record actual prerequisites, actions, commands, program output,
  relevant tracker logs, and outcomes there.
- If a scenario fails, record the failure and diagnosis in the progress log before proceeding.

### Disposable Verification Scripts

None planned for this issue.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| --- | --- | --- |
| AC1 | DONE | `manual-verification-evidence.md` V4 (each fragment pinned; red without `rustlib/src`); `cargo test -p torrust-tracker-test-helpers` passes |
| AC2 | DONE | `manual-verification-evidence.md` V1 (no definition left in the four packages) and V2 (`tracker-core` uses the shared function); their tests pass |
| AC3 | DONE | `manual-verification-evidence.md` V1 and its re-run (five definitions); the cross-link doc comments in the five files |
| AC4 | DONE | `manual-verification-evidence.md` V3 (`grep -rn 'must not hold a source path' packages`: one identical line per definition) |
| AC5 | DONE | `git diff --stat torrust/develop...HEAD -- '*.toml' Cargo.lock` is empty; the tests of the nine packages pass unchanged |

## Risks and Trade-offs

- **Drift between the five remaining definitions.** The doc comments only ask readers to change them together. Mitigation: a test that compares the fragment lists is a possible later step if drift happens.
- **SI-5 not merged yet.** Copies 8 and 9 reach `develop` with PR #3001 (SI-4's copies 5 to 7 are already there). Mitigation: implementation starts after that PR merges.
- **Failure wording changes in four copies' consumers.** Only text of failing assertions changes; no test pins it (checked: no `should_panic(expected = ...)` uses it).
- **`test-helpers` carries a `#[track_caller]` function outside test code.** Mitigation: the SI-6 brief carries the exemption of `packages/test-helpers/`.

## Implementation Completion Review

After implementation, compare the result with this specification. Record
invalidated assumptions, material design changes, unexpected validation
findings, and reusable lessons.

- Retrospective: none needed. The implementation matched the specification; the only surprise was clippy `too_long_first_doc_paragraph` on the shared function, fixed with a one-line summary.
- If needed, create `implementation-retrospective.md` from the repository
  template at `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this issue
  specification's directory.
- If no retrospective is needed, add a concise progress-log entry explaining
  why the work had no material discovery.
- When an independent reviewer receives this folder-style specification, it
  records its result in `agent-review-reports.md` using
  `docs/templates/AGENT-REVIEW-REPORTS.md`. Do not create that artifact for a
  legacy standalone specification or when no folder-style specification was
  supplied.

## References

- Parent EPIC: #2912, `docs/issues/open/2912-remove-source-locations-from-errors/EPIC.md`
- Finding F5 of PR #2991 (`docs/pr-reviews/pr-2991-review/PR-REVIEW.md` on the branch of that PR)
- Related issues: #2996 (this issue), #2914, #2915, #2916 (the sub-issues that added the copies), #2918 (SI-6)
- Related ADRs: `docs/adrs/20261021190318_share_workspace_test_support_through_testing_features.md`, `docs/adrs/20261118045443_errors_do_not_record_source_locations.md`
