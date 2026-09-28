---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - "issue #2295"
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/templates/PR-REVIEW-TEMPLATE.md
    - contrib/dev-tools/checks/agent-review-report-contract/src/main.rs
---

<!-- skill-link: process-pr-review -->

# PR #2300 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/2300>.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: <https://github.com/torrust/torrust-tracker/issues/2347#issuecomment-5865646588>

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

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2300-f1` | Copilot | Major (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2300-f2` | Copilot | Major (inferred) | documentation | RE_RAISE_OF:F1 | NO_ACTION | SUPERSEDED |
| F3 | `review-finding:pr-2300-f3` | Copilot | Major (inferred) | maintainability | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2300-f4` | Copilot | Major (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | `review-finding:pr-2300-f5` | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F6 | `review-finding:pr-2300-f6` | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F7 | `review-finding:pr-2300-f7` | Human | Minor | testing | ORIGINAL | NO_ACTION | RESOLVED |
| F8 | `review-finding:pr-2300-f8` | Human | Minor | correctness | ORIGINAL | NO_ACTION | RESOLVED |
| F9 | `review-finding:pr-2300-f9` | Human | Minor | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F10 | `review-finding:pr-2300-f10` | Human | Suggestion | maintainability | ORIGINAL | FOLLOW_UP | OPEN |
| F11 | `review-finding:pr-2300-f11` | Human | Suggestion | documentation | ORIGINAL | NO_ACTION | RESOLVED |
| F12 | `review-finding:pr-2300-f12` | Human | Minor | maintainability | ORIGINAL | NO_ACTION | RESOLVED |

Rows F5-F12 come from review 5284011916, submitted after the merge and tracked by #2347. The reviewer numbered them `loop F5`-`loop F12` after this record's F1-F4, so no ID collides. F5, F6, and F9 are fixed in follow-up PR #2363 and stay `FOLLOW_UP`/`OPEN` until it merges. F10 is owned by EPIC #2278 order 8. The `NO_ACTION` threads are resolved in the #2347 close-out.

## Finding Details

### F1 - Clarify the optional reviewer finding ID

- PR number: 2300
- Source review ID: 5279983104
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2300#discussion_r4073210083>
- Concern: The canonical 19-field roster implied every field is populated, although `Reviewer finding ID` is meaningful only when an audit ID is reassigned.
- Solution: States that `Reviewer finding ID` records the original ID when reassigned and otherwise uses `N/A`, matching the always-present template line.
- Current-tree verification: `grep -n -A5 'Reply URL' .github/skills/dev/pr-reviews/process-pr-review/SKILL.md` shows the `Reviewer finding ID` rule immediately after the roster; the template retains `- Reviewer finding ID: <OPTIONAL_ORIGINAL_FINDING_ID_WHEN_REASSIGNED>`.
- Resolution reference: `docs(pr-reviews): clarify audit roster optional fields`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2300#discussion_r4073336079>

### F2 - Duplicate optional reviewer finding ID suggestion

- PR number: 2300
- Source review ID: 5279983104
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2300#discussion_r4073210150>
- Concern: Duplicates F1's request for an explicit optional-field rule.
- Solution: No independent change; F1 owns the identical current-tree change.
- Current-tree verification: The source comment body and requested change are identical to F1; F1's resolution reference applies.
- Resolution reference: `docs(pr-reviews): clarify audit roster optional fields`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2300#discussion_r4073336408>

### F3 - Pin representative roster entries

- PR number: 2300
- Source review ID: 5279983104
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2300#discussion_r4073210208>
- Concern: Heading-only contract pins could allow a roster list to drift while its group headings remain.
- Solution: The contract checker now requires each roster group heading plus a representative entry: `Finding ID`, `Summary`, and `PR number`.
- Current-tree verification: `cargo run --package agent-review-report-contract` exits `0`; `REQUIRED_TEXT` includes the three heading/item pairs in `agent-review-report-contract/src/main.rs`.
- Resolution reference: `docs(pr-reviews): clarify audit roster optional fields`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2300#discussion_r4073336736>

### F4 - Correct the completion report follow-up state

- PR number: 2300
- Source review ID: 5279983104
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2300#discussion_r4073210260>
- Concern: The independent review report said it was still uncommitted, though it was already part of this PR.
- Solution: Reworded the follow-up action to state that the report and checkpoint update are included in this implementation PR.
- Current-tree verification: `grep -n -A2 '^\- Follow-up actions:' docs/issues/open/2295-2278-single-source-audit-roster/agent-review-reports.md` shows the current-state wording.
- Resolution reference: `docs(pr-reviews): clarify audit roster optional fields`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2300#discussion_r4073337001>

### F5 - Processing Log stamps precede the events they record

- PR number: 2300
- Source review ID: 5284011916
- Reviewer finding ID: loop F5
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2300#discussion_r4076706625>
- Concern: The 15:13 and 15:15 UTC log entries are stamped before the commit, replies, and push they record.
- Solution: `docs(pr-reviews): [#2347] correct PR #2300 audit log stamps` appends a correction entry that names both stamps and the re-derived event times, keeping the entries.
- Current-tree verification: `git log -1 --format=%aI` gives 15:20:35Z for `docs(pr-reviews): clarify audit roster optional fields` and 15:24:57Z for `docs(pr-reviews): audit PR 2300 Copilot review`. The replies' `created_at` values are 15:23:23Z-15:23:29Z, and the PR timeline's only push is `head_ref_force_pushed` at 16:12:36Z.
- Resolution reference: `docs(pr-reviews): [#2347] correct PR #2300 audit log stamps`
- Follow-up PR URL: <https://github.com/torrust/torrust-tracker/pull/2363>
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2300#discussion_r4124958505>

### F6 - AC6 is checked but false at the head it ships in

- PR number: 2300
- Source review ID: 5284011916
- Reviewer finding ID: loop F6
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2300#discussion_r4076706631>
- Concern: #2295's AC6, "No file under `docs/pr-reviews/` changes", is checked, but the PR added its own audit record.
- Solution: `docs(issues): [#2347] correct #2295 AC6 after the PR audit landed` appends a progress-log correction to the #2295 spec, keeping AC6 and M4 as recorded.
- Current-tree verification: `git diff --stat 3aeb62c5 caa6c674 -- docs/pr-reviews/` prints `docs/pr-reviews/pr-2300-review/PR-REVIEW.md | 117 +++`.
- Resolution reference: `docs(issues): [#2347] correct #2295 AC6 after the PR audit landed`
- Follow-up PR URL: <https://github.com/torrust/torrust-tracker/pull/2363>
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2300#discussion_r4124958728>

### F7 - V2's recorded grep output no longer reproduces, and M2 was not repeated after the review round that invalidated it

- PR number: 2300
- Source review ID: 5284011916
- Reviewer finding ID: loop F7
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2300#discussion_r4076706639>
- Concern: V2 records the pinned roster sentence at line 167, which moved after `docs(pr-reviews): clarify audit roster optional fields`, and M2 was not re-run after that round.
- Solution: No action, approved by the maintainer. Only the line number moved and the check still passes, so this row records the current output instead of editing closed evidence. The un-repeated verification is recorded as a recurring cause in the #2347 triage.
- Current-tree verification: on `develop` at `8a953724` (the #2347 branch does not change the skill), `grep -nF 'tracking row plus one matching detail entry carrying the remaining narrative and' .github/skills/dev/pr-reviews/process-pr-review/SKILL.md` prints `185:`, and `cargo run --package agent-review-report-contract` prints `Agent review report contract check passed.`
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2300#discussion_r4121104659>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2300#discussion_r4121104659>

### F8 - This ownership mark states something no audit record in the repository satisfies

- PR number: 2300
- Source review ID: 5284011916
- Reviewer finding ID: loop F8
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2300#discussion_r4076706646>
- Concern: The template marks `## Status Values` as copied verbatim, but at the PR head no record copied its `OPEN` sentence.
- Solution: No action, approved by the maintainer; the finding is not live. The separate policy-bullet concern in the same section is `review-finding:pr-2313-f7`, owned by #2362.
- Current-tree verification: on `develop` at `478516cf`, comparing the `## Status Values` section of each of the 119 records with the template shows 10 byte-identical records, `pr-2320` through `pr-2353`.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2300#discussion_r4121105015>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2300#discussion_r4121105015>

### F9 - The Inputs line cites three branch SHAs that no longer resolve at this head

- PR number: 2300
- Source review ID: 5284011916
- Reviewer finding ID: loop F9
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2300#discussion_r4076706651>
- Concern: The #2295 agent-review-reports Inputs line cites `1682aff5`, `b33e2807`, and `27a57e1b`, which the pre-merge force-push made unreachable.
- Solution: `docs(issues): [#2347] cite #2295 review inputs by commit subject` appends a correction entry that maps each id to its subject, keeping the 14:11 entry.
- Current-tree verification: `gh api repos/torrust/torrust-tracker/commits/<sha>` still serves the three objects, with subjects `docs(pr-reviews): single-source the audit field roster`, `docs(pr-reviews): mark audit template section ownership`, and `docs(issues): record issue 2295 verification evidence`. Each subject names exactly one commit on `develop`.
- Resolution reference: `docs(issues): [#2347] cite #2295 review inputs by commit subject`
- Follow-up PR URL: <https://github.com/torrust/torrust-tracker/pull/2363>
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2300#discussion_r4124958998>

### F10 - The new pins cover 3 of the 19 roster names, so the skill's "must match exactly" rule stays largely unenforced

- PR number: 2300
- Source review ID: 5284011916
- Reviewer finding ID: loop F10
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2300#discussion_r4076706673>
- Concern: Renaming or reordering roster fields such as `Concern` and `Solution` in the skill leaves the contract check and the validator green.
- Solution: Follow-up in EPIC #2278 order 8 ("Extend the audit validator to the adopted invariants", register F58), whose row now names this finding. #2349 is not the owner, because it excludes changing what the pins require, other than their granularity.
- Current-tree verification: `agent-review-report-contract/src/main.rs` pins `- Finding ID`, `- Summary`, and `- PR number` on the skill side. `grep -n 'pr-2300-f10'` finds the order 8 row in the #2278 EPIC.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2300#discussion_r4121105898>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2300#discussion_r4121105898>

### F11 - This commit also rewrites a historical progress entry, beyond what the issue scoped for the EPIC

- PR number: 2300
- Source review ID: 5284011916
- Reviewer finding ID: loop F11
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2300#discussion_r4076706680>
- Concern: `docs(issues): record issue 2295 verification evidence` dropped "; the GitHub label changes from `task` to `epic` when this specification merges" from the EPIC's 2026-09-22 06:59 UTC entry.
- Solution: No action, approved by the maintainer. The clause was stale future tense that the 09:40 entry had superseded, so restoring it would reintroduce a false forward-looking statement. This row records what was removed.
- Current-tree verification: the #2278 EPIC's 06:59 UTC entry no longer contains the clause.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2300#discussion_r4121106156>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2300#discussion_r4121106156>

### F12 - All four rows cite one commit subject that describes only F1's fix

- PR number: 2300
- Source review ID: 5284011916
- Reviewer finding ID: loop F12
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2300#discussion_r4076706663>
- Concern: `docs(pr-reviews): clarify audit roster optional fields` carries three independent fixes, so as the resolution reference for F3 and F4 it does not identify their changes.
- Solution: No action, approved by the maintainer; merged history is immutable. That commit carries the F1 fix (the skill sentence), the F3 fix (the contract pins), and the F4 fix (the report wording). #2347 uses one commit per fix.
- Current-tree verification: `git show --stat` of that commit lists `SKILL.md`, `agent-review-report-contract/src/main.rs`, and the #2295 `agent-review-reports.md`.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2300#discussion_r4121105570>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2300#discussion_r4121105570>

## Processing Log

- 2026-09-22 15:10 UTC - Fetched review 5279983104 and normalized four Copilot threads; F2 is a duplicate of F1.
- 2026-09-22 15:13 UTC - Committed fixes for F1, F3, and F4; pending replies and thread resolution.
- 2026-09-22 15:15 UTC - Pushed the fix, replied to F1-F4, resolved all four threads, and confirmed with GraphQL that no unresolved threads remain.
- 2026-09-28 09:00 UTC - Correction (post-merge, #2347): the 15:13 and 15:15 entries above are stamped before the events they record. Commit `docs(pr-reviews): clarify audit roster optional fields`, which carries the F1, F3, and F4 fixes, was authored at 15:20:35 UTC. The four replies were posted at 15:23:23-15:23:29 UTC, and commit `docs(pr-reviews): audit PR 2300 Copilot review` was authored at 15:24:57 UTC. The only push recorded in the PR timeline is the `head_ref_force_pushed` event at 16:12:36 UTC. No resolution time is captured for the threads. The two entries are kept unchanged.
- 2026-09-28 10:29 UTC - Posted a disposition reply on each of the eight post-merge threads of review 5284011916 (`created_at` 10:29:23Z-10:29:37Z). #2347 had posted tracking replies on 2026-09-26 from 12:05 UTC.
- 2026-09-28 10:44 UTC - Added rows F5-F12 for review 5284011916, as #2347 approved. Changed the Ownership section's `Post-merge workflow approval` from `N/A` to the #2347 approval record; that is the only in-place edit to earlier content.
- 2026-09-28 17:15 UTC - #2347 T7 close-out after PR #2363 merged into `develop` (17:00:27Z). josecelano posted the T7 replies on F5, F6, and F9 (`created_at` 17:07:50Z-17:07:53Z), then resolved those threads and the F7, F8, F11, and F12 `NO_ACTION` threads. This close-out records F5, F6, and F9 as `FIXED`/`RESOLVED`, each citing its fixing commit with its T7 reply as Reply URL, and F7, F8, F11, and F12 as `RESOLVED`, keeping the disposition replies. F10 stays `FOLLOW_UP`/`OPEN` (#2278 order 8).

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- For an outdated or superseded thread, reply exactly
  `Superseded by <FindingId>: <reason>.`, record `Disposition=NO_ACTION` and
  `Thread state=SUPERSEDED`, then resolve it.
- A consolidated PR conversation response may cover multiple review rounds only when it names
  every review ID and every finding ID with its disposition and resolution reference.
- Cite a fix by its unique Conventional Commit subject or durable reply URL, never by a branch SHA
  that can change after a rebase.
- Refresh review threads using GraphQL and confirm that no unresolved actionable thread remains.
