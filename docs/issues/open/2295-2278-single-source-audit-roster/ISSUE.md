---
schema-version: 1
doc-type: issue
issue-type: task
status: open
priority: p2
epic: 2278
github-issue: 2295
spec-path: docs/issues/open/2295-2278-single-source-audit-roster/ISSUE.md
branch: "2295-2278-single-source-audit-roster-spec"
related-pr: null
last-updated-utc: "2026-09-22 10:55"
semantic-links:
  skill-links:
    - create-issue
    - process-pr-review
  related-artifacts:
    - "issue #2278"
    - "issue #2003"
    - docs/issues/open/2278-2003-strengthen-pr-review-author-self-audit/EPIC.md
    - docs/issues/open/2278-2003-strengthen-pr-review-author-self-audit/retrospective-improvement-matrix.md
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/templates/PR-REVIEW-TEMPLATE.md
    - contrib/dev-tools/checks/agent-review-report-contract/src/main.rs
    - .github/skills/dev/planning/create-issue/SKILL.md
---

<!-- skill-link: create-issue -->

# Issue #2295 - Single-Source the Audit Field Roster and Mark Copied Template Sections

**Parent EPIC:** #2278 - Strengthen PR Review Author Self-Audit and Evidence Generation

Subissue 1 of #2278. Register items F58, F77, and F63 from the #2003 friction register, as
dispositioned in the parent EPIC's
[improvement matrix](../../open/2278-2003-strengthen-pr-review-author-self-audit/retrospective-improvement-matrix.md).

## Goal

Make the PR-review audit field roster exist once, one field per line, in the `process-pr-review`
skill, with the `PR-REVIEW-TEMPLATE.md` skeleton matching it field for field; and mark in the
template which sections an audit record copies verbatim and which are guidance that the record
omits.

## Background

Two normative documents describe the audit record: the skill section `Required Audit Fields` in
`.github/skills/dev/pr-reviews/process-pr-review/SKILL.md` and the skeleton in
`docs/templates/PR-REVIEW-TEMPLATE.md`. They disagree today:

- The skill roster is one prose sentence naming 17 fields (F77). The template skeleton carries 19:
  eight tracking-row columns, the `Summary` in the detail heading, and ten detail-entry lines. The
  skill omits `Concern` and `Solution`,
  which every detail entry must contain (F58). An author who checks a record against the skill
  passes; one who checks against the template fails.
- Nothing in the template says which of its sections a record reproduces verbatim (`Ownership`,
  `Status Values`, `Completion Rules`) and which are drafting guidance omitted from the record (the
  prose under `Findings` and `Finding Details`) (F63). The PR #2272 retrospective proposed a
  template-drift check; it cannot be specified until the copied set is explicit.

Subissue 7 of the parent EPIC will make the audit validator enforce the roster and byte-diff the
copied sections. It can only enforce a roster that exists once and a copied set that is named. That
is why this issue comes first in the EPIC order and unblocks subissues 2, 4, and 7.

The `agent-review-report-contract` check crate pins the current roster sentence and the current
"one compact tracking row" sentence as literal text. Restating the roster changes those lines, so
the pins must move with them in the same change; otherwise the check reports a false drift.

## Scope

### In Scope

- Restate `Required Audit Fields` in the skill as a list, one field per line, grouped by where the
  field lives (tracking row, detail-entry heading, detail-entry lines), in skeleton order, and
  including `Concern` and `Solution`.
- State in the skill that this list is the single source of the roster and that the template
  skeleton must match it exactly.
- In `PR-REVIEW-TEMPLATE.md`, replace any restatement of the roster with a reference to the skill
  section, and confirm the skeleton field set and order equal the roster.
- Precede every top-level section of `PR-REVIEW-TEMPLATE.md` with a plain HTML comment stating
  either that the section is copied verbatim into each audit record or that it is guidance omitted
  from the record. Plain prose comments only, local to this template.
- Update the `agent-review-report-contract` pinned literals that the restated sentences invalidate
  so the check keeps pointing at the roster's new single-source lines.
- Update the parent EPIC's `Subissues` row and AC2 evidence for this issue.

### Out of Scope

- Reconciling the other skill-versus-template rule contradictions (F60, F61, F62, F73, F76, F79,
  F80, F66): subissue 2 of #2278.
- Adding or removing audit fields. The roster is restated, not changed; the field set is the one
  the template skeleton already carries.
- Any validator change, including enforcing the roster or diffing copied sections: subissues 6 and
  7 of #2278.
- Changing what `agent-review-report-contract` reads or how it reports: subissue 5 of #2278. Only
  the literal pins that this issue's text edits break are touched here.
- A typed or repository-wide "copied section" marker syntax: EPIC #2264 owns the marker catalog.
- Editing any existing audit record under `docs/pr-reviews/`.

## Architectural Decisions

- Related ADRs: `docs/adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md`
- ADRs to create: `None known`. The skill is already declared the exclusive definer of audit fields
  by `.github/prompts/process-copilot-suggestions.prompt.md`; this issue applies that decision
  rather than making a new one.

## Design and Ownership Review

Not applicable.

## Bug-Fix Process

Not applicable.

## Regression Test Strategy

Not applicable.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| -- | ------ | ---- | ----------------------- |
| T1 | TODO | Restate the roster in the skill | `Required Audit Fields` lists 19 fields one per line, grouped as tracking row (8), detail heading (`Summary`), detail lines (10), in skeleton order; states it is the single source and that the skeleton must match. The existing narrative rules in that section (allowed values, reference immutability, resolution-reference forms) are kept below the list unchanged in meaning. |
| T2 | TODO | Move the contract-check pins | Every `REQUIRED_TEXT` and `WRAPPED_TEXT` entry in `agent-review-report-contract` that named a sentence T1 rewrote now names the replacement line; `cargo run --package agent-review-report-contract` exits `0`. |
| T3 | TODO | Align the template with the roster | Template text that lists fields refers to the skill section instead; skeleton field names and order equal the T1 list, verified by M1. |
| T4 | TODO | Mark copied and guidance sections | Each `##` section of the template is preceded by a plain HTML comment naming it copied-verbatim or guidance-omitted; the copied set equals the section set of the most recent audit records (M3). |
| T5 | TODO | Record evidence and update the EPIC | `manual-verification-evidence.md` holds M1-M3 output; the #2278 `Subissues` row is `DONE` with the merged PR; AC2 evidence in the EPIC names this issue. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T1 + T2 | Skill roster restated and the contract-check literals that describe it moved together | One commit: the pins are assertions about the skill text and must not be false at any commit. |
| T3 + T4 | Template references the roster and marks its sections | One commit after M1 and M3 pass. |
| T5 | Evidence file and EPIC row update | One `docs(issues)` commit after the implementation commits. |

Record a justified no-change decision in the task's evidence without creating an empty commit. Use
a Conventional Commit message with the narrow affected scope, and sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2278-single-source-audit-roster/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created, linked as a sub-issue of #2278, and issue number added to this spec
- [ ] Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, `cargo run --package agent-review-report-contract`, pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-09-22 09:26 UTC - GitHub Copilot - Drafted from matrix items F58, F77, and F63 after confirming the skill roster names 17 fields against the template's 19 and that `agent-review-report-contract` pins the sentence to be rewritten; awaiting maintainer review.
- 2026-09-22 09:40 UTC - GitHub Copilot - Maintainer approved the specification; created GitHub sub-issue #2295 under EPIC #2278 and moved this specification to `docs/issues/open/2295-2278-single-source-audit-roster/`.

## Acceptance Criteria

- [ ] AC1: `Required Audit Fields` in the `process-pr-review` skill lists every audit field exactly once, one field per line, including `Concern` and `Solution`, grouped by tracking row, detail-entry heading, and detail-entry lines, in the order the template skeleton uses.
- [ ] AC2: The skill states that this list is the single source of the roster; `PR-REVIEW-TEMPLATE.md` refers to it and no other repository document restates the field list.
- [ ] AC3: The template skeleton's field names and order equal the roster (M1).
- [ ] AC4: Every top-level section of `PR-REVIEW-TEMPLATE.md` is preceded by a plain HTML comment stating whether it is copied verbatim into an audit record or is guidance omitted from the record, and the copied set matches the sections present in the most recent audit records (M3).
- [ ] AC5: `agent-review-report-contract` exits `0` and none of its pins names text that no longer exists (M2).
- [ ] AC6: No file under `docs/pr-reviews/` changes.
- [ ] AC7: `linter all` exits with code `0`.
- [ ] AC8: Manual verification scenarios M1-M4 are executed and documented in issue-local `manual-verification-evidence.md`.
- [ ] AC9: Acceptance criteria are re-reviewed after implementation and reflect actual behavior.

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo run --package agent-review-report-contract` (stable Rust toolchain)
- `TORRUST_GIT_HOOKS_LOG_DIR=.tmp ./contrib/dev-tools/git/hooks/pre-commit.sh`
- Pre-push checks before opening the implementation PR

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| -- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1 | Roster equals skeleton | Extract the field names from the skill list and from the template's tracking-row header plus detail-entry lines (`grep`/`sed`, exact commands recorded), then `diff` the two sequences. | Empty diff: same 19 names, same order. | TODO | `manual-verification-evidence.md` section V1 |
| M2 | Contract pins are live | `cargo run --package agent-review-report-contract`; then, for each pin changed in T2, `grep -nF '<pin text>'` in its target file. | Exit `0`; every changed pin matches exactly one line. | TODO | `manual-verification-evidence.md` section V2 |
| M3 | Copied set matches real audits | List the `##` sections the template marks as copied; compare with `grep '^## ' docs/pr-reviews/pr-2288-review/PR-REVIEW.md` and one other recent audit. | Identical section lists; the guidance-marked prose does not appear in the audits. | TODO | `manual-verification-evidence.md` section V3 |
| M4 | Historical records untouched | `git diff --stat develop...HEAD -- docs/pr-reviews/` | No output. | TODO | `manual-verification-evidence.md` section V4 |

Notes:

- Manual verification is mandatory even when automated checks pass. Record actual commands,
  output, and the toolchain that produced them in `manual-verification-evidence.md`, created from
  `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`.
- If a scenario fails, record the failure and diagnosis in the progress log before proceeding.

### Disposable Verification Scripts

None planned. M1 is a one-line `diff` of two `grep` extractions; recording the commands in the
evidence file is sufficient. Subissue 7 of #2278 is the owner of any maintained check.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | TODO | Skill diff in the implementation PR |
| AC2 | TODO | `grep -rn` for the old roster sentence across `.github/` and `docs/` returns nothing outside `docs/issues/` and `docs/pr-reviews/` |
| AC3 | TODO | M1 |
| AC4 | TODO | M3 |
| AC5 | TODO | M2 |
| AC6 | TODO | M4 |
| AC7 | TODO | `linter all` output recorded in `manual-verification-evidence.md` |
| AC8 | TODO | `manual-verification-evidence.md` sections V1-V4 |
| AC9 | TODO | Post-implementation review entry in the progress log |

## Risks and Trade-offs

- The contract check is literal-pin based; a wording tweak during review can silently break a pin
  because nothing runs the check automatically. Mitigation: M2 is repeated after every review
  round, and T1 and T2 are committed together.
- Marking sections with prose HTML comments is not machine-parseable. Accepted: the matrix
  explicitly defers any typed marker to EPIC #2264; subissue 7 can key on the comment text or the
  section headings when it automates the diff.
- Restating the roster invites "while here" edits to the rules around it. Mitigation: the rule
  contradictions have their own subissue (2); this PR changes the list shape and section markers
  only, and the reviewer checks the skill diff for semantic change.

## Implementation Completion Review

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from
  `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this issue specification's directory.
- If no retrospective is needed, add a concise progress-log entry explaining why the work had no
  material discovery.
- When an independent reviewer receives this folder-style specification, it records its result in
  `agent-review-reports.md` using `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- Parent EPIC: #2278; grandparent EPIC: #2003.
- Decision input: `docs/issues/open/2278-2003-strengthen-pr-review-author-self-audit/retrospective-improvement-matrix.md` (rows F58, F77, F63).
- Register source: <https://github.com/torrust/torrust-tracker/issues/2003#issuecomment-5767266486>.
- Unblocks: #2278 subissues 2, 4, and 7.
- Related ADR: `docs/adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md`.
