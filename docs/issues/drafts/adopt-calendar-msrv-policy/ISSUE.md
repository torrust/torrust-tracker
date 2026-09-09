---
doc-type: issue
issue-type: task
status: draft
priority: p2
epic: null
github-issue: null
spec-path: docs/issues/drafts/adopt-calendar-msrv-policy/ISSUE.md
branch: "{issue-number}-adopt-calendar-msrv-policy"
related-pr: null
last-updated-utc: 2026-09-09 09:08
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - Cargo.toml
    - AGENTS.md
    - .github/workflows/testing.yaml
    - .github/skills/dev/maintenance/setup-dev-environment/SKILL.md
    - .github/skills/dev/maintenance/update-dependencies/SKILL.md
    - .github/skills/dev/git-workflow/release-new-version/SKILL.md
    - docs/release_process.md
    - docs/adrs/index.md
    - docs/issues/closed/1787-evaluate-msrv-bump.md
    - docs/issues/closed/1778-migrate-to-rust-edition-2024.md
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Adopt a calendar-based MSRV policy

## Goal

Replace the workspace's dependency-driven Minimum Supported Rust Version with a computed one: `rust-version` becomes the newest stable Rust release published at least one year before the day the pin is computed, recorded as a repository-wide ADR, enforced by a continuous-integration job that reads the manifest, and recomputed at the maintenance moments the policy names.

## Background

The workspace has raised its floor twice in recorded history, and neither number was chosen. The edition-2024 migration moved `rust-version` from `1.72` to `1.85` because edition 2024 was stabilised in Rust 1.85 and would not compile below it ([1778-migrate-to-rust-edition-2024.md](../../closed/1778-migrate-to-rust-edition-2024.md)). The following evaluation moved it from `1.85` to `1.88` on the explicit ground that 1.88 was "the minimum floor that avoids `cargo update` regressions on the current lockfile": below it, `cargo update` downgraded bollard, tonic, testcontainers, serde_with, time and ureq ([1787-evaluate-msrv-bump.md](../../closed/1787-evaluate-msrv-bump.md)). In both cases the workspace asked its dependency tree what the floor had to be and wrote the answer into the manifest.

That has three costs.

The first is unpredictability for downstream builders. This workspace publishes crates, independently per package, and ships a tracker that operators build from source; distributions freeze a toolchain and operators often pin one. A floor that can move whenever an unrelated crate publishes a release gives those builders nothing to plan against, neither the timing of the next raise nor its size.

The second is that the schedule belongs to third parties. A single dependency deciding to use a newly stabilised feature moves this workspace's floor for no benefit to this workspace, and the floor then sits higher than anything in the first-party sources requires. The 1.88 raise is precisely that: the sources needed 1.85 for edition 2024, and 1.88 came from `time`.

The third is that every raise is argued from scratch. With no rule, each bump needs its own evaluation, its own review and its own record. Issue #1787 is that negotiation written down, and it ended by deferring the actual policy question to a follow-up.

The dependency-driven floor also conflates two questions that have separate answers. "Which toolchains do we promise to support?" is a compatibility commitment to whoever builds this software. "Which dependency versions can we resolve?" is a build-time selection problem, and Cargo already solves it: the workspace root package declares `edition = "2024"` and no explicit `resolver`, so Cargo uses resolver version 3, the MSRV-aware resolver, which selects the newest dependency versions compatible with the declared `rust-version`. A dependency that wants a newer toolchain is a reason to select an older release of that dependency, not a reason to move the promise. The 1.88 rationale reads as a resolution argument used to settle a compatibility question.

There is a second reason to settle this now. Issue #1787 deferred a split policy to be applied after the `bittorrent-*` crates are extracted ([#1669](https://github.com/torrust/torrust-tracker/issues/1669)): the tracker application would track a recent stable release, while each extracted library would carry the lowest floor that compiles it. That split assumes the two crate classes need different answers, and it inherits the problem it was meant to solve. "A recent stable release" is undefined until someone picks one, and "the lowest floor that compiles" is discovered from the dependency tree, which is the status quo applied per crate instead of per workspace, with per-crate discovery and per-crate enforcement to pay for. A calendar floor answers both classes with one number that is already right for each: contributors and the application are never obstructed by a floor at most one year old, because development happens on current stable or nightly; and an extracted library gets exactly the bounded, published promise external consumers can plan against, strictly more generous than an undefined "recent stable" and strictly more predictable than a discovered minimum. A crate extracted from this workspace then carries the rule with it rather than a number, and the single `rust-version.workspace = true` inheritance across all 25 members stays intact.

Finally, the promise is currently unenforced. No workflow builds this workspace on the declared floor: every job in `.github/workflows/` installs `stable` or `nightly` through `dtolnay/rust-toolchain`, and no workflow mentions the pin at all. `rust-version` is therefore an assertion that nothing checks, and it can drift out of truth without any run turning red. The sibling `torrust-index` repository already runs the enforcing job this workspace lacks, reading the value out of `Cargo.toml` so the manifest stays the single source.

## Scope

### In Scope

- Record the calendar rule, its computation, the alternatives considered, and its consequences as a repository-wide ADR in `docs/adrs/`.
- Raise `[workspace.package] rust-version` in the root `Cargo.toml` to the value the rule computes on the day the change lands, and record the lockfile evidence for that raise.
- Add an MSRV job to `.github/workflows/testing.yaml` that reads `rust-version` from `Cargo.toml`, installs exactly that toolchain, and runs the workspace build checks on it.
- Update every live prose statement of the MSRV so none disagrees with the manifest, and replace the deferred split-policy note in `AGENTS.md` with the rule and a pointer to the ADR.
- Document the recomputation procedure at the maintenance moments the policy names, in the `update-dependencies` skill and in the release process, so the recomputation actually happens rather than depending on someone remembering the ADR.

### Out of Scope

- Changing the Rust edition. The workspace stays on edition 2024; the floor and the edition are independent, and a calendar floor will never fall below the edition's own requirement.
- Extracting `bittorrent-*` crates or any other package. This issue makes the extraction's MSRV question answerable in advance; it does not perform the extraction, and it does not introduce per-package floors.
- Changing dependency versions beyond whatever the resolver selects on its own. No `cargo update` is run for the purpose of this issue, and no dependency is added, removed, held, or replaced here.
- Fixing any pre-existing finding of the unused-dependency check. A plain `cargo machete` run already exits `1` on the `develop` baseline, reporting `torrust-tracker` in `packages/e2e-tools/Cargo.toml` and `torrust-tracker-client-lib` in `packages/test-helpers/Cargo.toml`. This change adds no dependency, so that result must be unchanged; those two findings are a separate condition and are not resolved here.
- Pinning a toolchain for local development. No `rust-toolchain.toml` is introduced; contributors keep building on current stable or nightly, and the container image keeps tracking the upstream `rust:slim-trixie` tag.
- Backfilling the floor's history. The closed specifications that recorded `1.72`, `1.85` and `1.88` are records of what the floor was when those decisions were made and are left as written.

## Architectural Decisions

- Related ADRs: [`docs/adrs/20260830124000_place_adrs_by_decision_scope.md`](../../../adrs/20260830124000_place_adrs_by_decision_scope.md) determines the collection; [`docs/adrs/20260629000000_adopt_independent_package_versioning.md`](../../../adrs/20260629000000_adopt_independent_package_versioning.md) establishes that publishable packages are released independently, which makes the floor a published promise rather than an internal setting; [`docs/adrs/20260612000000_adopt_sccache_for_ci_bare_builds.md`](../../../adrs/20260612000000_adopt_sccache_for_ci_bare_builds.md) governs caching for bare builds in `testing.yaml` and therefore constrains the new job's shape.
- Related records that are not ADRs: the edition-2024 migration and the two floor raises are recorded as issue specifications, not decision records, so this issue's ADR is the first ADR the workspace has on its MSRV.
- ADRs to create: one root ADR, `docs/adrs/{YYYYMMDDHHMMSS}_adopt_calendar_based_msrv_policy.md`.

The decision is repository-wide. It sets one value in `[workspace.package]` that every member inherits, it binds packages that are published independently, and it constrains a shared workflow; under the placement ADR that is a root decision, not one owned by any extractable package, even though the immediate manifest change touches a single file.

The ADR states the rule, shows the computation as a dated table, restates the alternatives below with the reason each is rejected for this workspace, and records the consequences: the raise itself, its breaking-change status for consumers below the new floor, the enforcement job, the guaranteed forward compatibility window, and the supersession of the split policy deferred by #1787. It also names the recomputation moments, so a reader who finds only the ADR learns when the number is expected to move.

### The rule

The workspace `rust-version` is the newest stable Rust release that was published at least one year before the day the pin is computed.

- The floor is computed, not negotiated: a function of the calendar and the Rust release history alone. No dependency requirement, no feature wish and no reviewer preference enters it.
- The floor only ever rises.
- It is recomputed at every release of the workspace and in any maintenance pass that touches dependencies or the toolchain. A recomputation that yields the current pin is a no-op needing no record.
- A recomputation that raises the pin is a breaking change and is recorded as such where this repository publishes its release notes, naming the policy as the reason. It needs no ADR of its own: the rule was decided once, and applying a rule is not a decision.
- A dependency requiring more than the computed floor is never a reason to raise it. The response is to hold that dependency at its newest floor-compatible release, which the MSRV-aware resolver already does, or, where that is untenable, to replace the dependency.

### Computation

Applied on the drafting date, 2026-09-09:

| Release | Published  | At least one year old on 2026-09-09 |
| ------- | ---------- | ----------------------------------- |
| 1.89.0  | 2025-08-07 | Yes, and the newest such release    |
| 1.90.0  | 2025-09-18 | No                                  |

The pin the rule yields today is therefore **1.89**. The publication dates above are taken from the sibling repository's ADR-T-011 and must be re-verified against the Rust release history when the change is implemented.

The value is a function of the implementation date, not of this specification. From 2026-09-18, one year after the publication of 1.90.0, the same rule yields **1.90**. The implementer recomputes on the day the change lands and uses that result; a value copied from this table without recomputation is a defect, not a shortcut.

### Alternatives considered

| Alternative                                          | How it sets the floor              | Why not                                                                                                                                                                                          |
| ---------------------------------------------------- | ---------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| A. Dependency-driven floor, the status quo           | Whatever the transitive tree demands | Unpredictable in timing and size, hands the schedule to third parties, and settles a compatibility promise with a resolution argument the resolver already handles. This is what produced 1.88.   |
| B. Track the latest stable release                   | The newest release                 | Excludes every frozen distribution toolchain and every pinned operator build, so consumers who are doing nothing wrong cannot build the workspace at all.                                          |
| C. A fixed number of releases behind the latest      | Latest minus N                     | Expressed in a unit downstream builders do not plan in; at the six-week release cadence it approximates the calendar rule anyway, so it is the same policy stated less legibly.                    |
| D. A pinned floor changed only by explicit decision  | Whatever was last agreed           | The status quo renamed: it reintroduces a per-raise negotiation and still leaves downstream with no way to anticipate the next move.                                                               |
| E. Per-class split, application and libraries        | Recent stable, and lowest that compiles | Two undefined targets instead of one computed number, requiring per-crate discovery and per-crate enforcement, and reproducing the dependency-driven floor inside each published library.       |
| F. Newest stable at least one year old, **chosen**   | The calendar                       | Predictable a year ahead, computable by anyone without consulting the maintainers, independent of dependency churn, stated in the unit distributions actually use, and identical for both crate classes. |

## Design and Ownership Review

Not applicable. This issue changes a manifest value, a workflow job, an ADR and prose. It introduces no child process, no asynchronous I/O, no network readiness wait, no resource whose lifetime must be reasoned about, and no reusable test fixture.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                          | Notes / Expected Output                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| --- | ------ | --------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | TODO   | Write the ADR and register it                 | `docs/adrs/{YYYYMMDDHHMMSS}_adopt_calendar_based_msrv_policy.md` created from `docs/templates/ADR.md`, stating the scope as repository-wide, the rule, the dated computation table, alternatives A to F, the recomputation moments, and the consequences including the supersession of the split policy deferred by #1787. A row is added to `docs/adrs/index.md`. The ADR carries no `- Status:` header.                                                                                            |
| T2  | TODO   | Recompute the pin and raise `rust-version`    | Recompute the floor on the implementation date against the Rust release history, record the computation in the progress log, and set `[workspace.package] rust-version` in the root `Cargo.toml` to that value. Confirm every member still inherits it: `git grep -c 'rust-version.workspace = true' -- '*/Cargo.toml' 'Cargo.toml'` covers all members and no member declares its own `rust-version`. Capture `git diff --exit-code Cargo.lock` showing the lockfile unchanged by the raise, and `cargo update --dry-run` output recording which packages a later maintenance pass would then be free to advance. |
| T3  | TODO   | Add the MSRV job to `testing.yaml`            | A new independent job `msrv` reads the pin with `sed -n 's/^rust-version = "\(.*\)"/\1/p' Cargo.toml \| head -1` into a step output, installs exactly that toolchain, and runs `cargo check --workspace --all-targets --all-features`. Actions are reused in the exact forms this repository already pins. The job does not duplicate the literal version anywhere.                                                                                                                                 |
| T4  | TODO   | Align every live prose MSRV statement         | `AGENTS.md` line 11 carries the recomputed value; the split-policy note on lines 12 to 15 is replaced by a one-sentence statement of the rule and a link to the ADR. `.github/skills/dev/maintenance/setup-dev-environment/SKILL.md` line 34 carries the recomputed value and points at the ADR instead of restating a bare number. The unopened draft `docs/issues/drafts/1840-workflow-performance-pgo-optimization.md` line 70 refers to the floor as a value to read from the manifest rather than the literal `1.88`, since it is a forward instruction that would otherwise be executed against a stale number. |
| T5  | TODO   | Document the recomputation procedure          | A short recomputation step is added to `.github/skills/dev/maintenance/update-dependencies/SKILL.md` before the dependency update, and to the release process in `docs/release_process.md` and `.github/skills/dev/git-workflow/release-new-version/SKILL.md`. Each states the rule in one sentence, links the ADR, and says what to do with the result: no-op when unchanged, otherwise raise the pin and record the raise in the release notes as a breaking change. This repository keeps no `CHANGELOG.md`, so the GitHub release created in the release process is where the breaking-change record belongs. |
| T6  | TODO   | Verify, review the criteria, and record       | Run the automatic checks, execute every manual scenario and attach its evidence, re-review the acceptance criteria against observed behavior, and complete the implementation completion review.                                                                                                                                                                                                                                                                                                  |

### Job shape

The job belongs in `.github/workflows/testing.yaml`, alongside the existing `unit`, `layer-bans` and `docker-e2e` jobs, because it is a build check on the workspace and shares that workflow's documentation-only skip policy. It runs independently, with no `needs`, so a floor failure surfaces without waiting on any other job.

Two constraints come from this repository rather than from the sibling implementation the job is modelled on.

Caching follows the sccache ADR, not the sibling's `Swatinem/rust-cache@v2`. That ADR replaced the cache action with `mozilla-actions/sccache-action@v0.0.11` for bare CI builds in this workflow specifically, and the existing jobs in the file set `RUSTC_WRAPPER`, `SCCACHE_GHA_ENABLED` and `CARGO_INCREMENTAL=0` accordingly. The MSRV job is a bare build in that same workflow and uses the same mechanism, even though `Swatinem/rust-cache@v2` is still used by other workflows in this repository.

Every `uses:` reference must already be permitted by the Torrust organization allowed-actions policy, or the whole workflow terminates as `startup_failure` before any job runs, which reads as a green check to anyone who only inspects job conclusions. The job therefore introduces no new action: `actions/checkout@v7`, `dtolnay/rust-toolchain@stable` and `mozilla-actions/sccache-action@v0.0.11` are the exact forms already pinned elsewhere in this same file. If a future revision needs a reference this repository does not already use, the `update-github-workflow-actions` skill governs the allowlist step first.

### What the job runs, and why not more

The job runs build checks on the floor and does not execute the test suite there.

The MSRV is a compile promise: it says that a consumer on that toolchain can build this software. `cargo check --workspace --all-targets --all-features` answers exactly that question, and `--all-targets` already compiles the test, bench and example targets, so a construct that the floor cannot compile is caught wherever it appears, including in test code. Executing the tests on the floor would answer a different question, whether the behavior differs between toolchains, which the stable and nightly jobs already cover and which no observed failure motivates. Buying that duplicate coverage costs a second long job on every push and adds a second place for an unrelated flake to fail the pipeline, while the promise itself would be no better enforced.

Formatting and lint checks are likewise not run on the floor. The repository's formatter of record is the nightly one, and the clippy configuration is exercised by the existing jobs; running either against an older compiler would test the old toolchain's diagnostics rather than this workspace's code.

## Commit Points

| Task | Coherent change set                                                                    | Commit policy                                                                                                              |
| ---- | -------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------- |
| T1   | The ADR and its index row.                                                             | Commit first, so the decision is reviewable before anything applies it. `docs(adrs): ...`                                   |
| T2   | The `rust-version` raise and its recorded lockfile evidence.                            | Commit after the ADR, on its own, so the manifest change is a one-line reviewable diff. `chore: ...`                        |
| T3   | The MSRV job in `testing.yaml`.                                                         | Commit separately; it is independently reviewable and independently revertible. `ci: ...`                                   |
| T4   | The prose alignment in `AGENTS.md`, the dev-environment skill, and the PGO draft.       | Commit after the manifest raise it must agree with. `docs: ...`                                                             |
| T5   | The recomputation step in the dependency-update skill and the release process.          | Commit separately from the prose alignment; it changes procedure rather than a stated value. `docs: ...`                    |

Record a justified no-change decision in the task's evidence without creating an empty commit. Use a Conventional Commit message with the narrow affected scope, and sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [ ] Folder-style spec drafted in `docs/issues/drafts/adopt-calendar-msrv-policy/ISSUE.md`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] GitHub issue created and issue number added to this spec
- [ ] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded (status + evidence)
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-09-09 09:08 UTC - Spec author - Draft created after verifying the manifest pin `1.88` at `Cargo.toml` line 67 and its inheritance by all 25 members, the absence of any MSRV job or floor-toolchain reference across `.github/workflows/`, the three live prose statements of the floor, the recorded raises `1.72` to `1.85` and `1.85` to `1.88` with their dependency-driven rationale, the absence of a `CHANGELOG.md`, and the sccache and ADR-placement decisions that constrain the new job and the ADR's collection - This spec

## Acceptance Criteria

- [ ] AC1: A repository-wide ADR in `docs/adrs/` states the calendar rule, its dated computation, alternatives A to F, the recomputation moments, and the consequences, and is registered with a row in `docs/adrs/index.md`.
- [ ] AC2: `[workspace.package] rust-version` equals the value the rule computes on the day the change lands, recomputed rather than copied from this specification, and the computation is recorded in the progress log.
- [ ] AC3: Every workspace member still inherits the floor through `rust-version.workspace = true`, and no member declares a floor of its own.
- [ ] AC4: `.github/workflows/testing.yaml` contains an MSRV job that derives the toolchain from `Cargo.toml` rather than from a duplicated literal, introduces no `uses:` reference this repository does not already pin, and passes on the floor toolchain.
- [ ] AC5: No live prose reference to the MSRV disagrees with the manifest, evidenced by a `git grep -n -i -e msrv -e rust-version` sweep excluding `docs/issues/closed/`, whose remaining hits are manifest inheritance lines, historical records, and statements that now agree with the pin.
- [ ] AC6: The deferred split-policy note is gone from `AGENTS.md`, replaced by the rule and a link to the ADR, and the ADR records that it supersedes the follow-up #1787 deferred.
- [ ] AC7: The recomputation procedure is documented at the moments the policy names, in the `update-dependencies` skill and in the release process, each stating what to do when the recomputed value is unchanged and when it rises.
- [ ] AC8: `Cargo.lock` is unchanged by the raise, or changed only as the resolver requires, with the diff recorded as evidence.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented (status + evidence)
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

Define verification before implementation starts and execute it before closing the issue.

### Automatic Checks

- `linter all`, which must exit `0`.
- The new MSRV job in `.github/workflows/testing.yaml`, green on the pull request.
- `cargo +{pin} check --workspace --all-targets --all-features` run once locally on the floor toolchain, reproducing what the job does.
- `cargo machete`, whose result must be identical to the two-finding `develop` baseline recorded in the out-of-scope list; this change adds no dependency, so any difference is a finding of this issue and the baseline itself is not.
- The pre-commit gate, `./contrib/dev-tools/git/hooks/pre-commit.sh --format=json`.

Routine local gates in this workspace run on the nightly toolchain. The single floor-toolchain run above is deliberate and is the local counterpart of the CI job; it is not a change to the routine gate toolchain.

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                                     | Command/Steps                                                                                                                                                                    | Expected Result                                                                                                                                    | Status | Evidence                              |
| --- | -------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------- | ------ | ------------------------------------- |
| M1  | The workspace compiles on the floor          | Install the pinned toolchain, then run `cargo +{pin} check --workspace --all-targets --all-features` against a target directory dedicated to that toolchain and empty at the start. | The check completes with exit code `0`, proving the promise on a build that shares no artifacts with a newer toolchain.                              | TODO   | {captured output and wall time}       |
| M2  | Cargo enforces the pin                       | Run the same check with the stable toolchain one minor release below the pin.                                                                                                      | Cargo refuses before compiling, reporting that the package requires a newer rustc than the one in use, which shows the floor is enforced by Cargo itself rather than only asserted in prose. | TODO   | {captured error message and `echo $?`} |
| M3  | The raise moves no dependency                | After editing `rust-version`, run `git diff --exit-code Cargo.lock`, then `cargo update --dry-run`.                                                                                 | The lockfile is unchanged by the raise; the dry run records which packages a later maintenance pass would be free to advance, and nothing is advanced by this issue. | TODO   | {captured diff status and dry-run output} |
| M4  | The job reads exactly one value              | Run `sed -n 's/^rust-version = "\(.*\)"/\1/p' Cargo.toml` against the edited manifest.                                                                                              | Exactly one line is printed, the new pin. The workspace inheritance lines `rust-version.workspace = true` do not match, so the job's extraction cannot pick up the wrong value. | TODO   | {captured output}                     |
| M5  | The prose and the manifest agree             | Run `git grep -n -i -e msrv -e rust-version -- . ':!docs/issues/closed'` and read every hit.                                                                                           | No remaining hit states a floor other than the manifest's, and the surviving hits are inheritance lines, historical records, and statements that agree. | TODO   | {captured listing with the reading}   |

Notes:

- Manual verification is mandatory even when automated tests pass.
- If a scenario fails, record the failure and diagnosis in the progress log before proceeding.
- M1 and M2 use a target directory dedicated to the toolchain under test, so neither result can be produced by artifacts a different compiler left behind.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence           |
| ----- | ---------------------- | ------------------ |
| AC1   | TODO                   | {test/log/PR link} |
| AC2   | TODO                   | {test/log/PR link} |
| AC3   | TODO                   | {test/log/PR link} |
| AC4   | TODO                   | {test/log/PR link} |
| AC5   | TODO                   | {test/log/PR link} |
| AC6   | TODO                   | {test/log/PR link} |
| AC7   | TODO                   | {test/log/PR link} |
| AC8   | TODO                   | {test/log/PR link} |

Re-review every acceptance criterion against observed behavior before closing the issue, and update each row with the evidence that establishes it.

## Risks and Trade-offs

- The raise is a breaking change for any consumer on a toolchain below the new floor. This is inherent to any floor that moves; the policy bounds it instead of removing it, guaranteeing that a toolchain at most one year old keeps working and that the next move is predictable a year ahead. The change is recorded as breaking in the release notes, which is where this repository states such things in the absence of a `CHANGELOG.md`.
- The computed value drifts between drafting and implementation, and it changes on 2026-09-18. Mitigation: T2 recomputes on the implementation day and records the computation, and AC2 fails a value copied from this specification.
- `--all-targets --all-features` compiles dev-dependencies, so a dev-dependency requiring more than the floor fails the job even though a consumer building the library would not be affected. This is the policy working as designed: the response is to let the resolver hold that dev-dependency at a floor-compatible release, or to replace it, not to raise the floor. The alternative, narrowing the job to non-test targets, would stop catching floor-incompatible constructs in test code, which is code this repository maintains.
- The job adds runner time on every non-documentation push. It is a check rather than a test run, it shares the workflow's sccache backend, and it replaces a promise that nothing currently verifies.
- The recomputation depends on people following a documented step. Mitigation: T5 places it inside the two procedures that already run at exactly the moments the policy names, rather than leaving it in the ADR alone.
- A future revision of the job that needs an action this repository does not already pin would terminate the workflow as `startup_failure`, which presents as a green check to a reviewer who inspects only job conclusions. Mitigation: the job introduces no new action, and the constraint is stated in the job-shape section so a later editor meets it deliberately.

## Implementation Completion Review

After implementation, compare the result with this specification. Record invalidated assumptions, material design changes, unexpected validation findings, and reusable lessons.

- Retrospective: `Not yet assessed`
- Create `implementation-retrospective.md` in this directory from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` if the recomputed floor turns out not to compile the workspace, if the resolver moves the lockfile in a way this plan did not anticipate, or if the job's shape has to depart from what the job-shape section specifies.
- If none of those occur, add a concise progress-log entry explaining why the work had no material discovery.
- When an independent reviewer receives this folder-style specification, it records its result in `agent-review-reports.md` using `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- Related issues: [#1787](https://github.com/torrust/torrust-tracker/issues/1787), whose deferred split policy this supersedes; [#1669](https://github.com/torrust/torrust-tracker/issues/1669), the package overhaul that excluded MSRV changes from its scope; [#1778](https://github.com/torrust/torrust-tracker/issues/1778), the edition-2024 migration that set `1.85`.
- Related specifications: [`docs/issues/closed/1787-evaluate-msrv-bump.md`](../../closed/1787-evaluate-msrv-bump.md), [`docs/issues/closed/1778-migrate-to-rust-edition-2024.md`](../../closed/1778-migrate-to-rust-edition-2024.md)
- Related ADRs: [`docs/adrs/index.md`](../../../adrs/index.md)
- Prior art: the sibling repository `torrust/torrust-index` adopted this policy as its ADR-T-011 and already runs the manifest-reading MSRV job in its `testing.yaml`.
