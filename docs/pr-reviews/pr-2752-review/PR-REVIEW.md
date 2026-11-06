---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2752 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2752.

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

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2752-f1 | Copilot | Minor (inferred) | link-integrity | ORIGINAL | NO_ACTION | SUPERSEDED |
| F2 | review-finding:pr-2752-f2 | Copilot | Minor (inferred) | link-integrity | ORIGINAL | NO_ACTION | SUPERSEDED |
| F3 | review-finding:pr-2752-f3 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2752-f4 | Copilot | Minor (inferred) | maintainability | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2752-f5 | Copilot | Minor (inferred) | maintainability | RE_RAISE_OF:F4 | FIXED | RESOLVED |
| F6 | review-finding:pr-2752-f6 | Copilot | Nit (inferred) | metadata | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2752-f7 | Copilot | Nit (inferred) | metadata | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2752-f8 | Copilot | Nit (inferred) | formatting | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-2752-f9 | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-2752-f10 | Copilot | Minor | metadata | ORIGINAL | FIXED | NON_RESOLVABLE |
| F11 | review-finding:pr-2752-f11 | Copilot | Major (inferred) | correctness | ORIGINAL | FIXED | NON_RESOLVABLE |

## Finding Details

### F1 - Keep the stable `issue #2751` reference in the closed SI-23 spec

- PR number: 2752
- Source review ID: 5801499766
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4493457398
- Concern: The closed SI-23 specification lists a free-text `"issue #2751"` entry in `semantic-links.related-artifacts`; the reviewer prefers a resolvable path or URL.
- Solution: No change. The entry is the stable reference form the frontmatter validator accepts, and a path to an `open/` spec breaks when it moves to `closed/`. PR 2698 made the same decision for SI-24 (`review-finding:pr-2698-f1`).
- Current-tree verification: `docs/issues/closed/2449-1488-si-23-migrate-rest-api-test-environment/ISSUE.md` lists `"issue #2751"`; the pre-commit frontmatter validator passed on the commit that introduced it.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4494417164
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4494417164

### F2 - Keep the stable `issue #2751` reference in the closed SI-24 spec

- PR number: 2752
- Source review ID: 5801499766
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4493457497
- Concern: Same concern as F1 for the closed SI-24 specification.
- Solution: No change, for the reason given for F1.
- Current-tree verification: `docs/issues/closed/2450-1488-si-24-migrate-health-check-api-test-environment/ISSUE.md` lists `"issue #2751"`; the frontmatter validator passed.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4494417848
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4494417848

### F3 - Reconcile "not merged yet" with the published 0.5.0 release

- PR number: 2752
- Source review ID: 5801499766
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4493457562
- Concern: The paragraph says torrust-server-lib PR 4 is "not merged yet" and then cites a merge commit and the published 0.5.0 release.
- Solution: Changed the wording to "merged".
- Current-tree verification: `grep -ni "not merged"` over the spec folder finds nothing.
- Resolution reference: docs(issues): [#2751] state that the torrust-server-lib deprecation PR is merged
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4494418352

### F4 - Merge the duplicate Rollback sections

- PR number: 2752
- Source review ID: 5801499766
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4493457639
- Concern: The spec has two `## Rollback` sections, which is redundant and can diverge.
- Solution: Removed the first section and kept the later one, which also notes that the dependency bump can be reverted independently.
- Current-tree verification: `grep -c "^## Rollback" ISSUE.md` prints 1.
- Resolution reference: docs(issues): [#2751] merge the duplicate Rollback sections of the SI-18 spec
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4494418889

### F5 - Merge the duplicate Rollback sections (re-raise of F4)

- PR number: 2752
- Source review ID: 5801499766
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4493457701
- Concern: Same request as F4, raised on the second `## Rollback` heading.
- Solution: Fixed by the same commit as F4.
- Current-tree verification: `grep -c "^## Rollback" ISSUE.md` prints 1.
- Resolution reference: docs(issues): [#2751] merge the duplicate Rollback sections of the SI-18 spec
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4494419433

### F6 - Quote last-updated-utc in the evidence file

- PR number: 2752
- Source review ID: 5801499766
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4493457759
- Concern: The evidence file leaves `last-updated-utc` unquoted while `ISSUE.md` quotes it, so a YAML parser can read it as a timestamp.
- Solution: Quoted the value.
- Current-tree verification: Line 4 of `manual-verification-evidence.md` is `last-updated-utc: "2026-11-06 14:00"`.
- Resolution reference: docs(issues): [#2751] quote the last-updated-utc value in the SI-18 evidence file
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4494419950

### F7 - Remove the duplicate related artifact

- PR number: 2752
- Source review ID: 5801499766
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4493457817
- Concern: `semantic-links.related-artifacts` lists `packages/axum-health-check-api-server/src/server.rs` twice.
- Solution: Removed the second entry.
- Current-tree verification: `grep -c "axum-health-check-api-server/src/server.rs" ISSUE.md` prints 1.
- Resolution reference: docs(issues): [#2751] remove the duplicate related artifact from the SI-18 spec
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4494420438

### F8 - Use one spelling of behavior

- PR number: 2752
- Source review ID: 5801499766
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4493457884
- Concern: The spec writes "Behaviour" in some places and "behavior" in others.
- Solution: Changed the three "behaviour" spellings to "behavior".
- Current-tree verification: `grep -ni behaviour` over the spec folder finds nothing.
- Resolution reference: docs(issues): [#2751] use the American spelling of behavior in the SI-18 spec
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4494420960

### F9 - Update the cleared external eligibility gate

- PR number: 2752
- Source review ID: 5803331264
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4494797047
- Concern: The eligibility gate still marks the `torrust-server-lib` item as blocked on 0.4.0, although the split section and the evidence record the published 0.5.0 release with migration guidance and the support window.
- Solution: Ticked the item and replaced the blocker text with the 0.5.0 guidance. The reviewer's ID F1 collided with audit finding F1, so the audit assigned F9.
- Current-tree verification: the gate item is `[x]` and cites 0.5.0 and its `CHANGELOG.md`; the changelog read from the local torrust-server-lib clone lists migration steps and says the items stay for all 0.5.x releases and are removed in 0.6.0.
- Resolution reference: docs(issues): [#2751] mark the torrust-server-lib gate item complete with the 0.5.0 guidance
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4495030346

### F10 - Record the spec-only branch name

- PR number: 2752
- Source review ID: 5803331264
- Reviewer finding ID: F10
- Source URL: https://github.com/torrust/torrust-tracker/pull/2752#pullrequestreview-5803331264
- Concern: The spec's `branch:` frontmatter uses the suffix-free name reserved for the implementation branch, but the `create-issue` skill requires the `-spec` branch name for the spec-only stage.
- Solution: Set `branch:` to `2751-deprecate-legacy-shutdown-api-spec`.
- Current-tree verification: line 10 of `ISSUE.md` is `branch: "2751-deprecate-legacy-shutdown-api-spec"`, the name of this PR's head branch.
- Resolution reference: docs(issues): [#2751] record the spec-only branch name in the SI-18 spec
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2752#issuecomment-6690075811

### F11 - Resolve the deprecation release dependency cycle

- PR number: 2752
- Source review ID: 5803331264
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2752#pullrequestreview-5803331264
- Concern: #2583 publishes the unpublished crates only after EPIC #1488 closes, but SI-19 is part of that EPIC and requires the public support window to have ended, so the crates cannot publish the promised deprecations before removal.
- Solution: Confirmed against #2583 (prerequisite "EPIC #1488 closed"). The maintainer chose Option A: the five unpublished crates have no public support window, so SI-19 may remove their symbols without waiting; the window applies only to `torrust-server-lib` and the root `HaltSignal` and `NestedServerTask`. Updated SI-18 (Versioning Decisions, requirement 5, risks) and the SI-19 draft, and told the EPIC #1669 coordinator that #2583 needs no change.
- Current-tree verification: `grep -n "support window" ISSUE.md` in SI-18 shows the Versioning Decisions entry; the SI-19 draft start gate names the already-public symbols only.
- Resolution reference: docs(issues): [#2751] resolve the SI-18 and SI-19 release-order cycle
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2752#issuecomment-6690075811

## Processing Log

- 2026-11-06 14:26 UTC - Copilot review 5801499766 submitted with 8 inline findings (F1 to F8); F5 re-raises F4.
- 2026-11-06 15:46 UTC - Five fix commits for F3 to F8 were made (committer times 15:44:22 to 15:46:14 UTC); they were pushed before the replies below.
- 2026-11-06 15:50 UTC - Replied on all 8 threads (the replies were created between 15:49:39 and 15:49:57 UTC after the fixes were pushed).
- 2026-11-06 16:27 UTC - Resolved all 8 threads; each mutation returned `isResolved: true`.
- 2026-11-06 16:30 UTC - Found Copilot review 5803331264 (submitted 16:21 UTC) with one inline finding (F9, reviewer ID F1, renumbered because F1 was taken) and two review-body findings (F10, F11).
- 2026-11-06 16:36 UTC - Fix commits for F9 to F11 were made (committer times 16:31:22 to 16:36:50 UTC) and pushed before the replies below.
- 2026-11-06 16:39 UTC - Replied on the F9 thread (16:39:20 UTC) and posted one PR conversation response covering F9 to F11 (16:39:23 UTC); resolved the F9 thread, which returned `isResolved: true`. F10 and F11 have no thread (`NON_RESOLVABLE`).

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
