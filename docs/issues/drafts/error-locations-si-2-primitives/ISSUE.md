---
schema-version: 1
doc-type: issue
issue-type: bug
status: draft
priority: p2
epic: null
github-issue: null
spec-path: docs/issues/drafts/error-locations-si-2-primitives/ISSUE.md
branch: "{issue-number}-error-locations-si-2-primitives"
related-pr: null
last-updated-utc: "2026-11-11 17:05"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/debugging/fix-bug/SKILL.md
    - .github/skills/dev/planning/create-issue/SKILL.md
    - docs/issues/drafts/remove-source-locations-from-errors/EPIC.md
    - docs/issues/open/2804-location-caller-track-caller-audit/inventory.md
    - packages/primitives/src/peer.rs
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Remove source locations from the errors of `torrust-tracker-primitives`

Parent EPIC: [Remove source locations from errors](../remove-source-locations-from-errors/EPIC.md) (draft, GitHub issue not
created yet, so `epic` is `null` and this folder carries no EPIC number). Sub-issue SI-2 of the EPIC.

## Goal

Remove the `location` field and its text from `ParsePeerRoleError` and `peer::IdConversionError` in
`packages/primitives`, so that their messages state the invalid input and nothing about source files.

## Background

This is the defect that started the audit: finding F3 of the primitives test work under EPIC #1347.
`"x".parse::<PeerRole>()` returns an error whose text ends in `packages/primitives/src/peer.rs:73:27`,
the same line for every caller (`docs/issues/open/2804-location-caller-track-caller-audit/manual-verification-evidence.md`, section D1).

Sites (inventory rows P1 and P2):

| Error variant | Format (line) | Built at | Shape |
| --- | --- | --- | --- |
| `ParsePeerRoleError::InvalidPeerRole` | `"invalid param {raw_param} in {location}"` (`peer.rs:83`) | `peer.rs:73`, `FromStr for PeerRole` | trait impl reached through `str::parse` |
| `IdConversionError::NotEnoughBytes` | `"not enough bytes for peer id: {message} {location}"` (`peer.rs:331`) | `peer.rs:391`, `TryFrom<Vec<u8>> for Id` | trait impl reached through `try_into` |
| `IdConversionError::TooManyBytes` | `"too many bytes for peer id: {message} {location}"` (`peer.rs:336`) | `peer.rs:397`, same impl | same |

The audit found no production path that prints these two errors (`inventory.md`, T2 table, last
row): they are not used outside `peer.rs`. The exposure is to library consumers of the published
crate. The crate was published at 3.0.0 and the publish plan already schedules 4.0.0
(`docs/issues/open/2583-1669-first-publish-of-workspace-crates/ISSUE.md`), so removing a public field
adds no release step if it lands before that publish.

## Scope

### In Scope

- Remove `location` from the three variants and from their `#[error(..)]` formats.
- Remove the three `Location::caller()` calls and the unused `std::panic::Location` import.
- Keep or improve the context in the message (`raw_param`, and the byte counts in `message`).
- Update the tests of `peer.rs` that build or match these variants.
- Record the breaking change for the 4.0.0 release notes of the crate, where the publish plan tracks them.

### Out of Scope

- Other packages (SI-3 to SI-5).
- Any other change to `PeerRole` or `peer::Id` parsing behavior.
- Publishing the crate (#2583).

## Architectural Decisions

- Related ADRs: the ADR created by sub-issue SI-1 of the parent EPIC (errors do not record source locations); `docs/adrs/20261005145329_return_result_only_for_concretely_fallible_public_apis.md`
  (`#[non_exhaustive]` policy for public error enums).
- ADRs to create: None known

During implementation, stop and create an ADR when a decision affects project
architecture or design patterns, selects an approach among meaningful
alternatives, or has consequences future contributors need to understand. Do not
create ADRs for routine implementation details or style choices already governed
by project conventions.

## Design and Ownership Review

Not applicable: the work changes error types and their text. It adds no child process,
asynchronous I/O, network readiness, resource cleanup or reusable test fixture.

## Bug-Fix Process

Removing a location that a client can read is substantively a bug fix (misleading output), so this
issue follows `.github/skills/dev/debugging/fix-bug/SKILL.md`:

- analysis: the call paths and audiences are in the audit inventory
  (`docs/issues/open/2804-location-caller-track-caller-audit/inventory.md`, T2 table); re-check them against the tree before the first change;
- real-artifact reproduction: a scratch program or `cargo test` snippet that prints `"x".parse::<PeerRole>().unwrap_err()` and `peer::Id::try_from(vec![0; 19]).unwrap_err()` before the change, like section D1 of the audit evidence;
- regression-test boundary: see Regression Test Strategy;
- red regression-test evidence, production fix, green test evidence, and the same reproduction run
  again after the last production change, all recorded in `manual-verification-evidence.md`.

## Regression Test Strategy

- **Smallest boundary**: a unit test at the error's constructor or `FromStr`/`TryFrom` seam that
  asserts the rendered `Display` holds no source path and still names what failed. It needs no
  server. Write it first and record its red output.
- **Trait-dispatch case**: where the error is built in a `FromStr` or `TryFrom` impl, call through
  `str::parse::<T>()` or `try_into()`, the call shape real callers use.
- **Portable path check**: a helper forbids these substrings: `.rs:`, `packages/`, `packages\`,
  `src/`, `src\`, `.cargo/registry` and `rustlib/src`, so workspace paths, registry checkouts, CI
  checkout paths and Windows paths are all caught.
- **No server boundary**: no production path prints these errors, so the unit tests are the whole regression suite of this sub-issue.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | TODO | Reproduce and record | Current text of the three errors recorded in `manual-verification-evidence.md` (toolchain named). |
| T2 | TODO | Red tests | Unit tests through `str::parse::<PeerRole>()` and `try_into()`/`Id::try_from` asserting no source path and the presence of the input; red output recorded. |
| T3 | TODO | Remove the field, the format text and the captures | The three variants have no `location`; `grep -n 'Location' packages/primitives/src` finds nothing outside tests. |
| T4 | TODO | Green and recheck | Green test output, then the T1 reproduction run again. |
| T5 | TODO | Record the breaking change | Entry where the 4.0.0 changes of the crate are tracked (publish plan checklist row or the crate changelog, whichever the plan uses at that time). |

## Commit Points

Map implementation-plan tasks that change code, tests, configuration, or documentation to small,
coherent commit opportunities. A commit point completes one independently reviewable behavior,
refactor, or evidence increment; do not group unrelated changes merely to reduce commit count.

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T2 | The red regression tests | Commit after focused validation and required review. |
| T3 | The production change with the tests now green | Commit after focused validation and required review. |
| T1, T4, T5 | Evidence and release-note entry | Commit after focused validation and required review. |

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

- [x] Folder-style spec drafted in `docs/issues/drafts/error-locations-si-2-primitives/ISSUE.md`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] GitHub issue created and issue number added to this spec
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

Append one line per meaningful update. When an AI agent did the work, name its model after the
agent, for example `GitHub Copilot (Claude Opus 5.5)`, so later reviews can relate outcomes to
the model that produced them.

- 2026-11-11 17:05 UTC - Claude Code (Fable 5.2) - Drafted from the audit of issue #2804 (`docs/issues/open/2804-location-caller-track-caller-audit/inventory.md`, `options.md`) as a sub-issue of the EPIC draft. Not yet approved; line numbers are those of `develop` at `4f0af379a` and must be re-checked before implementation.

## Acceptance Criteria

- [ ] AC1: `ParsePeerRoleError` and `IdConversionError` have no `location` field.
- [ ] AC2: `Display` of the three variants holds no source path and names the invalid input, proven by tests that go through `str::parse` and `try_into`.
- [ ] AC3: `packages/primitives/src` has no `Location::caller()` outside test code.
- [ ] AC4: The breaking change is recorded for the 4.0.0 release of the crate.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

Define verification before implementation starts and execute it before closing the issue.

### Automatic Checks

- `linter all`
- Relevant tests for changed components
- Pre-push checks (when applicable)

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| M1 | Parse an invalid peer role as a library consumer | In a scratch crate that depends on the workspace `torrust-tracker-primitives` by path, print `"x".parse::<PeerRole>().unwrap_err()` | The text names `x` and holds no file path or line | TODO | `manual-verification-evidence.md` section V1 |
| M2 | Convert a short peer id | Same crate: print `peer::Id::try_from(vec![0; 19]).unwrap_err()` | The text states the byte count and holds no file path or line | TODO | `manual-verification-evidence.md` section V2 |

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

### Disposable Verification Scripts

None planned. If one is needed, record here why it is better than a maintained Rust test, its
issue-local path, what it verifies and who removes it, before creating it.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| --- | --- | --- |
| AC1 | TODO | {test/log/PR link} |
| AC2 | TODO | {test/log/PR link} |
| AC3 | TODO | {test/log/PR link} |
| AC4 | TODO | {test/log/PR link} |

## Risks and Trade-offs

- **A consumer matches on the field.** Mitigation: the change ships in the already planned major
  release and is listed in its notes.
- **The change lands after 4.0.0 is published.** Mitigation: the EPIC orders this sub-issue before
  the first publish; if that fails, the sub-issue needs a new major and the EPIC must say so.

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
  `docs/templates/AGENT-REVIEW-REPORTS.md`. Do not create that artifact for a
  legacy standalone specification or when no folder-style specification was
  supplied.

## References

- Parent EPIC draft: `docs/issues/drafts/remove-source-locations-from-errors/EPIC.md`
- Audit: `docs/issues/open/2804-location-caller-track-caller-audit/ISSUE.md`, `inventory.md`, `options.md`, `experiments.md` (issue #2804)
- Related issues: #2804 (audit), #384 (external report), #2583 (first publish of the workspace crates)
- Reporting work: `docs/issues/open/2824-1347-add-tests-primitives/` (EPIC #1347)
