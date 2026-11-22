---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2990 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2990>.

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

Round 1 covers two reviews of the first commit of the PR, `docs(issues): [#2583] record that EPIC #1488 is closed in the publish spec`, which was the head when both were submitted. Review 6006219192 (Copilot, COMMENTED, a lite review) has three inline findings (F1 to F3) without a severity bracket. The audit severity scale has no medium level, so this audit maps Copilot's `Medium severity` badge to `Minor (inferred)` and its `Low severity` badge to `Nit (inferred)`, as the PR #2935 audit does. Review 6006237271 (da2ce7, APPROVED) has three inline findings. The reviewer numbered them F1, F2 and F3, which this audit already uses, so they are F4, F5 and F6 here, with the reviewer's ID in each detail entry. Its body has no finding beyond them.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2990-f1 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2990-f2 | Copilot | Nit (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2990-f3 | Copilot | Nit (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2990-f4 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2990-f5 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2990-f6 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The OQ2 row still says EPIC #1488 is expected to close soon

- PR number: 2990
- Source review ID: 6006219192
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2990#discussion_r4653326622
- Concern: The OQ2 row says "EPIC #1488 is expected to close soon" while the same row says it closed and the question is moot.
- Solution: The clause is replaced by "(Asked on 2026-11-18, when EPIC #1488 was still open.)", which keeps the date of the question.
- Current-tree verification: `grep -c 'expected to close soon' docs/issues/open/2583-1669-first-publish-of-workspace-crates/ISSUE.md` prints 0.
- Resolution reference: `docs(issues): [#2583] drop the stale 'expected to close soon' clause from OQ2`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2990#discussion_r4653365798

### F2 - Mixed tenses in the "Other prerequisites" decision row

- PR number: 2990
- Source review ID: 6006219192
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2990#discussion_r4653326638
- Concern: "closes first (met: it closed ...)" mixes tenses in one clause of the decision table.
- Solution: The row has two sentences: the 2026-10-20 decision ("must close first") and its state ("Met: it closed on 2026-11-19"). The reviewer's "closed first" was not used because the first sentence is the decision, not the event.
- Current-tree verification: `grep -c 'must close first. Met: it closed on 2026-11-19' docs/issues/open/2583-1669-first-publish-of-workspace-crates/ISSUE.md` prints 1.
- Resolution reference: `docs(issues): [#2583] separate the #1488 decision from its met state in the decision row`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2990#discussion_r4653365910

### F3 - Awkward sentence about what #2482 and #1488 could change

- PR number: 2990
- Source review ID: 6006219192
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2990#discussion_r4653326648
- Concern: The In Scope bullet "could change them until they closed (both are closed)" is awkward and mixes tenses.
- Solution: The bullet separates the closed EPICs from the ongoing risk of #1347. The "Stale order" risk had the same construction and was reworded in the same commit.
- Current-tree verification: `grep -c 'until they closed' docs/issues/open/2583-1669-first-publish-of-workspace-crates/ISSUE.md` prints 0.
- Resolution reference: `docs(issues): [#2583] reword the two sentences about what #2482 and #1488 could change`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2990#discussion_r4653366011

### F4 - The new section omits the #1488 changes to `configuration` and `primitives`

- PR number: 2990
- Source review ID: 6006237271
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2990#discussion_r4653341139
- Concern: "What EPIC #1488 changed that matters for publishing" reads as a complete list but leaves out SI-20 (#2668, PR #2678: public field `Configuration::application` and two public modules in `torrust-tracker-configuration`, a breaking change) and SI-21 (#2697, PR #2720: public `Readiness` in `torrust-tracker-primitives`).
- Solution: A first bullet "Published checklist crates" lists both, and the lead-in says the list is what is known to matter, not a full API diff, and that T4 checks each crate again.
- Current-tree verification: `grep -n 'pub application' packages/configuration/src/v3_0_0/mod.rs` finds the field and the struct has no `#[non_exhaustive]`; `grep -n 'pub struct Readiness' packages/primitives/src/readiness.rs` finds the type; issues #2668 and #2697 are closed and PRs #2678 and #2720 merged (`gh issue view`, `gh pr view`).
- Resolution reference: `docs(issues): [#2583] list the #1488 changes to the configuration and primitives crates`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2990#discussion_r4653366099

### F5 - SI-26 did change a public API, the root crate's

- PR number: 2990
- Source review ID: 6006237271
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2990#discussion_r4653341145
- Concern: The section says SI-26 consolidated the supervisors "without changing a public API", but PR #2870 removed `JobManager::register_legacy` and `JobManager::legacy_cancellation_token`, renamed `start_ban_cleanup_job` and added `bootstrap::jobs::token_aware_server` in the root crate. The SI-26 spec only claims no change in the unpublished packages.
- Solution: The bullet says SI-26 did not change the public API of a checklist crate and that its public changes are in the root crate. The same sentence in the PR description was corrected (sweep for the same mistake).
- Current-tree verification: `git diff 1c3daab4b^1 1c3daab4b -- src/bootstrap` shows the two removed methods, the rename and `pub mod token_aware_server`.
- Resolution reference: `docs(issues): [#2583] say SI-26 changed the root crate's public API, not a checklist crate's`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2990#discussion_r4653366237

### F6 - Row 14 still says #2482 may change its `Mock*` types

- PR number: 2990
- Source review ID: 6006237271
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/2990#discussion_r4653341149
- Concern: Row 14, outside the diff, is the one place left that treats #2482 as pending.
- Solution: The note reads "#2482, which could change its `Mock*` types, is closed."
- Current-tree verification: `grep -c 'may change' docs/issues/open/2583-1669-first-publish-of-workspace-crates/ISSUE.md` prints 0.
- Resolution reference: `docs(issues): [#2583] say #2482 is closed in the pre-publish note of row 14`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2990#discussion_r4653366387

## Processing Log

- 2026-11-21 15:23 UTC - Started audit. Round 1: review 6006219192 (Copilot, COMMENTED, 15:12:37Z) with findings F1 to F3 and review 6006237271 (da2ce7, APPROVED, 15:16:22Z) with findings F4 to F6, both on the first commit of the PR. Checked each claim on `develop`; all six are valid. Committed one fix per finding (committer times 15:15Z to 15:19Z). A push queued after the F3 commit ran later and delivered the fixes of F1 to F5. Replied on the six threads (15:22Z); the F6 commit was local then, and its reply says so. Edited the PR description for F5 and row 14 (two edits; the first left one garbled sentence, the second corrected it).
- 2026-11-21 15:24 UTC - The audit validator exits 0 (6 rows, 0 failures). Resolved the six threads after it. This record and the F6 fix are committed locally and not pushed yet.

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
