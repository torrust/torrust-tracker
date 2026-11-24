---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #3024 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/3024.

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

F1 to F7 come from CodeRabbit's review 6039690482 (COMMENTED on the first head, "docs(issues): [#2387] record order B′ and recheck the EPIC specs against develop"), in source order. CodeRabbit is a bot other than Copilot, so its author class is `Unknown`; severities are CodeRabbit's own labels.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-3024-f1 | Unknown | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-3024-f2 | Unknown | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-3024-f3 | Unknown | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-3024-f4 | Unknown | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-3024-f5 | Unknown | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-3024-f6 | Unknown | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-3024-f7 | Unknown | Minor | correctness | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - SI-15 AC1 conflicts with the pretty log style

- PR number: 3024
- Source review ID: 6039690482
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3024#discussion_r4680384881
- Concern: CodeRabbit noted that the SI-15 finding says the `pretty` style prints one event over several lines, while AC1 requires exactly one line, so AC1 is not testable as written.
- Solution: AC1 now requires one event with one `configuration` field, exactly one line in the single-line styles (the JSON default of #2766, `full`, `compact`), and excludes `pretty`, which prints every event over several lines by design. The finding line says the same.
- Current-tree verification: `grep -n "AC1\|pretty" docs/issues/drafts/2387-si-15-*/ISSUE.md` shows the new AC1 and the matching finding line.
- Resolution reference: docs(issues): [#2387] limit SI-15 AC1 to the single-line log styles
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3024#discussion_r4681126633

### F2 - SI-19 acceptance does not cover the failure and UDP latency paths

- PR number: 3024
- Source review ID: 6039690482
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3024#discussion_r4680384897
- Concern: CodeRabbit noted that AC1 to AC3 and M1 of SI-19 can pass while the three `response failed` handlers and the UDP trace still log `latency`.
- Solution: Added AC4 (the three `response failed` lines and the UDP `responded` trace carry an integer `latency_in_ms`, with a test for each path), extended AC1 to forbid a `latency` field in a capture at `trace`, and added manual scenario M2 for the failure and UDP paths, with its row in Acceptance Verification.
- Current-tree verification: `grep -n "AC1\|AC4\|^| M" docs/issues/drafts/2387-si-19-*/ISSUE.md` shows AC1, AC4, M1 and M2.
- Resolution reference: docs(issues): [#2387] check the failure and UDP latency paths in SI-19
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3024#discussion_r4681127237

### F3 - SI-20 excludes the request-line durations its rule covers

- PR number: 3024
- Source review ID: 6039690482
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3024#discussion_r4680384910
- Concern: CodeRabbit noted that the SI-20 finding puts request-line durations under the duration rule while Out of Scope excludes request-line checks beyond removed field names.
- Solution: Scope now says request lines are checked for the removed field names and the duration rule, produced by the health check probes and one UDP and one HTTP request the test sends; Out of Scope excludes request-line checks only beyond those two rules; the finding line points to them.
- Current-tree verification: `grep -n "Request lines\|request-handling" docs/issues/drafts/2387-si-20-*/ISSUE.md` shows the scope bullet and the narrowed exclusion.
- Resolution reference: docs(issues): [#2387] check request-line durations in the SI-20 log check
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3024#discussion_r4681127917

### F4 - SI-4 does not say how v2 keeps max_peer_timeout under the shared TrackerPolicy rename

- PR number: 3024
- Source review ID: 6039690482
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3024#discussion_r4680384916
- Concern: CodeRabbit noted that `TrackerPolicy` is shared with the v2 `Core` (`packages/configuration/src/v2_0_0/core.rs:45-46`), has a serde default and no `deny_unknown_fields`, so renaming its serde name would make a v2 `max_peer_timeout` fall back silently to 900; the draft's scope and v2 note must agree and T3 must name the test.
- Solution: Out of Scope now keeps v2 key names, `max_peer_timeout` included. A scope bullet requires that the rename does not change how a v2 file is read and leaves the mechanism (a v3-only type or serde name, or a v2 serde alias) to the specification; T3 and a new AC5 require a test that loads a v2 file with a non-default `max_peer_timeout` and gets that value. The Commit Points T3 row now matches the plan.
- Current-tree verification: `grep -n "v2\|AC5\|^| T3" docs/issues/drafts/2387-si-4-*/ISSUE.md` shows the scope bullet, the Out of Scope line, T3 in both tables and AC5.
- Resolution reference: docs(issues): [#2387] keep v2 max_peer_timeout readable in SI-4
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3024#discussion_r4681128403

### F5 - SI-7 says the health check JSON changes with the upstream fix

- PR number: 3024
- Source review ID: 6039690482
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3024#discussion_r4680384937
- Concern: CodeRabbit noted that SI-7's Out of Scope said the health check JSON keeps its slash until the crate changes, while the new finding says the upstream fix does not change `ServiceBinding::url()`.
- Solution: The Out of Scope line now says the JSON keeps its slash because it is filled with `ServiceBinding::url()`, which the `torrust-net-primitives` fix of `Display` does not change.
- Current-tree verification: `grep -n "health check API's JSON" docs/issues/drafts/2387-si-7-*/ISSUE.md` shows the corrected line.
- Resolution reference: docs(issues): [#2387] state that the health check JSON keeps its slash in SI-7
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3024#discussion_r4681128956

### F6 - No sub-issue owns the REST API Starting line that SI-9 relies on

- PR number: 3024
- Source review ID: 6039690482
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3024#discussion_r4680384950
- Concern: CodeRabbit noted that SI-9 logs settings on a REST API `Starting` line that does not exist until SI-10, which runs later, while SI-10 leaves duration names to SI-9.
- Solution: SI-10 owns the new `Starting` lines of the REST API and the health check API (as D4 assigns), logging their settings with the names of SI-9's rule; SI-9, which runs first, owns the rule and logs settings only on the `Starting` lines that exist today (UDP and HTTP trackers). SI-9's AC1 and finding and SI-10's Out of Scope say so.
- Current-tree verification: `grep -n "SI-10" docs/issues/drafts/2387-si-9-*/ISSUE.md` and `grep -n "naming rule" docs/issues/drafts/2387-si-10-*/ISSUE.md` show the split.
- Resolution reference: docs(issues): [#2387] give SI-10 the REST API and health check Starting lines
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3024#discussion_r4681129516

### F7 - SI-9 M1 expects a duration the issue does not log

- PR number: 3024
- Source review ID: 6039690482
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3024#discussion_r4680384962
- Concern: CodeRabbit noted that M1 requires every configured duration key in the log, including `event_listener_drain_in_secs`, which is not logged today and which the scope does not add.
- Solution: M1 now covers only the durations a lifecycle line logs today (the list of T1) and says durations not logged today, such as `event_listener_drain_in_secs`, are not expected; the finding line says why.
- Current-tree verification: `grep -n "^| M1\|event_listener_drain" docs/issues/drafts/2387-si-9-*/ISSUE.md` shows the narrowed M1 and the finding line.
- Resolution reference: docs(issues): [#2387] limit SI-9 M1 to the durations logged today
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3024#discussion_r4681130052

## Processing Log

- 2026-11-24 13:05 UTC - CodeRabbit's review 6039690482 (COMMENTED, submitted 13:04:59Z) with seven inline findings arrived on head "docs(issues): [#2387] record order B′ and recheck the EPIC specs against develop". The review poller had stopped with the previous session and did not watch CodeRabbit, so the review was found only when the maintainer asked at about 13:50 UTC whether the PR could merge; he had written an `ACK` on that head at 13:20:33Z. He chose to fix the findings before merging.
- 2026-11-24 14:04 UTC - Fixed F1 to F7 in seven commits, plus one commit that corrects the SI-3 finding about EPIC #2993 after PR 3025 (not a review finding). Pushed (pre-push hook passed; push finished 14:03:55 UTC by `date -u`), then replied on the seven threads (replies posted by 14:04:20 UTC). Started this audit after the replies, against the skill's order (record before the first reply); the reply URLs were known when it was written. The `ACK` of 13:20:33Z names the old head and is stale. Resolved the seven threads (`resolveReviewThread` returned `true` for each by 14:05:22 UTC; a GraphQL refresh shows no unresolved thread).

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
