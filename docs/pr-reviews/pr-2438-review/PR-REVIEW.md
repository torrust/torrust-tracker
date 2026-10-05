---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - "issue #2411"
---

<!-- skill-link: process-pr-review -->

# PR #2438 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/2438>.

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

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2438-f1` | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2438-f2` | Copilot | Major | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2438-f3` | Copilot | Major | correctness | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - A2 omits the health check API from its affected services

- PR number: 2438
- Source review ID: 5415361603
- Reviewer finding ID: F001
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2438#discussion_r4184624003>
- Concern: A2's `Affected` cell listed only HTTP and the REST API, while the same row's status records that the health check API sets none of the connection timeouts and the Existing Controls table maps that API to A2, so the row understated the case's scope.
- Solution: A2's `Affected` cell now lists the health check API in the inventory's list style (`HTTP, REST API, health check`); the case, effect, status and source cells are unchanged.
- Current-tree verification: the A2 row of `docs/issues/open/2411-spam-and-abuse-resistance/EPIC.md` inspected; the EPIC's 60 `path:line` citations resolve at `050726d46` with 0 failures; `linter markdown` and `linter cspell` run in the orchestrator's gate.
- Resolution reference: `docs(issues): [#2411] correct the A2, A6 and connection-ID rows after review`
- Follow-up PR URL: N/A
- Reply URL: <REPLY_URL_OR_NA>

### F2 - A6's case and effect contradict its confirmed status

- PR number: 2438
- Source review ID: 5415361603
- Reviewer finding ID: F002
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2438#discussion_r4184624096>
- Concern: A6's `Case` and `Effect` cells still described the scrape fix before it merged (option B growing memory, the same effect as A4 and A5), while its confirmed status says a scrape never adds a swarm, so the row contradicted itself.
- Solution: The `Case` cell now states the fixed behavior (scrape reads the database for authorized info hashes absent from memory when persistence is enabled) and the `Effect` cell states its consequence (database load via scrape, no memory growth because a scrape adds no swarm). The status, affected and source cells are unchanged.
- Current-tree verification: the A6 row inspected against `packages/tracker-core/src/scrape_handler.rs:72` and `packages/tracker-core/src/scrape_handler.rs:186` at `050726d46`; `linter markdown` and `linter cspell` run in the orchestrator's gate.
- Resolution reference: `docs(issues): [#2411] correct the A2, A6 and connection-ID rows after review`
- Follow-up PR URL: N/A
- Reply URL: <REPLY_URL_OR_NA>

### F3 - Connection-ID control overstates source-address binding

- PR number: 2438
- Source review ID: 5415361603
- Reviewer finding ID: F003
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2438#discussion_r4184624151>
- Concern: The UDP connection-ID validation row said an ID must be issued to the same source address, but the cookie module documents the fingerprint as arithmetic mixing rather than a MAC, so a cookie minted for one fingerprint can coincidentally pass for another; the wording overstated a control that later design work may rely on.
- Solution: The row's `What it bounds` cell now says the ID's issue time must fall within the cookie lifetime and that the cookie binds a fingerprint of the source address and the issue time by arithmetic mixing, not a MAC, so a cookie minted for one fingerprint can coincidentally pass for another; `packages/udp-core/src/connection_cookie.rs:66` is cited beside `packages/udp-core/src/connection_cookie.rs:159`.
- Current-tree verification: the connection-ID row inspected against `packages/udp-core/src/connection_cookie.rs:66` at `050726d46`, whose cited line is the module's "Fingerprint is NOT client authentication" section; `linter markdown` and `linter cspell` run in the orchestrator's gate.
- Resolution reference: `docs(issues): [#2411] correct the A2, A6 and connection-ID rows after review`
- Follow-up PR URL: N/A
- Reply URL: <REPLY_URL_OR_NA>

## Processing Log

- 2026-10-05 13:22 UTC - Opened the pull request with the inventory review commit, `docs(issues): [#2411] review the abuse-case inventory against the code`.
- 2026-10-05 13:40 UTC - Copilot review 5415361603 (round 1) recommended changes with three inline findings, recorded as F1 (`F001`, Minor), F2 (`F002`, Major) and F3 (`F003`, Major). Its overview restates them and adds no separate request, so it is summary context, not a row.
- 2026-10-05 13:43 UTC - Fixed F1, F2 and F3 in `docs(issues): [#2411] correct the A2, A6 and connection-ID rows after review`, changing only the three rows; the EPIC's citations resolve at `050726d46` with 0 failures.
- 2026-10-05 13:44 UTC - Recorded F1 to F3 here in `docs(pr-reviews): [#2411] add the PR #2438 review audit record`. The thread replies follow the push, so each Reply URL is the template's placeholder until its reply exists.

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
