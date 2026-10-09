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

Review 5469277375 (round 2, josecelano, state `CHANGES_REQUESTED`, submitted 2026-10-09 11:13 UTC on head `5d667d817`) left three inline comments, 4229487368, 4229487374, and 4229487384, bracketed `[Major][F3]`, `[Major][F4]`, and `[Minor][F5]`, and one independently actionable review-body finding, `[Nit][F6]`, on the description. The rows keep the reviewer's IDs, which continue the round-1 numbering, as the audit-local IDs. The rest of the body records the reviewer's checks at that head, a merge plan (this pull request is held until PR #2434 merges), and answers to the description's requested decisions; none is a further finding. F6 has no thread, so it takes the review URL as `Source URL` and `NON_RESOLVABLE`; its fix is outside the tree, so its Resolution reference is the PR conversation response that states it. F3 to F5 are `RESOLVED`, the state the threads take when they are resolved after the push. Each Reply URL, and F6's Resolution reference, keeps the template's placeholder `<REPLY_URL_OR_NA>` until the posted response's URL is recorded. F1's and F2's verifications name lines at their round-1 head and hold there; this round's verifications anchor to headings and quotes.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2431-f1` | Copilot | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2431-f2` | Copilot | Major | testing | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2431-f3` | Human | Major | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2431-f4` | Human | Major | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F5 | `review-finding:pr-2431-f5` | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F6 | `review-finding:pr-2431-f6` | Human | Nit | metadata | ORIGINAL | FIXED | NON_RESOLVABLE |

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

### F3 - The skill and template citations name other text on develop

- PR number: 2431
- Source review ID: 5469277375
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2431#discussion_r4229487368>
- Concern: "Re-derive the `process-pr-review` skill citations against current `develop`." Since the branch's base, `docs(skills): handle reviews on a closed, superseded pull request` added a 35-line section and bumped the skill to version 1.5, and `docs(skills): cite a PR conversation response for fixes outside the tree` added two sentences, so every skill citation past that section, including the Validation Script, the Completion Checklist, and the #2362 ranges, named other text, and the draft could not know the skill's version before order 5 bumps it.
- Solution: After the rebase onto `develop` `686a42f45`, each skill, template, and validator citation was re-derived there and replaced by a section heading or a short quote (the Validation Script, Workflow step 9, the Completion Checklist, the Finding Details placeholder, the Processing Log guidance, and the validator's own strings), and the draft states the skill's version, 1.5, where it says order 5 bumps it. The #2362 citations are F5's.
- Current-tree verification: in `docs/issues/drafts/2278-author-self-audit-gate/ISSUE.md`, under "Background", the skill sentence reads "version 1.5 at `686a42f45`" and quotes "before every audit commit", "Update the audit progressively", and "treat a non-zero exit", each found once in `.github/skills/dev/pr-reviews/process-pr-review/SKILL.md` at `686a42f45` (under "Validation Script", "Workflow", and "Validation Script"); under "In Scope", the bump bullet reads "1.5 at `686a42f45`, or the version #2362 leaves if it lands first"; no skill, template, or validator line number remains in the draft.
- Resolution reference: `docs(issues): [#2278] cite the skill, template, matrix and EPIC by stable references`
- Follow-up PR URL: N/A
- Reply URL: <REPLY_URL_OR_NA>

### F4 - Matrix rows and EPIC passages are identified by line number

- PR number: 2431
- Source review ID: 5469277375
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2431#discussion_r4229487374>
- Concern: "Identify matrix and EPIC rows by something that does not move, not by line number." The Scope table, AC2, and the Progress Log named matrix rows by line number, which PR #2494 shifted on `develop`, and the EPIC citations move when PR #2434 adds its Decision Record lines.
- Solution: Matrix rows are named by F-ID or by source PR and a short quote of the proposal (the Scope table, the template-guidance bullet, AC2, Dependencies, and the two earlier Progress Log entries), and EPIC passages by section heading or quote (In Scope, Out of Scope, the Phase 2 exit criteria, and the Subissues order-10 row).
- Current-tree verification: in `docs/issues/drafts/2278-author-self-audit-gate/ISSUE.md`, under "In Scope", the table header reads "| Matrix row |" and its first row names PR #2270 and "Run the audit validator before every reply and audit commit"; AC2 reads "the Scope table checks for the PR #2271 rows and F55"; each quoted proposal occurs once in `docs/issues/open/2278-2003-strengthen-pr-review-author-self-audit/retrospective-improvement-matrix.md` at `686a42f45`, under "Author Verification and Convergence"; no matrix or EPIC line number remains in the draft.
- Resolution reference: `docs(issues): [#2278] cite the skill, template, matrix and EPIC by stable references`
- Follow-up PR URL: N/A
- Reply URL: <REPLY_URL_OR_NA>

### F5 - The #2362 overlap is stated from the old base

- PR number: 2431
- Source review ID: 5469277375
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2431#discussion_r4229487384>
- Concern: "Re-check the #2362 overlap claim against current `develop`." The Dependencies bullet and the description's decision 1 gave #2362's planned rewrites as line ranges from the old base and concluded no overlap, while on `develop` a template paragraph and two skill sentences inside those ranges had already changed.
- Solution: The Dependencies bullet names each passage #2362 rewrites by quote, re-derived at `686a42f45`, states that no passage is rewritten by both orders, names the three sections both edit (Workflow step 7, the Completion Checklist, and the template guidance beside Finding Details) and the shared version bump, and notes that the fix-outside-the-tree rule already on `develop` is the text #2362's F5 and F6 wording starts from. The description's decision 1 states the same.
- Current-tree verification: in `docs/issues/drafts/2278-author-self-audit-gate/ISSUE.md`, under "Dependencies and Open Questions", the #2362 bullet reads "This issue rewrites none of those passages, so no sentence is changed by both orders" and quotes "`FIXED` resolution references are unique Conventional Commit subjects" and "Use a unique Conventional Commit subject as the `Resolution reference`", each found once at `686a42f45` (the skill's "Detail-Entry Fields"; the template under `### <FINDING_ID> - <SUMMARY>`).
- Resolution reference: `docs(issues): [#2278] restate the #2362 overlap on current develop`
- Follow-up PR URL: N/A
- Reply URL: <REPLY_URL_OR_NA>

### F6 - The description's Files Touched and Validation are out of date

- PR number: 2431
- Source review ID: 5469277375
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2431#pullrequestreview-5469277375>
- Concern: The review body says the description's Files Touched omits `docs/pr-reviews/pr-2431-review/PR-REVIEW.md`, and its Validation says citations resolve at `develop` `eb96d2957` while the branch was rebased onto `29afe946c`; both need refreshing after the rebase.
- Solution: The description, outside the tree, is replaced: Files Touched lists the audit record and the matrix, a section records the rebase onto `develop` `686a42f45` and the carried R2 and R4, the requested decisions carry the maintainer's answers, a Merge order section follows PR #2434, and Validation states the gate at the new head.
- Current-tree verification: the description is not in the tree; the PR conversation response that answers this finding links the replacement.
- Resolution reference: <REPLY_URL_OR_NA>
- Follow-up PR URL: N/A
- Reply URL: <REPLY_URL_OR_NA>

## Processing Log

- 2026-10-03 17:33 UTC - Copilot review 5401885876 submitted with inline comments 4174143264 (`created_at` 17:33:20Z) and 4174143287 (17:33:21Z).
- 2026-10-03 17:39 UTC - Committed the F1 fix `docs(issues): [#2278] specify the pre-posting pass of the self-audit gate` (author date 17:39:48Z).
- 2026-10-03 17:40 UTC - Committed the F2 fix `docs(issues): [#2278] record a comparison base per historical audit` (author date 17:40:59Z).
- 2026-10-03 17:46 UTC - Replied on the thread opened by comment 4174143264 (reply 4174182851, `created_at` 17:46:05Z) and on the thread opened by comment 4174143287 (reply 4174182987, 17:46:08Z).
- 2026-10-03 17:46 UTC - Both threads reported resolved in the thread capture taken after the replies; the capture records no resolution time.
- 2026-10-03 17:47 UTC - Ran `validate-audit-record.py --pr-number 2431` on the uncommitted record, with the post-reply review-comments capture as `--comments-file` and `--base` at the branch's base (`Merge torrust/torrust-tracker#2419: ci(workflows): [#2402] cancel superseded PR runs`): `{"status": "ok", "rows": 2, "log_entries": 5, "failures": 0}`, exit 0.
- 2026-10-09 11:13 UTC - Review 5469277375 (josecelano, `CHANGES_REQUESTED`, on head `5d667d817`) left inline comments 4229487368 `[Major][F3]`, 4229487374 `[Major][F4]`, and 4229487384 `[Minor][F5]` (`created_at` 11:13:00Z), and the review-body finding `[Nit][F6]`. The thread capture shows the three new threads `resolved=false` and the two round-1 threads resolved and outdated.
- 2026-10-09 11:16 UTC - Rebased the branch onto `develop` `686a42f45` after PR #2494 merged, and committed `docs(issues): [#2278] carry the PR #2494 review's deferred R2 and R4 into the order-5 change` (author date 11:16:51Z); those items belong to review 5467975631 on PR #2494, not to this record.
- 2026-10-09 11:25 UTC - Re-derived F3 and F4 at the rebased head and committed their fix, `docs(issues): [#2278] cite the skill, template, matrix and EPIC by stable references` (author date 11:25:36Z).
- 2026-10-09 11:26 UTC - Committed `docs(issues): [#2278] cover the PR #2484 items the matrix assigns to order 5` (author date 11:26:19Z), which no finding raised: the matrix assigns those items to order 5 since PR #2494.
- 2026-10-09 11:27 UTC - Re-derived F5 on `develop` `686a42f45` and committed its fix, `docs(issues): [#2278] restate the #2362 overlap on current develop` (author date 11:27:00Z).
- 2026-10-09 11:28 UTC - Recorded F3 to F5 `FIXED`/`RESOLVED` and F6 `FIXED`/`NON_RESOLVABLE`; F6's fix is the replacement description. The replies and the conversation response are drafted for posting after the push, and their URLs replace the placeholders then.

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
