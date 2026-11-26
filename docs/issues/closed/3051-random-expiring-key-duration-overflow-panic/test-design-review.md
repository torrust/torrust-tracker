---
doc-type: test-design-review
issue-spec: docs/issues/closed/3051-random-expiring-key-duration-overflow-panic/ISSUE.md
last-updated-utc: "2026-11-26 13:50"
---

# Test design review: random expiring key duration overflow

Prose-first Arrange-Act-Assert comparison of each regression test of the spec, written before maintainer review and
commit, as the `write-unit-test` skill asks.

## `generate_key` (unit, `packages/tracker-core/src/authentication/key/mod.rs`)

- Test: `it_should_return_an_error_with_the_requested_seconds_instead_of_panicking`.
- Arrange: nothing. The lifetime is `u64::MAX` seconds, which no clock can add.
- Act: call `generate_key(Some(lifetime))`.
- Assert: the result is `Err(DurationOverflow { seconds_valid: u64::MAX })`. The test reads as the behaviour the bug
  broke: an overflow is an error value, not a panic.
- Second test: `it_should_state_the_requested_seconds_in_the_error_message` pins the message, the same wording as
  `PeerKeyError::DurationOverflow`, so the REST reply does not change.
- Proof: replacing the `ok_or(..)?` with the old `unwrap()` makes the first test fail with the panic; restoring the
  code makes it pass (run recorded in the evidence file, V2c).

## `KeysHandler::add_peer_key` (unit, `packages/tracker-core/src/authentication/handler.rs`)

- Test: `it_should_fail_adding_a_randomly_generated_key_when_the_key_duration_exceeds_the_maximum_duration`, in
  `handling_expiring_peer_keys::randomly_generated`.
- Arrange: a private tracker configuration and a real `KeysHandler`, as the nearby tests do.
- Act: `add_peer_key` with no key and `u64::MAX` seconds.
- Assert: the error is `PeerKeyError::DurationOverflow`. It names the variant only, because the message is covered
  elsewhere.

## `generate_auth_key_handler` (unit, `packages/axum-rest-api-server/src/v1/context/auth_key/handlers.rs`)

- Test: `it_should_answer_bad_request_when_the_duration_of_the_generated_key_overflows`.
- Arrange: the fake port answers `AuthKeyError::DurationOverflow`.
- Act: call the handler with the segment `u64::MAX`.
- Assert: status `400` and the same body as the `add` endpoint, so both endpoints answer an overflow alike.
- The older test `it_should_answer_an_internal_server_error_..._for_any_other_error` is renamed: it covers only the
  database and invalid-key errors, so its old name ("whatever the error") was no longer true.

## REST contract (`packages/axum-rest-api-server/tests/server/v1/contract/context/auth_key.rs`)

- Tests: `should_fail_adding_a_random_auth_key_when_the_key_duration_overflows` (`POST /api/v1/keys`) and
  `should_fail_generating_an_auth_key_when_the_key_duration_overflows` (`POST /api/v1/key/{seconds}`).
- Arrange: an ephemeral private tracker.
- Act: send the overflowing duration to a real server.
- Assert: `400` with the stated body. Before the fix the request handler panicked and the client got no response.

## Duplication and naming

The two contract tests and the handler test repeat one expected body on purpose: each layer proves its own contract.
No helper hides the value, so a failing test shows the expected text.
