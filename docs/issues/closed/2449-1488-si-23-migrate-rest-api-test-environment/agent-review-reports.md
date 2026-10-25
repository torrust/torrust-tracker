---
semantic-links:
  related-artifacts:
    - .github/agents/complexity-auditor.agent.md
    - .github/agents/task-reviewer.agent.md
    - .github/agents/pr-reviewer.agent.md
    - docs/agents/orchestration.md
---

# Agent Review Reports - Migrate the REST API Test Environment to the Token Lifecycle (#2449)

> Append one completed independent-review entry at a time. Do not modify, reorder, or remove
> earlier entries. A correction is a new entry that names the earlier conclusion.

## Reports

### 2026-10-24 20:30 UTC - Task Reviewer

- Invocation scope: pre-PR review of branch `2449-migrate-rest-api-test-environment` against
  `torrust/develop` (its three commits, `refactor(axum-rest-api-server): [#2449] migrate the REST API test environment to the token lifecycle`, `docs(shutdown): [#2449] record the migrated REST API test environment in the task inventory`, and `docs(issues): [#2449] record the SI-23 baseline, mutation proofs, and manual verification`): AC1-AC7, T1 tests, D1-D4,
  the token-shape point, the task inventory, spec hygiene, and the evidence file.
- Inputs: `ISSUE.md`, `manual-verification-evidence.md`, `git diff torrust/develop...HEAD`,
  `packages/axum-rest-api-server/src/testing/environment.rs`,
  `packages/axum-health-check-api-server/tests/server/contract.rs`,
  `docs/features/shutdown-process/task-inventory.md`, the `review-task` and `write-unit-test` skills.
- Evidence (stable `rustc 1.99.1`, `cargo 1.99.1`):
  - `cargo test -p torrust-tracker-axum-rest-api-server -p torrust-tracker-axum-health-check-api-server`:
    health-check 3 + 8, REST API lib 14, REST API integration 58, all passed (the counts of the
    evidence file hold; REST API integration took 2.30 s). The eight `testing::environment` tests
    passed in six further consecutive runs (1.06 s each, no flake).
  - `cargo clippy ... --all-targets -D warnings`, `cargo +nightly fmt --check`, `linter markdown`,
    `linter cspell`: clean. `linter all` and the pre-push checks were not run (the spec leaves them open).
  - Mutations re-run on the committed file and restored (`git diff --quiet HEAD` exited 0 after each):
    E (sequential join, early return on a join error) failed s5 and s8; F (a drain `TimedOut`
    ignored) failed s6, s7 and s8. Both match the evidence table.
  - Grep over the workspace (`src/`, `tests/`, `console/`, `contrib/`, `docs/`, excluding closed specs and
    PR reviews): the only users of the REST API `testing::environment` are the REST API integration tests
    (`Started` only) and the health-check contract test; no `.server` field use, no `ApiServer::start`
    or `ApiServer::stop` call remains in the environment; `grep -rn "\.stop()" packages/axum-rest-api-server/tests`
    gives 58.
  - Code read: one token, one `DropGuard` (disarmed in `stop()`, then cancelled); a fresh token per
    `start()`; `join_owned_tasks` joins both handles concurrently with `tokio::join!`, reports every failure,
    and returns the launcher only when nothing failed; no handle is left detached on `stop()`. On drop only
    the cancellation is requested and the handles are detached; the type comment states this.
- Findings:
  - **F1 (blocking, convention) - no recorded prose-first Arrange-Act-Assert comparison.**
    `manual-verification-evidence.md` and `ISSUE.md` record mutation proofs but no prose-first comparison
    for the eight new tests (the only match for "prose" is the generic commit-point paragraph at
    `ISSUE.md:191`). `write-unit-test/SKILL.md:332` requires it recorded in the task evidence; the
    `review-task` Test Design checklist cannot pass without it.
  - **F2 (low, test design) - the production Act is inside a helper in s1, s2, s4.**
    `environment.rs:348` (`stop_within_deadline` wraps `environment.stop()`) and `:342`
    (`start_within_deadline` wraps `environment.start()`) are the Act of
    `it_should_stop_without_a_failure_...` (:358), `it_should_release_the_rest_api_binding_when_stopped`
    (:373) and `it_should_serve_requests_after_being_stopped_and_started_again` (:406; its Arrange also
    runs a whole stop through nested helpers). s5, s6, s7, s8 show the Act directly with the deadline.
    The helpers are coherent and named for their capability, so this is a note: either keep them and say in
    the prose-first record that the deadline wrapper is irreducible, or call `stop()` / `start()` under
    `tokio::time::timeout` in the test body.
  - **F3 (low, test design) - a failed deadline in s1 is reported as a stop failure.**
    `environment.rs:358-371` wraps `stop_within_deadline` in `tokio::spawn` and asserts `stop.is_ok()`, so a
    test-deadline expiry (an `expect` panic inside the spawned task) and a `stop()` panic produce the same
    message "should join ... without a failure". Not wrong; keep as is or distinguish in the message.
  - **F4 (info, test design) - polling in s3.** `environment.rs:386-402` (loop at :397) polls `TcpListener::bind` every 10 ms
    under a 10 s deadline. The skill discourages polling, but the spec (T1 list) requires exactly this,
    citing #2471, and the wait is bounded. Accepted.
  - **F5 (low, docs formatting) - badly wrapped list item.** `task-inventory.md:371` (point 8 of the
    conclusions): the added "as does the REST API one (SI-23, #2449). Final process outcome-to-exit-code and"
    makes the line about 140 characters while the lines around it wrap near 80; re-wrap the paragraph.
  - **F6 (info, spec) - evidence is honest where it is not run.** V2 says the 1 s drain poll is
    "read from the spec's Risks section ... no profile was taken"; the poll interval is real
    (`packages/axum-server/src/signals.rs:59`, one-second sleep). The drop-path claim at
    `ISSUE.md` ("read from source, not run") is still marked as such; the new drop test now covers the new
    path but not the legacy one, so the marker stays correct.
  - **F7 (info) - workspace and API.** The removed public `server` field is the only breaking change; its one
    consumer is migrated. `Environment<S>` lost the `Debug + Display` bound (now `S: Sync`), which widens, not
    narrows, the API. No leftover `Halted` or `ApiServer::start`/`stop` use in the environment.
- Acceptance criteria:
  - AC1 PASS (`start()` calls only `ApiServer::new` and `start_with_cancellation`; V1 lines match the tree).
  - AC2 PASS (cancel, join both; s1, s2; mutations A and B in the evidence, E and F re-run here).
  - AC3 PASS (s8 plus the `join_owned_tasks` tests; signature unchanged).
  - AC4 PASS (s4; fresh token in `start()`).
  - AC5 PASS (`contract.rs:121` uses `service.stop()`; 8 integration tests pass).
  - AC6 PASS (inventory table row and conclusions 4 and 8 updated; the F5 formatting note remains).
  - AC7 PASS (s3; mutation H isolates it in the evidence).
  - Behaviour is verified, but the checkboxes are left unchanged because the review fails on F1 and
    `linter all` and the pre-push checks are open; the spec says the AC rows stay `TODO` until a reviewer
    validates them. All rows in `ISSUE.md` correctly stay `TODO` with evidence pointers.
- Spec hygiene: workflow checkpoints are correct for the pre-PR state ("Automatic verification",
  "Acceptance criteria reviewed", "Reviewer validated", "Independent reviewer reports" unchecked); stamps agree
  (`last-updated-utc` and the last progress-log entry are both 2026-10-24 20:24, the commits fall within
  20:18-20:24 UTC); the T4 status `IN_PROGRESS` is right. The ISSUE.md checkpoint "Manual verification
  scenarios executed and recorded" is accurate (V1, V2, both run against the finished artifact).
- Completion review: the progress log says "No retrospective yet ... to be reconsidered after the independent
  review", which is neither a retrospective nor a final rationale. The implementation deviated from the plan
  (the extra `stop()` panic test, generic `E: Display`, removal of the public field; a mutation that
  swallowed failures went undetected). Material, reusable lessons exist (mutation D does not isolate the drop
  clause; H does; a non-compiling mutation runs no test), so a `implementation-retrospective.md` is
  warranted, or a progress-log entry with a concrete reason why not.
- Verdict: REVIEW FAILED
- Follow-up actions:
  - Record the prose-first Arrange-Act-Assert comparison for the eight tests in
    `manual-verification-evidence.md` (F1), noting the deadline-wrapper helpers' rationale (F2).
  - Re-wrap `task-inventory.md:371` (F5).
  - Create `implementation-retrospective.md`, or add a final progress-log rationale, after this review's
    findings are handled.
  - Run `linter all` and the pre-push checks, record them, and tick the matching checkpoints.
  - Re-run this review after remediation; then commit with the report.

### 2026-10-24 20:38 UTC - Task Reviewer

- Invocation scope: second-round pre-PR review of branch `2449-migrate-rest-api-test-environment`
  against `torrust/develop`, after the remediation commit
  `docs(issues): [#2449] record the first review round, the retrospective, and the prose-first record`.
  This entry supersedes the conclusion `REVIEW FAILED` of the `2026-10-24 20:30 UTC` entry: it re-checks
  F1, F5, the completion review, evidence V5, and mutations H and G, and re-validates AC1-AC7.
- Inputs: `ISSUE.md`, `manual-verification-evidence.md` (V3 to V5), `implementation-retrospective.md`,
  `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md`, `docs/features/shutdown-process/task-inventory.md`,
  `packages/axum-rest-api-server/src/testing/environment.rs`, `git log` and `git diff torrust/develop...HEAD`,
  the `write-unit-test` skill (prose-first Arrange-Act-Assert section).
- Evidence (stable `rustc 1.99.1`, `cargo 1.99.1`; the working tree was clean at the start and at the end):
  - `git log --stat torrust/develop..HEAD -- '*.rs'` lists `.rs` changes only in
    `refactor(axum-rest-api-server): [#2449] migrate the REST API test environment to the token lifecycle`
    (`environment.rs` and the health-check `contract.rs`); the later commits are documentation, so V3 and V5
    describe the final code. `git diff torrust/develop...HEAD` touches 7 files and not `Cargo.lock` (a
    two-dot diff shows a lockfile dependency difference only because `develop` moved on).
  - `cargo test -p torrust-tracker-axum-rest-api-server -p torrust-tracker-axum-health-check-api-server`:
    health-check 3 + 8, REST API lib 14, REST API integration 58 (2.33 s), all passed.
  - `linter all`: exit 0 (markdown, lychee, yaml, toml, cspell, clippy, rustfmt, shellcheck).
  - Mutation H (`Running` holds a plain `CancellationToken`, `stop()` still cancels it, the `stop()` panic
    test's hand-built `Running` adapted, the unused import removed because the crate denies warnings): only
    `it_should_release_the_rest_api_binding_when_dropped_without_being_stopped` failed (10 s deadline), 7 passed.
    The first attempt did not compile (unused import) and was discarded, as the evidence file warns.
  - Mutation G (`stop()` swallows the failures): only
    `it_should_panic_naming_each_failing_task_when_stopped_with_failed_tasks` failed, 7 passed.
    After each mutation `git checkout -- <file>` and `git diff --quiet HEAD` exited 0.
  - V4 compared with `environment.rs` test by test: s1 to s8 Arrange, Act and Assert prose match the code
    (s1 spawned `stop()` and `is_ok()`; s2 and s3 bind the address read before the stop or drop; s4 restart
    and `GET /api/health_check` 200; s5 panicking server task and a controller finishing one yield later;
    s6 exact `...server error; ...drain timeout` text; s7 exact drain-timeout text; s8 hand-built `Running`
    and the panic message naming both tasks). No mismatch.
  - `task-inventory.md` conclusion 8 is wrapped near 80 characters; no line of the paragraph is long.
  - A grep for 7 or more hexadecimal characters over the issue folder finds only the `rustc` version hash in
    `manual-verification-evidence.md:23`; no PR-branch commit id appears.
  - `last-updated-utc` of `ISSUE.md` and of the evidence file is `2026-10-24 20:33`, equal to the newest
    progress-log entry and earlier than the carrying commit (20:35 UTC).
- Findings:
  - **F1 (was blocking) - resolved.** `manual-verification-evidence.md` V4 holds the eight-row prose-first
    table and it matches the code. The record states honestly that it was written after the code and after the
    first review, and `implementation-retrospective.md` lists "write the prose while writing the tests" as an
    improvement. The skill asks for the record before maintainer review and commit; that ordering was missed
    and is disclosed, not hidden. Accepted, since the comparison now exists and changed no test body.
  - **F2 (was low) - resolved.** V4 states why `start_within_deadline` and `stop_within_deadline` stay: they
    add only the test deadline to one production call; s5 to s8 show the Act in the body. Sound.
  - **F3 (was low) - resolved.** The rationale holds from the code: the deadline `expect` panics inside the
    spawned task first with its own message. The claim that the mutation A run printed that message is read
    from the evidence file, not re-run here.
  - **F4 (was info) - accepted.** The bounded poll of s3 is required by the spec (T1 list).
  - **F5 (was low) - resolved.** `task-inventory.md` conclusion 8 is re-wrapped.
  - **F8 (info, cosmetic) - V4 s4 row omits the final cleanup.** The test ends with `stop_within_deadline(restarted)`
    after the assertion; the Arrange/Act/Assert prose does not mention it. It is not part of the specified
    behaviour; no action needed.
- Acceptance criteria:
  - AC1 PASS (V1; `start()` calls `ApiServer::new` and `start_with_cancellation` only).
  - AC2 PASS (s1, s2; mutations A, B and the re-run H and G confirm the tests guard their clause).
  - AC3 PASS (s8 plus the `join_owned_tasks` tests; mutation G re-run; signature unchanged).
  - AC4 PASS (s4; mutation C in V3).
  - AC5 PASS (`contract.rs` uses `service.stop()`; 8 integration tests re-run and pass).
  - AC6 PASS (task-inventory table row, conclusions 4 and 8).
  - AC7 PASS (s3; mutation H isolates it, re-run here).
  - `linter all` PASS, relevant tests PASS, manual scenarios M1 and M2 PASS (V1, V2 run against the finished
    artifact; no `.rs` change since).
- Spec hygiene: the checkpoints ticked are those completed (automatic verification, manual verification,
  completion review); "Acceptance criteria reviewed", "Reviewer validated", "Independent reviewer reports",
  "Committer verified", and "Issue closed" are correctly unticked. The AC rows and the matching checklist
  items stay `TODO` with evidence pointers; this reviewer was told not to edit existing files, so the caller
  ticks them with this report. Claims that were not run are marked ("read from source, not run" in the spec;
  "no profile was taken" in V2).
- Completion review: `implementation-retrospective.md` follows the template sections, and its claims are
  true against the code (the added `stop()` panic test, the generic `E: Display` because `RuntimeError` has a
  private field, the D and H mutation pitfalls, the evidence pointers) and against the progress log (the
  maintainer's design-review approval). The overcorrection section is reasonable. The completion review is
  resolved.
- Verdict: REVIEW PASSED
- Follow-up actions:
  - Caller: tick the AC rows (`TODO` to `DONE`), the four AC-related checklist items that this review
    verified, and the "Acceptance criteria reviewed", "Reviewer validated" and "Independent reviewer reports"
    checkpoints in `ISSUE.md`, and update `last-updated-utc` and the progress log, then commit them with this
    report through the Committer.
  - Run the pre-push checks again only if a commit after the one that carries this report touches a
    `.rs` or manifest file.
