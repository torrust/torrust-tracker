---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/drafts/2278-author-self-audit-gate/ISSUE.md
    - docs/issues/open/2278-2003-strengthen-pr-review-author-self-audit/EPIC.md
---

<!-- skill-link: process-pr-review -->

# PR #2431 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2431>.

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

Copilot review 5401885876 (round 1, state `COMMENTED`, <https://github.com/torrust/torrust-tracker/pull/2431#pullrequestreview-5401885876>) left two inline comments, 4174143264 and 4174143287. Its body is an overview only and creates no further finding:

> Changes recommended. The validation plan contains blocking inconsistencies that prevent the proposed workflow from being followed as written. Review effort: Balanced. Findings: 1 High · 1 Medium.

The overview renders "High" and "Medium" as severity badge images and lists the two open items "Define pre-posting validation and pending-reply gate behavior" and "Record historical comparison bases for completed audits". Each inline comment opens with an explicit `[Major][F1]` or `[Major][F2]` bracket, so both rows record `Major` from the bracket rather than a severity inferred from the badges; the badges rank F1 above F2. The bracketed IDs are used as the audit-local IDs.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2431-f1` | Copilot | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2431-f2` | Copilot | Major | testing | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The self-audit gate cannot pass before the first reply

- PR number: 2431
- Source review ID: 5401885876
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2431#discussion_r4174143264>
- Concern: The validator requires one posted reply per discussion-anchored row, and the skill treats every validator failure as blocking, so a gate that runs the validator before the first reply cannot pass, and repeating its checks by hand cannot produce a reply that has not been posted. The draft had to say which checks apply before posting, how a pending reply field is treated, and when the posted reply URL and its thread are validated, with AC3 and M1 aligned.
- Solution: The draft splits the gate into a pre-posting pass before each reply, which runs every check that needs no posted reply and names the pending reply-URL check as its only exception, and a full pass with no exception before the audit commit, each resolution, and completion; the audit record is committed after its replies. Scope rows 35 and 119, the gate bullet, AC3, AC8, M1, M2, Risks, and the Progress Log were updated.
- Current-tree verification: `git grep -n -E "pre-posting|reply-URL|committed after|cannot pass before the first reply" HEAD -- docs/issues/drafts/2278-author-self-audit-gate/ISSUE.md` matches lines 49, 59, 72, 163, 169, 174, 200, 201, 224, and 232; `git grep -n -E "^\| 119 \|" HEAD -- docs/issues/drafts/2278-author-self-audit-gate/ISSUE.md` matches line 70, which commits the record with each posted reply URL before the first resolution.
- Resolution reference: `docs(issues): [#2278] specify the pre-posting pass of the self-audit gate`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2431#discussion_r4174182851>

### F2 - The historical validator runs lack a comparison base

- PR number: 2431
- Source review ID: 5401885876
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2431#discussion_r4174143287>
- Concern: The validator resolves cited commit subjects over `<base>..HEAD` with `--base` defaulting to `develop`, so run from the implementation branch over historical audits it excludes fixes already merged into `develop`, and AC9 fails on valid records.
- Solution: The automatic check runs each of the three most recent audits with `--base <B>`, the recorded merge base of that audit's PR (`git merge-base <M>^1 <M>^2` for its merge commit `<M>`), and states that no record is edited to fit the default range; AC9 requires the recorded base and forbids editing a historical record.
- Current-tree verification: `git grep -n -E "\-\-base|merge-base|no historical record is edited|no record is edited" HEAD -- docs/issues/drafts/2278-author-self-audit-gate/ISSUE.md` matches lines 175 (AC9) and 191 (the automatic check).
- Resolution reference: `docs(issues): [#2278] record a comparison base per historical audit`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2431#discussion_r4174182987>

## Processing Log

- 2026-10-03 17:33 UTC - Copilot review 5401885876 submitted with inline comments 4174143264 (`created_at` 17:33:20Z) and 4174143287 (17:33:21Z).
- 2026-10-03 17:39 UTC - Committed the F1 fix `docs(issues): [#2278] specify the pre-posting pass of the self-audit gate` (author date 17:39:48Z).
- 2026-10-03 17:40 UTC - Committed the F2 fix `docs(issues): [#2278] record a comparison base per historical audit` (author date 17:40:59Z).
- 2026-10-03 17:46 UTC - Replied on the thread opened by comment 4174143264 (reply 4174182851, `created_at` 17:46:05Z) and on the thread opened by comment 4174143287 (reply 4174182987, 17:46:08Z).
- 2026-10-03 17:46 UTC - Both threads reported resolved in the thread capture taken after the replies; the capture records no resolution time.
- 2026-10-03 17:47 UTC - Ran `validate-audit-record.py --pr-number 2431` on the uncommitted record, with the post-reply review-comments capture as `--comments-file` and `--base` at the branch's base (`Merge torrust/torrust-tracker#2419: ci(workflows): [#2402] cancel superseded PR runs`): `{"status": "ok", "rows": 2, "log_entries": 5, "failures": 0}`, exit 0.

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
