---
schema-version: 1
doc-type: issue
issue-type: task
status: in-progress
priority: p2
epic: 1669
github-issue: 2454
spec-path: docs/issues/open/2454-1669-mark-public-error-enums-non-exhaustive/ISSUE.md
branch: "2454-1669-mark-public-error-enums-non-exhaustive"
related-pr: 2568
last-updated-utc: "2026-10-19 19:57"
semantic-links:
  skill-links:
    - create-issue
    - handle-errors-in-code
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - .github/skills/dev/rust-code-quality/handle-errors-in-code/SKILL.md
    - docs/adrs/20261005145329_return_result_only_for_concretely_fallible_public_apis.md
    - docs/issues/open/1669-overhaul-packages/EPIC.md
    - docs/issues/closed/2435-remove-misleading-panics-in-in-memory-torrent-repository/ISSUE.md
---

<!-- skill-link: create-issue -->

# Issue #2454 - Mark Public Error Enums `#[non_exhaustive]` Before First Publish

Parent EPIC: #1669 (Overhaul: Packages), Pre-publish API checklist.

## Goal

Every public error enum with real variants in a workspace crate that will be published on crates.io
is marked `#[non_exhaustive]`, so that adding a variant later is not a breaking change. The audit
also records derive and placeholder decisions, per the EPIC #1669 Pre-publish API checklist.

## Background

[ADR 20261005145329](../../../adrs/20261005145329_return_result_only_for_concretely_fallible_public_apis.md)
(issue #2435) decided that public APIs return `Result` only for concrete failures and accept a
semver-signalled breaking change when a new failure appears. It also requires existing public error
enums to be `#[non_exhaustive]` before their first publish: enums with real variants gain variants
as features grow, and without the attribute each new variant breaks downstream exhaustive matches.
EPIC #1669 records this as its Pre-publish API checklist. Every workspace package is expected on
crates.io within weeks.

Inventory at `develop` on 2026-10-06 (hypothesis to confirm in T1):

- No item in `packages/`, `console/`, or `src/` uses `#[non_exhaustive]`.
- `rg '^\s*pub enum \w*Error\w*\b'` (excluding tests and benches) finds 61 enums: root `src/` 11,
  `tracker-core` 10, `http-protocol` 7, `console/tracker-client` 6, `udp-server` 4, `udp-core` 3,
  and one or two in each of twelve more packages. Some may sit in private modules or binary-only
  code and so are not public API; others may be error types whose name lacks `Error`.
- `torrust-tracker-configuration`'s `Error` has an `Infallible` variant ("The error for errors
  that can never happen."). It is never constructed: its declaration in
  `packages/configuration/src/lib.rs` is the only `Infallible` in `packages/`, `console/`, and
  `src/`. EPIC checklist item 3 covers `Result<_, Infallible>` and empty error enums, not variants;
  the rule that covers this variant is the `handle-errors-in-code` skill's "Never use `Infallible`
  or an empty error enum as a placeholder".

## Scope

### In Scope

- Inventory public error enums per crate: types reachable from the crate's public API that are used
  as the `E` of a public `Result` or implement `std::error::Error`, regardless of name.
- Add `#[non_exhaustive]` to each such enum that has real variants.
- Fix every downstream exhaustive `match` in other workspace crates by adding a wildcard arm that
  maps to the most specific existing handling (for example the UDP `ErrorKind` mapping in
  `udp-server/src/event.rs`).
- Record, per enum, whether its derives are ones every future variant can keep (checklist item 2).
  Change a derive only with a recorded reason and maintainer approval.
- Evaluate placeholder variants such as `configuration::Error::Infallible` under the
  `handle-errors-in-code` skill rule ("Never use `Infallible` or an empty error enum as a
  placeholder"), and confirm that no public API returns `Result<_, Infallible>` or an empty error
  enum (EPIC checklist item 3). Removing a never-constructed placeholder variant is in scope, with
  maintainer approval.
- Record per-crate completion in the T1 inventory, and link that record from EPIC #1669's
  Pre-publish API checklist.

### Out of Scope

- `#[non_exhaustive]` on public structs, configuration types, or non-error enums (a separate EPIC
  #1669 decision if wanted).
- `#[non_exhaustive]` on individual enum variants.
- Changing error semantics or messages, or adding or removing variants, except removing a
  never-constructed placeholder variant (see In Scope; maintainer approval required).
- Publishing crates and editing `Cargo.toml` versions (owned by the release paths of ADR
  20260629000000: the tracker application release for the root `torrust-tracker` crate, the
  per-package publish for the others, and other EPIC #1669 subissues). This issue records the
  required bump; see AC6.

## Architectural Decisions

- Related ADRs: [ADR 20261005145329](../../../adrs/20261005145329_return_result_only_for_concretely_fallible_public_apis.md)
  (the policy this issue applies) and [ADR 20260629000000](../../../adrs/20260629000000_adopt_independent_package_versioning.md)
  (independent package versioning).
- ADRs to create: none expected. If the inventory shows a class of error types that the existing
  ADR does not cover, stop and propose an ADR amendment.

### Maintainer Decisions (2026-10-19, before implementation)

The maintainer answered the pre-implementation questions on 2026-10-19:

1. **Crates in scope**: every publishable workspace crate (`publish` unset or true, including the
   root `torrust-tracker` and `console/tracker-client`). Crates with `publish = []` (the
   `contrib/dev-tools` tools, benchmark and e2e crates) are excluded.
2. **Visibility**: only enums reachable from a crate's public API. Enums that sit in private
   modules or binary-only code are recorded in the inventory as not public API and left unchanged.
3. **Root crate**: `torrust-tracker` has a public `lib.rs` (`pub mod app`, `bootstrap`, `console`,
   `container`), so its error enums are marked like any other publicly reachable enum.
4. **`configuration::Error::Infallible`**: remove it in its own commit (T4), after
   `#[non_exhaustive]` lands for `torrust-tracker-configuration`.
5. **Inventory**: commit `error-enum-inventory.md` in this spec folder and update its per-crate
   completion status in each per-crate commit.
6. **Commits**: one signed commit per package in dependency order, typed `feat(<package>)!:`
   because each is a breaking API change; downstream wildcard arms go in the same commit as the
   enum change, so every commit builds.
7. **Guard (T2)**: the lightest option, no per-enum guard. The checklist plus mandatory manual
   check M1 are the guard.
8. **Derives**: record per-enum findings in the inventory only; ask the maintainer before changing
   any derive.
9. **Exhaustive exception** (2026-10-19, during the T1 review): `#[non_exhaustive]` applies when
   consumers can handle an unknown variant generically. An enum whose variants select different
   recovery actions stays exhaustive, with a doc comment saying why, so that a new variant is a
   deliberate breaking change. The ADR, the EPIC checklist, and the `handle-errors-in-code` skill
   record the rule. `events::receiver::RecvError` and `udp_protocol::request::RequestParseError`
   stay exhaustive; AC2 already allows a maintainer-approved reason.

## Design and Ownership Review

Not applicable: a compile-time API attribute with no processes, I/O, or resource lifetimes.

## Bug-Fix Process

Not applicable: no observed defect. This is a pre-publish API hardening task.

## Regression Test Strategy

Not applicable as a bug. The property is compile-time: a downstream crate cannot exhaustively match
a `#[non_exhaustive]` enum. The decision on how to guard it is recorded in T2.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task | Notes / Expected Output |
| --- | ------ | ---- | ----------------------- |
| T1  | DONE   | Inventory public error enums per crate | Issue-local `error-enum-inventory.md`: crate, enum, path, public reachability, variant count, derives, downstream exhaustive matches, decision, and a per-crate completion status that T3 and T4 update. For each crate already on crates.io (today the root `torrust-tracker`, `torrust-tracker-configuration`, `torrust-tracker-primitives`, and `torrust-tracker-test-helpers`, all at 3.0.0), it records that the next publish must be a semver-major bump (a minor bump for `0.x` crates), and the path that applies it: the tracker application release for the root crate, the per-package publish for the others. Confirms or corrects the 61-enum hypothesis. Maintainer reviews it before T3. Done 2026-10-19: [error-enum-inventory.md](error-enum-inventory.md) lists 60 enums in 18 packages (58 to mark, 2 kept exhaustive); the maintainer reviewed and approved it. |
| T2  | DONE   | Decide how to guard the property | Options: a per-crate `compile_fail` doctest, a lint, or the checklist only. Recommend the lightest option; maintainer decides. If a doctest is chosen, its first increment follows the `write-unit-test` skill's progressive test-development loop, and its design review is recorded before T3 starts. Decided 2026-10-19: checklist plus mandatory manual check M1 only; no doctest, so no test-development loop applies. |
| T3  | DONE   | Apply `#[non_exhaustive]` crate by crate | One commit per crate (or per tightly coupled group), including the downstream wildcard arms that the change forces. Order: dependency order, a crate before its dependents, which crates.io publication also forces. Commits are typed `feat(<package>)!:` (maintainer decision 6). The two exhaustive enums (E1, E2) get a doc-comment commit each instead (`docs(events)`, `docs(udp-protocol)`). Done 2026-10-19: 16 `feat(<package>)!:` commits and two `docs(<package>):` commits (E1, E2); the downstream arms are in the commit of the package whose enum forces them. Every commit builds: `cargo check --workspace --all-targets --all-features` (stable Rust toolchain, `rustc 1.99.0 (b940084d7 2026-09-28)`) passed on each of the branch's commits in a scratch worktree. |
| T4  | DONE   | Evaluate placeholders and derive decisions | `configuration::Error::Infallible` and any similar finding; derive decisions recorded in the inventory; changes only with maintainer approval. Done 2026-10-19: removed `configuration::Error::Infallible` (`refactor(configuration)!: [#2454] remove the never-constructed Infallible error variant`); derive findings are recorded in the inventory with no change (maintainer decision 8). |
| T5  | DONE   | Link the per-crate record from EPIC #1669 | Add a line under the Pre-publish API checklist linking the T1 inventory as the per-crate completion record. Done 2026-10-19: the checklist now links the inventory's per-crate completion section. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T1   | Inventory document | One `docs(issues)` commit after maintainer review. |
| T2   | Guard mechanism (if any) | Commit with the first crate that uses it. |
| T3   | One crate's enums plus the downstream match arms they force | One commit per crate after `cargo check --workspace --all-targets --all-features` and clippy. |
| T4   | Each approved placeholder or derive change | One commit per change. |
| T5   | EPIC #1669 link to the per-crate record | One `docs(issues)` commit. |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/1669-mark-public-error-enums-non-exhaustive/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [x] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation (PR #2456)
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [x] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [x] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [x] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-10-06 15:55 UTC - Copilot - Drafted as the follow-up to issue #2435 and the EPIC #1669 Pre-publish API checklist, at the maintainer's request. Inventory hypothesis from `rg` at `develop`: 61 enums named `*Error*`, no `#[non_exhaustive]` anywhere, and a `configuration::Error::Infallible` variant. Not yet reviewed.
- 2026-10-06 16:05 UTC - Copilot - The maintainer approved the draft as written, including the open manual-verification question, which is to be settled before implementation. Created GitHub issue #2454 with the `task` label, linked it as a sub-issue of EPIC #1669, and moved the spec to `docs/issues/open/` on the spec-only branch `2454-1669-mark-public-error-enums-non-exhaustive-spec`.
- 2026-10-06 20:31 UTC - Copilot - Addressed the PR #2456 reviews (da2ce7 F1 to F6, plus Copilot's three findings, two of which match da2ce7's F1 and F3). Changes: M1 is mandatory with a positive half, and the template AC wording is restored; added the toolchain-evidence rule; the `Infallible` placeholder variant now falls under the `handle-errors-in-code` rule, and its approved removal is exempt from Out of Scope; T3 uses dependency order and the T1 inventory holds per-crate completion; added a conditional test loop for a doctest guard; added the EPIC Details row. On Copilot's SemVer finding, the maintainer decided that the publish flow owns version bumps: new AC6 and EPIC checklist item 4. Audit: `docs/pr-reviews/pr-2456-review/PR-REVIEW.md`.
- 2026-10-07 08:04 UTC - Copilot - Addressed da2ce7's round-2 review of PR #2456: T1's list of already-published crates now includes the root `torrust-tracker` crate, and T1, Out of Scope, and Risks name the tracker application release as the path that applies its bump; the EPIC checklist lead-in now also covers later publishes. Updated the issue body and the PR body's crate count to match.
- 2026-10-19 16:16 UTC - GitHub Copilot - Started implementation on branch `2454-1669-mark-public-error-enums-non-exhaustive`, cut from `torrust/develop`. The maintainer answered the pre-implementation questions; the answers are in "Maintainer Decisions". T2 is decided (checklist plus manual M1, no doctest). The spec-only PR #2456 is merged, so that optional checkpoint is ticked; `branch` now names the implementation branch and `status` is `in-progress`.
- 2026-10-19 16:50 UTC - GitHub Copilot - During the T1 review the maintainer decided that an error enum whose variants select different recovery actions should stay exhaustive, because a wildcard arm would silently choose an action for a variant nobody designed for (decision 9). Recorded as an amendment of ADR 20261005145329, in EPIC checklist item 1, and in the `handle-errors-in-code` skill. `RecvError` (stop or continue a listener) and `RequestParseError` (reply or drop) stay exhaustive.
- 2026-10-19 16:53 UTC - GitHub Copilot - T1 is done: committed [error-enum-inventory.md](error-enum-inventory.md) after the maintainer's review. It lists 60 public error enums in 18 publishable packages, of which 58 get `#[non_exhaustive]` and two stay exhaustive (decision 9), eight downstream `match` sites that need a wildcard arm, the `configuration::Error::Infallible` finding, the derive findings (record only), and the per-crate publish bump. The maintainer confirmed `ErrorKind` out of scope and the `PeerKeyError` mapping to `AuthKeyError::Database`.
- 2026-10-19 17:21 UTC - GitHub Copilot - T5 is done: the Pre-publish API checklist in EPIC #1669 links the inventory's per-crate completion section.
- 2026-10-19 19:23 UTC - GitHub Copilot - T2 to T5 are done and verification is recorded. Implementation: 16 `feat(<package>)!:` commits, two `docs(<package>):` commits for the exhaustive enums, and `refactor(configuration)!: [#2454] remove the never-constructed Infallible error variant`. M1 passed ([evidence](manual-verification-evidence.md)); the pre-push checks passed on the rebased branch. Independent review: the Task Reviewer's pre-PR review ([report](agent-review-reports.md)) found no defect in the Rust changes and six process findings, which this entry and the commits around it address: the spec is updated, the evidence is committed, the inventory wording is corrected, the S5 note records the `report_runtime_failure` helper, the pre-push output is stored, and the branch is rebased onto `develop` (the rebase conflicted only in EPIC #1669's stamp and progress log). A scratch worktree built every commit with `cargo check --workspace --all-targets --all-features` (stable Rust toolchain). No retrospective is needed: the one material discovery, that a wildcard arm can silently choose a recovery action, changed the plan during the T1 review and is already recorded as decision 9, the ADR amendment, EPIC checklist item 1, and the `handle-errors-in-code` skill. The workflow checkpoints "Reviewer validated acceptance criteria" and "Committer verified spec progress" stay not ticked: they belong to the pull-request reviewer and to the committer step, and the AC ticks above rest on the Task Reviewer's check and the implementer's evidence.
- 2026-10-19 19:57 UTC - GitHub Copilot - Opened PR #2568 for the implementation (`Closes #2454`) and requested da2ce7's review. The branch's own commit range is `torrust/develop..HEAD`; PR review rounds are recorded in `docs/pr-reviews/pr-2568-review/PR-REVIEW.md`.

## Acceptance Criteria

- [x] AC1: The inventory lists every public error enum of every workspace crate planned for crates.io, with its decision.
- [x] AC2: Every inventoried enum with real variants is `#[non_exhaustive]`, or the inventory records a maintainer-approved reason why not.
- [x] AC3: The workspace compiles with every downstream exhaustive match updated; no behavior changes.
- [x] AC4: Placeholder findings (including `configuration::Error::Infallible`) and derive decisions are resolved or explicitly deferred with a reason.
- [x] AC5: The T1 inventory records per-crate completion, and EPIC #1669's Pre-publish API checklist links to it.
- [x] AC6: For every already-published crate whose public error enum gains `#[non_exhaustive]`, the inventory records that its next publish must be a semver-major bump (a minor bump for `0.x`), and EPIC #1669's checklist states that publish-time rule.
- [x] `linter all` exits with code `0`
- [x] Relevant tests pass
- [x] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [x] Documentation is updated when behavior/workflow changes

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo check --workspace --all-targets --all-features` and `cargo clippy --workspace --all-targets --all-features` after each crate
- `cargo test --tests --benches --examples --workspace --all-targets --all-features` and `cargo test --doc --workspace`
- Pre-push checks

Record the Rust toolchain for each `cargo` command whose result is recorded (for example `stable
Rust toolchain` or `nightly Rust toolchain`, with `rustc --version`). This includes M1, whose
`E0004` diagnostic text depends on the rustc version, and the pre-push checks, which mix nightly and
stable runs.

### Manual Verification Scenarios

Manual verification is mandatory even when automated tests pass. `#[non_exhaustive]` has no effect
inside the defining crate, so only a dependent crate shows the change: M1 is that real downstream use.

| ID  | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1  | A downstream crate must handle new variants | Scratch crate outside the workspace depending on one changed crate by path; write an exhaustive `match` on its error enum and run `cargo check`; then add a `_` arm and run `cargo check` again | The exhaustive `match` fails with `E0004` (non-exhaustive patterns); the same `match` with a `_` arm compiles | DONE | [`manual-verification-evidence.md`](manual-verification-evidence.md) section V1 |

### Disposable Verification Scripts

None planned.

### Acceptance Verification

| AC ID | Status | Evidence |
| ----- | ------ | -------- |
| AC1   | DONE   | [`error-enum-inventory.md`](error-enum-inventory.md); the Task Reviewer's independent rustdoc scan found no enum missing (see `agent-review-reports.md`) |
| AC2   | DONE   | 58 enums marked (`git diff torrust/develop..HEAD -- packages console src \| grep -c '^+ *#\[non_exhaustive\]'` printed `58`); `RecvError` and `RequestParseError` stay exhaustive by maintainer decision 9; `ErrorKind` is not an error type (R1) |
| AC3   | DONE   | Eight downstream wildcard arms (S1 to S5); every commit passes `cargo check --workspace --all-targets --all-features`; pre-push output in `evidence/pre-push-output.txt`; the only behaviour change is the removed never-constructed variant |
| AC4   | DONE   | Inventory "Placeholder findings" and "Derive findings"; the T4 commit removes `Infallible`; derives unchanged, as the maintainer decided |
| AC5   | DONE   | Inventory "Per-crate completion" and the EPIC #1669 Pre-publish API checklist link |
| AC6   | DONE   | Inventory "Publish bump per crate" and EPIC #1669 checklist item 4 |

## Risks and Trade-offs

- Adding `#[non_exhaustive]` breaks exhaustive matches in downstream crates. Inside the workspace
  the compiler finds them; for crates already on crates.io (for example
  `torrust-tracker-configuration`), it is a semver-breaking change. This issue does not edit
  versions: the inventory records the required bump (AC6), and EPIC #1669's checklist item 4 makes
  the publish apply it (the tracker application release for the root crate, the per-package
  publish for the others).
- Wildcard arms hide future variants from downstream code. Map them to the most specific existing
  handling and keep them few.
- An inventory filtered only by name misses error types with other names; T1 uses public
  reachability and the `std::error::Error` implementation, not names alone.

## Implementation Completion Review

- Retrospective: `Not needed`; see the progress-log entry of 2026-10-19 19:23 UTC.
- If needed, create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md`.
- If no retrospective is needed, add a concise progress-log entry explaining why.
- When an independent reviewer receives this specification, it records its result in
  `agent-review-reports.md` using `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- Related issues: #2435, #1669
- Related PRs: #2445, #2456 (spec-only), #2568 (implementation)
- Related ADRs: [20261005145329](../../../adrs/20261005145329_return_result_only_for_concretely_fallible_public_apis.md), [20260629000000](../../../adrs/20260629000000_adopt_independent_package_versioning.md)
