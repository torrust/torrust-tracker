---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/open/2416-2410-si-22-4-stop-producers-before-listeners/ISSUE.md
---

<!-- skill-link: process-pr-review -->

# PR #2623 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2623.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A

## Status Values

- Relationship: `ORIGINAL`, `RE_RAISE_OF:<FindingId>`
- Disposition: `FIXED`, `NO_ACTION`, `SUPERSEDED`, `FOLLOW_UP`
- Thread state: `OPEN`, `RESOLVED`, `NON_RESOLVABLE`, `SUPERSEDED`. `OPEN` applies prospectively
  to audits created or updated for approved follow-up work; historical audits remain valid without
  bulk migration.
- Severity: `Blocker`, `Major`, `Minor`, `Nit`, `Suggestion`; append `(inferred)` when derived
  from free prose.
- Author class: `Copilot`, `Human`, `Unknown`
- Category: `link-integrity`, `formatting`, `metadata`, `testing`, `correctness`,
  `documentation`, `maintainability`, `security`, `other`
- An outdated thread whose concern was fixed is `FIXED`/`RESOLVED`, even when GitHub marks the
  original thread outdated after the push. For in-PR feedback, use `NO_ACTION`/`SUPERSEDED` only
  for a duplicate, superseded, or no-change concern. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work.

## Findings

Review rounds 1 to 9 of the maintainer-requested review. Round 9 (review 5637591133, source of F20 and F21), round 8 (review 5637524522, source of F19), round 7 (review 5633217676, source of F18), round 6 (review 5631998994, source of F17) and round 5 (review 5631621502, source of F15 and F16) are described in the Processing Log. Round 2 (review 5618477521, source of F7 to F9), round 3 (review 5625229542, source of F10 and F11) and round 4 (review 5630959120, source of F12 and F13, and the Copilot review 5631096012, source of F14) are described in the Processing Log. The Copilot review (review 5610834925, a summary
that reports no open issues and has no inline thread) holds no independently actionable assertion and
has no row. The human review 5616192898 (verdict `APPROVED`, an ACK of the pushed head, and the PR
conversation comment 6343775843) holds the six inline findings below; its body and the ACK comment
contain no other request. Finding IDs are the reviewer's own.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2623-f1 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2623-f2 | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2623-f3 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2623-f4 | Human | Nit | testing | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2623-f5 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2623-f6 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2623-f7 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2623-f8 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-2623-f9 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-2623-f10 | Human | Major | maintainability | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-2623-f11 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F12 | review-finding:pr-2623-f12 | Human | Major | documentation | ORIGINAL | FIXED | RESOLVED |
| F13 | review-finding:pr-2623-f13 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F14 | review-finding:pr-2623-f14 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F15 | review-finding:pr-2623-f15 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F16 | review-finding:pr-2623-f16 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F17 | review-finding:pr-2623-f17 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F18 | review-finding:pr-2623-f18 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F19 | review-finding:pr-2623-f19 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F20 | review-finding:pr-2623-f20 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F21 | review-finding:pr-2623-f21 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The T10 row still lists the review, the retrospective and the script decision as pending

- PR number: 2623
- Source review ID: 5616192898
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2623#discussion_r4349862986
- Concern: The T10 row of the issue listed the independent review, the retrospective and the script decision as pending although the progress log records them as done, and the sub-EPIC plan row still read `TODO` while its Subissues row said `IN_PROGRESS`.
- Solution: Set the T10 row in the issue and in the sub-EPIC plan to `DONE`, naming what is left (the maintainer's acceptance review), and add progress-log entries to both records. The Subissues row stays `IN_PROGRESS` until acceptance and merge.
- Current-tree verification: `grep -n '^| T10 '` on `ISSUE.md` and the sub-EPIC `EPIC.md` shows `DONE` in both rows; `linter all` exit 0.
- Resolution reference: `docs(issues): [#2416] mark T10 done in the issue and the sub-EPIC plan`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2623#discussion_r4351330292

### F2 - 'A UDP request cannot be held in flight' is contradicted by the same crate

- PR number: 2623
- Source review ID: 5616192898
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2623#discussion_r4349862999
- Concern: The evidence, record entry 32 and the retrospective said a UDP request cannot be held in flight, so the UDP environment had no deterministic red test; the crate's own `HoldAcceptedRequestsSender` holds accepted requests, so a wrapping sender could give the UDP environment a deterministic red like the HTTP one.
- Solution: Took the first option: a wrapping sender in the UDP environment tests holds a connect request's `UdpRequestAccepted` publication while `stop()` drains the receive loop and forwards it to the real bus after release; the test checks the accepted-request counter after `stop()` returns. The wrong sentences were corrected in the evidence (V4 with the correction marked, and the new V7), record entry 32, the retrospective (with a lesson about searching the crate for an existing hook) and the spec.
- Current-tree verification: `cargo test -p torrust-tracker-udp-server --lib testing::environment`: 13 passed, and 15 repeated runs of the new test passed; with mutation U-M1 (listeners cancelled together with the receive loop) the new test and the staged-order test fail, and with U-M3 it fails too (evidence V7); `grep -rn 'cannot be held in flight' docs packages src` matches only records that quote the old claim in order to correct it (record entry 32, the retrospective, V4's marked correction, and this audit); none states it as true.
- Resolution reference: `test(udp-server): [#2416] hold a UDP request in flight to test the late event when stopping`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2623#discussion_r4351330658

### F3 - An abandoned `stop()` does leave detached listener tasks, cancelled ones

- PR number: 2623
- Source review ID: 5616192898
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2623#discussion_r4349863009
- Concern: The ADR, record entry 32 and an earlier commit body said an abandoned `stop()` cannot leave a detached listener, but the dropped join also drops the listener `JoinHandle`s, so the cancelled listener tasks finish their bounded drain unjoined.
- Solution: Reworded the ADR (section 8), record entry 32, the evidence row and the two abandoned-join test comments to say that the listeners are cancelled but not joined and that none is left that was never asked to stop. The body of the earlier commit that used the old words is history and was not rewritten.
- Current-tree verification: `grep -n 'cancelled but not joined'` in the ADR and the sub-EPIC `EPIC.md`; the environment tests pass (HTTP 11, UDP 13); `linter all` exit 0.
- Resolution reference: `docs(adrs): [#2416] say that an abandoned stop() cancels the listeners without joining them`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2623#discussion_r4351330995

### F4 - The precondition asserts a post-drop snapshot and prints the pre-drop one

- PR number: 2623
- Source review ID: 5616192898
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2623#discussion_r4349863021
- Concern: The UDP drop test took `holders_while_running` before `drop(environment)` but asserted a second measurement taken after it, which holds only because the current-thread runtime cannot run the listeners between the two lines.
- Solution: Assert `holders_while_running`, as the HTTP twin does; the post-drop count is used only by the wait loop.
- Current-tree verification: `grep -n 'holders_while_running.iter()' packages/udp-server/src/testing/environment.rs`; the UDP environment tests pass (13); `linter all` exit 0.
- Resolution reference: `test(udp-server): [#2416] assert the pre-drop listener holders in the UDP drop test`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2623#discussion_r4351331351

### F5 - 'or that comes after `cancel()`' reads as any registration

- PR number: 2623
- Source review ID: 5616192898
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2623#discussion_r4349863035
- Concern: `application-jobs.md` said a registration that comes after `cancel()` is rejected, but only a registration with edges is refused (`ShutdownRequested`); a component without edges can still be spawned and gets a cancelled token, and a self edge is refused as well.
- Solution: Took the reviewer's wording and added that a component without edges can still be spawned after `cancel()`.
- Current-tree verification: `grep -n 'or itself, or that adds edges' docs/architecture/application-jobs.md`; `linter all` exit 0.
- Resolution reference: `docs(architecture): [#2416] say that only registrations with edges are refused after cancel()`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2623#discussion_r4351331643

### F6 - The sub-EPIC reference still promises the diagrams

- PR number: 2623
- Source review ID: 5616192898
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2623#discussion_r4349863045
- Concern: The ADR's reference to the sub-EPIC said it held the diagrams, which moved to `events.md` and `application-jobs.md`.
- Solution: The reference now says the design record holds decisions D1 to D20 and the discussion record, and links the two architecture documents for the diagrams.
- Current-tree verification: `grep -c 'and the diagrams'` on the ADR returns 0; `linter all` exit 0 (local links and fragments resolve).
- Resolution reference: `docs(adrs): [#2416] point the ADR's sub-EPIC reference at the moved diagrams`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2623#discussion_r4351331965

### F7 - 'it gets a cancelled token' holds only for a component that waits for nobody

- PR number: 2623
- Source review ID: 5618477521
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2623#discussion_r4351630249
- Concern: A token is issued at `reserve`; a stop-before target reserved before `cancel()` sits in the shutdown plan's waiting list and keeps an uncancelled token when it is spawned after `cancel()`, so the round-1 wording "a component without edges gets a cancelled token" was too general (the reviewer noted that his own F5 comment carried the same wording).
- Solution: The jobs document now says a component that waits for nobody can still be spawned after `cancel()` with a cancelled token, and that a stop-before target keeps its uncancelled token until the components that stop before it have completed. Swept the documents and `manager.rs` for the same sentence: the `reserve` doc comment is about reservations issued after `cancel()` and is correct.
- Current-tree verification: `grep -rn -i 'cancelled token'` over `docs/architecture`, the ADR, `docs/features`, the spec, the sub-EPIC record and `src/bootstrap`; `linter all` exit 0.
- Resolution reference: `docs(architecture): [#2416] limit the cancelled-token sentence to a component that waits for nobody`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2623#discussion_r4357053019

### F8 - The AC2 evidence cell still lists only 'UDP regression plus staged-join tests'

- PR number: 2623
- Source review ID: 5618477521
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2623#discussion_r4351630258
- Concern: The AC2 cell of the Acceptance Verification table did not cite V7, the deterministic UDP red and green added in round 1.
- Solution: The cell now cites V4 and V7.
- Current-tree verification: `grep -n '^| AC1-AC2' docs/issues/open/2416-2410-si-22-4-stop-producers-before-listeners/ISSUE.md`; `linter all` exit 0.
- Resolution reference: `docs(issues): [#2416] cite the UDP red and green in the AC2 evidence`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2623#discussion_r4357053253

### F9 - V7 names no working-tree state, and its reason for using U-M1 is not borne out

- PR number: 2623
- Source review ID: 5618477521
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2623#discussion_r4351630263
- Concern: V7 did not say which tree it ran on, and said the tree before the T9 change "does not have the wrapper hooks the test needs", which is wrong: the wrapper and its helper need only the container fields and `stop()`.
- Solution: V7 names its working-tree state, says that mutation U-M1 reproduces the one-token order exactly and that the test was not run on the earlier tree, and marks the correction. A search for the old sentence finds no other record.
- Current-tree verification: `grep -rn 'wrapper hooks' docs packages src` matches only records that quote the old sentence in order to correct it (the marked correction in V7, the progress-log entry of the correction, and this audit entry); none states it as true; `linter all` exit 0.
- Resolution reference: `docs(issues): [#2416] name the tree of the UDP red run and give the true reason for mutation U-M1`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2623#discussion_r4357053539

### F10 - The branch now conflicts with `develop` in the ADR index, so it must be updated before merge

- PR number: 2623
- Source review ID: 5625229542
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2623#discussion_r4357175882
- Concern: `develop` appended the 20261021190318 row after the same 20261016154414 line this branch's 20261021184946 row follows, so `git merge-tree` of the head against `develop` stops on a content conflict in `docs/adrs/index.md`; the `open-pull-request` skill requires an update when the branch conflicts. The PR body's statement that a trial merge is clean no longer held.
- Solution: Merged `develop` into the branch with one signed merge commit, keeping both rows in timestamp order (this branch's first), instead of rebasing 21 signed commits (the count on the old base when the update was chosen; the figure of 23 that this entry gave in the round-3 audit commit was the count after the merge commit and that audit commit were added): the ADR that stopped requiring rebases allows 'otherwise updated' and a merge keeps the ids the review records cite. Corrected the PR body.
- Current-tree verification: `git merge-tree --write-tree HEAD torrust/develop` exits 0 after the merge; `grep -n '20261016154414\|20261021184946\|20261021190318' docs/adrs/index.md` shows the three rows in order; no conflict markers; `linter all` exit 0.
- Resolution reference: `chore(merge): [#2416] update the branch with develop and keep both ADR index rows`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2623#discussion_r4361329351

### F11 - 'the answered-request regression tests' counts one test as two

- PR number: 2623
- Source review ID: 5625229542
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2623#discussion_r4357175892
- Concern: Only the UDP environment has an answered-request test; the HTTP environment has the in-flight test and no answered-request one, so the plural in the AC2 cell was wrong.
- Solution: The cell now says 'the UDP answered-request regression test'.
- Current-tree verification: `grep -n 'UDP answered-request' docs/issues/open/2416-2410-si-22-4-stop-producers-before-listeners/ISSUE.md`; `grep -n 'fn it_should_count_the_statistics_event' packages/*/src/testing/environment.rs` lists one answered-request test (UDP) and the in-flight tests; `linter all` exit 0.
- Resolution reference: `docs(issues): [#2416] name the UDP answered-request test in the singular in the AC2 evidence`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2623#discussion_r4361329680

### F12 - The PR description still makes the two claims the records corrected, and the merge commit would carry them

- PR number: 2623
- Source review ID: 5630959120
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2623#discussion_r4361533891
- Concern: The PR body still said that a UDP request cannot be held in flight and that an abandoned `stop()` cannot leave detached listeners, which rounds 1 and 2 had corrected in the records; the merge tool copies the whole body into the merge commit, which cannot be repaired after merge.
- Solution: Edited the PR body (the fix is outside the repository tree, so the resolution reference is the PR conversation response that states it): the two sentences now say what the tree has, the evidence reference reads V1 to V7, and the Notes paragraph describes both merge commits. Read the whole body for the same mistake rather than only the two sentences.
- Current-tree verification: `grep -c 'cannot be held in flight\|cannot leave detached'` on `gh pr view 2623 --json body` returns 0; the body contains one closing keyword, `Closes #2416`; the other body claims were read against the branch.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2623#issuecomment-6369658764
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2623#discussion_r4361841049

### F13 - 'instead of rebasing 23 signed commits' counts the merge and the audit commit

- PR number: 2623
- Source review ID: 5630959120
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2623#discussion_r4361533896
- Concern: The audit's F10 solution said 23 signed commits, which counts the merge commit and the round-3 audit commit; the branch held 21 commits on the old base when the update was chosen.
- Solution: The audit now says 21 and notes what the 23 counted.
- Current-tree verification: `git rev-list --count a843009af..':/name the UDP answered-request test in the singular'` (the last commit on the old base, found by its subject) prints 21; `grep -n 'rebasing 21' docs/pr-reviews/pr-2623-review/PR-REVIEW.md`; `linter all` exit 0.
- Resolution reference: `docs(pr-reviews): [#2416] state the commit count of the branch when the update was chosen`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2623#discussion_r4361841393

### F14 - Preserve the four recheck logs in the issue's `evidence/` folder and link them

- PR number: 2623
- Source review ID: 5631096012
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2623#discussion_r4361636784
- Concern: V5's claims about lag, warnings, failures, and aborts depended on logs stored only under the git-ignored `.tmp/`, against the evidence template's rule on preserving artifacts, so later reviewers could not audit them from the repository.
- Solution: Retained the logs of the four M2 and M3 runs, the failed start, and the M1 burst run in `evidence/` with the per-run summaries (the colour-stripped logs minus only the per-request `request{` lines, by a stated command; V8 records their hashes) instead of rerunning, because the originals were available. While doing so I found that my earlier no-warning filter also dropped the `request{` lines, so I rechecked the claims on the full logs with no line excluded and recorded the counts in V5.
- Current-tree verification: `ls docs/issues/open/2416-2410-si-22-4-stop-producers-before-listeners/evidence/`; `grep -c 'request{'` on each retained log is 0; `sha256sum` of the retained logs matches V8; the full-log counts are 0 in V5; `linter all` exit 0.
- Resolution reference: `docs(issues): [#2416] retain the M1 to M3 recheck logs in the evidence folder`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2623#discussion_r4361841750

### F15 - The added clause attributes the count of 23 to the reviewer

- PR number: 2623
- Source review ID: 5631621502
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2623#discussion_r4362046602
- Concern: The F13 correction to the F10 solution said the 23 had been counted by the reviewer, but 23 was that entry's own wording, committed in the round-3 audit commit, and the review that pointed out what it counted came after.
- Solution: The F10 solution now says that the figure of 23 was the entry's own, taken after the merge commit and the round-3 audit commit were added, and names no person. Swept the audit and my replies for the same attribution: my reply on the F13 thread said the same and was edited; nothing else names a person for that figure.
- Current-tree verification: `git grep -n -E 'you count[e]d' -- docs` prints nothing (the bracket keeps the pattern from matching this line); a search written the same way for the earlier phrase that named a person for the figure prints nothing either, and the phrase is not quoted here so that this entry names no one; the F13 thread reply read back from GitHub no longer attributes the count to the reviewer; `linter all` exit 0.
- Resolution reference: `docs(pr-reviews): [#2416] attribute the figure of 23 to the audit entry, not to a reviewer`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2623#discussion_r4362201409

### F16 - The PR description cites this file as V1 to V7, but V8 now exists

- PR number: 2623
- Source review ID: 5631621502
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2623#discussion_r4362046612
- Concern: The PR body's evidence reference read 'V1 to V7' although V8 (the retained recheck logs) had been added to the evidence file after the body was last edited; the merge tool copies the body into the merge commit.
- Solution: Edited the PR body (the fix is outside the repository tree, so the resolution reference is the PR conversation response that states it) to read V1 to V8 and to say what V8 holds; read the rest of the body for other references the retention could have made stale and found none.
- Current-tree verification: `gh pr view 2623 --json body` contains 'V1 to V8' and no 'V1 to V7'; the body contains one closing keyword, `Closes #2416`.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2623#issuecomment-6370391654
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2623#discussion_r4362201670

### F17 - The verification says the grep finds nothing, but it finds its own line

- PR number: 2623
- Source review ID: 5631998994
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2623#discussion_r4362339577
- Concern: The F15 verification claimed that a search found nothing, but the search quoted its own patterns, so it matched the entry that quoted them, and one of those patterns named a person again.
- Solution: The F15 verification now quotes only a bracketed pattern that cannot match its own line, and describes the second search without quoting it. The same flaw existed in the verification lines of F2 and F9, which quoted the searched text; both were rewritten the same way (swept the other entries, none else claims a search finds nothing).
- Current-tree verification: `git grep -n -E 'you count[e]d' -- docs` prints nothing; the F2 and F9 verification lines were re-run as written and print what they say; `linter all` exit 0 and the audit validator exit 0.
- Resolution reference: `docs(pr-reviews): [#2416] make the audit's verification greps not match their own text`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2623#discussion_r4362553408

### F18 - 'Reviewer finding ID: F17' differs from the other sixteen entries

- PR number: 2623
- Source review ID: 5633217676
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2623#discussion_r4363271845
- Concern: The F17 detail entry recorded the reviewer's own ID as the Reviewer finding ID, while the skill says to record the original ID only when it was reassigned and the other sixteen entries use N/A.
- Solution: Changed the field of the F17 entry to N/A. Swept the other entries for the same field: none else records an ID.
- Current-tree verification: `grep -n '^- Reviewer finding ID:' docs/pr-reviews/pr-2623-review/PR-REVIEW.md | grep -vc 'N/A'` prints 0 (the pattern is anchored to the start of a field line, so the headings and verification lines that quote the field do not match it); the audit validator exit 0; `linter all` exit 0.
- Resolution reference: `docs(pr-reviews): [#2416] record N/A as the reviewer finding ID of the F17 entry`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2623#discussion_r4366495235

### F19 - The F18 pipeline prints 1 at this head, not 0

- PR number: 2623
- Source review ID: 5637524522
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2623#discussion_r4366533111
- Concern: The F18 verification said an unanchored search for the field name, filtered for N/A, prints 0, but the F18 heading that the same commit added quotes the old field and matches it, so the count was 0 at the pushed fix commit and 1 at the head that carried the entry.
- Solution: Anchored the F18 pattern to the start of a field line, so headings and verification lines that quote the field do not match, and measured it after writing the line. Swept the other quoted searches that state a count or an empty result: those of F6, F12 and F14 run against the ADR, the PR body and the retained logs, none of which contains the audit; F2 and F9 list the records they match.
- Current-tree verification: `grep -n '^- Reviewer finding ID:' docs/pr-reviews/pr-2623-review/PR-REVIEW.md | grep -vc 'N/A'` prints 0 with this entry written; the audit validator exit 0; `linter all` exit 0.
- Resolution reference: `docs(pr-reviews): [#2416] anchor the F18 verification so it does not match its own entry`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2623#discussion_r4366544198

### F20 - The review was submitted at 08:29, not 08:30

- PR number: 2623
- Source review ID: 5637591133
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2623#discussion_r4366586480
- Concern: The round 8 Processing Log entry gave 08:30 UTC as the submission time of review 5637524522; GitHub records 08:29:05.
- Solution: Appended a correction entry to the Processing Log that names the round 8 entry and gives 08:29; the entry itself is not rewritten, as the log rules require. Compared the stated submission time of every other round in the audit with `submitted_at` of its review.
- Current-tree verification: `gh api repos/torrust/torrust-tracker/pulls/2623/reviews` gives `submitted_at` 2026-10-24T08:29:05Z for review 5637524522; the review times stated in the other entries that give one agree with their reviews' `submitted_at` to the minute (compared by reading the two lists side by side, not by a script); the audit validator exit 0.
- Resolution reference: `docs(pr-reviews): [#2416] correct the round 8 log entry's review time and edit count`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2623#discussion_r4366603197

### F21 - 'Edited twice' and the reply's 'Edited once' disagree

- PR number: 2623
- Source review ID: 5637591133
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2623#discussion_r4366586487
- Concern: The round 8 log entry said the F19 reply was edited twice and that the reply states it, while the reply said 'Edited once'; the REST data carries no edit history.
- Solution: The sequence of edits comes from my own calls: two edits at about 08:33 to 08:34 UTC and a third at 08:52 UTC that replaced the note with a pointer to the log. The appended correction states three edits, and the reply no longer counts its own edits, so the two agree.
- Current-tree verification: the reply as read back from GitHub carries no count of its edits and points to the Processing Log; its `updated_at` is 2026-10-24T08:52:36Z; the correction entry gives the same time; the audit validator exit 0.
- Resolution reference: `docs(pr-reviews): [#2416] correct the round 8 log entry's review time and edit count`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2623#discussion_r4366603312

## Processing Log

- 2026-10-22 14:53 UTC - Review 5616192898 (`APPROVED`, six inline findings) and the ACK comment 6343775843 were submitted by the requested reviewer for the pushed head; the Copilot review 5610834925 of 07:47 UTC had no inline thread.
- 2026-10-22 17:17 UTC - Started audit: fixed F4, F2, F3, F5, F6, F1 in six signed commits (one per finding) and pushed them; posted one reply per thread (17:17 UTC, GitHub `created_at`), each naming the disposition and the commit subject, after re-deriving its claim from the current tree.
- 2026-10-22 17:18 UTC - Resolved the six threads after the replies and refreshed the GraphQL thread data: zero unresolved threads, all six resolved by the author.
- 2026-10-23 07:13 UTC - Review round 2: review 5618477521 (`APPROVED`, an ACK of the head after round 1, and three nits F7 to F9) was submitted 2026-10-22 17:47 UTC. Fixed F7, F8, F9 in three signed commits and pushed them; replied to each thread at 07:12 UTC (GitHub `created_at`) after re-deriving its claim from the current tree, then resolved the three threads and refreshed the GraphQL data: zero unresolved threads.
- 2026-10-23 15:39 UTC - Review round 3: review 5625229542 (`APPROVED` at the head after round 2, with one Major, not blocking, and one nit, F10 and F11) was submitted 2026-10-23 07:30 UTC. Fixed F11 in a signed commit and updated the branch with `develop` for F10 in a signed merge commit (a merge, not a rebase, so the cited commit ids stay valid), pushed both, and corrected the PR body; replied to each thread at 15:39 UTC (GitHub `created_at`) after re-deriving its claim from the current tree, then resolved the two threads and refreshed the GraphQL data: zero unresolved threads.
- 2026-10-23 16:29 UTC - Review round 4: review 5630959120 (`CHANGES_REQUESTED`, submitted 15:59 UTC, F12 Major blocking and F13 Nit) and the Copilot review 5631096012 (submitted 16:10 UTC, one inline finding, F14) arrived after round 3. At the maintainer's request the branch was also updated with the latest `develop` (GitHub reported it out of date; a clean signed merge commit). Fixed F14 and F13 in signed commits, fixed F12 by editing the PR body and posted the conversation response that states it, pushed the branch (replies after the push), replied to the three threads at 16:28 UTC (GitHub `created_at`) after re-deriving each claim, then resolved them and refreshed the GraphQL data: zero unresolved threads. A first version of the F12 response overstated what had been checked and was corrected in place, with a note in the comment.
- 2026-10-23 16:32 UTC - Correction of the F13 detail entry above: its Current-tree verification cited a branch commit by id, which the repository rules forbid (an ids check printed it after the round-4 push); it now names the commit by its subject. The reply on the F13 thread cited the same id and was edited the same way. No other record cites a branch commit id.
- 2026-10-23 17:07 UTC - Review round 5: review 5631621502 (`APPROVED`, submitted 2026-10-23 16:49 UTC, an ACK of the head after round 4, with two nits F15 and F16) was processed. Fixed F15 in a signed commit and pushed it, edited the PR body for F16 and posted the conversation response that states it; replied to the two threads at 17:06 UTC (GitHub `created_at`), after the push, then resolved them and refreshed the GraphQL data: zero unresolved threads. The reply on the F13 thread that attributed the figure of 23 to the reviewer was edited in the same round. The branch is 2 commits behind `develop` (a dependency update that merges cleanly); it was not updated again because no rule requires it and each update gives the reviewer a new head.
- 2026-10-23 17:50 UTC - Review round 6: review 5631998994 (`APPROVED`, submitted 2026-10-23 17:22 UTC, an ACK of the head after round 5, with one finding, F17) was processed. Fixed F17, and the same flaw in the F2 and F9 verification lines, in a signed commit and pushed it; replied to the thread at 17:46 UTC (GitHub `created_at`), after the push, then resolved it and refreshed the GraphQL data: zero unresolved threads.
- 2026-10-23 17:50 UTC - Correction of the model stamps in the progress logs of the spec and of the sub-EPIC: their entries name the author model as Claude Opus 5.5, copied from earlier entries without checking it, and the maintainer states that this session ran Claude Sonnet 5.5. The model of this session was not verifiable from inside it, so the stamps are unverified and the correction is appended to those logs as well. Those logs are not rewritten.
- 2026-10-24 08:13 UTC - Review round 7: review 5633217676 (`APPROVED`, submitted 2026-10-23 19:06 UTC, an ACK of the head after the round 6 audit and the retrospective, with one nit, F18) was processed. Fixed F18 in a signed commit and pushed it (2026-10-24 08:12 UTC); replied to the thread at 08:12 UTC (GitHub `created_at`), after the push, then resolved it: zero unresolved threads. The reviewer's account has no other finding on the retrospective, whose figures he recomputed.
- 2026-10-24 08:34 UTC - Review round 8: review 5637524522 (`APPROVED`, submitted 2026-10-24 08:30 UTC, an ACK of the head after round 7, with one finding, F19) was processed. Fixed F19 in a signed commit and pushed it (08:33 UTC); replied to the thread at 08:33 UTC (GitHub `created_at`), after the push, then resolved it: zero unresolved threads. The reply was edited twice, the last edit at 08:34 UTC (GitHub `updated_at`), to correct its sweep claim (it first said the other searches state no count, and named F3 for the ADR search, which is F6's); the edit is stated in the reply.
- 2026-10-24 08:52 UTC - Correction of the round 8 entry above, which is not rewritten. Review 5637524522 was submitted at 2026-10-24 08:29 UTC (GitHub `submitted_at` 08:29:05), not 08:30. The reply on the F19 thread was edited three times, not two: two edits at about 08:33 to 08:34 UTC (the last has `updated_at` 08:34:06), which corrected its sweep claim and the finding named for the ADR search, and a third at 08:52 UTC (`updated_at` 08:52:36) that replaced its note 'Edited once' with a pointer to this log. The entry's 'last edit at 08:34 UTC' held until that third edit. The review times of the other rounds in this log were re-compared with GitHub `submitted_at` for each review and agree to the minute.
- 2026-10-24 08:56 UTC - Review round 9: review 5637591133 (`APPROVED`, submitted 2026-10-24 08:50 UTC, `submitted_at` 08:50:21, an ACK of the head after round 8, with two nits, F20 and F21) was processed. Fixed both in one signed commit that appends a correction to this log, and pushed it (08:55 UTC); replied to the two threads at 08:56 UTC (GitHub `created_at`), after the push, then resolved them: zero unresolved threads.

Append entries only. If an entry was rewritten in place, restore the prior entries, then append a
correction that names the rewritten content; do not rewrite the record again.

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- For an outdated thread whose concern was fixed, record `Disposition=FIXED` and
  `Thread state=RESOLVED`, even if GitHub marks the thread outdated after the push. For a
  duplicate, superseded, or no-change in-PR thread, reply exactly
  `Superseded by <FindingId>: <reason>.`, record `Disposition=NO_ACTION` and
  `Thread state=SUPERSEDED`, then resolve it. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work.
- A consolidated PR conversation response may cover multiple review rounds only when it names
  every review ID and every finding ID with its disposition and resolution reference. Record its
  durable URL in each related row.
- Cite a fix by its unique Conventional Commit subject or durable reply URL, never by a branch SHA
  that can change after a rebase.
- Refresh review threads using GraphQL and confirm that no unresolved actionable thread remains.
