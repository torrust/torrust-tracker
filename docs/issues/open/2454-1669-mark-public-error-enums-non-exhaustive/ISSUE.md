---
schema-version: 1
doc-type: issue
issue-type: task
status: planned
priority: p2
epic: 1669
github-issue: 2454
spec-path: docs/issues/open/2454-1669-mark-public-error-enums-non-exhaustive/ISSUE.md
branch: "2454-1669-mark-public-error-enums-non-exhaustive-spec"
related-pr: null
last-updated-utc: "2026-10-06 16:05"
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
- Tick the checklist's completion in EPIC #1669 for each crate covered.

### Out of Scope

- `#[non_exhaustive]` on public structs, configuration types, or non-error enums (a separate EPIC
  #1669 decision if wanted).
- `#[non_exhaustive]` on individual enum variants.
- Changing error semantics or messages, or adding or removing variants, except removing a
  never-constructed placeholder variant (see In Scope; maintainer approval required).
- Publishing crates (owned by other EPIC #1669 subissues).

## Architectural Decisions

- Related ADRs: [ADR 20261005145329](../../../adrs/20261005145329_return_result_only_for_concretely_fallible_public_apis.md)
  (the policy this issue applies) and [ADR 20260629000000](../../../adrs/20260629000000_adopt_independent_package_versioning.md)
  (independent package versioning).
- ADRs to create: none expected. If the inventory shows a class of error types that the existing
  ADR does not cover, stop and propose an ADR amendment.

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
| T1  | TODO   | Inventory public error enums per crate | Issue-local `error-enum-inventory.md`: crate, enum, path, public reachability, variant count, derives, downstream exhaustive matches, decision. Confirms or corrects the 61-enum hypothesis. Maintainer reviews it before T3. |
| T2  | TODO   | Decide how to guard the property | Options: a per-crate `compile_fail` doctest, a lint, or the checklist only. Recommend the lightest option; maintainer decides. |
| T3  | TODO   | Apply `#[non_exhaustive]` crate by crate | One commit per crate (or per tightly coupled group), including the downstream wildcard arms that the change forces. Order by EPIC #1669 publication order. |
| T4  | TODO   | Evaluate placeholders and derive decisions | `configuration::Error::Infallible` and any similar finding; derive decisions recorded in the inventory; changes only with maintainer approval. |
| T5  | TODO   | Close the checklist in EPIC #1669 | Mark the Pre-publish API checklist items done for the covered crates, linking this issue. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T1   | Inventory document | One `docs(issues)` commit after maintainer review. |
| T2   | Guard mechanism (if any) | Commit with the first crate that uses it. |
| T3   | One crate's enums plus the downstream match arms they force | One commit per crate after `cargo check --workspace --all-targets --all-features` and clippy. |
| T4   | Each approved placeholder or derive change | One commit per change. |
| T5   | EPIC #1669 checklist update | One `docs(issues)` commit. |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/1669-mark-public-error-enums-non-exhaustive/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [ ] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-10-06 15:55 UTC - Copilot - Drafted as the follow-up to issue #2435 and the EPIC #1669 Pre-publish API checklist, at the maintainer's request. Inventory hypothesis from `rg` at `develop`: 61 enums named `*Error*`, no `#[non_exhaustive]` anywhere, and a `configuration::Error::Infallible` variant. Not yet reviewed.
- 2026-10-06 16:05 UTC - Copilot - The maintainer approved the draft as written, including the open manual-verification question, which is to be settled before implementation. Created GitHub issue #2454 with the `task` label, linked it as a sub-issue of EPIC #1669, and moved the spec to `docs/issues/open/` on the spec-only branch `2454-1669-mark-public-error-enums-non-exhaustive-spec`.

## Acceptance Criteria

- [ ] AC1: The inventory lists every public error enum of every workspace crate planned for crates.io, with its decision.
- [ ] AC2: Every inventoried enum with real variants is `#[non_exhaustive]`, or the inventory records a maintainer-approved reason why not.
- [ ] AC3: The workspace compiles with every downstream exhaustive match updated; no behavior changes.
- [ ] AC4: Placeholder findings (including `configuration::Error::Infallible`) and derive decisions are resolved or explicitly deferred with a reason.
- [ ] AC5: EPIC #1669's Pre-publish API checklist records completion for the covered crates.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

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
| M1  | A downstream crate must handle new variants | Scratch crate outside the workspace depending on one changed crate by path; write an exhaustive `match` on its error enum and run `cargo check`; then add a `_` arm and run `cargo check` again | The exhaustive `match` fails with `E0004` (non-exhaustive patterns); the same `match` with a `_` arm compiles | TODO | `manual-verification-evidence.md` section V1 |

### Disposable Verification Scripts

None planned.

### Acceptance Verification

| AC ID | Status | Evidence |
| ----- | ------ | -------- |
| AC1   | TODO   | `error-enum-inventory.md` |
| AC2   | TODO   | Inventory decisions and `rg '#\[non_exhaustive\]'` per crate |
| AC3   | TODO   | Workspace check, clippy, and tests |
| AC4   | TODO   | Inventory and T4 commits |
| AC5   | TODO   | EPIC #1669 diff |

## Risks and Trade-offs

- Adding `#[non_exhaustive]` breaks exhaustive matches in downstream crates. Inside the workspace
  the compiler finds them; for crates already on crates.io (for example
  `torrust-tracker-configuration`), it is a semver-breaking change and needs a version bump.
- Wildcard arms hide future variants from downstream code. Map them to the most specific existing
  handling and keep them few.
- An inventory filtered only by name misses error types with other names; T1 uses public
  reachability and the `std::error::Error` implementation, not names alone.

## Implementation Completion Review

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md`.
- If no retrospective is needed, add a concise progress-log entry explaining why.
- When an independent reviewer receives this specification, it records its result in
  `agent-review-reports.md` using `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- Related issues: #2435, #1669
- Related PRs: #2445
- Related ADRs: [20261005145329](../../../adrs/20261005145329_return_result_only_for_concretely_fallible_public_apis.md), [20260629000000](../../../adrs/20260629000000_adopt_independent_package_versioning.md)
