---
doc-type: mutation-evidence
issue: 1349
package: torrust-tracker-http-core
status: completed
measured-utc: 2026-10-21T15:14:40Z
---

# HTTP Core Mutation Evidence

This is a bounded mutation-testing assessment for Issue #1349. Unlike #1348, which sampled one file, the whole
package has only 96 mutants, so every source file was run. It is not a package score, a coverage target or a CI
gate.

## Configuration

```text
RUSTFLAGS="--cap-lints=warn" cargo mutants \
  --package torrust-tracker-http-core \
  --all-features \
  --timeout 300 \
  --no-times \
  --output .tmp/1349-mutants.out \
  -- --lib
```

`cargo-mutants 27.0.0`, stable Rust 1.99.1. The tree was the S6 branch after the `lib.rs` tests and the
`services/mod.rs` comment correction (production code identical to `develop` apart from that comment). The
baseline run without mutations ran the 191 unit tests of the package and passed; doc tests were not part of the run. The
`.tmp` output is a git-ignored working-session artifact. The first attempt was interrupted by the environment
during the baseline and was discarded and rerun from the start.

`--cap-lints=warn` follows the #1348 lesson (the workspace denies `unused`, which would turn replaced bodies into
unviable mutants). The unviable mutants below are therefore not lint failures.

## Results

96 mutants tested: **50 caught, 1 missed, 1 timeout, 44 unviable.**

| File | Caught | Missed | Timeout | Unviable |
| --- | ---: | ---: | ---: | ---: |
| `container.rs` | 0 | 0 | 0 | 4 |
| `event.rs` | 12 | 0 | 0 | 7 |
| `services/announce.rs` | 10 | 0 | 0 | 15 |
| `services/error_mapping.rs` | 0 | 0 | 0 | 1 |
| `services/scrape.rs` | 8 | 0 | 0 | 7 |
| `statistics/event/handler.rs` | 3 | 1 | 0 | 1 |
| `statistics/event/listener.rs` | 3 | 0 | 1 | 7 |
| `statistics/metrics.rs` | 12 | 0 | 0 | 1 |
| `statistics/mod.rs` | 1 | 0 | 0 | 0 |
| `statistics/repository.rs` | 1 | 0 | 0 | 1 |

### Missed

`statistics/event/handler.rs:37`: replace `log_counter_increased` with `()`. The function only writes a `tracing`
debug line after a counter increase. No test captures log output, and the handler plan decided that the
increase's debug closure is not worth a log-capturing harness (lines 20 and 25 of that file are the two decided
uncovered lines). No production behavior changes. Selected action: none; recorded in `lessons.md`.

### Timeout

`statistics/event/listener.rs:106`: replace `should_stop_after_receiving_event` with `false`. The drain then never
stops, so the run exceeds the 300 s timeout. This is the known limit recorded in the listener plan: the scripted
receiver returns `Closed` forever, so the mutated loop spins and no assertion can fire. The mutant is detected (the
suite cannot pass with it) but by the timeout, not by an assertion; the lifecycle policy belongs to #1488. Selected
action: none.

### Unviable

All 44 failed to build. None is a lint failure (see the configuration). The reasons, read from the per-mutant logs:

| Reason | Mutants | Examples |
| --- | ---: | --- |
| The replaced return type has no `Default` implementation, so `Default::default()` does not compile | 37 | `HttpTrackerCoreContainer::initialize` and its three siblings (`Arc<Self>`); every `AnnounceService`/`ScrapeService` constructor; `select_peer_ip` and `external_ip_for_loopback_connection` (`IpAddr`); `ConnectionContext::with_public_url`, `client_ip_addr`, `server_socket_addr` and the other `event.rs` accessors (`IpAddr`, `SocketAddr`, `IpFamily`, `IpType`, `ConfigurationInstanceId`); the 13 `From<...>` conversions into `HttpTrackerPolicy`, `HttpAnnounceError`, `HttpScrapeError` and `HttpProtocolErrorResponse`; `protocol_error_from_tracker_core_error`; `labels_and_request_kind` (`RequestKind`); `event_connection_id`; `run_event_listener_unspawned` and `dispatch_events` (`Completion`); `Repository::read_metrics` (generic `R`) |
| A `const fn` cannot call the non-const `Default::default()` | 3 | `PeerIpSelectionPolicy::disabled` and `enabled`; `Metrics::metric_collection` (`Box::leak(Box::new(Default::default()))`) |
| The generated replacement does not type-check | 4 | the four `JoinHandle` replacements of `run_event_listener` (`JoinHandle::from(())`, `from_iter`, `new(())`, `new()`) |

These are the behavior-changing mutants the tool could not try: for example, `select_peer_ip` returning a default
address. They are covered instead by the named mutations of the per-file plans, which were written to compile
(for example the announce plan's peer-IP selection mutations and the `error_mapping` plan's variant-by-variant
mutations), and by the other mutants caught in the same functions (the match-guard mutants of
`select_peer_ip_with_policy` were caught). The unviable `From` conversions and the error mapping are protected by
whole-value assertions on every error variant (see the error-mapping and announce plans).

## Scope And Limitations

- The tool generates only function-body and operator replacements. It cannot reach the decisions the plans pinned
  by hand: label tables, the policy matrix of the services, the `tcp4`/`tcp6` projection and the recorded
  metric time and unit. Those are proved by the named mutations in each plan.
- No score target and no CI gate. The 50 caught mutants show that the tests react to the replacements the tool can
  build; they do not measure test quality.

## Follow-Up

None selected. The two survivors (the debug log and the drain's stop condition) are recorded in `lessons.md`; the
second belongs to #1488.
