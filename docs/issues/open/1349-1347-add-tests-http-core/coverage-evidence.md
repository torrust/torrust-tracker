---
doc-type: coverage-evidence
issue: 1349
package: torrust-tracker-http-core
measured-commit: 87690b15e
measured-utc: 2026-10-20T12:20:54Z
---

# HTTP Core Coverage Evidence

This document records the clean package-source baseline and complete module inventory for Issue #1349. The reports were collected on `develop` at the merge of #2558 (`87690b15e`, slice S1), before any Issue #1349 test or production change. Earlier reports at `a3810b813` are superseded by the [S2 re-measurement](#re-measurement-before-slice-s2). Raw JSON reports stay ignored local artifacts under `.tmp/1349-coverage/`.

## Measurement Scope

`cargo-llvm-cov 0.6.16` on stable Rust 1.99.0 generated two clean reports, filtered to `packages/http-core/src/`. Totals include the in-file `#[cfg(test)]` modules because unit tests live in the same files, so they are not production-only coverage. The Criterion benches under `packages/http-core/benches/` are not run by these commands.

```text
cargo llvm-cov clean --workspace
cargo llvm-cov -p torrust-tracker-http-core --all-features --json --output-path .tmp/1349-coverage/t1b-aggregate.json
cargo llvm-cov clean --workspace
cargo llvm-cov -p torrust-tracker-http-core --all-features --lib --json --output-path .tmp/1349-coverage/t1b-unit.json
cargo llvm-cov report -p torrust-tracker-http-core --show-missing-lines
```

Both measurement commands completed successfully and ran the same 33 package unit tests. The package has no `tests/` directory and no `[[test]]` target, so there is no integration-only baseline, and the aggregate command selected the same `src/lib.rs` unit-test binary as `--lib`. Identical totals do not make aggregate coverage evidence of an independent test level. The last command lists the uncovered lines of the unit-only run. It can list one or two fewer lines than the summary's missed-line count for a file (for example, 72 listed against 73 missed in `container.rs`); the groups below follow the listed lines and the region check.

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

### Re-measurement Before Slice S2

Three more commits changed `packages/http-core/` on `develop` after `a3810b813`, so the package was measured again at `87690b15e` before any S2 change (same commands, stable Rust 1.99.0, `cargo-llvm-cov 0.6.16`; both reports ran the same 33 unit tests):

- `feat(http-core)!: [#2454] mark public error enums non_exhaustive` (merged through #2568) added `#[non_exhaustive]` to `HttpAnnounceError` and `HttpScrapeError`, and a wildcard arm to the `match` in `services/error_mapping.rs`, which handles a `TrackerCoreError` variant that the package does not name yet. No test can reach this arm today: `TrackerCoreError` has exactly four variants and, being `#[non_exhaustive]`, a crate other than `tracker-core` cannot construct a fifth one. Every line after the attribute in `services/announce.rs` and `services/scrape.rs` moved down by one.
- `refactor(http-core): [#2560] adopt the exact counter sum from torrust-metrics 0.2.0` (merged through #2561) replaced the private `whole_total` helper with `counter_total` and added the first `statistics/metrics.rs` test, a regression test for a counter total that an `f64` cannot represent. The package now has 33 unit tests.
- `chore(deps): [#2560] bump torrust-metrics to 0.2.0` is part of the same change.

| Scope | T1 baseline (`a3810b813`) | S2 baseline (`87690b15e`) |
| --- | ---: | ---: |
| Lines | 1,737 / 1,953 (88.94%) | 1,751 / 1,970 (88.88%) |
| Regions | 2,335 / 2,583 (90.40%) | 2,357 / 2,607 (90.41%) |
| Functions | 160 / 190 (84.21%) | 161 / 191 (84.29%) |

Only the two files below differ in counts; the other files differ only in line numbers (`announce.rs` and `scrape.rs`) or not at all.

| Source module | Lines before → after | Regions before → after | Functions before → after |
| --- | ---: | ---: | ---: |
| `services/error_mapping.rs` | 0 / 15 → 0 / 18 | 0 / 11 → 0 / 13 | 0 / 1 → 0 / 1 |
| `statistics/metrics.rs` | 34 / 43 → 48 / 57 | 44 / 52 → 66 / 74 | 7 / 8 → 8 / 9 |

The numbers in the sections below are from the S2 baseline.

## Test-Level Baseline

### Aggregate/Global Coverage

Aggregate coverage includes the selected default package test target and is broad navigation evidence only. It is not proof that a source seam has adequate unit coverage.

| Lines | Regions | Functions |
| ---: | ---: | ---: |
| 1,751 / 1,970 (88.88%) | 2,357 / 2,607 (90.41%) | 161 / 191 (84.29%) |

### Unit-Only Coverage

Unit-only (`--lib`) coverage is the primary Issue #1349 metric.

| Lines | Regions | Functions |
| ---: | ---: | ---: |
| 1,751 / 1,970 (88.88%) | 2,357 / 2,607 (90.41%) | 161 / 191 (84.29%) |

### Integration-Only Coverage

Not measured. `torrust-tracker-http-core` has no package integration test target at baseline. If a file plan selects one, this document records a separate clean `--test <name>` report. `axum-http-server` or root coverage is not substituted for unit coverage.

## Module Inventory And T1 Hypotheses

Every source file has one T1 hypothesis. It determines processing order but is not a terminal decision: every file still receives its own file test plan, and only that plan can conclude no test change. The property column means that the file plan must assess the named property; it does not require a property-testing dependency. The test count is the number of `#[test]` and `#[tokio::test]` functions in the file.

| Source module | Tests | Unit-only lines | Regions | Functions | Property candidate | Selected behavior or ownership boundary | T1 hypothesis |
| --- | ---: | ---: | ---: | ---: | --- | --- | --- |
| `services/announce.rs` | 8 | 675 / 727 (92.85%) | 878 / 938 | 56 / 63 | Yes: the peer-IP selection decision over `PeerIp` values, both policies, and connection addresses (for example, with the override disabled a client-supplied literal is never selected); the protocol-to-domain announcement mapping | Peer-IP selection and the instance policy, private-mode authentication before authorization and IP resolution, the request-to-announcement mapping (every event variant, absent byte counters), error adaptation to failure reasons, and event publication. Announce rules, whitelist policy and key validity remain `tracker-core`'s; header and `ip` parsing remain `http-protocol`'s. | Property candidate |
| `services/error_mapping.rs` | 0 | 0 / 18 (0.00%) | 0 / 13 | 0 / 1 | No | One match mapping each named `TrackerCoreError` variant, plus a wildcard arm for a variant the package does not name yet (the enum is `#[non_exhaustive]`), to a client-visible failure reason. The text is package-owned; the wrapped errors are collaborators'. | Test candidate |
| `services/scrape.rs` | 8 | 507 / 540 (93.89%) | 678 / 711 | 36 / 43 | No | Private-mode zeroing of unauthenticated scrapes (including the authenticated path), reverse-proxy client-IP resolution, error adaptation, and event publication. Scrape aggregation remains `tracker-core`'s. | Test candidate |
| `event.rs` | 7 | 174 / 187 (93.05%) | 277 / 294 | 16 / 20 | No | Connection-context label projection, client IP family and type, and public-URL projection are directly tested. The uncovered getters are unused: three have no caller, and `ClientConnectionContext::port` is called only by the uncalled `client_port`. | Probably small change |
| `container.rs` | 0 | 0 / 73 (0.00%) | 0 / 89 | 0 / 5 | No | Composition of the announce and scrape services with the instance policy, instance ID, event bus, and shared repository. Do not use the production container factories as a test bootstrap. | Test candidate |
| `statistics/event/handler.rs` | 5 | 158 / 161 (98.14%) | 215 / 221 | 16 / 17 | No | Event-to-counter routing and label propagation are directly tested; the remaining lines are failure and debug logging. | Probably no change |
| `statistics/event/listener.rs` | 4 | 133 / 148 (89.86%) | 183 / 204 | 19 / 22 | No | Dispatch loop behavior has direct tests, but only for `TcpScrape` events. The spawned wrapper and the unspawned entry point are lifecycle owned by #1488. | Lifecycle (#1488) |
| `statistics/metrics.rs` | 1 | 48 / 57 (84.21%) | 66 / 74 | 8 / 9 | No | The four TCP request-count projections, which share the private `counter_total` helper, are covered through the handler tests, and one test pins a total beyond `f64` precision. `set_gauge` has no caller. | Test candidate |
| `statistics/repository.rs` | 0 (3 doc tests) | 21 / 24 (87.50%) | 26 / 29 | 5 / 6 | No | Initialization, counter update and the read accessor are covered; the compile-fail and runnable doc tests protect the private lock and its accessor. `Default` is not exercised. | Test candidate |
| `statistics/mod.rs` | 0 | 8 / 8 (100.00%) | 10 / 10 | 1 / 1 | No | The exported metric name and description composition, covered through other modules. | Probably small change (metric-name contract) |
| `lib.rs` | 0 | 27 / 27 (100.00%) | 24 / 24 | 4 / 4 | No | Clock alias, log target and shared test samples; the samples are test support. | Test support only |
| `services/mod.rs` | 0 | No executable entries | - | - | No | Module declarations. | Wiring only |
| `statistics/event/mod.rs` | 0 | No executable entries | - | - | No | Module declarations. | Wiring only |

The 13 files are every `.rs` file under `packages/http-core/src/` (`git ls-tree`), each once.

## Uncovered Lines By Group

Unit-only uncovered lines from `cargo llvm-cov report --show-missing-lines`, grouped by behavior. Each group is a hypothesis for its file plan, which makes the final decision. Classifications follow the per-file workflow: **3A** a package-owned decision testable at the module's API; **3B** observable only through a package integration boundary; **owner** not testable here, with the owner named; **finding** looks wrong and is handed off; **logging** observability only. Line numbers are at `87690b15e` and shift when tests are added.

| File | Lines | Behavior | Hypothesis |
| --- | --- | --- | --- |
| `services/announce.rs` | 260-262 | `peer_from_request` arms for `Stopped`, `Completed` and `Empty` events (only `Started` is exercised) | 3A |
| `services/announce.rs` | 307-309, 311 | Private-mode authentication: a missing key (`MissingAuthKey`) and a presented key checked by the authentication service. No test runs a private tracker. | 3A for the package-owned decision (the `private` flag, the order before authorization and IP resolution); key validity is `tracker-core`'s |
| `services/announce.rs` | 380-384, 388-392, 396-400, 404-408, 412-416 | The five `From` conversions into `HttpAnnounceError` (peer-IP resolution, tracker-core, announce, whitelist and key errors) | 3A |
| `services/announce.rs` | 420-423, 426-428, 430 | `From<HttpAnnounceError> for HttpProtocolErrorResponse`: the client-visible failure reason of every variant | 3A |
| `services/announce.rs` | 211, 213, 215, 217, 226 (regions only; the lines count as covered) | The early-return error path of each `?` in `handle_announce`: authentication, authorization, client-IP resolution, peer-IP selection, and the tracker-core announce failure. The order of these steps is a package-owned decision and no test makes one of them fail through `handle_announce`. | 3A |
| `services/announce.rs` | 350 | The absent-sender path of `send_event` (every announce test supplies a sender; the scrape tests already cover it) | 3A |
| `services/announce.rs` (test module) | 487-491, 566, 569, 572, 598, 683 | The persistent-completed-stat fixture branch, the `Stopped`, `Completed` and `None` arms of the request-builder fixture, the `mock!` macro, and an `unreachable!` | test support; the file plan reviews the fixture |
| `services/error_mapping.rs` | 4-20, 22 | The function entry, the four named `TrackerCoreError` arms (lines 6-17) and the wildcard arm for an unknown variant (lines 18-20). The wildcard arm cannot be constructed by a test while `TrackerCoreError` has only four variants. | 3A for the four named arms; owner (`tracker-core`) for the wildcard arm: not testable until `tracker-core` adds a variant |
| `services/scrape.rs` | 112, 115, 127 (regions only) | The error paths of the `?` after the tracker-core scrape and after client-IP resolution in `handle_scrape`, and the arm where a key is presented (`Some(key)` at line 127; the no-key path is covered by the existing private-mode test) | 3A |
| `services/scrape.rs` | 128 | An authenticated scrape in private mode (`is_authenticated` with a key) | 3A |
| `services/scrape.rs` | 164-168, 172-176, 180-184, 188-192, 196-200, 204-207, 209 | The five `From` conversions into `HttpScrapeError` and the conversion to the failure response | 3A |
| `services/scrape.rs` (test module) | 318 | The `mock!` macro | test support |
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

## Per-File Results

Each completed file records its clean unit-only measurement (`cargo llvm-cov clean --workspace`, then `cargo llvm-cov -p torrust-tracker-http-core --all-features --lib`, `cargo-llvm-cov 0.6.16`, stable Rust 1.99.0). The totals include the in-file test module, so a file that gains tests gains covered lines; the file plan records which production decisions each mutation pins.

### `services/announce.rs` (slice S2)

Measured on 2026-10-20 on the S2 branch, whose production code is identical to `develop` at `87690b15e`. The plan is [services-announce-tests.md](test-refactor-plans/services-announce-tests.md).

| Scope | Before (`87690b15e`) | After |
| --- | ---: | ---: |
| `services/announce.rs` lines | 675 / 727 (92.85%) | 843 / 851 (99.06%) |
| `services/announce.rs` regions | 878 / 938 (93.60%) | 1,039 / 1,049 (99.05%) |
| `services/announce.rs` functions | 56 / 63 (88.89%) | 92 / 92 (100.00%) |

The file's 8 tests are now 50 cases (48 at the completed-file review; two were added after the independent review found a surviving mutant, see the plan). Every production line, region and function of the file is covered: the 8 uncovered lines and 10 uncovered regions are the failure branches of `matches!` assertions in the test module. Property-based testing was assessed and not selected: two mutants of the peer-IP selection survived the first examples (a loopback `ip` literal replaced by the external IP, and an empty `ip=` that skipped the loopback substitution), each is now killed by one added example, and no further survivor is known.

### `services/error_mapping.rs` (slice S2)

Measured on 2026-10-20 on the S2 branch, whose production code is identical to `develop` at `87690b15e`. The plan is [services-error-mapping-tests.md](test-refactor-plans/services-error-mapping-tests.md).

| Scope | Before (`87690b15e`) | After |
| --- | ---: | ---: |
| `services/error_mapping.rs` lines | 0 / 18 (0.00%) | 26 / 29 (89.66%) |
| `services/error_mapping.rs` regions | 0 / 13 | 17 / 19 (89.47%) |
| `services/error_mapping.rs` functions | 0 / 1 | 3 / 3 (100.00%) |

The 29 counted lines are the 18 production lines (15 covered) and 11 test-module lines. The three uncovered lines are the wildcard arm for an unknown `TrackerCoreError` variant, which no test can reach while the enum has four variants and is `#[non_exhaustive]`; `tracker-core` owns the change that would unlock it.

### Package After Slice S2

Measured on 2026-10-20 at the final S2 commit with the same commands. The package has 79 unit tests (33 before).

| Scope | Before (`87690b15e`) | After slice S2 | Change |
| --- | ---: | ---: | ---: |
| Lines | 1,751 / 1,970 (88.88%) | 1,919 / 2,101 (91.34%) | +2.45 pp |
| Regions | 2,357 / 2,607 (90.41%) | 2,511 / 2,716 (92.45%) | +2.04 pp |
| Functions | 161 / 191 (84.29%) | 198 / 221 (89.59%) | +5.30 pp |

The totals include the test modules, so part of the change is test code that is itself covered. The signal that matters is the per-file mutation proof in each plan.

### `services/scrape.rs` (slice S3)

Measured on 2026-10-20 on the S3 branch, whose production code is identical to `develop` at the merge of #2575. The plan is [services-scrape-tests.md](test-refactor-plans/services-scrape-tests.md).

| Scope | Before (S2 baseline) | After |
| --- | ---: | ---: |
| `services/scrape.rs` lines | 507 / 540 (93.89%) | 533 / 537 (99.26%) |
| `services/scrape.rs` regions | 678 / 711 (95.36%) | 730 / 735 (99.32%) |
| `services/scrape.rs` functions | 36 / 43 (83.72%) | 70 / 70 (100.00%) |

The file's 8 tests are now 22 cases (19 at the completed-file review; three were added after the independent review found a surviving mutant, see the plan). Every production line, region and function of the file is covered: the four uncovered lines and five uncovered regions are the failure branches of `matches!` assertions in the test module. A shared `RecordingSender` moved into `lib.rs`, whose test module is 41 / 41 lines.

### Package After Slice S3

Measured on 2026-10-20 at the final S3 commit with the same commands. The package has 93 unit tests (33 before slice S2).

| Scope | Original baseline (`87690b15e`) | After slice S2 | After slice S3 | Change since S2 |
| --- | ---: | ---: | ---: | ---: |
| Lines | 1,751 / 1,970 (88.88%) | 1,919 / 2,101 (91.34%) | 1,949 / 2,102 (92.72%) | +1.38 pp |
| Regions | 2,357 / 2,607 (90.41%) | 2,511 / 2,716 (92.45%) | 2,566 / 2,743 (93.55%) | +1.10 pp |
| Functions | 161 / 191 (84.29%) | 198 / 221 (89.59%) | 233 / 249 (93.57%) | +3.98 pp |

Since the original baseline the package gained +3.84 pp of lines, +3.14 pp of regions and +9.28 pp of functions. The totals include the test modules; the per-file mutation proof in each plan is the stronger signal.

### Re-measurement Before Slice S4

The S3 merge (#2586) brought four `develop` commits of #2414 that changed `packages/http-core/`: `fix(http-core): [#2414] drain the statistics listener's queue when it is cancelled`, its helper in `torrust-tracker-events`, a re-export of the default drain timeout, and a test import cleanup. Only `statistics/event/listener.rs` changed, in production code and in tests, so the S4 baseline was measured again at `fefefdd9e` (same commands; the package runs 95 unit tests, 93 at the S3 head).

| Scope | After slice S3 (`1b0176415`) | S4 baseline (`fefefdd9e`) |
| --- | ---: | ---: |
| Lines | 1,949 / 2,102 (92.72%) | 2,040 / 2,197 (92.85%) |
| Regions | 2,566 / 2,743 (93.55%) | 2,664 / 2,845 (93.64%) |
| Functions | 233 / 249 (93.57%) | 244 / 260 (93.85%) |

| Source module | Lines before → after | Regions before → after | Functions before → after |
| --- | ---: | ---: | ---: |
| `statistics/event/listener.rs` | 124 / 144 → 215 / 239 | 177 / 196 → 275 / 298 | 18 / 21 → 29 / 32 |

The inventory rows and gap entries for `statistics/event/listener.rs` above (for example, the `TcpAnnounce` region and the lifecycle entries owned by #1488) predate #2414 and must be re-read against the file before the S5 plan; do not trust their line numbers.

### `event.rs` (slice S4)

Measured on 2026-10-20 on the S4 branch, whose production code is identical to `develop` at the merge of #2586. The plan is [event-tests.md](test-refactor-plans/event-tests.md).

| Scope | Before (S4 baseline) | After |
| --- | ---: | ---: |
| `event.rs` lines | 157 / 187 (83.96%) | 135 / 135 (100.00%) |
| `event.rs` regions | 259 / 294 (88.10%) | 184 / 184 (100.00%) |
| `event.rs` functions | 15 / 20 (75.00%) | 22 / 22 (100.00%) |

The file's 7 tests are now 29 cases in 10 test functions (27 when the file was first completed; the independent review added two label cases). The 30 uncovered lines before were the four unused accessors (12 lines) and the dead `announce_events_match` helper (18 counted lines, removed with its module); the accessors are now pinned as public API.

### `container.rs` (slice S4)

Measured on 2026-10-21 after the pull-request review round, on the S4 branch, whose production code is identical to `develop` at the merge of #2586. The plan is [container-tests.md](test-refactor-plans/container-tests.md).

| Scope | Before (S4 baseline) | After |
| --- | ---: | ---: |
| `container.rs` lines | 0 / 73 (0.00%) | 518 / 520 (99.62%) |
| `container.rs` regions | 0 / 89 (0.00%) | 675 / 678 (99.56%) |
| `container.rs` functions | 0 / 5 (0.00%) | 58 / 58 (100.00%) |

The file had no tests. It now has 27, in four modules, one per entry point (20 when first completed; the independent review split three tests and the pull-request review parameterized four over the usage-statistics policy). The counted lines include the new test module; all 73 production lines are covered, and the uncovered regions are `matches!` failure branches in the test module. llvm-cov does not count the body lines of a test that is both an `rstest` table and a `#[tokio::test]`, which is why the counted totals fell by 80 lines when four tests became tables, while the production lines stayed covered. One mutation survived: the swarm registry's usage-statistics flag in `initialize`, which no part of this package can observe (see the plan).

### Package After Slice S4

Measured on 2026-10-21 after the pull-request review round, with the same commands. The package has 144 unit tests (95 at the S4 baseline).

| Scope | Original baseline (`87690b15e`) | After slice S3 | S4 baseline (`fefefdd9e`) | After slice S4 |
| --- | ---: | ---: | ---: | ---: |
| Lines | 1,751 / 1,970 (88.88%) | 1,949 / 2,102 (92.72%) | 2,040 / 2,197 (92.85%) | 2,536 / 2,592 (97.84%) |
| Regions | 2,357 / 2,607 (90.41%) | 2,566 / 2,743 (93.55%) | 2,664 / 2,845 (93.64%) | 3,264 / 3,324 (98.19%) |
| Functions | 161 / 191 (84.29%) | 233 / 249 (93.57%) | 244 / 260 (93.85%) | 309 / 315 (98.10%) |

Since the original baseline the package gained +8.96 pp of lines, +7.78 pp of regions and +13.81 pp of functions. Most of the S4 line gain is the test module of `container.rs`, which counts toward the totals; the per-file mutation proof in each plan is the stronger signal. The S3 column is not comparable line by line with the S4 baseline because #2414 changed `statistics/event/listener.rs` in between.

### Baseline Before Slice S5

Measured on 2026-10-21 on the S5 branch at the merge of #2593, whose `packages/http-core` is identical to the S4 head, so the package figures equal the final S4 ones: lines 2,536 / 2,592 (97.84%), regions 3,264 / 3,324 (98.19%), functions 309 / 315 (98.10%), 144 unit tests. The statistics files measured:

| Source module | Lines | Regions | Functions |
| --- | ---: | ---: | ---: |
| `statistics/event/handler.rs` | 158 / 161 | 215 / 221 | 16 / 17 |
| `statistics/event/listener.rs` | 215 / 239 | 275 / 298 | 29 / 32 |
| `statistics/metrics.rs` | 48 / 57 | 66 / 74 | 8 / 9 |
| `statistics/repository.rs` | 21 / 24 | 26 / 29 | 5 / 6 |
| `statistics/mod.rs` | 8 / 8 | 10 / 10 | 1 / 1 |

### `statistics/event/handler.rs` (slice S5)

Measured on 2026-10-21 on the S5 branch, whose production code for this file is identical to `develop` at the merge of #2593. The plan is [statistics-handler-tests.md](test-refactor-plans/statistics-handler-tests.md).

| Scope | Before (S5 baseline) | After |
| --- | ---: | ---: |
| `statistics/event/handler.rs` lines | 158 / 161 (98.14%) | 173 / 176 (98.30%) |
| `statistics/event/handler.rs` regions | 215 / 221 (97.29%) | 202 / 208 (97.12%) |
| `statistics/event/handler.rs` functions | 16 / 17 (94.12%) | 23 / 24 (95.83%) |

The file's 5 tests are now 11 cases in 5 test functions (10 cases in 4 functions when first completed; the independent review added a test of the recorded time). The production lines were already covered except the two the plan decides not to test: the failed-increase `Err` arm (line 20) and the debug closure (line 25). The figures are those after the review round.

### `statistics/event/listener.rs` (slice S5)

Measured on 2026-10-21 on the S5 branch, whose production code for this file is identical to `develop` at the merge of #2593. The plan is [statistics-listener-tests.md](test-refactor-plans/statistics-listener-tests.md).

| Scope | Before (S5 baseline) | After |
| --- | ---: | ---: |
| `statistics/event/listener.rs` lines | 215 / 239 (89.96%) | 354 / 356 (99.44%) |
| `statistics/event/listener.rs` regions | 275 / 298 (92.28%) | 440 / 444 (99.10%) |
| `statistics/event/listener.rs` functions | 29 / 32 (90.63%) | 48 / 48 (100.00%) |

The file keeps its 6 tests and gains 11 cases in 6 test functions: the metrics policy for announce events, every queued event drained after cancellation, the two public entry points (`Completion` and the spawned task, with a table that proves the spawned listener receives its token and its drain timeout) and the time an event is recorded at (the last two came from the independent review). All production code is covered except the log argument of the ignored-event `warn!` (line 132); the one other uncovered line is the `Lagged` arm of the test receiver. The figures are those after the review round.

### `statistics/metrics.rs`, `statistics/repository.rs` and `statistics/mod.rs` (slice S5)

Measured on 2026-10-21 on the S5 branch, whose production code for these files is identical to `develop` at the merge of #2593. The plans are [statistics-metrics-tests.md](test-refactor-plans/statistics-metrics-tests.md), [statistics-repository-tests.md](test-refactor-plans/statistics-repository-tests.md) and [statistics-mod-tests.md](test-refactor-plans/statistics-mod-tests.md).

| Source module | Lines before → after | Regions before → after | Functions before → after |
| --- | ---: | ---: | ---: |
| `statistics/metrics.rs` | 48 / 57 → 167 / 167 | 66 / 74 → 295 / 295 | 8 / 9 → 21 / 21 |
| `statistics/repository.rs` | 21 / 24 → 153 / 153 | 26 / 29 → 211 / 211 | 5 / 6 → 25 / 25 |
| `statistics/mod.rs` | 8 / 8 → 41 / 41 | 10 / 10 → 55 / 55 | 1 / 1 → 5 / 5 |

The figures are those after the review round. The counted lines include the new test modules, so the rises far exceed the production lines they protect; every production line and function of the three files is covered, including the `set_gauge` wrapper and `Repository::default`, which were the uncovered code. The mutation results in each plan are the stronger signal; the metric-name test is the only test that fails when the public series name changes, and the review round added whole-value tests that also pin the recorded time and the metric unit.

### Package After Slice S5

Measured on 2026-10-21 after the independent review round with the same commands. The package has 183 unit tests (144 at the S5 baseline).

| Scope | Original baseline (`87690b15e`) | S5 baseline (`b5ab35ae9`) | After slice S5 |
| --- | ---: | ---: | ---: |
| Lines | 1,751 / 1,970 (88.88%) | 2,536 / 2,592 (97.84%) | 2,974 / 2,996 (99.27%) |
| Regions | 2,357 / 2,607 (90.41%) | 3,264 / 3,324 (98.19%) | 3,875 / 3,905 (99.23%) |
| Functions | 161 / 191 (84.29%) | 309 / 315 (98.10%) | 372 / 373 (99.73%) |

Since the original baseline the package gained +10.39 pp of lines, +8.82 pp of regions and +15.44 pp of functions. The uncovered production lines of the statistics files are three, all decided in their plans: the failed-increase `Err` arm (line 20) and the debug closure (line 25) of `handler.rs`, and the ignored-event `warn!` argument (line 132) of `listener.rs`; the one other uncovered line (195 of `listener.rs`) is the `Lagged` arm of the test receiver. The totals include the test modules.

### Baseline Before Slice S6

Measured on 2026-10-21 on the S6 branch at the merge of #2608 (`889a6dc36`), whose `packages/http-core` is identical to the S5 head, so the package figures equal the final S5 ones: lines 2,974 / 2,996 (99.27%), regions 3,875 / 3,905 (99.23%), functions 372 / 373 (99.73%), 183 unit tests. Clean `cargo llvm-cov clean --workspace` then `cargo llvm-cov -p torrust-tracker-http-core --all-features --lib --json` on the stable toolchain.

| Source module | Lines | Regions | Functions |
| --- | ---: | ---: | ---: |
| `lib.rs` | 41 / 41 | 42 / 42 | 8 / 8 |
| `services/mod.rs` | no executable entries | no executable entries | no executable entries |
| `statistics/event/mod.rs` | no executable entries | no executable entries | no executable entries |

### `lib.rs` (slice S6)

Measured on 2026-10-21 on the S6 branch, whose production code in `lib.rs` is identical to `develop` (the diff has no removed line). The plan is [lib-tests.md](test-refactor-plans/lib-tests.md).

| Source module | Lines before → after | Regions before → after | Functions before → after |
| --- | ---: | ---: | ---: |
| `lib.rs` | 41 / 41 → 97 / 97 | 42 / 42 → 150 / 150 | 8 / 8 → 20 / 20 |

The file keeps no production line to cover; the eight new tests pin the public log target and the shared `RecordingSender` and fixtures that more than 90 call sites rely on. Package after the file: 191 unit tests, lines 3,030 / 3,052, regions 3,983 / 4,013, functions 384 / 385 (the totals include the test modules).

### `services/mod.rs` and `statistics/event/mod.rs` (slice S6)

Measured on 2026-10-21 on the S6 branch. Neither file has an executable entry, before or after, so no coverage figure moves. The plans are [services-mod-tests.md](test-refactor-plans/services-mod-tests.md) (no test; the module comment now says the services do not depend on the Axum web server, which the package's `Cargo.toml` confirms) and [statistics-event-mod-tests.md](test-refactor-plans/statistics-event-mod-tests.md) (no change). With these two rows, all 13 files of the ledger are processed.

## Final Measurement (T8)

Measured on 2026-10-21 at 15:16 UTC on the S6 branch (stable Rust 1.99.1, `cargo-llvm-cov`), after the last test and the `services/mod.rs` comment correction. Each command ran after `cargo llvm-cov clean --workspace`:

```text
cargo llvm-cov -p torrust-tracker-http-core --all-features --json --output-path .tmp/1349-coverage/final-aggregate.json
cargo llvm-cov -p torrust-tracker-http-core --all-features --lib --json --output-path .tmp/1349-coverage/final-unit.json
```

Both ran the same 191 unit tests. The package has no `tests/` directory, so there is no integration-only report, and the aggregate figures equal the unit-only ones.

| Scope | Original baseline (`87690b15e`) | Final (unit-only and aggregate) | Change |
| --- | ---: | ---: | ---: |
| Lines | 1,751 / 1,970 (88.88%) | 3,030 / 3,052 (99.28%) | +10.40 pp |
| Regions | 2,357 / 2,607 (90.41%) | 3,983 / 4,013 (99.25%) | +8.84 pp |
| Functions | 161 / 191 (84.29%) | 384 / 385 (99.74%) | +15.45 pp |

The counted totals include the new test modules, so the percentages rise more than the production lines alone; each file's production lines are the better judge, and the per-file plans record them. Tests went from 33 to 191.

The 22 uncovered lines are:

| Location | Lines | Kind | Owner or decision |
| --- | ---: | --- | --- |
| `services/error_mapping.rs` 18-20 | 3 | Production | The wildcard arm for an unknown `TrackerCoreError` variant; the enum is `#[non_exhaustive]` with four variants, so no code outside `tracker-core` can build a fifth (error-mapping plan). |
| `statistics/event/handler.rs` 20 and 25 | 3 (summary) / 2 (text) | Production | The failed-counter-increase `Err` arm and the debug closure; reachable only with a failing metrics repository or log capture (handler plan). The mutation run records `log_counter_increased` as the one missed mutant. |
| `statistics/event/listener.rs` 132 | 1 | Production | The `warn!` argument for an ignored event (listener plan). |
| `statistics/event/listener.rs` 200 | 1 | Test support | The `Lagged` arm of the scripted test receiver. |
| `container.rs`, `services/announce.rs`, `services/scrape.rs` test modules | 14 | Test modules | Lines inside test bodies that `llvm-cov` does not count for `rstest` tables combined with `#[tokio::test]` (see the lessons). |

Seven production lines remain uncovered, all with a recorded owner. The counts are those of the `llvm-cov` JSON summary; the line-level text report of the same run marks 20 lines instead of 22: it marks lines 20 and 25 of `handler.rs` where the summary counts three, and one line of `container.rs` where the summary counts two; the other files agree. File-level counts can therefore differ from a line listing by a line. Mutation evidence: [mutation-evidence.md](mutation-evidence.md).

### Final Test-Technique Assessment

| Technique | Outcome |
| --- | --- |
| Unit tests | Selected and delivered: all 13 ledger files processed, 191 tests. |
| Package integration tests | Not needed. No plan selected a 3B increment; the package has no network boundary of its own, so T5 had no work. |
| Runnable examples | Not needed. The existing doc tests of `statistics/repository.rs` were kept. |
| Root `tests/` or `packages/e2e-tools/` | Not needed. Real HTTP behavior stays with `axum-http-server` and the root tests; the manual verification (M1-M5) exercised it by hand. |
| Mutation testing | Done on the whole package (96 mutants; 50 caught, 1 missed, 1 timeout, 44 unviable). |
| Property-based testing | Not adopted. The announce plan assessed it: two mutants of `select_peer_ip_with_policy` survived the examples and were each fixed with one more example, so no property earned a new dependency. |
| Fuzzing | Not selected: untrusted parsing belongs to `http-protocol`. |
| Benchmarks | Out of scope. |
