---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - docs/templates/PR-REVIEW-RETROSPECTIVE.md
    - docs/pr-reviews/pr-2815-review/PR-REVIEW.md
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR Review Retrospective — PR #2815

Written by the PR author, at the maintainer's request, from the audit record and `git log`. It follows
[the template](../../templates/PR-REVIEW-RETROSPECTIVE.md), at the template's path rather than the
`docs/retrospectives/` path the request suggested, and adds a per-finding table.

## Review Summary

| Metric | Value |
| ------ | ----- |
| Review rounds | 4 (3 human, 1 bot) |
| Audit findings | 21 |
| Re-raised findings (`RE_RAISE_OF`) | 0 |
| Findings about the audit record or my own statements | 6 (F4, F10, F11, F14, F15, F21) |
| Commits on the branch | 15 at the time of writing, 4 of them audit-only |
| First review to last thread resolution | about 54 minutes (19:36 to about 20:30 UTC, 2026-11-10) |
| AI models used | Claude Sonnet 5.5 (author); reviewers unknown |

Numbers: the table in `PR-REVIEW.md` for findings, `git log torrust/develop..HEAD` for commits, and the
GitHub review and comment times for the duration.

## What Went Well

1. The fix itself and its regression tests drew no objection: every finding was about wording, tests of the
   fix, evidence or records, none about the production read order.
2. Re-deriving a claim before replying caught that two Copilot findings about `FakeGh::replace_after` were wrong
   (`next()` repeats the last response); they were answered with the reason, not changed.

## What Made the Review Costly

Per finding, grouped. The type is one of: doc/code mismatch, test gap, process, tooling, misread requirement.

| Findings | Flagged | Root cause | Type | What would have prevented it |
| -------- | ------- | ---------- | ---- | ---------------------------- |
| F19, F20, F6, F13 | Wording said "at least as new as any commit", "a push after this read", "the next poll reports it" | I wrote the invariant loosely and did not list the cases (push before the feedback, between the reads, after the head read, with new feedback) | doc/code mismatch | Write the invariant as a table of cases before writing the comment, and read it against `detect` order |
| F1, F12 | Risks section and PR description still described a second read | I changed the design (one read moved, no second read) after drafting the spec and PR text and did not sweep them | doc/code mismatch | After any design change, grep the spec, PR body and commit bodies for the old claim |
| F4 | Fix commit body said "read again" | The commit message was written from the spec text, not from the diff | doc/code mismatch | Write the commit body from `git diff` |
| F3, F7 | Ordinary-case test had no feedback; comment test used the same trigger as the review test | I counted tests by name, not by what each one fails on | test gap | Run each new test against the old code and check that exactly the intended ones fail, for the reason named |
| F2, F8, F16 | V2 did not name the build, the test count was wrong, an annotation sat in an output block | I summarized results instead of pasting them with the toolchain and the commit | process | Paste command output unedited with the commit subject and toolchain; label excerpts |
| F5, F9 | A criterion stayed unticked; `status` stayed `planned` | I updated the spec by hand without a checklist | process | A close-out checklist for the spec frontmatter and criteria |
| F14, F21, F10 | Stamps written before the events; replies cited a commit and a validation before they existed; no audit record | I invented stamps, wrote replies before the commit, and skipped the `process-pr-review` skill because a plain review looked like routine work | process | Read the skill when the first review arrives; take stamps from `git log --format=%cI`; commit and validate the record before replying |
| F11, F15 | Replies cited branch commit ids; one commit carried eight fixes | Same cause: the skill's rules (subjects, one fix per commit) were not read | process | Same as above |
| F17, F18 | Copilot: `replace_after` would panic on a second call | Misreading by the reviewer (the one-item queue repeats); my doc comment said "from then on" without saying how | misread requirement (reviewer) | Say in the helper's doc that a one-item queue repeats |

Two events were mine alone: the first resolve loop closed nine of the maintainer's threads that I had not read,
and I pushed a commit whose validator run had failed (F21's source review id).

## Root Causes

1. **Skill not read at the first review.** Four rounds' worth of record-keeping findings (F10, F11, F14, F15,
   F21) follow from not opening `process-pr-review` until the third round.
2. **No sweep after a design change.** The "one read moved" decision reached the code and the skill but not
   the spec, the PR text or the commit body.
3. **Loose wording of an invariant** with several cases.
4. **Tooling gaps:** a resolve-all loop that does not show what it resolves; no check that runs the
   audit validator before a push; the validator cannot pass before the replies it cites exist.

## What We Learnt

1. When the first review arrives, open `process-pr-review` and create the audit record before replying.
2. After a design change, search every record on the branch for the old claim.
3. A test counts only when it fails against the old code for the stated reason.
4. A reply states what has happened, not what is about to happen.

## Improvements for Future Reviews

1. Add the "open `process-pr-review` and start the audit at the first review" step to the `wait-for-pr-review`
   review loop. — `.github/skills/dev/pr-reviews/wait-for-pr-review/SKILL.md` — PROPOSED
2. A resolve helper that lists the thread and its author before resolving, and refuses an unread thread. —
   `resolve-review-threads` skill — PROPOSED
3. A pre-push check that runs `validate-audit-record.py` when `docs/pr-reviews/pr-*-review/` changes. — pre-push
   hook — PROPOSED

## Avoiding Overcorrection

Nothing in this review shows a need for new audit fields or a stricter reply format. The findings about
records were all covered by existing rules; the gap was in following them, not in the rules.

## Evidence

- Audit record: `docs/pr-reviews/pr-2815-review/PR-REVIEW.md`
- Pull request: https://github.com/torrust/torrust-tracker/pull/2815
- `git log torrust/develop..HEAD`, the review and comment times from `gh api repos/torrust/torrust-tracker/pulls/2815/reviews` and `.../comments`.
