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

# PR Review Retrospective - PR #2810

## Purpose

A blameless, evidence-based look at how PR #2810 (issue #2762, EPIC #2758: `package-coverage-check` follows the CLI output
contract) was reviewed and how its findings were processed. The audit record is
[`PR-REVIEW.md`](PR-REVIEW.md); finding IDs below are its IDs (F1 to F23), not the reviewers'.

## Review Summary

| Metric | Value |
| ------ | ----- |
| Review rounds | 5 (4 human, 1 bot): da2ce7 reviews 5846429188, 5846796508, 5847265767 and 5847623438; Copilot review 5846504036 |
| Audit findings | 23 |
| Re-raised findings (`RE_RAISE_OF`) | 8 (F9 to F12, F16, F17, F18, F22) |
| Findings about the audit record itself | 9 (F13 to F15, F18 to F23) |
| Commits on the branch | 8 (4 audit-only: the four `docs(pr-reviews)` commits) |
| First review to last thread resolution | about 17:33Z to 20:00Z on 2026-11-10 (the resolution times are not captured; 20:00Z is the last reply) |
| AI models used | Claude Sonnet 5.5 (author), unknown (reviewers) |

Numbers come from the audit table (rows and `Relationship` column), `git log torrust/develop..HEAD` and the review and
comment `created_at` times from the GitHub API.

## What Went Well

1. The one real defect in the code (F1, a failing `summary` hidden by the `jq` pipe) was caught in the first review, before merge, and the fix was one line.
2. The red tests were committed first, so each regression test was seen failing against the old binary; the mutation check (F4) showed the unit test fails when the exit code is wrong.
3. Both reviewers' later rounds were small and the last two were APPROVED; code and workflow were byte-identical after round 1, so each re-review was cheap.

## What Made the Review Costly

One real defect in the change (F1). Everything else is a defect in the records the author wrote (evidence, spec progress,
audit, replies), and most of it recurred across rounds. Per finding:

| ID | Round | What the reviewer flagged | Root cause | Type | What would have prevented it |
| -- | ----- | ------------------------- | ---------- | ---- | ---------------------------- |
| F1 | 1 | `jq` pipe hides a failing `summary`: the step has no `pipefail` (Major). | Assumed GitHub Actions bash runs with `pipefail` from memory; never read the runner's documented default (`bash -e {0}`) and never ran the failure path of the pipe. | Unverified assumption (tooling default) and test gap on the failure path | Run the exact step block with a failing `cargo run` before claiming V3; read the documented default shell. |
| F2 | 1 | TTY record's `exit=2` cannot come from the binary: `$?` in double quotes expands in the outer shell. | Quoting slip in an ad hoc command; the output was typed into the evidence, not copied from a verified run. | Tooling (shell quoting) | Single quotes, and paste the verbatim output of a command that is also stored in the evidence. |
| F3 | 1 | AC5 said "confirmed on the PR run" before the PR existed; M3 and T4 `DONE` against `actionlint` that did not run. | Wrote the expected result as the observed result. | Process (claim before evidence) | State only what ran; leave rows open until the evidence exists. |
| F4 | 1 | Evidence lacks the red output, the green output, the unit test seen red and the build of each recheck. | Did not read the `fix-bug` evidence list as a checklist. | Doc gap against the process | Tick the skill's evidence list (red, green, mutation, build id) before the first push. |
| F5 | 1 | T1 `DONE` for TTY tests, but no TTY test exists; the reason is only in a test header. | The test boundary changed during the work and the spec was not updated. | Doc and code mismatch | Update Regression Test Strategy in the same commit that changes the boundary. |
| F6 | 1 | `linter all` checkpoint ticked but the acceptance item open, no run recorded. | Two lists describing one fact edited at different times. | Doc inconsistency | Check every checkbox against the other lists once, after the last code change. |
| F7 | 1 | Test header says the TTY return is in `main`; it is in `run`. | Comment copied from the `ci-gate` tests, which have the same slip. | Copy-paste | Verify any copied comment against the code it describes. |
| F8 | 1 | Log entry and `last-updated-utc` say 12:00 though a run in the evidence is dated 16:49. | Invented a round time instead of reading it from git. | Process (stamps) | Take stamps from `git log --format=%cI` or the event time. |
| F9, F10 | 1 (Copilot) | Same concerns as F1 and F2, raised on the old head. | The review ran on the head before the fix. | Tooling (review timing) | None needed; reply with the fixing commit and resolve. |
| F11 | 2 | T4, AC5 and V3 cite a hosted-run note the PR does not carry. | The round-1 fix reworded the claim but still pointed to a note that was never written. | Process (claim before evidence) | Write the evidence first, then cite it. Here the hosted run was read and recorded as H1. |
| F12 | 2 | Corrected stamps are later than the commits carrying them. | Stamped the intended commit time, not an event already past. | Process (stamps) | Use the time of the previous commit or of the event; never a time after the commit. |
| F13 | 2 | Audit has no rows for Copilot's two findings. | Treated the Copilot findings as duplicates of F1 and F2 and skipped the audit step for re-raises. | Process (audit coverage) | Count review threads against audit rows before each push. |
| F14 | 2 | Evidence and audit cite PR-branch commits by id. | Cited ids for convenience; the rule is to cite the Conventional Commit subject. | Process (rule not applied) | Grep the changed files for hex ids before the push. |
| F15 | 2 | Audit stamp and log entry postdate their commit. | Same cause as F12. | Process (stamps) | Same as F12. |
| F16 | 3 | AC5 stays checked while the rendered hosted summary was not read. | Evidence supported AC5 only by inference (identical bytes, passing step). | Doc and evidence mismatch | Say what was observed and what was not; the page text and API do not expose the job summary. |
| F17, F18 | 3 | Stamps again later than their commit. | The F12 fix was applied only to the instances the reviewer named. | Process (incomplete sweep) | After a finding, search for every sibling of the same mistake, not only the cited lines. |
| F19 | 3 | Log says replies follow the push, but they were posted before the commit. | Replies cite the commit subject, so they were posted first; the log was written as if after. | Process (event order) | Write the log entry after the replies, with their `created_at` times. |
| F20 | 3 | A log entry was rewritten in place. | Edited the log instead of appending a correction. | Process (append-only record) | Restore the entry and append a correction that names what changed. |
| F21 | 3 | Ten older replies still name commits by id. | F14 was fixed in the files only, not in the replies the audit cites. | Process (incomplete sweep) | Sweep review replies as well as the tree; edit the replies through the API. |
| F22 | 4 | The round-3 entry again says replies follow the push. | F19 was corrected in the older entry only, and the new entry repeated the mistake. | Process (repeat of F19) | Write the entry after its events and check the new text against the same rule. |
| F23 | 4 | Round sentences out of order; "below" points the wrong way. | Appended text without rereading its position. | Doc | Reread the paragraph and the cross-references after each insertion. |

Patterns across the table:

1. **Records before events** (F3, F8, F11, F12, F15, F17, F18, F19, F22): a stamp or a claim written before the event it describes.
2. **Unverified claims** (F1, F2, F3, F11, F16): an assumption or an expected result recorded as an observation.
3. **Incomplete sweeps** (F17, F18, F21, F22): each fix covered the instances the reviewer cited, not their siblings.
4. **Cost:** four da2ce7 rounds and about four workflow approvals for one real defect; every push needs a maintainer approval.

## Root Causes

- Evidence and audit text were drafted ahead of the work (expected results, planned times) and corrected later, in place.
- No mechanical check relates review threads to audit rows, stamps to commit times, or commit ids in replies; the validator checks the table shape only.
- A fix was scoped to the cited lines; no step asked "where else does this mistake occur" (the `process-pr-review` sweep rule exists but was applied late).
- The failure path of the pipe was not exercised, and the runner's default shell was assumed.

## What We Learnt

1. Run the failure path of anything that changes an exit status before claiming it works, and read the tool's documented default.
2. Write a record after its event, from `git log --format=%cI` or the API time, and append corrections instead of editing.
3. After a finding, search the whole PR (files, replies, description) for the same mistake before the next push.
4. Count threads against audit rows before each push.

## Improvements for Future Reviews

1. Check that every review thread has an audit row, that no `last-updated-utc` or log stamp is later than its commit, and that no commit id appears in the audit or evidence — `frontmatter-validator` or a new audit checker — PROPOSED
2. Add a failure-path step for changed pipes and exit codes to the `implement-workflow` skill — `.github/skills` — PROPOSED
3. Put "write the log entry after the replies" and "sweep replies as well as the tree" into the `process-pr-review` checklist — `process-pr-review` skill — PROPOSED

None is applied in this PR: they change repository workflow and need the maintainer's approval.

## Avoiding Overcorrection

The evidence supports a checker for stamps, row counts and commit ids; it does not support more fields in the audit
template or more review rounds. The only code defect was found in round 1.

## Evidence

- Audit record: `docs/pr-reviews/pr-2810-review/PR-REVIEW.md`
- Pull request: <https://github.com/torrust/torrust-tracker/pull/2810>
- Reviews and comments: GitHub API `pulls/2810/reviews`, `pulls/2810/comments`, `issues/2810/comments`; commits: `git log torrust/develop..HEAD --format='%cI %s'`
