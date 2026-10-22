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

Review round 1 of the maintainer-requested review. The Copilot review (review 5610834925, a summary
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
- Current-tree verification: `cargo test -p torrust-tracker-udp-server --lib testing::environment`: 13 passed, and 15 repeated runs of the new test passed; with mutation U-M1 (listeners cancelled together with the receive loop) the new test and the staged-order test fail, and with U-M3 it fails too (evidence V7); `grep -n 'cannot be held in flight'` finds only the corrections that name the old claim.
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

## Processing Log

- 2026-10-22 14:53 UTC - Review 5616192898 (`APPROVED`, six inline findings) and the ACK comment 6343775843 were submitted by the requested reviewer for the pushed head; the Copilot review 5610834925 of 07:47 UTC had no inline thread.
- 2026-10-22 17:17 UTC - Started audit: fixed F4, F2, F3, F5, F6, F1 in six signed commits (one per finding) and pushed them; posted one reply per thread (17:17 UTC, GitHub `created_at`), each naming the disposition and the commit subject, after re-deriving its claim from the current tree.
- 2026-10-22 17:18 UTC - Resolved the six threads after the replies and refreshed the GraphQL thread data: zero unresolved threads, all six resolved by the author.

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
