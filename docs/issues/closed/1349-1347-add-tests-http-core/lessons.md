---
doc-type: issue-lessons
issue: 1349
package: torrust-tracker-http-core
---

# HTTP Core Testing Lessons

Candidate lessons collected while working on Issue #1349. Promoting any of them into `docs/testing/`,
`.github/skills/`, or a production change is a separate decision after this issue, per the
[subissue guidelines](../1347-overhaul-packages-testing/subissue-spec-guidelines.md).

## Follow-Up Candidates

| Source | Candidate | Evidence |
| --- | --- | --- |
| [Metrics plan](test-refactor-plans/statistics-metrics-tests.md) | Decide whether the `tcp4_*`/`tcp6_*` counters count requests per client family or per server binding, then make the docs, the REST API documentation and the code agree. Draft (git-ignored, not yet a GitHub issue): `.tmp/1349-tcp-family-counter-docs/ISSUE.md` with `HANDOFF.md`; replace this path with the issue number when it is filed. The same question exists for `udp4_*`/`udp6_*`, recorded by #1348 and never taken up, so the draft covers both. | The getters say "from IPv4/IPv6 peers" but select on `server_binding_address_ip_family`; on a dual-stack `[::]` listener every IPv4 client counts as `tcp6`/`udp6` (#1671 spec). A test pins the server-binding behavior. |
| [Event plan](test-refactor-plans/event-tests.md) | Decide whether to remove the four public accessors of `event.rs` that have no caller in the repository, or keep them as public API. | The maintainer chose to pin them as public API in S4; they are covered, and no hand-off was made. |
| [Container plan](test-refactor-plans/container-tests.md) | Make the swarm registry's usage-statistics flag in `HttpTrackerCoreContainer::initialize` observable, or accept it as untestable. | The only surviving mutation of the file: `initialize` always creates the registry with usage statistics disabled, which no `http-core` unit test can observe. |
| [Crate root plan](test-refactor-plans/lib-tests.md) | Declare the HTTP tracker log target once. | `HTTP_TRACKER_LOG_TARGET` is declared in both `http-core` and `axum-http-server` with the same value; `src/console/ci/e2e/logs_parser.rs` matches the `axum-http-server` copy while the core listener logs under the `http-core` copy. `http-core` cannot depend on the Axum package, so the owner of a shared constant must be chosen. |
| [Listener plan](test-refactor-plans/statistics-listener-tests.md) | Under #1488 (shutdown), decide the listener's closed-channel and drain policy and test it. | A mutant that makes the `Closed` arm of the drain return `false` hangs the suite: the scripted receiver returns `Closed` forever, so no timeout fires. #1488 owns lifecycle policy; the tests assert only the documented completion and event count. |
| [Handler plan](test-refactor-plans/statistics-handler-tests.md) | Cover or remove the failed-counter-increase `Err` arm and its debug closure (lines 20 and 25 of `handler.rs`), and the ignored-event `warn!` argument (line 132 of `listener.rs`). | Decided in the plans as not testable without a failing metrics repository; they are the only uncovered production lines of the package. |
| [Services plan](test-refactor-plans/services-mod-tests.md) | **Done in S6.** Correct the `services` module comment that said the services "does depend on the Axum web server". | `packages/http-core/Cargo.toml` has no Axum dependency. |
| [Event plan](test-refactor-plans/event-tests.md) | **Done in S4.** Remove the dead `announce_events_match` helper and its public `test` module. | No caller after the S2 announce tests; the Criterion benches have their own helpers. |

## Testing Lessons

- **Check the derives before calling something unobservable.** Three S5 plans said that the recorded sample time and
  the metric unit could not be observed. `Metrics` is `PartialEq` and `Clone`, so a whole-value comparison sees both;
  seven mutants had survived on that claim. Before recording a property as untestable, look for the type's derives
  and a public way to compare whole values.
- **Assert whole values against literals.** Counter totals, label sets and Prometheus text written as literals make a
  mutant that changes any part fail; assertions on a helper's own output cannot.
- **Table-test the policy, not one setting.** Copilot found that the S4 delivery test never disabled usage statistics,
  so it could not prove that publication is independent of that policy. Four tests became `rstest` tables over the
  policy.
- **A test double can need its own tests.** `RecordingSender` documents that it records only when the `send` future is
  polled. No service test dropped a future, so a sender recording at call time passed all 183 tests; only the new
  helper test killed it. Test the helpers that more than 90 call sites rely on, with the mutant that breaks their documented
  contract.
- **A coarse mutant proves little.** The first sender mutant recorded eagerly and again on poll, and failed 27 tests
  by doubling the events; it proved nothing about dropped futures. Make each mutant change one property.
- **`matches!` hides an uncovered region.** Its failure arm counts as one uncovered region of the test module;
  `assert_eq!` on a whole value (`result.and_then(Result::ok)`) does not.
- **Coverage counts do not move monotonically.** `llvm-cov` does not count the body lines of a test that is both an
  `rstest` table and `#[tokio::test]`, so converting tests to tables lowered the counted totals by 80 lines in S4
  while no production line lost coverage. Judge a slice by the production lines of the file, not by the total.
- **Under `unused = "deny"`, write mutants that compile.** Use `std::hint::black_box`, `drop(...)`,
  `.filter(|_| false)` or `(cond || true)`; revert each exactly and confirm that the diff against the base has no
  removed line.
- **An independent review per slice paid for itself.** Every slice review found survivors the author's own mutations
  missed (label tables in S4; the unit and time, the spawned listener's wiring and an error-swallowing path in S5).

## Process Lessons

- **Slicing meant `develop` moved under the work.** The S3 merge brought #2414 into `statistics/event/listener.rs`
  after the S1 plan was written, and the S4-start entry wrongly said `http-core` was unchanged. Re-measure the
  baseline and diff the package at the start of every slice; this is now recorded in each slice-start entry.
- **Do not run `git add` while another edit is in flight.** A parallel edit raced with staging and the S5-start
  commit lost its log entries; they shipped with the next commit and the records say so.
- **Read the clock, do not guess it.** The S6-start entry carried a guessed time (12:25 UTC for a 13:21 commit) and
  needed a correction entry; stamp every entry from `date -u`.
- **Edit mistakes were concentrated in one pattern.** An `old_str` longer than its replacement deletes the following
  text (it happened repeatedly in S4-S6 and each time was repaired and disclosed); keep replacements at least as
  long, and re-read the lines around the edit.
- **Keep using the edit tool for files.** Two `sed -i` slips and one script edit of the slice table were disclosed;
  none was harmful, but each broke the rule.
- **The manual run found one client-usage trap.** `tracker_client http scrape` takes the tracker's base URL, not the
  announce URL; the wrong URL reaches the announce parser and fails with a misleading "missing param peer_id".
