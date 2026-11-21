---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2916-2912-si-4-tracker-core/ISSUE.md
last-updated-utc: "2026-11-21 13:43"
---

<!-- cspell:ignore fromhex getrandbits QQQII EXPKEY -->

# Manual Verification Evidence

## Purpose

Record real, human-oriented verification of the completed behavior. This is
evidence from commands or interactions actually performed against the artifact;
do not invent commands, output, logs, or results.

## Preserving Verification Artifacts

The two tracker configurations, the request script and the UDP client lived in an agent scratchpad
outside the repository (`<scratch>` below); they are embedded verbatim in this document. The
captured replies and the test runs are stored in the [`evidence/`](evidence/) folder of this issue:

- [`evidence/responses-before.txt`](evidence/responses-before.txt) and
  [`evidence/responses-after.txt`](evidence/responses-after.txt): the eleven requests and their
  replies, before and after the fix.
- Red runs, at the commit `test(tracker-core): [#2916] add red tests that tracker-core error texts name no source path`:
  [`evidence/tests-red-tracker-core.txt`](evidence/tests-red-tracker-core.txt),
  [`evidence/tests-red-udp-server-lib.txt`](evidence/tests-red-udp-server-lib.txt),
  [`evidence/tests-red-udp-server-integration.txt`](evidence/tests-red-udp-server-integration.txt),
  [`evidence/tests-red-axum-http-server.txt`](evidence/tests-red-axum-http-server.txt) and
  [`evidence/tests-red-axum-rest-api-server.txt`](evidence/tests-red-axum-rest-api-server.txt).
- Green run, at the commit `docs(axum-http-server): [#2916] show the tracker-core authentication failure reasons without source paths`:
  [`evidence/tests-green.txt`](evidence/tests-green.txt).

In the stored output the worktree path is replaced with `<worktree>`, the Cargo home directory with
`<cargo-home>`, the REST API token with `<token>`, and thread ids are removed; the rest of each line
is verbatim.

## Environment and Prerequisites

- Date and time (UTC): 2026-11-21 13:26 to 13:42.
- Artifact under test: the `torrust-tracker` binary (`cargo build --bin torrust-tracker`, debug
  profile). "Before": built at 13:26 from the unchanged tree of `develop` at `4be22ee66`, before any
  edit on the branch. "After": built at 13:41 from the commit
  `fix(tracker-core): [#2916] replace LocatedError sources of peer key errors and drop torrust-located-error`
  (`git diff --quiet HEAD` exited 0 before the build). The only later commit,
  `docs(axum-http-server): ...`, changes module documentation only.
- Toolchain: stable Rust, `rustc 1.100.0 (0ef1bc386 2026-11-09)`; formatting with the nightly Rust
  toolchain, `cargo 1.102.0-nightly (f93c676e3 2026-11-15)`. Clients: `curl 8.18.0` and
  `Python 3.14.4` (the UDP client below).
- Operating system / environment: Linux 7.0.0-43-generic.
- Prerequisites and setup performed: two tracker instances started from the binary under test, one
  listed (UDP, HTTP and REST API) and one private (HTTP and REST API), each with a fresh SQLite
  database. The REST API token is the default development token of
  `share/default/config/tracker.development.sqlite3.toml`.

`listed.toml`:

```toml
[metadata]
app = "torrust-tracker"
purpose = "configuration"
schema_version = "3.0.0"

[logging]
trace_filter = "info"

[core]
listed = true
private = false

[core.database]
driver = "sqlite3"
path = "<scratch>/listed.db"

[[udp_trackers]]
bind_address = "127.0.0.1:17069"

[[http_trackers]]
bind_address = "127.0.0.1:17070"

[http_api]
bind_address = "127.0.0.1:17212"

[http_api.access_tokens]
admin = "MyAccessToken"

[health_check_api]
bind_address = "127.0.0.1:17313"
```

`private.toml`:

```toml
[metadata]
app = "torrust-tracker"
purpose = "configuration"
schema_version = "3.0.0"

[logging]
trace_filter = "info"

[core]
listed = false
private = true

[core.database]
driver = "sqlite3"
path = "<scratch>/private.db"

[[http_trackers]]
bind_address = "127.0.0.1:17080"

[http_api]
bind_address = "127.0.0.1:17213"

[http_api.access_tokens]
admin = "MyAccessToken"

[health_check_api]
bind_address = "127.0.0.1:17314"
```

`udp_announce.py`, a minimal BEP 15 client (connect, then announce). No UDP client binary ships with
the workspace; the maintained check of the same reply is the UDP contract test listed under V1.

```python
# Usage: python3 -I udp_announce.py <port> <info-hash-hex>
# Sends a BEP 15 connect and announce; prints the announce reply (error message if action 3).
import random, socket, struct, sys
port, ih = int(sys.argv[1]), bytes.fromhex(sys.argv[2])
s = socket.socket(socket.AF_INET, socket.SOCK_DGRAM); s.settimeout(5)
addr = ("127.0.0.1", port)
tx = random.getrandbits(32)
s.sendto(struct.pack(">QII", 0x41727101980, 0, tx), addr)
action, rtx, conn = struct.unpack(">IIQ", s.recv(2048)[:16])
tx = random.getrandbits(32)
s.sendto(struct.pack(">QII20s20sQQQIIIiH", conn, 1, tx, ih, b"-qB00000000000000001", 0, 0, 0, 0, 0, 0, -1, 17548), addr)
data = s.recv(2048)
action, rtx = struct.unpack(">II", data[:8])
print(f"action={action} message={data[8:].decode(errors='replace')!r}" if action == 3 else f"action={action} len={len(data)}")
```

`run.sh`, run as `run.sh <tracker-binary> before` and `run.sh <tracker-binary> after`. It starts
both trackers, waits for the four HTTP ports, sends the requests one after the other (each command
waits for its reply before the next starts, so W1 precedes W2, K1 precedes K2 and K4 precedes H4),
and stops the trackers with `SIGINT`:

```bash
#!/usr/bin/env bash
# Usage: run.sh <tracker-binary> <label>
set -u
DIR="$(cd "$(dirname "$0")" && pwd)"
BIN="$1"; LABEL="$2"
OUT="$DIR/$LABEL.txt"
IH_HEX='3b245504cf5f11bbdbe1201cea6a6bf45aee1bc0'
IH2_HEX='9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d'
IH='%3B%24U%04%CF%5F%11%BB%DB%E1%20%1C%EAjk%F4Z%EE%1B%C0'
VALID="info_hash=$IH&peer_id=-qB00000000000000001&port=17548"
T='token=MyAccessToken'
KEY='YZSl4lMZupRuOpSRC3krIKR5BPB14nrJ'
rm -f "$DIR/listed.db" "$DIR/private.db"
"$BIN" --config-toml-path "$DIR/listed.toml" > "$DIR/$LABEL.listed.log" 2>&1 & P1=$!
"$BIN" --config-toml-path "$DIR/private.toml" > "$DIR/$LABEL.private.log" 2>&1 & P2=$!
for port in 17070 17080 17212 17213; do
  for _ in $(seq 1 100); do curl -s -o /dev/null "http://127.0.0.1:$port/" && break; sleep 0.1; done
done
req() { # name method url [curl args...]
  local name="$1" method="$2" url="$3"; shift 3
  printf '%s\n  %s %s\n  ' "$name" "$method" "${url#http://127.0.0.1}" | sed "s/$T/token=<token>/"
  curl -s -X "$method" "$@" "$url"; printf '\n'
}
{
  echo "date -u: $(date -u '+%Y-%m-%d %H:%M:%S')"
  printf 'U1 UDP announce, info-hash not whitelisted (listed tracker)\n  udp_announce.py 17069 %s\n  ' "$IH_HEX"
  python3 -I "$DIR/udp_announce.py" 17069 "$IH_HEX"
  req "H1 HTTP announce, info-hash not whitelisted (listed tracker)" GET "http://127.0.0.1:17070/announce?$VALID"
  req "W1 REST whitelist the info-hash" POST "http://127.0.0.1:17212/api/v1/whitelist/$IH_HEX?$T"
  req "W2 REST whitelist the same info-hash again" POST "http://127.0.0.1:17212/api/v1/whitelist/$IH_HEX?$T"
  req "W3 REST remove an info-hash that is not whitelisted" DELETE "http://127.0.0.1:17212/api/v1/whitelist/$IH2_HEX?$T"
  req "H2 HTTP announce without key (private tracker)" GET "http://127.0.0.1:17080/announce?$VALID"
  req "H3 HTTP announce with an unregistered key (private tracker)" GET "http://127.0.0.1:17080/announce/$KEY?$VALID"
  req "K1 REST add a key valid for 60 s" POST "http://127.0.0.1:17213/api/v1/keys?$T" -H 'Content-Type: application/json' -d "{\"key\":\"$KEY\",\"seconds_valid\":60}"
  req "K2 REST add the same key again" POST "http://127.0.0.1:17213/api/v1/keys?$T" -H 'Content-Type: application/json' -d "{\"key\":\"$KEY\",\"seconds_valid\":60}"
  req "K3 REST remove a key that does not exist" DELETE "http://127.0.0.1:17213/api/v1/key/ZZZZ4lMZupRuOpSRC3krIKR5BPB14nrJ?$T"
  printf 'K4 REST generate a key valid for 1 s\n  POST /api/v1/key/1?token=<token>\n  '
  EXP=$(curl -s -X POST "http://127.0.0.1:17213/api/v1/key/1?$T"); echo "$EXP"
  EXPKEY=$(printf '%s' "$EXP" | python3 -I -c 'import json,sys; print(json.load(sys.stdin)["key"])')
  sleep 2
  req "H4 HTTP announce with the expired key (private tracker)" GET "http://127.0.0.1:17080/announce/$EXPKEY?$VALID"
} > "$OUT"
kill -INT $P1 $P2; wait $P1 $P2 2>/dev/null
cat "$OUT"
```

## Analysis

Re-checked against the tree before the first change (T1).
`grep -rn "Location\|Located\|track_caller" packages/tracker-core/src` finds the 7 variants of the
spec's table, the 12 driver sites, the two `#[track_caller]` conversions in `databases/error.rs`,
the `From<sqlx::Error>` of `authentication/key/mod.rs` and the six `Located(err).into()` calls in
`authentication/handler.rs`; the line numbers match the spec's. Two more `LocatedError` sources not
named in the spec's table surfaced: `authentication::key::Error::KeyVerificationError` (built by that
`From<sqlx::Error>`) and the `QueryReturnedNoRows`, `InvalidQuery`, `ConnectionError` and
`MigrationError` sources of `databases::error::Error` (built by the two conversions); both are within
the spec's "Replace the `LocatedError` sources" item.

Test-only users outside the spec's file list build the changed variants: `http-core`
(`services/announce.rs`, `services/scrape.rs`, `services/error_mapping.rs`), `udp-server/src/event.rs`
and `tracker-core/src/statistics/event/handler.rs` (`UpdateFailed`). They change with the variants.

Hypothesis: each format interpolates a `location` filled with `Location::caller()` in a function that
is not `#[track_caller]`, so it prints a fixed library line, and each `LocatedError` source prints its
own location after the wrapped text, so an error that passes two wrappers prints two paths (K2).

## Verification Processes

### V1 - UDP whitelist rejection (scenario M1)

- Goal: the UDP error response to an announce for a torrent that is not whitelisted names the
  info-hash and holds no source path.
- Initial state: the listed tracker above, empty whitelist.
- Status: `DONE`

#### Steps Performed

1. Before the fix, 13:26:47 UTC: `run.sh <before-binary> before`, request U1.
2. After the fix, 13:41:15 UTC: `run.sh <after-binary> after`, request U1.

#### Observed Result

Before (from `evidence/responses-before.txt`):

```text
action=3 message='tracker announce error: Tracker core whitelist error: The torrent: 3b245504cf5f11bbdbe1201cea6a6bf45aee1bc0, is not whitelisted, packages/tracker-core/src/whitelist/authorization.rs:61:23'
```

After (from `evidence/responses-after.txt`):

```text
action=3 message='tracker announce error: Tracker core whitelist error: The torrent: 3b245504cf5f11bbdbe1201cea6a6bf45aee1bc0, is not whitelisted'
```

Maintained tests: `udp-server` `server::contract::receiving_an_announce_request::should_reject_a_torrent_that_is_not_whitelisted_with_a_message_that_holds_no_source_path`
(through a running UDP tracker) and the four `event::tests::it_should_classify_*whitelist_error`
tests, which pin the `ErrorKind::Whitelist` payload to the full message (AC4).

#### Conclusion

Met: the message names the info-hash and holds no path.

### V2 - HTTP whitelist and key errors (scenario M2)

- Goal: the `failure reason` for a torrent that is not whitelisted, a missing key, an unregistered
  key and an expired key holds no source path.
- Initial state: the listed tracker (H1) and the private tracker (H2 to H4); for H4, a key valid for
  one second generated through the REST API (K4) two seconds before the announce.
- Status: `DONE`

#### Steps Performed

1. Before the fix, 13:26:47 UTC: requests H1 to H4 of `run.sh <before-binary> before`.
2. After the fix, 13:41:15 UTC: the same requests of `run.sh <after-binary> after`.

#### Observed Result

Before (bencoding framing removed):

```text
H1 Tracker whitelist error: The torrent: 3b245504cf5f11bbdbe1201cea6a6bf45aee1bc0, is not whitelisted, packages/tracker-core/src/whitelist/authorization.rs:61:23
H2 Tracker authentication error: Missing authentication key, packages/http-core/src/services/announce.rs:308:27
H3 Tracker authentication error: Failed to read key: YZSl4lMZupRuOpSRC3krIKR5BPB14nrJ, packages/tracker-core/src/authentication/service.rs:98:27
H4 Tracker authentication error: Key has expired, packages/tracker-core/src/authentication/key/mod.rs:158:31
```

After:

```text
H1 Tracker whitelist error: The torrent: 3b245504cf5f11bbdbe1201cea6a6bf45aee1bc0, is not whitelisted
H2 Tracker authentication error: Missing authentication key
H3 Tracker authentication error: Failed to read key: YZSl4lMZupRuOpSRC3krIKR5BPB14nrJ
H4 Tracker authentication error: Key has expired
```

Maintained tests: `axum-http-server` `failure_reasons_hold_no_source_path::an_announce_to_a_listed_tracker_for_a_torrent_that_is_not_whitelisted`,
`an_announce_to_a_private_tracker_without_a_key` and `an_announce_to_a_private_tracker_with_an_unregistered_key`;
the expired key is covered at the error's seam by `tracker-core`
`it_should_reject_an_expired_key_with_a_message_that_holds_no_source_path` (an HTTP test would need
a key that expires during the test, which the integration environment cannot set up without a sleep).

#### Conclusion

Met: the four failure reasons keep their text and lose the path.

### V3 - REST database failures (scenario M3)

- Goal: a REST API error built from a failed database write names the operation and holds no path,
  including no driver file.
- Initial state: the two trackers above. W1 whitelists an info-hash, K1 adds a key.
- Status: `DONE`

#### Steps Performed

1. Before the fix, 13:26:47 UTC: requests W1 to W3 and K1 to K3 of `run.sh <before-binary> before`.
2. After the fix, 13:41:15 UTC: the same requests of `run.sh <after-binary> after`.

#### Observed Result

W2 (the same info-hash again) and W3 (removing an info-hash that is not whitelisted) answer
`{"status":"ok"}` before and after: the persisted whitelist repository asks the database whether
the info-hash is whitelisted before it inserts or deletes
(`tracker-core/src/whitelist/repository/persisted.rs`, `add` and `remove`), so neither request
reaches a failing write. The key requests do.

Before:

```text
K2 Unhandled rejection: Err { reason: "failed to add key: The Sqlite3 query was malformed: error returned from database: (code: 2067) UNIQUE constraint failed: keys.key, packages/tracker-core/src/databases/driver/sqlite/auth_key_store.rs:74:22, packages/tracker-core/src/authentication/handler.rs:103:50" }
K3 Unhandled rejection: Err { reason: "failed to delete key: Failed to remove record from Sqlite3 database, error-code: 0, packages/tracker-core/src/databases/driver/sqlite/auth_key_store.rs:108:27" }
```

After:

```text
K2 Unhandled rejection: Err { reason: "failed to add key: The Sqlite3 query was malformed: error returned from database: (code: 2067) UNIQUE constraint failed: keys.key" }
K3 Unhandled rejection: Err { reason: "failed to delete key: Failed to remove record from Sqlite3 database, error-code: 0" }
```

The developer diagnostic that replaces the driver line, from the "after" log of the private tracker
(colors removed, token replaced):

```text
2026-11-21T13:41:16.029876Z  WARN request{method=DELETE uri=/api/v1/key/ZZZZ4lMZupRuOpSRC3krIKR5BPB14nrJ?token=<token> version=HTTP/1.1}: torrust_tracker_core::databases::driver::sqlite::auth_key_store: the delete did not remove exactly one row driver=Sqlite3 table="keys" rows_deleted=0
```

Maintained tests: `axum-rest-api-server` `auth_key::should_fail_without_a_source_path_when_the_same_auth_key_is_uploaded_twice`
and `should_fail_without_a_source_path_when_the_auth_key_to_delete_does_not_exist`; and, because
`assert_unhandled_rejection` now forbids source paths, the seven existing database-failure tests of the
key and whitelist contexts (they force the failure by dropping the tables). The red run shows all nine
failing with a path.

#### Conclusion

Met: the replies name the failed operation and the database's own text, with no path.

### Red and green runs (T2, T7)

Red, at the red-test commit (`git diff --quiet HEAD` exited 0 before the runs): 6 of 160
`tracker-core` lib tests fail, 4 `udp-server` lib tests, the new `udp-server` integration test, 3 of
14 selected `axum-http-server` integration tests and 9 of 60 `axum-rest-api-server` integration
tests, each with a `packages/...rs:line` path in the message. The shared SQLite driver test
`databases::driver::sqlite::tests::run_sqlite_driver_tests` runs its steps in sequence and stops at
the first failing one (the duplicate key), so its other three new steps were not reached in the red
run. Cargo stops at the first failing test target, so the `udp-server` integration target was run
separately.

Green, at the final code commit: the same five commands, all passing (165, 45 selected by
`event::`, 17, 14 selected, 60).

## Failures and Follow-up

None.
