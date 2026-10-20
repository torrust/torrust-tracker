# PR #2584 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/2584>.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: NA

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

## Processing Log

- 2026-10-20 19:35 UTC - Started audit of Copilot review 5589436901 on PR #2584.
- 2026-10-20 19:36 UTC - Fixed F1 and F2 in spec commit `ccfde7bc9`; rejected F3 with the repository
  timestamp convention as rationale.
- 2026-10-20 19:46 UTC - Resolved threads F1, F2, and F3 after replies were posted; no pending
  reviews or unresolved threads remained on PR #2584.

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
