---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2886 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2886.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: https://github.com/torrust/torrust-tracker/pull/2886#issuecomment-6926076793 (comment created 2026-11-16 19:00 UTC by `josecelano-bot` on behalf of the maintainer, who decided in chat at about 17:25 UTC to merge as is and leave F3 with no change; review 5924308761 was submitted at 17:20 UTC and the PR merged at 17:34 UTC)

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

F1 and F2 are in da2ce7's review 5923415439, in that source order, and keep his IDs. It was submitted as `APPROVED`; GitHub dismissed it at 16:30:15 UTC when the fixes were pushed, so its state is now `DISMISSED`.

F3 is the one finding in da2ce7's review 5924308761 and keeps his ID. His review 5924034380 between them had no finding; GitHub dismissed it at 17:13:12 UTC when a merge of `develop` was pushed. Review 5924308761 is `APPROVED`, the state the PR merged with.

Copilot's review 5923297015 has no finding: Copilot did not review the PR because the diff exceeds its limit of 20,000 changed lines.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2886-f1 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2886-f2 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2886-f3 | Human | Minor | metadata | ORIGINAL | NO_ACTION | RESOLVED |

## Finding Details

### F1 - Cite the tested commit by its subject in the evidence

- PR number: 2886
- Source review ID: 5923415439
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2886#discussion_r4588360262
- Concern: The manual-verification evidence cited the PR-branch commit of the tested state by its id four times, which a rebase before merge would leave dangling; `open-pull-request` forbids PR-branch commit ids in durable references.
- Solution: The T9, V1 and V5 sections name the tested state by its subject, `docs(adrs): [#2866] record the move of the skills to .claude/skills`, the T8 commit.
- Current-tree verification: A search of the issue folder for the cited id finds nothing, and no other commit of the branch is cited by id in the issue folder, the ADRs or the skills.
- Resolution reference: docs(issues): [#2866] cite the tested commit by its subject in the evidence
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2886#discussion_r4588463986

### F2 - Cross-link the ADR and the skill layout rules

- PR number: 2886
- Source review ID: 5923415439
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2886#discussion_r4588360280
- Concern: The new ADR named `skills.rs` only in its frontmatter, and the module doc of `skills.rs` did not name the ADR, against `create-adr` Step 3.5.
- Solution: The ADR has an "Affected Code" section that links `contrib/dev-tools/checks/frontmatter-validator/src/skills.rs`, and the module doc of `skills.rs` names ADR `20261116082711_store_agent_skills_flat_in_claude_skills.md`.
- Current-tree verification: The "Affected Code" section of the ADR and the module doc at the top of `skills.rs`.
- Resolution reference: docs(adrs): [#2866] cross-link the skills ADR and the skill layout rules
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2886#discussion_r4588464536

### F3 - Drop the closing keyword from a merge message

- PR number: 2886
- Source review ID: 5924308761
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/2886#discussion_r4589058022
- Concern: The message of the hand-resolved merge of `develop` said `develop` "pointed its related artifact at the closed #2850 spec"; `closed #2850` is a closing keyword, which `open-pull-request` asks to reword before merge.
- Solution: No change, by the maintainer's decision to merge as is: rewording the pushed merge commit needs a force push, and #2850 was already closed, so the keyword acts on nothing.
- Current-tree verification: Issue #2850 is closed, and its archived spec says `status: done`.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2886#issuecomment-6926076793; https://github.com/torrust/torrust-tracker/pull/2886#discussion_r4589120623
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2886#discussion_r4589120623

## Processing Log

- 2026-11-16 16:14 UTC - Review 5923297015 (Copilot, `COMMENTED`) submitted on the head commit `Merge torrust/torrust-tracker develop into 2866-move-skills-to-claude-skills`: no review, because the diff exceeds Copilot's limit of 20,000 changed lines; no finding. Reported to the maintainer, who decides whether Copilot's review is needed.
- 2026-11-16 16:21 UTC - Review 5923415439 (da2ce7, `APPROVED`) submitted on the same head: one Minor and one Nit finding, none blocking, recorded here as F1 and F2.
- 2026-11-16 16:30 UTC - Pushed the two fix commits and a commit that sets the spec's `related-pr` (committed 16:24 to 16:26 UTC; push log time 16:30:13 UTC); the push dismissed review 5923415439 (timeline event `review_dismissed`, 16:30:15 UTC). Replied to the two threads (16:30:28 and 16:30:30 UTC) and resolved them (16:30:42 UTC); a GraphQL refresh shows no unresolved thread.
- 2026-11-16 16:35 UTC - Merged `develop` into the branch (merge commit made 16:35:49 UTC) after GitHub reported a conflict; no manual change was needed.
- 2026-11-16 17:01 UTC - Review 5924034380 (da2ce7, `APPROVED`) submitted on that merge head: no finding.
- 2026-11-16 17:13 UTC - Merged `develop` again (merge commit made 17:09:26 UTC), resolving a conflict in the frontmatter of the archived #2838 spec by hand; the push dismissed review 5924034380 (timeline event `review_dismissed`, 17:13:12 UTC).
- 2026-11-16 17:20 UTC - Review 5924308761 (da2ce7, `APPROVED`) submitted on the hand-resolved merge head: one Minor finding, recorded here as F3.
- 2026-11-16 17:26 UTC - The maintainer decided in chat, at about 17:25 UTC, to merge as is; the decision is recorded in the comment named under Ownership. Replied to the F3 thread (17:26:20 UTC) and resolved it (17:26:24 UTC); a GraphQL refresh shows no unresolved thread. No further push.
- 2026-11-16 17:34 UTC - The maintainer merged PR #2886 (17:34:52 UTC). This record's last entries are added by the #2866 close-out PR.

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
