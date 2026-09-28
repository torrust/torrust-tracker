---
schema-version: 1
doc-type: issue
issue-type: task
status: planned
priority: p2
epic: 2278
github-issue: 2362
spec-path: docs/issues/open/2362-2278-reconcile-residual-audit-contract-rules/ISSUE.md
branch: "2362-2278-reconcile-residual-audit-contract-rules"
related-pr: null
last-updated-utc: "2026-09-28 10:08"
semantic-links:
  skill-links:
    - create-issue
    - process-pr-review
  related-artifacts:
    - docs/issues/open/2278-2003-strengthen-pr-review-author-self-audit/EPIC.md
    - docs/issues/closed/2308-2278-reconcile-audit-contract-rules/ISSUE.md
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/templates/PR-REVIEW-TEMPLATE.md
    - contrib/dev-tools/checks/agent-review-report-contract/src/main.rs
    - .github/skills/dev/planning/create-issue/SKILL.md
    - review-finding:pr-2313-f4
    - review-finding:pr-2313-f5
    - review-finding:pr-2313-f6
    - review-finding:pr-2313-f7
    - review-finding:pr-2313-f9
---

<!-- skill-link: create-issue -->

# Issue #2362 - Reconcile the Residual Audit-Contract Rules From the PR #2313 Post-Merge Review

**Parent EPIC:** #2278 - Strengthen PR Review Author Self-Audit and Evidence Generation

This subissue of #2278 follows #2308. It settles five contract gaps that the post-merge review of
PR #2313 (the #2308 implementation) found still open on `develop`. #2347 routed them here.

## Goal

Make `process-pr-review` and `PR-REVIEW-TEMPLATE.md` give exactly one answer for each of the five
cases below, so that an author following either document records the same disposition, thread
state, and resolution reference.

## Background

Issue #2308 reconciled eight contract rules. Reviewer da2ce7's post-merge review of PR #2313
(review `5293099957`) found five places where the reconciled contract still gives two answers or
none. The #2347 triage re-checked each one on `develop` at `478516cf`, and all five are live:

| Finding | Case | Current state |
| ------- | ---- | ------------- |
| `review-finding:pr-2313-f4` | A duplicate thread whose concern was fixed | The template says both ``An outdated thread whose concern was fixed is `FIXED`/`RESOLVED` `` and that `NO_ACTION`/`SUPERSEDED` applies to "a duplicate, superseded, or no-change concern". Nothing orders the two rules. The PR #2313 audit picked `FIXED` while its log called the item a duplicate. |
| `review-finding:pr-2313-f5` | `Resolution reference` stated three ways | The field placeholder `<UNIQUE_COMMIT_SUBJECT_OR_REPLY_URL>` and the Completion Rules bullet "Cite a fix by its unique Conventional Commit subject or durable reply URL" both contradict the FIXED-only rule ``Use a unique Conventional Commit subject as the `Resolution reference` for `FIXED`.``. `agent-review-report-contract` pins the placeholder. |
| `review-finding:pr-2313-f6` | `FIXED` by a change outside the repository | The skill says `` `FIXED` resolution references are unique Conventional Commit subjects ``. A fix made by editing a PR description, a label, or an issue therefore has no admissible value. This is the F76 case, which #2308 listed as reconciled. |
| `review-finding:pr-2313-f7` | Policy text inside a copied-verbatim section | `## Status Values` is marked "Copied verbatim into each audit record", yet its last bullet is a policy sentence (``A post-merge `NO_ACTION` requires maintainer approval to decline the follow-up work.``). This is the F63 anti-pattern. Records from `pr-2320` onward copy the sentence; `pr-2313` does not. |
| `review-finding:pr-2313-f9` | A consolidated reply covering one review's several findings | Step 8 and the checklist govern only responses that cover multiple review rounds. A single-round response covering several findings, as on #2272, is governed by no rule. |

The F6 gap already constrained #2347: it declined a PR-description fix (`review-finding:pr-2290-f7`)
partly because no `FIXED` resolution reference could cite it.

## Scope

### In Scope

- F4: state which rule governs a duplicate thread whose concern a change fixed, in both the skill
  and the template.
- F5: state the `Resolution reference` rule once, and make the placeholder and the Completion Rules
  bullet agree with it. Update the pinned placeholder in `agent-review-report-contract`. This is a
  one-string Rust pin change, coordinated with #2349.
- F6: define the admissible `Resolution reference` for a `FIXED` finding whose fix is not a
  repository change, for example the durable reply URL that records the edit.
- F7: move the policy sentence out of the copied-verbatim `## Status Values` section into guidance,
  or reclassify the section's mark. Coordinate with #2278 order 8, which byte-diffs the copied
  sections (F63).
- F9: state whether and how a consolidated response may cover several findings of one review.
- Bump the `process-pr-review` skill version, and record focused manual verification.
- Update the #2278 EPIC subissue row.

### Out of Scope

- Changing the 19-field roster (#2295).
- Validator changes, including the byte-diff of copied sections (#2278 orders 7 and 8).
- Pin granularity beyond the one placeholder F5 touches (#2349).
- Rewriting historical audit records under `docs/pr-reviews/`. The records from `pr-2320` onward
  keep the sentence they copied.

## Architectural Decisions

- Related ADRs: `docs/adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md`
- ADRs to create: `None known`. The task clarifies the existing review-workflow contract.

## Design and Ownership Review

Not applicable. This task changes documentation workflow rules and one pinned string; it adds no
process, I/O, or fixture code.

## Bug-Fix Process

Not applicable. The findings are contradictory or missing documentation rules, not broken runtime
behaviour.

## Regression Test Strategy

Not applicable. The existing `agent-review-report-contract` check guards the pinned text. Focused
manual verification checks each rule.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| -- | ------ | ---- | ----------------------- |
| T1 | TODO | Propose the five rules | One proposed wording per finding, approved by the maintainer before editing; recorded in the progress log. |
| T2 | TODO | Resolution-reference rules (F5, F6) | Skill and template state one rule, with a non-repository `FIXED` case; placeholder and pin updated; contract check passes. |
| T3 | TODO | Disposition and response rules (F4, F9) | Skill and template give one answer for a fixed duplicate and for a single-round consolidated response. |
| T4 | TODO | Copied-section content (F7) | `## Status Values` holds only what records copy, or its mark says otherwise. |
| T5 | TODO | Evidence and tracking | `manual-verification-evidence.md` V1-V3 recorded; skill version bumped; #2278 EPIC row updated. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T2 | Resolution-reference rules and pin | One signed `docs(pr-reviews)` commit, or one for F5 and one for F6 if they touch different sentences. |
| T3 | F4 and F9 rules | One signed `docs(pr-reviews)` commit per finding. |
| T4 | Status Values content | One signed `docs(pr-reviews)` commit. |
| T5 | Evidence and tracking | One signed `docs(issues)` commit. |

One finding per commit keeps each finding's `FIXED` resolution reference precise
(`review-finding:pr-2300-f12` describes the failure mode).

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2278-reconcile-residual-audit-contract-rules/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created, linked as a sub-issue of #2278, and issue number added to this spec
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

- 2026-09-28 10:04 UTC - GitHub Copilot - Drafted as follow-up FU-D of #2347 for `review-finding:pr-2313-f4`, `-f5`, `-f6`, `-f7`, and `-f9`, routed here by the #2347 T3 approval (<https://github.com/torrust/torrust-tracker/issues/2347#issuecomment-5865646588>). The #2278 register has no existing owner for these cases. Current-state quotes were re-checked on `develop` at `478516cf`.
- 2026-09-28 10:08 UTC - GitHub Copilot - Maintainer approved the specification as #2278 order 11. Created GitHub issue #2362, linked it as a sub-issue of #2278 (`parent_issue_url` verified), registered it in the #2278 EPIC, and moved this specification to `docs/issues/open/`.

## Acceptance Criteria

- [ ] AC1: For a duplicate thread whose concern a change fixed, the skill and the template give the
      same single disposition and thread state (F4).
- [ ] AC2: The `Resolution reference` rule is stated once. The template placeholder and the
      Completion Rules bullet agree with it, and the contract check pins the new placeholder (F5).
- [ ] AC3: A `FIXED` finding whose fix is not a repository change has a stated admissible
      `Resolution reference` (F6).
- [ ] AC4: Every line in a section marked copied verbatim is content that records copy, or the
      mark names the exception (F7).
- [ ] AC5: The skill and the template state whether and how one response may cover several
      findings of a single review (F9).
- [ ] AC6: `cargo run --package agent-review-report-contract` passes, and
      `validate-audit-record.py` still passes for the three most recent audit records.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo run --package agent-review-report-contract` (stable Rust toolchain)
- `python3 .github/skills/dev/pr-reviews/process-pr-review/scripts/validate-audit-record.py --pr-number <N>` for the three most recent audits
- Pre-push checks

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| -- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1 | One answer per case | For each of F4, F5, F6, and F9, search the skill and the template for the rule with a recorded `rg` pattern that matches every statement of it. | Each case has one rule, stated consistently in both documents. | TODO | `manual-verification-evidence.md` section V1 |
| M2 | Worked cases | Apply the new rules to the PR #2313 F2 thread (a fixed duplicate) and to `review-finding:pr-2290-f7` (a fix outside the repository). | Each gets one unambiguous row shape. | TODO | `manual-verification-evidence.md` section V2 |
| M3 | Copied sections | Compare `## Status Values` in the template with the same section in the newest audit record. | Only copied content remains in the section, or the mark names the exception. | TODO | `manual-verification-evidence.md` section V3 |

No disposable verification script is planned.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | TODO | M1, M2 |
| AC2 | TODO | M1; contract check |
| AC3 | TODO | M1, M2 |
| AC4 | TODO | M3 |
| AC5 | TODO | M1 |
| AC6 | TODO | Automatic checks |

## Risks and Trade-offs

- F5 changes a pinned placeholder while #2349 changes pin granularity. Mitigation: whichever lands
  second rebases, and the pin change here is one string.
- Moving the policy sentence (F7) makes the template's `## Status Values` differ from the section
  that records from `pr-2320` onward copied. Mitigation: historical records stay valid; order 8's
  byte-diff applies to records created after the change.

## Implementation Completion Review

- Retrospective: `Not yet assessed`. Create `implementation-retrospective.md` if the work shows why
  #2308's reconciliation left these gaps; otherwise record why none was needed.

## References

- Source review: <https://github.com/torrust/torrust-tracker/pull/2313#pullrequestreview-5293099957>.
- Findings: `review-finding:pr-2313-f4`, `-f5`, `-f6`, `-f7`, and `-f9` (audit
  `docs/pr-reviews/pr-2313-review/PR-REVIEW.md`, rows added by #2347).
- Precedent: #2308 (PR #2313).
- Routing: #2347.
