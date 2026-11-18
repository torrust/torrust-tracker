---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2951 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2951.

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
| F1 | `review-finding:pr-2951-f1` | Copilot | Suggestion (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Name the sub-issue identifier of 2221 in the problem list

- PR number: 2951
- Source review ID: 5963238031
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2951#discussion_r4619341298
- Concern: In the EPIC problem list, the status line of problem 3 names `#2221` by issue number only, while the other status lines use the roadmap identifier with the number, for example `SI-4 (#2169)`. The reviewer suggests `SI-5 (#2221)` so the problem list can be matched with the roadmap table.
- Solution: Changed it to `SI-5 (#2221)`. The same pattern was in three more places of the list, so they were changed in the same commit: `SI-10 (#2274)` in problem 4, `SI-14 (#2342)` and `SI-15 (#2370)` in problem 5. Problem 7 named `#2414` as `SI-22`, which is the identifier of its parent `#2410`; it is now `SI-22.2 (#2414)`. `#1586` keeps its plain number because the roadmap gives it no SI identifier.
- Current-tree verification: `grep -n -o "SI-[0-9.]* (#2221)\|SI-[0-9.]* (#2274)\|SI-[0-9.]* (#2342)\|SI-[0-9.]* (#2370)\|SI-[0-9.]* (#2414)" docs/issues/closed/1488-overhaul-tracker-shutdown/EPIC.md` prints the five identifiers on lines 115, 119, 124, 124 and 132. `ls docs/issues/closed` shows the folders `2221-1488-si-5-...`, `2274-1488-si-10-...`, `2342-1488-si-14-...`, `2370-1488-si-15-...` and `2414-2410-si-22-2-...`.
- Resolution reference: `docs(issues): [#1488] name the sub-issue identifiers in the EPIC problem list`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2951#discussion_r4619397335

## Processing Log

- 2026-11-18 19:11 UTC - Started audit. Review 5963238031 (Copilot, `COMMENTED`, on commit `1dbf98e63`) has one inline thread and no actionable assertion in its body. The body says Copilot could not run its full agentic review because it did not start before the timeout; the review effort is "Lite".
- 2026-11-18 19:11 UTC - F1 fixed in `docs(issues): [#1488] name the sub-issue identifiers in the EPIC problem list`, committed at 19:08:41Z (`git log --format=%cI`) and pushed at 19:11:05Z (reflog of the remote-tracking branch), before this record was created. The first commit of this subject listed three of the four issue numbers in its body and was amended before the push; only the amended commit was pushed.
- 2026-11-18 19:11 UTC - Replied on the thread of F1 at 19:11:28Z (`created_at` of the reply). `validate-audit-record.py --pr-number 2951`, run with the token of `josecelano-bot`, exited 0 (1 row, 0 failures).
- 2026-11-18 19:11 UTC - Resolved thread `PRRT_kwDOGp2yqc65JacV` as `josecelano-bot`. A GraphQL refresh of the review threads counts 0 unresolved threads.

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
