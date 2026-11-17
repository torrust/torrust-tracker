---
pr-number: 2922
pr-url: https://github.com/torrust/torrust-tracker/pull/2922
last-updated-utc: "2026-11-17 18:23"
---

<!-- skill-link: process-pr-review -->

# PR #2922 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2922.

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

Round 1: da2ce7 review 5944086342 (APPROVED) with findings F1 to F4; the reviewer's IDs are this audit's IDs. Copilot review 5944100743 (COMMENTED) with one finding, R1; the reviewer's ID is this audit's ID. R1 asks for the same current-tree change as F2.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2922-f1 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2922-f2 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2922-f3 | Human | Suggestion | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2922-f4 | Human | Suggestion | documentation | ORIGINAL | FIXED | RESOLVED |
| R1 | review-finding:pr-2922-r1 | Copilot | Nit | metadata | RE_RAISE_OF:F2 | NO_ACTION | SUPERSEDED |

## Finding Details

### F1 - The description pairs a closing keyword with an upstream issue reference

- PR number: 2922
- Source review ID: 5944086342
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2922#discussion_r4603826215
- Concern: The description's T2 bullet ended with a closing keyword followed by the upstream `zerocopy` issue reference, which the `open-pull-request` closing-keyword pattern matches. The pull request must not close that issue, and the merge tool copies the description into the merge commit. The specification line carries the same words but files are not parsed for keywords.
- Solution: Edited the description: the T2 bullet now says "a `zerocopy` release with the fix tracked in" the upstream issue, and the T1 bullet says "a change proposed in" the upstream pull request. Swept the branch for the same wording: two specification entries (T2 and the hold) had it and were reworded by the commit `docs(issues): [#2360] reword the upstream references without a closing keyword`.
- Current-tree verification: The closing-keyword pattern of the `open-pull-request` skill, run with `grep -ciE` on the description returned by `gh pr view 2922 --json body` after the edit, printed `0`; run on the specification at the head `docs(issues): [#2360] mark the issue blocked in the #2003 EPIC table`, it printed `0`; the folded commit messages of `torrust/develop..HEAD` had no match.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2922#issuecomment-6957005270
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2922#discussion_r4603914743

### F2 - The three new progress-log entries name the agent but not its model

- PR number: 2922
- Source review ID: 5944086342
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2922#discussion_r4603826227
- Concern: The `create-issue` skill and `docs/templates/ISSUE.md` ask for the form `Agent (model)` in each progress-log entry an AI agent writes. The three new entries gave the agent alone.
- Solution: The three entries now read `Claude Code (Claude Fable 5.2)`. The session changed model between the first checks and the entries, so the T1 entry also says which model ran which check.
- Current-tree verification: `grep -c "UTC - Claude Code (Claude Fable 5.2) - " docs/issues/open/2360-2003-resolve-udp-protocol-empty-enums-allowance-scope/ISSUE.md` prints `4` (the three entries and the later clarification entry); `grep -c "UTC - Claude Code - "` on the same file prints `0`.
- Resolution reference: docs(issues): [#2360] name the model in the new progress-log entries
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2922#discussion_r4603915281

### F3 - The hold entry leaves open what T3 becomes when the issue is reviewed again

- PR number: 2922
- Source review ID: 5944086342
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2922#discussion_r4603826234
- Concern: T2 chose Temporary, whose reason rewrite depends on nothing upstream, yet the issue is `blocked` and T3 stays `TODO`. The plan did not let a reader tell a hold from a dependency, nor say what T3 becomes if a fixed `zerocopy` release exists at the review.
- Solution: Appended a progress-log entry instead of rewriting the hold entry. It says the hold is a deferral, that the rewrite is deferred on purpose, and what T3 becomes in each case: removing A159 if a `zerocopy` release carries the upstream change, otherwise the Temporary rewrite.
- Current-tree verification: `grep -c "The hold is a deferral, not a dependency" docs/issues/open/2360-2003-resolve-udp-protocol-empty-enums-allowance-scope/ISSUE.md` prints `1`.
- Resolution reference: docs(issues): [#2360] say what T3 becomes after the hold
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2922#discussion_r4603915668

### F4 - The EPIC row for this issue still reads TODO while the specification says blocked

- PR number: 2922
- Source review ID: 5944086342
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2922#discussion_r4603826239
- Concern: Outside the diff: order 13 of the #2003 EPIC table kept `TODO` while the specification's frontmatter says `status: blocked`. The table's status values include `BLOCKED`.
- Solution: Order 13 now reads `BLOCKED` with a short note on the hold. The commit also updates the EPIC's `last-updated-utc`.
- Current-tree verification: `grep -c "ISSUE.md\` | BLOCKED | Follow-up of #2347" docs/issues/open/2003-overhaul-guardrails-and-automation/EPIC.md` prints `1`.
- Resolution reference: docs(issues): [#2360] mark the issue blocked in the #2003 EPIC table
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2922#discussion_r4603916069

### R1 - Include the model in the new progress-log attributions

- PR number: 2922
- Source review ID: 5944100743
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2922#discussion_r4603836372
- Concern: The three new entries identify only `Claude Code`; the issue template requires `Agent (model)`. Add the model to these three entries and leave the historical entries unchanged.
- Solution: Same current-tree change as F2, already made by the F2 commit; no further change. Replied with the prescribed `Superseded by` form.
- Current-tree verification: Same commands as F2: `4` entries with the model, `0` without. For the historical entries, `git diff torrust/develop...HEAD -- docs/issues/open/2360-2003-resolve-udp-protocol-empty-enums-allowance-scope/ISSUE.md | grep -c '^[-+]- 2026-09-28'` prints `0`.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2922#discussion_r4603916463
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2922#discussion_r4603916463

## Processing Log

- 2026-11-17 18:23 UTC - Started audit. Round 1: da2ce7 review 5944086342 (submitted 18:14:41Z) and Copilot review 5944100743 (submitted 18:15:35Z). Committed the F2, F1 (specification wording), F3 and F4 fixes as four signed commits and pushed them with `git-bot-push`, ending at `docs(issues): [#2360] mark the issue blocked in the #2003 EPIC table`. Edited the PR description for F1 and posted the PR conversation response (18:22:36Z). Replied to the five threads (18:22:46Z to 18:22:56Z).

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
