---
pr-number: 2920
pr-url: https://github.com/torrust/torrust-tracker/pull/2920
last-updated-utc: "2026-11-17 17:56"
---

<!-- skill-link: process-pr-review -->

# PR #2920 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2920.

<!-- Copied verbatim into each audit record after placeholder replacement. -->

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A

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
with its own IDs F1 and F2, which collide with the first review's and are recorded as F9 and F10). F8 and F10 have no row yet; see the Processing Log.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2920-f1` | Human | Minor | maintainability | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2920-f2` | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2920-f3` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2920-f4` | Human | Nit | correctness | ORIGINAL | FIXED | RESOLVED |
| F5 | `review-finding:pr-2920-f5` | Human | Suggestion | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | `review-finding:pr-2920-f6` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | `review-finding:pr-2920-f7` | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F9 | `review-finding:pr-2920-f9` | Copilot | Minor | correctness | RE_RAISE_OF:F7 | FIXED | RESOLVED |

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

## Processing Log

- 2026-11-17 17:55 UTC - Started audit.
- 2026-11-17 17:55 UTC - Round 1: the approving review `5943570147` of `da2ce7` (submitted 17:42:25Z) with eight findings and Copilot's review `5943581377` (submitted 17:43:14Z, forge `submitted_at`) with two. The `ACK` comments of `josecelano` (17:36:20Z and 17:43:52Z) and `da2ce7` (17:42:30Z, forge `created_at`) name the head these reviews read; the fixes move the head past them. The fixes were committed between 17:48:20Z and 17:51:07Z (`git log --format=%cI`, in UTC) and pushed; `git ls-remote` showed the fork branch at the local head at 17:53:45Z (`date -u`). The description was edited at 17:54:15Z, and the ten replies were posted between 17:54:17Z and 17:54:39Z (forge `created_at`), after the push.
- 2026-11-17 17:55 UTC - Two findings of round 1 are answered and not processed: the reviewer's F8 (comment `4603463689`) and Copilot's F2 (comment `4603473174`), both about the missing post-merge approval URL in the audit record of pull request 2905. The field needs a durable comment by the maintainer, who has been asked for it. They have no row yet and their threads stay unresolved. Copilot's suggestion to use the reply `4602755221` is not followed: the contributor account wrote that reply.
- 2026-11-17 17:56 UTC - `validate-audit-record.py --pr-number 2920 --base torrust/develop`, with its `gh` calls made through `gh-bot`, exited 0 (8 rows, 0 failures). Resolved the eight threads of F1 to F7 and F9 by their thread IDs, each with the contributor's reply as its last comment and its fix on the pull request. A GraphQL refresh showed 2 unresolved threads: the two about the approval URL, which wait for the maintainer's comment.

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
