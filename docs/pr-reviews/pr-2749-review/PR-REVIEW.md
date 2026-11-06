---
pr-number: 2749
pr-url: https://github.com/torrust/torrust-tracker/pull/2749
last-updated-utc: "2026-11-06 13:41"
---

# PR #2749 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2749.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A

## Status Values

- Relationship: `ORIGINAL`, `RE_RAISE_OF:<FindingId>`
- Disposition: `FIXED`, `NO_ACTION`, `SUPERSEDED`, `FOLLOW_UP`
- Thread state: `OPEN`, `RESOLVED`, `NON_RESOLVABLE`, `SUPERSEDED`
- Severity: `Blocker`, `Major`, `Minor`, `Nit`, `Suggestion`; append `(inferred)` when derived
  from free prose.
- Author class: `Copilot`, `Human`, `Unknown`
- Category: `link-integrity`, `formatting`, `metadata`, `testing`, `correctness`,
  `documentation`, `maintainability`, `security`, `other`

## Findings

Round 1: Copilot review 5799338911 on the head `6874dd70f` (five inline findings; the review body reports that Copilot's full agentic review timed out, so this was a lite review). Severities are inferred from the labels in the review overview (Medium and Low). F3 duplicates F2.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2749-f1 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2749-f2 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2749-f3 | Copilot | Minor (inferred) | documentation | RE_RAISE_OF:F2 | NO_ACTION | SUPERSEDED |
| F4 | review-finding:pr-2749-f4 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2749-f5 | Copilot | Nit (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Owner line does not state the end date of da2ce7's ownership

- PR number: 2749
- Source review ID: 5799338911
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2749#discussion_r4491831667
- Concern: The owner line implies da2ce7's ownership ended when josecelano resumed but never states the end date.
- Solution: Valid. The line now reads "held it from 2026-09-28 to 2026-11-06".
- Current-tree verification: `grep -n "to 2026-11-06 (" docs/issues/open/2003-overhaul-guardrails-and-automation/EPIC.md` shows it on line 46.
- Resolution reference: docs(issues): [#2003] state the end date of da2ce7's ownership
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2749#discussion_r4493008394

### F2 - Note about rows 16 to 19 under the list of items outside the order

- PR number: 2749
- Source review ID: 5799338911
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2749#discussion_r4491831708
- Concern: The added sentence under "Also linked as GitHub sub-issues, outside the order" refers to items that are now in the ordered table.
- Solution: Valid. The sentence is removed; rows 16 to 19 are in the Subissues table.
- Current-tree verification: `grep -n "rows 16 to 19" docs/issues/open/2003-overhaul-guardrails-and-automation/EPIC.md` finds nothing.
- Resolution reference: docs(issues): [#2003] drop the note about rows 16 to 19 from the unordered list
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2749#discussion_r4493008689

### F3 - Same note about rows 16 to 19 (duplicate comment)

- PR number: 2749
- Source review ID: 5799338911
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2749#discussion_r4491831732
- Concern: The same concern as F2 on the adjacent line.
- Solution: Duplicate of F2; the F2 fix removes the sentence both comments point at.
- Current-tree verification: Same grep as F2: no match for "rows 16 to 19".
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2749#discussion_r4493009044
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2749#discussion_r4493009044

### F4 - Historical checklist still reads like an active plan

- PR number: 2749
- Source review ID: 5799338911
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2749#discussion_r4491831757
- Concern: The line calling the order 12 close-out checklist historical does not separate it clearly from the unchecked boxes that follow.
- Solution: Valid. The lead-in is now a bold "Historical plan, as of 2026-09-28; not current work." line directly above the checklist.
- Current-tree verification: `grep -n "Historical plan" docs/issues/open/2003-overhaul-guardrails-and-automation/EPIC.md` shows it on line 202, above the checklist.
- Resolution reference: docs(issues): [#2003] mark the order 12 close-out checklist as a historical plan
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2749#discussion_r4493009424

### F5 - Progress log entry attributed to "Claude" instead of a handle

- PR number: 2749
- Source review ID: 5799338911
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2749#discussion_r4491831790
- Concern: Other Progress Log entries use GitHub handles; "Claude" breaks the attribution convention.
- Solution: Valid. The entry is attributed to "josecelano (via the Claude EPIC coordinator session)".
- Current-tree verification: `grep -n "via the Claude" docs/issues/open/2003-overhaul-guardrails-and-automation/EPIC.md` shows it on line 438.
- Resolution reference: docs(issues): [#2003] attribute the 2026-11-06 progress log entry to a maintainer handle
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2749#discussion_r4493009863

## Processing Log

- 2026-11-06 13:35 UTC - Claude - Fixed F1 and F2 in two signed commits (committed 13:35:16 and 13:35:54 UTC).
- 2026-11-06 13:37 UTC - Claude - Fixed F4 and F5 in two signed commits (committed 13:36:46 and 13:37:18 UTC).
- 2026-11-06 13:40 UTC - Claude - Pushed the four fix commits and replied on all five threads (13:40:37 to 13:40:48 UTC).

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
