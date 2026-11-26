---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/3060-udp-core-debug-log-target-field/ISSUE.md
last-updated-utc: "2026-11-26 16:39"
---

# Manual Verification Evidence

## Environment and Prerequisites

- Date and time (UTC): 2026-11-25 17:36 to 17:37 (a first run at 16:59 to 17:00 used `capture_logs_while` and is superseded by the run below, which uses the helper the spec plans for the maintained tests).
- Artifact under test: `torrust/develop` at the merge of #3053 (`f72f93c5c`, already on `develop`) plus the docs-only commit `docs(issues): add issue specification for #3060`, stable Rust toolchain. The only code change is the temporary tests below, reverted afterwards.
- Operating system / environment: Linux, local checkout.
- Prerequisites: none beyond `cargo`.

## Verification Processes

### V1 - Captured DEBUG lines of announce and scrape

- Goal: see the log target of the two DEBUG lines as an operator would.
- Initial state: unfixed code.
- Status: `IN_PROGRESS` (before-fix result recorded; after-fix rerun TODO)
- Outcome class: **Reproduced** (the wrong target itself was observed, for both services).

#### Steps Performed

1. Appended these two temporary tests to the `tests` modules of `announce.rs` and `scrape.rs` (verbatim; the complete diff is in `evidence/v1-temporary-tests.diff`):

   ```rust
       #[tokio::test]
       async fn temp_repro_announce_log_target() {
           use torrust_tracker_test_helpers::logging::capture_logs_in_scope;
           let mut event_sender = MockUdpCoreStatsEventSender::new();
           event_sender.expect_send().returning(|_| Box::pin(future::ready(Some(Ok(1)))));
           let service = AnnounceServiceFixture::public_tracker_publishing_events_to(event_sender).service;
           let request = announce_request(valid_connection_id_for(client_socket_addr()));
           let capture = capture_logs_in_scope();
           service
               .handle_announce(client_socket_addr(), server_service_binding(), &request, cookie_valid_range(), VALIDATE_COOKIE)
               .await
               .unwrap();
           println!("CAPTURED>>>{}<<<", capture.lines().join("\n"));
       }

       #[tokio::test]
       async fn temp_repro_scrape_log_target() {
           use torrust_tracker_test_helpers::logging::capture_logs_in_scope;
           let mut event_sender = MockUdpCoreStatsEventSender::new();
           event_sender.expect_send().returning(|_| Box::pin(future::ready(Some(Ok(1)))));
           let service = ScrapeServiceFixture::public_tracker_publishing_events_to(event_sender).service;
           let request = scrape_request(valid_connection_id_for(client_socket_addr()));
           let capture = capture_logs_in_scope();
           service
               .handle_scrape(client_socket_addr(), server_service_binding(), &request, cookie_valid_range(), VALIDATE_COOKIE)
               .await
               .unwrap();
           println!("CAPTURED>>>{}<<<", capture.lines().join("\n"));
       }
   ```

2. Ran `cargo test -p torrust-tracker-udp-core temp_repro_ -- --nocapture --test-threads=1` (started 17:36:48 UTC, ended 17:36:50 UTC). The saved output keeps only the lines `running`, `test ...` and `test result` (cargo's build lines carry local paths); it is in `evidence/v1-before-fix.txt`.
3. Reverted both files with `git checkout -- packages`; `git status --short` showed no change under `packages` afterwards.
4. Counted the library tests with `cargo test -p torrust-tracker-udp-core --lib -- --list | grep -c ': test$'`: 114 with the two temporary tests and 112 after the revert (hence the `112 filtered out` below).

#### Observed Result

Announce, verbatim:

```text
CAPTURED>>>2026-11-25T17:36:50.137933Z DEBUG torrust_tracker_udp_core::services::announce: Sending UdpAnnounce event: UdpAnnounce { connection: ConnectionContext { configuration_instance_id: ConfigurationInstanceId { service_role: UdpTracker, instance_index: 0 }, client_socket_addr: 203.0.113.10:51000, server_service_binding: ServiceBinding { protocol: UDP, bind_address: 203.0.113.196:6969 }, public_url: None }, info_hash: InfoHash([17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17]), announcement: Peer { peer_id: PeerId([45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45]), peer_addr: 203.0.113.10:6881, updated: 1795628210.137917836s, uploaded: NumberOfBytes(0), downloaded: NumberOfBytes(0), left: NumberOfBytes(0), event: Started } } target="UDP TRACKER"<<<
```

Scrape, verbatim:

```text
CAPTURED>>>2026-11-25T17:36:50.138117Z DEBUG torrust_tracker_udp_core::services::scrape: Sending UdpScrape event: UdpScrape { connection: ConnectionContext { configuration_instance_id: ConfigurationInstanceId { service_role: UdpTracker, instance_index: 0 }, client_socket_addr: 203.0.113.10:51000, server_service_binding: ServiceBinding { protocol: UDP, bind_address: 203.0.113.196:6969 }, public_url: None } } target="UDP TRACKER"<<<
```

Test summary line, as printed:

```text
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 112 filtered out; finished in 0.00s
```

In both lines the target printed before the message is the module path (`torrust_tracker_udp_core::services::announce` and `...::scrape`), and the intended value appears only as the trailing field `target="UDP TRACKER"`.

#### Conclusion

Both lines are logged under the module-path target with the stray field, as the spec describes. The after-fix rerun is TODO (task T4).

### V1 scope scans (same tree, 2026-11-25 17:37 UTC)

Single-line scan:

```text
$ git grep -nE '\(\s*target\s*=' -- '*.rs'
packages/udp-core/src/services/announce.rs:178:            tracing::debug!(target = crate::UDP_TRACKER_LOG_TARGET, "Sending UdpAnnounce event: {event:?}");
packages/udp-core/src/services/scrape.rs:140:            tracing::debug!(target = crate::UDP_TRACKER_LOG_TARGET, "Sending UdpScrape event: {event:?}");
```

Multi-line scan (the AC3 command; GNU `grep -z` reads a file as one record, so `(?s)` matches across lines; its output is NUL-separated, shown here through `tr '\0' '\n'`):

```text
$ grep -rPzo '(?s)\b(?:trace|debug|info|warn|error|event|span|trace_span|debug_span|info_span|warn_span|error_span)!\s*\(\s*(?:(?:[^;{}()]|\([^()]*\))*?,\s*)?target\s*=[^=]' packages src --include=*.rs | tr '\0' '\n'
packages/udp-core/src/services/scrape.rs:debug!(target =
packages/udp-core/src/services/announce.rs:debug!(target =
```

Both scans find only the two known sites. The match `debug!(target =` is printed with one trailing space, omitted here.

#### Detection check of the multi-line scan (first run 2026-11-25 18:03 UTC, rerun with the parenthesis case at 18:13 UTC)

An earlier version of this scan used `git grep -n -P -z -o` with a pattern that allowed no other argument before `target =`. `git grep` matches line by line even with `-z`, so on a sample file it did not detect a call written across lines (`git grep --no-index -c -P -z '(?s)debug!\(\s*target\s*=' -- s.rs` printed nothing), and the pattern missed `event!(Level::DEBUG, target = ...)`. Both were pointed out in review (findings F11 and F12 of PR #3062) and the scan above replaces it. Its detection (a call across lines, a leading argument, and an earlier argument with parentheses) is shown on `evidence/scan-sample.rs`:

```rust
fn f(){
    tracing::debug!(
        target = "X",
        "m"
    );
    tracing::event!(Level::DEBUG, target = "Y", "m");
    tracing::info!(target: "Z", a = 1, "ok");
    tracing::info!(a = 1, b = 2, "target = nope");
}

fn g() {
    tracing::event!(
        tracing::Level::DEBUG,
        count = (1 + 1),
        target = "UDP TRACKER",
        "message"
    );
}

fn h() {
    tracing::debug!(/* stats */ target = "X", "m");
}

fn i() {
    tracing::debug! { target = "X", "m" };
}

fn j() {
    tracing::info!("message, target = nope");
}
```

```text
$ grep -Pzo '(?s)\b(?:trace|debug|info|warn|error|event|span|trace_span|debug_span|info_span|warn_span|error_span)!\s*\(\s*(?:(?:[^;{}()]|\([^()]*\))*?,\s*)?target\s*=[^=]' docs/issues/open/3060-udp-core-debug-log-target-field/evidence/scan-sample.rs | tr '\0' '\n'
debug!(
        target =
event!(Level::DEBUG, target =
event!(
        tracing::Level::DEBUG,
        count = (1 + 1),
        target =
info!("message, target =
```

The multi-line `debug!` call, the `event!` call with a leading argument and the `event!` call whose earlier argument contains parentheses are found (the third case was added after review finding F14 of PR #3062); an earlier argument with nested parentheses, for example `count = ((1 + 1))`, is not matched: a regular expression cannot follow arbitrary nesting, so the claim of AC3 is limited to one level (finding F16 of PR #3062); a comment inside the macro arguments before `target =` (the last function of the sample) is not matched either and is documented as a known limit (finding F17 of PR #3062); `target: "Z"` (colon form) is not. Two cases of the sample show the limits of the scan: the brace-delimited `debug! { target = ... }` call (function `i`) is not matched, which is why AC3 covers parenthesized calls only (finding F18 of PR #3062), and `info!("message, target = nope")` (function `j`) is a false match, because the pattern takes the string up to its comma as an earlier argument and matches `target =` inside the string, so every match must be inspected by hand (finding F20). The commands of this section run from the repository root (finding F19). A false match only makes AC3 fail, the safe direction. The text `"target = nope"` of function `f`, with no comma before it, is not matched.

### T2 - Regression tests, red before the fix

- Goal: show that the two maintained regression tests fail on the unfixed code, for the right reason.
- Artifact under test: `torrust/develop` at the merge of #3070 (`66c12ffe1`, already on `develop`) plus the two new tests `it_should_log_the_sent_event_under_the_udp_tracker_log_target` in `announce.rs` and `scrape.rs`; the production call sites are unchanged. Stable Rust toolchain.
- Status: `DONE` (red run recorded; the green run is recorded after the fix)

#### Steps Performed

1. Added the two tests (one `it_should_log_the_sent_event_under_the_udp_tracker_log_target` in each of `announce.rs` and `scrape.rs`): each starts `capture_logs_in_scope()`, sends one announce (resp. scrape) through a service with a mock event sender, and asserts that exactly one captured line holds `Sending UdpAnnounce event` (resp. `UdpScrape`), that this line contains `UDP TRACKER: Sending ... event`, and that it contains neither the module-path target nor `target="UDP TRACKER"`.
2. Ran `cargo test -p torrust-tracker-udp-core it_should_log_the_sent_event -- --test-threads=1` at 2026-11-26 16:38:54 UTC. The saved lines (`running`, `test ...`, the panic messages and `test result`) are in `evidence/t2-red-before-fix.txt`.

#### Observed Result

```text
running 2 tests
test services::announce::tests::it_should_log_the_sent_event_under_the_udp_tracker_log_target ... FAILED
test services::scrape::tests::it_should_log_the_sent_event_under_the_udp_tracker_log_target ... FAILED
thread 'services::announce::tests::it_should_log_the_sent_event_under_the_udp_tracker_log_target' (1018165) panicked at packages/udp-core/src/services/announce.rs:726:9:
unexpected target: 2026-11-26T16:38:54.357108Z DEBUG torrust_tracker_udp_core::services::announce: Sending UdpAnnounce event: UdpAnnounce { connection: ConnectionContext { configuration_instance_id: ConfigurationInstanceId { service_role: UdpTracker, instance_index: 0 }, client_socket_addr: 203.0.113.10:51000, server_service_binding: ServiceBinding { protocol: UDP, bind_address: 203.0.113.196:6969 }, public_url: None }, info_hash: InfoHash([17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17]), announcement: Peer { peer_id: PeerId([45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45]), peer_addr: 203.0.113.10:6881, updated: 1795711134.357090495s, uploaded: NumberOfBytes(0), downloaded: NumberOfBytes(0), left: NumberOfBytes(0), event: Started } } target="UDP TRACKER"
thread 'services::scrape::tests::it_should_log_the_sent_event_under_the_udp_tracker_log_target' (1018166) panicked at packages/udp-core/src/services/scrape.rs:515:9:
unexpected target: 2026-11-26T16:38:54.357432Z DEBUG torrust_tracker_udp_core::services::scrape: Sending UdpScrape event: UdpScrape { connection: ConnectionContext { configuration_instance_id: ConfigurationInstanceId { service_role: UdpTracker, instance_index: 0 }, client_socket_addr: 203.0.113.10:51000, server_service_binding: ServiceBinding { protocol: UDP, bind_address: 203.0.113.196:6969 }, public_url: None } } target="UDP TRACKER"
test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 112 filtered out; finished in 0.00s
```

#### Conclusion

Both tests fail on the unfixed code at the `UDP TRACKER: Sending ... event` assertion, after the capture had exactly one event line, so the red run is caused by the wrong target (the line is logged under the module path) and not by an empty capture. A failed `assert!` stops the test, so the later assertions (no module-path target, no stray field) were not evaluated in this run; the green run after the fix shows that they pass.

## Failures and Follow-up

The first scope scan of this file (a `git grep -z` form) could not match a call written across lines; the detection check in section V1 scope scans replaces it (finding F12 of PR #3062).

The first V1 run (16:59 to 17:00 UTC, `capture_logs_while`, announce shortened with `...`, scrape cut) was replaced by the run above after review finding F6 of PR #3062 (`review-finding:pr-3062-f6`), which asked for verbatim code and output.
