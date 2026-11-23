---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #3014 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/3014.

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

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| --- | --- | --- | --- | --- | --- | --- | --- |
| F1 | review-finding:pr-3014-f1 | Unknown | Major (inferred) | testing | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-3014-f2 | Unknown | Major (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| R3 | review-finding:pr-3014-r3 | Copilot | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| R2 | review-finding:pr-3014-r2 | Copilot | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| R1 | review-finding:pr-3014-r1 | Copilot | Minor | correctness | RE_RAISE_OF:F2 | NO_ACTION | SUPERSEDED |
| F3 | review-finding:pr-3014-f3 | Unknown | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-3014-f4 | Unknown | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-3014-f5 | Unknown | Minor | security | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-3014-f6 | Unknown | Major | security | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-3014-f7 | Unknown | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-3014-f8 | Unknown | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| R4 | review-finding:pr-3014-r4 | Copilot | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| R5 | review-finding:pr-3014-r5 | Copilot | Minor | correctness | ORIGINAL | FIXED | NON_RESOLVABLE |
| F9 | review-finding:pr-3014-f9 | Unknown | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-3014-f10 | Unknown | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| R6 | review-finding:pr-3014-r6 | Copilot | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| R8 | review-finding:pr-3014-r8 | Copilot | Minor | testing | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Health-check scope has no verification

- PR number: 3014
- Source review ID: 6027255837
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3014#discussion_r4670206351
- Concern: Greptile (P2): the health-check API joined the SI-B scope, but the tasks, regression tests and manual scenarios still covered only the three original servers, although the health-check handler returns probe and join-error text to clients.
- Solution: Added task T7b with its commit point, a health-check regression test (failed probe, panicking probe job, shutting-down reply) and manual scenario M6 to the SI-B specification.
- Current-tree verification: Read `packages/axum-health-check-api-server/src/handlers.rs` on the branch: `probe_services` puts `the health check job failed: <join error>` and the probe error into the client report.
- Resolution reference: docs(issues): [#3000] split the health-check reply statuses and add its verification
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3014#discussion_r4670523578

### F2 - Health-check failure status recorded wrongly

- PR number: 3014
- Source review ID: 6027369330
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3014#discussion_r4670298238
- Concern: Greptile (P2): the inventory said the health-check API returns 503 for failed probes; failed probes return 200 with `health check failed`, and only the not-ready path returns 503 with `shutting down`.
- Solution: The SI-B background now records the shutting-down reply (503) and the failed-probe reply (200) separately, and says the statuses stay.
- Current-tree verification: `handlers.rs:28-33` returns `StatusCode::SERVICE_UNAVAILABLE` only when `readiness.is_ready()` is false; `probe_services` returns `Json<Report>` (200) from `responses::error("health check failed", ...)`. Tests `tests/server/contract.rs:147-188` and `:711-741` exist on the branch.
- Resolution reference: docs(issues): [#3000] split the health-check reply statuses and add its verification
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3014#discussion_r4670524038

### R3 - CI-tools draft lacks the test-development gate

- PR number: 3014
- Source review ID: 6027563286
- Reviewer finding ID: R3
- Source URL: https://github.com/torrust/torrust-tracker/pull/3014#discussion_r4670461881
- Concern: Copilot: T2 and T3 produce tests, but the draft did not require the red/green loop, the recorded prose-first Arrange-Act-Assert review after each increment, or the maintainer checkpoint that `docs/templates/ISSUE.md:118-130` requires.
- Solution: Added the test-development loop paragraph used by the sibling specifications, task T4b (maintainer checkpoint), AC4 and its verification row, and changed the T2 and T3 commit policy to name the red and green runs and the review.
- Current-tree verification: Compared with `docs/templates/ISSUE.md:118-130` and the "Implementation Plan" paragraph of `docs/issues/open/2999-2993-si-a-keep-secrets-out-of-errors-and-logs/ISSUE.md`.
- Resolution reference: docs(issues): add the test loop and per-site log scenarios to the CI-tools draft
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3014#discussion_r4670524934

### R2 - CI-tools manual check misses the DEBUG and compose sites

- PR number: 3014
- Source review ID: 6027563286
- Reviewer finding ID: R2
- Source URL: https://github.com/torrust/torrust-tracker/pull/3014#discussion_r4670461936
- Concern: Copilot: the E2E runner fixes tracing at INFO, so the DEBUG `docker run` event never appears, and the compose command paths are not exercised; a clean M1 log cannot show those sites are redacted.
- Solution: Split the manual check into M1 (INFO configuration lines), M2 (`docker run` event at DEBUG) and M3 (the qBittorrent E2E runner, which uses `compose.rs`, for both compose command lines), each run as a baseline and after the fix with evidence that the site ran.
- Current-tree verification: `src/console/ci/e2e/runner.rs:74` calls `tracing_stdout_init(LevelFilter::INFO)`; `grep` shows `compose.rs` used by `src/console/ci/qbittorrent_e2e/services_setup.rs`.
- Resolution reference: docs(issues): add the test loop and per-site log scenarios to the CI-tools draft
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3014#discussion_r4670525417

### R1 - Health-check lifecycle reply vs failed-probe report

- PR number: 3014
- Source review ID: 6027563286
- Reviewer finding ID: R1
- Source URL: https://github.com/torrust/torrust-tracker/pull/3014#discussion_r4670461981
- Concern: Copilot: only the not-ready branch returns 503; failed or panicking probes return 200 with an error report. The inventory combined them.
- Solution: Same current-tree change as F2; no separate change.
- Current-tree verification: Same inspection as F2.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3014#discussion_r4670524515
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3014#discussion_r4670524515

### F3 - The database-password marker cannot run through the E2E runner

- PR number: 3014
- Source review ID: 6027811353
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3014#discussion_r4670663043
- Concern: CodeRabbit: the runner forces the SQLite driver after loading the input TOML, so a network-database input with a password cannot complete the run, and M1 could not reach the checker log with it.
- Solution: M1 now checks the admin-token marker in the `tracker config:` line (`runner.rs:80`), which is written before the container starts; the database-password case moved to the T2 unit test.
- Current-tree verification: `src/console/ci/e2e/runner.rs:95-96` sets `DATABASE_DRIVER_OVERRIDE` to `SQLITE_DRIVER`.
- Resolution reference: docs(issues): make the CI-tools draft scenarios runnable and say where markers enter
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3014#discussion_r4670921433

### F4 - The checker log carries service addresses only

- PR number: 3014
- Source review ID: 6027811353
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3014#discussion_r4670663043
- Concern: CodeRabbit (second assertion of the same comment): `tracker_checker.rs:88` logs the running-services JSON, so a marker in the input TOML never reaches it.
- Solution: The background table now says the checker log holds service addresses only today and is listed because the function logs whatever configuration it gets; its marker test is a unit test.
- Current-tree verification: `runner.rs:117-120` serializes `running_services` and passes it to `tracker_checker::run`.
- Resolution reference: docs(issues): make the CI-tools draft scenarios runnable and say where markers enter
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3014#discussion_r4670921433

### F5 - The POST seconds route echoes a key in its path rejection

- PR number: 3014
- Source review ID: 6027811353
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3014#discussion_r4670663057
- Concern: CodeRabbit: `POST /api/v1/key/<key>` extracts the segment as `u64`, and the axum rejection includes the rejected value in the 400 reply.
- Solution: SI-A item 1 covers the rejection with the 400 status kept; the regression strategy adds a test that posts a key to that route.
- Current-tree verification: `packages/axum-rest-api-server/src/v1/context/auth_key/handlers.rs:65` is `Path<u64>`; `tests/server/v1/asserts.rs:129` pins `Invalid URL: Cannot parse ... to a u64`.
- Resolution reference: docs(issues): [#2999] cover the POST path rejection, malformed key segments and the client GET exceptions
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3014#discussion_r4670921850

### F6 - Redact the key segment whatever its shape

- PR number: 3014
- Source review ID: 6027811353
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3014#discussion_r4670663071
- Concern: CodeRabbit: a rule that redacts only key-shaped segments leaves a malformed key, such as a valid key with a stray character, in the log.
- Solution: SI-A item 3 now requires redaction of every segment after `/announce/`, `/scrape/` and `/api/v1/key/`, and the redaction test includes malformed keys.
- Current-tree verification: The spec text was the only place the choice was open ("decided at implementation"); no code exists yet.
- Resolution reference: docs(issues): [#2999] cover the POST path rejection, malformed key segments and the client GET exceptions
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3014#discussion_r4670922327

### F7 - AC2 claims health-check database coverage

- PR number: 3014
- Source review ID: 6027811353
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3014#discussion_r4670663080
- Concern: CodeRabbit: AC2 required a database-driver-text check through the health-check API, but no test or scenario showed one, and it is not clear a database failure can reach it.
- Solution: AC2 names the three servers that reach the database; AC2b covers the health-check failed-probe text and statuses.
- Current-tree verification: `grep -rn -i "database|sqlx" packages/axum-health-check-api-server/src` finds nothing.
- Resolution reference: docs(issues): [#3000] limit the driver-text criterion to the servers that reach the database
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3014#discussion_r4670922736

### F8 - A log entry claimed a push before it happened

- PR number: 3014
- Source review ID: 6027811353
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3014#discussion_r4670663085
- Concern: CodeRabbit: the 18:58 Processing Log entry said the fix commits were pushed together with the record, written before that push.
- Solution: Removed the push clause from that entry and added an entry with the push time taken from GitHub (the Docs Lint run created for head `12c44a815`).
- Current-tree verification: `gh-bot run list --branch 2993-refresh-specs-spec` shows the run for that head created at 2026-11-23T19:05:10Z.
- Resolution reference: docs(pr-reviews): [#2993] record review round 2 of PR 3014
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3014#discussion_r4670925538

### R4 - CI-tools markers cannot reach the checker or compose logs

- PR number: 3014
- Source review ID: 6028085158
- Reviewer finding ID: R4
- Source URL: https://github.com/torrust/torrust-tracker/pull/3014#discussion_r4670871714
- Concern: Copilot: the token and password markers never reach `tracker_checker.rs:88`, and M3 did not say how a marker enters `DockerCompose::with_env`, whose caller passes image, port and path values only. The first point is the same as F4.
- Solution: Same change as F3 and F4 for the checker; M3 now says the compose environment holds no credential today and the T3 unit test injects the marker through `DockerCompose::with_env`.
- Current-tree verification: `src/console/ci/qbittorrent_e2e/services_setup.rs:123-160` passes image names, ports and paths.
- Resolution reference: docs(issues): make the CI-tools draft scenarios runnable and say where markers enter
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3014#discussion_r4670923149

### R5 - REST client exceptions to header-token sending

- PR number: 3014
- Source review ID: 6028085158
- Reviewer finding ID: R5
- Source URL: https://github.com/torrust/torrust-tracker/pull/3014#pullrequestreview-6028085158
- Concern: Copilot (review body, "previously missed"): `get_request` sends no token, GET helpers keep a caller-supplied `Authorization` header, and the free GET helper wraps its send error at `:552`; the specs said the header is sent on every request.
- Solution: SI-A item 6 and the EPIC A6 evidence line name both exceptions and include `:552` in the `without_url` audit.
- Current-tree verification: `packages/rest-api-client/src/v1/client.rs:514-517` (`get_request` calls `get_result(url, None, None)`), `:486-489`, `:552`.
- Resolution reference: docs(issues): [#2999] cover the POST path rejection, malformed key segments and the client GET exceptions
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3014#issuecomment-7111763293

### F9 - The M3 baseline had no marker to show

- PR number: 3014
- Source review ID: 6028299786
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3014#discussion_r4671046950
- Concern: CodeRabbit (round 3, on head `8ce3ce3f8`): the note under the CI-tools draft's manual scenarios said every baseline shows the marker, but M3 has no marker because no credential reaches compose today.
- Solution: The note now says the baselines of M1 and M2 show the marker, and that the M3 baseline records the environment values the two compose lines print; the final run shows they are no longer printed.
- Current-tree verification: Read the note and the M3 row of `docs/issues/drafts/ci-tools-keep-secrets-out-of-logs/ISSUE.md` on the branch; `src/console/ci/qbittorrent_e2e/services_setup.rs:123-160` passes image names, ports and paths only.
- Resolution reference: docs(issues): say what the M3 baseline records in the CI-tools draft
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3014#discussion_r4671124313

### F10 - A workflow-run time was recorded as the push time

- PR number: 3014
- Source review ID: 6028344228
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3014#pullrequestreview-6028344228
- Concern: CodeRabbit, in its reply https://github.com/torrust/torrust-tracker/pull/3014#discussion_r4671082938 inside the F8 thread (the source URL is that reply's review, because the finding is a reply and not the first comment of a thread; the F8 thread is resolved): the 19:05 entry took the push time from the Docs Lint run creation time, which does not establish the exact push time; check the push event or call it an approximation.
- Solution: The 19:05, 19:39 and 19:50 entries now call the run time an upper bound on the push time. GitHub has no exact record: the PR timeline shows commit times only, and the fork's event list has no event for these pushes.
- Current-tree verification: `gh-bot api repos/torrust/torrust-tracker/issues/3014/timeline` lists `committed` events with commit dates and one `head_ref_force_pushed` (18:36:13 UTC, the rebase); `gh-bot api repos/josecelano-bot/torrust-tracker/events` lists no push to `2993-refresh-specs-spec`.
- Resolution reference: docs(pr-reviews): [#2993] call the workflow-run times upper bounds on the push times
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3014#discussion_r4671273059

### R6 - A placeholder was left in the F10 reply URL

- PR number: 3014
- Source review ID: 6028703592
- Reviewer finding ID: R6
- Source URL: https://github.com/torrust/torrust-tracker/pull/3014#discussion_r4671370711
- Concern: Copilot (on head `dc8ac4473`): the F10 entry still had the `REPLY_F10` placeholder, so the record could not pass the validator.
- Solution: The next commit had already replaced it with the reply URL; no further change.
- Current-tree verification: The F10 entry's Reply URL is `https://github.com/torrust/torrust-tracker/pull/3014#discussion_r4671273059`; the validator returned `status ok` on head `1f00028c2`.
- Resolution reference: docs(pr-reviews): [#2993] record the F10 reply and the validator run in the PR 3014 audit
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3014#discussion_r4671451408

### R8 - The UDP health-check texts were missing

- PR number: 3014
- Source review ID: 6028703592
- Reviewer finding ID: R8
- Source URL: https://github.com/torrust/torrust-tracker/pull/3014#discussion_r4671370607
- Concern: Copilot: the health-check report also forwards the UDP receive error's `Display` and the client construction error's `Debug` (`packages/tracker-client/src/udp/client.rs:245-286`), which the reqwest and join-error checks would miss.
- Solution: The SI-B inventory, the health-check regression test (reusing the junk-response contract test), T7b, AC2b and M6 now cover the UDP texts, with the 200 status kept.
- Current-tree verification: `client.rs:245-286` returns `Err(e.to_string())` and `Err(format!("{e:?}"))`; `packages/axum-health-check-api-server/tests/server/contract.rs:514` is `it_should_return_error_when_the_registered_udp_service_answers_the_check_with_junk`.
- Resolution reference: docs(issues): [#3000] add the UDP health-check failure texts
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3014#discussion_r4671450945

## Processing Log

- 2026-11-23 18:58 UTC - Started audit after the Copilot review 6027563286 (18:53 UTC); the two Greptile reviews 6027255837 (18:30 UTC) and 6027369330 (18:39 UTC) had arrived before it. Five findings, all checked against the branch.
- 2026-11-23 18:58 UTC - Replied on all five threads (replies 4670523578, 4670524038, 4670524515, 4670524934, 4670525417), before the two fix commits were pushed.
- 2026-11-23 19:05 UTC - The two fix commits and this record were pushed (head `12c44a815`) by this time at the latest: its Docs Lint run was created at 19:05:10 UTC. GitHub keeps no exact time for a plain push to the fork branch (the PR timeline has only commit times, and the fork's event list has no event for it), so the run time is an upper bound, not the push time. The five round-1 threads were resolved after the push.
- 2026-11-23 19:29 UTC - Round 2: CodeRabbit review 6027811353 (19:10 UTC, six assertions in five comments) and Copilot review 6028085158 (19:28 UTC, R4 inline and R5 in the body). Eight findings, all checked against the branch.
- 2026-11-23 19:33 UTC - Replied on the six inline threads (replies 4670921433, 4670921850, 4670922327, 4670922736, 4670923149, 4670925538) and answered R5 in PR comment 7111763293, before the fix commits were pushed.
- 2026-11-23 19:39 UTC - The round-2 fix commits and the record were pushed (head `8ce3ce3f8`) by this time at the latest (its Docs Lint run was created at 19:39:37 UTC; upper bound, as in the 19:05 entry). The six round-2 threads were resolved after the push.
- 2026-11-23 19:41 UTC - CodeRabbit posted five automatic replies (reviews 6028161719, 6028161801, 6028162354, 6028163012, 6028169121, 19:33-19:34 UTC) to the round-2 replies. Each checked head `12c44a815`, before the push, and said it could not verify the fix yet. No new finding. Answered in each thread that the fix is on head `8ce3ce3f8`.
- 2026-11-23 19:50 UTC - Round 3: CodeRabbit review 6028299786 (19:43 UTC) raised F9. Fixed, pushed (head `d7c45ec21`, pushed by 19:50:14 UTC at the latest, when its Docs Lint run was created), then replied (reply 4671124313, 19:50:16 UTC). A follow-up commit corrected the draft's `last-updated-utc`, which had been stamped ahead of the clock.
- 2026-11-23 19:57 UTC - CodeRabbit replies 6028337617 (19:46 UTC, F5 thread: verified, asked for the audit validator to be run) and 6028344228 (19:46 UTC, F8 thread: raised F10). Ran the validator (`status ok`, 0 failures) and recorded F10.
- 2026-11-23 20:03 UTC - The F10 fix was pushed, then replied in the F8 thread (reply 4671273059) and answered the validator request in the F5 thread (reply 4671273542).
- 2026-11-23 20:17 UTC - Round 4: Copilot review 6028703592 (20:10 UTC) raised R6 (on head `dc8ac4473`, already fixed by head `1f00028c2`, pushed by 20:11:10 UTC at the latest) and R8 (no R7 was posted). R8 was fixed and pushed (head `06eb49131`, by 20:17:08 UTC at the latest), then both threads were answered (replies 4671450945 and 4671451408).

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
