---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
    - "issue #3051"
---

<!-- skill-link: process-pr-review -->

# PR #3072 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/3072>.

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
- Reviewer: the author's GitHub login, for example `da2ce7` or `coderabbitai[bot]`; `unknown`
  when GitHub shows no author
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

Review 6075748165 (`da2ce7`, `CHANGES_REQUESTED`, 2026-11-26 14:23 UTC) posted four inline comments, F1 to F4; its body repeats the same four findings. Review 6077711472 (`da2ce7`, `APPROVED`, on head `docs(pr-reviews): [#3051] record the replies to review 6075748165 in the PR 3072 audit`) posted two inline comments, F5 and F6. Review 6075322623 (`claude[bot]`) only says that automated review is disabled for fork pull requests and has no finding.

| Finding ID | Review finding reference | Reviewer | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | -------- | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-3072-f1 | da2ce7 | Major | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-3072-f2 | da2ce7 | Major | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-3072-f3 | da2ce7 | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-3072-f4 | da2ce7 | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-3072-f5 | da2ce7 | Minor | documentation | ORIGINAL | FIXED | OPEN |
| F6 | review-finding:pr-3072-f6 | da2ce7 | Minor | documentation | ORIGINAL | FIXED | OPEN |

## Finding Details

### F1 - The "spec progress is up to date" tick contradicts the M3 and T5 rows

- PR number: 3072
- Source review ID: 6075748165
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3072#discussion_r4709683235>
- Concern: The archived `ISSUE.md` ticks that spec progress is up to date, while M3 and T5 are still `TODO`, although V3 is `DONE` and the fix PR is merged.
- Solution: Set T5 and M3 to `DONE` with their evidence (V3, the merged fix PR and the completion-review entry in the progress log), and recorded the change in the progress log.
- Current-tree verification: `grep -n "TODO" docs/issues/closed/3051-random-expiring-key-duration-overflow-panic/ISSUE.md` prints only the legend line of the status values.
- Resolution reference: docs(issues): [#3051] make the archived spec agree with its record
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3072#discussion_r4711109104>

### F2 - The "acceptance criteria updated with evidence" tick contradicts AC5

- PR number: 3072
- Source review ID: 6075748165
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3072#discussion_r4709683261>
- Concern: AC5 is `TODO` with an empty evidence cell, and the checkpoint on re-reviewing the acceptance criteria is unticked, although the evidence is the description of the merged fix PR.
- Solution: Filled the AC5 row with that evidence (the "Breaking public API change" section of the description of #3064), ticked AC5 like AC1 to AC4, and ticked the re-review checkpoint, since every row now has evidence.
- Current-tree verification: `grep -n "AC5" docs/issues/closed/3051-random-expiring-key-duration-overflow-panic/ISSUE.md` shows the ticked criterion and a `DONE` row with its evidence.
- Resolution reference: docs(issues): [#3051] make the archived spec agree with its record
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3072#discussion_r4711109527>

### F3 - The reason for the unticked reviewer checkpoint reads the wrong way

- PR number: 3072
- Source review ID: 6075748165
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3072#discussion_r4709683282>
- Concern: The progress-log entry says no reviewer other than the maintainer's ACK validated the criteria, which says the ACK did validate them.
- Solution: Reworded as the reviewer suggested: the ACK approved the head commit of each PR, and no reviewer checked the criteria against their checkboxes.
- Current-tree verification: `grep -n "approved the head commit" docs/issues/closed/3051-random-expiring-key-duration-overflow-panic/ISSUE.md` prints the progress-log entry.
- Resolution reference: docs(issues): [#3051] make the archived spec agree with its record
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3072#discussion_r4711109999>

### F4 - The V3 start command keeps the old path without a note

- PR number: 3072
- Source review ID: 6075748165
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3072#discussion_r4709683306>
- Concern: V1 says the folder has moved, but the V3 command, shown as it ran, names the `open/` path without a note.
- Solution: Added the same as-run note to V3: the command is shown as it ran, the folder has since moved to `docs/issues/closed/`, and the configuration file is the one in its `evidence/` folder.
- Current-tree verification: `grep -n "As in V1" docs/issues/closed/3051-random-expiring-key-duration-overflow-panic/manual-verification-evidence.md` prints the note.
- Resolution reference: docs(issues): [#3051] make the archived spec agree with its record
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3072#discussion_r4711110404>

### F5 - A progress-log entry stamped before the change it records

- PR number: 3072
- Source review ID: 6077711472
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3072#discussion_r4711242879>
- Concern: The 13:50 UTC entry also records the rows set to `DONE` after review 6075748165, a change made at 16:21 UTC, so the log's order is not true.
- Solution: Moved that sentence out of the 13:50 UTC entry into its own entry stamped when the edit was made, which also records this round's change.
- Current-tree verification: `grep -n "^- 2026-11-26" docs/issues/closed/3051-random-expiring-key-duration-overflow-panic/ISSUE.md` prints the 13:50 entry without that sentence and the new entry after it.
- Resolution reference: docs(issues): [#3051] leave the acceptance criteria for a reviewer to tick
- Follow-up PR URL: N/A
- Reply URL: pending

### F6 - Acceptance criteria ticked by the author of the work

- PR number: 3072
- Source review ID: 6077711472
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3072#discussion_r4711242890>
- Concern: AC5 and the "criteria re-reviewed" checkbox are ticked by the author, while the log says no reviewer checked the criteria; only a reviewer ticks acceptance criteria.
- Solution: Unticked the re-review checkbox and AC5, and AC1 to AC4 as well, because the same rule applies to them; the `DONE` rows of the Acceptance Verification table keep the evidence.
- Current-tree verification: `grep -n "^- \[x\] AC" docs/issues/closed/3051-random-expiring-key-duration-overflow-panic/ISSUE.md` prints nothing.
- Resolution reference: docs(issues): [#3051] leave the acceptance criteria for a reviewer to tick
- Follow-up PR URL: N/A
- Reply URL: pending

## Processing Log

- 2026-11-26 16:23 UTC - Review 6075748165 (`da2ce7`, `CHANGES_REQUESTED`, `submitted_at` 14:23:02Z) arrived on the head `chore(issues): archive closed issue #3051 spec to docs/issues/closed`: F1 to F4. This record was written before the first reply. `validate-audit-record.py` was not run by this session (it calls plain `gh`, which acts as `josecelano`).
- 2026-11-26 16:28 UTC - Pushed the fix commit `docs(issues): [#3051] make the archived spec agree with its record` and this record (`git ls-remote` showed the head on the fork branch before the replies), replied on the four threads and resolved them. `validate-audit-record.py` was not run by this session.
- 2026-11-26 17:13 UTC - Review 6077711472 (`da2ce7`, `APPROVED`) arrived on the head `docs(pr-reviews): [#3051] record the replies to review 6075748165 in the PR 3072 audit`: F5 and F6. This record was updated before the first reply. `validate-audit-record.py` was not run by this session.

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
