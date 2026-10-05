---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/open/2412-1488-si-16-migrate-standalone-http-environment/ISSUE.md
---

<!-- skill-link: process-pr-review -->

# PR #2439 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/2439>.

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

Copilot review 5416744478 (round 1, "Balanced" effort) left three inline comments with
reviewer-provided IDs and severities (`[Major][F2]`, `[Major][F1]`, `[Minor][F3]`); the audit keeps
those IDs. The review body is an overview only, so it creates no additional finding. Its badge
markup rates the two Major findings `Medium severity` and F3 `Low severity`; the inline brackets
are recorded as given.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F2 | `review-finding:pr-2439-f2` | Copilot | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F1 | `review-finding:pr-2439-f1` | Copilot | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2439-f3` | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F2 - A failed server start detaches the statistics listener

- PR number: 2439
- Source review ID: 5416744478
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2439#discussion_r4185609583>
- Concern: `Environment::start_with_health_check` spawned the statistics listener before the
  fallible server start. On a bind or registration failure, `expect` panicked and dropped the
  listener's handle while the task kept the only remaining token clone, so it was never
  cancelled.
- Solution: take the event receiver before the start, so the subscription exists before the
  server can publish, and spawn the listener only after the start succeeds. A failed start now
  leaves no task behind.
- Current-tree verification: new test
  `it_should_not_leave_the_statistics_listener_running_when_the_http_server_fails_to_start`
  occupies the port, keeps the container (and so the event bus) alive, and asserts a single holder
  of the statistics repository. With the previous order it fails with 2 holders. A first version
  that dropped the container passed even before the fix, because closing the bus ends a leaked
  listener; the test now states that condition. `cargo test -p torrust-tracker-axum-http-server`
  passes (43 + 61).
- Resolution reference: `fix(axum-http-server): [#2412] spawn the environment listener only after the server starts`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2439#discussion_r4186115365>

### F1 - A real drain timeout can leave `stop()` pending forever

- PR number: 2439
- Source review ID: 5416744478
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2439#discussion_r4185609715>
- Concern: `graceful_shutdown_on_cancellation` starts an unbounded graceful phase
  (`handle.graceful_shutdown(None)`). On `TimedOut` it returned without force-closing, so a
  connection held past the deadline kept the server task alive, and `join_owned_tasks` waited for
  it before reporting the timeout. The existing timeout test only passed because it dropped the
  connection itself.
- Solution: call `handle.shutdown()` at the deadline, which force-closes remaining connections
  (verified in axum-server 0.8.0: connection tasks still wait on the shutdown notifier during the
  graceful phase). This matches the legacy path, whose `graceful_shutdown(Some(90 s))` was
  bounded. The helper is shared, so the fix also applies to the REST API and health-check servers.
- Current-tree verification: new test
  `it_should_force_close_a_connection_still_open_when_the_drain_times_out` holds a real
  connection and requires the server task to finish after `TimedOut`; removing the force-close
  makes it fail with `Elapsed`. `cargo test` passes for `torrust-tracker-axum-server` (6),
  `-axum-http-server` (43 + 61), `-axum-rest-api-server` (6 + 58), and
  `-axum-health-check-api-server` (3 + 8); the `token_aware_drain` example still exits 0.
- Resolution reference: `fix(axum-server): [#2412] force-close connections when the token-aware drain times out`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2439#discussion_r4186115057>

### F3 - macOS builds the Unix signal implementation

- PR number: 2439
- Source review ID: 5416744478
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2439#discussion_r4185609794>
- Concern: the T4 progress-log entry said the Windows and macOS CI builds compile the example's
  non-Unix branch; macOS satisfies `cfg(unix)`, so only Windows does.
- Solution: corrected the entry in place, with a marked correction naming the original wording,
  and corrected the same sentence in the PR description.
- Current-tree verification: `grep -n 'Windows/macOS'` in the spec matches only the correction
  note that quotes the old wording.
- Resolution reference: `docs(issues): [#2412] correct the non-Unix CI claim and log the Copilot fixes`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2439#discussion_r4186115566>

## Processing Log

- 2026-10-05 15:32 UTC - Fetched review 5416744478 and its three inline threads with the
  `github-review-threads` tool; no human reviews.
- 2026-10-05 15:54 UTC - Committed fixes for F2 (15:45), F1 (15:52), and F3 separately; pushed.
- 2026-10-05 16:11 UTC - Replied on all three threads (16:07); updated the PR description;
  recorded the audit.

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
