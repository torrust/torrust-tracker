---
pr-number: 2920
pr-url: https://github.com/torrust/torrust-tracker/pull/2920
last-updated-utc: "2026-11-18 11:52"
---

<!-- skill-link: process-pr-review -->

# PR #2920 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2920.

<!-- Copied verbatim into each audit record after placeholder replacement. -->

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: https://github.com/torrust/torrust-tracker/pull/2920#issuecomment-6977200583 (retrospective record: the maintainer gave the approval in the working session on 2026-11-18 at about 09:58 UTC, and created this comment at 11:26:15Z)

<!-- Copied verbatim into each audit record. -->

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

Round 1 is two reviews on the first head that carried the post-merge audit rows of pull request 2905: the approving review
`5943570147` of `da2ce7` (F1 to F8, reviewer-provided IDs) and Copilot's review `5943581377` (COMMENTED, two findings
with its own IDs F1 and F2, which collide with the first review's and are recorded as F9 and F10). F8 and F10 were processed after the others, once the maintainer had posted the comment they ask for; see the Processing Log.
Round 2 is the approving review `5944190011` of `da2ce7` (F11 and F12, audit-local IDs; the reviewer numbers them F22
and F23, continuing the count of pull request 2905's record), on the head that carried the last audit commit of round 1.
Round 3 is the approving review `5944671302` of `da2ce7` (F13, audit-local ID; the reviewer numbers it F24), on the head
that carried the round 2 audit.
Round 4 is the approving review `5944877623` of `da2ce7` (F14, audit-local ID; the reviewer numbers it F25), on the head
that carried the round 3 audit.
Round 5 is the approving review `5953723621` of `da2ce7` (F15, audit-local ID; the reviewer numbers it F26), on the head
rebased onto `d8e14450d`.
Round 6 is the feedback still unprocessed when this pull request merged, both reviews submitted before the merge: the approving review `5953969566` of `da2ce7` (F16, audit-local ID; the reviewer numbers it
F27) and Copilot's review `5954232676` (COMMENTED, three comments without IDs, recorded as F17 to F19), both on the head
that carried the round 5 audit. Their fixes are in the follow-up pull request #2942.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2920-f1` | Human | Minor | maintainability | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2920-f2` | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2920-f3` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2920-f4` | Human | Nit | correctness | ORIGINAL | FIXED | RESOLVED |
| F5 | `review-finding:pr-2920-f5` | Human | Suggestion | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | `review-finding:pr-2920-f6` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | `review-finding:pr-2920-f7` | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F8 | `review-finding:pr-2920-f8` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F9 | `review-finding:pr-2920-f9` | Copilot | Minor | correctness | RE_RAISE_OF:F7 | FIXED | RESOLVED |
| F10 | `review-finding:pr-2920-f10` | Copilot | Minor | documentation | RE_RAISE_OF:F8 | FIXED | RESOLVED |
| F11 | `review-finding:pr-2920-f11` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F12 | `review-finding:pr-2920-f12` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F13 | `review-finding:pr-2920-f13` | Human | Nit | correctness | ORIGINAL | FIXED | RESOLVED |
| F14 | `review-finding:pr-2920-f14` | Human | Nit | correctness | ORIGINAL | FIXED | RESOLVED |
| F15 | `review-finding:pr-2920-f15` | Human | Suggestion | documentation | ORIGINAL | FIXED | RESOLVED |
| F16 | `review-finding:pr-2920-f16` | Human | Nit | documentation | ORIGINAL | FOLLOW_UP | OPEN |
| F17 | `review-finding:pr-2920-f17` | Copilot | Minor (inferred) | documentation | ORIGINAL | FOLLOW_UP | OPEN |
| F18 | `review-finding:pr-2920-f18` | Copilot | Minor (inferred) | documentation | RE_RAISE_OF:F17 | FOLLOW_UP | OPEN |
| F19 | `review-finding:pr-2920-f19` | Copilot | Minor (inferred) | metadata | ORIGINAL | FOLLOW_UP | OPEN |

## Finding Details

### F1 - The pointer files enter the container build context

- PR number: 2920
- Source review ID: 5943570147
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2920#discussion_r4603463621
- Concern: The pointer files under `src/`, `tests/`, `packages/` and `packages/configuration/` are in directories the container build admits. `.dockerignore` re-excludes `**/AGENTS.md` there, because no build stage reads agent instructions and an edit to one must not invalidate the source `COPY` layers (decision 3 of ADR `20260929183441`); the pointer files were not re-excluded.
- Solution: `.dockerignore` re-excludes `**/CLAUDE.md` on the line after `**/AGENTS.md`, under the same comment. The ADR is not edited: the new line follows the reason it gives.
- Current-tree verification: `grep -n -A3 "Re-exclusions inside admitted" .dockerignore` shows `**/AGENTS.md` and `**/CLAUDE.md` on consecutive lines. `linter all` passed in the pre-commit checks of the commit.
- Resolution reference: `build(container): [#2904] keep the pointer CLAUDE.md files out of the build context`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2920#discussion_r4603595739

### F2 - The conclusion about probes 1 and 2 says more than they show

- PR number: 2920
- Source review ID: 5943570147
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2920#discussion_r4603463631
- Concern: The evidence said a root `CLAUDE.md` alone stops the scoped `AGENTS.md` files "with a parent `CLAUDE.md` (probe 2) and without one (probe 4)". Probe 1 already answers NOT LOADED for the scoped rule in checkout P, so there the root pointer leaves them unloaded; only checkout N (probes 3 and 4) shows it stopping them. The Progress Log line and the description repeated the wording.
- Solution: The conclusion says any `CLAUDE.md` at the root or above stops the scoped files (probes 1, 2 and 4) and a root pointer restores only the root file, with the two checkouts told apart. The Progress Log line of `ISSUE.md` and the pull request description use the same wording.
- Current-tree verification: Read the conclusion of section V4 against the table of probes after the change. `grep -rn "with and without a" docs/issues/open/2904-2003-migrate-custom-agents-to-claude-code/` prints only lines about the two checkouts of the scenario, none about the root pointer stopping the scoped files.
- Resolution reference: `docs(issues): [#2904] correct the M4 evidence and store every probe's answer`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2920#discussion_r4603596181

### F3 - Two status cells lag the state this pull request records

- PR number: 2920
- Source review ID: 5943570147
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/2920#discussion_r4603463640
- Concern: The pull request ticks the checkpoint for the merged specification pull request, but the T1 row stayed `TODO`; the frontmatter moved to `in-progress` while row 23 of the EPIC's Subissues table stayed `TODO`.
- Solution: T1 is `DONE` in the Implementation Plan, in `docs(issues): [#2904] correct the M4 evidence and store every probe's answer`. Row 23 of the EPIC is `IN_PROGRESS`, with a Progress Log line.
- Current-tree verification: `grep -n "^| T1 " ISSUE.md` in the issue folder shows `DONE`; `grep -n "^| 23 " EPIC.md` in the EPIC folder shows `IN_PROGRESS`.
- Resolution reference: `docs(issues): [#2904] mark the EPIC row of issue 2904 in progress`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2920#discussion_r4603596608

### F4 - Two named commands do not produce what the evidence attributes to them

- PR number: 2920
- Source review ID: 5943570147
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/2920#discussion_r4603463654
- Concern: Plain `ls -l` cannot give a time to the second in UTC; `git clone --no-hardlinks --depth 1` of a local path ignores `--depth`; the parent-folder loop was described without its command.
- Solution: The evidence names the commands as run: `TZ=UTC ls -l --time-style=+%H:%M:%S`; the clone with the `file://` form, which the evidence had dropped and with which `--depth` applies; and the loop with its command and that it printed nothing. The same paragraph is corrected in one more place: the personal file is four folders above checkout P, not three.
- Current-tree verification: Compared each command in the Environment section and the steps of section V4 with the commands of the session that ran them. `linter markdown` and `linter cspell` passed.
- Resolution reference: `docs(issues): [#2904] correct the M4 evidence and store every probe's answer`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2920#discussion_r4603597014

### F5 - Store each probe's answer text, not only the summary table

- PR number: 2920
- Source review ID: 5943570147
- Reviewer finding ID: F5
- Source URL: https://github.com/torrust/torrust-tracker/pull/2920#discussion_r4603463663
- Concern: The template asks for the outputs a reviewer needs in the issue folder; the reason given for not storing them covers the full output, not its answer field. Probe 3, the one positive result behind "loads by itself", gains most.
- Solution: `evidence/m4-probe-answers.md` holds the answer field of all twelve probes, verbatim except for four local path fragments that are replaced and named at the top of the file. The evidence links it.
- Current-tree verification: `grep -c "^## Probe " evidence/m4-probe-answers.md` in the issue folder prints 12. `grep -c "/home/\|/tmp/"` on the same file prints 0.
- Resolution reference: `docs(issues): [#2904] correct the M4 evidence and store every probe's answer`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2920#discussion_r4603597441

### F6 - The description no longer matches the branch at this head

- PR number: 2920
- Source review ID: 5943570147
- Reviewer finding ID: F6
- Source URL: https://github.com/torrust/torrust-tracker/pull/2920#discussion_r4603463667
- Concern: "Files touched" omitted the audit record of pull request 2905, "addressed in the last commit" pointed at the wrong commit after a later push, and "the two audit rows follow in a later commit" described a commit already on the branch.
- Solution: The description was edited at 17:54:15Z (forge `updated_at`): it lists every changed file, names the commit of the independent review by its subject, and says the two audit rows are on the branch. No file changes.
- Current-tree verification: `gh pr view 2920 --json body` read after the edit against `git diff --name-only torrust/develop...HEAD`.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2920#discussion_r4603597887
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2920#discussion_r4603597887

### F7 - F20 of pull request 2905 is recorded fixed while its fix rides in this open follow-up

- PR number: 2920
- Source review ID: 5943570147
- Reviewer finding ID: F7
- Source URL: https://github.com/torrust/torrust-tracker/pull/2920#discussion_r4603463673
- Concern: The audit record of pull request 2905 recorded its post-merge finding F20 as `FIXED`/`RESOLVED` and its thread was resolved, while the fix is only on this branch. `process-pr-review` (Reviews Submitted After Merge, steps 5 and 6) keeps such a row `FOLLOW_UP`/`OPEN` until the follow-up merges.
- Solution: In the audit record of pull request 2905, F20 is `FOLLOW_UP`/`OPEN` with the reply on its thread as resolution reference, and a correction entry in its Processing Log names the wrong entry. Its thread was reopened by its thread ID at 17:48:28Z.
- Current-tree verification: `grep -n "^| F20 " docs/pr-reviews/pr-2905-review/PR-REVIEW.md` shows `FOLLOW_UP | OPEN`. A GraphQL read of the threads of pull request 2905 shows the thread of comment 4602381116 unresolved. `validate-audit-record.py --pr-number 2905` exited 0 (22 rows).
- Resolution reference: `docs(pr-reviews): [#2904] keep finding F20 of PR #2905 open until its follow-up merges`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2920#discussion_r4603598317

### F8 - The Ownership section of the audit of pull request 2905 has no post-merge approval

- PR number: 2920
- Source review ID: 5943570147
- Reviewer finding ID: F8
- Source URL: https://github.com/torrust/torrust-tracker/pull/2920#discussion_r4603463689
- Concern: Round 8 of the audit record of pull request 2905 is feedback unprocessed at merge, yet its Ownership section read `Post-merge workflow approval: N/A`. Step 2 of `process-pr-review` (Reviews Submitted After Merge) asks for the maintainer's approval as a durable comment URL before any mutating action; chat-only approval does not satisfy it.
- Solution: The maintainer posted the approval as a comment on pull request 2905 at 17:59:16Z. The Ownership section of that audit record holds its URL as a retrospective record, with the original approval time (about 16:44 UTC, in the working session) and the comment's creation time, and its Processing Log has an entry for it. The round 8 changes were made before the comment existed; the log of that record says so.
- Current-tree verification: `grep -n "Post-merge workflow approval" docs/pr-reviews/pr-2905-review/PR-REVIEW.md` shows the comment URL and both times. `gh api repos/torrust/torrust-tracker/issues/comments/6956439261` shows the author `josecelano` and `created_at` 17:59:16Z. `validate-audit-record.py --pr-number 2905` exited 0 (22 rows).
- Resolution reference: `docs(pr-reviews): [#2904] record the maintainer's approval of the post-merge work of PR #2905`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2920#discussion_r4603701212

### F9 - Keep F20 pending until the follow-up merges

- PR number: 2920
- Source review ID: 5943581377
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2920#discussion_r4603473090
- Concern: The same concern as F7, raised by Copilot's review 48 seconds later: F20 of pull request 2905 is marked `FIXED`/`RESOLVED` while its change exists only in this open pull request.
- Solution: The change recorded under F7.
- Current-tree verification: The checks recorded under F7.
- Resolution reference: `docs(pr-reviews): [#2904] keep finding F20 of PR #2905 open until its follow-up merges`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2920#discussion_r4603599135

### F10 - Record the authorization URL for this post-merge handoff

- PR number: 2920
- Source review ID: 5943581377
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2920#discussion_r4603473174
- Concern: The same concern as F8, raised by Copilot's review 49 seconds later. It suggests filling the field with the reply `discussion_r4602755221` on pull request 2905.
- Solution: The change recorded under F8. The suggested URL is not used: the contributor account wrote that reply, and the skill asks for the maintainer's approval.
- Current-tree verification: The checks recorded under F8.
- Resolution reference: `docs(pr-reviews): [#2904] record the maintainer's approval of the post-merge work of PR #2905`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2920#discussion_r4603701767

### F11 - Checkout P's description changed in substance without a note in the evidence

- PR number: 2920
- Source review ID: 5944190011
- Reviewer finding ID: F22
- Source URL: https://github.com/torrust/torrust-tracker/pull/2920#discussion_r4603897988
- Concern: Round 1 changed the description of checkout P (the personal file four folders above, not three; a worktree inside another checkout whose `AGENTS.md` is three folders above), and the evidence did not say it was a correction. The change appeared only in the reply to F4 and partly in its row.
- Solution: Failures and Follow-up of the evidence has a line for the correction: what the first version said, what is true, the commit that corrected the Environment section, that no probe was rerun, and why the conclusions do not depend on it.
- Current-tree verification: `grep -n "Correction to the description of checkout P" manual-verification-evidence.md` in the issue folder prints the line. Read the Q3 answers of the six probes run in checkout P (probes 1, 2, 7, 8, 11 and 12) in `evidence/m4-probe-answers.md`: probe 1 lists only the personal `CLAUDE.md` of the parent folder, the other five list that file and files of checkout P, and none of the six lists an `AGENTS.md` above checkout P. `linter markdown` and `linter cspell` passed.
- Resolution reference: `docs(issues): [#2904] note the corrected description of checkout P in the M4 evidence`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2920#discussion_r4604090320

### F12 - The answers file uses two placeholders its header does not name

- PR number: 2920
- Source review ID: 5944190011
- Reviewer finding ID: F23
- Source URL: https://github.com/torrust/torrust-tracker/pull/2920#discussion_r4603898005
- Concern: The header of `evidence/m4-probe-answers.md` names four replaced path fragments, but the answers also contain `<worktree>` and `<repo>`, so a reader cannot tell which angle-bracket names are replacements.
- Solution: The header says that `<worktree>` and `<repo>` are abbreviations the probes chose themselves, that each probe using one says which path it stands for, and that only that path was replaced.
- Current-tree verification: `sed -n '1,13p' evidence/m4-probe-answers.md` in the issue folder shows the new paragraph. `grep -c "<worktree>"` and `grep -c "<repo>"` on that file print 16 and 8 lines, one of each being the new paragraph.
- Resolution reference: `docs(issues): [#2904] note the corrected description of checkout P in the M4 evidence`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2920#discussion_r4604090708

### F13 - Two current-tree verifications of the round 2 rows do not show what they state

- PR number: 2920
- Source review ID: 5944671302
- Reviewer finding ID: F24
- Source URL: https://github.com/torrust/torrust-tracker/pull/2920#discussion_r4604278993
- Concern: The verification of F12 gave the counts 15 and 7 for the two placeholder names; at the head they are 16 and 8, the earlier counts being from before the fix. The verification of F11 cited a `grep -c` for a literal local path, which prints 0 whatever the probes answered because every path in the answers file is replaced, and it named a local folder the answers file withholds.
- Solution: The line of F12 gives 16 and 8 and says one of each is the new paragraph. The line of F11 cites the Q3 answers of the six probes of checkout P instead of the path search. The Processing Log has a correction entry.
- Current-tree verification: `grep -c "<worktree>"` and `grep -c "<repo>"` on `evidence/m4-probe-answers.md`, run again after the fix of round 2, print 16 and 8. A script listed the `AGENTS.md` and `CLAUDE.md` paths in the Q3 answers of probes 1, 2, 7, 8, 11 and 12: each list holds the parent folder's `CLAUDE.md` or files of checkout P only. A search of `docs/` for the local folder name the earlier line contained finds no match.
- Resolution reference: `docs(pr-reviews): [#2904] correct two verification lines in the review audit of PR #2920`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2920#discussion_r4604343733

### F14 - The corrected line of F11 says more than probe 1's answer shows

- PR number: 2920
- Source review ID: 5944877623
- Reviewer finding ID: F25
- Source URL: https://github.com/torrust/torrust-tracker/pull/2920#discussion_r4604444973
- Concern: The verification line of F11, as corrected in round 3, said each of the six answers of checkout P lists the parent folder's personal file and files of checkout P. Probe 1 lists only the parent folder's file: it ran before any pointer file. The clause the finding needs, that none of the six lists an `AGENTS.md` above checkout P, holds for all six.
- Solution: The line of F11 says that probe 1 lists only the parent folder's file, that the other five list it and files of checkout P, and that none of the six lists an `AGENTS.md` above checkout P. The Processing Log has a correction entry.
- Current-tree verification: A script over the Q3 answers of probes 1, 2, 7, 8, 11 and 12 in `evidence/m4-probe-answers.md` printed, per probe, whether the answer names the parent folder's file and whether it names a file of checkout P: both for probes 2, 7, 8, 11 and 12, and the parent folder's file only for probe 1.
- Resolution reference: `docs(pr-reviews): [#2904] say what probe 1 lists in the review audit of PR #2920`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2920#discussion_r4604513267

### F15 - Record the rebases in the Processing Log

- PR number: 2920
- Source review ID: 5953723621
- Reviewer finding ID: F26
- Source URL: https://github.com/torrust/torrust-tracker/pull/2920#discussion_r4611573587
- Concern: Two rebases set every committer time to 2026-11-18 09:02, so the `git log --format=%cI` the log entries name no longer prints the times they give, and the review threads cite commit ids the rebases replaced. Nothing in the log names the rebases.
- Solution: A Processing Log entry names both rebases with their times and bases, the EPIC conflict that required the first, and the `git log --format=%aI` form that prints the times of the earlier entries; it says the ids cited in the threads are from before the rebases and that each commit's subject identifies it.
- Current-tree verification: `git log --format=%aI` over the branch printed 18:00:55Z, 18:35:10Z, 18:58:46Z and 19:13:28Z for four of the six commit times the log entries before the rebase entry give, the same times the entries state. The round 1 entry gives the other two, 17:48:20Z and 17:51:07Z; they were not checked then. F16 corrects this line.
- Resolution reference: `docs(pr-reviews): [#2904] record the two rebases in the review audit of PR #2920`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2920#discussion_r4611619528

### F16 - The verification line of F15 checked four of the six commit times

- PR number: 2920
- Source review ID: 5953969566
- Reviewer finding ID: F27
- Source URL: https://github.com/torrust/torrust-tracker/pull/2920#discussion_r4611784133
- Concern: The Current-tree verification line of F15 spoke of "the four commits whose times the log entries give", but the entries before the rebase entry give six: the round 1 entry also gives 17:48:20Z and 17:51:07Z. The reviewer checked those two and they match, so the claim of the rebase entry holds; the line should say what ran.
- Solution: The line of F15 says it checked four of the six times and names the other two. Submitted before the merge and unprocessed when it merged: five rounds of fixes were used, the maintainer chose to merge with it open, and approved this follow-up after the merge.
- Current-tree verification: `git log --format='%aI %s' d8e14450d..68b866938^2` on `develop` prints 17:48:20Z and 17:51:07Z for `build(container): [#2904] keep the pointer CLAUDE.md files out of the build context` and `docs(pr-reviews): [#2904] keep finding F20 of PR #2905 open until its follow-up merges`, and the four times the line names, so all six match.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2920#discussion_r4613226963
- Follow-up PR URL: https://github.com/torrust/torrust-tracker/pull/2942
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2920#discussion_r4613226963

### F17 - The ellipsis in probe 3 is not among the replacements the header lists

- PR number: 2920
- Source review ID: 5954232676
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2920#discussion_r4611977896
- Concern: The header of `evidence/m4-probe-answers.md` says the outputs are verbatim except for four path fragments, but two paths in probe 3 start with `.../torrust-tracker/`, which is neither one of the four placeholders nor described.
- Solution: The ellipsis is the probe's own answer text, so the "verbatim" claim holds; a note in the header now says so, so a reader can tell it from a replacement.
- Current-tree verification: the raw output of probe 3, kept locally and not published, contains `.../torrust-tracker/packages/AGENTS.md` in its `result` field; the header of the answers file has the note.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2920#discussion_r4613227472
- Follow-up PR URL: https://github.com/torrust/torrust-tracker/pull/2942
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2920#discussion_r4613227472

### F18 - The same comment on the ellipsis, posted twice

- PR number: 2920
- Source review ID: 5954232676
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2920#discussion_r4611977966
- Concern: Same text as F17, posted as a second comment on the same lines in the same review.
- Solution: Fixed by the fix of F17.
- Current-tree verification: As for F17.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2920#discussion_r4613228009
- Follow-up PR URL: https://github.com/torrust/torrust-tracker/pull/2942
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2920#discussion_r4613228009

### F19 - `last-updated-utc` is not quoted in the manual verification evidence

- PR number: 2920
- Source review ID: 5954232676
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2920#discussion_r4611978031
- Concern: An unquoted timestamp in YAML front matter can be parsed as a date instead of a string, unlike the quoted field of `ISSUE.md`.
- Solution: The value is quoted.
- Current-tree verification: `sed -n 4p` on `manual-verification-evidence.md` prints `last-updated-utc: "2026-11-17 18:34"`; the pre-commit frontmatter check passed.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2920#discussion_r4613228532
- Follow-up PR URL: https://github.com/torrust/torrust-tracker/pull/2942
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2920#discussion_r4613228532

## Processing Log

- 2026-11-17 17:55 UTC - Started audit.
- 2026-11-17 17:55 UTC - Round 1: the approving review `5943570147` of `da2ce7` (submitted 17:42:25Z) with eight findings and Copilot's review `5943581377` (submitted 17:43:14Z, forge `submitted_at`) with two. The `ACK` comments of `josecelano` (17:36:20Z and 17:43:52Z) and `da2ce7` (17:42:30Z, forge `created_at`) name the head these reviews read; the fixes move the head past them. The fixes were committed between 17:48:20Z and 17:51:07Z (`git log --format=%cI`, in UTC) and pushed; `git ls-remote` showed the fork branch at the local head at 17:53:45Z (`date -u`). The description was edited at 17:54:15Z, and the ten replies were posted between 17:54:17Z and 17:54:39Z (forge `created_at`), after the push.
- 2026-11-17 17:55 UTC - Two findings of round 1 are answered and not processed: the reviewer's F8 (comment `4603463689`) and Copilot's F2 (comment `4603473174`), both about the missing post-merge approval URL in the audit record of pull request 2905. The field needs a durable comment by the maintainer, who has been asked for it. They have no row yet and their threads stay unresolved. Copilot's suggestion to use the reply `4602755221` is not followed: the contributor account wrote that reply.
- 2026-11-17 17:56 UTC - `validate-audit-record.py --pr-number 2920 --base torrust/develop`, with its `gh` calls made through `gh-bot`, exited 0 (8 rows, 0 failures). Resolved the eight threads of F1 to F7 and F9 by their thread IDs, each with the contributor's reply as its last comment and its fix on the pull request. A GraphQL refresh showed 2 unresolved threads: the two about the approval URL, which wait for the maintainer's comment.
- 2026-11-17 18:04 UTC - The maintainer posted the approval comment on pull request #2905 at 17:59:16Z (forge `created_at`). The two findings left from round 1 are recorded as F8 and F10. Their fix was committed at 18:00:55Z (`git log --format=%cI`, in UTC) and pushed; `git ls-remote` showed the fork branch at the local head at 18:03:26Z (`date -u`). Each thread already had a reply from 17:54 that said the fix was pending; the two replies that name the fix were posted at 18:03:39Z and 18:03:42Z (forge `created_at`), after the push.
- 2026-11-17 18:04 UTC - `validate-audit-record.py --pr-number 2920 --base torrust/develop`, with its `gh` calls made through `gh-bot`, exited 0 (10 rows, 0 failures). Resolved the two threads of F8 and F10 by their thread IDs; a GraphQL refresh showed 0 unresolved threads.
- 2026-11-17 18:39 UTC - Round 2: the approving review `5944190011` of `da2ce7` (submitted 18:21:14Z, forge `submitted_at`) with two nits, recorded as F11 and F12. The reviewer's `ACK` comment (18:21:18Z, forge `created_at`) names the head it read; the fix moves the head past it. The contributor (maintainer of the repository) chose to fix them in this pull request. The fix was committed at 18:35:10Z (`git log --format=%cI`, in UTC) and pushed; `git ls-remote` showed the fork branch at the local head at 18:38:31Z (`date -u`). The two replies were posted at 18:38:46Z and 18:38:48Z (forge `created_at`), after the push.
- 2026-11-17 18:39 UTC - `validate-audit-record.py --pr-number 2920 --base torrust/develop`, with its `gh` calls made through `gh-bot`, exited 0 (12 rows, 0 failures). Resolved the two threads of F11 and F12 by their thread IDs; a GraphQL refresh showed 0 unresolved threads.
- 2026-11-17 18:58 UTC - Correction to the Current-tree verification lines of F11 and F12, after the review `5944671302` of `da2ce7`. F12 gave the counts 15 and 7, taken before the fix added one line with each name; at the fix they are 16 and 8. F11 cited a `grep -c` for a local path that prints 0 whatever the probes answered, because every path in the answers file is replaced, and it named a local folder the answers file withholds; it now cites the Q3 answers of the six probes of checkout P, read in the file.
- 2026-11-17 19:01 UTC - Round 3: the approving review `5944671302` of `da2ce7` (submitted 18:55:59Z, forge `submitted_at`) with one nit, recorded as F13. The reviewer's `ACK` comment (18:56:04Z, forge `created_at`) names the head it read; the fix moves the head past it. The fix was committed at 18:58:46Z (`git log --format=%cI`, in UTC) and pushed; `git ls-remote` showed the fork branch at the local head before the reply. The reply was posted at 19:01:47Z (forge `created_at`), after the push.
- 2026-11-17 19:02 UTC - `validate-audit-record.py --pr-number 2920 --base torrust/develop`, with its `gh` calls made through `gh-bot`, exited 0 (13 rows, 0 failures). Resolved the thread of F13 by its thread ID; a GraphQL refresh showed 0 unresolved threads.
- 2026-11-17 19:13 UTC - Second correction to the Current-tree verification line of F11, after the review `5944877623` of `da2ce7`. The corrected line said each of the six answers of checkout P lists the parent folder's file and files of checkout P; probe 1 lists only the parent folder's file, because it ran before any pointer file existed. The line now says so. The reply of round 3 on the thread of F13 repeated the wrong wording.
- 2026-11-17 19:16 UTC - Round 4: the approving review `5944877623` of `da2ce7` (submitted 19:10:53Z, forge `submitted_at`) with one nit, recorded as F14. The reviewer's `ACK` comment (19:10:57Z, forge `created_at`) names the head it read; the fix moves the head past it. The fix was committed at 19:13:28Z (`git log --format=%cI`, in UTC) and pushed; `git ls-remote` showed the fork branch at the local head before the reply. The reply was posted at 19:16:20Z (forge `created_at`), after the push.
- 2026-11-17 19:17 UTC - `validate-audit-record.py --pr-number 2920 --base torrust/develop`, with its `gh` calls made through `gh-bot`, exited 0 (14 rows, 0 failures). Resolved the thread of F14 by its thread ID; a GraphQL refresh showed 0 unresolved threads.
- 2026-11-18 09:21 UTC - Two rebases onto `develop`, each keeping every commit. The first, at 2026-11-17 21:30 UTC onto `d24297c23` (the merge of pull request #2922), was required: that pull request changed the same two places of the EPIC file as this one, `last-updated-utc` and the end of its Progress Log. The resolution keeps the later `last-updated-utc` and orders the log lines by time. The second, at 2026-11-18 09:02 UTC onto `d8e14450d` (the merge of pull request #2932), brings in the fix for the nightly toolchain failure of the `OS Compatibility` check, unrelated to this pull request; `git range-diff` showed every commit unchanged. Each rebase reset the committer time of every commit, so `git log --format=%cI` now prints 2026-11-18 09:02 for all of them. The commit times given in the entries above are author times: `git log --format=%aI` prints them. The commit ids cited in the review threads and replies name commits from before the rebases; each of those commits keeps its Conventional Commit subject, which identifies it on the current branch.
- 2026-11-18 09:24 UTC - Round 5: the approving review `5953723621` of `da2ce7` (submitted 09:19:55Z, forge `submitted_at`) with one suggestion, recorded as F15; Copilot's review `5953709053` (09:19:03Z) on the same head recommends approval with no findings. The reviewer's `ACK` comment (09:19:59Z, forge `created_at`) names the head it read; the fix moves the head past it. The fix is the entry above, committed at 09:20:49Z (`git log --format=%aI`, in UTC) and pushed; `git ls-remote` showed the fork branch at the local head at 09:23:46Z (`date -u`). The reply was posted at 09:23:58Z (forge `created_at`), after the push. This is the fifth round of fixes on this pull request.
- 2026-11-18 09:25 UTC - `validate-audit-record.py --pr-number 2920 --base torrust/develop`, with its `gh` calls made through `gh-bot`, exited 0 (15 rows, 0 failures). Resolved the thread of F15 by its thread ID; a GraphQL refresh showed 0 unresolved threads.
- 2026-11-18 11:33 UTC - Round 6, after the merge. Pull request #2920 merged at 2026-11-18 09:54:45Z (forge `mergedAt`) with four threads open: the approving review `5953969566` of `da2ce7` (submitted 09:37:42Z, forge `submitted_at`) with one nit, recorded as F16, and Copilot's review `5954232676` (09:53:37Z) with three comments, recorded as F17 to F19 (F18 repeats F17). The reply to F16 was posted at 09:53:15Z, before the merge, after the maintainer chose to merge with the nit open. The first replies to F17, F18 and F19 were posted at 09:55:38Z, 09:55:41Z and 09:55:43Z, after the merge and before any durable approval: a slip, because `process-pr-review` (Reviews Submitted After Merge, step 2) allows no reply before the approval comment exists. Those replies only said what the next pull request would do. The maintainer's approval comment was created at 11:26:15Z and is recorded in Ownership. The fixes are in the follow-up pull request #2942, branched from `develop`; its two evidence fixes were pushed (`git ls-remote` showed the fork branch at the local head at 11:32:34Z, `date -u`) before the replies that name it, posted between 11:32:59Z and 11:33:07Z (forge `created_at`). The fix of F16 is in this record, which was not pushed yet when its reply said it was fixed: a second slip, of the order push, then reply. It was pushed right after, with this entry. The rows stay `FOLLOW_UP`/`OPEN` until it merges.
- 2026-11-18 11:52 UTC - Correction to the round 6 entry, after the review `5956009614` of `da2ce7` on pull request #2942. Its sentence "It was pushed right after, with this entry" was written before the push it states, in the commit that the push carried. The push did happen: `git ls-remote` showed the fork branch at the head `docs(pr-reviews): [#2904] record the late review of PR #2920 and correct the verification of F15` at 11:36:42Z (`date -u`). The entry's last words, "until it merges", mean until pull request #2942 merges.

<!-- Copied verbatim into each audit record. -->

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
