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

Four reviews: Copilot review 5845178584 (a summary with no actionable finding, so it has no row), da2ce7 review 5845212310 (round 1, eleven findings, which keep the reviewer's IDs) da2ce7 review 5845706883 (round 2, six findings) and da2ce7 review 5846280739 (round 3, three findings). The round 2 `F10` re-raises `F10` and became `F17`; the other round 2 IDs (`F12` to `F16`) are the reviewer's. In round 3 the reviewer's `F17` (a re-raise of `F14`) became `F20`; its `F18` and `F19` kept their IDs.

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
| F12 | review-finding:pr-2807-f12 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F17 | review-finding:pr-2807-f17 | Human | Nit | documentation | RE_RAISE_OF:F10 | FIXED | RESOLVED |
| F13 | review-finding:pr-2807-f13 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F14 | review-finding:pr-2807-f14 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F15 | review-finding:pr-2807-f15 | Human | Nit | maintainability | ORIGINAL | NO_ACTION | SUPERSEDED |
| F16 | review-finding:pr-2807-f16 | Human | Suggestion | documentation | ORIGINAL | FIXED | RESOLVED |
| F20 | review-finding:pr-2807-f20 | Human | Nit | documentation | RE_RAISE_OF:F14 | FIXED | RESOLVED |
| F18 | review-finding:pr-2807-f18 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F19 | review-finding:pr-2807-f19 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - `Started` is recorded as kept and tested, but nothing in this workspace uses or tests it

- PR number: 2807
- Source review ID: 5845212310
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2807#discussion_r4527363973
- Concern: The acceptance row, the criterion and the log say `Started` remains available and tested; no `.rs` file imports `torrust_server_lib::signals`.
- Solution: The acceptance row and the criterion say it remains available in `torrust-server-lib` 0.6.0 and is no longer used here, and the progress-log entry says it is no longer used or tested here.
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
- Solution: The row is IN_PROGRESS and the approval item links the merged `torrust-server-lib` PR 6 and PR 2803; the migration guide cites the spec's start gate and PR 2803.
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

### F12 - The live-spec block is not a runnable command, and its output drops one of the five matches

- PR number: 2807
- Source review ID: 5845706883
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2807#discussion_r4527713351
- Concern: The block is prose, shows four of five matches, rewrites a line and leaves a trailing space.
- Solution: The evidence pastes the real command with its full output (five lines).
- Current-tree verification: `git diff --check` prints nothing and the command prints the five lines.
- Resolution reference: docs(issues): [#2801] paste the real live-spec search and its full output
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2807#discussion_r4527857390

### F17 - Re-raise: the criterion still says only archived history matches, but the new search finds live specs

- PR number: 2807
- Source review ID: 5845706883
- Reviewer finding ID: F10
- Source URL: https://github.com/torrust/torrust-tracker/pull/2807#discussion_r4527713365
- Concern: The criterion, the acceptance row and the evidence item say nothing remains outside archived history, but five live-spec lines match.
- Solution: The three places (and the M2 expectation and T5 note) now say no use remains and descriptions match in archived docs and five live-spec lines.
- Current-tree verification: `git grep -n "five lines" docs/issues/open/2801-1488-si-19-remove-legacy-shutdown-api` lists the places.
- Resolution reference: docs(issues): [#2801] say that live specs still describe the removed API
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2807#discussion_r4527857932

### F13 - branch still names the spec branch, while related-pr names the implementation PR

- PR number: 2807
- Source review ID: 5845706883
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2807#discussion_r4527713374
- Concern: The frontmatter `branch` names the merged spec branch.
- Solution: `branch` is `2801-remove-legacy-shutdown-api`.
- Current-tree verification: Line 10 of the spec was read after the edit.
- Resolution reference: docs(issues): [#2801] point the spec branch at the implementation branch
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2807#discussion_r4527858338

### F14 - Two Solutions say more than the bytes

- PR number: 2807
- Source review ID: 5845706883
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2807#discussion_r4527713383
- Concern: F1 and F3 Solutions and the F3 reply claimed wording and links the files do not have.
- Solution: The two Solutions and two replies now match the files. In the same sweep the statements that the maintainer published 0.6.0 were removed from the spec and the evidence, because the evidence only shows it is published (commit "docs(issues): [#2801] state only what the evidence shows about the 0.6.0 publication").
- Current-tree verification: The Solutions, the replies and `git grep -n "maintainer published" docs/issues/open/2801-1488-si-19-remove-legacy-shutdown-api` (which prints only the progress-log entry that describes the removal, `ISSUE.md:242`) were checked.
- Resolution reference: docs(pr-reviews): [#2807] align the audit with the bytes and record the second round
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2807#discussion_r4527858834

### F15 - Two commits each carry several independent fixes

- PR number: 2807
- Source review ID: 5845706883
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2807#discussion_r4527713390
- Concern: Two commits of round 1 fix several findings each.
- Solution: No change: the reviewer notes a rewrite is not needed; the fixes of round 2 are in separate commits.
- Current-tree verification: The reply states the decision.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2807#discussion_r4527859329
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2807#discussion_r4527859329

### F16 - Record the ten invocations so M4 can be re-run from this file

- PR number: 2807
- Source review ID: 5845706883
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2807#discussion_r4527713414
- Concern: The evidence does not record the ten commands or the tracker configuration.
- Solution: The evidence records the ten invocations as one loop and inlines the tracker configuration.
- Current-tree verification: The evidence was read after the edit.
- Resolution reference: docs(issues): [#2801] record the ten signal verification invocations
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2807#discussion_r4527859808

### F20 - Two statements of this round's sweep go further than the bytes

- PR number: 2807
- Source review ID: 5846280739
- Reviewer finding ID: F17
- Source URL: https://github.com/torrust/torrust-tracker/pull/2807#discussion_r4528154725
- Concern: The F14 verification says a grep prints nothing but it prints the progress-log entry, and the F3 reply cites a merge the evidence no longer cites.
- Solution: The verification says what the grep prints; the evidence now says the `torrust-server-lib` PR is merged (commit "docs(issues): [#2801] cite the merge of the torrust-server-lib PR in the evidence"), so the F3 reply is true again.
- Current-tree verification: The grep was re-run, and the other grep commands named in this audit were re-run and print what their rows say.
- Resolution reference: docs(pr-reviews): [#2807] correct the audit verification of the 0.6.0 sweep
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2807#discussion_r4528211310

### F18 - The 16:54 stamp matches no commit or forge time

- PR number: 2807
- Source review ID: 5846280739
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2807#discussion_r4528154746
- Concern: The second-round log entry of the spec is stamped 16:54, but its commit was made at 16:55.
- Solution: The entry is stamped 16:55.
- Current-tree verification: `git log --format=%cI` of the commit was compared with the stamp.
- Resolution reference: docs(issues): [#2801] stamp the second round log entry at its commit minute
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2807#discussion_r4528211789

### F19 - The inlined configuration is not the copy of the development template the line describes

- PR number: 2807
- Source review ID: 5846280739
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2807#discussion_r4528154758
- Concern: The line says a copy of the development template, but the inlined file leaves out sections and adds one.
- Solution: The line describes a reduced copy and names what was left out and added.
- Current-tree verification: The line and the inlined file were compared.
- Resolution reference: docs(issues): [#2801] describe the verification configuration as a reduced copy
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2807#discussion_r4528212187

## Processing Log

- 2026-11-10 16:10 UTC - Copilot review received (16:10:42 UTC) (no actionable finding).
- 2026-11-10 16:12 UTC - da2ce7 review received (16:12:39 UTC) (approval with eleven findings, `F1` to `F11`).
- 2026-11-10 16:19 UTC - Replies posted on all eleven threads (16:18:48 to 16:19:14 UTC); the audit started in the commit that carries this entry.
- 2026-11-10 16:43 UTC - da2ce7 review received (round 2: approval of the pushed head, six findings, `F12` to `F17` here; submitted 16:43:09 UTC).
- 2026-11-10 16:56 UTC - Replies posted on the six threads (16:55:52 to 16:56:05 UTC) and two earlier replies edited; the audit corrected in the commit that carries this entry.
- 2026-11-10 17:22 UTC - da2ce7 review received (round 3: approval of the pushed head, three nits, `F18` to `F20` here; submitted 17:22:39 UTC).
- 2026-11-10 17:27 UTC - Replies posted on the three threads (17:27:51 to 17:27:56 UTC); the audit corrected in the commit that carries this entry.

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
