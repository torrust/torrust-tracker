---
pr-number: 2935
pr-url: https://github.com/torrust/torrust-tracker/pull/2935
last-updated-utc: "2026-11-18 10:55"
---

<!-- skill-link: process-pr-review -->

# PR #2935 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2935.

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

Round 1 covers two reviews of the first commit of the PR, `docs(issues): [#2583] make the #1347 publish prerequisite a per-crate testing gate`, which was the head when both were submitted. Review 5954581202 (da2ce7, APPROVED) has six inline findings (F1 to F6) and one finding in its body (F7, on the PR description, which has no thread). Review 5954617412 (Copilot, COMMENTED, a lite review) has three inline findings (F8 to F10) without a severity bracket. The audit severity scale has no medium level, so this audit maps Copilot's `Medium severity` badge to `Minor (inferred)` and its `Low severity` badge to `Nit (inferred)`, as the PR #2929 audit does.
Round 2: review 5954958077 (da2ce7, APPROVED) of the head that round 1 pushed has two inline findings on this record and one finding in its body on the PR description. The reviewer numbered them F8, F9 and F10, which this audit already uses, so they are F11, F12 and F13 here, with the reviewer's ID in each detail entry.
Round 3: review 5955142092 (da2ce7, APPROVED) of the head that round 2 pushed has one inline finding on this record, numbered F11 by the reviewer and F14 here. Copilot review 5955142083 (COMMENTED, of the head that round 1 pushed) reports no open issue and makes no row.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2935-f1 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2935-f2 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2935-f3 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2935-f4 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2935-f5 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2935-f6 | Human | Nit | correctness | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2935-f7 | Human | Nit | documentation | ORIGINAL | FIXED | NON_RESOLVABLE |
| F8 | review-finding:pr-2935-f8 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-2935-f9 | Copilot | Minor (inferred) | correctness | RE_RAISE_OF:F6 | NO_ACTION | SUPERSEDED |
| F10 | review-finding:pr-2935-f10 | Copilot | Nit (inferred) | formatting | ORIGINAL | NO_ACTION | SUPERSEDED |
| F11 | review-finding:pr-2935-f11 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F12 | review-finding:pr-2935-f12 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F13 | review-finding:pr-2935-f13 | Human | Nit | documentation | ORIGINAL | FIXED | NON_RESOLVABLE |
| F14 | review-finding:pr-2935-f14 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - "Always in checklist order" contradicts the dependency-only gate

- PR number: 2935
- Source review ID: 5954581202
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2935#discussion_r4612229665
- Concern: "Checklist order" in the prerequisites intro, T5 and the procedure lead-in read as row or level order, while the spec gates a crate only on its own testing gate and on its dependencies and dev-dependencies being `PUBLISHED`. Under level order the shortest-path claim for #2594 would not hold.
- Solution: The prerequisites intro, the In Scope bullet, the procedure lead-in and T5 say "dependency order (every dependency and dev-dependency first)". T5 adds that this is not row or level order, and the checklist intro says the row and level numbers only present the order.
- Current-tree verification: `grep -n 'checklist order' docs/issues/open/2583-1669-first-publish-of-workspace-crates/ISSUE.md` finds nothing; `grep -c 'every dependency and dev-dependency first' docs/issues/open/2583-1669-first-publish-of-workspace-crates/ISSUE.md` prints 5 (four spec lines and the round 1 progress-log entry).
- Resolution reference: `docs(issues): [#2583] say dependency order instead of checklist order`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2935#discussion_r4612343656

### F2 - The `primitives` state line omits #2914, which EPIC #2912 requires before `primitives` 4.0.0

- PR number: 2935
- Source review ID: 5954581202
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2935#discussion_r4612229686
- Concern: EPIC #2912 lists SI-2 (#2914) as "Must land before `primitives` 4.0.0 is published" and SI-5 (#2917) the same for `configuration` 4.0.0. The spec named neither, so "`primitives` still waits for prerequisites 3 and 5" read as the complete list.
- Solution: The `primitives` state line names #2914 as a constraint that comes from EPIC #2912, not a prerequisite decided in #2583, and the pre-publish notes of rows 1 and 6 name #2914 and #2917.
- Current-tree verification: `grep -n '2914\|2917' docs/issues/open/2912-remove-source-locations-from-errors/EPIC.md` shows both "Must land before" notes (rows 2 and 5); `grep -c '#2914' docs/issues/open/2583-1669-first-publish-of-workspace-crates/ISSUE.md` prints 3 and `grep -c '#2917' docs/issues/open/2583-1669-first-publish-of-workspace-crates/ISSUE.md` prints 2 (both counts include the round 1 progress-log entry). `gh issue view` reported #2914 and #2917 as open.
- Resolution reference: `docs(issues): [#2583] note the EPIC #2912 constraints on primitives and configuration 4.0.0`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2935#discussion_r4612344228

### F3 - Row 1 cites only #2824, but the EPIC #1347 table also holds #2876 for `primitives`

- PR number: 2935
- Source review ID: 5954581202
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2935#discussion_r4612229705
- Concern: The column definition did not say whether bug and follow-up rows of the EPIC #1347 table count, which crate a multi-crate row belongs to, or that the CI rows belong to none, so the column could not be recomputed by its own rule.
- Solution: Row 1 and the state list cite #2824 and #2876. The column legend says a crate's sub-issues are the rows about that package's own tests and their test-support follow-ups, and that the bug row #2838 and the coverage-tooling rows #2222, #2301 and #2361 belong to no crate. No gate value changed. The reply says the rule is the author's wording and that the maintainer may confirm it with OQ1.
- Current-tree verification: `grep -n 'How the rows of that table are counted' docs/issues/open/2583-1669-first-publish-of-workspace-crates/ISSUE.md` finds the legend paragraph; `grep -c '2876-1347-peer-id-test-helper' docs/issues/open/2583-1669-first-publish-of-workspace-crates/ISSUE.md` prints 1 (row 1).
- Resolution reference: `docs(issues): [#2583] list #2876 for primitives and say how #1347 rows are counted`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2935#discussion_r4612344719

### F4 - "Published for the first time" is narrower than the decision it reports

- PR number: 2935
- Source review ID: 5954581202
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2935#discussion_r4612229738
- Concern: The EPIC #1347 paragraph said a crate may be "published for the first time", which leaves the next publish of the three already published crates ungated when read alone.
- Solution: The paragraph says "may be published under #2583 (a first publish, or the next publish of an already published crate) once ...".
- Current-tree verification: `grep -n 'next publish of an already published crate' docs/issues/open/1347-overhaul-packages-testing/EPIC.md` finds the paragraph; `grep -n 'for the first time' docs/issues/open/1347-overhaul-packages-testing/EPIC.md` finds nothing.
- Resolution reference: `docs(issues): [#1347] cover the next publish of published crates in the gate paragraph`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2935#discussion_r4612345552

### F5 - EPIC #1347 gains a paragraph without a stamp or a Progress Log entry

- PR number: 2935
- Source review ID: 5954581202
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2935#discussion_r4612229748
- Concern: `last-updated-utc` of the EPIC #1347 spec stayed at its previous value and its Progress Log had no entry for the new paragraph, unlike the other two files of the commit.
- Solution: The stamp is "2026-11-18 10:20" and the Progress Log has an entry with the same time for the paragraph.
- Current-tree verification: `grep -n '2026-11-18 10:20' docs/issues/open/1347-overhaul-packages-testing/EPIC.md` finds the frontmatter stamp and the last Progress Log entry.
- Resolution reference: `docs(issues): [#1347] stamp and log the publishing gate paragraph`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2935#discussion_r4612346029

### F6 - Row 12 still says "Dev-dependency of 7 crates below"

- PR number: 2935
- Source review ID: 5954581202
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2935#discussion_r4612229728
- Concern: Row 12 said seven crates while OQ4 says six checklist crates and the root crate, which is what the Dev-depends on column and the manifests show.
- Solution: Row 12 says "Dev-dependency of 6 crates below and of the root crate".
- Current-tree verification: `grep -n '7 crates' docs/issues/open/2583-1669-first-publish-of-workspace-crates/ISSUE.md` finds nothing; `grep -c '6 crates below and of the root crate' docs/issues/open/2583-1669-first-publish-of-workspace-crates/ISSUE.md` prints 1.
- Resolution reference: `docs(issues): [#2583] correct the test-helpers dev-dependent count in row 12`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2935#discussion_r4612345099

### F7 - The description's "rebased on the current develop" predates a merge on `develop`

- PR number: 2935
- Source review ID: 5954581202
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2935#pullrequestreview-5954581202
- Concern: The PR description said the branch is rebased on the current `develop`, but #2920 merged 18 seconds after the first commit, before the PR was opened.
- Solution: The description's Validation section says the first commit was rebased on `develop` at `90a7c7d3f` before the PR was opened and that `develop` has moved since. The fix is outside the repository tree.
- Current-tree verification: `gh pr view 2935 --json body` shows the reworded Validation line and no "current `develop`" wording.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2935#issuecomment-6975743968
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2935#issuecomment-6975743968

### F8 - The dependency wait was stated in terms of the testing gate

- PR number: 2935
- Source review ID: 5954617412
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2935#discussion_r4612258392
- Concern: "A crate whose testing gate is met still waits for dependencies whose gate is not" does not match the rule before it, which blocks on dependencies that are not `PUBLISHED`.
- Solution: The sentence reads "still waits for every dependency and dev-dependency that is not `PUBLISHED`".
- Current-tree verification: `grep -n 'dependencies whose gate is not' docs/issues/open/2583-1669-first-publish-of-workspace-crates/ISSUE.md` finds nothing; `grep -c 'dev-dependency that is not' docs/issues/open/2583-1669-first-publish-of-workspace-crates/ISSUE.md` prints 1.
- Resolution reference: `docs(issues): [#2583] state the dependency wait in terms of PUBLISHED`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2935#discussion_r4612346501

### F9 - Conflicting counts of the crates that dev-depend on `test-helpers`

- PR number: 2935
- Source review ID: 5954617412
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2935#discussion_r4612258508
- Concern: OQ4 says six checklist crates and the row 12 note said seven.
- Solution: No further change: this is the mismatch of F6, already fixed by its commit when the review was processed.
- Current-tree verification: `grep -n '7 crates' docs/issues/open/2583-1669-first-publish-of-workspace-crates/ISSUE.md` finds nothing.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2935#discussion_r4612346967
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2935#discussion_r4612346967

### F10 - "subissue" instead of "sub-issue" in the EPIC #1347 paragraph

- PR number: 2935
- Source review ID: 5954617412
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2935#discussion_r4612258595
- Concern: Copilot asked for "sub-issue" for consistency with the rest of the docs.
- Solution: No change. The EPIC #1347 spec writes "subissue" throughout, including its `## Subissues` heading, so the new paragraph follows the file it is in.
- Current-tree verification: `grep -ci subissue docs/issues/open/1347-overhaul-packages-testing/EPIC.md` prints 53 and `grep -ci sub-issue docs/issues/open/1347-overhaul-packages-testing/EPIC.md` prints 0. The reply says 52: that count was taken before the stamp-and-log commit of F5 added a line, and was already 53 when the reply was posted.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2935#discussion_r4612347402
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2935#discussion_r4612347402

### F11 - Three "Current-tree verification" counts were not the output at the head

- PR number: 2935
- Source review ID: 5954958077
- Reviewer finding ID: F8
- Source URL: https://github.com/torrust/torrust-tracker/pull/2935#discussion_r4612541275
- Concern: The counts recorded for F1 (4), F2 (2 and 1) and F10 (52) were not what the commands print at the pushed head (5, 3 and 2, and 53).
- Solution: The three counts were written from the fix diffs without running the commands. Every command named in a Current-tree verification field of this record was then run on the current tree, and the F1, F2 and F10 entries record the printed values; the F10 entry notes that its reply says 52. The other entries already matched the output.
- Current-tree verification: on the current tree, `grep -c 'every dependency and dev-dependency first'`, `grep -c '#2914'` and `grep -c '#2917'` on the #2583 spec printed 5, 3 and 2, and `grep -ci subissue` on the EPIC #1347 spec printed 53; the `grep -n` and other `grep -c` commands of F1 to F10 printed what their entries say (no match, or 1, or the two `2026-11-18 10:20` lines).
- Resolution reference: `docs(pr-reviews): [#2583] record the verification outputs of the PR 2935 audit at the head`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2935#discussion_r4612566964

### F12 - The F7 concern reversed the timing

- PR number: 2935
- Source review ID: 5954958077
- Reviewer finding ID: F9
- Source URL: https://github.com/torrust/torrust-tracker/pull/2935#discussion_r4612541290
- Concern: The F7 concern said #2920 merged 18 seconds "before that statement was true of the opened PR", which reads as the opposite of what happened: the merge came 18 seconds after the first commit and before the PR was opened.
- Solution: The F7 concern says "#2920 merged 18 seconds after the first commit, before the PR was opened".
- Current-tree verification: `grep -c 'merged 18 seconds after the first commit' docs/pr-reviews/pr-2935-review/PR-REVIEW.md` prints 3 (the F7 concern, this entry's solution and this line, which quotes the pattern); `gh pr view 2935 --json createdAt` printed `2026-11-18T09:54:05Z`.
- Resolution reference: `docs(pr-reviews): [#2583] correct the timing in the F7 concern of the PR 2935 audit`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2935#discussion_r4612567484

### F13 - The description's gate table and consequence line lag the spec

- PR number: 2935
- Source review ID: 5954958077
- Reviewer finding ID: F10
- Source URL: https://github.com/torrust/torrust-tracker/pull/2935#pullrequestreview-5954958077
- Concern: The description's "State of the gate" table cited only #2824 for `primitives`, and its consequence line omitted the #2914 constraint that the spec states since round 1. The description is copied into the merge commit.
- Solution: The table cites #2824 and #2876, and the consequence line says `primitives` also waits for #2914 from EPIC #2912. The fix is outside the repository tree.
- Current-tree verification: `gh pr view 2935 --json body -q .body | grep -c '#2876'` and the same with `'#2914'` are recorded in the Processing Log entry of round 2.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2935#issuecomment-6976144390
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2935#issuecomment-6976144390

### F14 - The F12 verification parenthetical named the wrong lines

- PR number: 2935
- Source review ID: 5955142092
- Reviewer finding ID: F11
- Source URL: https://github.com/torrust/torrust-tracker/pull/2935#discussion_r4612699501
- Concern: The F12 verification count of 3 was right, but its parenthetical attributed one match to the F12 concern, whose wording differs from the pattern; the third match is the verification line itself, which quotes the pattern.
- Solution: The parenthetical names the three matching lines: the F7 concern, the F12 solution and the F12 verification line.
- Current-tree verification: the command quoted in the F12 verification field, run with `-n` at the fix commit, matched lines 151, 217 and 218 of this record: the F7 concern, the F12 solution and the F12 verification line. The round 3 paragraph of the Findings section then added two lines above them, so on the tree with this entry the same command matches lines 153, 219 and 220, the same three lines. This entry does not contain the pattern.
- Resolution reference: `docs(pr-reviews): [#2583] name the lines the F12 verification matches in the PR 2935 audit`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2935#discussion_r4612736513

## Processing Log

- 2026-11-18 10:25 UTC - Started audit. Round 1: review 5954581202 (da2ce7, APPROVED, 10:14:16Z) with findings F1 to F7 and review 5954617412 (Copilot, COMMENTED, 10:16:39Z) with findings F8 to F10, both on the first commit of the PR. Committed one fix per finding for F1 to F6 and F8 (10:16Z to 10:21Z) and a progress-log commit in the #2583 spec (10:23Z). Edited the PR description for F7 and answered it in the PR conversation (10:24Z). F9 re-raises F6 and F10 needs no change. Replied on the nine threads (10:23Z) and resolved them (10:24Z), before pushing, so that one push carries the fixes and this record: the fix commits were local when the replies were posted, and each reply says so.
- 2026-11-18 10:42 UTC - Round 2: review 5954958077 (da2ce7, APPROVED, 10:39:31Z) with findings F11 to F13 (the reviewer's F8 to F10). Ran every command named in the Current-tree verification fields and committed the corrected counts (F11, 10:40:14Z) and the F7 timing (F12, 10:40:59Z). Edited the PR description for F13; afterwards `gh pr view 2935 --json body -q .body | grep -c '#2876'` printed 1 and the same with `'#2914'` printed 2. Replied on the two threads (10:41:35Z and 10:41:38Z), answered F13 in the PR conversation (10:41:41Z) and resolved the two threads (10:42Z), before pushing, as in round 1; each reply says the fix was not pushed yet.
- 2026-11-18 10:55 UTC - Round 3: review 5955142092 (da2ce7, APPROVED, 10:52:08Z) with finding F14 (the reviewer's F11), and Copilot review 5955142083 (COMMENTED, 10:52:07Z) with no open issue. Committed the F14 fix (10:54:06Z), replied on its thread (10:54:59Z) and resolved it (10:55Z), before pushing, as in the earlier rounds; the reply says the fix was not pushed yet.

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
