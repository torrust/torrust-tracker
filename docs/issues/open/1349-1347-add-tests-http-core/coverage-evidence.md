---
doc-type: coverage-evidence
issue: 1349
package: torrust-tracker-http-core
measured-commit: a3810b813
measured-utc: 2026-10-17T08:24:45Z
---

# HTTP Core Coverage Evidence

This document records the clean package-source baseline and complete module inventory for Issue #1349. The reports were collected on `develop` at the merge of #2552, after spec-only PR #2546 merged and before any Issue #1349 test or production change. Raw JSON reports stay ignored local artifacts under `.tmp/1349-coverage/`.

## Measurement Scope

`cargo-llvm-cov 0.6.16` on stable Rust 1.99.0 generated two clean reports, filtered to `packages/http-core/src/`. Totals include the in-file `#[cfg(test)]` modules because unit tests live in the same files, so they are not production-only coverage. The Criterion benches under `packages/http-core/benches/` are not run by these commands.

```text
cargo llvm-cov clean --workspace
cargo llvm-cov -p torrust-tracker-http-core --all-features --json --output-path .tmp/1349-coverage/t1b-aggregate.json
cargo llvm-cov clean --workspace
cargo llvm-cov -p torrust-tracker-http-core --all-features --lib --json --output-path .tmp/1349-coverage/t1b-unit.json
cargo llvm-cov report -p torrust-tracker-http-core --show-missing-lines
```

Both measurement commands completed successfully and ran the same 32 package unit tests. The package has no `tests/` directory and no `[[test]]` target, so there is no integration-only baseline, and the aggregate command selected the same `src/lib.rs` unit-test binary as `--lib`. Identical totals do not make aggregate coverage evidence of an independent test level. The last command lists the uncovered lines of the unit-only run. It can list one or two fewer lines than the summary's missed-line count for a file (for example, 72 listed against 73 missed in `container.rs`); the groups below follow the listed lines and the region check.

Line coverage can hide untested branches, so the uncovered-lines table below also lists *regions only* entries: a region with a zero count (from the unit-only JSON segments) on a line that the missing-lines report counts as covered. A typical case is the early-return path of a `?`.

### Re-measurement After the Spec Baseline

The specification recorded a baseline at `a9d8c5703`. It required a re-measurement if `packages/http-core/` changed on `develop` afterwards. A first commit did: `refactor(http-core): [#2536] hide the statistics read lock and metric collection field` (merged through #2545) changed four statistics files (`get_stats` became `read_metrics`, and the metric collection field became private with a read accessor). This document's numbers replace the specification baseline, and the specification ledger and the EPIC #1347 tables carry them.

| Scope | Spec baseline (`a9d8c5703`) | T1 baseline (`a3810b813`) |
| --- | ---: | ---: |
| Lines | 1,743 / 1,957 (89.06%) | 1,737 / 1,953 (88.94%) |
| Regions | 2,337 / 2,582 (90.51%) | 2,335 / 2,583 (90.40%) |
| Functions | 157 / 186 (84.41%) | 160 / 190 (84.21%) |

A second commit changed the package before this slice merged: `refactor(http-core): [#2244] document the counter-total metric conversion` (merged through #2552) added a private `whole_total` helper to `statistics/metrics.rs`. The package was measured again on `develop` at `a3810b813`, and the numbers in this document are from that run. The raw reports of the earlier run at `04013e7f1` (lines 1,746 / 1,962) are not used.

Only the four files below differ from the spec baseline; every other file has identical counts.

| Source module | Lines before → after | Regions before → after | Functions before → after |
| --- | ---: | ---: | ---: |
| `statistics/event/handler.rs` | 159 / 160 → 158 / 161 | 220 / 223 → 215 / 221 | 15 / 15 → 16 / 17 |
| `statistics/event/listener.rs` | 133 / 148 → 133 / 148 | 191 / 212 → 183 / 204 | 19 / 22 → 19 / 22 |
| `statistics/metrics.rs` | 40 / 49 → 34 / 43 | 35 / 43 → 44 / 52 | 5 / 6 → 7 / 8 |
| `statistics/repository.rs` | 20 / 23 → 21 / 24 | 24 / 27 → 26 / 29 | 5 / 6 → 5 / 6 |

## Test-Level Baseline

### Aggregate/Global Coverage

Aggregate coverage includes the selected default package test target and is broad navigation evidence only. It is not proof that a source seam has adequate unit coverage.

| Lines | Regions | Functions |
| ---: | ---: | ---: |
| 1,737 / 1,953 (88.94%) | 2,335 / 2,583 (90.40%) | 160 / 190 (84.21%) |

### Unit-Only Coverage

Unit-only (`--lib`) coverage is the primary Issue #1349 metric.

| Lines | Regions | Functions |
| ---: | ---: | ---: |
| 1,737 / 1,953 (88.94%) | 2,335 / 2,583 (90.40%) | 160 / 190 (84.21%) |

### Integration-Only Coverage

Not measured. `torrust-tracker-http-core` has no package integration test target at baseline. If a file plan selects one, this document records a separate clean `--test <name>` report. `axum-http-server` or root coverage is not substituted for unit coverage.

## Module Inventory And T1 Hypotheses

Every source file has one T1 hypothesis. It determines processing order but is not a terminal decision: every file still receives its own file test plan, and only that plan can conclude no test change. The property column means that the file plan must assess the named property; it does not require a property-testing dependency. The test count is the number of `#[test]` and `#[tokio::test]` functions in the file.

| Source module | Tests | Unit-only lines | Regions | Functions | Property candidate | Selected behavior or ownership boundary | T1 hypothesis |
| --- | ---: | ---: | ---: | ---: | --- | --- | --- |
| `services/announce.rs` | 8 | 675 / 727 (92.85%) | 878 / 938 | 56 / 63 | Yes: the peer-IP selection decision over `PeerIp` values, both policies, and connection addresses (for example, with the override disabled a client-supplied literal is never selected); the protocol-to-domain announcement mapping | Peer-IP selection and the instance policy, private-mode authentication before authorization and IP resolution, the request-to-announcement mapping (every event variant, absent byte counters), error adaptation to failure reasons, and event publication. Announce rules, whitelist policy and key validity remain `tracker-core`'s; header and `ip` parsing remain `http-protocol`'s. | Property candidate |
| `services/error_mapping.rs` | 0 | 0 / 15 (0.00%) | 0 / 11 | 0 / 1 | No | One match mapping each `TrackerCoreError` variant to a client-visible failure reason. The text is package-owned; the wrapped errors are collaborators'. | Test candidate |
| `services/scrape.rs` | 8 | 507 / 540 (93.89%) | 678 / 711 | 36 / 43 | No | Private-mode zeroing of unauthenticated scrapes (including the authenticated path), reverse-proxy client-IP resolution, error adaptation, and event publication. Scrape aggregation remains `tracker-core`'s. | Test candidate |
| `event.rs` | 7 | 174 / 187 (93.05%) | 277 / 294 | 16 / 20 | No | Connection-context label projection, client IP family and type, and public-URL projection are directly tested. The uncovered getters are unused: three have no caller, and `ClientConnectionContext::port` is called only by the uncalled `client_port`. | Probably small change |
| `container.rs` | 0 | 0 / 73 (0.00%) | 0 / 89 | 0 / 5 | No | Composition of the announce and scrape services with the instance policy, instance ID, event bus, and shared repository. Do not use the production container factories as a test bootstrap. | Test candidate |
| `statistics/event/handler.rs` | 5 | 158 / 161 (98.14%) | 215 / 221 | 16 / 17 | No | Event-to-counter routing and label propagation are directly tested; the remaining lines are failure and debug logging. | Probably no change |
| `statistics/event/listener.rs` | 4 | 133 / 148 (89.86%) | 183 / 204 | 19 / 22 | No | Dispatch loop behavior has direct tests, but only for `TcpScrape` events. The spawned wrapper and the unspawned entry point are lifecycle owned by #1488. | Lifecycle (#1488) |
| `statistics/metrics.rs` | 0 | 34 / 43 (79.07%) | 44 / 52 | 7 / 8 | No | The four TCP request-count projections, which share the private `whole_total` helper, are covered through the handler tests. `set_gauge` has no caller. | Test candidate |
| `statistics/repository.rs` | 0 (3 doc tests) | 21 / 24 (87.50%) | 26 / 29 | 5 / 6 | No | Initialization, counter update and the read accessor are covered; the compile-fail and runnable doc tests protect the private lock and its accessor. `Default` is not exercised. | Test candidate |
| `statistics/mod.rs` | 0 | 8 / 8 (100.00%) | 10 / 10 | 1 / 1 | No | The exported metric name and description composition, covered through other modules. | Probably small change (metric-name contract) |
| `lib.rs` | 0 | 27 / 27 (100.00%) | 24 / 24 | 4 / 4 | No | Clock alias, log target and shared test samples; the samples are test support. | Test support only |
| `services/mod.rs` | 0 | No executable entries | - | - | No | Module declarations. | Wiring only |
| `statistics/event/mod.rs` | 0 | No executable entries | - | - | No | Module declarations. | Wiring only |

The 13 files are every `.rs` file under `packages/http-core/src/` (`git ls-tree`), each once.

## Uncovered Lines By Group

Unit-only uncovered lines from `cargo llvm-cov report --show-missing-lines`, grouped by behavior. Each group is a hypothesis for its file plan, which makes the final decision. Classifications follow the per-file workflow: **3A** a package-owned decision testable at the module's API; **3B** observable only through a package integration boundary; **owner** not testable here, with the owner named; **finding** looks wrong and is handed off; **logging** observability only. Line numbers are at `a3810b813` and shift when tests are added.

| File | Lines | Behavior | Hypothesis |
| --- | --- | --- | --- |
| `services/announce.rs` | 260-262 | `peer_from_request` arms for `Stopped`, `Completed` and `Empty` events (only `Started` is exercised) | 3A |
| `services/announce.rs` | 307-309, 311 | Private-mode authentication: a missing key (`MissingAuthKey`) and a presented key checked by the authentication service. No test runs a private tracker. | 3A for the package-owned decision (the `private` flag, the order before authorization and IP resolution); key validity is `tracker-core`'s |
| `services/announce.rs` | 379-383, 387-391, 395-399, 403-407, 411-415 | The five `From` conversions into `HttpAnnounceError` (peer-IP resolution, tracker-core, announce, whitelist and key errors) | 3A |
| `services/announce.rs` | 419-422, 425-427, 429 | `From<HttpAnnounceError> for HttpProtocolErrorResponse`: the client-visible failure reason of every variant | 3A |
| `services/announce.rs` | 211, 213, 215, 217, 226 (regions only; the lines count as covered) | The early-return error path of each `?` in `handle_announce`: authentication, authorization, client-IP resolution, peer-IP selection, and the tracker-core announce failure. The order of these steps is a package-owned decision and no test makes one of them fail through `handle_announce`. | 3A |
| `services/announce.rs` | 350 | The absent-sender path of `send_event` (every announce test supplies a sender; the scrape tests already cover it) | 3A |
| `services/announce.rs` (test module) | 486-490, 565, 568, 571, 597, 682 | The persistent-completed-stat fixture branch, the `Stopped`, `Completed` and `None` arms of the request-builder fixture, the `mock!` macro, and an `unreachable!` | test support; the file plan reviews the fixture |
| `services/error_mapping.rs` | 4-17, 19 | All four `TrackerCoreError` arms | 3A |
| `services/scrape.rs` | 112, 115, 127 (regions only) | The error paths of the `?` after the tracker-core scrape and after client-IP resolution in `handle_scrape`, and the arm where a key is presented (`Some(key)` at line 127; the no-key path is covered by the existing private-mode test) | 3A |
| `services/scrape.rs` | 128 | An authenticated scrape in private mode (`is_authenticated` with a key) | 3A |
| `services/scrape.rs` | 163-167, 171-175, 179-183, 187-191, 195-199, 203-206, 208 | The five `From` conversions into `HttpScrapeError` and the conversion to the failure response | 3A |
| `services/scrape.rs` (test module) | 317 | The `mock!` macro | test support |
| `event.rs` | 80-82, 85-87, 90-92, 125-127 | `ConnectionContext::client_ip_addr`, `client_port`, `server_socket_addr` and `ClientConnectionContext::port`. No caller exists in the repository for the first three. `port` is called only by `client_port` (line 86), so it is dead only together with it. | finding candidate (dead code): the file plan confirms and records a `lessons.md` row that removes `client_port` and `port` together or neither |
| `event.rs` (test support) | 244 | The `_ => false` arm of the `announce_events_match` helper | test support |
| `container.rs` | 36-42, 45-53, 56-57, 60-65, 67-71, 73, 76-106, 117, 119-120, 127, 129, 131-136 | `initialize` (needs a configured database), `initialize_from_tracker_core`, `initialize_from_services` wiring, and `HttpTrackerCoreServices::initialize_from` | 3A for the wiring of `initialize_from_services`; `initialize` composes `tracker-core` and is owned there |
| `statistics/event/handler.rs` | 20 | The `Err` arm that logs a failed counter increase | logging; reaching it needs a failing metric collection, which is `torrust-metrics` behavior |
| `statistics/event/handler.rs` | 25 | The closure that formats the debug line | logging |
| `statistics/event/listener.rs` | 24 (region only) | The `tokio::spawn` wrapper region | owner: #1488 lifecycle |
| `statistics/event/listener.rs` | 108 (region only) | The `TcpAnnounce` alternative of `event_connection_id`, which the listener calls for every event to look up the metrics policy. All four listener tests dispatch only `TcpScrape`, so the policy lookup for an announce event is unprotected. | 3A candidate for the listener plan |
| `statistics/event/listener.rs` | 25, 27-28 | The spawned `run_event_listener` wrapper | owner: #1488 lifecycle |
| `statistics/event/listener.rs` | 32-40 | `run_event_listener_unspawned`, the entry point the production bootstrap calls | owner: #1488 lifecycle |
| `statistics/event/listener.rs` | 100 | The `warn!` for an event from an unknown or metrics-disabled listener | logging; the branch itself is tested |
| `statistics/metrics.rs` | 40-48 | `Metrics::set_gauge`. No caller exists in the repository. | finding candidate (dead code) |
| `statistics/repository.rs` | 58-60 | `Default::default` | 3A (one assertion) or finding candidate if it has no caller |

## Prioritized Gaps

1. `services/announce.rs`: private-mode authentication, the order in which `handle_announce` fails (authentication, authorization, client-IP resolution, peer-IP selection), the other announce event variants, and the failure reason of every `HttpAnnounceError` variant. These are admission and client-visible decisions, and no unit test protects them.
2. `services/error_mapping.rs` and the matching `services/scrape.rs` conversions: the failure reasons clients see for tracker-core failures. `error_mapping.rs` has no coverage at all.
3. `services/scrape.rs`: the authenticated private-mode scrape and the error adaptations.
4. `container.rs`: no unit coverage for how configuration reaches the services it wires.
5. `statistics/metrics.rs`, `statistics/repository.rs` and `event.rs`: a few uncovered functions that look unused; these follow the tracker-answering behavior because they cannot change a tracker response.

## Test-Technique Assessment

EPIC #1347 requires every package subissue to assess each technique. The initial assessment in the specification is confirmed by this inventory; the property candidates are in the table above.

| Technique | Decision | Rationale |
| --- | --- | --- |
| Unit tests | Selected | Primary objective, in-file `#[cfg(test)]` modules. |
| Package integration tests | Not selected | The package has no network boundary of its own. A `tests/` target needs a file plan that records why a public-API contract is clearer. |
| Runnable examples | Not selected | The existing compile and compile-fail doc tests in `statistics/repository.rs` are reviewed with their file. |
| Root `tests/` or `packages/e2e-tools/` | Not selected | Real HTTP behavior is owned by `axum-http-server` (#2136, #2140) and the root tests. |
| Mutation testing | Selected (T6) | A bounded sample on a changed highest-risk seam, with `RUSTFLAGS="--cap-lints=warn"` and a stated reason for every unviable mutant. |
| Property-based testing | To assess per file | `services/announce.rs` is the candidate. A property is kept only if it catches a mutation the examples miss, and the dependency follows `add-rust-dependency`. |
| Fuzzing | Not selected | Untrusted parsing is owned by `http-protocol`. |
| Benchmarks | Out of scope | The Criterion benches are not tests of correctness. |
