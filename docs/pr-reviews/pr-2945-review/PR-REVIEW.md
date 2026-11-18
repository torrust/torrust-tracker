---
pr-number: 2945
pr-url: https://github.com/torrust/torrust-tracker/pull/2945
last-updated-utc: "2026-11-18 14:08"
---

<!-- skill-link: process-pr-review -->

# PR #2945 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2945.

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

Round 1 is the approving review `5958194168` of `da2ce7` (F1 to F3, reviewer-provided IDs) on the first head; Copilot's
review `5957852837` on the same head recommends approval with no findings and has no row.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2945-f1` | Human | Minor | documentation | ORIGINAL | NO_ACTION | SUPERSEDED |
| F2 | `review-finding:pr-2945-f2` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2945-f3` | Human | Nit | correctness | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The body of the second commit says six files; the commit changes seven

- PR number: 2945
- Source review ID: 5958194168
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2945#discussion_r4615228599
- Concern: The body of `docs(issues): quote last-updated-utc in the open manual verification evidence files` says six open evidence files; its diff changes seven.
- Solution: No change. Rewording it would rewrite a pushed commit, and the repository keeps intermediate commits unchanged; the reviewer says the rewording is allowed, not required. The description already gives the right count, and the merge commit copies the description.
- Current-tree verification: `git show --stat` on that commit lists seven `manual-verification-evidence.md` files; the description says seven and names the wrong count in the commit body.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2945#discussion_r4615258194
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2945#discussion_r4615258194

### F2 - The not-changed list of the description is stale after pull request 2942 merged

- PR number: 2945
- Source review ID: 5958194168
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2945#discussion_r4615228621
- Concern: The description said the evidence file of #2904 was not merged yet and that the validation grep lists three files; #2942 has merged, so on the merged tree the grep lists two.
- Solution: The description now says #2942 quoted that file and merged, and the Validation line says what the grep prints on `develop` at `4497837e6` and which two files remain after the merge.
- Current-tree verification: `git grep -n "^last-updated-utc: [0-9Y]" torrust/develop -- docs/templates docs/issues/open` at `4497837e6` prints the nine files this pull request changes and the #269 spec and #1347 guidelines page.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2945#issuecomment-6980964067
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2945#discussion_r4615258794

### F3 - The YAML-date reason does not hold for the `HH:MM` form

- PR number: 2945
- Source review ID: 5958194168
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/2945#discussion_r4615228637
- Concern: A YAML 1.1 timestamp needs seconds and the YAML 1.2 core schema has no timestamp type, so `2026-09-28 10:00` parses as the same string with or without quotes; only a date-only value becomes a date.
- Solution: The description drops the date reason, gives the remaining one (one form across the templates, the form the validator requires for issue and EPIC specs), and says the commit bodies give the wrong reason; the commits are not rewritten.
- Current-tree verification: PyYAML `safe_load` reads `2026-09-28 10:00` as `str`, `2026-09-28 10:00:00` as `datetime` and `2026-09-14` as `date`.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2945#issuecomment-6980964067
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2945#discussion_r4615259371

## Processing Log

- 2026-11-18 14:08 UTC - Started audit, before the replies of round 1.
- 2026-11-18 14:08 UTC - Round 1: the approving review `5958194168` of `da2ce7` (submitted 14:06:06Z, forge `submitted_at`) with three findings. No commit was needed: F1 is `NO_ACTION` (no history rewrite), and F2 and F3 are fixed in the description. The conversation response for F2 and F3 and the three replies were posted between 14:08:11Z and 14:08:18Z (forge `created_at`). `validate-audit-record.py --pr-number 2945 --base torrust/develop`, with its `gh` calls made through `gh-bot`, exited 0 (3 rows, 0 failures) at 14:08:33Z (`date -u`); the three threads were then resolved by their thread IDs by 14:08:43Z.

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
