---
schema-version: 1
doc-type: issue
issue-type: task
status: draft
priority: p3
epic: null
github-issue: null
spec-path: docs/issues/drafts/mine-pr-review-audit-records/ISSUE.md
branch: "chore/pr-review-data-mining-spec"
related-pr: null
last-updated-utc: "2026-09-29 11:55"
semantic-links:
  skill-links:
    - create-issue
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - .github/skills/dev/pr-reviews/process-pr-review/scripts/validate-audit-record.py
    - contrib/dev-tools/checks/frontmatter-validator
    - docs/adrs/20260519000000_define_global_cli_output_contract.md
    - docs/issues/open/2003-overhaul-guardrails-and-automation/EPIC.md
    - docs/issues/open/2278-2003-strengthen-pr-review-author-self-audit/EPIC.md
    - docs/issues/open/2264-2003-refactor-semantic-link-conventions/EPIC.md
    - docs/pr-reviews/README.md
    - docs/templates/PR-REVIEW-TEMPLATE.md
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Make PR Review Audit Records Minable for Recurring-Finding Analysis

**Suggested parent EPIC:** #2003 - Overhaul: Automation Tools and AI Agent Guardrails. The
suggestion is not settled: this draft is not a subissue yet, and `epic` stays `null` until a
maintainer decides where it belongs.

## Goal

Turn new PR review audit records under `docs/pr-reviews/` into a dataset that a deterministic
tool can read, so maintainers can find recurring review findings and decide which of them to
prevent before review. Prevention could be a new linter or check, a fix to an existing check, or
a correction to a skill or template.

## Background

Every PR review audit record lives in `docs/pr-reviews/pr-<PR_NUMBER>-review/PR-REVIEW.md`. The
unified template (#2219, #2233) already classifies each finding with fixed value lists for
`Author class`, `Severity`, `Category`, `Relationship`, `Disposition`, and `Thread state`. It
says these fields exist "for deterministic aggregation". Nothing uses them yet.

The maintainer wants to mine these records to answer questions such as:

- Which kinds of finding recur across PRs?
- Which findings a deterministic check could have caught before the PR reached review?
- Which findings come from a missing or contradictory skill or template rule?
- Which findings are noise that the review process should stop producing?

Three things block that today:

1. **No reliable way to tell formats apart.** Of about 126 record directories, only 43
   `PR-REVIEW.md` files use the unified `Finding ID` table. The rest use older formats: Copilot
   suggestion tables and `-copilot-suggestions-legacy` directories. A tool can only guess a
   record's format from its headings.
2. **No preventability field.** `Category` says what a finding is about, not whether it could
   have been prevented, or how. That is the main question the analysis has to answer.
3. **No export.** The Python prototype validator checks one record against GitHub data. No tool
   turns records into structured data for aggregation.

Layout discussion (2026-09-29): the maintainer asked whether `docs/pr-reviews/` should copy the
`drafts/`, `open/`, and `closed/` layout of `docs/issues/`. The conclusion was to keep it flat:

- A record is created from a PR that already has a number, so there is no draft phase.
- A post-merge finding can reopen a merged PR's record.
- The path pattern is hardcoded in the validator, the `agent-review-report-contract` check, the
  template, and the skills.
- Directory size is far below any filesystem or Git limit, and people can open a record with
  fuzzy file search.

Mining also rules out deleting old records: a record deleted from the working tree survives only
in Git history, which is much harder to mine.

The maintainer accepted that historical records will not be migrated. Mining covers only records
created in the new format from now on.

## Scope

### In Scope

- Record in `docs/pr-reviews/README.md` that the directory stays flat and that records are
  permanent data, not lifecycle documents to archive or delete.
- A frontmatter marker in the PR review template that identifies the record type and its body
  format version, and a strict frontmatter profile that validates it.
- A preventability classification for each finding, added to the template, to the canonical field
  roster in the `process-pr-review` skill, and to the audit validator.
- A read-only, offline Rust export that turns every marked record into JSON Lines, one line per
  finding, following the CLI output contract ADR.
- Usage documentation for the export, including one worked aggregation example.

### Out of Scope

- Migrating, backfilling, or re-marking historical records, including the 43 records in the
  unified format.
- Moving records into lifecycle subdirectories, or archiving or deleting records.
- The analysis itself: aggregation reports, ranked prevention proposals, and the linters or
  checks they lead to. Each is follow-up work that needs maintainer approval.
- Dashboards, CI integration of the export, and model-based (LLM) classification of findings.
- Porting the audit validator to Rust. EPIC #2278 already owns that (order 7, "Port the audit
  validator to Rust with parity fixtures").

## Architectural Decisions

- Related ADRs:
  [`docs/adrs/20260519000000_define_global_cli_output_contract.md`](../../../adrs/20260519000000_define_global_cli_output_contract.md)
  (export output contract).
- ADRs to create: a candidate root ADR, "Treat PR review audit records as a permanent, flat
  dataset". T1 decides whether a README statement is enough. An ADR is preferable if a future
  cleanup or archive skill could otherwise delete records.

During implementation, stop and create an ADR when a decision affects project architecture or
design patterns, selects an approach among meaningful alternatives, or has consequences future
contributors need to understand.

### Coordination

- **#2264 (semantic-link and frontmatter conventions)** owns frontmatter key naming. T2 must use
  its conventions for the record-type marker. It must not invent a parallel scheme.
- **#2278 (author self-audit)** owns the audit validator and its Rust port (orders 7 and 8). The
  T3 validator change and the T4 export parser must reuse that parser, so each record format is
  parsed in one place. If order 7 has not merged when this issue starts, T3's validator step and
  T4 are `BLOCKED` on it. Do not add a second Markdown parser for the same format.

## Design and Ownership Review

Not applicable. The work adds no child processes, asynchronous I/O, network readiness, resource
cleanup, or reusable test fixtures. The export is a synchronous, read-only file transformation.

## Bug-Fix Process

Not applicable. This is new capability, not a defect.

## Regression Test Strategy

Not applicable. This is not bug work. T2 through T4 use fixture tests as described below.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                        | Notes / Expected Output                                                                                                                                                                                                                                                 |
| --- | ------ | ------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | TODO   | Record the layout and retention decision    | `docs/pr-reviews/README.md` states that the directory is flat, that records are permanent, and why. Maintainer decides between a README statement and a root ADR.                                                                                                       |
| T2  | TODO   | Add a minable-record marker                 | The template's frontmatter declares the record type and a body format version, using #2264 conventions. `frontmatter-validator` gets a strict profile for it, with fixture tests. Unmarked records stay valid and are ignored by the export.                          |
| T3  | TODO   | Add a preventability classification         | A `Prevention` field with a fixed value list (proposal below) in the template tracking table, in the skill's canonical roster, and in the audit validator's accepted values. Validator step depends on #2278 order 7.                                                 |
| T4  | TODO   | Export marked records as JSON Lines         | A read-only, offline Rust command, reusing the #2278 audit parser, that emits one JSON object per finding. Each object carries the PR number, finding reference, every classification field, and format version. Fixture tests cover marked, unmarked, and malformed records. |
| T5  | TODO   | Document export usage and one aggregation   | The tool README documents the command, output fields, and exit codes. It includes one worked example that counts findings by `Category` and `Prevention`. `docs/pr-reviews/README.md` links to it and does not repeat it.                                               |

### Proposed `Prevention` Values

Maintainer review settles the list in T3. Proposal:

| Value                 | Meaning                                                                                               |
| --------------------- | ----------------------------------------------------------------------------------------------------- |
| `EXISTING_CHECK_GAP`  | An existing deterministic check should have caught the finding but was not run, was bypassed, or has a gap. |
| `NEW_CHECK_CANDIDATE` | The concern is an objective rule. A new linter, test, or validator could catch it before review.         |
| `GUIDANCE_GAP`        | A skill, template, or agent instruction was missing, ambiguous, or contradictory.                       |
| `JUDGMENT`            | The concern needs reviewer judgment and cannot reasonably be prevented deterministically.              |
| `UNASSESSED`          | The author has not classified the finding yet.                                                          |

The author classifies each finding against the concern, not the fix, as for `Category`.

## Commit Points

| Task | Coherent change set                                                                  | Commit policy                                        |
| ---- | ------------------------------------------------------------------------------------ | ---------------------------------------------------- |
| T1   | `docs/pr-reviews/README.md` layout and retention statement, or the ADR               | Commit after focused validation and required review. |
| T2   | Template frontmatter marker plus the strict frontmatter profile and its fixture tests | Commit after focused validation and required review. |
| T3   | `Prevention` field in the template, the skill roster, and the validator               | Commit after focused validation and required review. |
| T4   | Export command and its fixture tests                                                 | Commit after focused validation and required review. |
| T5   | Export usage documentation and README link                                           | Commit after focused validation and required review. |

Record a justified no-change decision in the task's evidence without creating an empty commit. For
test-producing work, use the `write-unit-test` skill and complete an explicit design review after
each passing test increment, before maintainer review and commit. Confirm that the test exposes the
one causal initial-state difference; its fixture owns only incidental mechanics; and the production
Act plus independently specified expected result remain visible. The review must use the mandatory
prose-first Arrange-Act-Assert comparison: write temporary prose for each section, refactor until
the code expresses it, remove redundant prose, and retain only irreducible context. Record this
review in task evidence or a file-local test plan. Assess helper boundaries by meaningful named
actions and abstraction-level alignment, not caller count. Use a Conventional Commit message with
the narrow affected scope, and sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/mine-pr-review-audit-records/ISSUE.md`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] Parent EPIC decided (suggested: #2003)
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

- 2026-09-29 11:55 UTC - GitHub Copilot - Drafted from the maintainer discussion on the
  `docs/pr-reviews/` layout: keep it flat, keep records permanently, make new records minable, and
  do not migrate historical records. No parent EPIC assigned; #2003 suggested.

## Acceptance Criteria

- [ ] AC1: `docs/pr-reviews/README.md` states that the directory is flat, that records are
  permanent, and why. If the maintainer chose an ADR in T1, the ADR exists and is linked.
- [ ] AC2: New records created from the template carry a record-type and body-format marker, and
  `frontmatter-validator` rejects a marked record whose marker is malformed.
- [ ] AC3: The template, the `process-pr-review` canonical roster, and the audit validator all
  define the same `Prevention` value list. The validator rejects values outside the list.
- [ ] AC4: The export reads only marked records and emits one JSON Lines object per finding under
  the CLI output contract. It reports malformed marked records as diagnostics with a nonzero exit
  code. It never writes to the repository or contacts GitHub.
- [ ] AC5: The export parses records with the same parser as the audit validator.
- [ ] AC6: Historical records are unchanged and do not fail any new check.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test` for `frontmatter-validator` and for the crate that owns the audit parser and export
- `frontmatter-validator docs/pr-reviews` (confirms that historical records still pass; `--all`
  is unsuitable because it exits `1` on unrelated legacy issue specs)
- Pre-push checks

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                             | Human-oriented command/steps                                                                                                 | Expected Result                                                                  | Status | Evidence                                     |
| --- | ------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------- | ------ | -------------------------------------------- |
| M1  | Create a record from the template    | Start an audit for a real PR from the updated template and fill in at least two findings with `Prevention` values            | The marker is present, and the frontmatter and audit validators both pass        | TODO   | `manual-verification-evidence.md` section V1 |
| M2  | Export the whole directory           | Run the export over `docs/pr-reviews/`                                                                                        | Output has one line per finding from marked records only; historical records are skipped | TODO   | `manual-verification-evidence.md` section V2 |
| M3  | Aggregate the export                 | Pipe the export through the documented aggregation example                                                                    | Counts by `Category` and `Prevention` match a hand count of the marked records   | TODO   | `manual-verification-evidence.md` section V3 |
| M4  | Reject a malformed marked record     | Copy a marked record to a scratch path, corrupt a `Prevention` value, and run the export and validator on it                  | Both report an actionable diagnostic and exit nonzero; no repository file changes | TODO   | `manual-verification-evidence.md` section V4 |

Notes:

- Manual verification is mandatory even when automated tests pass.
- Record the toolchain for every command result where it can affect behavior.
- Create `manual-verification-evidence.md` from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`.

### Disposable Verification Scripts

None planned.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence           |
| ----- | ---------------------- | ------------------ |
| AC1   | TODO                   | {test/log/PR link} |
| AC2   | TODO                   | {test/log/PR link} |
| AC3   | TODO                   | {test/log/PR link} |
| AC4   | TODO                   | {test/log/PR link} |
| AC5   | TODO                   | {test/log/PR link} |
| AC6   | TODO                   | {test/log/PR link} |

## Risks and Trade-offs

- **The dataset starts empty.** Because historical records are not migrated, only records created
  after T2 and T3 count. Useful aggregates need weeks of PRs. The maintainer accepted this in
  exchange for not rewriting history. A later issue can backfill the unified-format records if the
  wait proves too long.
- **Classification burden on authors.** A new field adds work to every audit. Mitigation: a short
  value list, an `UNASSESSED` escape value, and classification against the concern, like
  `Category`.
- **Inconsistent classification.** Different authors may classify the same concern differently.
  Mitigation: precise value definitions in the skill roster. Recurring disagreements are
  themselves a mining signal.
- **Dependency on #2278 order 7.** The export waits for the Rust validator. The alternative, a
  separate parser, would let two parsers of one format drift apart.
- **Frontmatter convention churn.** #2264 may still change key naming. Mitigation: T2 follows
  #2264's current conventions and is versioned.

## Implementation Completion Review

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from
  `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this directory.
- If no retrospective is needed, add a concise progress-log entry explaining why.
- When an independent reviewer receives this folder-style specification, it records its result
  in `agent-review-reports.md` using `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- Suggested parent EPIC: #2003
- Related issues: #2219, #2233, #2264, #2278
- Related ADRs: `docs/adrs/20260519000000_define_global_cli_output_contract.md`
