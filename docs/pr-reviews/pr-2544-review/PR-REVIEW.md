---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/open/2471-1488-fix-http-environment-drop-path/ISSUE.md
---

<!-- skill-link: process-pr-review -->

# PR #2544 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2544>.

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

Three reviews were submitted. Copilot's overview review 5537667869 reports no findings and has no
inline comment, so it adds no row. Copilot's review 5537902749 carries F1-F4 as inline threads; its
overview warns that the full agentic review did not start before a timeout and only lists those
four, so the body adds no rows. F1 and F2 are the same concern at two lines, so F2 is recorded as
`RE_RAISE_OF:F1`. da2ce7's review 5537908774 (changes requested) carries five inline threads whose
reviewer IDs F1-F5 collide with F1-F5 above, so they are recorded as F5-F9 with the reviewer's ID
kept in the detail entry. Its body is a round summary; its assertions that the bot's "restoring
cleanup" and "marks the fix complete" claims deserve findings repeat reviewer F1 and F2 (audit F5
and F6), so the body adds no rows. The reviews read the head whose subject is
"docs(issues): [#1488] mark #2471 done in the shutdown roadmap".

A second da2ce7 review, 5539246048 (approved), read the head whose subject is "docs(pr-reviews):
add the PR #2544 review audit" and left six non-blocking inline threads whose reviewer IDs F1-F6
again collide with existing IDs, so they are recorded as F10-F15 with the reviewer's ID kept in
the detail entry. Its body is a round summary that repeats those six findings and confirms the nine
earlier ones as resolved, so it adds no rows; a separate PR conversation comment, `ACK`, records
its verdict.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2544-f1 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2544-f2 | Copilot | Minor (inferred) | documentation | RE_RAISE_OF:F1 | FIXED | RESOLVED |
| F3 | review-finding:pr-2544-f3 | Copilot | Nit (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2544-f4 | Copilot | Suggestion (inferred) | maintainability | ORIGINAL | NO_ACTION | RESOLVED |
| F5 | review-finding:pr-2544-f5 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2544-f6 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2544-f7 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2544-f8 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-2544-f9 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-2544-f10 | Human | Minor | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-2544-f11 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F12 | review-finding:pr-2544-f12 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F13 | review-finding:pr-2544-f13 | Human | Nit | correctness | ORIGINAL | FIXED | RESOLVED |
| F14 | review-finding:pr-2544-f14 | Human | Suggestion | documentation | ORIGINAL | FIXED | RESOLVED |
| F15 | review-finding:pr-2544-f15 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The spec still shows the automatic checks as pending

- PR number: 2544
- Source review ID: 5537902749
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2544#discussion_r4286680497>
- Concern: the PR description says the pre-commit hook and pre-push checks passed, but the spec's
  T3 row still says `linter all` and the pre-push checks are pending.
- Solution: the PR description was right. The pre-commit hook, which runs `linter all`, passed on
  every commit and the pre-push checks passed on the push, so T3 is `DONE` and the notes name both
  runs.
- Current-tree verification: the `ISSUE.md` T3 row says `DONE` and names the pre-commit hook and
  the pre-push checks.
- Resolution reference: docs(issues): [#2471] record the passed pre-commit and pre-push checks in the spec
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2544#discussion_r4287541261>

### F2 - The automatic verification checkpoint is not ticked

- PR number: 2544
- Source review ID: 5537902749
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2544#discussion_r4286680577>
- Concern: the same text as F1, posted on the workflow checkpoint line.
- Solution: ticked the checkpoint, in the same commit as F1.
- Current-tree verification: `ISSUE.md` has `- [x] Automatic verification completed`.
- Resolution reference: docs(issues): [#2471] record the passed pre-commit and pre-push checks in the spec
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2544#discussion_r4287541535>

### F3 - The doc comment implies an immediate shutdown

- PR number: 2544
- Source review ID: 5537902749
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2544#discussion_r4286680618>
- Concern: "cancels the token, so the server shuts down" reads as immediate, but cancellation is
  cooperative and the test comment says the binding is not released at once.
- Solution: the doc comment now says dropping requests cancellation and the server shuts down once
  it observes it, so the binding is released shortly after the drop.
- Current-tree verification: the `Running` doc comment in
  `packages/axum-http-server/src/testing/environment.rs` has that wording; clippy with `-D warnings`
  and `cargo +nightly fmt --check` pass for the package.
- Resolution reference: docs(axum-http-server): [#2471] say that dropping the test environment requests cancellation
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2544#discussion_r4287541809>

### F4 - `disarm().cancel()` is dense

- PR number: 2544
- Source review ID: 5537902749
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2544#discussion_r4286680681>
- Concern: `cancel_on_drop.disarm().cancel()` is correct but dense; splitting it into two
  statements would show the lifecycle intent.
- Solution: declined. The UDP test environment uses the identical single expression, so the two
  read alike; the property that matters is that nothing awaits between the disarm and the cancel,
  which one expression keeps visible. A split would add a binding without changing behavior and
  would make HTTP diverge from UDP. This departs from the `process-pr-review` rule for a no-change
  thread, which prescribes the reply `Superseded by <FindingId>: <reason>.` and the
  `SUPERSEDED` thread state: no finding supersedes a declined suggestion, so the reply states the
  reason directly and the row records `NO_ACTION` with the thread `RESOLVED`.
- Current-tree verification: `grep -n 'disarm()'` finds `cancel_on_drop.disarm().cancel();` in
  both `packages/axum-http-server/src/testing/environment.rs` and
  `packages/udp-server/src/testing/environment.rs`.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2544#discussion_r4287542067>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2544#discussion_r4287542067>

### F5 - The pre-SI-16 HTTP drop path did not stop the server, so this is not a regression from SI-16

- PR number: 2544
- Source review ID: 5537908774
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2544#discussion_r4286685317>
- Concern: the spec, the evidence, and the EPIC roadmap said a dropped halt sender used to stop
  the HTTP test environment and SI-16 lost that. Before SI-16 the shutdown task panics on the
  closed halt channel before it stops the server, so the server kept serving and the gap predates
  SI-16.
- Solution: confirmed by reading the pre-SI-16 source and by running the drop test on that tree
  (V4: the binding was still taken after 10.02 s, with the server-lib panic). The spec, the V1
  conclusion, the EPIC roadmap row and rationale, and the SI-23 and SI-24 drop-path notes now say
  the gap predates SI-16 and that the change aligns HTTP with UDP. The GitHub issue #2471 body and
  the PR body were corrected too. The V1 claim was inferred from source and marked "not run"; it
  was wrong.
- Current-tree verification: `grep` for the old wording found it in the correction entries that
  quote it but missed V1's Hypothesis, which still stated the premise as fact until
  review-finding:pr-2544-f13 and now carries a marker; `evidence/v4-pre-si16-drop-run.txt` records
  the failing run; the issue body and PR body read "predates SI-16".
- Resolution reference: docs(issues): [#2471] correct the history: the HTTP drop gap predates SI-16
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2544#discussion_r4287544789>

### F6 - Roadmap row 14 is marked Done while #2471 is open

- PR number: 2544
- Source review ID: 5537908774
- Reviewer finding ID: F2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2544#discussion_r4286685323>
- Concern: the `cleanup-completed-issues` skill sets a row to Done when the closed issue's spec is
  archived after merge; this row linked the open spec, which says T3 was in progress.
- Solution: reverted the row to `Planned` with a note that it is in review in this PR and becomes
  Done at archive time.
- Current-tree verification: the row 14 status cell of the EPIC roadmap reads `Planned`.
- Resolution reference: docs(issues): [#1488] keep roadmap row 14 Planned until the #2471 spec is archived
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2544#discussion_r4287545085>

### F7 - The EPIC `last-updated-utc` is later than the commit that carries it

- PR number: 2544
- Source review ID: 5537908774
- Reviewer finding ID: F3
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2544#discussion_r4286685333>
- Concern: the stamp (17:50) is after the commit that carries it (17:42:43Z). The review cited the
  previous commit's stamp (17:35, before its commit at 17:41:29Z) as the correct contrast; only the
  17:50 stamp was wrong.
- Solution: every stamp written after the review is at or before its commit. The 17:50 stamp
  stays in history, because rewriting would invalidate the review round, and the new commits
  correct the current value.
- Current-tree verification: for each commit after the reviewed head, compared the
  `last-updated-utc` of every changed spec with the commit author time in UTC; all are at or before
  it (the EPIC stamp is 19:41 in a commit authored at 19:41:40Z).
- Resolution reference: docs(issues): [#1488] keep roadmap row 14 Planned until the #2471 spec is archived
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2544#discussion_r4287545380>

### F8 - Acceptance criteria were ticked before a reviewer validated them

- PR number: 2544
- Source review ID: 5537908774
- Reviewer finding ID: F4
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2544#discussion_r4286685339>
- Concern: the implementer agent rules say not to mark acceptance criteria as done; AC1 to AC3 and
  the Acceptance Verification rows were marked done before the reviewer checkbox.
- Solution: cleared AC1 to AC3 and reset their rows to `TODO`, keeping the V2 and V3 evidence
  pointers for the reviewer.
- Current-tree verification: `ISSUE.md` has `- [ ] AC1` to `- [ ] AC3` and `TODO` in the three
  Acceptance Verification rows.
- Resolution reference: docs(issues): [#2471] leave the acceptance criteria to the reviewer and mark the spec in progress
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2544#discussion_r4287545711>

### F9 - The frontmatter status is still `planned` and the manual verification boxes are not ticked

- PR number: 2544
- Source review ID: 5537908774
- Reviewer finding ID: F5
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2544#discussion_r4286685343>
- Concern: `status: planned` while T1 and T2 are done; M1 is `DONE` while both manual-verification
  boxes are not ticked.
- Solution: set `status: in-progress`, a value the frontmatter validator accepts. Ticked the
  workflow checkpoint, because V1 to V4 and M1 are executed and recorded; left the matching box in
  the Acceptance Criteria list clear, because acceptance criteria belong to the reviewer (F8).
- Current-tree verification: `ISSUE.md` has `status: in-progress`, the checkpoint is `[x]`, and
  the acceptance-criteria box is `[ ]`.
- Resolution reference: docs(issues): [#2471] leave the acceptance criteria to the reviewer and mark the spec in progress
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2544#discussion_r4287546142>

### F10 - The record cites a PR-branch commit by id

- PR number: 2544
- Source review ID: 5539246048
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2544#discussion_r4287770695>
- Concern: the Findings introduction cited the reviewed head by commit id, which no longer resolves
  after a rebase; the `open-pull-request` skill says no durable reference may cite a PR-branch
  commit by id and lists review audits as durable references.
- Solution: the introduction cites the reviewed head by its subject only. My earlier commit-id
  check ran before this audit existed, so it could not have caught it.
- Current-tree verification: the skill's commit-id check, run verbatim against `torrust/develop`,
  prints nothing, and `grep -rn` for the id finds nothing under `docs`.
- Resolution reference: docs(pr-reviews): [#2544] correct the audit's head citation and its F4, F5 and F7 entries
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2544#discussion_r4288019515>

### F11 - F4 is a no-change thread, which the skill records as SUPERSEDED with an exact reply

- PR number: 2544
- Source review ID: 5539246048
- Reviewer finding ID: F2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2544#discussion_r4287770707>
- Concern: the `process-pr-review` rule says a no-change thread gets the reply `Superseded by
  <FindingId>: <reason>.` and the `SUPERSEDED` state; F4's row says `NO_ACTION` and `RESOLVED`.
  The row should follow the rule or the record should say plainly that it departs from it.
- Solution: the record now says plainly that F4 departs from the rule and why: no finding
  supersedes a declined suggestion. The reply already posted stays as written. The rule may need a
  clause for declined suggestions; that is a skill change for a separate issue.
- Current-tree verification: the F4 Solution says "This departs from the `process-pr-review`
  rule for a no-change thread" and names the reply and state the rule prescribes.
- Resolution reference: docs(pr-reviews): [#2544] correct the audit's head citation and its F4, F5 and F7 entries
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2544#discussion_r4288019766>

### F12 - Specs cite audit findings by bare ID, not by their reference

- PR number: 2544
- Source review ID: 5539246048
- Reviewer finding ID: F3
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2544#discussion_r4287770715>
- Concern: the #2471 spec, the evidence, and the SI-23 and SI-24 logs cited findings as "PR #2544
  review finding F5 (reviewer ID F1)", but the template defines `review-finding:pr-2544-f5` for
  citing a finding from other artifacts, and on the PR the reviewer's own tags name different
  findings.
- Solution: replaced the bare citations with the reference form in those four files.
- Current-tree verification: `grep -rn "PR #2544 review finding" docs/issues/open` returns nothing.
- Resolution reference: docs(issues): [#2471] cite review findings by their review-finding reference
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2544#discussion_r4288020071>

### F13 - V1's Hypothesis still states the disproved premise

- PR number: 2544
- Source review ID: 5539246048
- Reviewer finding ID: F4
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2544#discussion_r4287770731>
- Concern: the V1 Hypothesis still says the halt sender's drop ended the server, and the F5
  verification claimed the old wording survived only in correction entries, which missed this line.
- Solution: added a marker to the Hypothesis pointing to the correction and V4, and corrected the
  F5 verification to admit the miss.
- Current-tree verification: the Hypothesis ends with "(The second sentence is disproved: see the
  correction under V1's conclusion and V4. ...)"; `grep` for the old premise finds it only there
  and in the quoted correction entries.
- Resolution reference: docs(issues): [#2471] mark V1's hypothesis as disproved
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2544#discussion_r4288020365>

### F14 - "Was not checked" for the REST and health-check drops, which the code answers

- PR number: 2544
- Source review ID: 5539246048
- Reviewer finding ID: F5
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2544#discussion_r4287770746>
- Concern: the SI-23, SI-24 and #2471 notes said whether the legacy REST and health-check drops
  stop their servers was not checked, but both servers spawn the same `graceful_shutdown` task as
  HTTP, so the code already answers it.
- Solution: confirmed in the code and stated in the three notes, marked as read from source and not
  run, because V4 ran only the HTTP environment.
- Current-tree verification: `axum-rest-api-server/src/server.rs` and
  `axum-health-check-api-server/src/server.rs` spawn `graceful_shutdown`, which awaits
  `shutdown_signal_with_message` in `axum-server/src/signals.rs` before it signals the handle;
  `grep -c "was not checked"` finds none in the three specs.
- Resolution reference: docs(issues): [#2471] record that the REST and health-check legacy drops do not stop the server either
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2544#discussion_r4288020691>

### F15 - F7's Concern misreads the 17:35 stamp

- PR number: 2544
- Source review ID: 5539246048
- Reviewer finding ID: F6
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2544#discussion_r4287770757>
- Concern: the reviewer cited 17:35 (before its commit) as the correct contrast to 17:50; F7's
  Concern and my reply made it read like a second defect.
- Solution: the F7 entry now says only the 17:50 stamp was wrong. The reply stays as posted and
  the corrected entry is the record.
- Current-tree verification: the F7 Concern cites 17:35 as the correct contrast and the Solution
  names only the 17:50 stamp.
- Resolution reference: docs(pr-reviews): [#2544] correct the audit's head citation and its F4, F5 and F7 entries
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2544#discussion_r4288020998>

## Processing Log

- 2026-10-15 19:35 UTC - Fetched every thread and review, classified and numbered the findings, and
  fixed F3 in its own signed commit.
- 2026-10-15 19:41 UTC - Fixed F5, F1 and F2, F8 and F9, and F6 and F7 in separate signed commits.
  F5 included running the drop test on the tree before SI-16 (V4).
- 2026-10-15 19:45 UTC - Pushed, corrected the PR body and the GitHub issue body, re-derived each
  reply claim against the current tree, and replied on all nine threads.
- 2026-10-15 19:46 UTC - Started this record, with a reply URL in every row.
- 2026-10-15 20:36 UTC - da2ce7's approving review 5539246048 (six non-blocking threads) arrived
  after the first round was closed. Fixed F12, F13, F14 and the audit's own F10, F11 and F15 in
  separate signed commits, pushed, and replied on all six threads.
- 2026-10-15 20:37 UTC - Added F10-F15 to this record. Review 5539246048 shows `DISMISSED` after
  the push that followed it, so the PR needs a fresh approval.

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
