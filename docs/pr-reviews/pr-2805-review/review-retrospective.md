---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - docs/templates/PR-REVIEW-RETROSPECTIVE.md
    - docs/pr-reviews/pr-2805-review/PR-REVIEW.md
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR Review Retrospective — PR #2805

PR #2805 was the spec-only PR for issue #2804 (`Location::caller()` without `#[track_caller]`). It
was merged into `develop` on its own (`3fe63d04b`). This file analyses its review and is carried to
`develop` by PR #2814, the next PR of the same issue, because #2805 was already merged when it was
written. It lives beside the audit record of the reviewed PR, as `PR-REVIEW-RETROSPECTIVE.md`
asks. It does not cover the review of #2814 itself.

## Purpose

Record, blamelessly and from evidence, why the review of #2805 took 24 findings over five rounds,
and what would have prevented the avoidable ones. It complements `PR-REVIEW.md`, which records what
each finding was; this file records why the process cost what it did.

## Review Summary

| Metric                                            | Value                                                                                                                                                                                                                                               |
| ------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Reviews submitted                                 | 7 by reviewers (6 human, 1 bot); findings came in 5 rounds and the last review (16:19 UTC) approved with no finding. Source: `gh api repos/torrust/torrust-tracker/pulls/2805/reviews --paginate`, excluding the replies posted as `josecelano-bot` |
| Audit findings                                    | 24 (21 human, 3 bot)                                                                                                                                                                                                                                |
| Re-raised findings (`RE_RAISE_OF`)                | 0                                                                                                                                                                                                                                                   |
| Findings about the audit record or replies        | 6 of 24 (F13, F15, F20, F21, F23, F24)                                                                                                                                                                                                              |
| Findings about statements in the spec or evidence | 12 (F1, F2, F5, F6, F7, F8, F10, F11, F12, F14, F16, F17)                                                                                                                                                                                           |
| Commits on the branch                             | 13 (7 audit-only; `406d1bb7d` also changes `ISSUE.md`). Source: `gh api .../pulls/2805/commits` and `git show --stat`                                                                                                                               |
| First review to last thread resolution            | 13:45:12 UTC (first review, forge) to 15:59 UTC (last resolution, audit Processing Log), 2 h 14 min; the final approval was at 16:19:06 UTC                                                                                                         |
| AI models used                                    | Claude Sonnet 5.5 (author); reviewers unknown                                                                                                                                                                                                       |

Derived from `PR-REVIEW.md` (rows and Processing Log) and `gh api .../pulls/2805/commits`. The
grouping of findings by theme is this author's reading, not a reviewer statement.

## Findings: cause, type and prevention

| Finding(s)                                       | Reviewer flagged                                                                                             | Root cause                                                                                                    | Type                                     | What would have prevented it                                                                                        |
| ------------------------------------------------ | ------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------- | ---------------------------------------- | ------------------------------------------------------------------------------------------------------------------- |
| F1, F10, F11                                     | UDP exposure claims named variants never built in production; scrape path wrong; database path missing       | Traced the output path from the error types, not from the call graph; F10 repeated a wrong suggestion from F1 | Misread code path; wrong claim in a spec | Trace every claimed path to a production constructor and a test before writing it; mark unchecked paths "to verify" |
| F2                                               | Cited a draft folder that does not exist                                                                     | Copied a path from the briefing without checking it in the tree                                               | Doc/code mismatch                        | `ls` every cited path before citing it                                                                              |
| F3                                               | Reproduction not recorded, no toolchain, red/fix/green folded into one task                                  | Skipped the `fix-bug` "Reproduce Before Review" step and the template's toolchain rule                        | Process (skill not followed)             | Read the skill's pre-review checklist before presenting the spec                                                    |
| F4                                               | Template passages dropped                                                                                    | Wrote the spec from memory of the template instead of copying it                                              | Process                                  | Copy the template and edit; diff against it before review                                                           |
| F5, F6, F7, F8                                   | Wrong conversion names, wrong HTTP messages, wrong test/prod split, wrong semver step                        | Facts stated from a first pass without recomputing                                                            | Doc/code mismatch                        | Recompute every number and quote every message from the source before stating it                                    |
| F9, F22                                          | PR description missing sections, then missing the new audit file                                             | Wrote the description once and did not update it when the branch gained files                                 | Process                                  | Regenerate the Files list from `git diff --stat` on every push                                                      |
| F12, F14, F20, F24 (stamps, references, wording) | Log stamps later than commits; run time later than its commit; one fix cited one of two commits; round order | Wrote times from the clock or from memory, not from `git log`; wrote references from the last commit only     | Doc/code mismatch (self-statements)      | Take every stamp from `git log --format=%cI`; cite all commits a change touched; sweep after the first such finding |
| F13, F23                                         | Replies cited branch SHAs; an edited reply restated a value the tree later changed                           | Reply text written before the final commit existed; replies are outside the tree, so no check re-reads them   | Process / tooling gap                    | A checker that scans replies for SHAs and for stale values; cite subjects only                                      |
| F15                                              | No audit record                                                                                              | The author-owned audit step was not part of the first round plan                                              | Process                                  | Create `PR-REVIEW.md` at the first review, not after the second                                                     |
| F16                                              | Evidence section numbering clashed with scenarios                                                            | Named the drafting section `V1` without checking the scenario table                                           | Doc/code mismatch                        | Number drafting sections apart from scenario IDs from the start                                                     |
| F17, F18, F19 (bot)                              | Async claim stated as fact; table syntax; test paths not fully specified                                     | F17 and F19 were fair: a claim marked "to verify" was worded as fact; F18 was a false positive                | Missed edge case (F17, F19)              | Word unverified claims as hypotheses; define portable substrings in the first draft                                 |

## What Went Well

1. Every reviewer claim was re-derived from the tree before replying, which is why the false
   positive (F18) was declined with evidence and no reviewer claim was applied blindly.
2. Reproducing the defect for F3 and re-running it with `#[track_caller]` turned a spec claim into a
   measured fact (the std line in `str::parse`).
3. The audit validator found a real problem in the first audit commit (padded table rows) before the
   reviewer could.

## What Made the Review Costly

- **Self-statements without a source check** (F5 to F8, F12, F14, F20, F23, F24): the same cause
  appeared in all five rounds. Round 1 (F5 to F8), round 2 (F12, F14: stamps later than their commits), round 3 (F20: one of two
  commits cited); a restated value that the tree later changed was flagged in round 4 (F23), and a misleading order and wording in round 5 (F24). The sweep rule
  (search all records for the same kind of statement) was applied to the value flagged, not to all
  values of the same kind.
- **Replies outside the tree** (F13, F23): replies are written once, edited later, and read by no
  tool. Two findings (F13, F23) and edits of the earlier replies came from them.
- **Wrong claims in the spec's central analysis** (F1, F10, F11): the exposure analysis is the main
  content of the spec, and it was corrected twice; the second correction was caused by following a
  reviewer's suggestion without re-checking the code.
- **Late audit record** (F15 and its follow-ups): the audit rows then needed a new commit for each
  later round, and seven of the 13 commits are audit-only.

## Root Causes

1. No mechanical check of timestamps, commit references and reply text; only the skill's prose
   ("Stamp an event only after it happened").
2. Facts were stated from a first pass; the recomputation step existed only as a reviewer action.
3. The author did not start the audit record at the first review, so the first complete audit
   arrived with round 2.
4. A suggestion in a review was treated as a verified fact (F10).

## What We Learnt

1. A suggestion from a reviewer is a claim to verify, not a fact to copy.
2. When a finding corrects a statement, search every record for the same kind of statement and fix
   them all in the same commit.
3. A value that changes (a date, a count, a field) should be stated once or derived, not restated in
   replies.
4. Create the audit record when the first review arrives.

## Improvements for Future Reviews

1. A check that fails when a Processing Log or progress-log stamp is later than the commit that
   carries it — `validate-audit-record.py` / a spec checker — `PROPOSED`.
2. A check that reports branch SHAs in review replies and in audit records — `validate-audit-record.py`
   — `PROPOSED`.
3. State in `process-pr-review` that a reviewer-suggested trigger or path is verified against the
   code before it is written into a spec — `process-pr-review` skill — `PROPOSED`.
4. Say in the `process-pr-review` intake step that the audit record is created at the first review —
   `process-pr-review` skill — `PROPOSED`.

None is applied in this PR; they change workflow artifacts and need maintainer approval.

## Avoiding Overcorrection

- The review cost came mostly from a few repeated causes; a new field or template section per
  finding is not justified.
- F18 (table syntax) was a false positive of one bot: it does not justify a new table lint.
- The five rounds were mostly nits after round 2; asking for fewer rounds is not an author decision.

## Evidence

- Audit record: `docs/pr-reviews/pr-2805-review/PR-REVIEW.md`
- Pull requests: <https://github.com/torrust/torrust-tracker/pull/2805> (reviewed),
  <https://github.com/torrust/torrust-tracker/pull/2814> (carries this file)
