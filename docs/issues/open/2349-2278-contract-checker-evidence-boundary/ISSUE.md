---
schema-version: 1
doc-type: issue
issue-type: task
status: planned
priority: p2
epic: 2278
github-issue: 2349
spec-path: docs/issues/open/2349-2278-contract-checker-evidence-boundary/ISSUE.md
branch: "2349-2278-contract-checker-evidence-boundary-spec"
related-pr: null
last-updated-utc: "2026-09-26 13:33"
semantic-links:
  skill-links:
    - create-issue
    - process-pr-review
  related-artifacts:
    - "issue #2278"
    - "issue #2003"
    - docs/issues/open/2278-2003-strengthen-pr-review-author-self-audit/EPIC.md
    - docs/issues/open/2278-2003-strengthen-pr-review-author-self-audit/retrospective-improvement-matrix.md
    - contrib/dev-tools/checks/agent-review-report-contract/src/main.rs
    - docs/adrs/20260519000000_define_global_cli_output_contract.md
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: create-issue -->

# Issue #2349 - Make `agent-review-report-contract` State What It Reads

**Parent EPIC:** #2278 - Strengthen PR Review Author Self-Audit and Evidence Generation

Subissue 6 of #2278. It implements the adopted false-evidence part of friction-register item F56,
item F57, and the PR #2271 retrospective item "Explain what `agent-review-report-contract` reads
so it is not cited as audit evidence".

## Goal

Make `agent-review-report-contract` say, in its documentation and in its own output, exactly which
files it checks and that it never reads an audit record, so that nobody cites a passing run as
evidence for an audit claim. Apply one consistent rule for how much text each template field pin
contains.

## Background

`contrib/dev-tools/checks/agent-review-report-contract` is a workspace binary that checks a fixed
set of 15 workflow documents for required and forbidden text: the `process-pr-review`,
`fetch-review-threads`, and `resolve-review-threads` skills and the two compatibility redirect
skills; `PR-REVIEW-TEMPLATE.md`, `AGENT-REVIEW-REPORTS.md`, and `REVIEW-FINDINGS.md`; four review
agents and the Copilot-suggestions prompt; `docs/agents/orchestration.md`; and the semantic
skill-link convention. It reads no file under `docs/pr-reviews/`. At `develop`:

- No pre-commit step, CI workflow, skill, agent, or template invokes or mentions it, so nothing
  states when to run it or what a pass means.
- Three audit records nevertheless cite it as `Current-tree verification` (PR #2271 F16, PR #2279,
  PR #2300). A pass shows only that the workflow documents still contain their pinned text; it says
  nothing about the audit record being verified (F56, false-evidence part).
- It prints plain text on success (stdout) and on failure (stderr), and it is not classified in the
  CLI output contract ADR. The ADR requires an existing command to be migrated when it is touched.
- Its template pins mix granularity (F57): `AGENT-REVIEW-REPORTS.md` fields are pinned as bare
  labels such as `- Verdict:` (`src/main.rs:21`), while `PR-REVIEW-TEMPLATE.md` fields are pinned
  with their full placeholder, such as
  `- Resolution reference: <UNIQUE_COMMIT_SUBJECT_OR_REPLY_URL>` (`src/main.rs:50`). No rule says
  which to use, so a harmless placeholder rename fails the check in one file and a dropped field
  passes silently in another only if its label survives.

## Scope

### In Scope

- Crate documentation (`//!` module docs) stating the checked file list, that audit records under
  `docs/pr-reviews/` are never read, and that a pass is not evidence for any audit claim.
- Migrate the binary to the `no-stdout-result` class: nothing on stdout; one NDJSON record per
  failure on stderr with a `kind` field; exit `0` on pass and `1` on failure. On pass, emit one
  stderr record naming the checked files and stating that audit records are not read.
- Register the binary in the ADR's binary classification table.
- Define and apply one pin-granularity rule: pin a field by its label; pin its placeholder too only
  when the placeholder itself states an admissible-value rule, with a short comment beside that pin
  saying so. Apply it to every template field pin in the crate.
- Add maintained tests: the success record lists every file the pins reference and none under
  `docs/pr-reviews/`, and the binary emits nothing on stdout.
- Add one sentence to `process-pr-review` naming this checker as workflow-document validation, not
  audit evidence.
- Update the parent EPIC row and AC4 evidence.

### Out of Scope

- Invoking the checker from pre-commit or `testing.yaml` (F56 CI part): an EPIC #2003 architecture
  decision.
- Making the checker read or validate audit records: that is the audit validator (#2278 orders 7
  and 8).
- Changing what the pins require, other than their granularity.
- Editing historical audit records that cite the checker; they stay unchanged.
- The broader self-audit evidence rules in `process-pr-review` (order 5).

## Architectural Decisions

- Related ADRs: `docs/adrs/20260519000000_define_global_cli_output_contract.md` (migration and
  classification), `docs/adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md`.
- ADRs to create: `None known`. The output class follows the existing ADR; the pin-granularity rule
  is local to this crate and is documented in its module docs.

## Design and Ownership Review

Not applicable. The binary stays a read-only file check with no child process, network, or
readiness concern; the change is output shape, documentation, and pin text.

## Bug-Fix Process

Not applicable. The checker behaves as written; the defect is in what it claims and how it is used.

## Regression Test Strategy

Not applicable as bug work. The maintained tests in T3 pin the output contract and the
checked-file list.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| -- | ------ | ---- | ----------------------- |
| T1 | TODO | Apply the pin-granularity rule | Every template field pin follows the label-first rule; each retained placeholder pin carries a comment naming its admissible-value rule; the check still exits `0` at `develop`. |
| T2 | TODO | Migrate output and document the boundary | Module docs state the checked files and the audit-record boundary; stdout is empty; failures and the pass record are NDJSON on stderr; ADR table row added. |
| T3 | TODO | Add maintained tests | Tests pin the pass record's file list (matches the pinned files, none under `docs/pr-reviews/`) and empty stdout. |
| T4 | TODO | State the boundary in `process-pr-review` | One sentence: the checker validates workflow documents and is not audit evidence. |
| T5 | TODO | Verify and record completion evidence | M1-M3, automatic checks, acceptance review, EPIC row and AC4. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T1 | Pin granularity | One signed `refactor(checks)` commit after the check passes. |
| T2 + T3 | Output migration, module docs, ADR row, and tests | One signed `feat(checks)` commit after focused tests and the test-design review. |
| T4 | Skill sentence | One signed `docs(pr-reviews)` commit. |
| T5 | Evidence and EPIC tracking | One signed `docs(issues)` commit. |

For T3, use the `write-unit-test` skill and complete the prose-first Arrange-Act-Assert review
before committing. Prove each new assertion fails against a working-tree mutant, then restore it.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2278-contract-checker-evidence-boundary/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created, linked as a sub-issue of #2278, and issue number added to this spec
- [ ] Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, crate tests, and pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-09-26 12:33 UTC - GitHub Copilot - Drafted after the maintainer chose order 6 before order 5. Local inspection at `develop` `0f1dcd28`: the crate checks 15 workflow documents (counted from the paths in `src/main.rs`) and no audit record; no hook, workflow, skill, agent, or template references it; audits for PRs #2271, #2279, and #2300 cite it as current-tree verification; it prints plain text and is unclassified in the output-contract ADR; `src/main.rs:21` and `:50` show the mixed pin granularity. Awaiting maintainer review.
- 2026-09-26 13:33 UTC - GitHub Copilot - Maintainer approved the specification as drafted, including the migration to the `no-stdout-result` output class. Created GitHub issue #2349, linked it as a sub-issue of #2278 (`parent_issue_url` verified), and moved this specification to `docs/issues/open/`. Spec-only PR pending.

## Acceptance Criteria

- [ ] AC1: The crate's module docs list the files it checks, state that it never reads `docs/pr-reviews/`, and state that a pass is not audit evidence.
- [ ] AC2: The binary writes nothing to stdout; every failure is an NDJSON record with a `kind` on stderr, exiting `1`; a pass exits `0` with one stderr record naming the checked files and the audit-record boundary.
- [ ] AC3: The binary is listed as `no-stdout-result` in the output-contract ADR table.
- [ ] AC4: Every template field pin follows one documented granularity rule, and each placeholder pin kept carries a comment naming its admissible-value rule (F57).
- [ ] AC5: `process-pr-review` states that the checker validates workflow documents and is not audit evidence.
- [ ] AC6: Maintained tests pin the pass record's file list and the empty stdout, and each fails against a mutant.
- [ ] `linter all` exits with code `0`.
- [ ] Crate tests and pre-push checks pass.
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`.
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior.

## Verification Plan

### Automatic Checks

- `cargo test --package agent-review-report-contract` and
  `cargo clippy --package agent-review-report-contract --all-targets -- -D warnings`
- `linter all`
- `TORRUST_GIT_HOOKS_LOG_DIR=.tmp ./contrib/dev-tools/git/hooks/pre-commit.sh --format=text`
- Pre-push checks before opening the implementation pull request

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| -- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1 | Pass on `develop` | `cargo run -q --package agent-review-report-contract > .tmp/contract.stdout 2> .tmp/contract.stderr; echo $?` | Exit `0`; empty stdout; one stderr record listing the checked files and no path under `docs/pr-reviews/`. | TODO | `manual-verification-evidence.md` section V1 |
| M2 | Failure on a broken pin | Temporarily delete a pinned line from `PR-REVIEW-TEMPLATE.md` in the working tree, run M1's command, then restore the file. | Exit `1`; empty stdout; one NDJSON failure record naming the file and missing text. | TODO | `manual-verification-evidence.md` section V2 |
| M3 | Placeholder rename under the rule | Temporarily rename the placeholder of a label-only pin in a template, run the check, then restore it. | The check still passes, because the pin is the label. | TODO | `manual-verification-evidence.md` section V3 |

Record the Rust toolchain used for every command result. No disposable verification script is
planned.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | TODO | Module docs diff |
| AC2 | TODO | Tests; M1 and M2 |
| AC3 | TODO | ADR diff |
| AC4 | TODO | Pin diff; M3 |
| AC5 | TODO | Skill diff |
| AC6 | TODO | Tests and mutation evidence |

## Risks and Trade-offs

- A label-only pin can miss a placeholder that silently changes meaning. Mitigation: placeholders
  that carry an admissible-value rule stay pinned, with a comment saying why.
- The `process-pr-review` sentence overlaps files that PR #2344 and the #2347 fixes also edit.
  Mitigation: it is one sentence in its own commit, rebased onto the latest `develop` before the PR.
- The pass record makes the success path chattier on stderr. Mitigation: it is one NDJSON line,
  within the output contract.

## Implementation Completion Review

- Retrospective: `Not yet assessed`.
- Create `implementation-retrospective.md` if the pin-granularity rule needs exceptions beyond the
  admissible-value case; otherwise record why none was needed.
- When an independent reviewer receives this folder-style specification, record the result in
  `agent-review-reports.md` using `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- Parent EPIC: #2278; grandparent EPIC: #2003.
- Decision input: `retrospective-improvement-matrix.md` (F56, F57, PR #2271 row).
- Output contract: `docs/adrs/20260519000000_define_global_cli_output_contract.md`.
- Audits citing the checker: `docs/pr-reviews/pr-2271-review/`, `pr-2279-review/`, `pr-2300-review/`.
