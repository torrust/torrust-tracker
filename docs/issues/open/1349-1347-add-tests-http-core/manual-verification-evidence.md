---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/1349-1347-add-tests-http-core/ISSUE.md
last-updated-utc: 2026-10-21 15:05
---

# Manual Verification Evidence

<!-- cspell:ignore disablede peerslee -->

## Purpose

Real use of the built tracker against the behavior `torrust-tracker-http-core` owns: HTTP announce and scrape,
the peer-IP override policy, private mode, and the HTTP statistics. Slices S1-S5 and S6 changed only tests,
documentation and one comment, so the tracker built from this branch behaves as `develop` does. Everything below
was run by hand; the outputs are copied from the terminal.

## Environment and Prerequisites

- Date and time (UTC): 2026-10-21, 15:01-15:03.
- Artifact under test: the `torrust-tracker` and `tracker_client` debug binaries built on the S6 branch at the
  commit `docs(issues): [#1349] record the wiring-only conclusions of slice S6`. The only production-source
  difference from `develop` in `packages/http-core` is the comment of `services/mod.rs` (the commit
  `docs(http-core): [#1349] correct the services module comment about the Axum dependency`).
- Operating system / environment: Linux, stable Rust toolchain, loopback interface.
- Setup: two isolated configurations, both with their own SQLite file under the git-ignored `.tmp/1349-manual/`
  and non-default loopback ports (embedded below). Each tracker was started alone and stopped before the next.

`public.toml` (used for V1, V2, V3, V5):

```toml
[metadata]
app = "torrust-tracker"
purpose = "configuration"
schema_version = "3.0.0"

[logging]
trace_filter = "info"
trace_style = "full"

[core]
inactive_peer_cleanup_interval = 120
listed = false
private = false

[core.database]
driver = "sqlite3"
path = ".tmp/1349-manual/public.sqlite3.db"

[core.tracker_policy]
max_peer_timeout = 300
persistent_torrent_completed_stat = false
remove_peerless_torrents = false

[[http_trackers]]
bind_address = "127.0.0.1:17070"
tracker_usage_statistics = true
use_ip_from_query_string = false

[[http_trackers]]
bind_address = "127.0.0.1:17071"
tracker_usage_statistics = true
use_ip_from_query_string = true

[http_api]
bind_address = "127.0.0.1:11212"

[http_api.access_tokens]
admin = "MyAccessToken"
```

`private.toml` (used for V4) is the same except `private = true`, one HTTP tracker on `127.0.0.1:17072`, the API
on `127.0.0.1:11213` and the database `.tmp/1349-manual/private.sqlite3.db`.

Started with `./target/debug/torrust-tracker --config-toml-path "$PWD/.tmp/1349-manual/<name>.toml"`. The startup
was confirmed with `ss -tlnp` (all configured ports listening) before the first request, and after each stop
`ss -tlnp` showed none of them. The tracker logs of both runs contain no `WARN`/`ERROR`-level line and no panic
(the only case-insensitive matches of `error` were metric descriptions in the startup dump).

Every step below ran alone, after the previous one had returned. Times are `date -u +%T.%3N`.

## Verification Processes

### V1 - HTTP announce

- Goal: a valid bencoded announce response from the HTTP tracker.
- Initial state: fresh database, no torrent.
- Status: `DONE`

#### Steps Performed

1. 15:01:54.329 - `./target/debug/tracker_client http announce http://127.0.0.1:17070/announce 0123456789abcdef0123456789abcdef01234567`

#### Observed Result

```text
{"complete":1,"incomplete":0,"interval":120,"min interval":120,"peers":[]}
```

#### Conclusion

Met: a valid announce response; the announcing peer is the only seeder and is not echoed back.

### V2 - HTTP scrape

- Goal: the scrape returns the announced torrent.
- Initial state: the torrent of V1 exists.
- Status: `DONE`

#### Steps Performed

1. 15:01:58.822 - `./target/debug/tracker_client http scrape http://127.0.0.1:17070/announce 0123456789abcdef0123456789abcdef01234567`
   (a wrong first attempt: the scrape takes the tracker's base URL, not the announce URL).
2. 15:02:05.179 - `./target/debug/tracker_client http scrape http://127.0.0.1:17070 0123456789abcdef0123456789abcdef01234567`

#### Observed Result

Step 1 (the client sent a scrape to the announce path; the tracker rejected it before any service ran):

```text
{"failure reason":"Bad request. Cannot parse query params for announce request: missing param peer_id in packages/http-protocol/src/v1/requests/announce.rs:564:23"}
{"error":{"kind":"runtime_failure","source":"runtime","message":"unrecognized scrape response from tracker"}}
```

Step 2:

```text
{"0123456789abcdef0123456789abcdef01234567":{"complete":1,"downloaded":0,"incomplete":0}}
```

#### Conclusion

Met by step 2. Step 1 was my mistake with the URL, recorded because the request reached the tracker and V5 shows
it was not counted.

### V3 - Peer IP override policy

- Goal: `use_ip_from_query_string = false` rejects a client-supplied `ip`; `true` stores it.
- Initial state: the tracker of V1 (listener `17070` disabled, listener `17071` enabled; both share one core).
- Status: `DONE`

#### Steps Performed

1. 15:02:20.123 - `curl -s -i "http://127.0.0.1:17070/announce?info_hash=%aa%aa%aa%aa%aa%aa%aa%aa%aa%aa%aa%aa%aa%aa%aa%aa%aa%aa%aa%aa&peer_id=-qB00000000000000001&port=6881&uploaded=0&downloaded=0&left=0&event=started&compact=0&ip=203.0.113.9"`
2. 15:02:25.506 - the same request to `http://127.0.0.1:17071/announce`.
3. 15:02:25.540 - `curl -s http://127.0.0.1:11212/api/v1/torrent/aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa -H "Authorization: Bearer MyAccessToken"`

#### Observed Result

Step 1 (status line and body; the header lines are omitted):

```text
HTTP/1.1 200 OK
d14:failure reason37:Client-supplied peer IPs are disablede
```

Step 2:

```text
HTTP/1.1 200 OK
d8:completei1e10:incompletei0e8:intervali120e12:min intervali120e5:peerslee
```

Step 3:

```json
{"info_hash":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","seeders":1,"completed":0,"leechers":0,"peers":[{"peer_id":{"id":"0x2d71423030303030303030303030303030303031","client":null},"peer_addr":"203.0.113.9:6881","updated":1792594945525,"updated_milliseconds_ago":1792594945525,"updated_at_ms":1792594945525,"uploaded":0,"downloaded":0,"left":0,"event":"Started"}]}
```

#### Conclusion

Met. Disabled: a bencoded failure naming the disabled override (HTTP 200, as the BitTorrent protocol requires).
Enabled: the stored peer address is the supplied `203.0.113.9:6881`. The torrent has exactly one peer, so the
rejected request of step 1 stored nothing. (In the step 3 JSON, `updated_milliseconds_ago` equals `updated`; the REST
API documents both fields as deprecated aliases of the absolute `updated_at_ms`, and they are not an `http-core`
matter.)

### V4 - Private mode

- Goal: authentication on announce, zeroed scrape without a key, success with a key.
- Initial state: the private tracker, fresh database.
- Status: `DONE`

#### Steps Performed

1. 15:02:43.284 - announce without a key: `curl -s "http://127.0.0.1:17072/announce?info_hash=%aa%aa%aa%aa%aa%aa%aa%aa%aa%aa%aa%aa%aa%aa%aa%aa%aa%aa%aa%aa&peer_id=-qB00000000000000001&port=6881&uploaded=0&downloaded=0&left=0&event=started&compact=0"`
2. Scrape without a key (run right after step 1; not separately timed): `curl -s "http://127.0.0.1:17072/scrape?info_hash=%aa%aa%aa%aa%aa%aa%aa%aa%aa%aa%aa%aa%aa%aa%aa%aa%aa%aa%aa%aa"`
3. 15:02:47.665 - `curl -s -X POST http://127.0.0.1:11213/api/v1/key/3600 -H "Authorization: Bearer MyAccessToken"`
4. 15:02:52.620 - announce with the key: the step 1 request to `/announce/<key>`.
5. 15:02:52.648 - scrape with the key: `/scrape/<key>?info_hash=%aa%aa%aa%aa%aa%aa%aa%aa%aa%aa%aa%aa%aa%aa%aa%aa%aa%aa%aa%aa`

#### Observed Result

Step 1:

```text
d14:failure reason108:Tracker authentication error: Missing authentication key, packages/http-core/src/services/announce.rs:308:27e
```

Step 2 (the 20-byte info hash key is shown as raw bytes):

```text
d5:filesd20:<20 bytes>d8:completei0e10:downloadedi0e10:incompletei0eeee
```

Step 3:

```json
{"key":"TB62nrOj4IKXiB64Ir2DpIRwT717TebF","valid_until":1792598567,"expiry_time":"2026-10-21 16:02:47 UTC"}
```

Step 4:

```text
d8:completei1e10:incompletei0e8:intervali120e12:min intervali120e5:peerslee
```

Step 5:

```text
d5:filesd20:<20 bytes>d8:completei1e10:downloadedi0e10:incompletei0eeee
```

#### Conclusion

Met. The announce without a key fails with an authentication failure reason; the scrape without a key returns
zeroed data; both succeed with the key (the scrape then shows the announced seeder). The key above was created for
this run only and expired with it.

### V5 - HTTP core statistics

- Goal: the REST API statistics reflect the requests of V1 and V2.
- Initial state: after V1 and V2 on the public tracker, before V3.
- Status: `DONE`

#### Steps Performed

1. 15:02:11.629 - `curl -s http://127.0.0.1:11212/api/v1/stats -H "Authorization: Bearer MyAccessToken"`, keeping the `tcp*` fields.

#### Observed Result

```text
{'tcp4_connections_handled': 2, 'tcp4_announces_handled': 1, 'tcp4_scrapes_handled': 1, 'tcp6_connections_handled': 0, 'tcp6_announces_handled': 0, 'tcp6_scrapes_handled': 0}
```

#### Conclusion

Met: one announce and one scrape on the IPv4-bound listener. The rejected request of V2 step 1 (a scrape sent to
the announce path, which failed in request parsing) was not counted, because the counters are updated by the
services after parsing.
