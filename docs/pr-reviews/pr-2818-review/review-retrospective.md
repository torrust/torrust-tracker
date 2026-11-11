---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - docs/templates/PR-REVIEW-RETROSPECTIVE.md
    - docs/pr-reviews/pr-2818-review/PR-REVIEW.md
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR Review Retrospective — PR #2818

Written by the PR author from the audit record and `git log`, following
[the template](../../templates/PR-REVIEW-RETROSPECTIVE.md).

## Review Summary

| Metric | Value |
| ------ | ----- |
| Review rounds | 7 (6 human, 1 bot) |
| Audit findings | 22 |
| Re-raised findings (`RE_RAISE_OF`) | 0 |
| Findings about the audit record itself | 14 (F6, F7, F8, F10, F12, F13, F15, F16, F17, F18, F20, F21, F22, F23) |
| Commits on the branch | 14 at the time of writing (10 audit and retrospective only) |
| First review to last thread resolution | not captured (first review 09:35 UTC, 2026-11-11; the forge's resolution times were not recorded) |
| AI models used | Claude Sonnet 5.5 (author), unknown (reviewer) |

Numbers: the table in `PR-REVIEW.md`, `git log torrust/develop..HEAD`, and the review times from the GitHub API.

## What Went Well

1. The change was small (two skills, 13 lines added and 4 removed), so there were seven rounds and every finding was minor.
2. The reviewer re-checked the motivation against the audits and found that the description overstated it
   (F3); the description now states only what the audits show.

## What Made the Review Costly

F1 and F2 were gaps in my wording: the first sentence forbade a late start instead of handling it, and the
trigger fired on events that need no record. Both came from writing the rule for the common case and not
walking the other cases (a takeover after round 1, a clean first review). F3 came from summarizing six
retrospectives from memory of the task brief instead of from each audit. Copilot's F4 and F5 (ambiguous
terms, the rule stated in full in two skills) are the same kind: wording and duplication I could have checked before pushing.

Round 3 (F6 to F9) was about the record: a conflict between the start rule and the validator step, a Solution
that described removed bytes (F7), and a log stamp that was the time of my replies instead of the review's
arrival (F8). These are recording defects, not defects in the skills' intent.

Round 4 (F10, F12) and round 5 (F13, F15, F16, F17) were again about the record: an in-place log rewrite, a
Solution describing removed bytes, branch commit ids, and a resolution time written before it happened. Each fix
created the next nit because I edited the record without re-reading every claim in it against the tree.

Round 6 (F18 to F21) was again recording detail: a Resolution reference, a count, a Concern and two log stamps; F19 was about this retrospective's own count.

Round 7 (F22, F23) came from pushing the replies before the record: the reviewer saw a head that answered the threads but had no rows for them. Rows can be pushed with the fix and their reply URLs added in a following commit, so the branch never shows answered threads without rows for them.

## Root Causes

1. A new rule was written without listing its cases, as in earlier retrospectives (PR 2815, wording of an invariant).
2. A motivation claim was not checked against each cited source.
3. The record again started after the replies. The validator step said to run it "before replying", which I
   read as requiring reply URLs that exist only after replying. The sequence create, reply, fill in the reply
   URLs, validate works; the conflicting line now says to validate once the reply URLs are recorded.

## What We Learnt

1. Before committing a rule, list the cases it covers and the ones it must not cover.
2. Check a claim about N sources against each source, not against a summary of them.

## Improvements for Future Reviews

1. Scope the validator step to rows whose reply URLs are recorded, and say in the start rule that reply URLs
   are filled in after replying. — `process-pr-review` skill — APPLIED_IN_THIS_PR

## Avoiding Overcorrection

Nothing here justifies new audit fields or a stricter reply format; the findings were about wording.

## Evidence

- Audit record: `docs/pr-reviews/pr-2818-review/PR-REVIEW.md`
- Pull request: <https://github.com/torrust/torrust-tracker/pull/2818>
- Review `5856655819`; `git log torrust/develop..HEAD`.
