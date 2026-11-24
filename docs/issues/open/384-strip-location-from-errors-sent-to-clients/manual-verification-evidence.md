---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/384-strip-location-from-errors-sent-to-clients/ISSUE.md
last-updated-utc: "2026-11-24 14:37"
---

<!-- cspell:ignore getrandbits QQQII EXPKEY fromhex whitelistede hashe numwante bytese characterse keye expirede -->

# Manual Verification Evidence

## Purpose

Real verification on running trackers that no reply to an HTTP, UDP or REST client holds a source
path: a baseline on `develop` before the removal and the same requests on `develop` after SI-1 to SI-6.
Nothing here is simulated; the replies are the captured output.

## Preserving Verification Artifacts

The configurations and scripts lived in an agent scratchpad (`<scratch>` below) and are embedded
verbatim in this document. The captured output is in [`evidence/`](evidence/):

- [`evidence/responses-before.txt`](evidence/responses-before.txt) and [`evidence/responses-after.txt`](evidence/responses-after.txt): the 17 requests of `run.sh`, before and after.
- [`evidence/responses-extra-before.txt`](evidence/responses-extra-before.txt) and [`evidence/responses-extra-after.txt`](evidence/responses-extra-after.txt): seven more error requests (`extra.sh`), before and after.
- [`evidence/tests-green.txt`](evidence/tests-green.txt): `cargo test --workspace --no-fail-fast source_path` on the commit under test (T4).

In the stored output the home directory is replaced with `<home>` and the scratch directory with `<scratch>`; the rest is verbatim.

## Environment and Prerequisites

- Date and time (UTC): 2026-11-24 13:28 to 13:30.
- Artifacts under test: the `torrust-tracker` binary, debug profile (`cargo build --bin torrust-tracker`).
  "Before": built from a detached worktree at `305027e70` (`develop` after the ADR of SI-1 and before
  any removal, SI-2 merged next as `f6d2f4df8`). "After": built from `c94bda536` (`develop` with SI-1 to
  SI-6 and the SI-6 close-out merged; the worktree was clean).
- Toolchain: stable Rust, `rustc 1.100.0 (0ef1bc386 2026-11-09)`. Clients: `curl 8.18.0` and `Python 3.14.4`.
- Operating system: Linux 7.0.0-44-generic.
- Setup: two trackers per run, a listed one (UDP, HTTP, REST API) and a private one (HTTP, REST API), each with a fresh SQLite database. The REST API token is the development token `MyAccessToken`.

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

`udp_announce.py`, a minimal BEP 15 client (no UDP client ships with the workspace):

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

`run.sh`:

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
  req "H5 HTTP announce without info_hash (listed tracker)" GET "http://127.0.0.1:17070/announce?peer_id=-qB00000000000000001&port=17548"
  req "H6 HTTP announce with numwant=x (listed tracker)" GET "http://127.0.0.1:17070/announce?$VALID&numwant=x"
  req "H7 HTTP announce with a short info_hash (listed tracker)" GET "http://127.0.0.1:17070/announce?info_hash=abc&peer_id=-qB00000000000000001&port=17548"
  req "H8 HTTP announce with a malformed key (private tracker)" GET "http://127.0.0.1:17080/announce/not-a-key?$VALID"
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

`extra.sh`:

```bash
#!/usr/bin/env bash
# Usage: extra.sh <tracker-binary>  -- further REST and HTTP error replies
S="$(cd "$(dirname "$0")" && pwd)"; BIN="$1"; T='token=MyAccessToken'
"$BIN" --config-toml-path "$S/listed.toml" >/dev/null 2>&1 & P1=$!
for _ in $(seq 1 100); do curl -s -o /dev/null http://127.0.0.1:17212/ && break; sleep 0.1; done
req() { n="$1"; m="$2"; u="$3"; shift 3; printf '%s\n  %s %s\n  ' "$n" "$m" "${u#http://127.0.0.1}" | sed "s/$T/token=<token>/"; curl -s -i -X "$m" "$@" "$u" | tr -d '\r' | sed -n '1p;/^$/,$p' | tr '\n' ' '; printf '\n'; }
req "R1 whitelist an invalid info-hash" POST "http://127.0.0.1:17212/api/v1/whitelist/zzz?$T"
req "R2 no token" POST "http://127.0.0.1:17212/api/v1/whitelist/3b245504cf5f11bbdbe1201cea6a6bf45aee1bc0"
req "R3 wrong token" POST "http://127.0.0.1:17212/api/v1/whitelist/3b245504cf5f11bbdbe1201cea6a6bf45aee1bc0?token=bad"
req "R4 key with invalid duration" POST "http://127.0.0.1:17212/api/v1/key/abc?$T"
req "R5 torrent info for an invalid hash" GET "http://127.0.0.1:17212/api/v1/torrent/zzz?$T"
req "R6 scrape with an invalid info_hash (HTTP)" GET "http://127.0.0.1:17070/scrape?info_hash=abc"
req "R7 announce with an invalid port (HTTP)" GET "http://127.0.0.1:17070/announce?info_hash=%3B%24U%04%CF%5F%11%BB%DB%E1%20%1C%EAjk%F4Z%EE%1B%C0&peer_id=-qB00000000000000001&port=x"
kill -INT $P1; wait $P1 2>/dev/null
```

## Analysis

Re-checked against the tree (fix-bug analysis step). `grep -rnE 'Location::caller|track_caller|torrust_located_error' --include=*.rs packages src console contrib tests` on the commit under test finds only test or check code, and the `source-location-check` of SI-6 enforces it. The audit's T2 surfaces (HTTP `failure reason`, UDP error message, REST error text) are all exercised below.

Hypothesis: every reply that held a path before holds none after, and the text still names what failed.

## Verification Processes

### V1 - M1 - HTTP failure reason, as in the report (H1, H5, H6, H7)

- Goal: no reply holds a source path and each still names what failed.
- Requests: H5 (missing `info_hash`), H6 (`numwant=x`) and H7 (short `info_hash`), plus H1 (not whitelisted).
- Status: `DONE`

#### Observed Result

Before (`305027e70`):

```text
H1 HTTP announce, info-hash not whitelisted (listed tracker)
  GET :17070/announce?info_hash=%3B%24U%04%CF%5F%11%BB%DB%E1%20%1C%EAjk%F4Z%EE%1B%C0&peer_id=-qB00000000000000001&port=17548
  d14:failure reason158:Tracker whitelist error: The torrent: 3b245504cf5f11bbdbe1201cea6a6bf45aee1bc0, is not whitelisted, packages/tracker-core/src/whitelist/authorization.rs:61:23e
H5 HTTP announce without info_hash (listed tracker)
  GET :17070/announce?peer_id=-qB00000000000000001&port=17548
  d14:failure reason145:Bad request. Cannot parse query params for announce request: missing param info_hash in packages/http-protocol/src/v1/requests/announce.rs:564:23e
H6 HTTP announce with numwant=x (listed tracker)
  GET :17070/announce?info_hash=%3B%24U%04%CF%5F%11%BB%DB%E1%20%1C%EAjk%F4Z%EE%1B%C0&peer_id=-qB00000000000000001&port=17548&numwant=x
  d14:failure reason155:Bad request. Cannot parse query params for announce request: invalid param value x for numwant in packages/http-protocol/src/v1/requests/announce.rs:654:27e
H7 HTTP announce with a short info_hash (listed tracker)
  GET :17070/announce?info_hash=abc&peer_id=-qB00000000000000001&port=17548
  d14:failure reason334:Bad request. Cannot parse query params for announce request: invalid param value abc for info_hash in not enough bytes for infohash: got 3 bytes, expected 20 <home>/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/torrust-info-hash-0.2.0/src/info_hash.rs:273:27, packages/http-protocol/src/v1/requests/announce.rs:559:42e
```

After (`c94bda536`):

```text
H1 HTTP announce, info-hash not whitelisted (listed tracker)
  GET :17070/announce?info_hash=%3B%24U%04%CF%5F%11%BB%DB%E1%20%1C%EAjk%F4Z%EE%1B%C0&peer_id=-qB00000000000000001&port=17548
  d14:failure reason98:Tracker whitelist error: The torrent: 3b245504cf5f11bbdbe1201cea6a6bf45aee1bc0, is not whitelistede
H5 HTTP announce without info_hash (listed tracker)
  GET :17070/announce?peer_id=-qB00000000000000001&port=17548
  d14:failure reason84:Bad request. Cannot parse query params for announce request: missing param info_hashe
H6 HTTP announce with numwant=x (listed tracker)
  GET :17070/announce?info_hash=%3B%24U%04%CF%5F%11%BB%DB%E1%20%1C%EAjk%F4Z%EE%1B%C0&peer_id=-qB00000000000000001&port=17548&numwant=x
  d14:failure reason94:Bad request. Cannot parse query params for announce request: invalid param value x for numwante
H7 HTTP announce with a short info_hash (listed tracker)
  GET :17070/announce?info_hash=abc&peer_id=-qB00000000000000001&port=17548
  d14:failure reason142:Bad request. Cannot parse query params for announce request: invalid param value abc for info_hash: it must percent-decode to exactly 20 bytese
```

#### Conclusion

Before: paths of `http-protocol`, `tracker-core` and, for H7, a `~/.cargo/registry` path of `torrust-info-hash`. After: none, and the text still names the parameter and the value.

### V2 - M2 - UDP error message (U1)

- Goal: no reply holds a source path and each still names what failed.
- Requests: U1, UDP announce for an info-hash that is not whitelisted.
- Status: `DONE`

#### Observed Result

Before (`305027e70`):

```text
U1 UDP announce, info-hash not whitelisted (listed tracker)
  udp_announce.py 17069 3b245504cf5f11bbdbe1201cea6a6bf45aee1bc0
  action=3 message='tracker announce error: Tracker core whitelist error: The torrent: 3b245504cf5f11bbdbe1201cea6a6bf45aee1bc0, is not whitelisted, packages/tracker-core/src/whitelist/authorization.rs:61:23'
```

After (`c94bda536`):

```text
U1 UDP announce, info-hash not whitelisted (listed tracker)
  udp_announce.py 17069 3b245504cf5f11bbdbe1201cea6a6bf45aee1bc0
  action=3 message='tracker announce error: Tracker core whitelist error: The torrent: 3b245504cf5f11bbdbe1201cea6a6bf45aee1bc0, is not whitelisted'
```

#### Conclusion

Before: `packages/tracker-core/src/whitelist/authorization.rs:61:23`. After: none; the info-hash is still named.

### V3 - M3 - REST API errors (K2, K3, W1-W3, R1-R5)

- Goal: no reply holds a source path and each still names what failed.
- Requests: K2, add the same key twice; K3, remove a missing key; and the extra REST errors.
- Status: `DONE`

#### Observed Result

Before (`305027e70`):

```text
W1 REST whitelist the info-hash
  POST :17212/api/v1/whitelist/3b245504cf5f11bbdbe1201cea6a6bf45aee1bc0?token=<token>
  {"status":"ok"}
W2 REST whitelist the same info-hash again
  POST :17212/api/v1/whitelist/3b245504cf5f11bbdbe1201cea6a6bf45aee1bc0?token=<token>
  {"status":"ok"}
W3 REST remove an info-hash that is not whitelisted
  DELETE :17212/api/v1/whitelist/9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d?token=<token>
  {"status":"ok"}
K2 REST add the same key again
  POST :17213/api/v1/keys?token=<token>
  Unhandled rejection: Err { reason: "failed to add key: The Sqlite3 query was malformed: error returned from database: (code: 2067) UNIQUE constraint failed: keys.key, packages/tracker-core/src/databases/driver/sqlite/auth_key_store.rs:74:22, packages/tracker-core/src/authentication/handler.rs:103:50" }
K3 REST remove a key that does not exist
  DELETE :17213/api/v1/key/ZZZZ4lMZupRuOpSRC3krIKR5BPB14nrJ?token=<token>
  Unhandled rejection: Err { reason: "failed to delete key: Failed to remove record from Sqlite3 database, error-code: 0, packages/tracker-core/src/databases/driver/sqlite/auth_key_store.rs:108:27" }
R1 whitelist an invalid info-hash
  POST :17212/api/v1/whitelist/zzz?token=<token>
  HTTP/1.1 400 Bad Request  Invalid URL: invalid infohash param: string "zzz", expected a 40 character long string
R2 no token
  POST :17212/api/v1/whitelist/3b245504cf5f11bbdbe1201cea6a6bf45aee1bc0
  HTTP/1.1 500 Internal Server Error  Unhandled rejection: Err { reason: "unauthorized" }
R3 wrong token
  POST :17212/api/v1/whitelist/3b245504cf5f11bbdbe1201cea6a6bf45aee1bc0?token=bad
  HTTP/1.1 500 Internal Server Error  Unhandled rejection: Err { reason: "token not valid" }
R4 key with invalid duration
  POST :17212/api/v1/key/abc?token=<token>
  HTTP/1.1 400 Bad Request  Invalid URL: Cannot parse `abc` to a `u64`
R5 torrent info for an invalid hash
  GET :17212/api/v1/torrent/zzz?token=<token>
  HTTP/1.1 400 Bad Request  Invalid URL: invalid infohash param: string "zzz", expected a 40 character long string
```

After (`c94bda536`):

```text
W1 REST whitelist the info-hash
  POST :17212/api/v1/whitelist/3b245504cf5f11bbdbe1201cea6a6bf45aee1bc0?token=<token>
  {"status":"ok"}
W2 REST whitelist the same info-hash again
  POST :17212/api/v1/whitelist/3b245504cf5f11bbdbe1201cea6a6bf45aee1bc0?token=<token>
  {"status":"ok"}
W3 REST remove an info-hash that is not whitelisted
  DELETE :17212/api/v1/whitelist/9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d?token=<token>
  {"status":"ok"}
K2 REST add the same key again
  POST :17213/api/v1/keys?token=<token>
  Unhandled rejection: Err { reason: "failed to add key: The Sqlite3 query was malformed: error returned from database: (code: 2067) UNIQUE constraint failed: keys.key" }
K3 REST remove a key that does not exist
  DELETE :17213/api/v1/key/ZZZZ4lMZupRuOpSRC3krIKR5BPB14nrJ?token=<token>
  Unhandled rejection: Err { reason: "failed to delete key: Failed to remove record from Sqlite3 database, error-code: 0" }
R1 whitelist an invalid info-hash
  POST :17212/api/v1/whitelist/zzz?token=<token>
  HTTP/1.1 400 Bad Request  Invalid URL: invalid infohash param: string "zzz", expected a 40 character long string
R2 no token
  POST :17212/api/v1/whitelist/3b245504cf5f11bbdbe1201cea6a6bf45aee1bc0
  HTTP/1.1 500 Internal Server Error  Unhandled rejection: Err { reason: "unauthorized" }
R3 wrong token
  POST :17212/api/v1/whitelist/3b245504cf5f11bbdbe1201cea6a6bf45aee1bc0?token=bad
  HTTP/1.1 500 Internal Server Error  Unhandled rejection: Err { reason: "token not valid" }
R4 key with invalid duration
  POST :17212/api/v1/key/abc?token=<token>
  HTTP/1.1 400 Bad Request  Invalid URL: Cannot parse `abc` to a `u64`
R5 torrent info for an invalid hash
  GET :17212/api/v1/torrent/zzz?token=<token>
  HTTP/1.1 400 Bad Request  Invalid URL: invalid infohash param: string "zzz", expected a 40 character long string
```

#### Conclusion

K2 and K3 held `packages/tracker-core/...` paths before and hold none after; the other replies held none in either run (they never carried a location). W2 and W3 answer `ok`, so they are not errors. The extra replies hold none in either run.

### V4 - M4 - Private tracker key errors (H2, H3, H4, H8)

- Goal: no reply holds a source path and each still names what failed.
- Requests: missing, unregistered, expired and malformed keys.
- Status: `DONE`

#### Observed Result

Before (`305027e70`):

```text
H8 HTTP announce with a malformed key (private tracker)
  GET :17080/announce/not-a-key?info_hash=%3B%24U%04%CF%5F%11%BB%DB%E1%20%1C%EAjk%F4Z%EE%1B%C0&peer_id=-qB00000000000000001&port=17548
  d14:failure reason156:Tracker authentication error: Invalid format for authentication key param. Error in packages/axum-http-server/src/v1/extractors/authentication_key.rs:109:23e
H2 HTTP announce without key (private tracker)
  GET :17080/announce?info_hash=%3B%24U%04%CF%5F%11%BB%DB%E1%20%1C%EAjk%F4Z%EE%1B%C0&peer_id=-qB00000000000000001&port=17548
  d14:failure reason108:Tracker authentication error: Missing authentication key, packages/http-core/src/services/announce.rs:308:27e
H3 HTTP announce with an unregistered key (private tracker)
  GET :17080/announce/YZSl4lMZupRuOpSRC3krIKR5BPB14nrJ?info_hash=%3B%24U%04%CF%5F%11%BB%DB%E1%20%1C%EAjk%F4Z%EE%1B%C0&peer_id=-qB00000000000000001&port=17548
  d14:failure reason141:Tracker authentication error: Failed to read key: YZSl4lMZupRuOpSRC3krIKR5BPB14nrJ, packages/tracker-core/src/authentication/service.rs:98:27e
H4 HTTP announce with the expired key (private tracker)
  GET :17080/announce/s4P5Ecf5GilrCfhjt4YSTxPgsCQTxVRW?info_hash=%3B%24U%04%CF%5F%11%BB%DB%E1%20%1C%EAjk%F4Z%EE%1B%C0&peer_id=-qB00000000000000001&port=17548
  d14:failure reason105:Tracker authentication error: Key has expired, packages/tracker-core/src/authentication/key/mod.rs:158:31e
```

After (`c94bda536`):

```text
H8 HTTP announce with a malformed key (private tracker)
  GET :17080/announce/not-a-key?info_hash=%3B%24U%04%CF%5F%11%BB%DB%E1%20%1C%EAjk%F4Z%EE%1B%C0&peer_id=-qB00000000000000001&port=17548
  d14:failure reason110:Tracker authentication error: Invalid format for authentication key param: expected 32 alphanumeric characterse
H2 HTTP announce without key (private tracker)
  GET :17080/announce?info_hash=%3B%24U%04%CF%5F%11%BB%DB%E1%20%1C%EAjk%F4Z%EE%1B%C0&peer_id=-qB00000000000000001&port=17548
  d14:failure reason56:Tracker authentication error: Missing authentication keye
H3 HTTP announce with an unregistered key (private tracker)
  GET :17080/announce/YZSl4lMZupRuOpSRC3krIKR5BPB14nrJ?info_hash=%3B%24U%04%CF%5F%11%BB%DB%E1%20%1C%EAjk%F4Z%EE%1B%C0&peer_id=-qB00000000000000001&port=17548
  d14:failure reason82:Tracker authentication error: Failed to read key: YZSl4lMZupRuOpSRC3krIKR5BPB14nrJe
H4 HTTP announce with the expired key (private tracker)
  GET :17080/announce/JQcPe8c9tJoBLBy1jwPh2WbOTB8lYHlP?info_hash=%3B%24U%04%CF%5F%11%BB%DB%E1%20%1C%EAjk%F4Z%EE%1B%C0&peer_id=-qB00000000000000001&port=17548
  d14:failure reason45:Tracker authentication error: Key has expirede
```

#### Conclusion

Each reply held a path before (`http-core`, `tracker-core`, `axum-http-server`) and none after.

### Summary of the path check

On the "after" output, `grep -nF -e '.rs:' -e 'packages/' -e 'packages\' -e 'src/' -e 'src\' -e '.cargo/registry' -e 'rustlib/src' evidence/responses-after.txt evidence/responses-extra-after.txt` (the seven forbidden substrings of the spec, Windows forms included) finds nothing (exit status 1, run 2026-11-24 14:37 UTC). R6 and R7 of `extra.sh` (HTTP scrape and announce with an invalid parameter) are further replies that held a path before and hold none after.

Server logs are out of scope (spec) and were not checked.
