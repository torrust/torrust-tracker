---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - docs/templates/PR-REVIEW-TEMPLATE.md
    - docs/templates/IMPLEMENTATION-RETROSPECTIVE.md
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR Review Retrospective — PR #<PR_NUMBER>

> Create this concrete artifact only as lowercase `review-retrospective.md` inside
> `docs/pr-reviews/pr-<PR_NUMBER>-review/`, beside the `PR-REVIEW.md` audit record.
> Write it when the review process itself, not the reviewed change, produced reusable lessons:
> many rounds, many re-raises, or findings about the audit record rather than the code.

## Purpose

Record a blameless, evidence-based review of how the pull request was reviewed and how its
findings were processed. This complements the audit record, which tracks what each finding was
and how it was resolved; the retrospective explains why the process cost what it did and what to
change. It does not replace the audit record and is not required for routine reviews.

## Review Summary

| Metric | Value |
| ------ | ----- |
| Review rounds | <COUNT> (<HUMAN> human, <BOT> bot) |
| Audit findings | <COUNT> |
| Re-raised findings (`RE_RAISE_OF`) | <COUNT> |
| Findings about the audit record itself | <COUNT> |
| Commits on the branch | <COUNT> (<AUDIT_ONLY> audit-only) |
| First review to last thread resolution | <DURATION_OR_RANGE> |

Derive every number from the audit record, `git log`, or the GitHub API, and say how.

## What Went Well

1. <PRACTICE_OR_DECISION_THAT_REDUCED_COST_OR_CAUGHT_A_REAL_DEFECT>
2. <PRACTICE_OR_DECISION_THAT_REDUCED_COST_OR_CAUGHT_A_REAL_DEFECT>

## What Made the Review Costly

Describe the concrete failure modes, each tied to finding IDs in the audit record and to the
round in which they appeared. Distinguish defects in the reviewed change from defects in how the
findings were recorded or replied to.

## Root Causes

Explain the process or tooling gaps that allowed each failure mode to recur. Focus on system
causes: missing checks, ambiguous rules, wrong sequencing, tool limitations. Do not assign blame.

## What We Learnt

1. <LESSON_STATED_AS_A_REUSABLE_RULE_OF_THUMB>
2. <LESSON_STATED_AS_A_REUSABLE_RULE_OF_THUMB>

## Improvements for Future Reviews

List specific, reusable changes with an owner artifact (skill, template, checker, agent) and a
disposition: `APPLIED_IN_THIS_PR`, `FOLLOW_UP_ISSUE`, or `PROPOSED`. Do not apply substantial
workflow changes inside the reviewed pull request without maintainer approval.

1. <IMPROVEMENT> — <OWNER_ARTIFACT> — <DISPOSITION>
2. <IMPROVEMENT> — <OWNER_ARTIFACT> — <DISPOSITION>

## Avoiding Overcorrection

State which additional rules, fields, or tooling are not justified by the evidence.

## Evidence

- Audit record: `docs/pr-reviews/pr-<PR_NUMBER>-review/PR-REVIEW.md`
- Pull request: <PR_URL>
- <COMMITS_REVIEW_IDS_OR_COMMANDS_THAT_SUPPORT_THE_NUMBERS_ABOVE>
