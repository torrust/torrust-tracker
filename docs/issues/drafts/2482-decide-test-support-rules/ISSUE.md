---
schema-version: 1
doc-type: issue
issue-type: task
status: draft
priority: p2
epic: 2482
github-issue: null
spec-path: docs/issues/drafts/2482-decide-test-support-rules/ISSUE.md
branch: "chore/draft-2482-decide-test-support-rules"
related-pr: null
last-updated-utc: "2026-10-21 16:31"
semantic-links:
  skill-links:
    - create-issue
    - create-adr
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - .github/skills/dev/planning/create-adr/SKILL.md
    - docs/issues/open/2482-1669-reorganize-shared-test-support/EPIC.md
    - docs/issues/open/2482-1669-reorganize-shared-test-support/test-support-inventory.md
    - docs/issues/open/1669-overhaul-packages/DECISIONS.md
    - docs/issues/open/1669-overhaul-packages/workspace-coupling-report-2026-10-06.md
    - docs/issues/closed/2582-1669-remove-health-check-self-dev-dependency/ISSUE.md
    - docs/adrs/20260629000000_adopt_independent_package_versioning.md
    - .github/workflows/testing.yaml
    - .github/workflows/deployment-packages.yaml
    - contrib/dev-tools/git/hooks/pre-push.sh
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Decide how packages share test support code

Parent EPIC: #2482 (Reorganize test support code shared between workspace packages), a sub-EPIC
of #1669 (Overhaul: Packages). This is row 1 of the
[#2482 Subissues table](../../open/2482-1669-reorganize-shared-test-support/EPIC.md#subissues).
It blocks every other row.

## Goal

Record, as one approved ADR backed by verified Cargo behaviour, the rules by which workspace
packages share test support code, and turn the result into a migration plan of drafted subissues
for the other rows of #2482, so no package moves test code before the rules exist.

## Background

[#2482](../../open/2482-1669-reorganize-shared-test-support/EPIC.md) found five test support
patterns (A to E) in the workspace, two of which hurt: public `testing` modules that compile test
infrastructure and nine test-only dependency edges into production builds (pattern A), and a
shared `test-helpers` crate that has become a coupling hub (pattern B). DEC-13 chose the public
unconditional module on the premise that `tracker-core/src/test_helpers.rs` was the same pattern;
that module is `#[cfg(test)]` and private, so the premise was wrong
(see the [inventory](../../open/2482-1669-reorganize-shared-test-support/test-support-inventory.md)).

The EPIC lists eight options (O1 to O8) and three ways for a crate's own targets to use its own
`testing` feature (O1a to O1c). Several of their trade-offs rest on Cargo behaviour that nobody
has recorded with commands:

- whether dev-dependency features stay out of normal builds (O1);
- whether a dev-dependency cycle gives a crate's unit tests a second, incompatible copy of the
  crate (O2, O8);
- which dependency forms `cargo package` strips or needs on a registry (O1c, O2);
- which of a crate's own targets get its feature under each mechanism (O1a to O1c).

The cost of guessing is known: #2582 failed in CI because a self dev-dependency carried a
`version`, and two findings on the #2482 spec PR (#2614) were claims copied without checking. This
subissue therefore verifies the Cargo rules first, then writes the ADR, then plans the moves.

Decisions the maintainer already made for #2482 and that bind this subissue:

- Strictly one subissue at a time, each with its own GitHub issue, spec-only PR and
  implementation PR (2026-10-21).
- AC8 of #2482 scopes out the root crate's `src/console/ci/` runners and its normal `tempfile`
  dependency (2026-10-21).
- D5 links findings 1 and 2 of the 2026-10-06 coupling report only; the 2026-10-17 report is not
  edited (2026-10-21).
- `required-features` on a test or example target is not allowed unless CI checks that the target
  runs (2026-10-21).

## Scope

### In Scope

- D1: verify the Cargo rules the options rely on (MV1 to MV4), in scratch Cargo workspaces, with
  the commands and results recorded in this folder.
- D2: write the ADR in `docs/adrs/` and register it in `docs/adrs/index.md`. It carries the
  criteria, every option (O1 to O8, O1a to O1c) with strengths, weaknesses and the reason each
  rejected one is rejected, one rule per pattern (A to E), the ownership map for test fixtures,
  and, if O1 is chosen, the mechanism the package's own targets use. The maintainer approves it
  before it is written (see T3).
- D3: amend DEC-13 with a new DEC entry that links the ADR and corrects its premise.
- D4: write the migration plan as drafts for rows 2 to 8 of #2482 under
  `docs/issues/drafts/2482-<slug>/ISSUE.md`, with destination, final order and the EPIC #1488
  gating; update the #2482 Subissues table. Row 8 is drafted only if the ADR chooses O1.
- D5: link the drafts from findings 1 and 2 of the
  [2026-10-06 coupling report](../../open/1669-overhaul-packages/workspace-coupling-report-2026-10-06.md).
- Update the #2482 checkpoints, acceptance verification and the inventory (a destination for each
  item) as the work lands.

### Out of Scope

- Moving, gating or deleting any test support code, and changing any `Cargo.toml`, workflow,
  hook or tool. This subissue changes documentation only.
- Creating GitHub issues for rows 2 to 8. The next subsessions create them one at a time, after
  the maintainer approves each spec.
- Editing the 2026-10-17 coupling report, #2583's spec, or any non-#2482 work.
- Publishing any crate, or deciding any version bump (#2583 decides them).
- The root crate's CI runners and `tempfile` (AC8 scope-out above).

## Options

The EPIC records the decision options with their strengths and weaknesses; this table condenses
them and says what evidence settles each. The ADR (T3) carries the full comparison and writes the
reason for every option it rejects. No option is rejected by this spec, except that
`required-features` (O1b) is already banned by the maintainer unless CI checks that its targets
run.

| ID | Option | Strengths | Weaknesses | Settled by |
| --- | --- | --- | --- | --- |
| O1 | `testing` Cargo feature per package, optional dependencies, enabled from consumers' `[dev-dependencies]` | Code stays next to what it tests; nothing ships unless enabled; one compiled copy, so unit tests and other packages share types | Own `tests/`, `examples/` and doc tests need a mechanism (O1a to O1c); feature is public semver surface; test API appears in docs unless hidden; optional dependencies look like normal edges to `cargo metadata`; relies on feature isolation | MV1, MV4 |
| O1a | O1 with `--features testing` or `--all-features` on the command line only | No manifest changes; CI, pre-push, nightly and the publish workflow already pass `--all-features` | Plain `cargo test -p <package>` and the pre-commit `cargo test --doc --workspace` fail to compile the targets that use the feature | MV4 |
| O1b | O1 with `required-features = ["testing"]` on test and example targets | Builds without the feature | Cargo skips the targets silently when the feature is off; banned unless CI checks they run | MV4 (to record the skip) |
| O1c | O1 with a path-only self dev-dependency enabling the feature | Plain `cargo test -p` builds every target kind with the feature | Needs proof that packaging strips it (#2582 failed with a versioned one); published `.crate` tests need `--features testing` | MV3, MV4 |
| O2 | Separate support crate per package, used as a dev-dependency | Zero test code in the production crate; explicit ownership; `tests/`, `examples/` and other packages work with plain `cargo test` | More members; only the public API is reachable; unit tests get a second crate copy with mismatched types; a `publish = false` support crate must be path-only | MV2, MV3 |
| O3 | `tests/common/` in the consuming package | Simplest; nothing leaves the consumer | Not shareable; duplicates when several packages need the helper | (no Cargo question) |
| O4 | Generic leaf `test-helpers` with protocol-agnostic helpers only | One home for cross-cutting utilities; no protocol coupling | Needs a rule against protocol helpers creeping back; bump decided by #2583 | (no Cargo question) |
| O5 | `#[cfg(test)]` / `cfg_attr(test, ...)` for crate-private helpers and mocks | No API or dependency cost; `events` already does it | Unusable by the crate's own `tests/` and by other packages | (no Cargo question) |
| O6 | Keep DEC-13 (public unconditional module) | No work | Keeps both problems | MV1 to MV4 show whether anything better is viable |
| O7 | Environments become a supported production start API | Their dependencies become legitimately normal; examples get a real startup path | Test-only parts become public API; a second startup path beside `src/bootstrap/jobs/`; keeps `axum-rest-api-server`'s six edges as runtime coupling | (design decision) |
| O8 | One workspace-wide test-support crate for all environments | One member instead of four | Recreates the coupling hub; cycles with all four servers; no single owner | MV2 |

## Alternatives for this subissue's own approach

| Alternative | Why rejected |
| --- | --- |
| Decide from the inventory and the EPIC alone, without verifying Cargo | The ADR leans on Cargo behaviour four times (the MV1 to MV4 questions above). #2582 failed from one unchecked assumption, and #2614 had two copied claims |
| Run MV1 to MV4 inside the real workspace | 32 members and shipped manifests confound the result and risk leaving edits behind. The scratch workspaces isolate Cargo's behaviour; only the packaging baseline (MV3 baseline) runs read-only on the real workspace |
| Keep verification scripts in the repository | The scenarios test Cargo, not repository code, and the EPIC records that no script is kept. Verbatim commands and manifests in the evidence file let a reviewer repeat them |
| One ADR per pattern | The rules interlock: the ownership map and the O1 mechanism cut across patterns A, B and D. The EPIC asks for one ADR with one rule per pattern |
| Create GitHub issues for rows 2 to 8 in this subissue | The maintainer chose strictly one subissue at a time, and each later scope depends on the approved ADR. Drafts carry the plan until their turn |
| Edit DEC-13 in place | `DECISIONS.md` is a log of decisions. Rewriting an adopted entry hides what was decided and why it changed. A new entry that links the ADR and a one-line pointer on DEC-13 keep both visible |
| Record only a pass/fail summary of the MV scenarios | A summary cannot be re-checked or compared when Cargo changes. Commands, manifests, output and toolchain versions go into the evidence file |

## Architectural Decisions

- Related ADRs: [independent package versioning](../../../adrs/20260629000000_adopt_independent_package_versioning.md)
  (every publishable package stays publishable); DEC-13 in
  [`DECISIONS.md`](../../open/1669-overhaul-packages/DECISIONS.md) (amended by D3).
- ADRs to create: one repository-wide ADR in `docs/adrs/`, "how workspace packages share test
  support code" (D2). It is repository-wide because it binds every package and the
  inter-package dev-dependency edges, so it does not live in a package-local collection.

## Design and Ownership Review

This subissue changes documentation, but the ADR it writes defines the ownership of reusable test
fixtures, so the review below applies to the decision and to the verification harness.

- **Ownership of the fixtures (what the ADR must state).** The package whose server an
  environment starts owns that environment: `udp-server` owns the UDP environment,
  `axum-http-server` the HTTP one, `axum-rest-api-server` the REST API one and
  `axum-health-check-api-server` the health check one. Other packages may use only its published
  test API (O1 or O2) or write their own `tests/` code (O3), unless the ADR chooses O7. No
  package, and no shared crate, owns another package's environment. The ADR also says who owns
  generic helpers (`test-helpers`, O4) and crate-private ones (O5).
- **Narrow interface.** The ADR states each pattern's rule as a short decision table, so a
  contributor can answer "where does this helper go?" without reading the options.
- **Verification harness.** Each scratch workspace belongs to one MV scenario and is removed at
  the end of it, including after a failed run. Every Cargo command runs under an absolute
  deadline (`timeout 600`), and a command that hits it is recorded as a failure, not retried
  silently. No process outlives a scenario, and no network readiness wait is involved.
- **Normal, failure and drop paths.** Normal: the scenario runs, evidence is written, the
  workspace is removed. Failure: the failing command and output are recorded and the workspace is
  still removed. Drop: if a session ends mid-scenario, the next session removes the leftovers
  named in the evidence file before starting.
- **Design-review checkpoints.** After MV1, the first vertical slice, the harness (layout,
  commands, recording format) is reviewed before MV2 to MV4 reuse it. After the ADR draft, the
  maintainer decision in T3 is the checkpoint before D3 to D5 start.

## Bug-Fix Process

Not applicable. This is design and documentation work, not a defect fix.

## Regression Test Strategy

Not applicable. No code, workflow or manifest changes. The checks that protect against a bad
result are the recorded verification scenarios and the link and lint gates below.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | TODO | D1: run MV1 to MV4 in scratch workspaces and record them | `manual-verification-evidence.md` in this folder: exact commands, manifests, output, toolchain versions (stable, and 1.88.0, the MSRV). A packaging baseline on the real workspace (`cargo package --workspace --no-verify`, read-only). A result that contradicts the EPIC is recorded and raised with the maintainer. Scratch workspaces removed |
| T2 | TODO | Summarize the evidence for the maintainer: per option, what the verification shows | Short table in the evidence file, sent to the maintainer through `ask_user` with a recommendation |
| T3 | TODO | D2: write the ADR after the maintainer decides | New file in `docs/adrs/` with a timestamp name, registered in `docs/adrs/index.md`. Criteria; all of O1 to O8 and O1a to O1c with strengths, weaknesses and the reason for each rejection; one rule per pattern A to E; the ownership map; the mechanism for a package's own targets if O1 is chosen. Approved by the maintainer |
| T4 | TODO | D3: add the DEC entry amending DEC-13 | New entry (DEC-18) at the top of `DECISIONS.md` linking the ADR and correcting the `tracker-core/src/test_helpers.rs` premise; a pointer line on DEC-13 (status line unchanged otherwise) |
| T5 | TODO | D4: write the migration drafts for rows 2 to 8 | Drafts `docs/issues/drafts/2482-<slug>/ISSUE.md` with `epic: 2482`, destination, final order, and the #1488 gating (#2416 for rows 4 and 5, #2449 for row 6, #2450 for row 7). Row 8 only if O1 is chosen. Update the #2482 Subissues table and give each inventory item a destination |
| T6 | TODO | D5: link the drafts from findings 1 and 2 of the 2026-10-06 coupling report | Links resolve; the 2026-10-17 report is untouched |
| T7 | TODO | Close out | Update the #2482 checkpoints and acceptance verification; re-review this spec's acceptance criteria; implementation completion review and, if warranted, `implementation-retrospective.md` |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T1 | The evidence file with MV1 to MV4 and the packaging baseline | Commit after the run is complete and the scratch workspaces are removed; one commit per scenario when they are recorded separately. |
| T2 | The evidence summary and the maintainer's decision recorded in this spec | Commit after the maintainer answers, before writing the ADR. |
| T3 | The ADR and its index entry | Commit after the ADR is complete and passes `linter markdown` and `linter cspell`. |
| T4 | The new DEC entry and the pointer on DEC-13 | One commit. |
| T5 | One commit per draft, then the Subissues table and inventory update | Each draft stands alone; the table commit follows them. |
| T6 | The two coupling report links | One commit. |
| T7 | Spec checkpoints, EPIC progress and any retrospective | Commit after the re-review. |

Every commit is signed, uses a Conventional Commit message with the narrow scope, and carries the
Copilot trailer.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2482-decide-test-support-rules/ISSUE.md`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] GitHub issue created, linked as a sub-issue of #2482, and its number added to this spec
- [ ] Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, pre-commit and pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

Append one line per meaningful update.

- 2026-10-21 16:31 UTC - AI assistant (Copilot SDK in VS Code, #2482 subissue 1) - Drafted from
  the Phase 1 plan in the #2482 EPIC (tasks D1 to D5, MV1 to MV4). Re-checked before writing:
  32 workspace members with 10 `publish = false`; `test-helpers` is a dev-dependency of the same 7
  packages the inventory lists; #2416, #2449, #2450 and #2410 are open; the publish workflow runs
  `cargo test -p <crate> --all-targets --all-features`.

## Acceptance Criteria

- [ ] AC1: `manual-verification-evidence.md` records MV1 to MV4 and the packaging baseline with
      exact commands, manifests, output and toolchain versions; every result that contradicts the
      EPIC is recorded and was raised with the maintainer; scratch workspaces are removed.
- [ ] AC2: An ADR in `docs/adrs/`, registered in its index, records the criteria; all of O1 to O8
      and O1a to O1c with strengths, weaknesses and the reason each rejected one is rejected; one
      rule per pattern A to E; the ownership map; and, if O1 is chosen, the mechanism a
      package's own targets use. The maintainer approved the decision before it was written and
      approved the ADR.
- [ ] AC3: `DECISIONS.md` has a new entry amending DEC-13 that links the ADR and corrects the
      `tracker-core/src/test_helpers.rs` premise; DEC-13 points to it.
- [ ] AC4: Rows 2 to 8 of the #2482 Subissues table (row 8 only if the ADR chooses O1) are drafts
      under `docs/issues/drafts/2482-<slug>/` with `epic: 2482`, destination, final order and the
      #1488 gating; every inventory item has a destination and a draft; no GitHub issue exists
      for them.
- [ ] AC5: Findings 1 and 2 of the 2026-10-06 coupling report link the drafts, the links
      resolve, and the 2026-10-17 report is unchanged.
- [ ] AC6: The change set touches documentation only (plus new terms in `project-words.txt`): no
      `Cargo.toml`, source, workflow, hook or tool changes.
- [ ] `linter all` exits with code `0`
- [ ] Pre-push checks pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] The #2482 EPIC's checkpoints, Subissues table and acceptance verification (AC2, AC3, AC5)
      are updated; AC4 and AC6 of the EPIC stay open until the issues exist and the last
      subissue lands

## Verification Plan

### Automatic Checks

- `linter all` (through `./contrib/dev-tools/git/hooks/pre-commit.sh` on every commit, with
  `TORRUST_GIT_HOOKS_LOG_DIR=.tmp`).
- Local link and fragment checks (part of `linter all`) for every new draft, the ADR and the
  coupling report links.
- `git diff --stat torrust/develop...HEAD -- ':!docs' ':!project-words.txt'` is empty (AC6).
- The `open-pull-request` commit-id check before every push of specs, evidence or audits.
- Pre-push checks before opening the implementation PR.

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

The scenarios test Cargo's behaviour, not this repository's code. They run in scratch Cargo
workspaces outside the repository (for example under `/tmp`), each created and removed by the
recorded commands. No script is kept. Each scenario runs on the stable toolchain and on 1.88.0,
the workspace MSRV (the root `Cargo.toml` sets `rust-version = "1.88"`), and records `cargo -V`.
Dependent steps run one at a time.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| MV1 | Dev-dependency features stay out of normal builds | Scratch workspace with the repository's edition and resolver. Crate A has a `testing` feature gating a module; crate B enables it only from `[dev-dependencies]`. Run `cargo build -p B`, `cargo build --workspace`, `cargo tree -e features -p B`, and `cargo test -p B --no-run` | The feature is off in the normal builds and on when building B's tests | TODO | `manual-verification-evidence.md` section V1 |
| MV2 | Dev-dependency cycle and type identity | Crate `support` depends on `a`; `a` dev-depends on `support`. Use a `support` type from `a`'s `#[cfg(test)]` code and from `a/tests/` | Records whether unit tests see mismatched types and integration tests do not | TODO | section V2 |
| MV3 | Publishing with optional and dev dependencies | `cargo publish --dry-run` and `cargo package --workspace --no-verify` on a scratch crate with an optional path dependency, on one with a path-only dev-dependency on a `publish = false` crate, and on one with a versioned dev-dependency on it. Also `cargo package --list` and the packaged `Cargo.toml`. Baseline: `cargo package --workspace --no-verify` on this repository, read-only | Records which forms need the dependency on a registry and which ones packaging strips | TODO | section V3 |
| MV4 | A crate's own targets using its own `testing` feature | Scratch crate with a `testing` feature and a gated module used from `src/` unit tests (`#[cfg(any(test, feature = "testing"))]`), `tests/`, `examples/` and a doc test. For O1a, O1b and O1c, run `cargo test -p <crate>`, `cargo test -p <crate> --all-features`, `cargo test --doc`, `cargo test --workspace --all-targets` and `cargo package --no-verify` | Per mechanism and target: compiles, runs, is skipped, or fails | TODO | section V4 |

Notes:

- Create `manual-verification-evidence.md` from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`.
  Embed every scratch manifest and source file verbatim; the scratch workspaces are removed, so
  nothing may exist only there.
- If a result contradicts the EPIC (for example, if a path-only self dev-dependency does not
  package), record it, stop, and tell the maintainer before writing the ADR.
- If a scenario fails, record the failure and diagnosis in the progress log before proceeding.

### Disposable Verification Scripts

None. The scenarios run as recorded commands.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | TODO                   |          |
| AC2   | TODO                   |          |
| AC3   | TODO                   |          |
| AC4   | TODO                   |          |
| AC5   | TODO                   |          |
| AC6   | TODO                   |          |

## Risks and Trade-offs

- **Cargo versions differ.** A rule verified on one Cargo may not hold on another. Mitigation: run
  on stable and on 1.88.0, and record both versions.
- **A result contradicts the EPIC.** The ADR would then rest on a changed premise. Mitigation:
  stop and tell the maintainer before writing the ADR (T2).
- **The decision needs a trade-off only the maintainer can make.** O1c trades a manifest oddity
  for plain `cargo test`; O2 trades members for a clean production crate. Mitigation: T2 puts the
  evidence and a recommendation before him before the ADR is written.
- **Long pole.** This subissue gates #2576, #2583 and #2594 through #2482, and rows 4 to 7 wait on
  #1488. Mitigation: nothing here waits on #1488; the drafts record the ordering so later rows
  start as soon as their gate clears.
- **Drafts go stale.** Rows 2 to 8 are not issues until their turn, and #1488 subissues change
  the environments in the meantime. Mitigation: each later session re-reads its draft against
  `develop` before asking for approval.
- **Scope creep into implementation.** A scratch result may tempt a manifest change in the real
  workspace. Mitigation: AC6 and the empty non-`docs` diff check.

## Implementation Completion Review

After implementation, compare the result with this specification. Record invalidated
assumptions, material design changes, unexpected validation findings, and reusable lessons.

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from the repository template at
  `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this issue specification's directory.
- If no retrospective is needed, add a concise progress-log entry explaining why the work had no
  material discovery.
- When an independent reviewer receives this folder-style specification, it records its result in
  `agent-review-reports.md` using `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- Parent EPIC: [`#2482`](../../open/2482-1669-reorganize-shared-test-support/EPIC.md) (its Phase 1
  plan and verification table are the source of T1 to T6)
- Inventory: [`test-support-inventory.md`](../../open/2482-1669-reorganize-shared-test-support/test-support-inventory.md)
- Grandparent EPIC: [`#1669`](../../open/1669-overhaul-packages/EPIC.md)
- Decision log: DEC-13 in [`DECISIONS.md`](../../open/1669-overhaul-packages/DECISIONS.md)
- Source findings 1 and 2: [2026-10-06 coupling report](../../open/1669-overhaul-packages/workspace-coupling-report-2026-10-06.md)
- Packaging check and the self dev-dependency it removed: #2582
  ([spec](../../closed/2582-1669-remove-health-check-self-dev-dependency/ISSUE.md))
- EPIC #1488 subissues that gate rows 4 to 7: #2416, #2449, #2450
- Related PRs: #2614 (the #2482 spec refresh)
- Related ADRs: [`20260629000000_adopt_independent_package_versioning.md`](../../../adrs/20260629000000_adopt_independent_package_versioning.md)
