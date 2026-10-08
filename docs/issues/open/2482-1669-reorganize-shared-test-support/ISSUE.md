---
schema-version: 1
doc-type: issue
issue-type: task
status: planned
priority: p2
epic: 1669
github-issue: 2482
spec-path: docs/issues/open/2482-1669-reorganize-shared-test-support/ISSUE.md
branch: "2482-1669-decouple-test-helpers-from-unpublished-crates-spec"
related-pr: null
last-updated-utc: "2026-10-08 08:42"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/open/2482-1669-reorganize-shared-test-support/test-support-inventory.md
    - docs/issues/open/1669-overhaul-packages/DECISIONS.md
    - docs/issues/open/1669-overhaul-packages/workspace-coupling-report-2026-10-06.md
    - docs/issues/open/1669-overhaul-packages/EPIC.md
    - docs/adrs/20260629000000_adopt_independent_package_versioning.md
    - packages/test-helpers/Cargo.toml
    - tests/common/mod.rs
---

<!-- skill-link: create-issue -->

# Issue #2482 - Reorganize test support code shared between workspace packages

Subissue of EPIC [#1669](../1669-overhaul-packages/EPIC.md) (Overhaul: Packages).

## Goal

Decide how workspace packages provide test support code (test environments, fixtures, helpers
and mocks) to their own tests and to other packages, so that:

1. no shared crate becomes a coupling hub, as `torrust-tracker-test-helpers` has; and
2. code and dependencies that exist only for tests do not reach a package's production build or
   its public API unless a consumer explicitly opts in.

This issue delivers the decision and a migration plan. The code moves are done in follow-up
subissues, one package or small group each.

## Background

The [test support inventory](test-support-inventory.md) found five patterns across the 31
workspace members:

- **A. Public `testing`/`environment` modules** in `axum-http-server`, `axum-rest-api-server`,
  `udp-server` and `axum-health-check-api-server`. They start a server for tests and are compiled
  into every build. They force 9 normal dependency edges that production code does not use, six
  of them in `axum-rest-api-server`. DEC-13 put them there so other packages' tests can import
  them.
- **B. The shared `test-helpers` crate**, a dev-dependency of 7 packages. Its `http` and `udp`
  modules have one consumer (the root crate's `tests/common/`) but pull three unpublished
  protocol and client crates into every dependent.
- **C. Crate-private test helpers in `src/`** (`tracker-core`): `#[cfg(test)]` inside, so nothing
  ships, but declared `pub mod`, and out of reach of the crate's own integration tests.
- **D. Unconditional mocks**: `tracker-core`'s four `#[automock]` database traits make `mockall` a
  normal dependency and put `Mock*` types in the public API. `events` already gates them with
  `cfg_attr(test, automock)`.
- **E. Per-package `tests/common/` folders**, the right place for single-package helpers, but not
  shareable across packages.

The `Environment` types were once meant for production too. Production and test startup have
diverged: the application starts its servers through `src/bootstrap/jobs/`, while tests need
ephemeral ports, readiness waits and teardown. The environments are test support and stay so.

All workspace packages are planned to be published once EPIC #1488 (shutdown overhaul) and this
EPIC finish, so publishability is not the driver. The drivers are coupling and test code in
production builds. This issue replaces the drafts for finding 1 (`test-helpers` depends on
unpublished crates) and finding 2 (server `testing` modules turn test-only edges into runtime
dependencies) of the
[2026-10-06 coupling report](../1669-overhaul-packages/workspace-coupling-report-2026-10-06.md).

## Scope

### In Scope

- Keep the [inventory](test-support-inventory.md) current until the decision is made.
- Verify the Cargo rules each option relies on (see Verification Plan).
- Compare the options below against these criteria:
  - a package's normal build contains no test-only code or dependencies unless the consumer
    opts in;
  - no crate depends on unrelated packages just to share helpers;
  - unit tests (`#[cfg(test)]` in `src/`), integration tests (`tests/`), examples and benches
    can all use the support code they need, with the same types as the code under test;
  - every package stays publishable;
  - few extra workspace members and simple rules for contributors.
- Record the decision as an ADR, with a rule per pattern, and amend DEC-13 in `DECISIONS.md`.
- Write the migration plan: one follow-up subissue draft per package or small group, listed in
  the EPIC.

#### Options

| ID | Option | Strengths | Weaknesses |
| --- | --- | --- | --- |
| O1 | `testing` Cargo feature per package: gate the module, make its dependencies optional, consumers enable it from `[dev-dependencies]` | Code stays next to what it tests; nothing ships unless enabled; unit tests use it directly | `#[cfg(feature = "testing")]` noise; optional dependencies show as normal edges in `cargo metadata` (the coupling tool and the diagram must label them); relies on the resolver keeping dev-dependency features out of normal builds |
| O2 | Separate support crate per package (for example `torrust-tracker-udp-server-test-support`) used as a dev-dependency | Production crate has zero test code or dependencies; ownership is explicit | More workspace members; only the public API is reachable; `src/` unit tests that use it get a second copy of the crate whose types do not match `crate::` types (dev-dependency cycle) |
| O3 | `tests/common/` in the consuming package | Simplest; nothing leaves the consumer | Not shareable; duplication when several packages need the same helper |
| O4 | Generic leaf `test-helpers` keeping only protocol-agnostic helpers (`configuration`, `logging`, `random`) | One home for cross-cutting test utilities; no protocol coupling | Needs a rule to stop protocol helpers creeping back in |
| O5 | `#[cfg(test)]` and `cfg_attr(test, …)` for helpers and mocks used only inside one crate | No API or dependency cost; already used by `events` | Crate-private: unusable by the crate's own `tests/` and by other packages |
| O6 | Keep DEC-13 (public module, unconditional) | No work | Keeps both problems; rejected unless the comparison shows otherwise |

The expected outcome is a combination, for example O4 for generic helpers, O3 for single-consumer
helpers, O5 for crate-private helpers and mocks, and O1 or O2 for environments shared across
packages. The ADR decides.

### Out of Scope

- Moving code: each move is a follow-up subissue from the migration plan.
- Changing what tests verify or how the environments behave.
- Reusing the test environments for production startup.
- The root crate's CI runners under `src/console/ci/`, which live in the application crate.
- Publishing any crate.

## Architectural Decisions

- Related decisions: DEC-13 in [`DECISIONS.md`](../1669-overhaul-packages/DECISIONS.md)
  (relocated the environments to public `src/testing/` modules); its premise that
  `tracker-core/src/test_helpers.rs` was the same pattern does not hold, because that module is
  `#[cfg(test)]` and private to its crate.
- Related ADRs: [`20260629000000_adopt_independent_package_versioning.md`](../../../adrs/20260629000000_adopt_independent_package_versioning.md)
- ADR to create in `docs/adrs/` (repository-wide, multi-package): how packages provide test
  support code without production coupling. `DECISIONS.md` gets a DEC-13 amendment that links it.

## Design and Ownership Review

The environments are reusable test fixtures that start servers and await readiness. The ADR
states who owns them: the package whose server an environment starts owns that environment, and
other packages use only its published test API (O1 or O2) or their own `tests/` code (O3). Their
start, readiness and teardown behaviour is out of scope here; follow-up subissues that move them
keep it unchanged.

## Bug-Fix Process

Not applicable: no behaviour is wrong; the problem is where test code lives and what it costs.

## Regression Test Strategy

Not applicable. Each follow-up subissue keeps the existing tests, examples and benches passing.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task | Notes / Expected Output |
| --- | ------ | ---- | ----------------------- |
| T1  | DONE   | Inventory every test support pattern | [`test-support-inventory.md`](test-support-inventory.md) |
| T2  | TODO   | Verify the Cargo rules the options rely on | MV1 to MV3 recorded in `manual-verification-evidence.md` |
| T3  | TODO   | Write the ADR: criteria, options, one rule per pattern | ADR approved by the maintainer |
| T4  | TODO   | Amend DEC-13 in `DECISIONS.md`, linking the ADR | New DEC entry |
| T5  | TODO   | Write the migration plan as follow-up subissue drafts and list them in the EPIC | Every inventory item has a destination and a draft |
| T6  | TODO   | Update the coupling report findings 1 and 2 to point at the follow-ups | Report links resolve |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T2   | Verification evidence | One commit after the experiments run. |
| T3   | ADR | One commit after maintainer approval. |
| T4   | `DECISIONS.md` amendment | One commit, after T3. |
| T5   | Follow-up drafts and EPIC entries | One commit per draft. |
| T6   | Coupling report links | One commit. |

Use a Conventional Commit message with the issue reference and sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted in `docs/issues/drafts/`
- [x] Spec reviewed and approved by user/maintainer (narrow scope, PR #2462)
- [x] GitHub issue created and issue number added to this spec
- [x] Spec moved to `docs/issues/open/` with issue number prefix
- [x] Inventory of test support patterns written (T1)
- [ ] Broadened spec reviewed and approved by the maintainer
- [ ] Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed (T2 to T6)
- [ ] Automatic verification completed (`linter all` and pre-push checks)
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
- 2026-10-08 07:02 UTC - GitHub Copilot - Maintainer approved the draft (reviewed in PR #2462).
  Created issue #2482, linked it as a subissue of EPIC #1669, and moved the spec to
  `docs/issues/open/`.
- 2026-10-08 07:04 UTC - GitHub Copilot - The root crate has no `integration` test target any
  more; the automatic check now runs its `[[test]]` targets with `cargo test -p torrust-tracker
  --tests`.
- 2026-10-08 08:42 UTC - GitHub Copilot - Broadened at the maintainer's request from moving the
  `test-helpers` protocol helpers to deciding how packages share test support code. Added the
  inventory (T1). The feature-gate option is no longer rejected, because all packages will be
  published. Folded in the draft for finding 2 (`1669-gate-server-testing-modules-behind-feature`),
  whose feature-gating plan is now option O1, and renamed the folder from
  `2482-1669-decouple-test-helpers-from-unpublished-crates`.

## Acceptance Criteria

- [ ] AC1: The inventory covers every workspace member and every pattern found, with consumers
      and forced dependencies.
- [ ] AC2: Each Cargo behaviour the decision relies on is verified with recorded commands
      (MV1 to MV3).
- [ ] AC3: An ADR records the criteria, the options with trade-offs, and one rule per pattern,
      and the maintainer has approved it.
- [ ] AC4: `DECISIONS.md` amends DEC-13 and links the ADR.
- [ ] AC5: Every inventory item has a destination in the migration plan, and each follow-up has a
      draft subissue listed in the EPIC.
- [ ] AC6: `linter all` exits with code `0`.
- [ ] AC7: The acceptance criteria are re-reviewed after implementation.

## Verification Plan

### Automatic Checks

- `linter all`
- Pre-push checks

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | -------- | ---------------------------- | --------------- | ------ | -------- |
| MV1 | Dev-dependency features stay out of normal builds | In a scratch workspace with the same edition, enable a `testing` feature of crate A only from crate B's `[dev-dependencies]`, then `cargo build -p B` and `cargo tree -e features -p B` | The feature is off in the normal build and on in `cargo test -p B` | TODO | `manual-verification-evidence.md` section MV1 |
| MV2 | Dev-dependency cycle and type identity | In a scratch workspace, let `support` depend on `a` and `a` dev-depend on `support`; use a `support` type from `a`'s `#[cfg(test)]` code and from `a/tests/` | Records whether unit tests see mismatched types and integration tests do not | TODO | `manual-verification-evidence.md` section MV2 |
| MV3 | Publishing with optional and dev dependencies | `cargo publish --dry-run` on a scratch crate with an optional path dependency, and on one with a path-only dev-dependency | Records which forms need the dependency on a registry | TODO | `manual-verification-evidence.md` section MV3 |

Create `manual-verification-evidence.md` from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`
when executing these scenarios, and record the toolchain for each `cargo` command.

### Disposable Verification Scripts

MV1 to MV3 use scratch Cargo workspaces under `.tmp/`, created and removed by the commands
recorded in the evidence. They test Cargo's behaviour, not this repository's code, so they are
not maintained tests. No script is kept.

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

- A mix of rules is harder to remember than one rule; the ADR should give a short decision table
  ("one consumer: `tests/common/`; …").
- The `cargo metadata` view of optional dependencies (O1) needs coupling tool support, or the
  report and diagram keep showing test-only edges as runtime edges. If O1 is chosen, that tool
  change is a follow-up; it was AC8 of the folded draft.
- Moving environments touches many test files in several packages; the migration plan keeps each
  follow-up small.
- The examples start their trackers through the test environments; the plan must give them a
  production-style startup or label them as test-support demonstrations.

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

- EPIC: [`docs/issues/open/1669-overhaul-packages/EPIC.md`](../1669-overhaul-packages/EPIC.md)
- Inventory: [`test-support-inventory.md`](test-support-inventory.md)
- Source findings 1 and 2: [2026-10-06 coupling report](../1669-overhaul-packages/workspace-coupling-report-2026-10-06.md)
- Baseline issue: #2446
- Shutdown overhaul EPIC: #1488
