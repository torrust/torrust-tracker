---
doc-type: file-test-plan
issue: 1349
package: torrust-tracker-http-core
target-file: packages/http-core/src/container.rs
status: done
semantic-links:
  related-artifacts:
    - packages/http-core/src/container.rs
    - packages/http-core/src/services/announce.rs
    - packages/http-core/src/services/scrape.rs
    - docs/issues/open/1349-1347-add-tests-http-core/coverage-evidence.md
---

# HTTP Core Container File Test Plan

Follow the shared [guidance](README.md). This plan covers only `packages/http-core/src/container.rs`. It reuses
the approach of the [announce](services-announce-tests.md), [scrape](services-scrape-tests.md) and
[event](event-tests.md) plans.

## Current State

- **Unit-only coverage:** 0 / 73 lines, 0 / 89 regions, 0 / 5 functions, measured at the S4 baseline
  (`develop` at the merge of #2586; the file has not changed since S1). The file has no test module. The
  composition is exercised only by other packages (`axum-http-server`, the application container), which the
  unit-only measure does not count.
- **Existing tests:** none.
- **Module-owned decisions:** the composition of one HTTP tracker instance. `HttpTrackerCoreServices::initialize_from`
  creates the statistics event bus (publication always enabled, issue #2039), the sender that the services
  publish to, and an empty statistics repository. `HttpTrackerCoreContainer::initialize_from_services` wires the
  announce and scrape services from the tracker-core collaborators, the per-instance `HttpTracker` policy (reverse
  proxy mode, peer-IP override, public URL), the instance id, and the shared sender. `initialize_from_tracker_core`
  adds the services; `initialize` adds the swarm registry and tracker-core composition. What the collaborators
  do is `tracker-core`'s and the services' (tested in S2 and S3); this file owns only which collaborator goes
  where.
- **T1 hypothesis:** test candidate.

## Current Tests Review

None to review. The package has no test for any function in the file (`HttpTrackerCoreContainer::initialize`,
`initialize_from_tracker_core`, `initialize_from_services` and `HttpTrackerCoreServices::initialize_from`).

## Coverage and Decision Analysis

| Behavior group | Classification | Decision |
| --- | --- | --- |
| `HttpTrackerCoreServices::initialize_from`: an enabled event bus whose `stats_event_sender` delivers to a receiver of the same bus; an empty statistics repository; two calls give independent services | 3A unit | C1. Pins the always-enabled publication decision of #2039 (a bus built `Disabled` would publish nothing and every HTTP metric would stay at zero) and that the sender and the bus are the same channel. |
| `initialize_from_services`: the container publishes the exact collaborators it was given (`tracker_core_container`, `http_tracker_config`, `event_bus`, `stats_event_sender`, `stats_repository`) | 3A unit | C2. Identity checks with `Arc::ptr_eq`. Only the wiring, not the collaborators' behavior. |
| `initialize_from_services`: the announce and scrape services publish to the services' sender, with the instance id and the public URL of the passed `HttpTracker` | 3A unit | C3. A hand-built `HttpTrackerCoreServices` holding a `RecordingSender`; the event's connection context carries the passed instance id and public URL, for both services. A service wired to another sender, `None`, another id or a default config fails. |
| `initialize_from_services`: the per-instance policy reaches the services (`network.on_reverse_proxy` and `use_ip_from_query_string`) | 3A unit | C4. Two tables, one per service where it applies: with `on_reverse_proxy` the event's client address is the `X-Forwarded-For` one, without it a missing proxy header is the service error; `use_ip_from_query_string` is honored by the announce service. The decisions themselves are tested in S2 and S3; this only proves the config reaches them. |
| `initialize_from_services`: the services use the tracker-core collaborators of the passed container (core config, handlers, authentication, whitelist) | 3A unit, collaboration | C5. A private-mode core config makes both services demand a key; an announce stores the peer in the passed container's torrent repository; a scrape reads from it. Tracker-core semantics stay tracker-core's: each case asserts only that the container's own collaborator was the one used. |
| `initialize_from_tracker_core`: delegates to `HttpTrackerCoreServices::initialize_from` and `initialize_from_services` | 3A unit | C6. An announce through the container's `announce_service` is received on the container's own `event_bus`, so the services built inside the method are the ones the container exposes. |
| `initialize`: the swarm registry and the tracker-core container composed from the core config, with the persistence-required bridge | 3B integration, decision needed | See "Decisions needed" (D2). It writes a SQLite file; the composition itself is `tracker-core`'s. |
| The unused `_tracker_core_container` parameter of `initialize_from` | not tested | A parameter with no behavior; it is part of the public signature that the application and `axum-http-server` call. |
| The `Default` fallback to the default SQLite path when `core.database` is `None` (`unwrap_or_default`) | not tested | The default path (`./storage/tracker/lib/database/sqlite3.db`) is relative to the working directory, so a test would create a database under the package directory. Owner of the policy: the persistence-optional composition ADR and the application bootstrap. Recorded in Results. |

No property candidate: the file composes values, with no input space to generate.

## Test Bootstrap

A tiny in-file builder constructs a `TrackerCoreContainer` by hand: it has only public fields, and each
collaborator is a one-line in-memory constructor (whitelist, authentication, torrent repository, torrents manager,
statistics repository, announce and scrape handler, `persistence: None`). This follows the guardrail that a test
bootstrap constructs only what the test needs and does not call a production container factory, and it is the
reason the file does not call `TrackerCoreContainer::initialize_from`. `HttpTrackerCoreServices` is built by
struct literal with a `RecordingSender` from `crate::tests` when a test needs to read the events, and with
`HttpTrackerCoreServices::initialize_from` only in the tests of that function. The `HttpTracker` configs and the
core config come from `torrust_tracker_test_helpers::configuration`, as in the S2 and S3 fixtures.

## Decisions needed

- **D1 (hand-built tracker-core container):** build the `TrackerCoreContainer` by hand (recommended, honors the
  guardrail, about 30 lines in the test module) or call `TrackerCoreContainer::initialize_from(.., None)`
  (shorter, deterministic and in-memory, but a production factory).
- **D2 (`initialize`):** add one async test that calls `initialize` with `configuration::ephemeral()` (a real
  SQLite file in the temp directory, as the application and `axum-http-server` tests do) and asserts the
  composed container is wired to a working tracker core, plus a `#[should_panic]` test that a database path in
  a missing directory panics with the documented message (recommended: without it the file keeps its
  composition entry point and its `# Panics` contract at 0%), or leave `initialize` untested and record it as
  `tracker-core` composition.

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | N/A | The file has no tests to refactor. | Recorded in the plan. |
| R2 | DONE | Add C1-C6, and D2 if approved. | Focused tests, `cargo +nightly fmt --all -- --check`, `git diff --check`, prose-first AAA review; the named mutations M1-M7 each make a test fail. All seven did, plus eight survivor probes (one survivor, recorded in Results). Pending the maintainer's completed-file review. |
| R3 | DONE | Select an integration increment, or `none`. | The `initialize` test is the increment: it opens a real SQLite file in the temp directory and runs the migrations. |
| R4 | DONE | Re-run unit-only coverage; record before and after. | Results and `coverage-evidence.md`. |

## Named Mutations

Each is a temporary hand-made change that its test must catch; it is reverted after the run and recorded in
Results. The workspace denies `unused` and `warnings`: use a form that compiles, and count a mutation only when
its test runs and fails.

| ID | Test | Site | Temporary change |
| --- | --- | --- | --- |
| M1 | C1 | `HttpTrackerCoreServices::initialize_from` | Build the bus with `SenderStatus::Disabled`. |
| M2 | C1 | the same function | Return the sender of a second, unrelated bus. |
| M3 | C3 | `initialize_from_services`, the announce service | Pass `None` instead of the services' sender. |
| M4 | C3 | the scrape service | Pass another configuration instance id. |
| M5 | C4 | `initialize_from_services` | Pass `&HttpTracker::default()` to the announce service instead of the instance config. |
| M6 | C5 | `initialize_from_services` | Pass `Arc::new(Core::default())` instead of `tracker_core_container.core_config` to the scrape service. |
| M7 | C6 | `initialize_from_tracker_core` | Create a second `HttpTrackerCoreServices` for the container's `event_bus` field. |

Gates: this plan needs approval first (including D1 and D2); the completed file needs the maintainer's
completed-file review before its one signed commit.

## Prose-First AAA Review

27 tests (17 cases in 13 functions for `initialize_from_services`) in four nested modules, one per entry point;
20 tests when the file was first completed, 23 after the independent review split three of them, and 27 after the
pull-request review round below parameterized four over the usage-statistics policy.
Each builds the tracker-core collaborators by hand (`TrackerCore`) and, except the `initialize` tests, a
`Fixture` whose services publish to a `RecordingSender`. Every test reads as one arrangement, one call
through the container, and whole-value assertions. No test calls a tracker-core production factory; the tests
of `initialize` call the function under test.

| Test | Behavior it states | Cases | Expected value is |
| --- | --- | --- | --- |
| `the_http_tracker_core_services::it_should_deliver_the_events_of_the_services_sender_to_the_receivers_of_the_services_event_bus` | The services' sender and bus are one channel, and publication is enabled whatever the usage-statistics policy (#2039). | 2 (usage statistics enabled, disabled) | The event that was sent, read with `try_recv`. |
| `...::it_should_not_share_the_event_bus_between_two_sets_of_services` and `...::it_should_not_share_the_statistics_repository_between_two_sets_of_services` | Each call creates its own bus and its own repository. | 1 each | `TryRecvError::Empty` on the other bus; distinct repositories. |
| `...::it_should_start_with_a_fresh_statistics_repository` | The repository holds no samples. | 1 | The metrics of a new repository. |
| `the_container_composed_from_services::it_should_expose_the_collaborators_it_was_composed_from` | The container publishes the exact collaborators it was given. | 1 | `Arc::ptr_eq` on five fields. |
| `...::it_should_publish_the_announce_event_of_its_instance_with_the_public_url_to_the_services_sender` and `...scrape_event...` | Instance id, public URL and sender reach both services, whatever the usage-statistics policy. | 2 each (usage statistics enabled, disabled) | A whole event with instance id 7 and a configured public URL. |
| `...::it_should_resolve_the_client_of_an_announce_by_the_reverse_proxy_policy_of_the_http_tracker` and `...of_a_scrape...` | `network.on_reverse_proxy` reaches both services. | 2 each | A whole event whose client is the forwarded or the connection address. |
| `...::it_should_honor_an_ip_in_the_query_string_when_the_http_tracker_is_configured_to_use_it` and `...::it_should_reject_an_ip_in_the_query_string_when_...not_configured_to_use_it` | `use_ip_from_query_string` reaches the announce service. | 1 each | A whole event with the query-string peer address, or `PeerIpOverrideDisabled` and no event. |
| `...::it_should_reject_an_announce_without_a_key_when_the_tracker_core_is_private` and `...::it_should_zero_a_scrape_without_a_key_when_the_tracker_core_is_private` | The container's core config makes both services private. | 1 each | An authentication error with no peer stored, or zeroed swarm metadata for a swarm that has a seeder. |
| `...::it_should_authenticate_both_services_with_the_keys_of_the_tracker_core` | Both services use the container's authentication service. | 1 | An accepted announce and one seeder in the scrape. |
| `...::it_should_reject_an_announce_of_a_torrent_missing_from_the_whitelist_of_the_tracker_core` and `...::it_should_accept_an_announce_of_a_torrent_added_to_the_whitelist_of_the_tracker_core` | The announce service uses the container's whitelist. | 1 each | A whitelist error for an unlisted torrent; success after the torrent is added to the container's whitelist. |
| `...::it_should_scrape_the_swarm_that_the_announce_service_registered_in_the_tracker_core` | Both services share the container's torrent repository. | 1 | One seeder; the torrent is in the repository. |
| `the_container_composed_from_tracker_core::it_should_publish_the_events_of_both_services_to_its_own_event_bus` | `initialize_from_tracker_core` exposes the bus its services publish to, whatever the usage-statistics policy. | 2 (usage statistics enabled, disabled) | The announce event, the scrape event, then an empty receiver. |
| `the_container_initialized_from_the_core_configuration::it_should_compose_the_tracker_core_over_the_configured_database_with_the_given_core_configuration` | `initialize` composes persistence and passes the core configuration on. | 1 | Persistence present; the same core config `Arc`. |
| `...::it_should_publish_the_events_of_its_instance_with_the_public_url_to_its_own_event_bus` | `initialize` forwards the HTTP config and the instance id to the services. | 1 | A whole announce event with instance id 7 and the public URL, read from the container's bus. |
| `...::it_should_panic_when_the_configured_database_cannot_be_initialized` | The documented `# Panics` contract. | 1 | The message of the `expect`. |

Reading the review against the code: the `rstest` tables vary only the configuration; the expected events are
written out as values, not derived from the production conversion; none mocks or sleeps. One disclosed
duplication: `announce_request()` repeats the announce test helper of `services/announce.rs`, which is private
to that module's test code; it was not moved to `crate::tests` to leave the reviewed S2 file untouched.

## Results

Unit-only coverage (`cargo llvm-cov -p torrust-tracker-http-core --all-features --lib`), measured on
2026-10-20 at 21:02 UTC. The baseline is the S4 baseline re-measured at `fefefdd9e` (see
`coverage-evidence.md`).

| Scope | Lines before | Lines after | Regions before | Regions after | Functions before | Functions after |
| --- | --- | --- | --- | --- | --- | --- |
| `container.rs` | 0 / 73 (0.00%) | 582 / 584 (99.66%) | 0 / 89 | 774 / 777 | 0 / 5 | 60 / 60 |
| Package `http-core` | 2,018 / 2,145 (94.08%) | 2,600 / 2,656 (97.89%) | 2,589 / 2,735 (94.66%) | 3,363 / 3,423 (98.25%) | 251 / 262 (95.80%) | 311 / 317 (98.11%) |

The package "before" column is the state after `event.rs` (the previous commit of this slice); the S4 baseline
is in the Current State above. The counted lines include the 511 counted lines of the new test module (810
lines in the file). All 73 production lines are covered: the two uncovered lines (717 and 780) are the failure
branches of two `matches!` assertions in the test module. The package has 135 unit tests (115 after `event.rs`,
95 at the S4 baseline). Production code is byte-identical to `HEAD`.

Named mutations (each reverted; the production diff was checked empty afterwards):

| ID | Result | Tests that failed |
| --- | --- | --- |
| M1 | killed | 3 (the bus delivery, the independence test and the container bus test) |
| M2 | killed | 2 (the bus delivery and the container bus test) |
| M3 | killed | 5 (four announce tests and the container bus test) |
| M4 | killed | 4 (three scrape tests and the container bus test) |
| M5 | killed | 3 announce tests |
| M6 | killed | the private scrape test |
| M7 | killed | the container bus test |

Survivor probes beyond the plan:

| ID | Temporary change | Result |
| --- | --- | --- |
| P1 | `initialize` forwards the HTTP config without its public URL | killed by the `initialize` test |
| P2 | `initialize` forwards another instance id | killed by the `initialize` test |
| P3 | `initialize` passes no database to tracker-core | killed (the `initialize` test and the panic test) |
| P4 | the scrape service authenticates against another key store | killed by the key test |
| P5 | the announce service authorizes against another whitelist | killed by the whitelist test |
| P6 | the container holds a new statistics repository | killed by the identity test |
| P8 | the scrape service gets no sender | killed (4 tests) |
| P9 | the announce service authenticates against another key store | killed (2 tests) |
| P7 | `initialize` always creates the swarm registry with usage statistics disabled | **survived** |

P7 is not reachable from this package: the flag only chooses whether the swarm registry publishes statistics
events, and `HttpTrackerCoreContainer` does not expose the registry or its sender. A test would need the
`tracker-core` statistics listener that the application bootstrap starts. Owner: the swarm-coordination
registry and the application container. Recorded here and not handed off as a finding, because the behavior
is not known to be wrong. Three mutation forms did not compile under the workspace lints (an unused variable
in two `initialize` probes and an unused return value in M7) and were redone in a compiling form before they
counted; one revert did not apply (an ambiguous edit) and the production-code diff caught it before the next
result was trusted.

Not tested, by decision: the `unwrap_or_default()` fallback to `./storage/tracker/lib/database/sqlite3.db` in
`initialize` (a relative path would create a database under the working directory), and the unused
`_tracker_core_container` parameter of `HttpTrackerCoreServices::initialize_from`.

### Independent Review Round

The independent Task Reviewer (see `agent-review-reports.md`) confirmed every figure above and the P7
survivor, and raised three test-design findings in this file, all fixed:

- F2 (logic in a test): the announce reverse-proxy table no longer computes its expected peer address with a
  `match`; it is a case parameter.
- F3 (several reasons to fail, split Act): the bus-and-repository independence test is two tests, the
  whitelist test is two (an unlisted torrent is rejected; a torrent added to the container's whitelist is
  accepted, with the add in Arrange), and the `initialize` test is two (composition; forwarding of the HTTP
  config and instance id). The fresh-repository test stays as it is: it compares with a new repository because
  empty metrics have no literal.
- F4 (fixed missing directory): the panic test now takes the missing parent directory from
  `configuration::ephemeral_sqlite_database()`, a random name, instead of a fixed one.

The splits were checked against the mutations that depend on them: P5 (the whitelist) is killed by the accept
test and P1 (the HTTP config forwarded without its public URL) by the forwarding test. After the round the file
has 23 tests and the package 140.

Final measurement after the round (same commands, 2026-10-21 at about 07:20 UTC, with the production code still
byte-identical to `fefefdd9e`):

| Scope | Lines | Regions | Functions |
| --- | --- | --- | --- |
| `container.rs` | 598 / 600 (99.67%) | 808 / 811 (99.63%) | 65 / 65 (100.00%) |
| Package `http-core` | 2,616 / 2,672 (97.90%) | 3,397 / 3,457 (98.26%) | 316 / 322 (98.14%) |

The counted lines include the 527 counted lines of the test module (834 lines in the file). All 73 production
lines are still covered; the uncovered regions are three `matches!` failure branches in the test module (their
line numbers moved when the tests were split, so lines 717 and 780 above are no longer current).

### Pull-Request Review Round

Copilot reviewed the pull request head and raised one finding (PR #2593, review 5596527098, recorded in the
audit `docs/pr-reviews/pr-2593-review/PR-REVIEW.md`): the delivery test ran with `Core::default()`, whose
`tracker_usage_statistics` is `true`, so it did not distinguish unconditional publication (the #2039 contract) from
publication gated by that flag. This was a real gap and a survivor the plan had not hunted. It is fixed by
turning four tests into tables over the usage-statistics policy (the tracker core's and the first HTTP tracker's, both
set to the same value): the services' delivery test, the announce and scrape publication tests of the container
composed from services, and the bus test of the container composed from tracker core. Each now has an enabled
and a disabled case, so the file has 27 tests and the package 144.

New mutations, each reverted:

| ID | Temporary change | Result |
| --- | --- | --- |
| Q1 | `HttpTrackerCoreServices::initialize_from` creates the bus with the core's `tracker_usage_statistics` as its sender status | killed by the two disabled cases of the delivery test and the bus test; the enabled cases pass, so the earlier test let it survive |
| Q2 | the announce service gets the sender only when the HTTP tracker's `tracker_usage_statistics` is true | killed by the disabled cases of the announce publication test and the bus test |
| Q3 | the same for the scrape service | killed by the disabled cases of the scrape publication test and the bus test |

Final measurement after this round (same commands, 2026-10-21 at about 07:50 UTC, production code still
byte-identical to `fefefdd9e`):

| Scope | Lines | Regions | Functions |
| --- | --- | --- | --- |
| `container.rs` | 518 / 520 (99.62%) | 675 / 678 (99.56%) | 58 / 58 (100.00%) |
| Package `http-core` | 2,536 / 2,592 (97.84%) | 3,264 / 3,324 (98.19%) | 309 / 315 (98.10%) |

The counted totals fell although tests were added: llvm-cov does not count the body lines of a test that is both an
`rstest` table and a `#[tokio::test]` (the older reverse-proxy tables were never counted either, while plain
`#[tokio::test]` bodies are), so converting four tests removed about 80 counted test-module lines. No production line
lost coverage: all 73 are still covered, and the four tests still run in two cases each.

## Approval

Approved by Jose Celano on 2026-10-20 (20:49 UTC): C1-C6, R3 as the `initialize` integration test, mutations M1-M7, D1 (hand-built tracker-core container) and D2 (test `initialize`, including the `#[should_panic]` test).

## Progress Log

- 2026-10-20 20:45 UTC - GitHub Copilot - Created the plan from `container.rs`, the S4 baseline and the S2, S3
  and S4 plans. No Rust test or production code changed. The plan awaits maintainer approval, with two
  decisions: D1 (hand-built tracker-core container) and D2 (the `initialize` entry point).
- 2026-10-20 20:49 UTC - Jose Celano - Approved the plan with D1 hand-built and D2 test `initialize`. R2 starts.
- 2026-10-20 21:02 UTC - GitHub Copilot - R2-R4 done: 20 tests, mutations M1-M7 and eight extra probes killed,
  one survivor (P7, the swarm registry's usage-statistics flag, not observable from this package), `container.rs`
  at 582 / 584 lines with all 73 production lines covered. The AAA review and the results are recorded above.
- 2026-10-21 07:00 UTC - Jose Celano - Approved the completed file (the reply was recorded at this time, after
  a pause in the session). The plan is `done`.
- 2026-10-21 07:25 UTC - GitHub Copilot - The independent review found three test-design issues in this file
  (a `match` in a test, tests with several reasons to fail, and a fixed missing directory) and no
  production or coverage problem. All are fixed in the review-round commit; the file has 23 tests and 598 / 600
  lines, and the Independent Review Round section above records the final figures.
- 2026-10-21 07:55 UTC - GitHub Copilot - Copilot's review of the pull request found that the delivery test
  did not exercise publication with usage statistics disabled. Four tests are now parameterized over the
  policy, mutations Q1-Q3 are killed only by the disabled cases, and the file has 27 tests; the Pull-Request
  Review Round section above records the figures.
