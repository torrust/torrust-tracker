---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
    - "issue #2999"
---

<!-- skill-link: process-pr-review -->

# PR #3050 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/3050>.

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

Review 6058988660 (da2ce7, approved, 2026-11-25 14:00 UTC) posted three inline comments, F1 to F3, and two findings in its body without a thread, F4 and F5, which are recorded as `NON_RESOLVABLE` with the review URL as source. The review has no reviewer finding ids. The other bot comments on the pull request (CodeRabbit's rate-limit notice, the Claude Code Review notice that automated review is disabled for fork pull requests) hold no finding.

Review 6059384372 (da2ce7, approved, 2026-11-25 14:22 UTC), of the head with the first version of this record, posted three inline comments, F7 to F9, and one finding in its body without a thread, F6, recorded as `NON_RESOLVABLE` with the review URL as source. It has no reviewer finding ids. Its body also says the bot summary reports no actionable comment, which is not a finding.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-3050-f1 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-3050-f2 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-3050-f3 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-3050-f4 | Human | Nit | other | ORIGINAL | FIXED | NON_RESOLVABLE |
| F5 | review-finding:pr-3050-f5 | Human | Nit | documentation | ORIGINAL | NO_ACTION | NON_RESOLVABLE |
| F6 | review-finding:pr-3050-f6 | Human | Minor | documentation | ORIGINAL | FIXED | NON_RESOLVABLE |
| F7 | review-finding:pr-3050-f7 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-3050-f8 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-3050-f9 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The corrected detail counts seven files, the row names six

- PR number: 3050
- Source review ID: 6058988660
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3050#discussion_r4695994123>
- Concern: the corrected F21 detail of the PR 3042 audit said "the seven files the row names", but the Solution line names six; the secrecy ADR amendment is a seventh file that the row does not name. The log entry of 13:29 repeats "four of the seven".
- Solution: the detail says "the six files the row names and the secrecy ADR amendment". The 13:29 entry had been pushed for review by the archive commit of this pull request (it is not on `develop`; an earlier version of this detail and of the reply on the thread wrongly called it merged), and the rules say an entry that is already recorded is not rewritten, so a correction entry names it.
- Current-tree verification: `grep -n 'six files the row names and the secrecy ADR' docs/pr-reviews/pr-3042-review/PR-REVIEW.md` prints the corrected detail; `validate-audit-record.py --pr-number 3042` prints 24 rows and 0 failures.
- Resolution reference: docs(issues): [#2999] answer the review of the close-out pull request
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3050#discussion_r4696097263>

### F2 - The audit pointer follows an ACK clause the audit does not hold

- PR number: 3050
- Source review ID: 6058988660
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3050#discussion_r4695994148>
- Concern: the progress log of the archived spec pointed at the PR 3042 audit right after mentioning the ACKs, but that record holds no ACK.
- Solution: the entry cites the two ACK comments (`issuecomment-7161743992` of the maintainer and `issuecomment-7161627380` of da2ce7, both on the head of the merged pull request) and says the audit holds the review findings.
- Current-tree verification: `grep -n 'issuecomment-7161743992' docs/issues/closed/2999-2993-si-a-keep-secrets-out-of-errors-and-logs/ISSUE.md` prints the entry.
- Resolution reference: docs(issues): [#2999] answer the review of the close-out pull request
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3050#discussion_r4696097854>

### F3 - The #2917 spec changed without a new `last-updated-utc`

- PR number: 3050
- Source review ID: 6058988660
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3050#discussion_r4695994172>
- Concern: the reference repair in the closed spec of #2917 left its `last-updated-utc` at 2026-11-23, against step 4 of the `cleanup-completed-issues` skill.
- Solution: `last-updated-utc` of that spec is the date of the repair.
- Current-tree verification: `grep -n last-updated-utc docs/issues/closed/2917-2912-si-5-startup-and-udp-server-errors/ISSUE.md` prints `2026-11-25 14:02`; the other documents the archive commit changed already had it updated.
- Resolution reference: docs(issues): [#2999] answer the review of the close-out pull request
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3050#discussion_r4696098527>

### F4 - The title carries `[#2999]` for an issue the pull request does not close

- PR number: 3050
- Source review ID: 6058988660
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3050#pullrequestreview-6058988660>
- Concern: the title and the commit subject carry `[#2999]`, which `open-pull-request` keeps for the issue a pull request closes; this one is already closed and the body links it.
- Solution: the pull request title is now `chore(issues): archive closed issue #2999 spec to docs/issues/closed`, like the recent archive pull requests (for example #3046, #3035, #3021 and #3015, which archived the specs of #3019, #2924, #3004 and #2917), which have no bracket; the merge commit copies the title. The pushed commit keeps its subject, because history is not rewritten. The first version of the reply cited a precedent that does not exist and was corrected in place; the reply says so.
- Current-tree verification: `gh pr view 3050 --json title` prints the new title; `git log torrust/develop --grep='archive closed issue'` shows the recent subjects without a bracket.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/3050#issuecomment-7164822271>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3050#issuecomment-7164822271>

### F5 - The commit body does not match the diff

- PR number: 3050
- Source review ID: 6058988660
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3050#pullrequestreview-6058988660>
- Concern: the commit body says it corrects "the verification command" where only the reported result changes, and it omits the #2917 repair, as does the description's `rg` list.
- Solution: the commit is pushed and history is not rewritten, so its body stays. The description now names the #2917 repair and lists that file; the next commit says what it changed.
- Current-tree verification: `gh pr view 3050 --json body` prints the repair in the summary and in the `rg` line.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/3050#issuecomment-7164822271>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3050#issuecomment-7164822271>

### F6 - The description repeats the miscount and omits the new audit record

- PR number: 3050
- Source review ID: 6059384372
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3050#pullrequestreview-6059384372>
- Concern: the description said the corrected audit detail matches "four of the seven files the row names", and its file list omitted the audit record of this pull request. The merge commit copies the body.
- Solution: the description says four of the six files the row names and the secrecy ADR amendment, adds the audit record to the summary and the file list, and the closing-keyword scan of the body matches nothing. It is outside the repository tree, so the durable URL of the reply states it.
- Current-tree verification: `gh pr view 3050 --json body` prints both corrections.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/3050#issuecomment-7167491844>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3050#issuecomment-7167491844>

### F7 - "They said the row names seven files" can be read as the reviewer's claim

- PR number: 3050
- Source review ID: 6059384372
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3050#discussion_r4696279244>
- Concern: the 14:02 entry of the PR 3042 audit says "they said the row names seven files", which can be read as the reviewer saying it, when the review said the opposite.
- Solution: the entry is left as written (an entry already recorded is not rewritten); an entry appended after it names the subject: the 13:29 entry and the F21 detail were my text, and the reviewer said the row names six files.
- Current-tree verification: `grep -n 'Correction of the 14:02 entry' docs/pr-reviews/pr-3042-review/PR-REVIEW.md` prints the new entry; `validate-audit-record.py --pr-number 3042` prints 24 rows and 0 failures.
- Resolution reference: docs(pr-reviews): [#2999] correct three statements of the review audits
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3050#discussion_r4697424935>

### F8 - The 13:29 entry is called merged, but it is not on `develop`

- PR number: 3050
- Source review ID: 6059384372
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3050#discussion_r4696279255>
- Concern: the reply on the F1 thread called the 13:29 entry "a merged entry"; it was added by the archive commit of this pull request, which is pushed for review and not merged.
- Solution: the F1 detail and the reply on its thread say the entry was pushed for review and not merged, and that an earlier version wrongly said merged; the reply on the F1 thread was corrected in place with a note, and the reasoning (an entry already recorded is kept and a correction appended) stands.
- Current-tree verification: `grep -n 'wrongly called it merged' docs/pr-reviews/pr-3050-review/PR-REVIEW.md` prints the corrected detail; the reply on the F1 thread contains the same phrase.
- Resolution reference: docs(pr-reviews): [#2999] correct three statements of the review audits
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3050#discussion_r4697453068>

### F9 - Issues cited where the pull requests are meant

- PR number: 3050
- Source review ID: 6059384372
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3050#discussion_r4696279266>
- Concern: the F4 detail cites #3019, #2924, #3004 and #2917 as the recent archive pull requests, but those are the issues; the pull requests are #3046, #3035, #3021 and #3015.
- Solution: the F4 detail names the pull requests and says they archived the specs of those four issues.
- Current-tree verification: `grep -n '#3046, #3035, #3021 and #3015' docs/pr-reviews/pr-3050-review/PR-REVIEW.md` prints it; `git log torrust/develop --grep='archive closed issue'` shows those pull request numbers in the merge subjects.
- Resolution reference: docs(pr-reviews): [#2999] correct three statements of the review audits
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3050#discussion_r4697453756>

## Processing Log

- 2026-11-25 14:00 UTC - Review 6058988660 by da2ce7 (approved) arrived with three inline findings and two body findings, recorded as F1 to F5.
- 2026-11-25 14:09 UTC - Replied on the three threads after the fix commit was pushed (replies 4696097263, 4696097854 and 4696098527) and in one conversation comment for F4 and F5 (comment 7164822271, edited at 14:08 UTC to correct a precedent that the first version cited wrongly and again at 14:09 UTC to remove a wrong time from its note). The pull request title and description were edited.
- 2026-11-25 14:10 UTC - `validate-audit-record.py --pr-number 3050`, run with the bot token set for that one command (identity `josecelano-bot`), printed `{"status": "ok", "rows": 5, "log_entries": 2, "failures": 0}` before this entry; the three threads were then resolved by `josecelano-bot`, and a refreshed GraphQL listing shows 3 threads and 0 unresolved.
- 2026-11-25 15:59 UTC - Review 6059384372 by da2ce7 (approved, submitted at 14:22 UTC) with one body finding and three inline nits was recorded as F6 to F9. The fixes were pushed (the audit statements of the F1 and F4 details and a correction entry in the PR 3042 audit) and the replies were posted after that push: on the three threads (replies 4697424935, 4697453068 and 4697453756) and in one conversation comment for F6 (comment 7167491844). Several of these writes to GitHub answered with a server error (HTTP 502 or an empty response); each was checked against the listing of comments before it was retried, so no reply is duplicated. The first reply on the F7 thread was posted by mistake as a short placeholder and replaced in place with its text; the reply on the F1 thread was corrected in place to say the 13:29 entry was not merged.
- 2026-11-25 16:01 UTC - `validate-audit-record.py --pr-number 3050`, run with the bot token set for that one command (identity `josecelano-bot`) before this entry, printed `{"status": "ok", "rows": 9, "log_entries": 4, "failures": 0}`. The F7, F8 and F9 threads were then resolved by `josecelano-bot`, and a refreshed GraphQL listing shows 6 threads and 0 unresolved.

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- For an outdated thread whose concern was fixed, record `Disposition=FIXED` and
  `Thread state=RESOLVED`, even if GitHub marks the thread outdated after the push.
- Cite a fix by its unique Conventional Commit subject or durable reply URL, never by a branch SHA
  that can change after a rebase.
- Refresh review threads using GraphQL and confirm that no unresolved actionable thread remains.
