---
schema-version: 1
doc-type: issue
issue-type: task
status: draft
priority: p2
epic: 2278
github-issue: null
spec-path: docs/issues/drafts/2278-author-self-audit-gate/ISSUE.md
branch: "{issue-number}-2278-author-self-audit-gate"
related-pr: null
last-updated-utc: "2026-10-03 17:03"
semantic-links:
  skill-links:
    - create-issue
    - process-pr-review
  related-artifacts:
    - "issue #2278"
    - "issue #2003"
    - docs/issues/open/2278-2003-strengthen-pr-review-author-self-audit/EPIC.md
    - docs/issues/open/2278-2003-strengthen-pr-review-author-self-audit/retrospective-improvement-matrix.md
    - docs/pr-reviews/pr-2270-review/review-retrospective.md
    - docs/pr-reviews/pr-2271-review/review-retrospective.md
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/templates/PR-REVIEW-TEMPLATE.md
    - .github/skills/dev/planning/create-issue/SKILL.md
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Add the Author Self-Audit Gate to `process-pr-review`

**Parent EPIC:** #2278 - Strengthen PR Review Author Self-Audit and Evidence Generation

Subissue 5 of #2278: the manual author self-audit gate that the EPIC's matrix adopts from the PR #2270 and PR #2271 retrospectives, with F65 and the manual part of F55.

## Goal

Make `process-pr-review` require a recorded self-audit before each audit commit, reply, thread resolution, and re-review request, re-deriving each gated claim from Git and GitHub evidence so intent is never recorded as current-tree fact.

## Background

The EPIC (`docs/issues/open/2278-2003-strengthen-pr-review-author-self-audit/EPIC.md`) asks for this gate with its triggers and evidence-first ordering (lines 106-108), keeps it usable with direct Git and GitHub evidence when automation is unavailable (lines 137-138), and closes Phase 2 when a processed pull request records the self-audit before each reply (lines 201-203).

The PR #2270 retrospective counts about a third of its findings as re-raises, traces them to bookkeeping from memory rather than from the bytes, and asks for the convergence, re-derivation, and compaction rules (`docs/pr-reviews/pr-2270-review/review-retrospective.md`, lines 56-57, 115, and 166-173). The PR #2271 retrospective asks for a recorded self-audit pass before each re-review request and judges that the manual pass alone would have prevented rounds four through seven (`docs/pr-reviews/pr-2271-review/review-retrospective.md`, lines 119-122, 130-131, and 145-146).

At `develop`, the skill (`.github/skills/dev/pr-reviews/process-pr-review/SKILL.md`) runs the validator before every audit commit, reply, and resolution (lines 205-207) and asks for progressive updates (lines 101-102), but has no rule for a validator that cannot run, for what to re-derive and when, or for recording a self-audit, and no checklist item for progressive updates (lines 280-297). The template (`docs/templates/PR-REVIEW-TEMPLATE.md`) states no evidence-first or stamp-source rule at its verification placeholder (line 95) or its Processing Log guidance (lines 110-111).

## Scope

### In Scope

Matrix line numbers refer to `docs/issues/open/2278-2003-strengthen-pr-review-author-self-audit/retrospective-improvement-matrix.md`.

| Matrix | Rule this issue adds | Where |
| ------ | -------------------- | ----- |
| 35 | Run `validate-audit-record.py` when it can; otherwise do its checks by hand and record the commands. No helper is a prerequisite. | Skill gate; validator section |
| 39 | On a finding's second re-raise, stop editing and re-derive every field of every affected row from source before the next reply. | Skill gate |
| 40 | A corrected value is re-derived from a named Git or GitHub command recorded in `Current-tree verification`. | Skill gate; template |
| 41 | After context compaction, pass the gate over every row before writing anything. | Skill gate |
| 42 | For each row the event touches: re-run its verification; re-raise targets are findings of this audit; row/detail parity and order hold. | Skill gate |
| 43 | Each cited commit carries one repository fix; audit updates are in their own commits. | Skill gate |
| 44 | `Current-tree verification` is written from the command output, command and result first. | Skill step 5 and gate; template |
| 45 | Confirm each cited commit is on the branch and contains the change (`git show --stat <commit> -- <path>`). | Skill gate |
| 46 | Audit prose states an equality or a scoped list, never a self-referential count or a universal claim. | Skill gate; template |
| 47 | Stamp log entries from `git log --format=%cI` or GitHub `created_at`, not recollection, and keep log order. | Skill gate; template |
| 82 | F65: each progressive update passes the gate before its audit commit. | Skill step 9; checklist |
| 119 | F55, manual part: the audit record exists at its canonical path before the first reply. | Skill gate; checklist |

- The gate is a new skill section naming its four gated events (audit commit, reply, thread resolution, re-review request); new sentences in steps 5, 7, and 9 and the validator section point to it.
- Each gate pass is one append-only Processing Log entry naming the gated event, the rows checked, and the commands run with their outcome. No per-finding field is added.
- Template guidance outside the copied sections: rows 40, 44, and 46 beside `## Finding Details`; the gate entry and row 47 beside `## Processing Log`.
- Bump the skill version, record verification, and update the EPIC row.

### Out of Scope

- Automating any gate check or changing the validator (orders 7 to 9).
- Adding, removing, or renaming audit fields or template placeholders.
- Rewriting the sentences #2362 changes (see Dependencies and Open Questions).
- A CI check that an audit exists (F55 CI part) and `agent-review-report-contract` changes (#2349).
- Proportionate evidence (order 10), reviewer-side `review-pr` changes, and historical audit records.

## Architectural Decisions

- Related ADRs: `docs/adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md`
- ADRs to create: `None known`. The gate extends the existing review-workflow contract.

## Design and Ownership Review

Not applicable. Documentation rules only; no process, I/O, readiness, cleanup, or fixture concern.

## Bug-Fix Process

Not applicable. The task adds a missing workflow control, not a fix for broken runtime behavior.

## Regression Test Strategy

Not applicable. Documentation-only; M1 to M3 exercise the gate.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| -- | ------ | ---- | ----------------------- |
| T1 | TODO | Add the gate to the skill | Gate section and pointer sentences per the Scope table; skill version bumped. |
| T2 | TODO | Add the template guidance | Finding Details and Processing Log guidance; no copied section or placeholder changed. |
| T3 | TODO | Add the checklist counterparts | Completion Checklist items for F65 and the manual part of F55. |
| T4 | TODO | Evidence and tracking | V1-V3 evidence, acceptance review, EPIC row. |

## Commit Points

Map implementation-plan tasks that change code, tests, configuration, or documentation to small,
coherent commit opportunities. A commit point completes one independently reviewable behavior,
refactor, or evidence increment; do not group unrelated changes merely to reduce commit count.

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T1 | Skill gate section, step pointers, validator rule, and version | One `docs(pr-reviews)` commit after focused manual verification. |
| T2 | Template Finding Details and Processing Log guidance | One `docs(pr-reviews)` commit after focused manual verification. |
| T3 | Completion Checklist items | One `docs(pr-reviews)` commit after focused manual verification. |
| T4 | Evidence and EPIC tracking | One `docs(issues)` commit after the implementation commits. |

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
with the narrow affected scope.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2278-author-self-audit-gate/ISSUE.md`
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

Append one line per meaningful update.

- 2026-10-03 17:03 UTC - da2ce7 - Drafted from the #2278 matrix rows owned by the self-audit gate (lines 35, 39-47, 82, and 119) against `develop` at `Merge torrust/torrust-tracker#2419: ci(workflows): [#2402] cancel superseded PR runs`; awaiting maintainer review.

## Acceptance Criteria

- [ ] AC1: The skill has one gate section naming its four gated events, and steps 5, 7, and 9 point to it.
- [ ] AC2: The gate states the Scope table checks for rows 42 to 47 and 119, including `git show --stat <commit> -- <path>` and the stamp sources `git log --format=%cI` and GitHub `created_at`.
- [ ] AC3: The gate runs `validate-audit-record.py` when it can, states the manual checks and command recording when it cannot, and names no helper as a prerequisite.
- [ ] AC4: The skill states both re-derivation triggers: a finding's second re-raise, and context compaction.
- [ ] AC5: The skill and the template put the command and its result before narrative in `Current-tree verification` and re-derive a corrected value from a named Git or GitHub command recorded there.
- [ ] AC6: The skill and the template's Processing Log guidance state the one-entry-per-gate-pass record; the roster and template skeleton are unchanged.
- [ ] AC7: The Completion Checklist has the F65 item and the F55 manual item.
- [ ] AC8: In one review round processed under the revised skill, a gate entry is stamped before each reply, resolution, audit commit, and re-review request.
- [ ] AC9: `cargo run --package agent-review-report-contract` passes, and `validate-audit-record.py` passes for the three most recent audits and this issue's implementation PR.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

Define verification before implementation starts and execute it before closing the issue.

### Automatic Checks

- `linter all`
- `TORRUST_GIT_HOOKS_LOG_DIR=.tmp ./contrib/dev-tools/git/hooks/pre-commit.sh --format=text`
- `cargo run --package agent-review-report-contract` (stable Rust toolchain)
- `python3 .github/skills/dev/pr-reviews/process-pr-review/scripts/validate-audit-record.py --pr-number <N>` for the three most recent audits
- Pre-push checks before opening the implementation PR

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| -- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1 | One review round under the gate | Process the first review round of this issue's implementation PR with the revised skill; record the thread data and reply `created_at` values. | A gate entry naming the event and commands, stamped from Git or GitHub, precedes each gated event. | TODO | `manual-verification-evidence.md` section V1 |
| M2 | Gate without the validator | During M1, pass the gate once for one row by hand. | The entry lists the manual commands and their outcome; the row matches the current tree. | TODO | `manual-verification-evidence.md` section V2 |
| M3 | One rule per matrix row | Find each Scope table rule where the table places it with a recorded `rg -U` pattern. | One rule per row; no copied template section changed. | TODO | `manual-verification-evidence.md` section V3 |

Record the scenarios from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`. Without a review round on the implementation PR, M1 and M2 stay `BLOCKED`, never simulated. No disposable verification script is planned.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | TODO | M3 |
| AC2 | TODO | M3 |
| AC3 | TODO | M2; M3 |
| AC4 | TODO | M3 |
| AC5 | TODO | M1; M3 |
| AC6 | TODO | M1; M3 |
| AC7 | TODO | M3 |
| AC8 | TODO | M1 |
| AC9 | TODO | Automatic checks |

## Risks and Trade-offs

- Gate cost on small reviews. Mitigation: a pass checks only the rows its event touches.
- A gate entry recording intent as fact. Mitigation: it names the commands and their outcome; M1 checks each stamp against its event.
- Concurrent edits by #2362 and #2349. Mitigation: insert-only sentences, one commit per task, rebased before the PR.

## Dependencies and Open Questions

- Depends on #2295 and #2308 (done); coordinates with #2349 (a skill sentence) and order 7 (its port replaces the gate's validator command).
- Issue #2362's pending proposals rewrite skill lines 87-92, 94-96, 197-199, and 289-291 and template lines 40-43, 96, 100, and 119-129; this issue rewrites none. Which lands first? The second rebases and takes the next skill version.
- Should the copied `## Completion Rules` carry a gate bullet? This draft says no: every new record would copy it and order 8 would byte-diff it.
- May one gate pass precede several replies posted together if it covers every replied row? This draft assumes yes.
- F75 (`RE_RAISE_OF` does not say whose finding ID it takes) is a T2 wording row that the register places with order 8; the gate only checks that each target is a finding of this audit. Should F75's wording land here?
- The EPIC's order-10 row lists "F55 as a self-audit step" (`docs/issues/open/2278-2003-strengthen-pr-review-author-self-audit/EPIC.md:160`); matrix line 119 places it in T2. Its own T3 commit lets it move unchanged.

## Implementation Completion Review

After implementation, compare the result with this specification. Record invalidated assumptions, material design changes, unexpected validation findings, and reusable lessons.

- Retrospective: `Not yet assessed`
- Create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` here if M1 shows a check that cannot be done by hand or an unanticipated cost.
- If no retrospective is needed, add a concise progress-log entry explaining why the work had no material discovery.
- When an independent reviewer receives this folder-style specification, it records its result in `agent-review-reports.md` using `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- Parent EPIC: #2278; grandparent EPIC: #2003. Siblings: #2308, #2349, #2362.
