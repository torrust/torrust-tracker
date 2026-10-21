---
semantic-links:
  related-artifacts:
    - .github/agents/task-reviewer.agent.md
    - docs/issues/open/1349-1347-add-tests-http-core/ISSUE.md
---

# Agent Review Reports - Issue #1349 Add Tests to the http-core Package

> Append one completed independent-review entry at a time. Do not modify, reorder, or remove
> earlier entries. A correction is a new entry that names the earlier conclusion.

## Reports

### 2026-10-17 08:13 UTC - Task Reviewer (slice S1, first review)

- Invocation scope: Slice S1 (T1-T3, documentation only) of #1349 on branch
  `1349-1347-add-tests-http-core-inventory`: the three commits `docs(issues): [#1349] record http
  core coverage baseline and inventory`, `docs(issues): [#1349] add http core plan guidance and
  the announce test plan`, and `docs(issues): [#1349] record the approval of the announce plan and
  slices`.
- Inputs: `ISSUE.md`, `coverage-evidence.md`, `test-refactor-plans/README.md`,
  `test-refactor-plans/services-announce-tests.md`, the #1347 `EPIC.md`, the subissue guidelines,
  the raw coverage reports and `packages/http-core/src/`.
- Evidence: the diff touches only five files under `docs/issues/open`; every recomputed coverage
  number, test count and the 13-file list match; `linter markdown`, `linter cspell` and
  `linter lychee` pass; no branch commit ids or closing keywords were added.
- Findings:
  - MEDIUM: `coverage-evidence.md` classified the zero region at `listener.rs:108` as logging; it is
    the `TcpAnnounce` alternative of `event_connection_id`, which runs for every event, and no
    listener test dispatches a `TcpAnnounce` event.
  - MEDIUM: `coverage-evidence.md` described the scrape region at line 127 as the no-key path; it is
    the `Some(key)` arm.
  - MEDIUM: the ledger showed the announce row `PENDING` ("no plan yet") although its plan was
    approved, and workflow step 0 requires `IN_PROGRESS`.
  - LOW: "2 doc tests" in `statistics/repository.rs` is stale; there are 3.
  - LOW: the plan's test review missed that the tcp6 announce event test name describes a peer-IP
    change its Arrange does not set up.
  - LOW: the missed-line counts in the coverage summary exceed the listed lines by one or two in
    five files, with no note.
  - LOW (optional): the announce plan is longer than the roughly 40 lines the guidelines suggest,
    and the EPIC row mentions only T1.
- Verdict: REVIEW FAILED
- Follow-up actions:
  - Fix the first six findings and re-run the Task Reviewer. The two optional items are not blocking.

### 2026-10-17 08:21 UTC - Task Reviewer (slice S1, re-review)

- Invocation scope: Re-review of S1 after the fixes for the 2026-10-17 08:13 UTC report, in the
  commits `docs(issues): [#1349] correct the listener, scrape and doc-test evidence`,
  `docs(issues): [#1349] mark the announce ledger row in progress` and `docs(issues): [#1349]
  record the misleading tcp6 event test name in the announce plan`.
- Inputs: the same files and sources as the first review, plus the three fix commits.
- Evidence: each of the six fixes was verified against the source lines and the raw reports; the
  recount of listed against missed lines gave a gap of one or two in every case; only one ledger row
  is `IN_PROGRESS`; the diff against `torrust/develop` is still documentation only; `linter
  markdown`, `linter cspell` and `linter lychee` pass.
- Findings:
  - LOW: `ISSUE.md` `last-updated-utc` and the progress log do not record the review-fix round.
  - LOW: the plan gained the tcp6 rename after its approval, but R1's description and the plan log
    do not mention the amendment.
- Verdict: REVIEW PASSED
- Follow-up actions:
  - Record the review round in the progress log and the plan log, and add the rename to R1.

### 2026-10-17 09:14 UTC - Task Reviewer (slice S1, re-measurement)

- Invocation scope: The commit `docs(issues): [#1349] re-measure the http core baseline after #2244`
  on the S1 branch, rebased onto `develop` `a3810b813` after #2244 (merged through #2552) changed
  `statistics/metrics.rs`.
- Inputs: the S1 deliverables, the EPIC #1347 rows and log, and the raw reports of the second
  measurement in `.tmp/1349-coverage/` (`t1b-*`).
- Evidence: recomputed totals (lines 1,737 / 1,953, regions 2,335 / 2,583, functions 160 / 190);
  only `statistics/metrics.rs` differs from the first measurement and the uncovered-line lists are
  identical; every file carrying the baseline agrees; the stamps are real and chronological; the
  diff is documentation only; `linter markdown`, `linter cspell` and `linter lychee` pass.
- Findings:
  - MEDIUM: `coverage-evidence.md` still named the superseded commit `04013e7f1` as the base of the
    uncovered line numbers.
  - LOW: its reproduction commands wrote to the superseded `t1-*` reports instead of the `t1b-*`
    reports that hold the numbers.
  - LOW: "One commit did:" no longer read correctly above the added second-commit paragraph.
- Verdict: REVIEW FAILED
- Follow-up actions:
  - Fix the three items and confirm.

### 2026-10-17 09:23 UTC - Task Reviewer (slice S1, confirmation)

- Invocation scope: The commit `docs(issues): [#1349] point the coverage evidence at the
  re-measured reports`, which fixes the three findings of the 09:14 UTC report.
- Inputs: `coverage-evidence.md`, the other S1 deliverables, the EPIC, and the raw `t1b-*` reports.
- Evidence: the three fixes are present; the remaining `04013e7f1` mentions are dated history
  followed by entries that record the re-measurement; no new inconsistency; the diff against
  `develop` is documentation only; `linter markdown`, `linter cspell` and `linter lychee` pass.
- Findings:
  - None.
- Verdict: REVIEW PASSED
- Follow-up actions:
  - None.

### 2026-10-20 - Task Reviewer (slice S2)

- Time: the report returned after the S2 commits (the last at 14:57 UTC) and before the first fix at
  16:05 UTC; the exact time was not recorded.
- Invocation scope: Slice S2 (ledger rows 1-2: `services/announce.rs` and `services/error_mapping.rs`) of
  #1349 on branch `1349-1347-add-tests-http-core-announce`: the commits `docs(issues): [#1349]
  re-measure the http-core baseline before slice S2`, `test(http-core): [#1349] pin the announce
  service decisions with unit tests` and `test(http-core): [#1349] pin the failure reason of each
  tracker core error`.
- Inputs: `ISSUE.md`, `coverage-evidence.md`, both file plans, the plan guidance, the S1 reports, the
  EPIC, the `write-unit-test`, `review-task` and `add-rust-dependency` skills, and both source files.
- Evidence: production code before `#[cfg(test)]` is byte-identical to `develop` at `87690b15e`; the
  only manifest change is `rstest = "0.28.0"` (the latest stable; `"0"` would have locked 0.27.0);
  77 unit tests and 3 doc tests passed, also under `--test-threads=1` and on repeated runs (the
  thread-local stopped clock is not flaky); nightly `rustfmt`, `clippy -D warnings`, `cargo machete`,
  `git diff --check`, `linter markdown` and `linter cspell` passed; a clean `cargo llvm-cov` run
  reproduced every documented number, including the baseline measured in a worktree at `87690b15e`;
  mutations M2a, M3, M4, P1 and one extra (passing the remote client address instead of the peer IP to
  the announce handler) each failed only the expected tests and were reverted; the tree was clean after;
  the wildcard arm of `error_mapping.rs` cannot be reached by a test; the ledger, slice table, plan
  statuses, amendment confirmations, progress-log order and EPIC rows agree.
- Findings:
  - MEDIUM: mutant P2 survived all 77 tests: `PeerIp::Empty` returning the connection address and so
    skipping the loopback substitution that an absent `ip` gets. This contradicted the plan, the
    evidence and the progress log, which said no mutation survived the examples. By the plan's own
    rule a survivor must be fixed or the claim restated. (`announce.rs`, the `PeerIp::Absent |
    PeerIp::Empty` arm.)
  - LOW: mutation E2 as written does not compile under the workspace lints (`unused variable: source`);
    its recorded result holds only with `RUSTFLAGS="--cap-lints=warn"`. (`services-error-mapping-tests.md`,
    Named Mutations.)
  - LOW: `it_should_report_a_tracker_core_error_with_the_tracker_core_failure_reason` repeated the
    `Tracker whitelist error:` literal that `error_mapping.rs` now pins itself; optional.
- Verdict: REVIEW PASSED
- Follow-up actions:
  - Fix or restate the P2 finding before the pull request; fix the E2 wording; optionally remove the
    duplicated literal.
  - Handling (2026-10-20 16:05 UTC, commit `test(http-core): [#1349] kill the empty-ip loopback mutant
    and record the S2 review`): P2 is fixed by a two-case test that fails the mutant (the maintainer
    decision was the plan's own rule, not a restated claim); the E2 row now names the `{ source: _ }`
    form that was actually run; the duplicated literal is gone. Package coverage after the fixes:
    lines 91.34%, regions 92.45%, functions 89.59%, with 79 unit tests.

### 2026-10-20 - Task Reviewer (slice S3)

- Time: the report returned after the S3 commit (17:11 UTC) and before the first fix at 17:46 UTC; the
  exact time was not recorded.
- Invocation scope: Slice S3 (ledger row 3: `services/scrape.rs`) of #1349 on branch
  `1349-1347-add-tests-http-core-scrape`: the commits `docs(issues): [#1349] record the S2 merge and
  start slice S3` and `test(http-core): [#1349] pin the scrape service decisions with unit tests`.
- Inputs: `ISSUE.md`, `coverage-evidence.md`, both file plans (scrape and the announce template), the
  plan guidance, the S1 and S2 reports, the EPIC, the `write-unit-test` skill, and the three changed Rust
  files.
- Evidence: production code before `#[cfg(test)]` in `scrape.rs` and `announce.rs` is byte-identical to
  `develop` at `fe70cf908`; `lib.rs` gains only the moved `RecordingSender`; no manifest change;
  90 unit tests and 3 doc tests passed, also under `--test-threads=1`; nightly `rustfmt`,
  `clippy -D warnings`, `cargo machete`, `git diff --check`, `linter markdown` and `linter cspell`
  passed; a clean `cargo llvm-cov` run and a worktree at `fe70cf908` reproduced every documented number;
  M1 (compiling form), M2, M3 and M4 (with the lint cap) failed only the tests named for them and the
  extra mutants (b), (c), (e) and (f) were killed; dropping the planned "public tracker ignores a key"
  test was right; the ledger, slice table, plan status, progress logs, EPIC rows and signatures agree.
- Findings:
  - MEDIUM (F1): mutant (a) survived all 90 tests: the zeroed branch of `handle_scrape` returning early,
    which skips the client-IP resolution and the event. No test published an event or resolved the
    client IP on a private tracker; the replacement for the old `with_zeroed_data` event tests was never
    written.
  - LOW (F2): `RecordingSender` recorded when `send` was called, not when its future was polled, so
    `drop(sender.send(event))` instead of `.await` passed every test.
  - LOW (F3, informational): mutant (a'') (publish only for a non-empty result) survives, and is
    equivalent for any request the protocol accepts.
  - LOW (F4): the coverage document explained the fall in absolute counts too broadly: lines and regions
    fell, functions rose.
  - LOW (F5): mutant (d) (the HTTP-tracker constructor always enables the reverse proxy) was killed only
    incidentally, by the public-URL test.
  - LOW (F6): the ledger and slice table did not say that the R1 gate was satisfied by a combined review.
  - LOW (F7): the no-sender test seeded a peer that nothing asserted on.
- Verdict: REVIEW PASSED, with F1 required before the pull request.
- Follow-up actions:
  - Handling (2026-10-20 17:48 UTC, commit `test(http-core): [#1349] pin the private scrape path and
    record the S3 review`): F1 is fixed with a private-tracker event test and a private case in the
    unresolvable client-IP table, which fail the mutant; F2 is fixed by recording inside the returned
    future, after which the dropped-future mutant fails six tests; F4 is fixed by dropping the claim; F5 is
    fixed by a test that fails the mutant directly; F6 is recorded in the `ISSUE.md` log; F7 is fixed by
    asserting the data. F3 is recorded in the plan as an equivalent mutant and needs no test. The scrape
    file now has 22 cases and the package 93 unit tests: lines 1,949 / 2,102 (92.72%), regions 2,566 /
    2,743 (93.55%), functions 233 / 249 (93.57%).

### 2026-10-21 - Task Reviewer (slice S4)

- Time: the report returned on 2026-10-21 after the `container.rs` commit (07:01 UTC) and before the fix
  commit; the exact time was not recorded.
- Invocation scope: Slice S4 (ledger rows 4 and 5: `event.rs` and `container.rs`) of #1349 on branch
  `1349-1347-add-tests-http-core-events-container`, `fefefdd9e..HEAD`: the commits `docs(issues): [#1349]
  record the S3 merge and start slice S4`, `test(http-core): [#1349] pin the event label contract and
  connection context` and `test(http-core): [#1349] pin how the container composes the HTTP services`.
- Inputs: `ISSUE.md`, `coverage-evidence.md`, both file plans, the plan guidance, the S1-S3 reports, the
  EPIC, the `write-unit-test` skill and the two changed Rust files.
- Evidence: production code before `#[cfg(test)]` in `event.rs` is byte-identical to `fefefdd9e`, and
  `container.rs` only gains the appended test module; no other file under `packages/` or `src/` changed;
  135 unit tests, 1 doc test and 2 other tests passed, 12 more parallel runs and one with
  `--test-threads=1`; nightly `rustfmt`, `clippy -D warnings`, `cargo machete`, `git diff --check` and
  `linter all` passed; a clean `cargo llvm-cov` run and a worktree at `fefefdd9e` reproduced every
  documented figure; L2, L4, M3 and M6 failed only the tests named for them, the planned survivor P7
  survived as documented, and ten new mutants were tried (eight killed); no test pins behavior that looks
  like a bug; the ledger, slice table, plan status, progress logs and signatures agree.
- Findings:
  - LOW (F1): two new mutants survived in the `event.rs` label table: the `server_binding_protocol` label
    hard-coded to `http` (no case used HTTPS) and the `server_binding_address_ip_type` label hard-coded to
    `plain` (no case bound the server to an IPv4-mapped IPv6 address). Related INFO: every case used port
    7070.
  - LOW (F2): logic in two tests: a `match` over `ResolvedIp` computed the expected address
    (`it_should_expose_the_client_address_and_port` and the announce reverse-proxy table).
  - LOW (F3): tests with more than one reason to fail or a split Act: the bus-and-repository
    independence test, the whitelist test (its first announce ran in Arrange) and the `initialize` test
    (four concerns). The fresh-repository test compares with `Repository::new()`, which is acceptable
    because there is no literal for empty metrics.
  - LOW (F4): the `initialize` panic test used a fixed missing directory under the temp directory, which
    would stop panicking and could create a file if that directory ever existed.
  - INFO: the commit `docs(issues): [#1349] record the S3 merge and start slice S4` still states the stale
    claim that `http-core` did not change after S3 (corrected in the progress log and the coverage
    evidence; history is not rewritten); the EPIC row for S3 shows the S3-head figure, not the S4
    baseline; the event plan said "about 31 lines" for the removed helper where 18 counted lines were
    uncovered; the event plan's measurement time (20:00 UTC) cannot be verified.
- Verdict: REVIEW PASSED, with no finding required before the pull request.
- Follow-up actions:
  - Handling (2026-10-21, commit `test(http-core): [#1349] address the S4 independent review`): F1 is
    fixed with two label cases (an HTTPS server on port 443 and a server bound to `::ffff:127.0.0.1`),
    which kill the protocol, address-type and port mutants; F2 is fixed with an explicit `expected_ip`
    case parameter; F3 is fixed by splitting the three tests (the container now has 23 tests); F4 is fixed
    by taking the missing directory from a random name. The INFO items are corrected in the plans or the
    EPIC, except the commit message, which stays. After the fixes: 140 unit tests; `event.rs` 135 / 135
    lines; `container.rs` 598 / 600 lines with all 73 production lines covered; package lines 2,616 / 2,672
    (97.90%), regions 3,397 / 3,457 (98.26%), functions 316 / 322 (98.14%).

### 2026-10-21 - Task Reviewer (slice S5)

- Time: the report returned on 2026-10-21 after the `statistics/mod.rs` commit (10:3x UTC) and before the fix
  commit; the exact time was not recorded.
- Invocation scope: Slice S5 (ledger rows 6-10: `statistics/event/handler.rs`, `statistics/event/listener.rs`,
  `statistics/metrics.rs`, `statistics/repository.rs`, `statistics/mod.rs`) of #1349 on branch
  `1349-1347-add-tests-http-core-statistics`, `b5ab35ae9..HEAD`: the commits `docs(issues): [#1349] record the S4
  merge and start slice S5` and one commit per file.
- Inputs: `ISSUE.md`, `coverage-evidence.md`, the five file plans, the plan guidance, the S1-S4 reports, the
  EPIC, the `write-unit-test` skill and the five changed Rust files.
- Evidence: production code outside the test modules is byte-identical to `b5ab35ae9` in all five files and no
  other file under `packages/` or `src/` changed; 174 unit tests, the doc tests and the other test results passed,
  also 30 repeat runs, 3 runs with `--test-threads=1` and 10 runs pinned to one CPU; nightly `rustfmt`, `clippy -D
  warnings`, `cargo machete`, `git diff --check` and `linter all` passed; a clean `cargo llvm-cov` run and a
  worktree at `b5ab35ae9` reproduced every documented figure; H2, K2, K4, G2, B3 and D1 failed only the tests named
  for them and five further mutants were killed; the ledger, slice table, plan status, progress logs, dates,
  signatures and the citation check agree.
- Findings:
  - MEDIUM (F1): the plans called the stored sample time and the metric unit unobservable, which is false:
    `Metrics` is `PartialEq`, so a whole-value comparison sees both. Seven mutants survived all 174 tests: `now +
    7 s` in the handler, the listener, the repository and, twice, the metrics wrapper, and `Unit::Seconds` in
    `describe_metrics`.
  - LOW (F2): `run_event_listener` could be given a fresh cancellation token or a zero drain timeout in place of its
    arguments and every test passed (the unspawned entry point already caught both).
  - LOW (F3): a mutant that makes the `Closed` arm of the loop return `false` hangs the suite instead of failing it,
    because the scripted receiver returns `Closed` forever and never yields. Pre-existing pattern.
  - LOW (F4): swallowing the collection's error in `Repository::increase_counter` survived; the plan's "unreachable"
    held only from outside the module.
  - LOW (F5): the concurrency test is deterministic but no mutation can fail it.
  - INFO: removing `biased;` is killed in about 77% of runs; the unspawned entry-point test had an `if` and two
    assertions; the handler label tables repeat their eight-label literals (defensible); the `peers` wording
    question is a likely design question to carry into `lessons.md`; process slips (`sed -i` on three plan status
    lines, and the S5-start log entries shipped with the handler commit).
- Verdict: REVIEW PASSED, with F1 and F2 worth fixing before the pull request.
- Follow-up actions:
  - Handling (2026-10-21, commit `test(http-core): [#1349] address the S5 independent review`): F1 is fixed with
    four whole-value tests (handler, listener, metrics twice, repository) and one comparison for the unit, and
    every survivor is killed; F2 is fixed with a two-case table; F4 is fixed with a test inside the repository
    module; F3 is recorded in the listener plan as a known limit, F5 is kept as a recorded design check and the
    unspawned test lost its branch. The INFO items are noted in the plans or the log, and the `peers` question
    stays open for the retrospective. After the fixes: 183 unit tests, lines 2,974 / 2,996 (99.27%), regions
    3,875 / 3,905 (99.23%), functions 372 / 373 (99.73%).

### 2026-10-21 - Task Reviewer (slice S6 and tasks T5-T8)

- Time: the report returned on 2026-10-21 at about 15:25 UTC, after the evidence commit
  `docs(issues): [#1349] record the final measurements, mutation run, manual verification and lessons`.
- Invocation scope: Slice S6 (ledger rows 11-13: `lib.rs`, `services/mod.rs`, `statistics/event/mod.rs`) and the
  closing tasks T5-T8 of #1349 on branch `1349-1347-add-tests-http-core-final`, `torrust/develop...HEAD` (merge base
  `889a6dc36`, the merge of #2608): five commits (the S6 start, the `lib.rs` tests, the `services/mod.rs` comment,
  the wiring-only conclusions and the final evidence).
- Inputs: `ISSUE.md`, the #1347 subissue guidelines, `coverage-evidence.md`, `mutation-evidence.md`,
  `manual-verification-evidence.md`, `lessons.md`, `implementation-retrospective.md`, the three S6 plans, the plan
  index, the EPIC diff, `project-words.txt`, the raw `cargo-mutants` output, the git-ignored draft hand-off, the
  manual-verification template, `lib.rs`, `services/mod.rs`, and `gh pr view` for #2538, #2546, #2558, #2575, #2586,
  #2593 and #2608.
- Evidence: production code is unchanged except one comment and the appended tests; a clean `cargo llvm-cov` run
  reproduced 191 tests, lines 3,030 / 3,052, regions 3,983 / 4,013 and functions 384 / 385, with the 22 uncovered lines
  adding up as 3 + 3 + 1 + 1 + 14; the raw mutation files hold 50, 1, 1 and 44 mutants and the unviable reasons add up
  as 37 + 3 + 4; three hand-applied mutations (eager recording, the log target, the event order together with the IPv6
  address) each failed only the tests named for them and were reverted exactly; `cargo +nightly fmt --all --
  --check`, `cargo test -p torrust-tracker-http-core` (191 unit tests and the doc tests), `git diff --check` and
  `linter all` passed; all five commits are signed; the PR figures of the retrospective match `gh`; the draft
  hand-off exists and `lessons.md` names it; the manual evidence follows the template with consistent times and
  no invented output.
- Findings:
  - MEDIUM (F1): the EPIC tables and row still carried the S5 interim figures (99.27%, 99.23%, 99.73%), which T7
    and AC9 require to be final.
  - MEDIUM (F2): AC12 says that no test pins a reported behavior, while `metrics.rs` pins the `tcp4`/`tcp6` behavior by
    the maintainer's S5 decision; the Acceptance Verification row must record that decision.
  - LOW (F3): the retrospective said 4 reasons for the unviable mutants (the table has 3) and about 14 minutes for
    the run (it was about 11), and `measured-utc` of the mutation evidence was after the run ended.
  - LOW (F4): V3 and V4 abbreviated the info hash in the recorded commands; two V4 steps carried no time; the
    `updated_milliseconds_ago` value equal to `updated` deserved a note; `disablede` and `peerslee` entered the
    dictionary from joined bencode.
  - LOW (F5): "about 100 tests" for the helpers is 91 call sites; one `lib.rs` test has two reasons to fail; the
    EPIC S6-start entry kept its guessed 12:25 UTC stamp.
  - INFO: the `root_tests` design follows the checklist; the non-address fields of the sample peer are pinned only
    indirectly through 18 service and container tests, which is acceptable.
- Verdict: REVIEW FAILED (F1 blocking), with no change required in the code or the measurements.
- Follow-up actions:
  - Handling (2026-10-21, commit `docs(issues): [#1349] address the S6 independent review`): F1 is fixed (both EPIC
    tables and the row carry the final figures, with a new log entry); F2 is recorded in the Acceptance Verification
    row of AC12; F3 is corrected in the retrospective (3 reasons, about 11 minutes) and `measured-utc` is 15:10:55Z;
    F4 is fixed in the recorded commands, one step now says it was not separately timed, a note explains the
    deprecated REST fields, and the two dictionary words stay because the evidence keeps the verbatim output; F5 is
    corrected to "more than 90 call sites" and the EPIC entry now carries the correction (the two-reason `lib.rs`
    test is kept: the delegation and the literal address are both its subject).
