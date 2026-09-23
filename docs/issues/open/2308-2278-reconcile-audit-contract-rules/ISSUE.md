---
schema-version: 1
doc-type: issue
issue-type: task
status: in-review
priority: p2
epic: 2278
github-issue: 2308
spec-path: docs/issues/open/2308-2278-reconcile-audit-contract-rules/ISSUE.md
branch: "2308-2278-reconcile-audit-contract-rules"
related-pr: https://github.com/torrust/torrust-tracker/pull/2313
last-updated-utc: "2026-09-23 09:52"
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
    - .github/skills/dev/planning/create-issue/SKILL.md
---

<!-- skill-link: create-issue -->

# Issue #2308 - Reconcile Audit Skill and Template Rule Contradictions

**Parent EPIC:** #2278 - Strengthen PR Review Author Self-Audit and Evidence Generation

Subissue 2 of #2278. It implements the adopted F60, F61, F62, F66, F73, F76, F79, and F80
contract rules from the #2003 friction register after subissue 1 established the canonical roster.

## Goal

Make `process-pr-review` and `PR-REVIEW-TEMPLATE.md` state one unambiguous audit contract for
outdated threads, append-only corrections, re-raises, resolution references, consolidated replies,
review-body findings, and suppressed Copilot comments.

## Background

The parent EPIC's approved improvement matrix identifies eight places where an author can follow
one normative review-workflow document and be contradicted by the other, or where a material case
has no stated rule. These gaps caused review rounds over process claims rather than product
behavior.

The resolution must preserve the 19-field roster established by #2295. This task reconciles the
meaning of existing fields and workflow steps; it does not add fields, change the audit validator,
or change `fetch-review-threads`, which subissue 3 owns.

## Scope

### In Scope

- Align the template with the skill: an outdated thread whose concern is fixed records
  `Disposition=FIXED` and `Thread state=RESOLVED`. For in-PR feedback, `NO_ACTION` and
  `SUPERSEDED` apply only to an unimplemented duplicate, superseded, or no-change concern;
  preserve the post-merge `NO_ACTION` path when a maintainer declines approved follow-up work
  (F60).
- State in the template that every re-raise has its own tracking row and detail entry, related to
  the earlier finding by `RE_RAISE_OF:<FindingId>` (F73).
- Define admissible `Resolution reference` values for each disposition without adding a field:
  a unique commit subject for `FIXED` and a durable reply URL for `NO_ACTION`, `SUPERSEDED`, or
  `FOLLOW_UP`. Record a `FOLLOW_UP` pull request only in the existing separate Follow-up PR URL
  field (F76).
- State in the template that `Processing Log` entries are append-only, and state in the skill how
  to recover from an in-place rewrite: restore prior entries, append a correction that names the
  rewritten content, and do not rewrite the record again (F61, F62).
- Make the skill completion-checklist wording preserve Step 8's condition: a consolidated PR
  conversation response is needed only when one response covers multiple review rounds; it must
  name every covered review ID and finding ID with its disposition and resolution reference, and
  its durable URL must be recorded in each related row (F79).
- State in the template that an independently actionable review-body finding uses the submitted
  review URL as `Source URL` and `Thread state=NON_RESOLVABLE` (F80).
- State whether Copilot's collapsed `Suppressed comments` are normalized findings. Treat them as
  findings only when their full content is retrievable and contains an independently actionable
  assertion; otherwise record no row and retain the review summary as non-actionable context
  (F66).
- Update the relevant manual-verification evidence and the parent EPIC subissue tracking.

### Out of Scope

- Adding, removing, reordering, or redefining the canonical 19 audit fields; #2295 owns the
  roster.
- Updating `fetch-review-threads` queries or scripts; subissue 3 owns F17, F18, and F32.
- Adding the author self-audit gate or general review workflow changes beyond these contradictions;
  subissue 4 owns that work.
- Changing, porting, or extending the audit validator; subissues 6 and 7 own that work.
- Changing historical audit records under `docs/pr-reviews/`.
- Introducing a repository-wide marker syntax; EPIC #2264 owns that decision.

## Architectural Decisions

- Related ADRs: `docs/adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md`
- ADRs to create: `None known`. This task clarifies the existing review workflow contract within
  its already established ownership boundary.

## Design and Ownership Review

Not applicable. This task changes documentation workflow rules only; it does not introduce child
processes, asynchronous I/O, network readiness, resource cleanup, or reusable fixtures.

## Bug-Fix Process

Not applicable. The task reconciles incomplete and contradictory documentation, not broken runtime
behavior.

## Regression Test Strategy

Not applicable. The changed contract is documentation-only; the issue uses focused manual
verification of each rule against the current skill and template. Validator enforcement belongs to
subissue 7.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| -- | ------ | ---- | ----------------------- |
| T1 | DONE | Define disposition and re-raise rules | Skill and template agree on F60, F73, and F76 without changing the roster. |
| T2 | DONE | Define log recovery and response scope | Skill and template state the F61, F62, and F79 rules. |
| T3 | DONE | Define review-source edge cases | Skill and template state F80 and F66 handling. |
| T4 | DONE | Record evidence and update EPIC tracking | `manual-verification-evidence.md` demonstrates AC1-AC8 and parent row is `IN_PROGRESS` pending the implementation PR. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T1 | Disposition, resolution-reference, and re-raise contract alignment | One signed `docs(pr-reviews)` commit after focused manual verification. |
| T2 + T3 | Log recovery, consolidated-response, review-body, and suppressed-comment rules | One signed `docs(pr-reviews)` commit after focused manual verification. |
| T4 | Evidence and parent EPIC tracking | One signed `docs(issues)` commit after the implementation commits. |

Record a justified no-change decision in task evidence without creating an empty commit. Use a
Conventional Commit message with the narrow affected scope, and sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2278-reconcile-audit-contract-rules/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created, linked as a sub-issue of #2278, and issue number added to this spec
- [x] Spec-only PR #2310 merged into `develop` before implementation
- [x] Implementation completed
- [x] Automatic verification completed (`linter all` and pre-push checks)
- [x] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [x] Evidence-based implementation completion review recorded: no retrospective needed, or a retrospective records a material discovery
- [x] Reviewer validated acceptance criteria and updated checkboxes
- [x] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-09-22 18:10 UTC - GitHub Copilot - Drafted from the approved #2278 matrix entries F60, F61, F62, F66, F73, F76, F79, and F80 after #2295 established the canonical roster; awaiting maintainer review.
- 2026-09-23 07:24 UTC - GitHub Copilot - Maintainer approved the specification; created GitHub sub-issue #2308 under EPIC #2278 and moved this specification to `docs/issues/open/2308-2278-reconcile-audit-contract-rules/`.
- 2026-09-23 09:27 UTC - GitHub Copilot - PR #2310 merged as `56381009`; implemented T1-T3 in `docs(pr-reviews): reconcile disposition rules` and `docs(pr-reviews): define audit recovery rules`.
- 2026-09-23 09:27 UTC - GitHub Copilot - Recorded M1-M4 in `manual-verification-evidence.md`, re-reviewed AC1-AC8 and AC10-AC11, and updated the parent EPIC row to `IN_PROGRESS`; no retrospective is needed because the implementation matched the approved scope without a material discovery.
- 2026-09-23 09:29 UTC - GitHub Copilot - Ran `TORRUST_GIT_HOOKS_LOG_DIR=.tmp ./contrib/dev-tools/git/hooks/pre-commit.sh --format=text`; all eight checks passed and AC9 is satisfied through its `linter all` step.
- 2026-09-23 09:41 UTC - GitHub Copilot Task Reviewer - Independently reviewed commits `09e70c23`, `c5f57f42`, and `c52c61f7`; all AC1-AC11 and T1-T4 pass. Re-ran `linter all` and the required pre-push suite, then recorded the review in `agent-review-reports.md`.
- 2026-09-23 09:52 UTC - GitHub Copilot - Opened implementation PR #2313 and created its audit record; initial GraphQL and PR metadata collection found no reviews, comments, or unresolved threads.

## Acceptance Criteria

- [x] AC1: The skill and template agree that a fixed outdated thread is `FIXED`/`RESOLVED`; in-PR duplicate, superseded, or no-change concerns are `NO_ACTION`/`SUPERSEDED`; and a maintainer-approved post-merge decline remains `NO_ACTION` (F60).
- [x] AC2: The template makes the append-only Processing Log rule explicit, and the skill defines the F62 recovery sequence for an in-place rewrite.
- [x] AC3: The template requires a separate row and detail entry for every re-raise and uses `RE_RAISE_OF:<FindingId>` to identify its predecessor (F73).
- [x] AC4: The skill and template define the permitted `Resolution reference` evidence for `FIXED`, `NO_ACTION`/`SUPERSEDED`, and `FOLLOW_UP` without adding an audit field, and use the separate Follow-up PR URL field for a follow-up pull request (F76).
- [x] AC5: The skill's completion checklist limits the consolidated-response requirement to responses that cover multiple review rounds and requires every covered review ID, finding ID, disposition, resolution reference, and durable response URL (F79).
- [x] AC6: The template defines `Source URL` and `Thread state=NON_RESOLVABLE` for a review-body finding (F80).
- [x] AC7: The skill defines the normalization decision for retrievable and non-retrievable Copilot `Suppressed comments` (F66).
- [x] AC8: The parent #2278 tracking records this subissue as `IN_PROGRESS` only while its implementation PR is pending, and records the verification evidence.
- [x] AC9: `linter all` exits with code `0`.
- [x] AC10: Manual verification scenarios M1-M4 are executed and documented in issue-local `manual-verification-evidence.md`.
- [x] AC11: Acceptance criteria are re-reviewed after implementation and reflect actual behavior.

## Verification Plan

Define verification before implementation starts and execute it before closing the issue.

### Automatic Checks

- `linter all`
- `TORRUST_GIT_HOOKS_LOG_DIR=.tmp ./contrib/dev-tools/git/hooks/pre-commit.sh --format=text`
- Pre-push checks before opening the implementation PR

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| -- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1 | Compare outdated-thread and re-raise rules | Read the skill and template sections governing dispositions, thread state, and findings. | Both documents state the same F60 and F73 behavior. | DONE | `manual-verification-evidence.md` section V1 |
| M2 | Compare resolution-reference rules | Read the skill and template resolution-reference rules for each disposition and Follow-up PR URL. | The admissible evidence is explicit for `FIXED`, `NO_ACTION`/`SUPERSEDED`, and `FOLLOW_UP`; the follow-up pull request remains in its separate field. | DONE | `manual-verification-evidence.md` section V2 |
| M3 | Review append-only and response-scope rules | Read Processing Log guidance, correction recovery, and the completion checklist. | The append-only recovery and conditional consolidated-response rule, including every covered ID, disposition, resolution reference, and durable URL, are explicit and consistent. | DONE | `manual-verification-evidence.md` section V3 |
| M4 | Review body-only and suppressed-comment handling | Inspect the template's review-body guidance and the skill's normalization rule. | The source/thread-state and suppressed-comment outcomes are unambiguous. | DONE | `manual-verification-evidence.md` section V4 |

Notes:

- Manual verification is mandatory even though the work is documentation-only. Record the exact
  inspected sections and observed rules in `manual-verification-evidence.md` created from
  `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`.
- No disposable verification script is planned: the rule comparisons are compact, human-oriented
  contract inspections, and maintained validator automation is explicitly owned by subissue 7.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | DONE | M1; skill/template review |
| AC2 | DONE | M3; skill/template review |
| AC3 | DONE | M1; template review |
| AC4 | DONE | M2; skill/template review |
| AC5 | DONE | M3; skill review |
| AC6 | DONE | M4; template review |
| AC7 | DONE | M4; skill review |
| AC8 | DONE | Parent EPIC row and progress log |
| AC9 | DONE | Successful full pre-commit gate, including `linter all` |
| AC10 | DONE | `manual-verification-evidence.md` |
| AC11 | DONE | Post-implementation criteria review |

## Risks and Trade-offs

- Over-specifying source metadata could imply a new audit field. Mitigation: reuse the existing
  `Source URL`, `Thread state`, and `Resolution reference` fields only.
- Treating every collapsed Copilot summary as a finding could create unverifiable audit rows.
  Mitigation: normalize it only when full content is retrievable and independently actionable.
- A documentation-only change can leave a hidden contradiction. Mitigation: M1-M4 compare every
  adopted matrix item against both normative documents before review.

## Implementation Completion Review

- Retrospective: `Not needed`; the implementation matched the approved scope without a material discovery.
- If implementation uncovers a material ambiguity not represented by F60, F61, F62, F66, F73,
  F76, F79, or F80, create `implementation-retrospective.md` from
  `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this issue directory.
- If no retrospective is needed, add a concise progress-log entry explaining why the work had no
  material discovery.
- When an independent reviewer receives this folder-style specification, it records its result in
  `agent-review-reports.md` using `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- Parent EPIC: #2278; grandparent EPIC: #2003.
- Decision input: `docs/issues/open/2278-2003-strengthen-pr-review-author-self-audit/retrospective-improvement-matrix.md`.
- Prerequisite: #2295, archived at `docs/issues/closed/2295-2278-single-source-audit-roster/ISSUE.md`.
- Unblocks: #2278 subissue 4.
- Related ADR: `docs/adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md`.
