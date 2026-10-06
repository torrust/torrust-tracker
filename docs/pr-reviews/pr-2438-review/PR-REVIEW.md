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
| F4 | `review-finding:pr-2438-f4` | Human | Minor | metadata | ORIGINAL | FIXED | UNRESOLVED |
| F5 | `review-finding:pr-2438-f5` | Human | Nit | metadata | ORIGINAL | FIXED | UNRESOLVED |
| F6 | `review-finding:pr-2438-f6` | Human | Suggestion | documentation | ORIGINAL | FIXED | UNRESOLVED |

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
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2438#discussion_r4184991883>

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
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2438#discussion_r4184992253>

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
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2438#discussion_r4184992626>

### F4 - The #2417 sub-issue row claims an in-progress status

- PR number: 2438
- Source review ID: 5431638596
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2438#discussion_r4197940880>
- Concern: The alignment set the #2417 sub-issue row to `IN_PROGRESS`, but #2417 closed when #2444 merged, so the status is false at merge time; the archive PR #2457 sets the row to `DONE` and moves its link to the closed folder, and both PRs edit the row.
- Solution: The maintainer's first option: the row is restored to its `develop` text (`TODO`, the open-folder link), so this PR no longer changes it and #2457 owns the change; that also removes one of the hunks both PRs edit.
- Current-tree verification: the #2417 row of `docs/issues/open/2411-spam-and-abuse-resistance/EPIC.md` is identical to the row on `develop` at `9be79fc5a`; the Review basis line makes no claim about the row; `linter markdown` and `linter cspell` run in the orchestrator's gate.
- Resolution reference: `docs(issues): [#2411] record the HTTP request-size control and stamp the alignment after review`
- Follow-up PR URL: N/A
- Reply URL: (pending — recorded after the reply is posted)

### F5 - The stamp and the Progress Log predate the alignment commit

- PR number: 2438
- Source review ID: 5431638596
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2438#discussion_r4197940902>
- Concern: `last-updated-utc` still read `2026-10-05 13:13` after the alignment commit edited the EPIC on 2026-10-06, and the Progress Log had no entry for that edit; no convention exempts review-alignment commits from the stamp.
- Solution: `last-updated-utc` now carries the minute of the fix commit, and one Progress Log entry in the log's form (actor `da2ce7`) records what the alignment restated and what the fix adds.
- Current-tree verification: the EPIC's `last-updated-utc` reads `2026-10-06 16:43`, and its last Progress Log entry carries the same stamp and covers both commits; `linter markdown` and `linter cspell` run in the orchestrator's gate.
- Resolution reference: `docs(issues): [#2411] record the HTTP request-size control and stamp the alignment after review`
- Follow-up PR URL: N/A
- Reply URL: (pending — recorded after the reply is posted)

### F6 - The HTTP scrape cap does not bound query parsing

- PR number: 2438
- Source review ID: 5431638596
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2438#discussion_r4197940916>
- Concern: The HTTP scrape cap bounds decoding, but the whole query string is split and every `name=value` pair stored in a `MultiMap` before `Scrape::try_from` applies it, and the server configures no header or URI size limit, so only Hyper's default request buffer limit bounds that parsing; the Existing Controls table did not record it.
- Solution: A new `HTTP request size` row in Existing Controls (cases A1 and A3) cites the server's header-read and keep-alive timeout lines, states that no header or URI size limit is configured so Hyper's default buffer limit applies, and cites the parse order from the extractor through `Query::from_str` and its `MultiMap` insertion to the cap in `Scrape::try_from`; the A3 note gains the clause that the cap bounds decoding, not query parsing.
- Current-tree verification: at `9be79fc5a`, `packages/axum-server/src/custom_axum_server.rs:68` and `packages/axum-server/src/custom_axum_server.rs:72` set only the timeouts, and no `max_buf_size`, `max_headers`, body or URI limit appears in `packages/axum-server/src`, `packages/axum-http-server/src` or `src`; `packages/axum-http-server/src/v1/extractors/scrape_request.rs:70`, `packages/http-protocol/src/v1/query.rs:112`, `packages/http-protocol/src/v1/query.rs:120` and `packages/http-protocol/src/v1/requests/scrape.rs:66` confirm the parse order; the EPIC's 65 citations resolve at `7970cdf0a` with 0 failures; `linter markdown` and `linter cspell` run in the orchestrator's gate.
- Resolution reference: `docs(issues): [#2411] record the HTTP request-size control and stamp the alignment after review`
- Follow-up PR URL: N/A
- Reply URL: (pending — recorded after the reply is posted)

## Processing Log

- 2026-10-05 13:22 UTC - Opened the pull request with the inventory review commit, `docs(issues): [#2411] review the abuse-case inventory against the code`.
- 2026-10-05 13:40 UTC - Copilot review 5415361603 (round 1) recommended changes with three inline findings, recorded as F1 (`F001`, Minor), F2 (`F002`, Major) and F3 (`F003`, Major). Its overview restates them and adds no separate request, so it is summary context, not a row.
- 2026-10-05 13:43 UTC - Fixed F1, F2 and F3 in `docs(issues): [#2411] correct the A2, A6 and connection-ID rows after review`, changing only the three rows; the EPIC's citations resolve at `050726d46` with 0 failures.
- 2026-10-05 13:44 UTC - Recorded F1 to F3 here in `docs(pr-reviews): [#2411] add the PR #2438 review audit record`. The thread replies follow the push, so each Reply URL is the template's placeholder until its reply exists.
- 2026-10-05 14:16 UTC - Pushed the fix and the record after the hub gate passed, replied on the three threads, and recorded each reply URL in its entry.
- 2026-10-06 16:35 UTC - Human review 5431638596 (josecelano, round 2, at `docs(issues): [#2411] align the A3 note and the scrape-count control with #2444`) found the citations sound and raised three inline findings, recorded under his own IDs F4 (Minor), F5 (Nit) and F6 (Suggestion), which follow F1 to F3. He noted the merge order with the archive PR #2457, which edits the same #2417 row.
- 2026-10-06 16:43 UTC - Fixed F4, F5 and F6 in `docs(issues): [#2411] record the HTTP request-size control and stamp the alignment after review`: the #2417 row is back to its `develop` text, the stamp and a Progress Log entry record the alignment, and the HTTP request size control and the A3 clause are added; the EPIC's citations resolve at `7970cdf0a` with 0 failures.
- 2026-10-06 16:44 UTC - Recorded F4 to F6 here in `docs(pr-reviews): [#2411] record the PR #2438 round-2 findings F4-F6`. The replies follow the push, so each Reply URL is pending and each thread stays `UNRESOLVED` until its reply is posted.

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
