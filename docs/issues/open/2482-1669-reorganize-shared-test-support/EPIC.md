---
schema-version: 1
doc-type: epic
status: planned
epic: 1669
github-issue: 2482
spec-path: docs/issues/open/2482-1669-reorganize-shared-test-support/EPIC.md
epic-owner: josecelano
last-updated-utc: "2026-10-21 16:38"
semantic-links:
  skill-links:
    - create-issue
    - create-adr
  related-artifacts:
    - docs/issues/open/2482-1669-reorganize-shared-test-support/test-support-inventory.md
    - docs/issues/open/1669-overhaul-packages/EPIC.md
    - docs/issues/open/1669-overhaul-packages/DECISIONS.md
    - docs/issues/open/1669-overhaul-packages/workspace-coupling-report-2026-10-06.md
    - docs/issues/closed/2582-1669-remove-health-check-self-dev-dependency/ISSUE.md
    - docs/adrs/20260629000000_adopt_independent_package_versioning.md
    - .github/workflows/testing.yaml
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
workspace members of 2026-10-08. Its 2026-10-21 refresh covers the 32nd member, `ci-gate`, which
adds none:

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
ephemeral ports, readiness waits and teardown. The environments are test support today. Option O7
below records the alternative of turning them into a supported start API.

Every workspace member without `publish = false`, including every crate in Patterns A and B, is
planned to be published once EPIC #1488 (shutdown overhaul) and EPIC #1669 finish, as issue #2482
states. The
[independent versioning ADR](../../../adrs/20260629000000_adopt_independent_package_versioning.md)
publishes those packages to crates.io as they evolve. Its "Unpublished tooling" tier covers the
members with `publish = false`: ten on 2026-10-21, of which the ADR's tier table names three as
examples. Publishability is therefore not the driver. The drivers are coupling and test code in
production builds. This EPIC replaces the drafts for finding 1 (`test-helpers` depends on
unpublished crates) and finding 2 (server `testing` modules turn test-only edges into runtime
dependencies) of the
[2026-10-06 coupling report](../1669-overhaul-packages/workspace-coupling-report-2026-10-06.md).
Those findings exist only in that report; the regenerated 2026-10-17 report has no numbered
findings.

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
  - verify the Cargo rules each option relies on (MV1 to MV4 below);
  - compare the options below against the criteria below;
  - record the decision as an ADR in `docs/adrs/` (repository-wide, multi-package), with a rule
    per pattern;
  - amend DEC-13 in `DECISIONS.md`, linking the ADR;
  - turn the provisional migration subissues below into drafts.
- Apply the rules (subissues 2 onwards): move each package's test support code to its
  destination, one package or small group per subissue.
- Link the subissues from findings 1 and 2 of the
  [2026-10-06 coupling report](../1669-overhaul-packages/workspace-coupling-report-2026-10-06.md),
  where those findings live. Update the dependency diagram and, if O1 is chosen, the coupling
  tool, so test-only edges are no longer shown as runtime edges. After the last subissue,
  regenerate the coupling report to check AC8. The 2026-10-17 report's "New findings" section
  belongs to EPIC #1669's own review of that report, not to this EPIC.

#### Criteria

- A package's normal build contains no test-only code or dependencies unless the consumer opts
  in.
- No crate depends on unrelated packages just to share helpers.
- Unit tests (`#[cfg(test)]` in `src/`), integration tests (`tests/`), examples, benches and doc
  tests can all use the support code they need, with the same types as the code under test.
- A package's own test targets build and run with a plain `cargo test -p <package>`, and none is
  skipped without notice when a feature is off. CI, pre-push and the nightly checks pass
  `--all-features`, but contributors and the pre-commit `cargo test --doc --workspace` do not.
- Every package without `publish = false` stays publishable, and
  `cargo package --workspace --no-verify` (the CI job and pre-push step added by #2582) passes.
- Few extra workspace members and simple rules for contributors.

#### Options

| ID | Option | Strengths | Weaknesses |
| --- | --- | --- | --- |
| O1 | `testing` Cargo feature per package: gate the module, make its dependencies optional, consumers enable it from `[dev-dependencies]` | Code stays next to what it tests; nothing ships unless enabled; one compiled copy of the crate, so `src/` unit tests (through `#[cfg(any(test, feature = "testing"))]`) and other packages see the same types | The crate's own `tests/`, `examples/` and doc tests do not get the feature unless one of the mechanisms in the next table is used. `#[cfg(feature = "testing")]` noise. The feature is public semver surface of a published crate. docs.rs and `cargo doc --all-features` publish the test API unless it is hidden. Optional dependencies need `dep:` syntax, or each one creates an implicit feature. Optional dependencies show as normal edges in `cargo metadata`, so the coupling tool and the diagram must label them. It relies on the resolver keeping dev-dependency features out of normal builds (MV1) |
| O2 | Separate support crate per package (for example `torrust-tracker-udp-server-test-support`) used as a dev-dependency | Production crate has zero test code or dependencies; ownership is explicit; the crate's own `tests/`, `examples/` and other packages use it with a plain `cargo test` | More workspace members. Only the public API is reachable. `src/` unit tests that use it get a second copy of the crate whose types do not match `crate::` types (dev-dependency cycle, MV2). A `publish = false` support crate must be a path-only dev-dependency, or `cargo package` fails, as it did for #2582; packaging then strips it, so the published `.crate`'s tests cannot compile standalone |
| O3 | `tests/common/` in the consuming package | Simplest; nothing leaves the consumer | Not shareable; duplication when several packages need the same helper |
| O4 | Generic leaf `test-helpers` keeping only protocol-agnostic helpers (`configuration`, `logging`, `random`) | One home for cross-cutting test utilities; no protocol coupling | Needs a rule to stop protocol helpers creeping back in. The published 3.0.0 exports only `configuration` and `random`, so removing the unreleased `http` and `udp` modules is not breaking by itself; whether the next release needs a major bump is #2583's comparison with the published API |
| O5 | `#[cfg(test)]` and `cfg_attr(test, …)` for helpers and mocks used only inside one crate | No API or dependency cost; already used by `events` | Crate-private: unusable by the crate's own `tests/` and by other packages |
| O6 | Keep DEC-13 (public module, unconditional) | No work | Keeps both problems; rejected unless the comparison shows otherwise |
| O7 | Turn the environments into production API: a supported embedded-tracker start API per server crate | Their dependencies become legitimately normal; the examples get a real startup path instead of test infrastructure; no feature or extra crate | The test-only parts (ephemeral ports, readiness waits, teardown, `add_torrent_peer`) become public API to maintain and version. Production startup lives in the root crate's `src/bootstrap/jobs/` and has diverged from the environments, so this adds a second startup path. It keeps `axum-rest-api-server`'s six edges as real runtime coupling instead of removing them |
| O8 | One workspace-wide test-support crate holding all four environments | One extra member instead of four; one place to look | Recreates the coupling hub this EPIC removes: every consumer compiles every server. It needs dev-dependency cycles with all four server crates, with O2's type mismatch for their unit tests. It has no single owner |

O1 and O2 are weak in opposite places. O1 serves `src/` unit tests (same types) but not the
crate's own `tests/` and `examples/` without extra mechanism. O2 serves `tests/`, `examples/` and
other packages but not `src/` unit tests. All four pattern-A crates have their own `tests/` using
their environment, and `udp-server` also uses it from `src/` unit tests (`handlers/mod.rs`,
`server/processor.rs`), so the ADR must say which targets each choice leaves behind.

Under O1, a crate's own targets can get its `testing` feature in three ways. MV4 verifies all of
them:

| ID | Mechanism | Strengths | Weaknesses |
| --- | --- | --- | --- |
| O1a | `--features testing` or `--all-features` on the command line only | No manifest changes; CI, pre-push and the nightly checks already pass `--all-features` | `cargo test -p <package>` fails to compile the targets that use the feature; the pre-commit `cargo test --doc --workspace` does not pass it |
| O1b | `required-features = ["testing"]` on each `[[test]]` and `[[example]]` | Builds without the feature | Cargo skips those targets without failing when the feature is off, so tests can stop running unnoticed. Not allowed unless CI checks that the targets run |
| O1c | Path-only self dev-dependency: `<crate> = { path = ".", features = ["testing"] }`, with no `version` | Plain `cargo test -p <package>` builds unit tests, `tests/` and examples with the feature. A probe on 2026-10-21 showed that `cargo package --no-verify` strips it and succeeds | #2582 removed a self dev-dependency, but that one had a `version`, which is why packaging failed. The ADR must say why a path-only one is different. The published `.crate`'s tests need `--features testing` |

The expected outcome is a combination, for example O4 for generic helpers, O3 for single-consumer
helpers, O5 for crate-private helpers and mocks, and O1 or O2 for environments shared across
packages. The ADR decides.

### Out of Scope

- Changing what tests verify or how the environments behave.
- Changing how the application starts its servers (`src/bootstrap/jobs/`). Whether the
  environments become a public start API is option O7, which the ADR decides.
- The root crate's CI runners under `src/console/ci/` and its normal `tempfile` dependency,
  which live in the application crate (the `e2e-tools` binaries call them).
- Publishing any crate.

## Subissues

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

Only subissue 1 (#2618) is defined. Rows 2 onwards are provisional: one per inventory item, with
the destination and final split set by subissue 1's ADR and migration plan. They become drafts in
subissue 1 and GitHub issues one at a time afterwards.

| Order | Issue | Local Spec | Status | Notes |
| --- | --- | --- | --- | --- |
| 1 | #2618 - Decide how packages share test support code | [`docs/issues/open/2618-2482-decide-test-support-rules/ISSUE.md`](../2618-2482-decide-test-support-rules/ISSUE.md) | IN_PROGRESS | Tasks D1 to D5; MV1 to MV4. Blocks every other row. |
| 2 | #[To be assigned] - Move the `test-helpers` protocol helpers to their consumer | Provisional | TODO | Pattern B; coupling report finding 1. `test-helpers` is published as 3.0.0 with only `configuration` and `random`, so the move alone does not break that API; #2583 decides the bump against the published API. |
| 3 | #[To be assigned] - Gate `tracker-core`'s test modules and mocks | Provisional | TODO | Patterns C and D. |
| 4 | #[To be assigned] - Relocate the `udp-server` test environment | Provisional | TODO | Pattern A; includes `examples/udp_only_public_tracker.rs`. After #2416 (its T9 changes this environment). |
| 5 | #[To be assigned] - Relocate the `axum-http-server` test environment | Provisional | TODO | Pattern A; includes `examples/http_only_public_tracker.rs` and the doc-link-only `configuration` edge. After #2471, which merged on 2026-10-16, and after #2416 (its T9 changes this environment). |
| 6 | #[To be assigned] - Relocate the `axum-rest-api-server` test environment | Provisional | TODO | Pattern A; six forced edges; also removes the stale `rest-api-core` comment in `src/testing/mod.rs`. After #2449. |
| 7 | #[To be assigned] - Relocate the `axum-health-check-api-server` test environment | Provisional | TODO | Pattern A. After #2450 and after rows 4 to 6, whose environments its tests use. |
| 8 | #[To be assigned] - Label optional dependency edges in the coupling tool and diagram | Provisional | TODO | Only if the ADR chooses O1; before the first O1 move merges. |

## Delivery Strategy

### Phase 1: Decide

- Outcome: subissue 1 merged. The ADR states one rule per pattern, DEC-13 is amended, and every
  inventory item has a destination and a draft subissue.
- Exit criteria: the maintainer has approved the ADR; rows 2 onwards in the table above are
  drafts with their final scope and order.

Plan for subissue 1. Its own spec, [#2618](../2618-2482-decide-test-support-rules/ISSUE.md),
is authoritative where the two differ:

| ID | Task | Notes / Expected Output |
| --- | --- | --- |
| D0 | Inventory every test support pattern | DONE in this folder: [`test-support-inventory.md`](test-support-inventory.md) |
| D1 | Verify the Cargo rules the options rely on | MV1 to MV4 recorded in subissue 1's `manual-verification-evidence.md` |
| D2 | Write the ADR: criteria, options, one rule per pattern, and the ownership map for shared test fixtures | ADR approved by the maintainer. It states that the package whose server an environment starts owns that environment. Other packages use only its published test API (O1 or O2) or their own `tests/` code (O3), unless the ADR chooses O7. Under O1 it names the mechanism from O1a to O1c that the package's own targets use |
| D3 | Amend DEC-13 in `DECISIONS.md`, linking the ADR | New DEC entry |
| D4 | Write the migration plan as subissue drafts and update the Subissues table | Every inventory item has a destination and a draft |
| D5 | Link the subissues from findings 1 and 2 of the 2026-10-06 coupling report | Report links resolve; the 2026-10-17 report is not edited |

Manual verification for D1 (scratch Cargo workspaces under `.tmp/`, created and removed by the
recorded commands; they test Cargo's behaviour, not this repository's code, so no script is
kept):

| ID | Scenario | Human-oriented command/steps | Expected Result |
| --- | --- | --- | --- |
| MV1 | Dev-dependency features stay out of normal builds | In a scratch workspace with the same edition, enable a `testing` feature of crate A only from crate B's `[dev-dependencies]`, then `cargo build -p B` and `cargo tree -e features -p B` | The feature is off in the normal build and on in `cargo test -p B` |
| MV2 | Dev-dependency cycle and type identity | In a scratch workspace, let `support` depend on `a` and `a` dev-depend on `support`; use a `support` type from `a`'s `#[cfg(test)]` code and from `a/tests/` | Records whether unit tests see mismatched types and integration tests do not |
| MV3 | Publishing with optional and dev dependencies | `cargo publish --dry-run` and `cargo package --workspace --no-verify` on a scratch crate with an optional path dependency, on one with a path-only dev-dependency on a `publish = false` crate, and on one with a versioned dev-dependency on it | Records which forms need the dependency on a registry and which ones packaging strips |
| MV4 | A crate's own targets using its own `testing` feature | In a scratch crate with a `testing` feature, use the gated module from `src/` unit tests (`#[cfg(any(test, feature = "testing"))]`), `tests/`, `examples/` and a doc test. Try O1a, O1b and O1c, each with `cargo test -p <crate>`, with `--all-features`, with `cargo test --doc` and with `cargo package --no-verify` | Records, per mechanism and target, whether it compiles, runs or is skipped |

### Phase 2: Apply

- Outcome: every inventory item is resolved by its subissue, in the order the migration plan
  sets.
- Exit criteria: no pattern A, B, C or D item remains in the inventory unless the ADR keeps it,
  and a coupling report regenerated after the last subissue, and the diagram, show no test-only
  runtime edge.

### How we work in this EPIC

- **Decision first.** No migration subissue starts before subissue 1 merges. A move that
  needs a rule the ADR does not give goes back to the ADR, not into the subissue.
- **One subissue at a time.** Each subissue gets its own GitHub issue (a sub-issue of #2482),
  its own spec-only PR, and its own implementation PR. The next one starts after the previous
  implementation PR merges, so each review round covers one subissue and none waits on another's
  findings. The maintainer confirmed this on 2026-10-21 and rejected the alternative below.

  | Sequencing | Strengths | Weaknesses |
  | --- | --- | --- |
  | Strictly one subissue at a time (chosen) | One review round at a time. No two PRs edit this spec, the inventory or the coupling report at once. Each subissue starts from the previous one's result | Long: 8 subissues with 2 PRs each, and rows 6 and 7 also wait on #1488. This EPIC blocks #2576, which blocks the end of #1669 (see "Related work in EPIC #1669") |
  | Rows 2 and 3 alongside the environment moves (rejected) | Shorter: rows 2 and 3 share no source files with rows 4 to 7 or with #1488 | Two open subissues edit this spec and the inventory at once, and the maintainer reviews two at a time. The maintainer rejected it on 2026-10-21 in favour of the one-review-at-a-time rule above |

- **Naming.** Subissue folders are `docs/issues/{drafts,open}/{issue}-2482-{slug}/` (drafts:
  `2482-{slug}`), with `epic: 2482` in their frontmatter. Branches follow the repository rule:
  `{issue}-2482-{slug}-spec` for the spec PR, `{issue}-2482-{slug}` for the implementation.
- **Behaviour stays the same.** A move changes where test support code lives and how it is
  compiled, not what tests verify. Every test, example and bench that used the moved code still
  builds and passes, and no test target is skipped without notice: `required-features` on a test
  or example target is not allowed unless CI checks that the target runs.
- **Coordination with EPIC #1488.** Its open subissues change the environments this EPIC moves:
  #2449 (REST API environment), #2450 (health check API environment) and #2416 (stop producers
  before event listeners, part of #2410), whose task T9 changes the HTTP and UDP environments.
  #2471 (HTTP environment drop path) merged on 2026-10-16. Each environment move starts after
  the #1488 subissue that changes the same environment merges, or its spec records why the order
  does not matter. #1488 work is not delayed for this EPIC.
- **The inventory is a living document.** Each subissue marks the inventory rows it resolves,
  and updates the dependency diagram when it removes edges.

For each subissue implementation in this EPIC, the default completion policy is:

1. Run automatic checks (`linter all`, the affected packages' tests, examples and benches, and
   pre-push checks). A move under O1 also builds the package without the `testing` feature and
   runs `cargo test -p <package>` without `--all-features`, and the test count does not drop.
2. Run manual verification scenarios and record evidence in the subissue's
   `manual-verification-evidence.md`.
3. Re-review acceptance criteria after implementation and update verification evidence.
4. Complete an evidence-based implementation review. Create or update an
   issue-local retrospective for reusable lessons, material design changes, or
   meaningful deviations from the plan; otherwise record why one was unnecessary
   in the issue progress log.

### Closing the EPIC

The EPIC closes when every subissue is closed and no draft with `epic: 2482` remains. Then the
row for #2482 in EPIC #1669 is marked `DONE`, #2576 loses its `Blocked` label (with the
maintainer's approval), and this folder moves to `docs/issues/closed/`.

### Related work in EPIC #1669

| Issue | Relation |
| --- | --- |
| #2576 (package README refresh) | Blocked on this EPIC: the READMEs must describe the final API this EPIC leaves and cover any crate it adds |
| #2583 (first publish of the unpublished crates) | Its per-crate publish order follows the dependency edges this EPIC removes. Its T4 decides the `test-helpers` bump by comparing the final API with the published 3.0.0 (`configuration` and `random` only) |
| #2594 (extract the console client) | Blocked on #2583, so it waits on this EPIC too |

This EPIC does not publish anything or change those issues; it reports to #1669 when a change
affects them.

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted in `docs/issues/drafts/` (as task #2482's draft, PR #2462)
- [x] GitHub issue created and issue number added to this spec
- [x] Inventory of test support patterns written (D0)
- [x] Converted from a task to a sub-EPIC of #1669
- [x] Epic spec reviewed and approved by user/maintainer (josecelano, 2026-10-21, after the
      #1669 orchestrator's review)
- [x] Spec-only PR merged into `develop` (PR #2614, merge commit `0126fa3f7`, 2026-10-21)
- [x] Subissue 1 created and linked in this spec (#2618)
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
- 2026-10-08 14:48 UTC - GitHub Copilot - The conversion also dropped Design and Ownership
  Review; its ownership rule for the environments is now part of D2's expected output, per PR
  #2484 review finding F12. Subissue 1's spec will carry the full Design and Ownership Review.
- 2026-10-08 15:20 UTC - GitHub Copilot - The 14:48 entry now says subissue 1's spec "will
  carry" the section, since that spec does not exist yet, per PR #2484 review finding F13.
- 2026-10-16 07:10 UTC - GitHub Copilot - #2471 (HTTP environment drop path) merged on
  2026-10-16, so the move of the `axum-http-server` test environment (subissue 5) is no longer
  blocked on it. Its spec moved to `docs/issues/closed/`; updated the links and the ordering
  text. #2449 and #2450 are still open.
- 2026-10-21 13:59 UTC - AI assistant (Copilot SDK in VS Code, #2482 coordinator) - Applied
  the #1669 orchestrator's review of 2026-10-21 with the maintainer's decisions of the same day.
  Changes:
  - O1 now records its main weakness: a crate's own `tests/`, `examples/` and doc tests do not
    get its `testing` feature. The three mechanisms O1a to O1c have their own table, and MV4
    checks them. O1 also gains the cons and the pro the review listed.
  - Added O7 (environments as production API) and O8 (one workspace-wide support crate), and the
    O4 and O2 cons.
  - D5 links findings 1 and 2 of the 2026-10-06 report. AC8 is checked against a report
    regenerated after the last subissue, and it scopes out the root crate's CI runners.
  - The maintainer kept strict one-subissue-at-a-time sequencing; the rejected parallel option
    is recorded.
  - Refreshed facts: 32 members, 10 with `publish = false`, the `cargo package` check from
    #2582, related #1669 issues, #2410.
  - Added the long-pole, silent-test-loss and semver risks. Row 6 owns the stale `rest-api-core`
    comment.
  - Two corrections to the review, both verified: a path-only self dev-dependency packages fine
    (only #2582's versioned one failed), and the new member is `ci-gate`, not `e2e-tools`.
- 2026-10-21 14:40 UTC - AI assistant (Copilot SDK in VS Code, #2482 coordinator) - The
  maintainer (josecelano) approved the corrected spec; ticked the approval checkpoint.
  Rewrapped the first line of the 13:59 entry to 100 columns (no content change).
- 2026-10-21 15:21 UTC - AI assistant (Copilot SDK in VS Code, #2482 coordinator) - Correction to the 13:59
  entry, which kept "#2410" among the refreshed facts on the claim that it changes no
  environment (taken from the orchestrator's review, not checked). #2410's child #2416 still
  lists task T9, which changes the HTTP and UDP environments. The "Coordination with EPIC #1488"
  rule, rows 4 and 5 and the long-pole risk now name #2416 (PR #2614 review finding F1).
- 2026-10-21 15:23 UTC - AI assistant (Copilot SDK in VS Code, #2482 coordinator) - Correction to the 13:59
  entry, whose O4 con and semver risk treated removing the `test-helpers` `http` and `udp`
  modules as a semver-major change. The published `torrust-tracker-test-helpers` 3.0.0 (tag
  `v3.0.0` and the crates.io `.crate`) exports only `configuration` and `random`; `logging`,
  `http` and `udp` are unreleased. The O4 weakness, row 2, the #2583 related-work row, the
  Semver risk and the inventory now leave the bump to #2583's comparison with the published API
  (PR #2614 review finding F2).
- 2026-10-21 16:38 UTC - AI assistant (Copilot SDK in VS Code, #2482 subissue 1) - Created
  subissue 1 as #2618 after the maintainer (josecelano) approved its spec, linked it under #2482,
  and set row 1 of the Subissues table to IN_PROGRESS. Ticked "Spec-only PR merged into `develop`"
  (true since PR #2614 merged) and "Subissue 1 created and linked in this spec". The Phase 1 plan
  now defers to #2618's spec.

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
- [ ] AC8: After the last subissue, no workspace library package's normal build contains
      test-only code or dependencies unless the ADR keeps them, and a coupling report regenerated
      after the last subissue, and the dependency diagram, show no test-only runtime edge. The
      root crate's `src/console/ci/` runners and its normal `tempfile` dependency are out of
      scope.

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
  them a production-style startup (O7 for the examples) or label them as test-support
  demonstrations.
- Four subissues of EPIC #1488 change the environments this EPIC would move: #2449 (REST API
  environment), #2450 (health check API environment), #2416 (HTTP and UDP environments, task
  T9) and #2471 (HTTP environment drop path, merged on 2026-10-16). The ordering rule in "How we
  work in this EPIC" handles them.
- **Long pole.** Eight sequential subissues, each with a spec PR and an implementation PR, and
  rows 4 to 7 wait on #1488 (#2416, #2449, #2450). This EPIC blocks #2576, then #2583, then
  #2594, so it gates the end of #1669. The maintainer accepted that cost for one review at a
  time (2026-10-21).
- **Silent test loss under O1.** `required-features` (O1b) makes Cargo skip a target when the
  feature is off, and a plain `cargo test -p <package>` still passes. Mitigation: the rule in "How
  we work in this EPIC" and the test-count check in the completion policy.
- **Semver.** Removing a public module, gating it behind a feature or moving it to another crate
  breaks a consumer only if the item is in a published release. The published `test-helpers`
  3.0.0 exports only `configuration` and `random`; `logging`, `http` and `udp` are unreleased.
  #2583 decides every bump by comparing the final API with the published one. The unpublished
  crates owe nothing, because their first publish (#2583) starts their history.
- A sub-EPIC adds one level of tracking; the Subissues table here and the single row in #1669
  keep it to one place each.

## References

- Parent EPIC: [`docs/issues/open/1669-overhaul-packages/EPIC.md`](../1669-overhaul-packages/EPIC.md)
- Inventory: [`test-support-inventory.md`](test-support-inventory.md)
- Source findings 1 and 2: [2026-10-06 coupling report](../1669-overhaul-packages/workspace-coupling-report-2026-10-06.md)
- Packaging check and the self dev-dependency it removed: #2582
  ([spec](../../closed/2582-1669-remove-health-check-self-dev-dependency/ISSUE.md))
- Related decisions: DEC-13 in [`DECISIONS.md`](../1669-overhaul-packages/DECISIONS.md)
  (relocated the environments to public `src/testing/` modules); its premise that
  `tracker-core/src/test_helpers.rs` was the same pattern does not hold, because that module is
  `#[cfg(test)]` and private to its crate.
- Related ADRs: [`20260629000000_adopt_independent_package_versioning.md`](../../../adrs/20260629000000_adopt_independent_package_versioning.md)
- Baseline issue: #2446
- Shutdown overhaul EPIC: #1488, with subissues that change the environments (#2471 is closed):
  [#2449](../2449-1488-si-23-migrate-rest-api-test-environment/ISSUE.md),
  [#2450](../2450-1488-si-24-migrate-health-check-api-test-environment/ISSUE.md) and
  [#2471](../../closed/2471-1488-fix-http-environment-drop-path/ISSUE.md)
- Earlier review of this spec: PR #2462 (as a draft task), PR #2484 (spec PR), and the #1669
  orchestrator's review of 2026-10-21 (spec refresh PR)
