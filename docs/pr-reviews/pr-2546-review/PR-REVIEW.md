---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/open/1349-1347-add-tests-http-core/ISSUE.md
---

<!-- skill-link: process-pr-review -->

# PR #2546 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2546>.

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

PR #2546 is the spec-only PR for #1349. Two reviews carry findings; the PR was open when both were
processed, so this is the normal pre-merge workflow.

- Copilot's review 5539689636 (COMMENTED, 2026-10-15 20:46:27 UTC) has two inline findings,
  recorded as F5 and F6. Copilot gives them no ID, and F1-F4 were already taken by the other
  review's own IDs, so they take the next audit-local ordinals. Its overview marks both "Medium
  severity", recorded as `Minor (inferred)`. Its warning that the full agentic review did not start
  and its change summary are non-actionable and add no row.
- da2ce7's review 5543910487 (state `DISMISSED` when fetched, submitted 2026-10-16 06:36:17 UTC)
  carries four inline findings with their own IDs, F1-F4. Its body raises nothing further to act
  on: it reports checks with no finding, and it states that the two Copilot comments were declined.
  The threads for F5 and F6 were still unresolved, and the concerns were small and cheap to fix,
  so both were fixed rather than declined; each row says so.
- da2ce7's second review 5544329988 (APPROVED, 2026-10-16 07:23:42 UTC, at the head after the
  first round's fixes) carries two inline `Nit` findings with the IDs F1 and F2, which collide
  with this audit's F1 and F2, so they are recorded as F7 and F8 with the original ID kept in each
  detail entry. Its body raises nothing further to act on.
- da2ce7's third review 5544488533 (APPROVED, 2026-10-16 07:41:21 UTC, at the head after the
  second round's fixes) carries one inline `Nit` finding with the ID F1, which collides with this
  audit's F1, so it is recorded as F9 with the original ID kept in its detail entry. Its body
  raises nothing further to act on.
- da2ce7's fourth review 5545066382 (APPROVED, 2026-10-16 08:32:31 UTC, at the head after the
  third round's fixes) has no inline finding and adds no row; its ACK comment names that head.
- Copilot's second review 5545078410 (COMMENTED, 2026-10-16 08:33:35 UTC) has one inline finding,
  F10. It gives no severity, so it is recorded as `Nit (inferred)`. Its overview marks the two
  round-1 findings resolved.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2546-f1` | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2546-f2` | Human | Suggestion | testing | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2546-f3` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2546-f4` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | `review-finding:pr-2546-f5` | Copilot | Minor (inferred) | metadata | ORIGINAL | FIXED | RESOLVED |
| F6 | `review-finding:pr-2546-f6` | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | `review-finding:pr-2546-f7` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | `review-finding:pr-2546-f8` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F9 | `review-finding:pr-2546-f9` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F10 | `review-finding:pr-2546-f10` | Copilot | Nit (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The progress-log entries and stamps predate the events they record

- PR number: 2546
- Source review ID: 5543910487
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2546#discussion_r4291608051>
- Concern: the spec's 16:55 UTC entry and `last-updated-utc` record opening the PR, but the PR was
  created at 20:30:24Z, 3 h 35 min later. The 16:30 and 16:40 entries and the EPIC's 16:30 entry
  and stamp also precede the first commit by about four hours, so the merged record would state
  times the events did not have.
- Solution: re-stamp every entry from sources outside the file: the session event log (spec file
  created 2026-10-15 16:17:56Z, EPIC edited 16:18:12Z, approval answered 20:26:56Z), the issue
  update (20:27:44Z), the PR creation (20:30:24Z) and the branch reflog. The spec entries are now
  16:18, 20:27 and 20:30 UTC. While fixing F5 and F6 I had also guessed two stamps (06:20 and
  06:30); the real clock gave 06:53, which is now the EPIC entry and stamp.
- Current-tree verification: at the head `docs(pr-reviews): [#2546] record the review audit for the
  #1349 spec PR`, `grep -nE '^- 2026-10-1[56] [0-9:]+ UTC'` on the spec lists the entries 16:18,
  20:27, 20:30 and 07:04 (the review-round entry), and on the EPIC the three earlier #1348 entries
  from 2026-10-15 (07:23, 11:56, 15:15), then 16:18 and 06:53. The spec's `last-updated-utc` is
  `2026-10-16 07:04` and the EPIC's is `2026-10-16 06:53`, each equal to its file's latest entry.
- Resolution reference: docs(issues): [#1349] re-stamp the progress-log entries from the real event times
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2546#discussion_r4291771951>

### F2 - Two udp-core testing lessons that apply here are not carried

- PR number: 2546
- Source review ID: 5543910487
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2546#discussion_r4291608060>
- Concern: the udp-core `lessons.md` records that `tracker-core` exports `#[automock]` store
  doubles, and that an expectation-free double proves an ordering decision. This package owns such
  decisions (failure reasons, and the order of authentication, authorization and client-IP
  resolution), yet the risk line said service tests "need real `tracker-core` collaborators".
- Solution: qualify the risk line with the exported doubles, and name both lessons in the T2 task,
  which creates the shared plan guidance. Verified first that the doubles exist: `#[automock]` on
  the keys, whitelist, torrent-metrics and schema store traits under
  `packages/tracker-core/src/databases/traits/`.
- Current-tree verification: `grep -rn automock packages/tracker-core/src/databases/traits/`
  lists the four trait files; the spec's Risks section and the T2 row both name the doubles and
  the ordering lesson.
- Resolution reference: docs(issues): [#1349] carry the udp-core store-double and ordering lessons
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2546#discussion_r4291772067>

### F3 - The slice branches leave the reserved implementation branch unused, undocumented

- PR number: 2546
- Source review ID: 5543910487
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2546#discussion_r4291608063>
- Concern: `create-issue` reserves `1349-1347-add-tests-http-core` for the implementation after
  the `-spec` branch, but the six slice branches use suffixed names and the spec did not say that
  this departs from the skill.
- Solution: took the reviewer's first option. The Branches rule now states that the slice
  branches replace the reserved name on purpose, because the skill assumes one implementation PR,
  and that the base name stays unused rather than naming S1, which would suggest it carries the
  whole implementation.
- Current-tree verification: the Branches bullet in the spec's Pull-Request Slices section
  contains "replace that reserved name on purpose" (`grep -n`).
- Resolution reference: docs(issues): [#1349] record that the slice branches replace the reserved branch
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2546#discussion_r4291772233>

### F4 - The S0 row prescribes one commit and the PR delivers two

- PR number: 2546
- Source review ID: 5543910487
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2546#discussion_r4291608068>
- Concern: the S0 Commit Points row asked for one `docs(issues)` commit, but the spec records its
  own PR number, which exists only once the PR is open, so a second commit was unavoidable.
- Solution: reworded the row instead of folding the commit away: one signed `docs(issues)`
  commit, a follow-up commit that records the PR number, and any review-fix commits.
- Current-tree verification: the Commit Points table's S0 row contains "plus a follow-up signed
  commit that records the spec-only PR number" (`grep -n`).
- Resolution reference: docs(issues): [#1349] describe the S0 commit policy as delivered
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2546#discussion_r4291772408>

### F5 - The frontmatter `status` is `planned` while the EPIC marks the subissue `IN_PROGRESS`

- PR number: 2546
- Source review ID: 5539689636
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2546#discussion_r4288104643>
- Concern: the spec frontmatter says `planned`, while the EPIC row said `IN_PROGRESS`; align them,
  for example to `in_progress`.
- Solution: fixed with a different resolution than suggested. `in_progress` is not a valid value
  (the validator allows `draft`, `planned`, `in-progress`, `blocked`, `in-review`, `done`), and
  #1348's spec kept `planned` through its whole spec-only PR and moved to `in-progress` when T1
  started. T1 is `TODO` and every ledger row is `PENDING`, so `planned` is right and the EPIC row
  was the inconsistency. The row is now `TODO`, and the spec documents when both change. The
  other reviewer's body also declined this comment on the same ground; the EPIC row was still
  fixed because it contradicted the spec.
- Current-tree verification: the spec has `status: planned`; the EPIC's #1349 row reads `TODO`
  with "Becomes `IN_PROGRESS` when T1 starts"; the spec states the rule (`grep -n`).
- Resolution reference: docs(issues): [#1349] keep the subissue status planned until T1 starts
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2546#discussion_r4291771657>

### F6 - The approval checkpoint keeps the template's "user/maintainer" wording

- PR number: 2546
- Source review ID: 5539689636
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2546#discussion_r4288104720>
- Concern: the checkpoint reads "approved by user/maintainer", which is leftover template wording
  while the progress log names the approver.
- Solution: the checkpoint now names the approver and the date. The other reviewer's body noted
  that the wording is the template's own and declined the comment; it was fixed anyway because the
  named approver matches the durable log entry at no cost.
- Current-tree verification: the checkpoint reads "approved by the maintainer, Jose Celano
  (2026-10-15)" (`grep -n`), matching the 20:27 UTC log entry.
- Resolution reference: docs(issues): [#1349] name the approver in the spec approval checkpoint
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2546#discussion_r4291771805>

### F7 - The F1 current-tree line describes the tree before the audit's own commit

- PR number: 2546
- Source review ID: 5544329988
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2546#discussion_r4291938615>
- Concern: the F1 `Current-tree verification` line reported the grep result before the audit
  commit. At the head the spec's stamp and entries, and the EPIC entries the grep also lists, differ,
  although the conclusion still stands.
- Solution: the line now states the grep result at the audit commit, named by its subject: the
  spec entries 16:18, 20:27, 20:30 and 07:04 with stamp 07:04, and the EPIC entries with stamp
  06:53.
- Current-tree verification: re-ran `grep -nE '^- 2026-10-1[56] [0-9:]+ UTC'` on the spec and the
  EPIC at that head and compared with the F1 entry; they agree.
- Resolution reference: docs(pr-reviews): [#2546] state F1's verification result at the audit commit
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2546#discussion_r4291964811>

### F8 - The S0 row's commit policy does not cover the review-audit commit

- PR number: 2546
- Source review ID: 5544329988
- Reviewer finding ID: F2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2546#discussion_r4291938624>
- Concern: the S0 Commit Points row lists the spec commit, the PR-number follow-up and review
  fixes, but the audit commit, which `process-pr-review` requires on the PR branch separately from
  the fixes, is none of those, and every later slice PR will carry one too.
- Solution: the S0 row now names the audit commit and lists the audit in its change set, and a new
  "Review round (every PR, including this one)" row covers each later slice PR.
- Current-tree verification: the spec's Commit Points table has both rows (`grep -n "Review
  round"` and the S0 row), and the audit file is the one this record describes.
- Resolution reference: docs(issues): [#1349] cover the review-audit commit in the commit points
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2546#discussion_r4291965087>

### F9 - The S0 row calls the review round row the "next row", but the next row is T1

- PR number: 2546
- Source review ID: 5544488533
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2546#discussion_r4292065484>
- Concern: the S0 Commit Points row points to the review-audit commit row as "next row", but the
  row after S0 is T1; the "Review round" row sits after T2-T3, T4 and Slice close.
- Solution: the S0 row now names the row ("see the `Review round` row below") instead of relying
  on its position.
- Current-tree verification: the S0 row of the spec's Commit Points table contains `see the "Review
  round" row below` (`grep -n`); its only remaining "next row" is this round's progress-log entry.
- Resolution reference: docs(issues): [#1349] name the review round row instead of calling it the next row
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2546#discussion_r4292332164>

### F10 - The PR description says the EPIC row was set to `IN_PROGRESS`, but the diff keeps `TODO`

- PR number: 2546
- Source review ID: 5545078410
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2546#discussion_r4292515318>
- Concern: the PR description's Files section said the EPIC #1349 row was set to `IN_PROGRESS`,
  but review round 1 changed the row to `TODO`, so the description no longer matched the diff.
- Solution: edited the PR description. Its Files section now says the row stays `TODO` until T1
  starts and was `IN_PROGRESS` only in the first commit, and it also lists this audit file, which
  the description had omitted. No repository file changed.
- Current-tree verification: `git diff --name-only torrust/develop...HEAD` lists the spec, the
  EPIC and this audit, and the EPIC row reads `TODO`; the edited description lines match both, and
  the closing-keyword scan of the new body printed nothing.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2546#issuecomment-6211505974>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2546#discussion_r4292530053>

## Processing Log

- 2026-10-16 06:16 UTC - Started audit; fetched the threads and reviews (Copilot's two threads).
- 2026-10-16 06:53 UTC - Committed the F6 and F5 fixes after a GPG passphrase timeout was retried
  with the maintainer's approval.
- 2026-10-16 06:55 UTC - Re-fetched and found da2ce7's review 5543910487 with F1-F4, submitted
  at 06:36:17 UTC.
- 2026-10-16 07:00 UTC - Committed the F1-F4 fixes and pushed all six.
- 2026-10-16 07:02 UTC - Replied on the six threads and resolved them.
- 2026-10-16 07:04 UTC - Recorded this audit and ran the validator and a final thread refresh.
- 2026-10-16 07:24 UTC - da2ce7 approved the head (review 5544329988) with two new `Nit` threads,
  F7 and F8; fetched them.
- 2026-10-16 07:26 UTC - Committed the F7 and F8 fixes and pushed.
- 2026-10-16 07:28 UTC - Replied on the two threads and resolved them; recorded them here.
- 2026-10-16 08:12 UTC - da2ce7 approved the head (review 5544488533) with one new `Nit` thread,
  F9; fetched it and committed the fix.
- 2026-10-16 08:14 UTC - Pushed, replied on the thread and resolved it; recorded it here.
- 2026-10-16 08:17 UTC - Re-requested review from da2ce7 and Copilot after the push.
- 2026-10-16 08:34 UTC - Found da2ce7's approval (review 5545066382, no finding) and Copilot's
  review 5545078410 with one new thread, F10; edited the PR description.
- 2026-10-16 08:35 UTC - Posted the conversation response, replied on the thread and resolved it.

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
