---
schema-version: 1
doc-type: issue
issue-type: task
status: draft
priority: p2
epic: 1669
github-issue: null
spec-path: docs/issues/drafts/1669-decouple-test-helpers-from-unpublished-crates/ISSUE.md
branch: "{issue-number}-1669-decouple-test-helpers-from-unpublished-crates"
related-pr: null
last-updated-utc: "2026-10-07 12:39"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/test-helpers/Cargo.toml
    - packages/test-helpers/src/http.rs
    - packages/test-helpers/src/udp.rs
    - tests/common/mod.rs
    - docs/issues/open/1669-overhaul-packages/workspace-coupling-report-2026-10-06.md
    - docs/issues/open/1669-overhaul-packages/EPIC.md
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Decouple published `test-helpers` from unpublished crates

Subissue of EPIC [#1669](../../open/1669-overhaul-packages/EPIC.md) (Overhaul: Packages).

## Goal

Make `torrust-tracker-test-helpers` publishable again by removing its normal dependencies on
unpublished workspace crates, without losing the HTTP and UDP client helpers the root
integration tests use.

## Background

The 2026-10-06 workspace coupling report
([finding 1](../../open/1669-overhaul-packages/workspace-coupling-report-2026-10-06.md#1-published-test-helpers-depends-on-unpublished-crates),
issue #2446) found that `torrust-tracker-test-helpers`, published on crates.io as 3.0.0, grew from
one to five workspace dependencies. Its `http` and `udp` modules import
`torrust-tracker-client-lib`, `torrust-tracker-http-protocol` and
`torrust-tracker-udp-protocol`, none of which is published, so a new `test-helpers` release cannot
be published.

The modules arrived with commit "fix(tracker): replace address-keyed bootstrap containers with
ordered vec" (2026-07-29). Their public functions (`http_announce`, `http_scrape`,
`udp_announce`, `udp_complete_download`, `udp_scrape` and three `send_invalid_connection_id*`
helpers) are used only by the root crate's integration tests under `tests/` (through
`tests/common/mod.rs`). The other modules (`configuration`, `logging`, `random`) are used across
the workspace and depend only on `configuration` and `primitives`, both published.

## Scope

### In Scope

- Move the `http` and `udp` helpers out of `test-helpers` and record the decision in
  `docs/issues/open/1669-overhaul-packages/DECISIONS.md`. Destinations to assess:
  1. The root crate's `tests/common/` (their only consumer today).
  2. A new workspace package with `publish = false`, if other packages are expected to need
     them.
- Remove the then-unused dependencies from `test-helpers` and update the affected tests.
- Regenerate or annotate the coupling report and the dependency diagram for the changed edges.

Rejected approaches, both of which leave `test-helpers` unpublishable:

- Gating the modules behind a Cargo feature: `cargo publish` needs every dependency, optional
  ones included, to come from a registry.
- Publishing `client-lib` and the protocol crates first: a release decision outside this EPIC.

### Out of Scope

- Publishing any crate to crates.io.
- Publishing the protocol crates or `client-lib`.
- Changing the behaviour of the helpers or of the tests that use them.

## Architectural Decisions

- Related ADRs: [`docs/adrs/20260629000000_adopt_independent_package_versioning.md`](../../../adrs/20260629000000_adopt_independent_package_versioning.md)
- Decision to record: the chosen destination, as the next free entry in
  [`DECISIONS.md`](../../open/1669-overhaul-packages/DECISIONS.md)

## Design and Ownership Review

Not applicable unless the helpers move into `tests/common/`: in that case, keep each helper's
responsibilities unchanged and review the moved code against the `write-unit-test` skill's
fixture guidance.

## Bug-Fix Process

Not applicable: no product behaviour is wrong; the problem is a publishing constraint.

## Regression Test Strategy

Not applicable. The existing root integration tests that use the helpers must still pass.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                                    | Notes / Expected Output                                                   |
| --- | ------ | ----------------------------------------------------------------------- | ------------------------------------------------------------------------- |
| T1  | TODO   | Choose the destination of the `http` and `udp` helpers and record it in `DECISIONS.md` | Decision entry with the chosen destination and why                        |
| T2  | TODO   | Move the helpers and remove the unpublished dependencies from `test-helpers` | `test-helpers` has no normal dependency on an unpublished crate          |
| T3  | TODO   | Update the coupling report observations and the dependency diagram      | Finding 1 marked resolved; diagram edges match `cargo metadata`          |

## Commit Points

| Task | Coherent change set                                    | Commit policy                                         |
| ---- | ------------------------------------------------------ | ----------------------------------------------------- |
| T1   | `DECISIONS.md` entry                                   | One commit after maintainer approval of the decision. |
| T2   | Helper move plus manifest changes                      | One commit after the affected tests pass.             |
| T3   | Coupling report note and diagram edges                 | One commit after MV2.                                 |

Use a Conventional Commit message with the issue reference and sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted in `docs/issues/drafts/`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] GitHub issue created and issue number added to this spec
- [ ] Spec moved to `docs/issues/open/` with issue number prefix
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests and pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local
      `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded
- [ ] Issue closed and spec moved to `docs/issues/closed/`

### Progress Log

- 2026-10-06 16:01 UTC - GitHub Copilot - Drafted from finding 1 of the 2026-10-06 workspace
  coupling report (#2446).
- 2026-10-07 08:40 UTC - GitHub Copilot - Removed the escape clauses and the options that leave
  `test-helpers` unpublishable (feature gate, publish first), per PR #2462 review finding F3.
- 2026-10-07 09:25 UTC - GitHub Copilot - Copied the template's completion-review conditions,
  per PR #2462 review finding F15. Backfilled at 12:39 UTC, per review finding F20.

## Acceptance Criteria

- [ ] AC1: The decision is recorded in `DECISIONS.md`.
- [ ] AC2: `torrust-tracker-test-helpers` has no dependency (normal or optional) on an
      unpublished workspace crate.
- [ ] AC3: The root integration tests that used the `http` and `udp` helpers still pass.
- [ ] AC4: `cargo publish --dry-run -p torrust-tracker-test-helpers` succeeds.
- [ ] AC5: `linter all` exits with code `0`.
- [ ] AC6: Manual verification scenarios are executed and documented in issue-local
      `manual-verification-evidence.md`.
- [ ] AC7: The acceptance criteria are re-reviewed after implementation.

## Verification Plan

### Automatic Checks

- `cargo test --test integration` (root integration tests)
- `linter all`
- Pre-push checks

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                              | Human-oriented command/steps                                         | Expected Result                                   | Status | Evidence                                      |
| --- | ------------------------------------- | -------------------------------------------------------------------- | ------------------------------------------------- | ------ | --------------------------------------------- |
| MV1 | `test-helpers` dependencies           | `cargo metadata --no-deps` and list the package's normal dependencies | No unpublished workspace crate among them         | TODO   | `manual-verification-evidence.md` section MV1 |
| MV2 | `test-helpers` can be packaged        | `cargo publish --dry-run -p torrust-tracker-test-helpers`            | Succeeds                                          | TODO   | `manual-verification-evidence.md` section MV2 |

Create `manual-verification-evidence.md` from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`
when executing these scenarios, and record the toolchain for each `cargo` command.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | TODO                   |          |
| AC2   | TODO                   |          |
| AC3   | TODO                   |          |
| AC4   | TODO                   |          |
| AC5   | TODO                   |          |
| AC6   | TODO                   |          |
| AC7   | TODO                   |          |

## Risks and Trade-offs

- The next `test-helpers` release no longer contains the `http` and `udp` helpers. External
  users of 3.0.0 are unaffected, because 3.0.0 does not contain them.
- A new `publish = false` package adds a workspace member; prefer `tests/common/` unless a
  second consumer exists.

## Implementation Completion Review

After implementation, compare the result with this specification. Record
invalidated assumptions, material design changes, unexpected validation
findings, and reusable lessons.

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from the repository
  template at `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this issue
  specification's directory.
- If no retrospective is needed, add a concise progress-log entry explaining
  why the work had no material discovery.
- When an independent reviewer receives this folder-style specification, it
  records its result in `agent-review-reports.md` using
  `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- EPIC: [`docs/issues/open/1669-overhaul-packages/EPIC.md`](../../open/1669-overhaul-packages/EPIC.md)
- Source finding: [2026-10-06 coupling report](../../open/1669-overhaul-packages/workspace-coupling-report-2026-10-06.md)
- Baseline issue: #2446
