# PR #2584 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/2584>.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: <https://github.com/torrust/torrust-tracker/pull/2584#issuecomment-6301227746>
- Merge-gate waiver (reviewer ACK and Copilot approval): <https://github.com/torrust/torrust-tracker/pull/2584#issuecomment-6310493858>

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

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2584-f1 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2584-f2 | Copilot | Nit (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2584-f3 | Copilot | Nit (inferred) | metadata | ORIGINAL | NO_ACTION | RESOLVED |
| WAIT-001 | review-finding:pr-2584-wait-001 | Copilot | Major | correctness | ORIGINAL | FIXED | NON_RESOLVABLE |

## Finding Details

### F1 - Manual verification of the non-interactive poller path is unclear

- PR number: 2584
- Source review ID: 5589436901
- Reviewer finding ID: NA
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2584#discussion_r4328457426>
- Concern: The non-interactive bullet suggested that manual runs can be verified from a terminal,
  but the poller writes machine-readable output to stdout, so a TTY-based check does not show the
  intended behaviour for a manual run.
- Solution: The bullet now says manual runs redirect stdout to a file or pipe it (for example
  `| jq .`), so the manual check captures the same output an agent reads.
- Current-tree verification: Read the Non-interactive bullet in
  `docs/issues/open/2581-2003-pr-review-wait-poller/ISSUE.md` after commit `ccfde7bc9`; it now
  describes the redirect or pipe path.
- Resolution reference: docs(issues): clarify manual runs and folder prefix in #2581 spec
- Follow-up PR URL: NA
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2584#discussion_r4329431695>

### F2 - Folder prefix wording does not match the create-issue convention

- PR number: 2584
- Source review ID: 5589436901
- Reviewer finding ID: NA
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2584#discussion_r4328457526>
- Concern: Decision 4 described the folder prefix loosely, while the spec folder actually follows
  the `{issue}-{epic}-` prefix used by the create-issue convention.
- Solution: Decision 4 now states the folder uses the `{issue}-{epic}-` prefix (`2581-2003-`),
  following the create-issue convention.
- Current-tree verification: Read Decision 4 in
  `docs/issues/open/2581-2003-pr-review-wait-poller/ISSUE.md` after commit `ccfde7bc9`; the prefix
  wording matches the folder name `2581-2003-pr-review-wait-poller` and the create-issue skill.
- Resolution reference: docs(issues): clarify manual runs and folder prefix in #2581 spec
- Follow-up PR URL: NA
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2584#discussion_r4329431687>

### F3 - `last-updated-utc` should use RFC 3339

- PR number: 2584
- Source review ID: 5589436901
- Reviewer finding ID: NA
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2584#discussion_r4328457595>
- Concern: The reviewer asked that `last-updated-utc` use an RFC 3339 timestamp.
- Solution: Not applied. The repository convention for issue frontmatter is
  `"YYYY-MM-DD HH:MM"` in UTC, as shown in `docs/templates/ISSUE.md` and `docs/templates/EPIC.md`
  and used by sibling specs. The spec keeps that convention and its timestamp was bumped to
  `"2026-10-20 19:33"`.
- Current-tree verification: `grep -n "last-updated-utc" docs/templates/ISSUE.md
  docs/templates/EPIC.md docs/issues/open/2581-2003-pr-review-wait-poller/ISSUE.md` shows the
  template and the spec use the same `"YYYY-MM-DD HH:MM"` format.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2584#discussion_r4329431689>
- Follow-up PR URL: NA
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2584#discussion_r4329431689>

### WAIT-001 - The review-loop exit condition checks only the latest commit

- PR number: 2584
- Source review ID: 5591068061
- Reviewer finding ID: WAIT-001
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2584#pullrequestreview-5591068061>
- Concern: The spec's exit condition requires Copilot's review and an empty unresolved-thread
  list on the latest commit only. The loop can finish while actionable feedback from earlier
  commits stays unresolved. The reviewer asked for a latest-head review plus a check of all PR
  threads, including older and outdated ones. The review body has no inline thread, so the
  finding is `NON_RESOLVABLE`.
- Solution: The maintainer first approved an audit-only disposition (approval comment above). The
  spec fix then followed in PR #2596: the exit condition requires Copilot's latest review on the
  head commit and no unresolved actionable thread anywhere in the PR, including earlier commits.
- Current-tree verification: `grep -n "latest commit" docs/issues/open/2581-2003-pr-review-wait-poller/ISSUE.md`
  on `develop` at `fcd78975c` shows the exit condition at lines 108-110 ("has reviewed the latest
  commit ... and has no unresolved threads on that commit") and line 156 ("has reviewed the latest
  commit with no unresolved threads"). The finding remained live at that point. Re-checked on
  `develop` at `d21282aa4` after PR #2596 merged: `grep -c "has no unresolved threads on that
  commit"` on the spec returns 0, and the exit condition (lines 112-114) now counts unresolved
  actionable threads over the whole PR, including earlier commits. Decision 1 states the same.
- Resolution reference: `docs(issues): [#2581] count unresolved threads over the whole PR in the poller exit condition`
- Follow-up PR URL: <https://github.com/torrust/torrust-tracker/pull/2596>
- Reply URL: NA

## Processing Log

- 2026-10-20 19:35 UTC - Started audit of Copilot review 5589436901 on PR #2584.
- 2026-10-20 19:36 UTC - Fixed F1 and F2 in spec commit `ccfde7bc9`; rejected F3 with the repository
  timestamp convention as rationale.
- 2026-10-20 19:46 UTC - Resolved threads F1, F2, and F3 after replies were posted; no pending
  reviews or unresolved threads remained on PR #2584.
- 2026-10-20 20:53 UTC - Recorded the maintainer's audit-only disposition for Copilot review
  5591068061 (WAIT-001) in approval comment
  <https://github.com/torrust/torrust-tracker/pull/2584#issuecomment-6301227746>, created at
  20:52:32 UTC. Added the WAIT-001 tracking row and detail entry to this audit. The record was
  not yet validated or committed at this entry.
- 2026-10-21 07:25 UTC - Recorded the maintainer's waiver of the reviewer ACK and the Copilot-approval
  merge gate for PR #2584 in comment
  <https://github.com/torrust/torrust-tracker/pull/2584#issuecomment-6310493858>, created at
  07:25:32 UTC. PR #2584 merged at 2026-10-20 20:45 UTC with only the author's ACK.
- 2026-10-21 09:34 UTC - PR #2596 merged (`mergedAt` 09:33:59 UTC) with the spec fix for WAIT-001.
  Changed WAIT-001 from `FOLLOW_UP` to `FIXED`, with the fix commit subject as the resolution
  reference and #2596 as the follow-up PR. The tracking row, Solution, Current-tree verification,
  and Follow-up PR URL of the WAIT-001 entry were updated in place; the earlier log entries are unchanged.

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
