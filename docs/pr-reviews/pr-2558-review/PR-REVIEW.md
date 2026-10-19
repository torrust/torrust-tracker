---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/open/1349-1347-add-tests-http-core/ISSUE.md
---

<!-- skill-link: process-pr-review -->

# PR #2558 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2558>.

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

PR #2558 is the S1 pull request for #1349 and was open when its review was processed, so this is the
normal pre-merge workflow. Copilot's review 5557264376 (COMMENTED, 2026-10-17 09:46:28 UTC, at the
head `docs(issues): [#1349] record the S1 pull request #2558 in the specification`) has one inline
finding, F1. The reviewer wrote `[Minor][F1]` in the comment, and the overview marks it "Low
severity"; the comment's own tag is used. Its "Approval recommended" verdict and change summary are
non-actionable and add no row.

- Copilot's second review 5557812391 (COMMENTED, 2026-10-17 12:45:12 UTC, at the head
  `docs(pr-reviews): [#2558] record the review audit for the #1349 S1 PR`) reports no open finding
  and adds no row.
- da2ce7's review 5562047992 (APPROVED, 2026-10-18 08:05:58 UTC, at that same head) carries three
  inline findings with the IDs F1, F2 and F3, which collide with this audit's F1, so they are
  recorded as F2, F3 and F4 with the original IDs kept in the detail entries. His ACK comment names
  that head, and his review body raises nothing further to act on.
- da2ce7's second review 5571057583 (APPROVED, 2026-10-19 11:47:25 UTC, at the head
  `docs(pr-reviews): [#2558] record the second review round in the audit`) and his ACK comment of
  that head carry no finding.
- Copilot's third review 5571126185 (COMMENTED, 2026-10-19 11:53:47 UTC, at that same head) has one
  inline finding, F5. Its overview marks it "Low severity" and the comment is tagged `[Minor]`; the
  comment's own tag is used.
- da2ce7's third review 5571966713 (APPROVED, 2026-10-19 13:05:30 UTC, at the head `docs(pr-reviews):
  [#2558] record the third review round in the audit`) and his ACK comment of that head carry one
  inline `Nit` finding tagged `[Nit][F6]`. The ID does not collide with this audit's, so it is kept
  as F6 and its `Reviewer finding ID` is `N/A`.
- Copilot's third re-review 5571996499 (COMMENTED, 2026-10-19 13:07:53 UTC, at the head
  `docs(pr-reviews): [#2558] record the third review round in the audit`) reports no open finding, marks
  F5 resolved, and adds no row.
- Copilot's fourth review 5574223021 (COMMENTED, 2026-10-19 15:55:07 UTC, at the head `docs(pr-reviews):
  [#2558] record the fourth review round in the audit`) has one inline finding, F7, tagged
  `[Nit][F7]`; the ID does not collide, so its `Reviewer finding ID` is `N/A`. Its overview marks
  it "Low severity" and the comment's own tag is used.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2558-f1` | Copilot | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2558-f2` | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2558-f3` | Human | Suggestion | testing | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2558-f4` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | `review-finding:pr-2558-f5` | Copilot | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F6 | `review-finding:pr-2558-f6` | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F7 | `review-finding:pr-2558-f7` | Copilot | Nit | metadata | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The precedence test needs an external IP in the policy the plan refactors

- PR number: 2558
- Source review ID: 5557264376
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2558#discussion_r4302250313>
- Concern: the announce plan's test review treated the precedence test as having only a
  predicate-built expectation and an `is_ok()` assertion, but the test builds its service with
  `PeerIpSelectionPolicy::enabled()`, whose `external_ip` is `None`, so the external IP in its
  fixture never reaches the service and the test cannot detect the external IP taking precedence.
- Solution: the plan's test review now records the missing setup, and R1 gains a step that gives
  the test a policy with overrides enabled and a distinct external IP, and confirms it fails under
  a temporary change that prefers the external IP. This amends the approved plan a second time; the
  plan log asks the maintainer to confirm it when reviewing the R1 result.
- Current-tree verification: `PeerIpSelectionPolicy::enabled()` sets `external_ip: None`
  (`announce.rs` lines 89-93) and the test passes it at line 859; the plan's review row and the R1
  step both name the setup (`grep -n "external IP"` on the plan).
- Resolution reference: docs(issues): [#1349] add the external-IP setup to the precedence test in the announce plan
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2558#discussion_r4302644616>

### F2 - The plan names no case for the public URL the announce event carries

- PR number: 2558
- Source review ID: 5562047992
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2558#discussion_r4306277848>
- Concern: `HttpTrackerPolicy::from` maps the instance `public_url`, the service stores it and
  `send_event` attaches it with `with_public_url`, and `ConnectionContext`'s `PartialEq` compares it,
  but every announce test runs without a URL, so dropping that call or mapping the URL to `None`
  passes all eight tests, and the plan had no case for it.
- Solution: the plan has a decision row and a new R2k (an instance with a public URL publishes it
  in the event), the Module-owned decisions bullet mentions it, and R2 reads R2a-R2k. This is a
  third amendment to the approved plan, adding one test; the plan log asks the maintainer to
  confirm it at the R2 review.
- Current-tree verification: `announce.rs` lines 74, 192 and 342 are the mapping, the stored field
  and the `with_public_url` call; `announce_events_match` compares `*connection ==
  *expected_connection`; the plan's table has the row and R2 reads "R2a-R2k" (`grep -n "R2k"`).
- Resolution reference: docs(issues): [#1349] add the instance public URL case to the announce plan
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2558#discussion_r4313284088>

### F3 - Name where the temporary change that prefers the external IP goes

- PR number: 2558
- Source review ID: 5562047992
- Reviewer finding ID: F2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2558#discussion_r4306277851>
- Concern: the plan's R1 proof ("confirm it fails under a temporary change that prefers the
  external IP") and R2d's "temporary order swap" did not say where the change goes, so the next
  reviewer could not check them.
- Solution: a Named Mutations table (M1-M5) gives each proof its site and exact change. M1 is the
  `PeerIp::Literal(ip)` arm change the reviewer suggested; M2 swaps `authenticate` and `authorize`;
  M3 removes the `is_loopback()` guard; M4 maps `Stopped` to `None`; M5 deletes `with_public_url`.
  R1, R2 and the order row point at them. M1's expression was compile-checked with a temporary
  patch that was reverted, and `packages/` is unchanged.
- Current-tree verification: lines 211, 213, 215, 260, 288, 298 and 342 of `announce.rs` hold the
  statements the table names; `cargo check -p torrust-tracker-http-core` passed with M1 applied;
  `git diff -- packages` is empty.
- Resolution reference: docs(issues): [#1349] name the mutations that prove the announce tests
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2558#discussion_r4313284349>

### F4 - `ClientConnectionContext::port` has a caller: the uncalled `client_port`

- PR number: 2558
- Source review ID: 5562047992
- Reviewer finding ID: F3
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2558#discussion_r4306277855>
- Concern: the evidence said no caller exists for the four uncovered getters, but `port` is called
  by `ConnectionContext::client_port` (`event.rs` line 86), so it is dead only together with it.
- Solution: the evidence says the first three getters have no caller and that `port` is called only
  by the uncalled `client_port`, so the file plan removes both or neither.
- Current-tree verification: a search of `src`, `packages` and `console` finds no other caller of
  `client_port` or `ClientConnectionContext::port`; the two evidence rows carry the new wording.
- Resolution reference: docs(issues): [#1349] say that ClientConnectionContext::port is called by client_port
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2558#discussion_r4313284642>

### F5 - Mutation M5 does not compile under the workspace lints

- PR number: 2558
- Source review ID: 5571126185
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2558#discussion_r4313486866>
- Concern: deleting the `with_public_url` call removes the only read of the private
  `AnnounceService::public_url` field, and the workspace denies `unused` and `warnings`, so the
  documented M5 fails compilation before R2k runs and proves nothing.
- Solution: the plan's Named Mutations section says to run M5 with `RUSTFLAGS="--cap-lints=warn"`
  (as T6 does), to count it only when R2k runs and fails and not on a compile error, and to try a new
  mutation without the cap first. M1-M4 were compile-checked too and compile under the default
  lints.
- Current-tree verification: applying M5 gave `field public_url is never read` and exit 101 with the
  default lints, and exit 0 with the cap; M1-M4 gave exit 0 by default; `git diff -- packages` was
  empty afterwards; the plan's M5 row and the section note carry the new wording.
- Resolution reference: docs(issues): [#1349] run mutation M5 with the lint cap in the announce plan
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2558#discussion_r4313979913>

### F6 - F5's `Reviewer finding ID` should be `N/A`

- PR number: 2558
- Source review ID: 5571966713
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2558#discussion_r4314150959>
- Concern: the audit recorded `F5` as F5's `Reviewer finding ID`, but the skill says to record the
  original ID only when it was reassigned, and Copilot's `[Minor][F5]` tag did not collide, so the
  id was kept, as for its `[Minor][F1]`, which the audit records as `N/A`.
- Solution: F5's `Reviewer finding ID` is `N/A`. Only F2, F3 and F4, which were reassigned, carry
  original ids; F1, F5 and F6 kept their own ids.
- Current-tree verification: the comment is tagged `[Nit][F6]`. `grep -n "Reviewer finding ID"` on
  the audit gives `N/A` (F1), `F1` (F2), `F2` (F3), `F3` (F4), `N/A` (F5) and `N/A` (F6), and each
  matches the tag on its source comment (`[Minor][F1]`, `[Minor][F1]`, `[Suggestion][F2]`,
  `[Nit][F3]`, `[Minor][F5]`, `[Nit][F6]`); the skill text at `process-pr-review/SKILL.md` lines
  217-219 states the rule.
- Resolution reference: docs(pr-reviews): [#2558] record N/A as F5's reviewer finding ID
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2558#discussion_r4315768038>

### F7 - F6's source finding ID is wrong

- PR number: 2558
- Source review ID: 5574223021
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2558#discussion_r4315950536>
- Concern: F6 recorded `F1` as its source finding ID, but the source comment is tagged
  `[Nit][F6]`, so the ID was kept and `Reviewer finding ID` is `N/A`, as for F5. The intro bullet and
  the F6 entry's explanation and verification output repeated the wrong claim.
- Solution: the intro bullet, F6's `Reviewer finding ID` and its solution and verification text now
  say the ID was kept. I wrote `F1` without reading the comment. Every entry's recorded ID was then
  cross-checked against the tag on its source comment through the API, and F6 was the only mismatch.
- Current-tree verification: the six source comments are tagged `[Minor][F1]`, `[Minor][F1]`,
  `[Suggestion][F2]`, `[Nit][F3]`, `[Minor][F5]` and `[Nit][F6]`; the audit records `N/A`, `F1`, `F2`,
  `F3`, `N/A` and `N/A` for them; the audit validator passes.
- Resolution reference: docs(pr-reviews): [#2558] correct F6's source finding ID and the claims about it
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2558#discussion_r4316006471>

## Processing Log

- 2026-10-17 09:46 UTC - Copilot submitted review 5557264376 with the one inline finding.
- 2026-10-17 09:48 UTC - Found the review while polling every two minutes; verified the claim
  against `announce.rs`.
- 2026-10-17 09:49 UTC - Edited the plan to record the smell and the R1 step.
- 2026-10-17 12:21 UTC - Committed the fix after a GPG passphrase timeout was retried with the
  maintainer's approval.
- 2026-10-17 12:23 UTC - Pushed, replied on the thread and resolved it.
- 2026-10-17 12:24 UTC - Recorded this audit.
- 2026-10-17 12:45 UTC - Copilot's second review 5557812391 reported no open finding.
- 2026-10-18 08:05 UTC - da2ce7 submitted review 5562047992 (approval and an ACK of the head) with
  three inline findings.
- 2026-10-19 11:22 UTC - Found the review at the start of the next session and verified the three
  claims against the source.
- 2026-10-19 11:27 UTC - Committed the F2, F3 and F4 fixes (11:24, 11:26 and 11:27 UTC), one commit
  each.
- 2026-10-19 11:30 UTC - Pushed, replied on the three threads and resolved them.
- 2026-10-19 11:31 UTC - Recorded this round here.
- 2026-10-19 11:47 UTC - da2ce7 approved the head (review 5571057583) and posted an ACK with no
  finding.
- 2026-10-19 11:53 UTC - Copilot's third review 5571126185 raised F5 on that head.
- 2026-10-19 12:43 UTC - Found F5 while verifying the PR before posting the maintainer's ACK, and
  verified it by applying mutations M1-M5 and compiling each.
- 2026-10-19 12:44 UTC - Committed the F5 fix.
- 2026-10-19 12:47 UTC - Pushed, replied on the thread and resolved it.
- 2026-10-19 12:48 UTC - Recorded this round here.
- 2026-10-19 12:51 UTC - Pushed the third-round audit and re-requested review from da2ce7 and
  Copilot.
- 2026-10-19 13:05 UTC - da2ce7 approved the head (review 5571966713) and posted an ACK, with one
  `Nit` thread, F6.
- 2026-10-19 13:06 UTC - Found it while polling and verified the claim against the skill text.
- 2026-10-19 15:33 UTC - Committed the F6 fix after a GPG passphrase timeout was retried with the
  maintainer's approval; the wait spanned about two hours.
- 2026-10-19 15:36 UTC - Pushed, replied on the thread and resolved it.
- 2026-10-19 15:36 UTC - Recorded this round here.
- 2026-10-19 15:40 UTC - Pushed the fourth-round audit and re-requested review from da2ce7 and
  Copilot.
- 2026-10-19 15:55 UTC - Copilot's fourth review 5574223021 raised F7 on that head.
- 2026-10-19 15:56 UTC - Found it while polling and checked the source comment's tag through the API.
- 2026-10-19 15:58 UTC - Committed the F7 fix.
- 2026-10-19 16:01 UTC - Pushed, replied on the thread and resolved it.
- 2026-10-19 16:01 UTC - Recorded this round here.

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
