---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/3060-udp-core-debug-log-target-field/ISSUE.md
last-updated-utc: "2026-11-25 17:06"
---

# Manual Verification Evidence

## Environment and Prerequisites

- Date and time (UTC): 2026-11-25 16:59 to 17:01
- Artifact under test: `torrust/develop` at the merge of #3053 (`f72f93c5c`, already on `develop`), stable Rust toolchain, no changes except the temporary test below.
- Operating system / environment: Linux, local checkout.
- Prerequisites: none beyond `cargo`.

## Verification Processes

### V1 - Captured DEBUG lines of announce and scrape

- Goal: see the log target of the two DEBUG lines as an operator would.
- Initial state: unfixed `develop`.
- Status: `IN_PROGRESS` (before-fix result recorded; after-fix rerun TODO)
- Outcome class: **Reproduced** (the wrong target itself was observed).

#### Steps Performed

1. Appended this temporary test to the `tests` module of `announce.rs` (and the same shape, with `ScrapeServiceFixture`, `scrape_request` and `handle_scrape`, to `scrape.rs`):

   ```rust
   #[test]
   fn temp_repro_debug_target() {
       use torrust_tracker_test_helpers::logging::capture_logs_while;
       let mut event_sender = MockUdpCoreStatsEventSender::new();
       event_sender.expect_send().returning(|_| Box::pin(future::ready(Some(Ok(1)))));
       let service = AnnounceServiceFixture::public_tracker_publishing_events_to(event_sender).service;
       let request = announce_request(valid_connection_id_for(client_socket_addr()));
       let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
       let (_r, logs) = capture_logs_while(|| {
           rt.block_on(service.handle_announce(client_socket_addr(), server_service_binding(), &request, cookie_valid_range(), VALIDATE_COOKIE))
       });
       println!("CAPTURED>>>{logs}<<<");
   }
   ```

2. Ran `cargo test -p torrust-tracker-udp-core temp_repro_debug_target -- --nocapture` (announce at 16:59:53 UTC, scrape at 17:00:39 UTC).
3. Reverted both files with `git checkout -- <file>`; `git status --short` printed nothing afterwards.

#### Observed Result

Announce (event payload shortened with `...`; the rest is verbatim):

```text
2026-11-25T17:00:27.475978Z DEBUG torrust_tracker_udp_core::services::announce: Sending UdpAnnounce event: UdpAnnounce { ... } target="UDP TRACKER"
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 112 filtered out
```

Scrape (cut after the public_url field):

```text
2026-11-25T17:00:43.044160Z DEBUG torrust_tracker_udp_core::services::scrape: Sending UdpScrape event: UdpScrape { ... }
```

#### Conclusion

Both lines are logged under the module-path target, and for announce the intended value appears as a trailing `target="UDP TRACKER"` field, as the spec describes. The scrape output was cut in this record before its trailing field; the after-fix rerun records both lines in full. After-fix rerun: TODO.

## Failures and Follow-up

None yet.
