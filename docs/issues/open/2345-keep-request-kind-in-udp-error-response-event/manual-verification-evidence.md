---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2345-keep-request-kind-in-udp-error-response-event/ISSUE.md
last-updated-utc: 2026-09-27 08:50
---

# Manual Verification Evidence

## Purpose

Record how the bug was reproduced before the fix, and later the like-for-like recheck after the
fix. Every command and output below was actually run and observed.

## Environment and Prerequisites

- Date and time (UTC): 2026-09-26 10:37-10:52
- Artifact under test: `develop` at `dee5ec21` (merge of torrust/torrust-tracker#2340), unmodified
  except for the temporary test in V2, which was reverted afterwards.
- Operating system / environment: Linux 7.0.0-34-generic; stable Rust toolchain
  `rustc 1.98.1 (48a229cea 2026-09-01)`.
- Prerequisites and setup performed: none beyond a working Rust toolchain. Both processes use
  ephemeral test configuration and loopback sockets.

## Why Two Processes

The defective value lives only in the internal `Event::UdpResponseSent` published on the UDP
server event bus. No public surface shows it:

- the UDP client receives an ordinary error response, identical with or without the bug;
- the only event consumer (`statistics/event/handler/response_sent.rs`) ignores `opt_req_kind`
  for errors, so metrics are identical with or without the bug;
- no log line prints published events (checked: the statistics and banning listeners and the
  statistics handlers are not instrumented with the event).

V1 therefore shows that a real tracker reaches the affected code path, and V2 observes the wrong
event value directly on the processor's event bus.

## Verification Processes

### V1 - Real Tracker Reaches the Parsed-Request Error Path

- Goal: show that a strict-mode tracker answers a parsed request with an invalid connection ID
  through the handler-error path (the path whose event is wrong).
- Initial state: ephemeral UDP tracker on loopback, `ConnectionIdValidationPolicy::Strict`.
- Status: `DONE`. Outcome: **Trigger only**; the event field is not observable here, see above.

#### Steps Performed

1. Ran the existing real-loopback contract, which starts the tracker and sends eleven announce
   requests with `ConnectionId::new(0)` from a real UDP client socket, each answered with an error
   response, then a twelfth that is banned before handling:

   ```text
   cargo test -p torrust-tracker-udp-server --test integration should_ban_the_client_ip_if_it_sends_more_than_10_requests_with_a_cookie_value_not_normal
   ```

#### Observed Result

Eleven warnings like the following (one shown), then the test passed:

```text
WARN UDP TRACKER: response error error=tracker announce error: Connection cookie error: cookie value is expired: -0.00000000000000000000000000000000005855520076693037, expected > 1790418899.0128546 client_socket_addr=127.0.0.1:40991 server_socket_addr=127.0.0.1:46138 service_binding=udp://127.0.0.1:46138 request_id=e8801512-ade2-414d-8ca1-14c8a12b40a8 transaction_id=2005589032
test server::contract::receiving_an_announce_request::should_ban_the_client_ip_if_it_sends_more_than_10_requests_with_a_cookie_value_not_normal ... ok
```

#### Conclusion

The request was parsed as an announce and failed in its handler, so the tracker sent an error
response through `Processor::send_response` with a known request kind. This is the trigger for the
bug; the published event is not visible in this artifact.

### V2 - Published Event Drops the Known Request Kind

- Goal: observe `Event::UdpResponseSent` for a parsed request that fails in its handler.
- Initial state: the existing `processor.rs` test fixture (ephemeral container, strict
  validation, direct event-bus receiver) plus a real loopback client socket with a nonzero port.
- Status: `DONE`. Outcome: **Reproduced**.

#### Steps Performed

1. Temporarily appended this test to the `tests` module of
   `packages/udp-server/src/server/processor.rs`. It reuses the module's existing
   `setup_processor_with_event_receiver` and `receive_event` helpers:

   ```rust
   #[tokio::test]
   async fn temporary_reproduction_parsed_scrape_error_response_event_kind() {
       use torrust_tracker_udp_protocol::{ConnectionId, InfoHash, ScrapeRequest};

       let (processor, mut event_receiver) = setup_processor_with_event_receiver().await;
       let client_socket = tokio::net::UdpSocket::bind("127.0.0.1:0").await.unwrap();
       let client_addr = client_socket.local_addr().unwrap();

       let scrape_request = Request::from(ScrapeRequest {
           connection_id: ConnectionId::new(0),
           transaction_id: TransactionId(0i32.into()),
           info_hashes: vec![InfoHash([0u8; 20])],
       });
       let mut payload = Vec::new();
       scrape_request.write_bytes(&mut payload).unwrap();

       processor.process_request(RawRequest { payload, from: client_addr }).await;

       loop {
           let event = receive_event(&mut event_receiver).await;
           println!("published event: {event:?}");
           if let Event::UdpResponseSent { kind, .. } = event {
               println!("UdpResponseSent.kind = {kind:?}");
               break;
           }
       }
   }
   ```

2. Ran it:

   ```text
   cargo test -p torrust-tracker-udp-server temporary_reproduction_parsed_scrape_error_response_event_kind --lib -- --nocapture
   ```

3. Reverted the temporary test with `git checkout -- packages/udp-server/src/server/processor.rs`
   (it was the only change in that file).

#### Observed Result

Contexts shortened to `...` for readability:

```text
published event: UdpRequestAccepted { context: ..., kind: Scrape }
published event: UdpError { context: ..., kind: Some(Scrape), error: ConnectionCookie("cookie value is expired: ...") }
published event: UdpResponseSent { context: ..., kind: Error { opt_req_kind: None }, req_processing_time: 57.538µs }
UdpResponseSent.kind = Error { opt_req_kind: None }
test server::processor::tests::temporary_reproduction_parsed_scrape_error_response_event_kind ... ok
```

#### Conclusion

Bug reproduced. For the same parsed scrape request, `UdpRequestAccepted` and `UdpError` both
carry `Scrape`, but `UdpResponseSent` reports `Error { opt_req_kind: None }`. The documented
contract requires `Error { opt_req_kind: Some(Scrape) }`.

## Regression Tests Before the Fix (B2)

- Date and time (UTC): 2026-09-27 08:50
- Code state: the commit `docs(issues): start implementation of #2345` plus the two new tests in
  `packages/udp-server/src/server/processor.rs`; production code unchanged.

Added to the `processor.rs` tests module:

- `it_should_keep_the_request_kind_in_the_error_response_event_when_a_parsed_request_fails`: the
  maintained form of V2 (a scrape with `ConnectionId::new(0)` from a real loopback client).
- `it_should_publish_an_error_response_event_without_a_request_kind_when_the_payload_is_unparsable`:
  the valid unknown-kind case (a 3-byte payload). It guards the fix against over-correction and is
  expected to pass before and after the fix.
- `receive_response_sent_kind`: skips `UdpRequestAccepted` and `UdpError` until
  `UdpResponseSent`, bounded by one `tokio::time::Instant` deadline with `timeout_at`.

Command:

```text
cargo test -p torrust-tracker-udp-server --lib server::processor::tests
```

Observed result (red, as expected):

```text
test server::processor::tests::it_should_publish_an_error_response_event_without_a_request_kind_when_the_payload_is_unparsable ... ok
test server::processor::tests::it_should_publish_a_discard_event_when_a_client_uses_port_zero ... ok
test server::processor::tests::it_should_keep_the_request_kind_in_the_error_response_event_when_a_parsed_request_fails ... FAILED

assertion `left == right` failed: a scrape with an invalid connection ID was parsed, so its error-response event should keep the request kind
  left: Error { opt_req_kind: None }
 right: Error { opt_req_kind: Some(Scrape) }

test result: FAILED. 2 passed; 1 failed; 0 ignored; 0 measured; 170 filtered out; finished in 0.03s
```

### Prose-First Arrange-Act-Assert Review

Parsed-request test:

- Arrange: "A processor with a direct event receiver gets a scrape request, from a real loopback
  client, whose connection ID is invalid." The code states this: `scrape_request_from` names the
  request kind and `invalid_connection_id` names the causal value. The client exists only so the
  response can be delivered and the event published; `bind_loopback_client` names that.
- Act: "The processor processes the request." `processor.process_request(request)` is visible.
- Assert: "The response event reports an error that keeps the scrape kind." The expected value is
  written out independently, not derived through production code.

Unparsable-payload test: same structure; `unparsable_payload` names the causal state and the
expected value is `Error { opt_req_kind: None }`.

Decisions:

- The temporary prose was removed; the code expresses it. No comments were kept.
- The ID is invalid only because the shared fixture uses strict validation. The fixture was left
  unchanged, as the spec asks, to keep the #2283 rebase small; `invalid_connection_id` states the
  consequence in the test body.
- The deadline lives in `receive_response_sent_kind`, not in `receive_event`, because the helper
  loops over several receives and the existing helper restarts its timeout per receive.
- Both tests were written in one increment instead of reviewing the first before adding the
  second, a small deviation from the spec's Design and Ownership Review. Reviewing them together
  afterwards found the fixture and helpers coherent for both tests, so no change followed.

## Post-Fix Recheck

`TODO` during implementation: rerun V1 unchanged, and rerun V2's observation through the
maintained regression test (expected `Error { opt_req_kind: Some(Scrape) }`). Record the green
regression-test output here.

## Failures and Follow-up

None.
