---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - docs/templates/PR-REVIEW-TEMPLATE.md
    - docs/templates/PR-REVIEW-RETROSPECTIVE.md
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR Review Retrospective — PR #2812

## Purpose

Record a blameless, evidence-based review of how PR #2812 was reviewed and how its findings were
processed. It complements the audit record
[`PR-REVIEW.md`](PR-REVIEW.md), which tracks what each finding was and how it was resolved. It was
requested by the maintainer through a peer session and written by the PR author (an AI agent).

## Review Summary

| Metric | Value |
| ------ | ----- |
| Review rounds | 8 reviews (6 human, 2 bot); the bot reviews had no findings |
| Audit findings | 20 (F1 to F20), all `FIXED`, none declined |
| Re-raised findings (`RE_RAISE_OF`) | 0 recorded; F15 and F20 are the same defect class at different places |
| Findings about the audit record or the replies | 7 (F10, F11, F15, F16, F18, F19, F20); F17 is about the spec's progress log |
| Commits on the branch | 40 before this retrospective's commit (9 audit-only `docs(pr-reviews)` commits) |
| First review to last thread resolution | 17:35 to about 21:27 UTC on 2026-11-10 |
| AI models used | Claude Sonnet 5.5 (author), unknown (reviewers) |

Derived from the audit record (`grep -c '| ORIGINAL |'` gives 20 rows), `git log --oneline
torrust/develop..HEAD` (40 commits), `git log --format=%s` filtered on `docs(pr-reviews)` (9), and
the GitHub reviews API for the review list and times.

## What Went Well

1. The reviewer's findings were concrete and verifiable from the bytes, so every one could be
   fixed without a design discussion. No finding was declined.
2. The audit validator (`validate-audit-record.py`) caught row, reply and commit-subject errors
   mechanically once the record existed.
3. Before the review, the implementation passed every automated check (metadata check,
   `cargo package --list`, doctests, `linter all`). No review finding concerned a crate's code.

## What Made the Review Costly

The change itself drew 9 findings (F1 to F9) in the first two reviews. The other 11 came from how
the findings were recorded and replied to, and 20 findings took 6 rounds and 40 commits.

| Finding | Flagged | Root cause | Problem type | What would have prevented it |
| ------- | ------- | ---------- | ------------ | ---------------------------- |
| F1 | 13 crates' keywords repeat a crate-name word, against the spec's rule | I proposed a keyword table that broke the rule I had written in the spec; I saw the conflict while generating the manifests and did not raise it before the maintainer approved the table | Doc/code mismatch (spec vs manifests) | Checking the proposed table against the rule before asking for approval; stating the exception in the rule from the start |
| F2 | M1 marked DONE with no evidence; evidence file off-template | I ticked the scenario from automated checks and wrote a short evidence file instead of following the template | Process (evidence not recorded) | Following `MANUAL-VERIFICATION-EVIDENCE.md` literally; running the URL and read-through checks before ticking |
| F3 | Audit claimed a decision the spec did not record | I summarized a decision from memory, not from the spec text | Misread requirement | Quoting the spec's decisions table when writing audit and log text |
| F4 | Template missing the index row, frontmatter and skill pointer | I skipped the steps of the `create-markdown-template` skill | Process (skill steps skipped) | Reading the skill's checklist before creating a template |
| F5 | Audit notes claimed fixes the PR did not make | Notes were generated from a table, not checked against `git diff --stat` | Doc/code mismatch | Checking every "fixed by" claim against the diff |
| F6 | README pointed at deprecated re-exports | I listed the crate's public names without checking their deprecation attributes | Missed edge case | Grepping `#[deprecated]` in the crate before listing its API |
| F7 | Spec bookkeeping lagged the implementation | I ticked boxes at the end without re-reading the frontmatter and checkpoints | Process (stale bookkeeping) | A final pass over the spec frontmatter and progress list before each push |
| F8 | License section named another product ("prosperity") | I copied the license block from the client README | Doc/code mismatch (copy-paste) | Fixing the product name and spelling when copying shared text |
| F9 | The amended rule's word list missed three words of the approved table | I fixed F1 by listing four words instead of pointing at the table | Missed edge case | Making the table the exception, not a second list |
| F10 | F2 to F8 replies were posted before the commit they cite existed | A first commit attempt failed on a lint, but I posted replies in the same command chain | Process (reply before push) | The rule "reply only after the commit is pushed", and a script that gates replies on `git ls-remote` |
| F11 | No audit record on the branch | I did not know the repo's `process-pr-review` skill requires one and did not read the skill before replying | Process (skill not read) | Reading `process-pr-review` as soon as the first review arrives |
| F12 | Time stamps later than the commit; log entries without a time | I wrote the time of a planned edit, not of the event | Doc/code mismatch (stamps) | Taking stamps from `git log --format=%cI` or `date -u` at edit time |
| F13 | M3 pointed at the wrong section after a renumbering | I renumbered evidence sections and did not sweep references | Doc/code mismatch | Searching for section names after any renumbering |
| F14 | Baseline said "every member" inherited a field | Overstated from the tracker packages to all members | Missed edge case | Counting members from `cargo metadata` instead of recalling |
| F15 | Audit introduction cited branch commit ids | I wrote heads by id in prose although the skill bars it | Process (rule not applied) | Running the skill's commit-id check before every audit push |
| F16 | "Later pushed" suggested a timing fault that did not happen | Careless wording next to F10 | Doc/code mismatch (wording) | Stating only what the forge data shows |
| F17 | A progress-log entry named a record that did not exist yet | Log entry written ahead of the event | Process (stamp before the event) | The "stamp an event only after it happened" rule, applied to the spec log too |
| F18 | F17 was missing from the record and its thread was resolved without a reply | I fetched only the newest comments and resolved every open thread in one loop | Tooling/process (bulk resolve) | Listing each review's comments by review ID; resolving only threads with my reply |
| F19 | Intro still counted three rounds; reply timing was not recorded | A hand-kept round count went stale | Doc/code mismatch (derived count) | Not keeping counts in prose; listing review IDs |
| F20 | A log entry named a review head by branch commit id | The F15 fix did not sweep the rest of the file | Missed edge case | Running the skill's "sweep for the same mistake" search on the whole record, not the cited line |

Rounds 3 to 6 contained only findings about records (F15 to F20); round 6 was the same defect as
round 3's F15 elsewhere in the file.

## Root Causes

1. **The audit and reply rules were not read first.** `process-pr-review` was applied only after a
   reviewer cited it (F10, F11, F15). Replies, stamps and ids then went wrong one at a time.
2. **Fixes were local, not swept.** F1, F9, F15 and F20 are the same defect fixed at the cited
   line while the same mistake stayed elsewhere. The skill's sweep rule asks for the search, but
   nothing runs it.
3. **Records describe themselves.** Each audit commit changes the record that describes the
   previous commit, so each fix invited a new bookkeeping finding (F15 to F20 followed F10 and F11).
4. **Chained commands.** Commit, push, reply and resolve ran in one command chain or one loop, so a
   failed step did not stop the later ones (F10, F18).
5. **Slow pre-push hook.** About 9 minutes and one dropped SSH connection made a push expensive,
   which tempted batching and replying early.

## What We Learnt

1. Read the review-processing skill before the first reply, not after the first finding about it.
2. A fix for a defect class includes a search for the same class over the whole branch.
3. State an event only after it happened, and take its time from the system, not from memory.
4. Never reply or resolve in the same step as the commit or push it depends on.
5. Resolve only the threads you replied to; list comments by review ID, not by recency.
6. Do not keep derived counts or commit ids in prose; list review IDs and commit subjects.

## Improvements for Future Reviews

1. A script that, for each open thread, prints the review ID, the reply state and whether the cited
   commit is on the remote, and refuses to resolve a thread without a reply — `wait-for-pr-review`
   or `resolve-review-threads` skill — `PROPOSED`.
2. Run the `open-pull-request` commit-id check and a timestamp-versus-`git log` check as part of
   `validate-audit-record.py` — `process-pr-review` scripts — `PROPOSED` (this overlaps issue #2503,
   the author self-audit gate).
3. Make the poller list every comment of a review in its result so a finding is not missed —
   `wait-for-pr-review` tool — `PROPOSED`.
4. State in `process-pr-review` that the audit record and the spec progress log are reviewed like
   code, so the same rules apply to both — `process-pr-review` skill — `PROPOSED`.

None of these is applied in this PR: changing the skills or checkers needs the maintainer's
approval and a separate issue.

## Avoiding Overcorrection

- No new field is needed in the audit record: the 19 required fields were enough.
- No rule against batching fixes is needed; the cost came from replying before the push, not from
  batching.
- A reviewer-facing rule to stop at a round count is not supported by this evidence: every finding
  was valid and was fixed.

## Evidence

- Audit record: [`docs/pr-reviews/pr-2812-review/PR-REVIEW.md`](PR-REVIEW.md)
- Pull request: <https://github.com/torrust/torrust-tracker/pull/2812>
- Reviews (human): 5846465906, 5846866430, 5847274601, 5847680359, 5847973119, 5848344870; bot:
  5846485416, 5848128038 (`gh api repos/torrust/torrust-tracker/pulls/2812/reviews`).
- Commit counts: `git log --oneline torrust/develop..HEAD`.
