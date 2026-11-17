---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2923 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2923.

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
- When the current identity cannot resolve a thread (`viewerCanResolve` is `false`), keep the
  disposition established from the current tree and record `Thread state=OPEN`. The reviewer or PR
  author resolves it; permission says nothing about whether the concern was fixed.

## Findings

Round 1 is da2ce7's review `5944329364` (18:30:53 UTC, approval) of the head whose last commit is
`docs(issues): [#2902] record the tracker-core temporary-database fix`; its inline findings keep the
reviewer's IDs F1 to F4. Copilot's review `5944310231` (18:29:28 UTC, comment) of the same head is an
overview with no inline comment and no request (its full review timed out), so it needs no row.
Round 2 is da2ce7's review `5944598265` (18:50:32 UTC, approval) of the head whose last commit is
`docs(pr-reviews): [#2902] record the first review round of #2923`; its one inline finding keeps the
reviewer's ID F6. (The reviewer's IDs skip F5, which is the spec's own register finding.)
Round 3 is da2ce7's review `5944756705` (19:02:10 UTC, approval) of the head whose last commit is
`docs(pr-reviews): [#2902] record the second review round of #2923`; its one optional inline finding
keeps the reviewer's ID F7.
Round 4 is Copilot's review `5946673166` (21:09:58 UTC, comment) of the head whose last commit is
`docs(pr-reviews): [#2902] record the third review round of #2923`; its one inline comment has no
reviewer ID and is recorded as F8.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2923-f1 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2923-f2 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2923-f3 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2923-f4 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2923-f6 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2923-f7 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2923-f8 | Copilot | Minor (inferred) | maintainability | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The E1 step said 167 unit tests

- PR number: 2923
- Source review ID: 5944329364
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2923#discussion_r4604004209
- Concern: The E1 completion evidence said 167 unit tests; the package has 157 at this head.
- Solution: The E1 row says `157 unit tests pass.`; 167 was the count on the S1 branch.
- Current-tree verification: `grep -n "167" docs/issues/open/2902-1347-add-tests-tracker-core/test-refactor-plans/test-support-ephemeral-database-tests.md` prints nothing.
- Resolution reference: docs(issues): [#2902] correct the unit-test count of the E1 step
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2923#discussion_r4604046254

### F2 - The findings register said none is fixed

- PR number: 2923
- Source review ID: 5944329364
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2923#discussion_r4604004215
- Concern: F5 is `FIXED`, but the register lead sentence said "None is fixed in this issue".
- Solution: The lead sentence now reads "Each is a proposal for the maintainer unless its status says otherwise."
- Current-tree verification: `grep -n "unless its status says otherwise" docs/issues/open/2902-1347-add-tests-tracker-core/ISSUE.md` shows the sentence.
- Resolution reference: docs(issues): [#2902] qualify the findings register lead sentence
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2923#discussion_r4604046682

### F3 - Row T2b was still TODO

- PR number: 2923
- Source review ID: 5944329364
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/2923#discussion_r4604004218
- Concern: The plan and the progress log record the SQLite half of T2b as approved, but the T2b row was `TODO`.
- Solution: Row T2b is `IN_PROGRESS` and names the plan as the approved SQLite half.
- Current-tree verification: `grep -n "^| T2b | IN_PROGRESS" docs/issues/open/2902-1347-add-tests-tracker-core/ISSUE.md` shows the row.
- Resolution reference: docs(issues): [#2902] mark checkpoint T2b in progress
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2923#discussion_r4604047118

### F4 - "This PR" lost its referent after merge

- PR number: 2923
- Source review ID: 5944329364
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/2923#discussion_r4604004224
- Concern: "This PR covers `tracker-core` only" names no pull request, so it loses its referent once merged.
- Solution: The plan names #2923 and records that the SQLite part of #2907 moves to a new #2392 subissue.
- Current-tree verification: `grep -n "#2923 covers" docs/issues/open/2902-1347-add-tests-tracker-core/test-refactor-plans/test-support-ephemeral-database-tests.md` shows the sentence.
- Resolution reference: docs(issues): [#2902] name the pull request of the temporary-database split
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2923#discussion_r4604047504

### F6 - The shared fix was still handed to #2907

- PR number: 2923
- Source review ID: 5944598265
- Reviewer finding ID: F6
- Source URL: https://github.com/torrust/torrust-tracker/pull/2923#discussion_r4604219719
- Concern: "a new #2392 subissue" named no issue, and the spec (F5, Q7) and the description still gave the shared fix to #2907; "the #2907 session" read as process jargon.
- Solution: The plan names #2924 (the guard in `test-helpers`) and #2925 (the other packages, skipping `tracker-core`); F5 and Q7 point at them; the description relates to #2924 and #2925 instead of #2907. The 18:04 UTC progress-log entry stays, as the log is append-only.
- Current-tree verification: `grep -n "2924" docs/issues/open/2902-1347-add-tests-tracker-core/test-refactor-plans/test-support-ephemeral-database-tests.md docs/issues/open/2902-1347-add-tests-tracker-core/ISSUE.md` shows the plan, F5 and Q7; `grep -n "#2907 session" docs/issues/open/2902-1347-add-tests-tracker-core/test-refactor-plans/test-support-ephemeral-database-tests.md` prints nothing.
- Resolution reference: docs(issues): [#2902] name the issues that take the shared SQLite fix
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2923#discussion_r4604248385

### F7 - The F5 row gave the split a reason that postdates it

- PR number: 2923
- Source review ID: 5944756705
- Reviewer finding ID: F7
- Source URL: https://github.com/torrust/torrust-tracker/pull/2923#discussion_r4604348283
- Concern: The F6 fix made #2924 and #2925 the reason for the split, but both were opened after it; the reason was #2907's plan.
- Solution: The F5 row says the split happened because #2907 planned the shared fix, which #2924 and #2925 now take.
- Current-tree verification: `grep -n "because #2907 planned the shared fix" docs/issues/open/2902-1347-add-tests-tracker-core/ISSUE.md` shows the F5 row.
- Resolution reference: docs(issues): [#2902] give the F5 split its original reason
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2923#discussion_r4604374966

### F8 - The tracker fixtures were named like trackers

- PR number: 2923
- Source review ID: 5946673166
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2923#discussion_r4605828713
- Concern: `listed_tracker()` reads like it returns a tracker, but it returns an `EphemeralTrackerConfiguration` whose guard must stay alive.
- Solution: All four named fixtures end in `_configuration`: `public_tracker_configuration()`, `private_tracker_configuration()`, `private_tracker_configuration_with_keys_expiration_check_disabled()` and `listed_tracker_configuration()`; the plan uses the new names.
- Current-tree verification: `grep -rnE '\b(public|private|listed)_tracker\(' packages/tracker-core/src` prints nothing; `cargo test -p torrust-tracker-core --all-features` passes.
- Resolution reference: test(tracker-core): [#2902] name the tracker fixtures after the configurations they return
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2923#discussion_r4605924207

## Processing Log

- 2026-11-17 18:31 UTC - The review poller reported both round 1 reviews; all four findings verified against the tree.
- 2026-11-17 18:34 UTC - Fixed each finding in its own commit; replied to the four threads up to 18:34:50 UTC, before pushing the fixes and this record together; the threads are resolved after the push.
- 2026-11-17 18:53 UTC - Round 2 (approval with F6): fixed in its own commit and in the description; replied at 18:53:09 UTC, before pushing the fix and this record together; the thread is resolved after the push.
- 2026-11-17 19:04 UTC - Round 3 (approval with the optional F7): fixed in its own commit; replied at 19:04:41 UTC, before pushing the fix and this record together; the thread is resolved after the push.
- 2026-11-17 21:18 UTC - Round 4 (Copilot comment, Low, recorded as F8, Minor (inferred) from its Low label): fixed in its own commit; replied at 21:18:55 UTC, before pushing the fix and this record together; the thread is resolved after the push.

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- For an outdated thread whose concern was fixed, record `Disposition=FIXED` and
  `Thread state=RESOLVED`, even if GitHub marks the thread outdated after the push. For a
  duplicate, superseded, or no-change in-PR thread, reply exactly
  `Superseded by <FindingId>: <reason>.`, record `Disposition=NO_ACTION` and
  `Thread state=SUPERSEDED`, then resolve it. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work. When the current identity cannot resolve a thread
  (`viewerCanResolve` is `false`), still reply, keep the disposition established from the current
  tree, and record `Thread state=OPEN`; the reviewer or PR author resolves it.
- A consolidated PR conversation response may cover multiple review rounds only when it names
  every review ID and every finding ID with its disposition and resolution reference. Record its
  durable URL in each related row.
- Cite a fix by its unique Conventional Commit subject or durable reply URL, never by a branch SHA
  that can change after a rebase.
- Refresh review threads using GraphQL and confirm that no unresolved actionable thread remains.
