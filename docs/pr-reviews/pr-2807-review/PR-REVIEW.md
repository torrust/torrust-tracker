---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2807 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2807.

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

Two reviews: Copilot review 5845178584 (a summary with no actionable finding, so it has no row) and da2ce7 review 5845212310 (round 1, eleven findings, which keep the reviewer's IDs).

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2807-f1 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2807-f2 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2807-f3 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2807-f4 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2807-f5 | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2807-f6 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2807-f7 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2807-f8 | Human | Nit | maintainability | ORIGINAL | NO_ACTION | SUPERSEDED |
| F9 | review-finding:pr-2807-f9 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-2807-f10 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-2807-f11 | Human | Suggestion | testing | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - `Started` is recorded as kept and tested, but nothing in this workspace uses or tests it

- PR number: 2807
- Source review ID: 5845212310
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2807#discussion_r4527363973
- Concern: The acceptance row, the criterion and the log say `Started` remains available and tested; no `.rs` file imports `torrust_server_lib::signals`.
- Solution: The three places now say it remains available in `torrust-server-lib` 0.6.0 and is no longer used or tested here.
- Current-tree verification: `git grep -n "no longer used" docs/issues/open/2801-1488-si-19-remove-legacy-shutdown-api/ISSUE.md` finds the three statements.
- Resolution reference: docs(issues): [#2801] correct the SI-19 spec and evidence after the review
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2807#discussion_r4527434600

### F2 - T5 is DONE and names the EPIC row, but the EPIC is not changed

- PR number: 2807
- Source review ID: 5845212310
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2807#discussion_r4527363984
- Concern: T5 says the EPIC row is updated, yet this PR leaves row 19 as Planned.
- Solution: T5 no longer lists the row and its notes say the close-out archive step flips it, as for SI-18.
- Current-tree verification: The T5 row of the Implementation Plan was read after the edit.
- Resolution reference: docs(issues): [#2801] correct the SI-19 spec and evidence after the review
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2807#discussion_r4527435128

### F3 - Start-gate reviewed is DONE but unchecked elsewhere, and the approval has no link

- PR number: 2807
- Source review ID: 5845212310
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2807#discussion_r4527363993
- Concern: The row is DONE while the criterion is open and no review has concluded; the approval cites only a chat.
- Solution: The row is IN_PROGRESS and the approval item links the merged PR 6, the publication and PR 2803.
- Current-tree verification: The row and the first evidence section were read after the edit.
- Resolution reference: docs(issues): [#2801] correct the SI-19 spec and evidence after the review
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2807#discussion_r4527435687

### F4 - Test 1 records a paraphrase, not the command output, and miscounts the ADRs

- PR number: 2807
- Source review ID: 5845212310
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2807#discussion_r4527364002
- Concern: The evidence elides output, miscounts two ADRs as three and does not include `e2e.sh`.
- Solution: The evidence pastes the raw output and inlines the script.
- Current-tree verification: The commands were re-run at this head and their output pasted.
- Resolution reference: docs(issues): [#2801] correct the SI-19 spec and evidence after the review
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2807#discussion_r4527436173

### F5 - The converted test duplicates the existing drain test

- PR number: 2807
- Source review ID: 5845212310
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2807#discussion_r4527364008
- Concern: The converted HTTP test repeats the arrange, act and assertion of the drain test.
- Solution: The test is deleted.
- Current-tree verification: `git grep -n "preserve_the_launcher_bind_address" packages` prints nothing.
- Resolution reference: refactor: [#2801] cover the token-aware server owner and fix the docs left by the removal
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2807#discussion_r4527436734

### F6 - Three UdpError variants are never built and their docs describe the removed stop path

- PR number: 2807
- Source review ID: 5845212310
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2807#discussion_r4527364026
- Concern: The enum and `OwnedReceiveLoop` docs still describe the removed stop and join.
- Solution: The `UdpError` doc names the unused variants and says why they stay; the `OwnedReceiveLoop` doc no longer mentions joining.
- Current-tree verification: The two doc comments were read after the edit; `cargo check --workspace --all-targets` is warning-free.
- Resolution reference: refactor: [#2801] cover the token-aware server owner and fix the docs left by the removal
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2807#discussion_r4527437197

### F7 - The module rustdoc still routes starts through removed functions

- PR number: 2807
- Source review ID: 5845212310
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2807#discussion_r4527364033
- Concern: The REST module doc names `ApiServer::start` and `Launcher::start`; "This additive path" has nothing left to add to.
- Solution: The module doc describes `start_with_cancellation`, and the phrase is "This path" in the four servers.
- Current-tree verification: `git grep -n "additive path" packages` prints nothing.
- Resolution reference: refactor: [#2801] cover the token-aware server owner and fix the docs left by the removal
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2807#discussion_r4527437590

### F8 - The lock edit for axum-server lands one commit after its manifest edit

- PR number: 2807
- Source review ID: 5845212310
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2807#discussion_r4527364042
- Concern: The lock line arrives in the next commit, so a `--locked` build at one commit would fail.
- Solution: No change: the reviewer says a rewrite is not needed, and rewriting a reviewed branch is avoided.
- Current-tree verification: The reply states the decision.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2807#discussion_r4527438132
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2807#discussion_r4527438132

### F9 - The workflow status lags the work

- PR number: 2807
- Source review ID: 5845212310
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2807#discussion_r4527364055
- Concern: The spec-only checkpoint is unchecked and the frontmatter still says planned with no PR.
- Solution: The checkpoint is ticked and `status` and `related-pr` are updated.
- Current-tree verification: The frontmatter and the checkpoint were read after the edit.
- Resolution reference: docs(issues): [#2801] correct the SI-19 spec and evidence after the review
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2807#discussion_r4527438615

### F10 - The workspace search excludes docs/issues

- PR number: 2807
- Source review ID: 5845212310
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2807#discussion_r4527364062
- Concern: The search skipped live specs, one of which names a removed function.
- Solution: The evidence also searches `docs/issues/open` and `docs/issues/drafts` and pastes the output; the line in the legacy-shape draft stays because the frontmatter validator refuses to stage that file.
- Current-tree verification: The search was re-run and its output is in the evidence.
- Resolution reference: docs(issues): [#2801] correct the SI-19 spec and evidence after the review
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2807#discussion_r4527439123

### F11 - Port the two nested-owner abort tests to TokenAwareServerTask

- PR number: 2807
- Source review ID: 5845212310
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2807#discussion_r4527364074
- Concern: The removed tests were the only tests of the owner-abort contract.
- Solution: A test constructs `TokenAwareServerTask::new` and checks both children are aborted with the component.
- Current-tree verification: `cargo test -p torrust-tracker --lib manager` passes (48 passed).
- Resolution reference: refactor: [#2801] cover the token-aware server owner and fix the docs left by the removal
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2807#discussion_r4527439588

## Processing Log

- 2026-11-10 16:10 UTC - Copilot review received (16:10:42 UTC) (no actionable finding).
- 2026-11-10 16:12 UTC - da2ce7 review received (16:12:39 UTC) (approval with eleven findings, `F1` to `F11`).
- 2026-11-10 16:19 UTC - Replies posted on all eleven threads (16:18:48 to 16:19:14 UTC); the audit started in the commit that carries this entry.

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
