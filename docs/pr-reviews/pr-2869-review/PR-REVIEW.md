---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2869 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2869.

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

F2, R1, F1 and R2 are in Copilot's review 5912836984, in that source order. The IDs are Copilot's own and collide with nothing earlier in this record. The task numbers in the replies and solutions are those of the current spec: the fix for R1 added a task T2 and renumbered the later tasks, so the fixes for F1 and R2 were written as T2 and T5 and are now T3 and T6.

F7, F8, F3, F4, F5 and F6 are in da2ce7's review 5913004050, in that source order. It was submitted as `APPROVED`; GitHub dismissed it at 21:36:23 UTC when the fixes were pushed, so its state is now `DISMISSED`. His F1 and F2 collide with Copilot's F1 and F2 above, so they are numbered F7 and F8, the next free audit-local IDs, and his IDs are kept as the reviewer finding IDs; his F3 to F6 collide with nothing and keep their IDs.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F2 | review-finding:pr-2869-f2 | Copilot | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| R1 | review-finding:pr-2869-r1 | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F1 | review-finding:pr-2869-f1 | Copilot | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| R2 | review-finding:pr-2869-r2 | Copilot | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2869-f7 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2869-f8 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2869-f3 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2869-f4 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2869-f5 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2869-f6 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F2 - Require the validator to detect a missing redirect stub

- PR number: 2869
- Source review ID: 5912836984
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2869#discussion_r4580157972
- Concern: Deleting only one legacy stub left the other stubs and the index valid under the planned rules, so the checks could pass while historical links broke, against AC5.
- Solution: `.github/skills/README.md` inventories the 53 original paths, and the validator requires the stub files and the inventory to match exactly; a skill added after the move is not inventoried and needs no stub. AC5 and M5 cover deleting only a stub. The sweep for the same kind of gap found that a later rename or removal of an inventoried skill had no defined stub; inventory rows and stubs are now never deleted, a renamed skill's stub links to the new name, and a removed skill's stub says so.
- Current-tree verification: The stub bullets under "Validator rules after the move", the T4 row, AC5 and M5 in the spec.
- Resolution reference: docs(issues): [#2866] require a stub for every inventoried old skill path
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2869#discussion_r4580200870

### R1 - Apply the bug workflow to the discovery failure

- PR number: 2869
- Source review ID: 5912836984
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2869#discussion_r4580157988
- Concern: The Background describes broken skill discovery, so the Bug-Fix Process and Regression Test Strategy sections could not be "Not applicable"; the plan needed a real-repository reproduction, a justified regression boundary and a like-for-like recheck, with the results in `manual-verification-evidence.md`.
- Solution: Following `fix-bug`, the Background explains the bug in plain terms; the reproduction ran in a fresh clone of `develop` at `abd7a10ff` (M0, Reproduced: no project skill listed, `/create-issue` not run as a command) and is recorded in the new `manual-verification-evidence.md` (V0); a placement-rule regression task (new T2) is proven red before the move, and M1 and M2 repeat M0's commands after it. The later tasks are renumbered T3 to T9, and AC10 covers the placement rule.
- Current-tree verification: The "Bug-Fix Process" and "Regression Test Strategy" sections, the T2 row, M0 to M2 and AC10 in the spec; V0 in `manual-verification-evidence.md`.
- Resolution reference: docs(issues): [#2866] follow the bug workflow for the skill discovery defect
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2869#discussion_r4580201034

### F1 - Allow depth-dependent script changes in the move commit

- PR number: 2869
- Source review ID: 5912836984
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2869#discussion_r4580157996
- Concern: `run-tracker-locally/scripts/validate-skill-links.sh` walks six folders up to the repository root, so the rule that moved files change only in their links would leave it broken after the move.
- Solution: The move task (now T3), its commit point and the risk allow depth-dependent path changes; the reference inventory names the script; M3 runs it from the new path. A search of the moved scripts, tests and manifest for `..` path walks, `dirname`, `__file__` and manifest paths found no other file that resolves a path by depth.
- Current-tree verification: `bash .github/skills/dev/environment-setup/run-tracker-locally/scripts/validate-skill-links.sh` printed `Skill links validation passed` on the current tree; the T3 row, commit point, risk and M3 in the spec.
- Resolution reference: docs(issues): [#2866] allow depth-dependent path fixes in the move commit
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2869#discussion_r4580201146

### R2 - Include the required test-design review checkpoints

- PR number: 2869
- Source review ID: 5912836984
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2869#discussion_r4580158008
- Concern: The validator task adds unit tests, but its commit policy left out the progressive test-design review the issue template and `create-issue` require.
- Solution: The validator task (now T6) names `write-unit-test`, a recorded prose-first Arrange-Act-Assert review after each passing increment and before the next, one commit per reviewed increment, and a maintainer-review stop after the last increment before the final verification and the pull request. The paragraph under Commit Points spells the review out; when the fix for R1 added T2, which also produces tests, the paragraph was widened to cover T2 and T6.
- Current-tree verification: The T6 row and its commit point, the T2 commit point, and the test-design paragraph under Commit Points in the spec.
- Resolution reference: docs(issues): [#2866] add the test-design review checkpoints to T5
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2869#discussion_r4580201266

### F7 - The V0 excerpt does not hold what its lead-in says

- PR number: 2869
- Source review ID: 5913004050
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2869#discussion_r4580303237
- Concern: The V0 lead-in promised the category table and the whole Skills table, but the block held two category rows, and the 26-row breakdown and the only-`Project`-row claim rested on a scratch file outside the issue folder.
- Solution: The block holds the category, Memory Files and Skills tables verbatim, with the clone's scratch path shortened; the lead-in says so and that the MCP Tools table is left out; the text names `grep -c 'Project' m0-context.md`, which printed `1`. Two words of the verbatim output are ignored for the spell checker in that file only.
- Current-tree verification: V0 "Observed Result" in `manual-verification-evidence.md`; `linter cspell` and `linter markdown` pass.
- Resolution reference: docs(issues): [#2866] keep the whole V0 /context excerpt in the evidence
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2869#discussion_r4580342071

### F8 - The description-length row does not recompute at `abd7a10ff`

- PR number: 2869
- Source review ID: 5913004050
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2869#discussion_r4580303242
- Concern: The reviewer's recomputation at `abd7a10ff` gave 19,944 characters, mean 376 and 43 of 53 descriptions of 300 or more, against the row's 19,489, 368 and 37.
- Solution: Recomputed on the YAML-parsed `description` values, the total, mean and maximum hold (19,489 / 368 / 724), but the count is 42, not 37: a miscount. The raw frontmatter text, with quotes and escapes, gives about 19,950 / 376 and 43, the reviewer's figures. The spec states the method and both results. Sweeping the table's other measurements found the dropped-descriptions row wrong as well: 7 skills, not 9, all with source `claude.ai sync`, not built-in; fixed in `docs(issues): [#2866] correct the count of skills that lost their descriptions`.
- Current-tree verification: A Python recomputation over `git show abd7a10ff:<path>` for the 53 `SKILL.md` files printed `chars 53 19489 368 724 42`; a raw-text variant printed `raw 19950 376 724 43`; `grep '< 20'` on the saved scratch `/context` output printed seven `claude.ai sync` rows. The context-cost table and the paragraph below it in the spec.
- Resolution reference: docs(issues): [#2866] correct the description count and state how it was measured
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2869#discussion_r4580342183

### F3 - Row 21 has no EPIC Progress Log line

- PR number: 2869
- Source review ID: 5913004050
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2869#discussion_r4580303245
- Concern: The spec commit added row 21 and moved the EPIC's `last-updated-utc`, but the EPIC Progress Log gained no line, unlike row 20.
- Solution: The Progress Log has a line naming #2866, when its row was added and this PR; `last-updated-utc` moved with it.
- Current-tree verification: The last Progress Log line and the frontmatter of the EPIC.
- Resolution reference: docs(issues): [#2866] log row 21 in the EPIC #2003 progress log
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2869#discussion_r4580342305

### F4 - The PR description no longer matches the branch

- PR number: 2869
- Source review ID: 5913004050
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2869#discussion_r4580303250
- Concern: The description named two of the four files, spoke of "both files" and "the commit", and predated the bug workflow.
- Solution: The description is rewritten: all four files, a validation list that matches the branch, and a summary that covers the M0 reproduction, the T2 placement rule, the stub inventory and the renumbered plan.
- Current-tree verification: `gh pr view 2869 --json body` after the edit at 21:36 UTC.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2869#issuecomment-6901750326
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2869#discussion_r4580342476

### F5 - The `.github/` inventory row names the wrong files

- PR number: 2869
- Source review ID: 5913004050
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2869#discussion_r4580303253
- Concern: At `abd7a10ff` the six files are the five agent profiles and `.github/agents/README.md`, which is live; `.github/workflows/AGENTS.md` and one profile link through `../skills/dev/` and are outside the count.
- Solution: The row names the profiles and `README.md`; the count is said to be of the `.github/skills` literal; In Scope names `.github/agents/README.md`; the two relative-link files are listed with the T9 search that covers them.
- Current-tree verification: `git grep -l '\.github/skills' abd7a10ff -- .github ':!.github/skills'` listed the six files; `git grep -n '\.\./skills/' abd7a10ff -- .github ':!.github/skills'` listed the two relative-link files.
- Resolution reference: docs(issues): [#2866] name the right .github files in the reference inventory
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2869#discussion_r4580342601

### F6 - "order 22" names a sub-issue the EPIC does not record

- PR number: 2869
- Source review ID: 5913004050
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2869#discussion_r4580303257
- Concern: No row, draft or recorded decision in the EPIC names the custom-agents migration, and the EPIC orders a sub-issue when its issue is created.
- Solution: The Background calls it a planned EPIC #2003 sub-issue that is not created yet.
- Current-tree verification: `git grep -n 'order 22' -- docs/issues/open/2866-2003-move-skills-to-claude-skills docs/issues/open/2003-overhaul-guardrails-and-automation` prints nothing; this record quotes the finding's title.
- Resolution reference: docs(issues): [#2866] drop the order number of the custom-agents migration
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2869#discussion_r4580342750

## Processing Log

- 2026-11-15 20:41 UTC - Review 5912836984 (Copilot, `COMMENTED`) submitted on the first head of the PR: F2, R1, F1, R2.
- 2026-11-15 20:52 UTC - Started this audit after the five fix commits (committed 20:45 to 20:51 UTC).
- 2026-11-15 20:54 UTC - Pushed the five fix commits (push log time 20:54:18 UTC), then replied to the four threads (reply times 20:54:36 to 20:54:43 UTC); the reply URLs are in the entries.
- 2026-11-15 20:55 UTC - Resolved the four threads after the replies; a GraphQL refresh shows no unresolved thread.
- 2026-11-15 21:25 UTC - Review 5913004050 (da2ce7, `APPROVED`) submitted on the head commit `docs(pr-reviews): [#2866] record the first review of PR #2869`: six findings, none blocking, recorded here as F7, F8, F3, F4, F5 and F6.
- 2026-11-15 21:36 UTC - Pushed the six fix commits (committed 21:29 to 21:33 UTC; push log time 21:36:21 UTC); the push dismissed review 5913004050 (timeline event `review_dismissed`, 21:36:23 UTC). Rewrote the PR description (F4), posted the F4 statement (21:36:45 UTC) and replied to the six threads (21:36:47 to 21:36:58 UTC).
- 2026-11-15 21:38 UTC - Resolved the six threads after the replies; a GraphQL refresh shows no unresolved thread.

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
