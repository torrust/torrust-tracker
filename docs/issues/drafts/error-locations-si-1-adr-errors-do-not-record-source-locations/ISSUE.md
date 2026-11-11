---
schema-version: 1
doc-type: issue
issue-type: task
status: draft
priority: p1
epic: null
github-issue: null
spec-path: docs/issues/drafts/error-locations-si-1-adr-errors-do-not-record-source-locations/ISSUE.md
branch: "{issue-number}-error-locations-si-1-adr-errors-do-not-record-source-locations"
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
    - .github/skills/dev/rust-code-quality/handle-errors-in-code/SKILL.md
    - .github/skills/dev/planning/create-adr/SKILL.md
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Record the decision that errors do not record source locations

Parent EPIC: [Remove source locations from errors](../remove-source-locations-from-errors/EPIC.md) (draft, GitHub issue not
created yet, so `epic` is `null` and this folder carries no EPIC number). Sub-issue SI-1 of the EPIC.

## Goal

Write the ADR that says errors in this workspace do not record or print source locations, and
change the `handle-errors-in-code` skill so that it gives that rule instead of recommending
`torrust-located-error`. Every other sub-issue of the EPIC implements this decision.

## Background

The audit of issue #2804 found that the source location recorded in errors is never the caller's
(`docs/issues/open/2804-location-caller-track-caller-audit/inventory.md`): none of the 38 production `Location::caller()` sites is in a
`#[track_caller]` function, and `#[track_caller]` cannot fix any of them, because they are reached
through `str::parse`, `try_into`, closures, `async fn` or `async` blocks (`options.md`, fact 6). The
location is a fixed library line, and 23 `Display` formats print it into text that reaches HTTP, UDP
and REST clients, event payloads and logs. `LocatedError` (`torrust-located-error` 3.0.0) prints its
own location as well.

Issue #384 (2023) reported the exposure. The answer then was that the information helps
administrators and developers more than it risks. The audit removes that argument, and the
maintainer listed three further objections: the text lets an attacker infer the running version, it
couples the user-facing interface to the implementation, and it makes messages unstable for
consumers.

The current guidance points the other way: the `handle-errors-in-code` skill has a section "The
`located-error` Package" and a checklist item "Consider `located-error` for diagnostics-rich errors".
No ADR covers located errors.

## Scope

### In Scope

- An ADR in `docs/adrs/` (repository-wide decision), created with the `create-adr` skill, that records:
  the context (audit result, #384, the three objections), the decision, the alternatives considered
  (the four options of `options.md`) and the consequences.
- The decision to record, for review:
  1. Error types have no `location` field and do not call `Location::caller()`.
  2. The workspace does not use `torrust-located-error`; a source error is kept as a plain `#[source]`.
  3. `Display` of an error states what failed and the offending input. It holds no source path.
  4. Text sent to a client is the error's own message. A source error from a crate the workspace
     does not control is not formatted into client text.
  5. Developer diagnostics go to `tracing` events at the site, which record file and line on the
     server only.
  6. `#[track_caller]` stays allowed in test helpers and assertion helpers.
- Update `.github/skills/dev/rust-code-quality/handle-errors-in-code/SKILL.md`: remove the
  `located-error` section and checklist item, add the rule and a link to the ADR.
- Add the ADR to `docs/adrs/index.md`.

### Out of Scope

- Any change to Rust code (sub-issues SI-2 to SI-6).
- Changes to `torrust-located-error` or `torrust-info-hash` (other repositories).

## Architectural Decisions

- Related ADRs: none about located errors today; `docs/adrs/20261005145329_return_result_only_for_concretely_fallible_public_apis.md`
  (`#[non_exhaustive]` policy for public error enums).
- ADRs to create: `Errors do not record source locations` (root collection `docs/adrs/`, because it binds every package)

During implementation, stop and create an ADR when a decision affects project
architecture or design patterns, selects an approach among meaningful
alternatives, or has consequences future contributors need to understand. Do not
create ADRs for routine implementation details or style choices already governed
by project conventions.

## Design and Ownership Review

Not applicable: the work changes error types and their text. It adds no child process,
asynchronous I/O, network readiness, resource cleanup or reusable test fixture.

## Bug-Fix Process

Not applicable: this sub-issue records a decision and changes guidance; it changes no behavior.

## Regression Test Strategy

Not applicable: this sub-issue records a decision and changes guidance; it changes no behavior.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | TODO | Draft the ADR with the `create-adr` skill | ADR file in `docs/adrs/` with context, decision (the six points), alternatives and consequences; listed in `docs/adrs/index.md`. |
| T2 | TODO | Update the `handle-errors-in-code` skill | The `located-error` section and checklist item are replaced by the rule and a link to the ADR. |
| T3 | TODO | Search for other guidance that recommends located errors | `grep -rn 'located' .github docs AGENTS.md packages/*/README.md` reviewed; each hit updated or recorded as historical. |

## Commit Points

Map implementation-plan tasks that change code, tests, configuration, or documentation to small,
coherent commit opportunities. A commit point completes one independently reviewable behavior,
refactor, or evidence increment; do not group unrelated changes merely to reduce commit count.

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T1 | The ADR and its index entry | Commit after focused validation and required review. |
| T2-T3 | The skill update and any other guidance change | Commit after focused validation and required review. |

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

- [x] Folder-style spec drafted in `docs/issues/drafts/error-locations-si-1-adr-errors-do-not-record-source-locations/ISSUE.md`
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

- [ ] AC1: The ADR exists in `docs/adrs/`, is listed in the index and states the six decision points or the form the reviewers agreed.
- [ ] AC2: The `handle-errors-in-code` skill no longer recommends `torrust-located-error` and links to the ADR.
- [ ] AC3: No live guidance in the repository recommends recording a source location in an error.
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
| M1 | Read the guidance as a contributor | Open the `handle-errors-in-code` skill and follow its error checklist for a new error variant | The checklist leads to an error without a `location` field and points to the ADR | TODO | `manual-verification-evidence.md` section V1 |

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

## Risks and Trade-offs

- **The decision is contested in review.** Mitigation: this sub-issue goes first and the code
  sub-issues wait for it.
- **Guidance and code disagree while the EPIC is in progress.** Mitigation: the ADR states that
  existing uses are removed by the EPIC's sub-issues and links to the EPIC.

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
