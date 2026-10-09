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
last-updated-utc: "2026-10-09 11:24"
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

The EPIC (`docs/issues/open/2278-2003-strengthen-pr-review-author-self-audit/EPIC.md`) asks for this gate with its triggers and evidence-first ordering (In Scope: "Add an explicit author self-audit gate"), keeps it usable without automation (Out of Scope: "Making an audit helper a prerequisite for the manual self-audit gate"), and closes Phase 2 when a processed pull request records the self-audit before each reply (Delivery Strategy, "Phase 2: Author gate and honest checkers", Exit criteria).

The PR #2270 retrospective counts about a third of its findings as re-raises, traces them to bookkeeping from memory rather than from the bytes, and asks for these rules (`docs/pr-reviews/pr-2270-review/review-retrospective.md`, lines 56-57, 115, and 166-173). The PR #2271 retrospective asks for a recorded self-audit pass before each re-review request and judges it would alone have prevented rounds four through seven (`docs/pr-reviews/pr-2271-review/review-retrospective.md`, lines 119-122, 130-131, and 145-146).

At `develop`, the skill (`.github/skills/dev/pr-reviews/process-pr-review/SKILL.md`, version 1.5 at `686a42f45`) runs the validator "before every audit commit" and before any reply or resolution (Validation Script) and asks to "Update the audit progressively" (Workflow step 9, Verify completion), but has no rule for a validator that cannot run, for what to re-derive and when, or for recording a self-audit, and its Completion Checklist has no progressive-update item. The template (`docs/templates/PR-REVIEW-TEMPLATE.md`) has no evidence-first or stamp-source rule: neither the `Current-tree verification` placeholder under Finding Details nor the Processing Log guidance ("Append entries only.") states one.

That run cannot pass before the first reply: the validator (`.github/skills/dev/pr-reviews/process-pr-review/scripts/validate-audit-record.py`) requires one posted reply on each discussion-anchored row's own thread ("expected exactly one discussion Reply URL"), and the skill's Validation Script section says to "treat a non-zero exit" as blocking. Hence a pre-posting pass and a full pass.

## Scope

### In Scope

Matrix rows are named by F-ID, or by source PR and a short quote of the proposal, in `docs/issues/open/2278-2003-strengthen-pr-review-author-self-audit/retrospective-improvement-matrix.md`; the PR rows are under Author Verification and Convergence.

| Matrix row | Rule this issue adds | Where |
| ---------- | -------------------- | ----- |
| PR #2270, "Run the audit validator before every reply and audit commit" | Before each reply, a pre-posting pass runs every check that needs no posted reply (the validator with `--audit-file` on the uncommitted record, or by hand with recorded commands), naming the pending reply-URL check as its only exception. Before the audit commit, each resolution, and completion, a full pass has none. No helper is a prerequisite. | Skill gate; validator section |
| PR #2270, "Stop and re-derive affected fields" | On a finding's second re-raise, stop and re-derive every field of every affected row from source before the next reply. | Skill gate |
| PR #2270, "Require corrections to re-derive claims" | A corrected value is re-derived from a named Git or GitHub command recorded in `Current-tree verification`. | Skill gate; template |
| PR #2270, "Revalidate after context compaction" | After context compaction, pass the gate over every row before writing anything. | Skill gate |
| PR #2271, "Self-audit every row before requesting re-review" | For each row the event touches: re-run its verification; re-raise targets are findings of this audit; row/detail parity and order hold. | Skill gate |
| PR #2271, "Keep one repository fix per commit" | Each cited commit carries one repository fix; audit updates are in their own commits. | Skill gate |
| PR #2271, "Write verification command and result before narrative" | `Current-tree verification` is written from the command output, command and result first. | Skill step 5 and gate; template |
| PR #2271, "proves it contains the described change" | Confirm each cited commit is on the branch and contains the change (`git show --stat <commit> -- <path>`). | Skill gate |
| PR #2271, "Avoid self-referential counts" | Audit prose states an equality or a scoped list, never a self-referential count or a universal claim. | Skill gate; template |
| PR #2271, "Derive log events from Git and GitHub timestamps" | Stamp log entries from `git log --format=%cI` or GitHub `created_at`, not recollection, and keep log order. | Skill gate; template |
| F65 | Each progressive update passes the gate before its audit commit. | Skill step 9; checklist |
| F55, manual part | The audit record exists at its canonical path before the first reply and is committed, with each posted reply URL, before the first resolution. | Skill gate; checklist |

- The gate is a new skill section with four gated events: the pre-posting pass gates each reply; the full pass gates the audit commit, thread resolution, and re-review request; the record is committed after its replies. New sentences in steps 5, 7, and 9 and the validator section point to it.
- Each gate pass is one append-only Processing Log entry naming the event, rows checked, and commands run with their outcome; no field is added.
- Template guidance outside the copied sections: the "Require corrections to re-derive claims", "Write verification command and result before narrative", and "Avoid self-referential counts" rules beside `## Finding Details`; the gate entry and the "Derive log events from Git and GitHub timestamps" rule beside `## Processing Log`.
- Bump the skill version from the one on `develop` when this lands (1.5 at `686a42f45`, or the version #2362 leaves if it lands first), record verification, and update the EPIC row.

### Out of Scope

- Automating gate checks or changing the validator (orders 7 to 9).
- Changing audit fields or template placeholders.
- Rewriting the sentences #2362 changes (see Dependencies).
- A CI check that an audit exists (F55 CI part); `agent-review-report-contract` changes (#2349).
- Proportionate evidence (order 10), `review-pr`, and historical audit records.

## Architectural Decisions

- Related ADRs: `docs/adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md`
- ADRs to create: `None known`. The gate extends the existing review-workflow contract.

## Design and Ownership Review

Not applicable. Documentation rules only; no process, I/O, readiness, cleanup, or fixture concern.

## Bug-Fix Process

Not applicable. A missing workflow control, not broken runtime behavior.

## Regression Test Strategy

Not applicable. Documentation-only; M1 to M3 exercise the gate.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| -- | ------ | ---- | ----------------------- |
| T1 | TODO | Add the gate to the skill | Gate section and pointer sentences per the Scope table; skill version bumped from 1.5, or from the version #2362 leaves. |
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

- 2026-10-03 17:03 UTC - da2ce7 - Drafted from the #2278 matrix rows owned by the self-audit gate (the PR #2270 and PR #2271 rows of Author Verification and Convergence owned by T2, F65, and F55) against `develop` at `Merge torrust/torrust-tracker#2419: ci(workflows): [#2402] cancel superseded PR runs`; awaiting maintainer review.
- 2026-10-03 17:38 UTC - da2ce7 - PR #2431 round 1 (`review-finding:pr-2431-f1`): split the gate into a pre-posting pass and a full pass, with the record committed after its replies; the Scope rows for "Run the audit validator before every reply and audit commit" and F55, AC3, AC8, M1, and M2 updated.
- 2026-10-09 11:24 UTC - da2ce7 - PR #2431 round 2 (review 5469277375, F3 and F4), after the rebase onto `develop` `686a42f45`: the skill (version 1.5), template, and validator citations are re-derived there and given as section headings or quotes; matrix rows are named by F-ID or by source PR and quote, and EPIC passages by section heading or quote, in Background, Scope, AC2, the Verification Plan, Dependencies, and the two entries above, which named matrix rows by their line numbers at their base.

## Acceptance Criteria

- [ ] AC1: The skill has one gate section naming its four gated events, and steps 5, 7, and 9 point to it.
- [ ] AC2: The gate states, with their commands, the Scope table checks for the PR #2271 rows and F55.
- [ ] AC3: The gate defines the pre-posting pass (the pending reply-URL check its only named exception) and the full pass (no exception), each by `validate-audit-record.py` or by hand with recorded commands; no helper is a prerequisite.
- [ ] AC4: The skill states both re-derivation triggers: a finding's second re-raise, and context compaction.
- [ ] AC5: The skill and the template put command and result before narrative in `Current-tree verification` and re-derive corrections from a named command recorded there.
- [ ] AC6: The skill and the template's Processing Log guidance state the one-entry-per-gate-pass record; the roster and template skeleton are unchanged.
- [ ] AC7: The Completion Checklist has the F65 item and the F55 manual item.
- [ ] AC8: In one round processed under the revised skill, a pre-posting entry precedes each reply, the record is committed after the replies with each reply URL on its own thread, and a full-pass entry precedes the audit commit, each resolution, and the re-review request.
- [ ] AC9: `cargo run --package agent-review-report-contract` passes; `validate-audit-record.py` passes for this issue's PR and, each with its recorded `--base`, the three most recent audits; no historical record is edited.
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
- `python3 .github/skills/dev/pr-reviews/process-pr-review/scripts/validate-audit-record.py --pr-number <N> --base <B>` for the three most recent audits, `<B>` being each audit PR's recorded merge base (`git merge-base <M>^1 <M>^2` for its merge commit `<M>`); the `develop` default excludes merged fixes (the validator's `default="develop"` for `--base` and its `f"{base}..HEAD"` subject range), and no record is edited to fit it.
- Pre-push checks before opening the implementation PR

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| -- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1 | One review round under the gate | Process the first review round of this issue's implementation PR with the revised skill; record thread data and reply `created_at` values. | Each reply follows a pre-posting entry excepting only its pending reply URL; the audit commit follows the replies with 0 validator failures; no resolution precedes it. | TODO | `manual-verification-evidence.md` section V1 |
| M2 | Gate without the validator | During M1, pass the pre-posting gate once for one row by hand. | The entry lists the manual commands and outcome; the row matches the tree. | TODO | `manual-verification-evidence.md` section V2 |
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
- The pre-posting exception hiding a reply defect. Mitigation: it is named, and the full pass checks every posted URL before any resolution.
- Concurrent edits by #2362 and #2349. Mitigation: insert-only sentences, rebased.

## Dependencies and Open Questions

- Depends on #2295 and #2308 (done); coordinates with #2349 (a skill sentence) and order 7 (its port replaces the validator command).
- Issue #2362's pending proposals rewrite skill lines 87-92, 94-96, 197-199, and 289-291 and template lines 40-43, 96, 100, and 119-129; this issue rewrites none. Which lands first? The second rebases and takes the next skill version.
- Should the copied `## Completion Rules` carry a gate bullet? This draft says no: every new record would copy it and order 8 would byte-diff it.
- May one pre-posting pass cover several replies posted together? This draft assumes yes.
- F75 (whose finding ID `RE_RAISE_OF` takes) is a T2 wording row that the register places with order 8; should its wording land here?
- The EPIC's order-10 row in its Subissues table lists "F55 as a self-audit step"; the matrix's F55 row places the manual step in T2. Its own T3 commit lets it move.

## Implementation Completion Review

After implementation, compare the result with this specification. Record invalidated assumptions, material design changes, unexpected validation findings, and reusable lessons.

- Retrospective: `Not yet assessed`
- Create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` if M1 shows a check that cannot be done by hand.
- If no retrospective is needed, add a concise progress-log entry explaining why the work had no material discovery.
- When an independent reviewer receives this folder-style specification, it records its result in `agent-review-reports.md` using `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- Parent EPIC: #2278; grandparent EPIC: #2003. Siblings: #2308, #2349, #2362.
