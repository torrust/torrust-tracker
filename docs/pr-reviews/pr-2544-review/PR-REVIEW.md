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
and F6), so the body adds no rows. The reviews read head `75c7be0fc`, whose subject is
"docs(issues): [#1488] mark #2471 done in the shutdown roadmap".

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
  would make HTTP diverge from UDP. The reply form `Superseded by <FindingId>: <reason>.` does not
  fit a declined suggestion, because no finding supersedes it, so the reply states the reason
  directly and the thread was resolved.
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
- Current-tree verification: `grep` for the old wording finds it only in the correction entries
  that quote it; `evidence/v4-pre-si16-drop-run.txt` records the failing run; the issue body and PR
  body read "predates SI-16".
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
- Concern: the stamp (17:50) is after the commit that carries it (17:42:43Z), and the previous
  commit's stamp was also before its own time.
- Solution: every stamp written after the review is at or before its commit. The two earlier
  stamps stay in history, because rewriting would invalidate the review round and the new commits
  correct the current values.
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

## Processing Log

- 2026-10-15 19:35 UTC - Fetched every thread and review, classified and numbered the findings, and
  fixed F3 in its own signed commit.
- 2026-10-15 19:41 UTC - Fixed F5, F1 and F2, F8 and F9, and F6 and F7 in separate signed commits.
  F5 included running the drop test on the tree before SI-16 (V4).
- 2026-10-15 19:45 UTC - Pushed, corrected the PR body and the GitHub issue body, re-derived each
  reply claim against the current tree, and replied on all nine threads.
- 2026-10-15 19:46 UTC - Started this record, with a reply URL in every row.

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
