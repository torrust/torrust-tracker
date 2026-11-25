---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/3051-random-expiring-key-duration-overflow-panic/ISSUE.md
last-updated-utc: "2026-11-25 19:14"
---

# Manual Verification Evidence

## Purpose

Evidence of the bug before the fix (V1, V2) and, after the fix, the like-for-like recheck (V3).

## Environment and Prerequisites

- Date and time (UTC): 2026-11-25, 14:12 to 14:15
- Artifact under test: `torrust/develop` at `11c54fb48` (Merge #3047), unchanged, plus the temporary test in V2
- Operating system: Linux 7.0.0-44-generic
- Toolchain: stable Rust, `rustc 1.100.0 (0ef1bc386 2026-11-09)`, `cargo 1.100.0`
- Setup: `cargo build --bin torrust-tracker`; configuration
  [`evidence/tracker.repro.toml`](evidence/tracker.repro.toml) (private tracker, SQLite database in `./.tmp/`,
  REST API on `127.0.0.1:18212`, admin token `ReproToken`, a throw-away value)

## Verification Processes

### V1 - Real tracker, before the fix

- Goal: observe what a client gets for an overflowing duration on each path.
- Initial state: fresh SQLite database; the tracker started in the background, as run, with
  `./target/debug/torrust-tracker --config-toml-path docs/issues/drafts/random-expiring-key-duration-overflow-panic/evidence/tracker.repro.toml > .tmp/tracker.log 2>&1`,
  so `.tmp/tracker.log` holds its output (stdout and stderr). The folder has since moved to
  `docs/issues/open/3051-random-expiring-key-duration-overflow-panic/`; the configuration file is the one in its
  `evidence/` folder.
- Status: `DONE` (outcome: **Reproduced**)

#### Steps Performed

Run one at a time, in this order:

1. 14:14:31 control, a valid random key (the only request called the control in this file):
   `curl -sS -i -X POST 'http://127.0.0.1:18212/api/v1/keys?token=ReproToken' -H 'content-type: application/json' -d '{"key":null,"seconds_valid":3600}'`
2. 14:14:34 random key, overflowing duration:
   `curl -sS -i -X POST 'http://127.0.0.1:18212/api/v1/keys?token=ReproToken' -H 'content-type: application/json' -d '{"key":null,"seconds_valid":18446744073709551615}'`
3. 14:14:34 pre-generated key, overflowing duration (the comparison that already returns `400`):
   `curl -sS -i -X POST 'http://127.0.0.1:18212/api/v1/keys?token=ReproToken' -H 'content-type: application/json' -d '{"key":"YZSl4lMZupRuOpSRC3krIKR5BPB14nrJ","seconds_valid":18446744073709551615}'`
4. 14:14:37 generate-key endpoint:
   `curl -sS -i -X POST 'http://127.0.0.1:18212/api/v1/key/18446744073709551615?token=ReproToken'`
5. Not timestamped: it ran in the same terminal command as request 4, which printed only its own start time
   (14:14:37), so the end of request 4 and the start of request 5 were not recorded. V3 stamps both ends.
   `curl -sS -o /dev/null -w '%{http_code}' 'http://127.0.0.1:18212/api/v1/stats?token=ReproToken'`

#### Observed Result

```text
1. HTTP/1.1 200 OK
   {"key":"<32-character random key, redacted>","valid_until":1795619671,"expiry_time":"2026-11-25 15:14:31 UTC"}
2. curl: (52) Empty reply from server
3. HTTP/1.1 400 Bad Request
   Invalid URL: invalid auth key duration: "18446744073709551615"
4. curl: (52) Empty reply from server
5. 200
```

The tracker log has one panic per failing request. [`evidence/tracker-panic-before-fix.log`](evidence/tracker-panic-before-fix.log)
is an extract made with
`sed 's/\x1b\[[0-9;]*m//g' .tmp/tracker.log | grep -A1 "panicked" | cut -c1-250` (ANSI colors removed; the two matches are
separated by `--`). One of them:

```text
thread 'tokio-rt-worker' (4055249) panicked at packages/tracker-core/src/authentication/key/mod.rs:111:79:
called `Option::unwrap()` on a `None` value
```

#### Conclusion

The wrong outcome was observed: both random-key paths drop the connection and panic at the `unwrap()`, while the
pre-generated path returns the expected `400`. The tracker keeps serving (step 5).

### V2 - Unit seam, before the fix

- Goal: reproduce at `KeysHandler::add_peer_key` with a deterministic test.
- Status: `DONE`

The temporary test, added to the `handling_expiring_peer_keys::randomly_generated` module (not the
`handling_permanent_peer_keys` one) in `packages/tracker-core/src/authentication/handler.rs`
and reverted afterwards (it becomes the maintained regression test, T1):

```rust
#[tokio::test]
async fn it_should_fail_adding_a_randomly_generated_key_when_the_key_duration_exceeds_the_maximum_duration() {
    let ephemeral = private_tracker_configuration();
    let keys_handler = instantiate_keys_handler(&ephemeral.config).await;

    let result = keys_handler
        .add_peer_key(AddKeyRequest {
            opt_key: None,
            opt_seconds_valid: Some(u64::MAX),
        })
        .await;

    assert!(matches!(result.unwrap_err(), PeerKeyError::DurationOverflow { .. }));
}
```

Command (14:13:31, stable toolchain):
`cargo test -p torrust-tracker-core --lib randomly_generated_key_when_the_key_duration`

```text
test ...randomly_generated::it_should_fail_adding_a_randomly_generated_key_when_the_key_duration_exceeds_the_maximum_duration ... FAILED
thread '...' panicked at packages/tracker-core/src/authentication/key/mod.rs:111:79:
called `Option::unwrap()` on a `None` value
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 302 filtered out
```

#### Conclusion

The same panic at the same line, reproduced deterministically.

### V2b - Maintained regression tests, red before the fix

- Goal: show that each regression test of the spec fails for the bug, before any production change.
- Status: `DONE`

Run on the branch `3051-random-expiring-key-duration-overflow-panic` with only the tests added (no production
change), stable toolchain, from 19:06:33 to 19:06:34 UTC on 2026-11-25. Four tests of the spec's table fail
(the `generate_key` unit test is added with the fix, T3, and proven by mutate-then-restore).

| Test                                                                                                    | Command (`cargo test -q -p ...`)                                                | Red result                                                                                       |
| ------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------ |
| `KeysHandler`: `it_should_fail_adding_a_randomly_generated_key_when_the_key_duration_exceeds_the_maximum_duration` | `torrust-tracker-core --lib duration_exceeds_the_maximum`                      | panics at `packages/tracker-core/src/authentication/key/mod.rs:111:79`                           |
| `generate_auth_key_handler`: `it_should_answer_bad_request_when_the_duration_of_the_generated_key_overflows` | `torrust-tracker-axum-rest-api-server --lib duration_of_the_generated_key_overflows` | answers `500` `Unhandled rejection ... failed to generate key: duration overflow: 18446744073709551615`, expected `400` |
| REST contract (`POST /api/v1/keys`): `should_fail_adding_a_random_auth_key_when_the_key_duration_overflows` | `torrust-tracker-axum-rest-api-server --test integration overflows`            | the request handler panics at `key/mod.rs:111:79`, the client gets no response                   |
| REST contract (`POST /api/v1/key/{seconds}`): `should_fail_generating_an_auth_key_when_the_key_duration_overflows` | same command                                                                    | same panic at `key/mod.rs:111:79`                                                                |

#### Conclusion

All four fail for the reported bug: three on the panic, and the handler test on the missing `400` arm. The
`it_should_fail_adding_a_randomly_generated_key...` filter also matched an unrelated passing test (`1 passed`).

### V2c - Green run and mutation proof after the fix

- Goal: show the four regression tests of V2b pass with the fix, and that the new `generate_key` unit test fails for the
  old behaviour.
- Status: `DONE`

Green, from 19:08:57 UTC on 2026-11-25 (stable toolchain): `cargo test -q -p torrust-tracker-core` (305 passed),
`cargo test -q -p torrust-tracker-axum-rest-api-server` (170 lib, 63 integration passed), and the tests of
`http-core`, `axum-http-server`, `rest-api-runtime-adapter` and `persistence-benchmark`. `cargo test --workspace
--doc` passes, including the new doctest on `generate_key`. `linter all` exits `0`.

Mutation proof for `when_the_lifetime_of_the_key_overflows_the_clock`: with the `ok_or(..)?` in `generate_key`
replaced by the old `unwrap()` the test fails (`1 passed; 1 failed`, panic in `key/mod.rs`); with the fix restored it
passes (`2 passed`).

### V3 - Like-for-like recheck after the fix

- Goal: repeat V1 steps 1 to 5 against the fixed build and compare.
- Initial state: fresh SQLite database (`.tmp/repro-keys.db` removed); artifact under test: the branch
  `3051-random-expiring-key-duration-overflow-panic` at `0e94a2cfd`
  (`fix(tracker-core): [#3051] return an error when a random key duration overflows the clock`), the last production
  change; stable toolchain; `cargo build --bin torrust-tracker`; same configuration and requests as V1. The tracker
  was started in the background with
  `./target/debug/torrust-tracker --config-toml-path docs/issues/open/3051-random-expiring-key-duration-overflow-panic/evidence/tracker.repro.toml > .tmp/tracker.log 2>&1`
  and answered `200` to the stats request at 19:13:41 UTC on 2026-11-25.
- Status: `DONE` (outcome: **Fixed**)

#### Steps Performed

Run one at a time, in this order, all on 2026-11-25 (UTC); every request started and ended in the same second:

1. 19:13:51 to 19:13:51 control, a valid random key.
2. 19:13:51 to 19:13:51 random key, overflowing duration (`seconds_valid: 18446744073709551615`).
3. 19:13:51 to 19:13:51 pre-generated key, overflowing duration.
4. 19:13:51 to 19:13:51 generate-key endpoint (`POST /api/v1/key/18446744073709551615`).
5. 19:13:51 to 19:13:51 stats request, to see that the tracker still serves.

The commands are those of V1; each ran as one command that printed the time before and after it.

#### Observed Result

```text
1. HTTP/1.1 200 OK
   {"key":"<32-character random key, redacted>","valid_until":1795637631,"expiry_time":"2026-11-25 20:13:51 UTC"}
2. HTTP/1.1 400 Bad Request
   Invalid URL: invalid auth key duration: "18446744073709551615"
3. HTTP/1.1 400 Bad Request
   Invalid URL: invalid auth key duration: "18446744073709551615"
4. HTTP/1.1 400 Bad Request
   Invalid URL: invalid auth key duration: "18446744073709551615"
5. 200
```

The tracker log holds no `panicked` line (`sed 's/\x1b\[[0-9;]*m//g' .tmp/tracker.log | grep -c panicked` printed `0`).

#### Conclusion

Both random-key paths now answer `400`, with the same body as the pre-generated path (request 3, unchanged), instead of
dropping the connection. The control still answers `200`, and the tracker logs no panic. The bug is fixed.

## Failures and Follow-up

None. The first tracker start failed on a missing `core.listed` option in the configuration; the option was added
and the tracker restarted before step 1.
