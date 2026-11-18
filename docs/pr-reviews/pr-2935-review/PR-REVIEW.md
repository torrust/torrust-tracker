---
pr-number: 2935
pr-url: https://github.com/torrust/torrust-tracker/pull/2935
last-updated-utc: "2026-11-18 10:25"
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
- Concern: The PR description said the branch is rebased on the current `develop`, but #2920 merged 18 seconds before that statement was true of the opened PR.
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

## Processing Log

- 2026-11-18 10:25 UTC - Started audit. Round 1: review 5954581202 (da2ce7, APPROVED, 10:14:16Z) with findings F1 to F7 and review 5954617412 (Copilot, COMMENTED, 10:16:39Z) with findings F8 to F10, both on the first commit of the PR. Committed one fix per finding for F1 to F6 and F8 (10:16Z to 10:21Z) and a progress-log commit in the #2583 spec (10:23Z). Edited the PR description for F7 and answered it in the PR conversation (10:24Z). F9 re-raises F6 and F10 needs no change. Replied on the nine threads (10:23Z) and resolved them (10:24Z), before pushing, so that one push carries the fixes and this record: the fix commits were local when the replies were posted, and each reply says so.

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
