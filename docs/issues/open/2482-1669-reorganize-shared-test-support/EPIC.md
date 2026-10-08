---
schema-version: 1
doc-type: epic
status: planned
epic: 1669
github-issue: 2482
spec-path: docs/issues/open/2482-1669-reorganize-shared-test-support/EPIC.md
epic-owner: josecelano
last-updated-utc: "2026-10-08 14:43"
semantic-links:
  skill-links:
    - create-issue
    - create-adr
  related-artifacts:
    - docs/issues/open/2482-1669-reorganize-shared-test-support/test-support-inventory.md
    - docs/issues/open/1669-overhaul-packages/EPIC.md
    - docs/issues/open/1669-overhaul-packages/DECISIONS.md
    - docs/issues/open/1669-overhaul-packages/workspace-coupling-report-2026-10-06.md
    - docs/adrs/20260629000000_adopt_independent_package_versioning.md
    - packages/test-helpers/Cargo.toml
    - tests/common/mod.rs
---

<!-- skill-link: create-issue -->

# EPIC #2482 - Reorganize test support code shared between workspace packages

Sub-EPIC of EPIC [#1669](../1669-overhaul-packages/EPIC.md) (Overhaul: Packages).

## Goal

Make workspace packages provide test support code (test environments, fixtures, helpers and
mocks) to their own tests and to other packages so that:

1. no shared crate becomes a coupling hub, as `torrust-tracker-test-helpers` has; and
2. code and dependencies that exist only for tests do not reach a package's production build or
   its public API unless a consumer explicitly opts in.

The EPIC first decides the rules (one ADR), then applies them package by package.

## Why This Is Needed

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

Every workspace member without `publish = false`, including every crate in Patterns A and B, is
planned to be published once EPIC #1488 (shutdown overhaul) and EPIC #1669 finish, as issue #2482
states. The
[independent versioning ADR](../../../adrs/20260629000000_adopt_independent_package_versioning.md)
publishes those packages to crates.io as they evolve and keeps an "Unpublished tooling" tier for
the nine members with `publish = false`. Publishability is therefore not the driver. The drivers
are coupling and test code in production builds. This EPIC replaces the drafts for finding 1
(`test-helpers` depends on unpublished crates) and finding 2 (server `testing` modules turn
test-only edges into runtime dependencies) of the
[2026-10-06 coupling report](../1669-overhaul-packages/workspace-coupling-report-2026-10-06.md).

### Why a sub-EPIC

EPIC #1669 finds coupling problems; this one fixes one family of them. All of its subissues
share one decision (the ADR), one inventory, one set of criteria and an ordering constraint with
EPIC #1488. Listed flat in #1669 they would lose that shared context, so #1669 tracks this EPIC
as a single row and the subissues are tracked here.

## Scope

### In Scope

- Keep the [inventory](test-support-inventory.md) current: each subissue updates the rows it
  resolves.
- Decide the rules (subissue 1):
  - verify the Cargo rules each option relies on (MV1 to MV3 below);
  - compare the options below against the criteria below;
  - record the decision as an ADR in `docs/adrs/` (repository-wide, multi-package), with a rule
    per pattern;
  - amend DEC-13 in `DECISIONS.md`, linking the ADR;
  - turn the provisional migration subissues below into drafts.
- Apply the rules (subissues 2 onwards): move each package's test support code to its
  destination, one package or small group per subissue.
- Update the coupling report findings 1 and 2, the dependency diagram and, if O1 is chosen, the
  coupling tool, so test-only edges are no longer shown as runtime edges.

#### Criteria

- A package's normal build contains no test-only code or dependencies unless the consumer opts
  in.
- No crate depends on unrelated packages just to share helpers.
- Unit tests (`#[cfg(test)]` in `src/`), integration tests (`tests/`), examples and benches can
  all use the support code they need, with the same types as the code under test.
- Every package without `publish = false` stays publishable.
- Few extra workspace members and simple rules for contributors.

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

- Changing what tests verify or how the environments behave.
- Reusing the test environments for production startup.
- The root crate's CI runners under `src/console/ci/`, which live in the application crate.
- Publishing any crate.

## Subissues

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

Only subissue 1 is defined. Rows 2 onwards are provisional: one per inventory item, with the
destination and final split set by subissue 1's ADR and migration plan. They become drafts in
subissue 1 and GitHub issues one at a time afterwards.

| Order | Issue | Local Spec | Status | Notes |
| --- | --- | --- | --- | --- |
| 1 | #[To be assigned] - Decide how packages share test support code | `docs/issues/drafts/2482-decide-test-support-rules/ISSUE.md` (to be created from the Phase 1 plan below) | TODO | Tasks D1 to D5; MV1 to MV3. Blocks every other row. |
| 2 | #[To be assigned] - Move the `test-helpers` protocol helpers to their consumer | Provisional | TODO | Pattern B; coupling report finding 1. |
| 3 | #[To be assigned] - Gate `tracker-core`'s test modules and mocks | Provisional | TODO | Patterns C and D. |
| 4 | #[To be assigned] - Relocate the `udp-server` test environment | Provisional | TODO | Pattern A; includes `examples/udp_only_public_tracker.rs`. |
| 5 | #[To be assigned] - Relocate the `axum-http-server` test environment | Provisional | TODO | Pattern A; includes `examples/http_only_public_tracker.rs` and the doc-link-only `configuration` edge. After #2471. |
| 6 | #[To be assigned] - Relocate the `axum-rest-api-server` test environment | Provisional | TODO | Pattern A; six forced edges. After #2449. |
| 7 | #[To be assigned] - Relocate the `axum-health-check-api-server` test environment | Provisional | TODO | Pattern A. After #2450 and after rows 4 to 6, whose environments its tests use. |
| 8 | #[To be assigned] - Label optional dependency edges in the coupling tool and diagram | Provisional | TODO | Only if the ADR chooses O1; before the first O1 move merges. |

## Delivery Strategy

### Phase 1: Decide

- Outcome: subissue 1 merged. The ADR states one rule per pattern, DEC-13 is amended, and every
  inventory item has a destination and a draft subissue.
- Exit criteria: the maintainer has approved the ADR; rows 2 onwards in the table above are
  drafts with their final scope and order.

Plan for subissue 1, kept here until its own spec is created:

| ID | Task | Notes / Expected Output |
| --- | --- | --- |
| D0 | Inventory every test support pattern | DONE in this folder: [`test-support-inventory.md`](test-support-inventory.md) |
| D1 | Verify the Cargo rules the options rely on | MV1 to MV3 recorded in subissue 1's `manual-verification-evidence.md` |
| D2 | Write the ADR: criteria, options, one rule per pattern | ADR approved by the maintainer |
| D3 | Amend DEC-13 in `DECISIONS.md`, linking the ADR | New DEC entry |
| D4 | Write the migration plan as subissue drafts and update the Subissues table | Every inventory item has a destination and a draft |
| D5 | Update the coupling report findings 1 and 2 to point at the subissues | Report links resolve |

Manual verification for D1 (scratch Cargo workspaces under `.tmp/`, created and removed by the
recorded commands; they test Cargo's behaviour, not this repository's code, so no script is
kept):

| ID | Scenario | Human-oriented command/steps | Expected Result |
| --- | --- | --- | --- |
| MV1 | Dev-dependency features stay out of normal builds | In a scratch workspace with the same edition, enable a `testing` feature of crate A only from crate B's `[dev-dependencies]`, then `cargo build -p B` and `cargo tree -e features -p B` | The feature is off in the normal build and on in `cargo test -p B` |
| MV2 | Dev-dependency cycle and type identity | In a scratch workspace, let `support` depend on `a` and `a` dev-depend on `support`; use a `support` type from `a`'s `#[cfg(test)]` code and from `a/tests/` | Records whether unit tests see mismatched types and integration tests do not |
| MV3 | Publishing with optional and dev dependencies | `cargo publish --dry-run` on a scratch crate with an optional path dependency, and on one with a path-only dev-dependency | Records which forms need the dependency on a registry |

### Phase 2: Apply

- Outcome: every inventory item is resolved by its subissue, in the order the migration plan
  sets.
- Exit criteria: no pattern A, B, C or D item remains in the inventory unless the ADR keeps it,
  and the coupling report and diagram show no test-only runtime edge.

### How we work in this EPIC

- **Decision first.** No migration subissue starts before subissue 1 merges. A move that
  needs a rule the ADR does not give goes back to the ADR, not into the subissue.
- **One subissue at a time.** Each subissue gets its own GitHub issue (a sub-issue of #2482),
  its own spec-only PR, and its own implementation PR. The next one starts after the previous
  implementation PR merges, so each review round covers one subissue and none waits on another's
  findings.
- **Naming.** Subissue folders are `docs/issues/{drafts,open}/{issue}-2482-{slug}/` (drafts:
  `2482-{slug}`), with `epic: 2482` in their frontmatter. Branches follow the repository rule:
  `{issue}-2482-{slug}-spec` for the spec PR, `{issue}-2482-{slug}` for the implementation.
- **Behaviour stays the same.** A move changes where test support code lives and how it is
  compiled, not what tests verify. Every test, example and bench that used the moved code still
  builds and passes.
- **Coordination with EPIC #1488.** Its open subissues #2449, #2450 and #2471 change the
  environments this EPIC moves. Each environment move starts after the #1488 subissue that
  changes the same environment merges, or its spec records why the order does not matter.
  #1488 work is not delayed for this EPIC.
- **The inventory is a living document.** Each subissue marks the inventory rows it resolves,
  and updates the coupling report and diagram when it removes edges.

For each subissue implementation in this EPIC, the default completion policy is:

1. Run automatic checks (`linter all`, the affected packages' tests, examples and benches, and
   pre-push checks). A move under O1 also builds the package without the `testing` feature.
2. Run manual verification scenarios and record evidence in the subissue's
   `manual-verification-evidence.md`.
3. Re-review acceptance criteria after implementation and update verification evidence.
4. Complete an evidence-based implementation review. Create or update an
   issue-local retrospective for reusable lessons, material design changes, or
   meaningful deviations from the plan; otherwise record why one was unnecessary
   in the issue progress log.

### Closing the EPIC

The EPIC closes when every subissue is closed and no draft with `epic: 2482` remains. Then the
row for #2482 in EPIC #1669 is marked `DONE` and this folder moves to `docs/issues/closed/`.

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted in `docs/issues/drafts/` (as task #2482's draft, PR #2462)
- [x] GitHub issue created and issue number added to this spec
- [x] Inventory of test support patterns written (D0)
- [x] Converted from a task to a sub-EPIC of #1669
- [ ] Epic spec reviewed and approved by user/maintainer
- [ ] Spec-only PR merged into `develop`
- [ ] Subissue 1 created and linked in this spec
- [ ] Subissues 2 onwards drafted by subissue 1 and linked in this spec
- [ ] Subissue statuses kept up to date in the `Subissues` table
- [ ] For each implemented subissue: automatic checks completed and recorded
- [ ] For each implemented subissue: manual verification completed and recorded
- [ ] For each implemented subissue: acceptance criteria reviewed post-implementation
- [ ] For each implemented subissue: implementation completion review recorded
- [ ] Epic acceptance criteria reviewed and checked off
- [ ] Epic issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

Append one line per meaningful update.

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
- 2026-10-08 12:40 UTC - AI assistant (Copilot SDK in VS Code) - Addressed review 5454599321
  (`da2ce7`, round 1) on PR #2484: the inventory now dates the `test-helpers` dependency growth
  in two steps, drops `torrust-info-hash` from `udp`, says which `tracker-core` test modules stay
  public and which environments alias `Unstarted`, and states how intra-doc links are counted.
  This spec now scopes the publishing plan to members without `publish = false` (nine members set
  it, so the 08:42 entry's "all packages will be published" overstates it) and names the
  open #1488 subissues #2449, #2450 and #2471 that change the environments.
- 2026-10-08 12:53 UTC - GitHub Copilot - Converted #2482 from a task to a sub-EPIC of #1669 at
  the maintainer's request: `ISSUE.md` became `EPIC.md`. The decision work (former T2 to T6,
  MV1 to MV3) becomes subissue 1 (tasks D1 to D5, plan kept in Delivery Strategy until its
  spec exists); the moves become provisional subissues 2 to 8. Added "How we work in this
  EPIC" and the closing rule. The task-only sections (Bug-Fix Process, Regression Test
  Strategy, Commit Points, Verification Plan) move to the subissues.
- 2026-10-08 14:43 UTC - GitHub Copilot - Rewrapped the 12:40 entry to the file's 100-column
  width, per PR #2484 review finding F8.

## Acceptance Criteria

- [ ] AC1: The inventory covers every workspace member and every pattern found, with consumers
      and forced dependencies.
- [ ] AC2: An ADR records the criteria, the options with trade-offs, and one rule per pattern;
      the maintainer has approved it, and `DECISIONS.md` amends DEC-13 and links it.
- [ ] AC3: Each Cargo behaviour the ADR relies on is verified with recorded commands.
- [ ] AC4: All required subissues are created and linked, and every inventory item has one.
- [ ] AC5: Implementation order is explicit and justified, including the ordering with EPIC
      #1488's environment subissues.
- [ ] AC6: Epic status reflects the actual state of the linked subissues.
- [ ] AC7: Every completed subissue includes automated and manual verification evidence, a
      post-implementation acceptance criteria review, and an implementation completion review.
- [ ] AC8: After the last subissue, no package's normal build contains test-only code or
      dependencies unless the ADR keeps them, and the coupling report and diagram show no
      test-only runtime edge.

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
| AC8   | TODO                   |          |

## Risks and Trade-offs

- A mix of rules is harder to remember than one rule; the ADR should give a short decision table
  ("one consumer: `tests/common/`; …").
- The `cargo metadata` view of optional dependencies (O1) needs coupling tool support, or the
  report and diagram keep showing test-only edges as runtime edges. If O1 is chosen, that tool
  change is subissue 8; it was AC8 of the folded draft.
- Moving environments touches many test files in several packages; one package per subissue
  keeps each change small.
- The examples start their trackers through the test environments; their subissues must give
  them a production-style startup or label them as test-support demonstrations.
- Three open subissues of EPIC #1488 change the environments this EPIC would move: #2449
  (REST API environment), #2450 (health check API environment) and #2471 (HTTP environment
  drop path). The ordering rule in "How we work in this EPIC" handles them.
- A sub-EPIC adds one level of tracking; the Subissues table here and the single row in #1669
  keep it to one place each.

## References

- Parent EPIC: [`docs/issues/open/1669-overhaul-packages/EPIC.md`](../1669-overhaul-packages/EPIC.md)
- Inventory: [`test-support-inventory.md`](test-support-inventory.md)
- Source findings 1 and 2: [2026-10-06 coupling report](../1669-overhaul-packages/workspace-coupling-report-2026-10-06.md)
- Related decisions: DEC-13 in [`DECISIONS.md`](../1669-overhaul-packages/DECISIONS.md)
  (relocated the environments to public `src/testing/` modules); its premise that
  `tracker-core/src/test_helpers.rs` was the same pattern does not hold, because that module is
  `#[cfg(test)]` and private to its crate.
- Related ADRs: [`20260629000000_adopt_independent_package_versioning.md`](../../../adrs/20260629000000_adopt_independent_package_versioning.md)
- Baseline issue: #2446
- Shutdown overhaul EPIC: #1488, with open subissues that change the environments:
  [#2449](../2449-1488-si-23-migrate-rest-api-test-environment/ISSUE.md),
  [#2450](../2450-1488-si-24-migrate-health-check-api-test-environment/ISSUE.md) and
  [#2471](../2471-1488-fix-http-environment-drop-path/ISSUE.md)
- Earlier review of this spec: PR #2462 (as a draft task), PR #2484 (spec PR)
