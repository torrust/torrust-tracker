---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #3011 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/3011>.

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

Round 1 covers review 6025635934 (da2ce7, CHANGES_REQUESTED) with six findings, review 6025446613 (Greptile, COMMENTED) and review 6025738529 (Copilot, COMMENTED), all on the first commit `docs(issues): [#2924] record the T1 design decision and the V1 reproduction`. Greptile's and Copilot's findings repeat F1 and F5 and are re-raises. Round 2 is review 6025843021 (da2ce7, COMMENTED) on the head `docs(issues): [#2924] rerun V1 with stamped steps and address review of PR 3011`. Its reviewer IDs F7 to F9 collide with this audit, so they are F10 to F12 here with the reviewer's ID in the detail entry. Round 3 is review 6026135793 (da2ce7, APPROVED); its reviewer IDs F10 to F12 are F13 to F15 here. Copilot's `High severity` badge maps to `Major (inferred)` and `Low severity` to `Nit (inferred)`; Greptile's `P2` maps to `Minor (inferred)`.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-3011-f1 | Human | Major | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-3011-f2 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-3011-f3 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-3011-f4 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-3011-f5 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-3011-f6 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-3011-f7 | Unknown | Minor (inferred) | documentation | RE_RAISE_OF:F1 | FIXED | RESOLVED |
| F8 | review-finding:pr-3011-f8 | Copilot | Major (inferred) | documentation | RE_RAISE_OF:F1 | FIXED | RESOLVED |
| F9 | review-finding:pr-3011-f9 | Copilot | Nit (inferred) | documentation | RE_RAISE_OF:F5 | FIXED | RESOLVED |
| F10 | review-finding:pr-3011-f10 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-3011-f11 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F12 | review-finding:pr-3011-f12 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F13 | review-finding:pr-3011-f13 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F14 | review-finding:pr-3011-f14 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F15 | review-finding:pr-3011-f15 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F16 | review-finding:pr-3011-f16 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The recorded V1 steps cannot produce the count of 6

- PR number: 3011
- Source review ID: 6025635934
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/3011#discussion_r4668947649
- Concern: `TMPDIR=$(mktemp -d) cargo ...` sets `TMPDIR` for the `cargo` command only, so the count step reads a different directory.
- Solution: V1 was rerun with `export TMPDIR=$(mktemp -d)` before both the test run and the count, with ordered stamps; the count is still 6.
- Current-tree verification: `grep -n 'export TMPDIR' docs/issues/open/2924-2392-sqlite-test-database-guard/manual-verification-evidence.md` finds the export in the embedded script and in step 1.
- Resolution reference: `docs(issues): [#2924] rerun V1 with stamped steps and address review of PR 3011`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3011#discussion_r4669025756

### F2 - No issue owns the adoption of the shared type by tracker-core

- PR number: 3011
- Source review ID: 6025635934
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/3011#discussion_r4668947662
- Concern: The spec hands the adoption to #2925, whose spec skips `tracker-core`.
- Solution: The T1 Decision section now says no issue owns the adoption yet and the maintainer decides later.
- Current-tree verification: `grep -n 'No issue owns' docs/issues/open/2924-2392-sqlite-test-database-guard/ISSUE.md` finds the sentence.
- Resolution reference: `docs(issues): [#2924] rerun V1 with stamped steps and address review of PR 3011`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3011#discussion_r4669026808

### F3 - The evidence file leaves out items that fix-bug and its template require

- PR number: 3011
- Source review ID: 6025635934
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/3011#discussion_r4668947671
- Concern: No outcome class, date without time, no step stamps, no Preserving Verification Artifacts section, no symptom or hypothesis.
- Solution: The evidence file now has the symptom, the hypothesis, the outcome class, full UTC stamps and the section.
- Current-tree verification: `grep -n 'Outcome class\|Preserving Verification Artifacts\|Symptom' docs/issues/open/2924-2392-sqlite-test-database-guard/manual-verification-evidence.md` finds all three.
- Resolution reference: `docs(issues): [#2924] rerun V1 with stamped steps and address review of PR 3011`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3011#discussion_r4669027348

### F4 - The PR description departs from open-pull-request

- PR number: 3011
- Source review ID: 6025635934
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/3011#discussion_r4668947677
- Concern: `Refs #2924` where `Related to #N` is prescribed, and no files-touched or validation items.
- Solution: The PR description was edited with `gh-bot pr edit` to use `Related to #2924`, name the two files and list the validation. This is a fix outside the repository tree; the reply states it.
- Current-tree verification: `gh-bot pr view 3011 --repo torrust/torrust-tracker --json body` shows `Related to #2924` and the two file names.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3011#discussion_r4669027802
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3011#discussion_r4669027802

### F5 - "below" should read "above", and the http-core caller is a test

- PR number: 3011
- Source review ID: 6025635934
- Reviewer finding ID: F5
- Source URL: https://github.com/torrust/torrust-tracker/pull/3011#discussion_r4668947680
- Concern: The T1 row says the decision is recorded below although it is above, and the caller is called a helper.
- Solution: The row says "above" and the spec names the caller as the test `it_should_panic_when_the_configured_database_cannot_be_initialized`.
- Current-tree verification: `grep -n 'Decision section above' docs/issues/open/2924-2392-sqlite-test-database-guard/ISSUE.md` finds the row.
- Resolution reference: `docs(issues): [#2924] rerun V1 with stamped steps and address review of PR 3011`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3011#discussion_r4669028345

### F6 - The update stamps are later than the commit that carries them

- PR number: 3011
- Source review ID: 6025635934
- Reviewer finding ID: F6
- Source URL: https://github.com/torrust/torrust-tracker/pull/3011#discussion_r4668947682
- Concern: `last-updated-utc` and the log entry say 16:15 UTC, the commit says 16:10 UTC.
- Solution: The stamps now use the time of the edit.
- Current-tree verification: `grep -n 'last-updated-utc' docs/issues/open/2924-2392-sqlite-test-database-guard/ISSUE.md` shows the new time.
- Resolution reference: `docs(issues): [#2924] rerun V1 with stamped steps and address review of PR 3011`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3011#discussion_r4669028874

### F7 - Greptile: the reproduction counts the wrong directory

- PR number: 3011
- Source review ID: 6025446613
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3011#discussion_r4668791659
- Concern: The same concern as F1: the `TMPDIR` assignment applies to `cargo` only.
- Solution: Fixed with F1.
- Current-tree verification: Same check as F1.
- Resolution reference: `docs(issues): [#2924] rerun V1 with stamped steps and address review of PR 3011`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3011#discussion_r4669026244

### F8 - Copilot: step 2 runs with an unset TMPDIR

- PR number: 3011
- Source review ID: 6025738529
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3011#discussion_r4669025399
- Concern: The same concern as F1, raised on the first commit of the pull request.
- Solution: Already fixed with F1.
- Current-tree verification: Same check as F1.
- Resolution reference: `docs(issues): [#2924] rerun V1 with stamped steps and address review of PR 3011`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3011#discussion_r4669040766

### F9 - Copilot: the note says below but the section is above

- PR number: 3011
- Source review ID: 6025738529
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3011#discussion_r4669025503
- Concern: The same concern as F5.
- Solution: Already fixed with F5.
- Current-tree verification: Same check as F5.
- Resolution reference: `docs(issues): [#2924] rerun V1 with stamped steps and address review of PR 3011`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3011#discussion_r4669041376

### F10 - The threads are answered but the head carries no audit record

- PR number: 3011
- Source review ID: 6025843021
- Reviewer finding ID: F7
- Source URL: https://github.com/torrust/torrust-tracker/pull/3011#discussion_r4669110203
- Concern: `process-pr-review` has the author create the audit record before the first reply; none exists.
- Solution: This record is created now from the template and the late start is logged in the Processing Log.
- Current-tree verification: `ls docs/pr-reviews/pr-3011-review/PR-REVIEW.md` lists the file.
- Resolution reference: `docs(pr-reviews): [#3011] record the audit of review rounds 1 and 2`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3011#discussion_r4669143960

### F11 - Eight replies cite the branch commit by id

- PR number: 3011
- Source review ID: 6025843021
- Reviewer finding ID: F8
- Source URL: https://github.com/torrust/torrust-tracker/pull/3011#discussion_r4669110216
- Concern: The round-1 replies cite the commit that answered round 1 by its id, which can change on a rebase.
- Solution: The replies stay as written; each is mapped in this record to the commit subject `docs(issues): [#2924] rerun V1 with stamped steps and address review of PR 3011` in its Resolution reference.
- Current-tree verification: `grep -c 'rerun V1 with stamped steps' docs/pr-reviews/pr-3011-review/PR-REVIEW.md` is non-zero and every F1 to F9 row except F4 cites the subject; F4's fix is the PR description, so its row cites its reply URL.
- Resolution reference: `docs(pr-reviews): [#3011] record the audit of review rounds 1 and 2`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3011#discussion_r4669144370

### F12 - The Observed Result is not what step 3 prints

- PR number: 3011
- Source review ID: 6025843021
- Reviewer finding ID: F9
- Source URL: https://github.com/torrust/torrust-tracker/pull/3011#discussion_r4669110222
- Concern: `count=6` comes from a throwaway script that the record does not show.
- Solution: The script is embedded verbatim in the evidence file, the step stamps and the `count=` line it writes are recorded, and the shebang added afterwards is disclosed.
- Current-tree verification: `grep -n 'v1r.times' docs/issues/open/2924-2392-sqlite-test-database-guard/manual-verification-evidence.md` finds the embedded script.
- Resolution reference: `docs(issues): [#2924] embed the V1 script in the evidence file`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3011#discussion_r4669144753

### F13 - The record cites two branch commit ids

- PR number: 3011
- Source review ID: 6026135793
- Reviewer finding ID: F10
- Source URL: https://github.com/torrust/torrust-tracker/pull/3011#discussion_r4669338000
- Concern: The record quoted the first commit's id and a reply text with a branch id; the commit-id check prints both.
- Solution: The record now says "the first commit of the pull request" and describes the reply text without the id.
- Current-tree verification: The `open-pull-request` commit-id check on the diff prints nothing.
- Resolution reference: `docs(pr-reviews): [#3011] cite commits by subject in the audit record`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3011#discussion_r4669343191

### F14 - F4's row does not cite the subject, so the F11 check text is wrong

- PR number: 3011
- Source review ID: 6026135793
- Reviewer finding ID: F11
- Source URL: https://github.com/torrust/torrust-tracker/pull/3011#discussion_r4669338011
- Concern: The check says every F1 to F9 row cites the subject, but F4's row cites its reply URL.
- Solution: The check text says every F1 to F9 row except F4 and names why.
- Current-tree verification: `grep -n 'except F4' docs/pr-reviews/pr-3011-review/PR-REVIEW.md` finds it.
- Resolution reference: `docs(pr-reviews): [#3011] cite commits by subject in the audit record`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3011#discussion_r4669343968

### F15 - The description's file list no longer matches the diff

- PR number: 3011
- Source review ID: 6026135793
- Reviewer finding ID: F12
- Source URL: https://github.com/torrust/torrust-tracker/pull/3011#discussion_r4669338024
- Concern: The description names two files; the branch also adds the audit record and a dictionary word.
- Solution: The PR description was edited with `gh-bot pr edit` to list the audit record and `project-words.txt`. A fix outside the repository tree; the reply states it.
- Current-tree verification: `gh-bot pr view 3011 --repo torrust/torrust-tracker --json body` lists the four files.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3011#discussion_r4669344642
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3011#discussion_r4669344642

### F16 - Two wording slips in the record

- PR number: 3011
- Source review ID: 6026327069
- Reviewer finding ID: F13
- Source URL: https://github.com/torrust/torrust-tracker/pull/3011#discussion_r4669477128
- Concern: The intro paragraph describes round 3 before round 2, and F11's Concern says "the branch commit of round 1", which reads as the commit reviewed in round 1.
- Solution: Round 3's sentence moved after round 2's, and the Concern now says "the commit that answered round 1".
- Current-tree verification: `grep -n 'commit that answered round 1' docs/pr-reviews/pr-3011-review/PR-REVIEW.md` finds the new wording.
- Resolution reference: `docs(pr-reviews): [#3011] fix two wording slips in the audit record`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3011#discussion_r4669490778

## Processing Log

- 2026-11-23 17:06 UTC - Started audit, late: the replies of round 1 (da2ce7 review 6025635934, Greptile 6025446613, Copilot 6025738529) had been posted and the threads resolved before this record existed, which da2ce7 raised as F10. Created from the template now. Round 2 (review 6025843021) was replied to before this commit; its reply URLs are recorded above. All twelve rows were checked against the current tree.
- 2026-11-23 17:20 UTC - Round 3: review 6026135793 (da2ce7, APPROVED) with three findings, F13 to F15 here (its F10 to F12). Replied on the three threads, edited the PR description for F15, fixed F13 and F14 in this record.
- 2026-11-23 17:31 UTC - Round 4: review 6026327069 (da2ce7, APPROVED, on the head after round 3) with one nit, F16 here (its F13). Replied, fixed the two wording slips in this record.

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
