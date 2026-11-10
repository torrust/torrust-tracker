---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2815 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2815.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A

## Status Values

- Relationship: `ORIGINAL`, `RE_RAISE_OF:<FindingId>`
- Disposition: `FIXED`, `NO_ACTION`, `SUPERSEDED`, `FOLLOW_UP`
- Thread state: `OPEN`, `RESOLVED`, `NON_RESOLVABLE`, `SUPERSEDED`
- Severity: `Blocker`, `Major`, `Minor`, `Nit`, `Suggestion`; append `(inferred)` when derived from free prose.
- Author class: `Copilot`, `Human`, `Unknown`
- Category: `link-integrity`, `formatting`, `metadata`, `testing`, `correctness`, `documentation`, `maintainability`, `security`, `other`

## Findings

Findings F1 to F9 and F10 to F16 are the maintainer's own IDs of two review rounds. Copilot's four comments gave no IDs and were raised first; they are F17 to F20, because F1 to F9 were already taken by the maintainer's IDs. F4 stays `OPEN`: the maintainer decides whether the fix commit message is reworded.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2815-f1 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2815-f2 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2815-f3 | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2815-f4 | Human | Minor | documentation | ORIGINAL | NO_ACTION | OPEN |
| F5 | review-finding:pr-2815-f5 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2815-f6 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2815-f7 | Human | Nit | testing | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2815-f8 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-2815-f9 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-2815-f10 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-2815-f11 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F12 | review-finding:pr-2815-f12 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F13 | review-finding:pr-2815-f13 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F14 | review-finding:pr-2815-f14 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F15 | review-finding:pr-2815-f15 | Human | Minor | maintainability | ORIGINAL | FIXED | RESOLVED |
| F16 | review-finding:pr-2815-f16 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F17 | review-finding:pr-2815-f17 | Copilot | Minor (inferred) | correctness | ORIGINAL | NO_ACTION | SUPERSEDED |
| F18 | review-finding:pr-2815-f18 | Copilot | Minor (inferred) | correctness | ORIGINAL | NO_ACTION | SUPERSEDED |
| F19 | review-finding:pr-2815-f19 | Copilot | Nit (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F20 | review-finding:pr-2815-f20 | Copilot | Nit (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F21 | review-finding:pr-2815-f21 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The Risks section still describes a second read

- PR number: 2815
- Source review ID: 5848169757
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2815#discussion_r4529600470
- Concern: Risks and Trade-offs said option 1 adds one request and names a second head read, which the implementation does not make.
- Solution: The Risks section now says no request is added and names the remaining window as a push between the feedback reads and the head read.
- Current-tree verification: `grep -n "adds no request" ISSUE.md` finds the new text.
- Resolution reference: test(wait-for-pr-review): address #2815 review: ordinary-case wait test, comment trigger, evidence
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2815#discussion_r4529743008

### F2 - V2 does not name the commit whose build it ran on

- PR number: 2815
- Source review ID: 5848169757
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2815#discussion_r4529600487
- Concern: The final recheck named only the base commit, not the commit subject of the build.
- Solution: V2 names the commit subject of the fix and says the build was of the working tree before it was committed.
- Current-tree verification: `grep -n "read the head after the feedback" manual-verification-evidence.md` finds it.
- Resolution reference: test(wait-for-pr-review): address #2815 review: ordinary-case wait test, comment trigger, evidence
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2815#discussion_r4529743611

### F3 - The ordinary-case test has no feedback in it

- PR number: 2815
- Source review ID: 5848169757
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2815#discussion_r4529600499
- Concern: The snapshot ordinary-case test had empty feedback and passed with or without the fix; AC2 cited it.
- Solution: The snapshot ordinary-case test now has a review of the head, and a wait-level ordinary-case test was added.
- Current-tree verification: `cargo test --lib` passes 67 tests, including both ordinary-case tests.
- Resolution reference: test(wait-for-pr-review): address #2815 review: ordinary-case wait test, comment trigger, evidence
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2815#discussion_r4529744046

### F4 - The fix commit body describes a repeated read that its diff does not make

- PR number: 2815
- Source review ID: 5848169757
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2815#discussion_r4529600507
- Concern: The body of the fix commit says the pull request is read again, while the diff moves the one read.
- Solution: Not changed: the branch is pushed and the repository keeps mistake-and-correction pairs; the correction commit states the right fact. The maintainer may ask for a reword; the thread stays open for that decision.
- Current-tree verification: `git show` of the fix commit shows the moved read; the correction commit is in the branch.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2815#discussion_r4529744413
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2815#discussion_r4529744413

### F5 - The linter criterion stays unchecked while the checkpoint is ticked

- PR number: 2815
- Source review ID: 5848169757
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2815#discussion_r4529600515
- Concern: The checkpoint marked automatic verification complete while the `linter all` criterion was unchecked.
- Solution: The `linter all` criterion is ticked.
- Current-tree verification: `grep -n "linter all" ISSUE.md` shows `[x]`.
- Resolution reference: test(wait-for-pr-review): address #2815 review: ordinary-case wait test, comment trigger, evidence
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2815#discussion_r4529744838

### F6 - "A push after this read" is ambiguous

- PR number: 2815
- Source review ID: 5848169757
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2815#discussion_r4529600522
- Concern: The comment and SKILL.md did not separate a push between the feedback reads and the head read from one after it.
- Solution: Both now separate the cases.
- Current-tree verification: The snapshot.rs comment and the SKILL.md paragraph name both cases.
- Resolution reference: test(wait-for-pr-review): address #2815 review: ordinary-case wait test, comment trigger, evidence
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2815#discussion_r4529745261

### F7 - The comment test repeats the review test interleaving

- PR number: 2815
- Source review ID: 5848169757
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2815#discussion_r4529600529
- Concern: The comment test moved the head on the reviews call, so the comment played no part.
- Solution: The test now triggers on the conversation-comments endpoint.
- Current-tree verification: Run against the old read order, the test fails; against the new, it passes.
- Resolution reference: test(wait-for-pr-review): address #2815 review: ordinary-case wait test, comment trigger, evidence
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2815#discussion_r4529745686

### F8 - The evidence miscounts the tests and omits command output

- PR number: 2815
- Source review ID: 5848169757
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2815#discussion_r4529600536
- Concern: The evidence said three tests were added and gave counts rather than output and toolchain.
- Solution: The evidence states the number of tests added, the toolchain and the red and green result lines.
- Current-tree verification: `grep -n "tests were added" manual-verification-evidence.md`.
- Resolution reference: test(wait-for-pr-review): address #2815 review: ordinary-case wait test, comment trigger, evidence
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2815#discussion_r4529746123

### F9 - The frontmatter status is still planned

- PR number: 2815
- Source review ID: 5848169757
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2815#discussion_r4529600549
- Concern: `status` stayed `planned` and `last-updated-utc` was not bumped.
- Solution: `status` is `in-progress` and `last-updated-utc` was bumped.
- Current-tree verification: `grep -n "^status" ISSUE.md`.
- Resolution reference: test(wait-for-pr-review): address #2815 review: ordinary-case wait test, comment trigger, evidence
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2815#discussion_r4529746555

### F10 - No review audit record at this head

- PR number: 2815
- Source review ID: 5848499555
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2815#discussion_r4529857575
- Concern: The threads were answered and resolved without `docs/pr-reviews/pr-2815-review/PR-REVIEW.md`.
- Solution: This record was added and validated with `validate-audit-record.py`.
- Current-tree verification: `validate-audit-record.py --pr-number 2815` exits 0.
- Resolution reference: docs(pr-reviews): add the #2815 review audit record
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2815#discussion_r4529921637

### F11 - Eleven replies cite branch commits by id

- PR number: 2815
- Source review ID: 5848499555
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2815#discussion_r4529857581
- Concern: Replies cited branch commit ids, which a rebase rewrites.
- Solution: The replies stay as posted; the Commit Id Map below maps each id to its subject, and later replies cite subjects.
- Current-tree verification: The map lists every id the replies cite.
- Resolution reference: docs(pr-reviews): add the #2815 review audit record
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2815#discussion_r4529922028

### F12 - The description still says what the comment now corrects

- PR number: 2815
- Source review ID: 5848499555
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2815#discussion_r4529857589
- Concern: The PR description Summary kept the old wording about a push after the head read.
- Solution: The Summary was restated and two test names were renamed.
- Current-tree verification: `gh pr view 2815 --json body` shows the new Summary.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2815#issuecomment-6778000616
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2815#discussion_r4529922479

### F13 - "the next poll reports it as head_changed" holds only without new feedback

- PR number: 2815
- Source review ID: 5848499555
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2815#discussion_r4529857599
- Concern: SKILL.md said the next poll reports the push as `head_changed`, but new feedback is reported first.
- Solution: The paragraph now says so.
- Current-tree verification: The SKILL.md paragraph names the exception.
- Resolution reference: docs(wait-for-pr-review): say when a later push is reported as head_changed
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2815#discussion_r4529922907

### F14 - The new stamps predate the review they record

- PR number: 2815
- Source review ID: 5848499555
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2815#discussion_r4529857605
- Concern: The spec stamps were written before the events and not from commit times.
- Solution: A correction entry names the wrong stamps with their commit times; `last-updated-utc` holds the edit time.
- Current-tree verification: `git log --format=%cI` times match the correction entry.
- Resolution reference: docs(issues): correct the #2714 stamps that predate the review round
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2815#discussion_r4529923283

### F15 - One commit carries eight independent fixes

- PR number: 2815
- Source review ID: 5848499555
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2815#discussion_r4529857608
- Concern: The previous round put eight fixes in one commit.
- Solution: This round uses one Conventional Commit per finding; the earlier commit is not rewritten.
- Current-tree verification: `git log` shows one commit per finding of this round.
- Resolution reference: test(wait-for-pr-review): name the head-race tests for the fetch order
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2815#discussion_r4529923732

### F16 - The green output block carries an annotation

- PR number: 2815
- Source review ID: 5848499555
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2815#discussion_r4529857614
- Concern: An annotation sat inside a block presented as `cargo test` output.
- Solution: The block is called an excerpt and the annotation is removed.
- Current-tree verification: `grep -n "excerpt" manual-verification-evidence.md`.
- Resolution reference: docs(issues): mark the #2714 green test output as an excerpt
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2815#discussion_r4529924126

### F17 - `replace_after` would panic on a second call

- PR number: 2815
- Source review ID: 5848158391
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2815#discussion_r4529591689
- Concern: Copilot said the replacement queue holds one item and a second call panics.
- Solution: No change: `next()` pops the front only when the queue has more than one item, otherwise it returns a clone, so the response repeats.
- Current-tree verification: The `next()` of `FakeGh` was read, and `cargo test` passes with the helper.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2815#discussion_r4529617884
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2815#discussion_r4529617884

### F18 - `replace_after` would panic on a second call (second thread)

- PR number: 2815
- Source review ID: 5848158391
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2815#discussion_r4529591741
- Concern: The same claim on the doc comment of the helper.
- Solution: Superseded by F17.
- Current-tree verification: Same as F17.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2815#discussion_r4529921310
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2815#discussion_r4529921310

### F19 - "at least as new as any commit" is ambiguous (SKILL.md)

- PR number: 2815
- Source review ID: 5848158391
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2815#discussion_r4529591808
- Concern: Copilot said the wording reads as commit ordering.
- Solution: SKILL.md describes the fetch order.
- Current-tree verification: The paragraph says "fetched after".
- Resolution reference: docs(wait-for-pr-review): state the head ordering in terms of fetch time (#2714)
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2815#discussion_r4529618878

### F20 - "at least as new as any commit" is ambiguous (snapshot.rs)

- PR number: 2815
- Source review ID: 5848158391
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2815#discussion_r4529591859
- Concern: The same wording in the doc comment.
- Solution: The doc comment describes the fetch order.
- Current-tree verification: `snapshot.rs` doc comment says "fetches ... after".
- Resolution reference: docs(wait-for-pr-review): state the head ordering in terms of fetch time (#2714)
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2815#discussion_r4529619335

### F21 - Two replies cite the record's commit, and one says it was validated, before either happened

- PR number: 2815
- Source review ID: 5848499555
- Reviewer finding ID: F17
- Source URL: https://github.com/torrust/torrust-tracker/pull/2815#discussion_r4530138247
- Concern: The replies to F10 and F11 (20:07 UTC) cited the audit commit and said the record was validated, before the commit (20:09:01 UTC) and any passing validator run existed.
- Solution: The replies stay as posted. A Processing Log correction entry records the order of events. The reviewer's ID F17 collided with the Copilot finding F17, so this row has the next audit-local ID.
- Current-tree verification: The commit times come from `git log --format=%cI`; the reply times from the GitHub comments listing.
- Resolution reference: docs(pr-reviews): record the early statements of the F10 and F11 replies
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2815#discussion_r4530147854

## Commit Id Map

Replies cite branch commits by id (finding F11). The ids map to these subjects:

| Id | Subject |
| -- | ------- |
| `0c7141853` | fix(wait-for-pr-review): read the head after the feedback (#2714) |
| `7e9d37d66` | docs(wait-for-pr-review): the head read moves, it does not add a request (#2714) |
| `2e7e9e713` | docs(wait-for-pr-review): state the head ordering in terms of fetch time (#2714) |
| `3a97a8b84` | test(wait-for-pr-review): address #2815 review: ordinary-case wait test, comment trigger, evidence |

## Processing Log

- 2026-11-10 19:36 UTC - Copilot's review (5848158391) arrived; `2e7e9e713` was committed at 19:36 UTC and the replies to its threads were posted after it.
- 2026-11-10 19:40 UTC - The maintainer's first-round threads F1 to F9 were fixed in `3a97a8b84` (commit time); their threads were answered and resolved after it, before this record existed. The first resolve loop also closed the nine threads before they were read; they were reopened, answered and resolved again.
- 2026-11-10 20:01 UTC - The maintainer's second review (5848499555) arrived with F10 to F16 (cursor of the poller result).
- 2026-11-10 20:08 UTC - Fixes for F11 to F16 committed one per finding, this record added, and the replies posted (reply creation times).
- 2026-11-10 20:09 UTC - Correction to the 20:08 entry: this record was committed at 20:09 UTC (commit time of the commit `docs(pr-reviews): add the #2815 review audit record`), not at 20:08; the fixes and the replies were before it. The validator ran with exit 0 after that commit.
- 2026-11-10 20:29 UTC - Correction (finding F21): the replies to F10 and F11, posted at 20:07 UTC, said the record was validated and cited its commit before either existed. The record was committed at 20:09:01 UTC and the validator first passed after that commit; no passing run could come earlier, because the record cites replies posted until 20:08 UTC. The replies stay as posted.

Append entries only. If an entry was rewritten in place, restore the prior entries, then append a
correction that names the rewritten content; do not rewrite the record again.

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- Cite a fix by its unique Conventional Commit subject or durable reply URL, never by a branch SHA that can change after a rebase.
- Refresh review threads using GraphQL and confirm that no unresolved actionable thread remains.
