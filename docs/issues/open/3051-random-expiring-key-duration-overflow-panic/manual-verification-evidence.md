---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/3051-random-expiring-key-duration-overflow-panic/ISSUE.md
last-updated-utc: "2026-11-25 14:18"
---

# Manual Verification Evidence

## Purpose

Evidence of the bug before the fix (V1, V2) and, after the fix, the like-for-like recheck (V3, `TODO`).

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
- Initial state: fresh SQLite database; the tracker started with
  `./target/debug/torrust-tracker --config-toml-path docs/issues/open/3051-random-expiring-key-duration-overflow-panic/evidence/tracker.repro.toml`.
- Status: `DONE` (outcome: **Reproduced**)

#### Steps Performed

Run one at a time, in this order:

1. 14:14:31 control, a valid random key:
   `curl -sS -i -X POST 'http://127.0.0.1:18212/api/v1/keys?token=ReproToken' -H 'content-type: application/json' -d '{"key":null,"seconds_valid":3600}'`
2. 14:14:34 random key, overflowing duration:
   `curl -sS -i -X POST 'http://127.0.0.1:18212/api/v1/keys?token=ReproToken' -H 'content-type: application/json' -d '{"key":null,"seconds_valid":18446744073709551615}'`
3. 14:14:34 pre-generated key, overflowing duration:
   `curl -sS -i -X POST 'http://127.0.0.1:18212/api/v1/keys?token=ReproToken' -H 'content-type: application/json' -d '{"key":"YZSl4lMZupRuOpSRC3krIKR5BPB14nrJ","seconds_valid":18446744073709551615}'`
4. 14:14:37 generate-key endpoint:
   `curl -sS -i -X POST 'http://127.0.0.1:18212/api/v1/key/18446744073709551615?token=ReproToken'`
5. `curl -sS -o /dev/null -w '%{http_code}' 'http://127.0.0.1:18212/api/v1/stats?token=ReproToken'`

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

The tracker log has one panic per failing request ([`evidence/tracker-panic-before-fix.log`](evidence/tracker-panic-before-fix.log)):

```text
thread 'tokio-rt-worker' panicked at packages/tracker-core/src/authentication/key/mod.rs:111:79:
called `Option::unwrap()` on a `None` value
```

#### Conclusion

The wrong outcome was observed: both random-key paths drop the connection and panic at the `unwrap()`, while the
pre-generated path returns the expected `400`. The tracker keeps serving (step 5).

### V2 - Unit seam, before the fix

- Goal: reproduce at `KeysHandler::add_peer_key` with a deterministic test.
- Status: `DONE`

The temporary test, added to the `randomly_generated` module in `packages/tracker-core/src/authentication/handler.rs`
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

### V3 - Like-for-like recheck after the fix

- Status: `TODO`. Repeat V1 steps 1 to 5 against the fixed build, after the last production change; name the commit
  by its Conventional Commit subject.

## Failures and Follow-up

None. The first tracker start failed on a missing `core.listed` option in the configuration; the option was added
and the tracker restarted before step 1.
