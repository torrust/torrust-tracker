---
schema-version: 1
doc-type: issue
issue-type: task
status: done
priority: p2
epic: 2912
github-issue: 2918
spec-path: docs/issues/closed/2918-2912-si-6-drop-located-error-and-enforce-the-rule/ISSUE.md
branch: "2918-drop-located-error-and-enforce-the-rule"
related-pr: 3018
last-updated-utc: "2026-11-24 12:43"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .claude/skills/fix-bug/SKILL.md
    - .claude/skills/create-issue/SKILL.md
    - docs/issues/open/2912-remove-source-locations-from-errors/EPIC.md
    - docs/issues/closed/2804-location-caller-track-caller-audit/inventory.md
    - packages/tracker-client/src/udp/mod.rs
    - contrib/dev-tools/checks/source-location-check/src/lib.rs
---

<!-- skill-link: create-issue -->

# Issue #2918 - Drop `torrust-located-error` from the workspace and enforce the rule

Parent EPIC: [Remove source locations from errors](../2912-remove-source-locations-from-errors/EPIC.md) (#2912). Sub-issue SI-6 of the EPIC.

## Goal

Remove the last uses of `torrust-located-error` from the workspace and add a repository check that
fails when non-test code records a source location in an error, so the rule of the ADR stays true.

## Background

After SI-3 to SI-5 the only remaining user of the crate is expected to be
`packages/tracker-client`, which imports the type alias `torrust_located_error::DynError`
(`src/udp/mod.rs:5`) and lists the crate in its `Cargo.toml`.

The audit looked for a lint (`docs/issues/closed/2804-location-caller-track-caller-audit/inventory.md`, T3): neither rustc nor clippy 1.99.1 has one
for `Location::caller()` in a function that is not `#[track_caller]`. With the ADR the rule is
simpler than the one the audit considered: non-test code does not use `Location::caller()`,
`#[track_caller]` or `torrust_located_error` at all. A text-based check can enforce that.

Two standalone crates in other repositories carry the same pattern (`inventory.md`, standalone
crates): `torrust-info-hash` 0.2.0 builds `ConversionError` with `Location::caller()` in a `TryFrom`
impl (`info_hash.rs:273`, `:279`), and `torrust-located-error` 3.0.0 is the wrapper itself.

## Scope

### In Scope

- Replace `torrust_located_error::DynError` in `packages/tracker-client` with a local alias or the
  spelled-out type, and remove the dependency from its `Cargo.toml`.
- Confirm no workspace `Cargo.toml` depends on `torrust-located-error` and that `Cargo.lock` no
  longer lists it (unless a registry dependency still pulls it in; record which).
- A repository check under `contrib/dev-tools/checks/` (the place of the existing checks) that fails
  when `Location::caller()`, `#[track_caller]` or `torrust_located_error` appears in a `.rs` file
  outside `#[cfg(test)]` modules, `tests/` directories and an explicit allow-list with a reason per
  entry. Wire it where the other checks run (pre-commit and CI).
- Draft the text of the two upstream issues (remove the location from
  `torrust_info_hash::ConversionError`; decide the future of `torrust-located-error`) and hand them
  to the maintainer, who opens them in those repositories.

### Out of Scope

- Changing `torrust-info-hash` or `torrust-located-error` (other repositories).
- A custom clippy or rustc lint.
- Extracting `tracker-client` (#2594).

## Architectural Decisions

- Related ADRs: the ADR created by sub-issue SI-1 of the parent EPIC (errors do not record source locations); `docs/adrs/20261005145329_return_result_only_for_concretely_fallible_public_apis.md`
  (`#[non_exhaustive]` policy for public error enums).
- ADRs to create: None known. If the check needs a policy for the allow-list, record it in the ADR of SI-1 as an amendment

During implementation, stop and create an ADR when a decision affects project
architecture or design patterns, selects an approach among meaningful
alternatives, or has consequences future contributors need to understand. Do not
create ADRs for routine implementation details or style choices already governed
by project conventions.

## Design and Ownership Review

Not applicable: the work changes error types and their text. It adds no child process,
asynchronous I/O, network readiness, resource cleanup or reusable test fixture.

## Bug-Fix Process

Not applicable: this sub-issue removes a dependency and adds a check; the behavior fixes are SI-2 to SI-5.

## Regression Test Strategy

Not applicable: this sub-issue removes a dependency and adds a check; the behavior fixes are SI-2 to SI-5.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | DONE | Remove the last uses | `tracker-client` without the dependency; `grep -rn 'located' --include=Cargo.toml .` finds no workspace manifest; `Cargo.lock` state recorded. Done: `udp::Error` kept its type and lost the unused `From<Error> for DynError`; `Cargo.lock` no longer lists `torrust-located-error` and no registry crate pulls it in. |
| T2 | DONE | Design the check | Short design note in this spec: a Rust crate like the existing checks in `contrib/dev-tools/checks/`, how test code is recognized, the allow-list format, false-positive cases tried. See "Design note" below. |
| T3 | DONE | Implement the check with tests | The check, its tests (a fixture that must fail and one that must pass) and its wiring into pre-commit and CI. |
| T4 | DONE | Prove the check fails | Evidence of the check failing on a temporary `Location::caller()` added to production code, then passing after its removal. |
| T5 | DONE | Draft the upstream issues | Two issue texts in this folder, handed to the maintainer; links added here once opened. |

### Design note (T2)

- **Tool:** the crate `source-location-check` in `contrib/dev-tools/checks/source-location-check/`, a
  library (detection) plus a CLI adapter that follows the global CLI output contract
  (`no-stdout-result`: JSON lines on stderr, exit 0, 1 or 2). It runs from the repository root over
  the `.rs` files of `git ls-files --cached --others --exclude-standard`, so a staged new file counts.
- **Forbidden text** (non-comment lines): `Location::caller`, `track_caller`, `torrust_located_error`,
  `torrust-located-error`. The crate builds these with `concat!` so its own source does not match.
- **Scanned text:** the source with comments (line, nested block) and the content of string, raw
  string and character literals blanked, so text in them is never matched and never counts as a delimiter.
- **Test code**: the item after a `#[cfg(test)]` attribute (it ends at the `;` or the closing delimiter
  that balances `{}`, `()` and `[]`, so `use std::{a, b};` ends at its semicolon); files declared by
  `#[cfg(test)] mod name;` (resolved under the declaration's inline-module ancestry) and everything
  below them, unless a production declaration, for example in `main.rs`, also includes the file; files
  under any `tests/` directory; and `packages/test-helpers/` (SI-8 #2996 moves the shared helper there).
- **Allow-list:** `contrib/dev-tools/checks/source-location-check/allow-list.txt`, `<path>: <reason>`
  per line; an entry without a reason fails the check. It starts, and must stay, empty: SI-2 to SI-5
  migrated every file.
- **False positives tried:** `#[track_caller]` helpers in `#[cfg(test)]` modules (six files),
  files declared `#[cfg(test)] mod x;` (`http-protocol/src/test_helpers.rs`,
  `tracker-core/src/test_helpers.rs`), `tests/` helpers in three server packages, comments that
  mention the patterns, and production code after a `#[cfg(test)]` module (still flagged). Known
  limits, all of which fail safe (the code is flagged, never skipped): a `#[cfg(all(test, ...))]` or
  `#[cfg_attr]` form and a `#[cfg(test)]` that does not start its line or spans several lines are not
  recognized as test code; a `#[path = "..."]` attribute is resolved only when it is on one line, and the
  file it names is resolved lexically against the declaring file.
- **Wiring:** a `pre-commit.sh` step (`-- --staged`, which reads the Git index for the sources, the module
  classification and the allow-list, so the check sees what the commit contains) and a `Source Locations`
  job in `testing.yaml` (the working tree of the checkout); the crate is a workspace member and admitted
  in `.dockerignore`.

## Commit Points

Map implementation-plan tasks that change code, tests, configuration, or documentation to small,
coherent commit opportunities. A commit point completes one independently reviewable behavior,
refactor, or evidence increment; do not group unrelated changes merely to reduce commit count.

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T1 | The dependency removal | Commit after focused validation and required review. |
| T3 | The check, its tests and its wiring | Commit after focused validation and required review. |
| T2, T4, T5 | Design note, evidence and the upstream issue drafts | Commit after focused validation and required review. |

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

- [x] Folder-style spec drafted in `docs/issues/drafts/` and moved to `docs/issues/open/`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [ ] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [x] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [x] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [x] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [x] Committer verified spec progress is up to date before commit
- [x] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

Append one line per meaningful update. When an AI agent did the work, name its model after the
agent, for example `GitHub Copilot (Claude Opus 5.5)`, so later reviews can relate outcomes to
the model that produced them.

- 2026-11-11 17:05 UTC - Claude Code (Fable 5.2) - Drafted from the audit of issue #2804 (`docs/issues/open/2804-location-caller-track-caller-audit/inventory.md`, `options.md`) as a sub-issue of the EPIC draft. Not yet approved; line numbers are those of `develop` at `4f0af379a` and must be re-checked before implementation.
- 2026-11-17 17:30 UTC - Claude Code (Claude Opus 5.5) - EPIC coordinator: the maintainer approved the specification on 2026-11-17; issue #2918 created as `josecelano-bot` and the specification moved from `docs/issues/drafts/` to `docs/issues/open/` (SI-6 of EPIC #2912).
- 2026-11-23 21:27 UTC - Claude Code (Claude Sonnet 5.5) - Implemented SI-6: removed the last use of `torrust-located-error` (`tracker-client`; the unused `From<Error> for DynError` impl is gone, the crate left `Cargo.lock`), added the `source-location-check` tool with tests, wired it into pre-commit and CI, updated the docs and the #2583 spec, and drafted the two upstream issues. No retrospective: the work matched the spec; the only deviation is that the check also exempts files declared `#[cfg(test)] mod x;`, which the spec did not list.
- 2026-11-23 22:27 UTC - Claude Code (Claude Sonnet 5.5) - Review round 1 of PR #3018 (audit: `docs/pr-reviews/pr-3018-review/PR-REVIEW.md`): the check now blanks comments and strings, ends a `#[cfg(test)]` item by balancing delimiters, resolves test module files under their inline-module ancestry and keeps a file checked when a production declaration includes it; M1 was re-run through the real hook (V1 step 4). The two earlier progress-log stamps of 2026-11-23 that were estimated ahead are corrected in place to 21:27.
- 2026-11-24 00:44 UTC - Claude Code (Claude Sonnet 5.5) - Review rounds 2 to 5 of PR #3018 (audit: `docs/pr-reviews/pr-3018-review/PR-REVIEW.md`) hardened the check (balanced delimiters, masking of comments, strings, raw byte and raw C strings, module ancestry, production declarations, explicit `#[path]` modules) and made the pre-commit step read the Git index (`--staged`); M1 was re-run through the hook after that change (V1 step 5).
- 2026-11-24 12:43 UTC - Claude Code (Claude Sonnet 5.5) - PR #3018 merged (`df8ff6fc3`) and issue #2918 closed; spec archived to `docs/issues/closed/`. AC4 stays open: the two upstream issues are still to be opened by the maintainer from the drafts in this folder.

## Acceptance Criteria

- [x] AC1: No workspace package depends on `torrust-located-error`.
- [x] AC2: The check fails on `Location::caller()`, `#[track_caller]` or `torrust_located_error` in non-test code and passes on the tree, proven by its tests and by the T4 evidence.
- [x] AC3: The check runs in pre-commit and in CI.
- [ ] AC4: The two upstream issue drafts exist and the maintainer has them. (Drafts exist; the maintainer is told in the PR.)
- [x] `linter all` exits with code `0`
- [x] Relevant tests pass
- [x] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [x] Documentation is updated when behavior/workflow changes

## Verification Plan

Define verification before implementation starts and execute it before closing the issue.

### Automatic Checks

- `linter all`
- Relevant tests for changed components
- The new check on the whole tree
- Pre-push checks (when applicable)

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| M1 | A contributor reintroduces a location | Add `location: Location::caller()` to a production error on a scratch branch and commit | The pre-commit check fails and names the file and the rule | DONE (the hook's command run directly, then a real `git commit` rejected by the hook) | `manual-verification-evidence.md` section V1 |
| M2 | Test helpers stay allowed | Run the check on the tree, which has `#[track_caller]` assertion helpers in test code | The check passes | DONE | `manual-verification-evidence.md` section V2 |

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

Temporary scripts may automate an issue-local verification scenario, but they
are neither maintained automatic tests nor manual-verification evidence. Before
creating one, record in the issue specification:

- why temporary automation is better for this concrete scenario than a
  maintained Rust automatic test;
- the script's issue-local path, what it verifies, and its intended removal or
  retention owner; and
- when using Python instead of Rust, why Rust is not suitable for that specific
  script.

Keep the script in the issue-specification folder so later reviewers can inspect
the verification performed. Promote durable product-behavior checks into Rust
automatic tests when practical, then remove the disposable script.

None planned for this issue.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| --- | --- | --- |
| AC1 | DONE | `grep -rn located --include=Cargo.toml .` finds no manifest; `Cargo.lock` has no `torrust-located-error` |
| AC2 | DONE | `source-location-check` unit and CLI tests; `manual-verification-evidence.md` V1 |
| AC3 | DONE | `pre-commit.sh` step and the `Source Locations` job of `testing.yaml` |
| AC4 | TODO | `upstream-issue-info-hash.md`, `upstream-issue-located-error.md`; links after the maintainer opens them |

## Risks and Trade-offs

- **False positives in test detection.** A text check cannot parse Rust. Mitigation: the allow-list
  with reasons, and tests for the detection.
- **A registry crate still pulls `torrust-located-error` in.** Mitigation: T1 records it; removal
  then waits for that crate.
- **Runs before SI-3 to SI-5 are merged.** Mitigation: the EPIC orders it last; the check's
  allow-list may start with the not-yet-migrated files and must end empty of them.

## Implementation Completion Review

After implementation, compare the result with this specification. Record
invalidated assumptions, material design changes, unexpected validation
findings, and reusable lessons.

- Retrospective: none needed; see the progress log entry
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
- Audit: `docs/issues/closed/2804-location-caller-track-caller-audit/ISSUE.md`, `inventory.md`, `options.md`, `experiments.md` (issue #2804)
- Related issues: #2804 (audit), #384 (external report), #2583 (first publish of the workspace crates)
- Related: #2594 (extraction of `torrust-tracker-client`)
