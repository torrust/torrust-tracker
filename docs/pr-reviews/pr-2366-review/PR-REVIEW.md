---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - "issue #2003"
---

<!-- skill-link: process-pr-review -->

# PR #2366 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/2366>.

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

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| R1 | `review-finding:pr-2366-r1` | Human | Major | documentation | ORIGINAL | FIXED | RESOLVED |
| R2 | `review-finding:pr-2366-r2` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| R3 | `review-finding:pr-2366-r3` | Human | Minor | documentation | ORIGINAL | NO_ACTION | SUPERSEDED |
| R4 | `review-finding:pr-2366-r4` | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### R1 - The Friction Register section describes two index comments

- PR number: 2366
- Source review ID: 5468071770
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2366#discussion_r4228502538>
- Concern: The section said the register is kept in two comments, with part 2 holding the former-number map, the counts and the method, while the live index spans four comments and keeps those in part 3, with a twelfth group and 238 labels; the introduction counted 127 frictions, the counts table of 2026-09-28 read as current, and the issue body mirrors the stale text.
- Solution: The section describes the four comments at index v31 (2026-10-08 13:31 UTC), what each holds and the totals, and says part 3 carries the live counts; the 2026-09-28 table is kept as a dated snapshot, which stays true as later batches change the counts; the bullet on the former-number map names part 3; the introduction drops its count; the issue body is re-derived from the new head.
- Current-tree verification: in `docs/issues/open/2003-overhaul-guardrails-and-automation/EPIC.md`, under "Friction Register", the paragraph reads "The register is kept in four comments on this issue", the bullets name "[Index part 3]" and "[Index part 4]", and the table caption reads "kept as a snapshot"; the phrase "127 recorded frictions" no longer occurs.
- Resolution reference: `docs(issues): [#2003] refresh the friction register section and the checkpoint range`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2366#discussion_r4229235460

### R2 - The checkpoint names orders 1–14

- PR number: 2366
- Source review ID: 5468071770
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2366#discussion_r4228502566>
- Concern: The checkpoint said "(orders 1–14)" while the Subissues table runs to order 15, and the line is new in this pull request.
- Solution: The range is dropped from the checkpoint and from the same phrase in the Delivery Strategy, so neither goes stale with the next row.
- Current-tree verification: in `docs/issues/open/2003-overhaul-guardrails-and-automation/EPIC.md`, under "Workflow Checkpoints", the line reads "- [x] Early-implementation subissues created and listed in the Subissues table"; the phrase "orders 1–14" no longer occurs.
- Resolution reference: `docs(issues): [#2003] refresh the friction register section and the checkpoint range`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2366#discussion_r4229235951

### R3 - The PR #2484 paragraph is not yet true on develop

- PR number: 2366
- Source review ID: 5468071770
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2366#discussion_r4228502576>
- Concern: The paragraph says the PR #2484 retrospective's six proposals are dispositioned in #2278's improvement matrix, which holds only once PR #2494 merges.
- Solution: No change: PR #2494 is approved and acknowledged, and this pull request's description states that it merges after PR #2494, so the sentence is true when this one merges.
- Current-tree verification: in `docs/issues/open/2003-overhaul-guardrails-and-automation/EPIC.md`, under "Friction Register", the paragraph "**PR #2484 review retrospective (2026-10-08).**" is unchanged; the description's Merge order section names PR #2494.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2366#discussion_r4229236279
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2366#discussion_r4229236279

### R4 - The PR description stops at the fourth commit

- PR number: 2366
- Source review ID: 5468071770
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2366#discussion_r4228502591>
- Concern: The description covered four commits and validated the fourth commit's head, without the fifth commit or the re-derived issue body.
- Solution: The description, outside the tree, is replaced: a section for each of the seven commits, a Merge order section, and validation at the new head with the issue-body re-derivation.
- Current-tree verification: the description is not in the tree; the reply that answers this thread links the replacement.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2366#discussion_r4229237173
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2366#discussion_r4229237173

## Processing Log

- 2026-10-09 09:10 UTC - Review 5468071770 (josecelano) requested changes for R1 (Major) and R2 (Minor) and added R3 (Minor) and R4 (Nit), each on its own inline thread.
- 2026-10-09 10:25 UTC - Fixed R1 and R2 in one commit. R3 needs no change: the sentence holds once PR #2494, approved and acknowledged, merges first, which this pull request's description states. R4 is answered by the new description. The replies are drafted for posting after the push.
- 2026-10-09 10:28 UTC - Drafted R3's reply in the prescribed `Superseded by R3:` form. R3 is an original no-change finding, so its reply names itself; the rule's `<FindingId>` has no other referent for that case.

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
