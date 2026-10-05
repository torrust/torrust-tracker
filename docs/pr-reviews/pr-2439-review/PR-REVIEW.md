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

da2ce7 review 5417753246 (round 2, APPROVED at `bb54338c`) left seven inline findings numbered
F4-F10 to avoid colliding with Copilot's IDs; the audit keeps them. Its body adds one actionable
assertion without a thread, recorded as F11: the PR description's Validation still gave the
round-1 test counts.

da2ce7 review 5418725511 (round 3, CHANGES_REQUESTED at `17f17864`) left one inline finding that
it numbered F11. That ID was already this record's review-body finding, so it is recorded as F12
with the original ID kept. The review body raises no other new assertion.

da2ce7 review 5418876195 (round 4, CHANGES_REQUESTED at `e27c0f29`) left one inline finding,
F13, kept as given; its body confirms the F12 handling and adds no other assertion.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F2 | `review-finding:pr-2439-f2` | Copilot | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F1 | `review-finding:pr-2439-f1` | Copilot | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2439-f3` | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2439-f4` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | `review-finding:pr-2439-f5` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | `review-finding:pr-2439-f6` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | `review-finding:pr-2439-f7` | Human | Suggestion | testing | ORIGINAL | FIXED | RESOLVED |
| F8 | `review-finding:pr-2439-f8` | Human | Nit | testing | ORIGINAL | FIXED | RESOLVED |
| F9 | `review-finding:pr-2439-f9` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F10 | `review-finding:pr-2439-f10` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F11 | `review-finding:pr-2439-f11` | Human | Nit (inferred) | documentation | ORIGINAL | FIXED | NON_RESOLVABLE |
| F12 | `review-finding:pr-2439-f12` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F13 | `review-finding:pr-2439-f13` | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F2 - A failed server start detaches the statistics listener

- PR number: 2439
- Source review ID: 5416744478
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2439#discussion_r4185609583>
- Concern: `Environment::start_with_health_check` spawned the statistics listener before the
  fallible server start. On a bind or listener-setup failure, `expect` panicked and dropped the
  listener's handle while the task kept the only remaining token clone, so it was never
  cancelled. (A registration failure was not affected: that path cancels the token it was given,
  a clone of the environment's token, so the listener finished. Narrowed per da2ce7 F10.)
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

### F4 - Evidence V2 names its tree by a pre-rebase commit id

- PR number: 2439
- Source review ID: 5417753246
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2439#discussion_r4186389282>
- Concern: V2 cited branch commit `2f8a58bd`, which a rebase rewrote, so a reader could not tell
  which tree V2 and V3 ran against; the evidence template asks for commit subjects.
- Solution: V2 now names the commit by subject plus the then-uncommitted T4 example change; V1's
  durable `develop` id stays.
- Current-tree verification: `grep -n 2f8a58bd` in `manual-verification-evidence.md` finds
  nothing.
- Resolution reference: `docs(issues): [#2412] address da2ce7 review spec findings`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2439#discussion_r4187152197>

### F5 - Token-aware start docs list errors the path cannot return

- PR number: 2439
- Source review ID: 5417753246
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2439#discussion_r4186389288>
- Concern: the `# Errors` docs of `start_with_cancellation_and_health_check` (and the older
  `start_with_cancellation`) mentioned startup-notification errors; the token-aware path has no
  startup oneshot.
- Solution: both now name only `Error::Bind`, `Error::Listener`, and `Error::Registration`.
- Current-tree verification: `packages/axum-http-server/src/server.rs` docs of both methods;
  `cargo doc -p torrust-tracker-axum-http-server` reports no warnings.
- Resolution reference: `refactor(axum-http-server): [#2412] address da2ce7 review code findings`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2439#discussion_r4187152498>

### F6 - Example doc claims the whole HTTP library never listens for signals

- PR number: 2439
- Source review ID: 5417753246
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2439#discussion_r4186389292>
- Concern: AC7 was narrowed to direct subscriptions because the legacy path still subscribes
  indirectly until SI-19; the example's module doc still made the broad claim.
- Solution: the sentence now says the token-aware path the example uses never listens for
  signals.
- Current-tree verification: `packages/axum-http-server/examples/http_only_public_tracker.rs`
  module doc.
- Resolution reference: `refactor(axum-http-server): [#2412] address da2ce7 review code findings`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2439#discussion_r4187152790>

### F7 - Binding test's recorded mutation fails before its assertion

- PR number: 2439
- Source review ID: 5417753246
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2439#discussion_r4186389297>
- Concern: the listener-abort mutation made `stop()` panic before
  `it_should_release_the_http_binding_when_stopped` reached its bind, so it did not show the test
  guards binding release; the reviewer suggested a `stop()` that does not await the server task.
- Solution: ran the suggested mutation and two variants. Not awaiting the server task, and
  awaiting neither the server task nor the drain controller, both survive: cancellation alone
  makes axum stop accepting and drop the listening socket. A `stop()` that never stops the server
  (no cancel, handles dropped) fails the test at its own bind with `AddrInUse`; that is recorded
  as the proof (spec progress log 17:40 UTC, T2 row).
- Current-tree verification: mutations applied in the working tree only and reverted by hand;
  `git diff` after the revert held only the F8 test changes; the environment tests pass.
- Resolution reference: `docs(issues): [#2412] address da2ce7 review spec findings`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2439#discussion_r4187153097>

### F8 - Test startups are not bounded by a deadline

- PR number: 2439
- Source review ID: 5417753246
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2439#discussion_r4186389309>
- Concern: the spec requires every test await to have an explicit deadline; `stop()` was bounded
  but the environment starts were not.
- Solution: a `start_within_deadline` helper bounds every start; the failed-start test bounds its
  spawned task from outside, so a hang fails the test instead of passing as the expected panic.
- Current-tree verification: `grep -n '\.start()'` in `environment.rs` finds only the production
  `Environment::new` and the two bounded calls; `cargo test -p torrust-tracker-axum-http-server`
  passes (43 + 61).
- Resolution reference: `refactor(axum-http-server): [#2412] address da2ce7 review code findings`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2439#discussion_r4187153391>

### F9 - Shared drain-helper change missing from the spec's scope and completion review

- PR number: 2439
- Source review ID: 5417753246
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2439#discussion_r4186389323>
- Concern: the `handle.shutdown()` change affects every production caller but was recorded only in
  a log entry; its observable effect (the health-check API's 5 s budget now force-closes instead of
  being aborted at the 10 s supervisor deadline) was not stated.
- Solution: added design decision D7 and an In Scope item stating the production effect, and
  revisited the completion review.
- Current-tree verification: D7 in the spec; checked against
  `axum-health-check-api-server/src/server.rs` (5 s budget), `src/bootstrap/jobs/health_check_api.rs`
  (reports `TimedOut` as a component error), and `src/main.rs` (10 s deadline).
- Resolution reference: `docs(issues): [#2412] address da2ce7 review spec findings`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2439#discussion_r4187153616>

### F10 - F2's Concern includes registration failures

- PR number: 2439
- Source review ID: 5417753246
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2439#discussion_r4186389342>
- Concern: on a registration failure the start path cancels the token it was given, a clone of
  the environment's token, so the listener finished; only bind and listener-setup failures
  detached it.
- Solution: narrowed F2's Concern in this record, noting the correction.
- Current-tree verification: this record's F2 entry; `server.rs` cancels the token on
  registration failure.
- Resolution reference: `docs(pr-reviews): record da2ce7 review on #2439`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2439#discussion_r4187153859>

### F11 - PR description gives stale test counts

- PR number: 2439
- Source review ID: 5417753246
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2439#pullrequestreview-5417753246>
- Concern: the review body notes the PR description's Validation still said five environment
  tests and 42 + 61.
- Solution: edited the PR description to list 1 + 6 + 1 new tests and the current counts for the
  four packages that use the changed code, and to point to this record.
- Current-tree verification: `gh pr view 2439 --json body` shows the updated Validation section.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2439#issuecomment-6000204904>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2439#issuecomment-6000204904>

### F12 - Progress log claimed the F10 audit correction before it was committed

- PR number: 2439
- Source review ID: 5418725511
- Reviewer finding ID: F11
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2439#discussion_r4187209512>
- Concern: at `17f17864` the spec's 17:40 entry said "F10: corrected in the PR #2439 audit
  record", but this record was unchanged there: F2's Concern still included registration failures
  and rows F4-F11 were missing.
- Solution: the record commit was pushed next (`e27c0f29`), making the sentence true. The cause
  was ordering: the spec commit was pushed before the record commit.
- Current-tree verification: at `e27c0f29` this record's F2 Concern names bind and listener-setup
  failures only, and rows F4-F11 exist; the validator reports 11 rows, 0 failures.
- Resolution reference: `docs(pr-reviews): record da2ce7 review on #2439`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2439#discussion_r4187408212>

### F13 - Processing Log entry stamped before the events it records

- PR number: 2439
- Source review ID: 5418876195
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2439#discussion_r4187339307>
- Concern: the 17:40 UTC entry records the spec-fix commit and the push, which happened later
  (17:52:42Z and 17:58:46Z); only the code-fix commit (17:39:48Z) precedes the stamp.
- Solution: kept the 17:40 entry unchanged and appended a correction naming it, with the actual
  commit and push times, as the append-only rule requires.
- Current-tree verification: commit author times from `git log`; push time from the
  `review_dismissed` event of review 5417753246 at tip `17f17864`.
- Resolution reference: `docs(pr-reviews): correct the 17:40 audit log entry on #2439`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2439#discussion_r4187505083>

## Processing Log

- 2026-10-05 15:32 UTC - Fetched review 5416744478 and its three inline threads with the
  `github-review-threads` tool; no human reviews.
- 2026-10-05 15:54 UTC - Committed fixes for F2 (15:45), F1 (15:52), and F3 separately; pushed.
- 2026-10-05 16:11 UTC - Replied on all three threads (16:07); updated the PR description;
  recorded the audit.
- 2026-10-05 17:40 UTC - Fetched da2ce7 review 5417753246 (seven inline threads, one review-body
  finding); committed the code fixes (F5, F6, F8) and the spec fixes (F4, F7, F9) separately;
  pushed.
- 2026-10-05 18:03 UTC - Replied on the seven threads (18:02); updated the PR description (F11);
  posted the consolidated round-2 response; recorded round 2 and the F10 correction.
- 2026-10-05 18:31 UTC - Before resolving, the reply guard found da2ce7 review 5418725511
  (submitted 18:08, at `17f17864`, before the record commit was pushed). Recorded its finding as
  F12 and replied (18:31).
- 2026-10-05 18:42 UTC - Correction to the 17:40 UTC entry (F13): it was stamped before two of the
  events it records. Actual times: code-fix commit 17:39:48Z, spec-fix commit 17:52:42Z, push
  17:58:46Z. The 17:40 entry is left unchanged. Recorded da2ce7 review 5418876195 (submitted
  18:23, at `e27c0f29`) as F13 and replied (18:42).

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
