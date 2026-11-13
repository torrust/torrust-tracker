---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2848 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2848.

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

Rounds: da2ce7 review `5878128462` (F1 to F6), Copilot reviews `5878561072` (F7 to F9) and
`5881738305` (F10), da2ce7 reviews `5887686736` (F11, F12), `5887812564` (F13, F14) and
`5889285431` (F15). da2ce7's
`F7` of the third round collided with Copilot's audit-local `F7`, so it is `F11` here with the
reviewer ID recorded.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2848-f1 | Human | Major | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2848-f2 | Human | Minor | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2848-f3 | Human | Major | metadata | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2848-f4 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2848-f5 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2848-f6 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2848-f7 | Copilot | Major (inferred) | link-integrity | ORIGINAL | NO_ACTION | RESOLVED |
| F8 | review-finding:pr-2848-f8 | Copilot | Major (inferred) | metadata | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-2848-f9 | Copilot | Minor (inferred) | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-2848-f10 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-2848-f11 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F12 | review-finding:pr-2848-f12 | Human | Minor | documentation | RE_RAISE_OF:F4 | FIXED | RESOLVED |
| F13 | review-finding:pr-2848-f13 | Human | Minor | documentation | RE_RAISE_OF:F4 | FIXED | RESOLVED |
| F14 | review-finding:pr-2848-f14 | Human | Minor | documentation | RE_RAISE_OF:F11 | FIXED | RESOLVED |
| F15 | review-finding:pr-2848-f15 | Human | Minor | documentation | RE_RAISE_OF:F11 | FIXED | RESOLVED |

## Finding Details

### F1 - The SI-15 checkpoint path was rewritten although SI-15 was never a discarded draft

- PR number: 2848
- Source review ID: 5878128462
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2848#discussion_r4553219564
- Concern: The path substitution rewrote a historical checkpoint to a discarded-drafts path that never existed.
- Solution: Restored the original historical line naming docs/issues/drafts/1488-si-15-define-udp-active-request-policy/ISSUE.md.
- Current-tree verification: Read the line in the #2370 spec at this head; it names the original draft path.
- Resolution reference: docs(issues): [#1488] address review of PR #2848
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2848#discussion_r4555797675

### F2 - The SI-17 related artifact pointed to a draft that was never discarded

- PR number: 2848
- Source review ID: 5878128462
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2848#discussion_r4553219579
- Concern: The same substitution pointed the #2412 frontmatter at a discarded-draft path for SI-17, which became #2448.
- Solution: The artifact now points at the closed #2448 spec.
- Current-tree verification: Read the #2412 frontmatter; the entry names the #2448 spec and appears once.
- Resolution reference: docs(issues): [#1488] address review of PR #2848
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2848#discussion_r4555798123

### F3 - The four new specs keep github-issue: null

- PR number: 2848
- Source review ID: 5878128462
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/2848#discussion_r4553219589
- Concern: The placeholder was not replaced after the issues were created.
- Solution: Set github-issue to 2844, 2845, 2846 and 2847.
- Current-tree verification: grep of github-issue in the four specs shows the numbers.
- Resolution reference: docs(issues): [#1488] address review of PR #2848
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2848#discussion_r4555798494

### F4 - SI-27 recommends a location that copies a deregister-before-join-error order

- PR number: 2848
- Source review ID: 5878128462
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/2848#discussion_r4553219602
- Concern: The SI-24 Environment::stop deregisters before mapping the join error, which SI-27 forbids, and the spec called the test environment the only production-code user.
- Solution: The scope names the order difference, T1 must not copy it, and the wording says caller. The caller wording was re-raised as F12 and F13.
- Current-tree verification: Read the SI-27 scope; it names the order and the environment.
- Resolution reference: docs(issues): [#1488] address review of PR #2848
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2848#discussion_r4555798883

### F5 - report_shutdown is private to the binary

- PR number: 2848
- Source review ID: 5878128462
- Reviewer finding ID: F5
- Source URL: https://github.com/torrust/torrust-tracker/pull/2848#discussion_r4553219619
- Concern: Candidate 1 of SI-28 implied reuse of a private function.
- Solution: Candidate 1 names shutdown_exit_code and incomplete_components in bootstrap::jobs::manager and says report_shutdown is private.
- Current-tree verification: Read the SI-28 scope at this head.
- Resolution reference: docs(issues): [#1488] address review of PR #2848
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2848#discussion_r4555799187

### F6 - The branch frontmatter choice is not recorded

- PR number: 2848
- Source review ID: 5878128462
- Reviewer finding ID: F6
- Source URL: https://github.com/torrust/torrust-tracker/pull/2848#discussion_r4553219635
- Concern: The specs name the reserved implementation branches instead of the spec branch, with no recorded reason.
- Solution: The EPIC states the choice after the recommended-order paragraph.
- Current-tree verification: Read the EPIC paragraph at this head.
- Resolution reference: docs(issues): [#1488] address review of PR #2848
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2848#discussion_r4555799601

### F7 - The EPIC links to discarded-drafts files that the diff shows as edits of the old paths

- PR number: 2848
- Source review ID: 5878561072
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2848#discussion_r4553564381
- Concern: The reviewer read the diff as edits to docs/issues/drafts files with no moves to the new folder.
- Solution: No change: the files are moved in the PR and the diff view shows rename detection.
- Current-tree verification: Listed the discarded-drafts folder and ran linter all; all local Markdown links pass.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2848#discussion_r4555799997
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2848#discussion_r4555799997

### F8 - github-issue: null contradicts the body

- PR number: 2848
- Source review ID: 5878561072
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2848#discussion_r4553564459
- Concern: Same defect as F3 in the four new specs.
- Solution: Set github-issue in all four specs (same change as F3).
- Current-tree verification: grep of github-issue in the four specs shows the numbers.
- Resolution reference: docs(issues): [#1488] address review of PR #2848
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2848#discussion_r4555800299

### F9 - The SI-15 checkpoint claims a discarded-drafts location

- PR number: 2848
- Source review ID: 5878561072
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2848#discussion_r4553564537
- Concern: Same defect as F1.
- Solution: Restored the original line (same change as F1).
- Current-tree verification: Read the line in the #2370 spec.
- Resolution reference: docs(issues): [#1488] address review of PR #2848
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2848#discussion_r4555800619

### F10 - SI-28 Scope lists T1 and T2 while the plan has T1 to T4

- PR number: 2848
- Source review ID: 5881738305
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2848#discussion_r4556112444
- Concern: The Scope bullets and the Implementation Plan used different task boundaries.
- Solution: The Scope bullets list T1 to T4 like the plan.
- Current-tree verification: Read the SI-28 Scope and plan tables.
- Resolution reference: docs(issues): [#1488] align SI-28 scope with its implementation plan
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2848#discussion_r4560871461

### F11 - No review audit record at the head

- PR number: 2848
- Source review ID: 5887686736
- Reviewer finding ID: F7
- Source URL: https://github.com/torrust/torrust-tracker/pull/2848#discussion_r4560843486
- Concern: process-pr-review asks for docs/pr-reviews/pr-2848-review/PR-REVIEW.md before the first reply, and the replies cited short commit ids.
- Solution: This record is added; the Processing Log lists the late start and maps each short id to its subject.
- Current-tree verification: Ran the validator against this record.
- Resolution reference: docs(pr-reviews): [#1488] record the review audit of PR #2848
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2848#discussion_r4561859955

### F12 - Line 52 still says production-code caller

- PR number: 2848
- Source review ID: 5887686736
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/2848#discussion_r4560843481
- Concern: The test environment sits behind the testing feature and is the only deregister caller in the tree.
- Solution: The line says the only caller anywhere in the tree is the test environment, behind the testing feature.
- Current-tree verification: lib.rs line 1 is the testing cfg, and environment.rs:147 is the only deregister call outside server-lib.
- Resolution reference: docs(issues): [#1488] name the health-check test environment as the only deregister caller
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2848#discussion_r4561859588

### F13 - Line 52 is unchanged and the earlier thread was resolved without a reply

- PR number: 2848
- Source review ID: 5887812564
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/2848#discussion_r4560942783
- Concern: Re-raise of F12 after I resolved its thread without a reply.
- Solution: Same change as F12; replied on the thread.
- Current-tree verification: Read line 52 of the SI-27 spec.
- Resolution reference: docs(issues): [#1488] name the health-check test environment as the only deregister caller
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2848#discussion_r4561860407

### F14 - No audit record and the F7 thread was resolved without a reply

- PR number: 2848
- Source review ID: 5887812564
- Reviewer finding ID: F7
- Source URL: https://github.com/torrust/torrust-tracker/pull/2848#discussion_r4560942791
- Concern: Re-raise of F11 after I resolved its thread without a reply; nine replies cite a branch commit by short id.
- Solution: This record is added and maps the short ids to subjects (see the Processing Log); replied on the thread.
- Current-tree verification: Ran the validator against this record.
- Resolution reference: docs(pr-reviews): [#1488] record the review audit of PR #2848
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2848#discussion_r4561860878

### F15 - The replies said the audit record was added, but it was not yet pushed

- PR number: 2848
- Source review ID: 5889285431
- Reviewer finding ID: F7
- Source URL: https://github.com/torrust/torrust-tracker/pull/2848#discussion_r4562073107
- Concern: F11/F14's replies claimed the audit record was added, but at the time of the review the record was written locally and not yet committed or pushed, so it did not exist at the reviewed head.
- Solution: Pushed the record at 524249b65 and replied citing that head.
- Current-tree verification: `git log --format=%h -1` on the pushed branch is `524249b65`; the file exists at that head.
- Resolution reference: docs(pr-reviews): [#1488] record the review audit of PR #2848
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2848#discussion_r4562282202

## Processing Log

- 2026-11-12 20:15 UTC - Replied on the nine threads of the first two reviews at 20:14:46Z to 20:15:04Z (forge `created_at`) and resolved them, before this record existed. This record was started late, on 2026-11-13, after da2ce7 asked for it.
- 2026-11-13 07:43 UTC - Replied on the Copilot thread of the second round (F10) at 07:43:29Z and resolved the open threads. That resolution also closed two new da2ce7 threads (F12 and F11) without a reply.
- 2026-11-13 09:42 UTC - Fix commit for F12 and F13 made at 09:42:25Z (`git log`), after the wording was checked against `lib.rs` and the `deregister(` callers.
- 2026-11-13 09:50 UTC - Replied on the four da2ce7 threads of the third and fourth reviews (F11 to F14); the threads were already resolved.
- 2026-11-13 10:12 UTC - da2ce7 re-raised F7 again as F15 (comment created 10:12:54Z), because the audit commit was not yet pushed when they reviewed `8cd0c930d`.
- 2026-11-13 10:20 UTC - Audit commit `524249b65` pushed; replied on F15's thread and resolved F13, F14 and F15 (none had been caught by the earlier resolve pass, since it ran before their replies existed). GraphQL refresh showed 0 unresolved threads.
- Short commit ids cited in the first replies, mapped to subjects: `56cc1be` is `docs(issues): [#1488] address review of PR #2848` and `572ae56` is `docs(issues): [#1488] align SI-28 scope with its implementation plan`.

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- Cite a fix by its unique Conventional Commit subject or durable reply URL, never by a branch SHA
  that can change after a rebase.
- Refresh review threads using GraphQL and confirm that no unresolved actionable thread remains.
