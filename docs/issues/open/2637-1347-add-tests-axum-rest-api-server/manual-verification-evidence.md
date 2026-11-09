---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2637-1347-add-tests-axum-rest-api-server/ISSUE.md
last-updated-utc: 2026-11-09 10:36
---

# Manual Verification Evidence

<!-- cspell:ignore tcp4 tcp6 Aiqnh OHCK Hgzn -->

## Purpose

Real use of the built tracker against the behavior `torrust-tracker-axum-rest-api-server` owns: the public and
protected routes, the token sources, the configuration-gated contexts, the statistics formats, and TLS with a
graceful shutdown. Scenarios M1-M5 of the issue spec are recorded as V1-V5 below. The commands were typed for this
run and executed one after another by a sequential shell script against the tracker binary; the output is copied from
the script's transcript. Each transcript line is the `curl` command as executed. The response bodies are cut to the
first characters by the script (120 to 330 characters, chosen per command: 200 for most, 160 for the stats JSON, 330
for the Prometheus bodies); the status code, the timestamps and the shutdown lines are complete and nothing is
edited.

## Environment and Prerequisites

- Date and time (UTC): 2026-11-09, 10:36 (the first run of 2026-11-06 was repeated in this form after the review of
  PR 2772, finding F3; its results were the same).
- Artifact under test: the `torrust-tracker` debug binary (`cargo build --bin torrust-tracker`, stable Rust 1.99.1)
  built on the S6 branch after the commit `docs(pr-reviews): [#2637] record the review of PR 2772`. The production
  source of the package is the same as `develop` apart from the documentation of `lib.rs`.
- Operating system / environment: Linux, loopback interface.
- Setup: four isolated configurations, each with its own SQLite file under the git-ignored `.tmp/2637-manual/` (the
  only place the run wrote to) and a distinct loopback API port; UDP and HTTP trackers on port 0. The tracker was
  started in the background, polled until `/api/health_check` answered, probed, stopped with `SIGINT` and waited for
  before the next one started. The commands ran one after another in one shell script that stamped each step with
  `date -u +%T.%3N`; no dependent step ran in parallel.

`public.toml` (V1, V2, V3 default mode, V4; API on `127.0.0.1:41212`):

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
path = ".tmp/2637-manual/public.sqlite3.db"

[[udp_trackers]]
bind_address = "127.0.0.1:0"

[[http_trackers]]
bind_address = "127.0.0.1:0"

[http_api]
bind_address = "127.0.0.1:41212"

[http_api.access_tokens]
admin = "MyAccessToken"
```

The other configurations differ from it only as `diff --suppress-blank-empty public.toml <file>` shows (the line numbers are those of
`public.toml`):

`private.toml` (V3, `private = true`; API on port 41213):

```text
13c13
< private = false
---
> private = true
17c17
< path = ".tmp/2637-manual/public.sqlite3.db"
---
> path = ".tmp/2637-manual/private.sqlite3.db"
26c26
< bind_address = "127.0.0.1:41212"
---
> bind_address = "127.0.0.1:41213"
```

`listed.toml` (V3, `listed = true`; API on port 41214):

```text
12c12
< listed = false
---
> listed = true
17c17
< path = ".tmp/2637-manual/public.sqlite3.db"
---
> path = ".tmp/2637-manual/listed.sqlite3.db"
26c26
< bind_address = "127.0.0.1:41212"
---
> bind_address = "127.0.0.1:41214"
```

`tls.toml` (V5, API on port 41215 with a self-signed certificate,
generated for the run with `openssl req -x509 -out .tmp/2637-manual/localhost.crt -keyout .tmp/2637-manual/localhost.key -newkey rsa:2048 -nodes -sha256 -days 1 -subj '/CN=localhost' -addext 'subjectAltName=DNS:localhost,IP:127.0.0.1' -addext 'keyUsage=digitalSignature' -addext 'extendedKeyUsage=serverAuth'`):

```text
17c17
< path = ".tmp/2637-manual/public.sqlite3.db"
---
> path = ".tmp/2637-manual/tls.sqlite3.db"
26c26
< bind_address = "127.0.0.1:41212"
---
> bind_address = "127.0.0.1:41215"
29a30,33
>
> [http_api.tls_config]
> ssl_cert_path = ".tmp/2637-manual/localhost.crt"
> ssl_key_path = ".tmp/2637-manual/localhost.key"
```

## Verification Processes

### V1 - Public and protected routes (M1)

- Goal: the health check answers without a token; a protected route fails without or with a wrong token and
  answers with the configured token.
- Initial state: `public.toml`, tracker just started.
- Status: `DONE`

#### Steps Performed

`curl` of `/api/health_check` with no token, then of `/api/v1/stats` with no token, with `Bearer wrong`, and with
`Bearer MyAccessToken`.

#### Observed Result

```text
[10:36:11.636] $ target/debug/torrust-tracker --config-toml-path .tmp/2637-manual/public.toml
[10:36:12.153] API answering on http://127.0.0.1:41212
[10:36:12.155] $ curl http://127.0.0.1:41212/api/health_check
HTTP 200 (curl exit 0)
{"status":"Ok"}
[10:36:12.164] done
[10:36:12.165] $ curl http://127.0.0.1:41212/api/v1/stats
HTTP 500 (curl exit 0)
Unhandled rejection: Err { reason: "unauthorized" }
[10:36:12.174] done
[10:36:12.176] $ curl -H "Authorization: Bearer wrong" http://127.0.0.1:41212/api/v1/stats
HTTP 500 (curl exit 0)
Unhandled rejection: Err { reason: "token not valid" }
[10:36:12.184] done
[10:36:12.185] $ curl -H "Authorization: Bearer MyAccessToken" http://127.0.0.1:41212/api/v1/stats
HTTP 200 (curl exit 0)
{"torrents":0,"seeders":0,"completed":0,"completed_in_session":0,"completed_persisted":0,"completed_persisted_enabled":false,"leechers":0,"tcp4_connections_hand
[10:36:12.193] done
```

#### Conclusion

Met. The health check answered `200` without a token; the protected route answered `500` (an unhandled rejection,
see the observation at the end) without a token and with a wrong token, and `200` with the configured token.

### V2 - Token sources and precedence (M2)

- Goal: the header token decides when both are present; a header with another scheme is rejected.
- Initial state: the same tracker as V1, still running.
- Status: `DONE`

#### Steps Performed

`curl` of `/api/v1/stats` with a valid `Bearer` header and `?token=bad`; with `Bearer bad` and `?token=MyAccessToken`;
with `Authorization: Basic MyAccessToken`; and with a bare `Authorization: MyAccessToken`.

#### Observed Result

```text
[10:36:12.195] $ curl -H "Authorization: Bearer MyAccessToken" http://127.0.0.1:41212/api/v1/stats?token=bad
HTTP 200 (curl exit 0)
{"torrents":0,"seeders":0,"completed":0,"completed_in_session":0,"completed_persisted":0,"completed_persisted_enabled":f
[10:36:12.203] done
[10:36:12.205] $ curl -H "Authorization: Bearer bad" http://127.0.0.1:41212/api/v1/stats?token=MyAccessToken
HTTP 500 (curl exit 0)
Unhandled rejection: Err { reason: "token not valid" }
[10:36:12.213] done
[10:36:12.215] $ curl -H "Authorization: Basic MyAccessToken" http://127.0.0.1:41212/api/v1/stats
HTTP 500 (curl exit 0)
Unhandled rejection: Err { reason: "unknown token provided" }
[10:36:12.222] done
[10:36:12.225] $ curl -H "Authorization: MyAccessToken" http://127.0.0.1:41212/api/v1/stats
HTTP 500 (curl exit 0)
Unhandled rejection: Err { reason: "unknown token provided" }
[10:36:12.232] done
```

#### Conclusion

Met. A valid header won over an invalid query (`200`); an invalid header was not rescued by a valid query (`500`
`token not valid`); another scheme and a bare token were rejected (`500` `unknown token provided`).

### V3 - Configuration-gated contexts (M3)

- Goal: `409` with the disabled reason in the default mode; the key and whitelist operations succeed in their modes.
- Initial state: three trackers, one per configuration, each started and stopped in turn.
- Status: `DONE`

#### Steps Performed

`POST /api/v1/key/60` and `POST /api/v1/whitelist/9c38422213e30bff212b30c360d26f9a02136422` with the token, against
`public.toml` (shown in V1/V2/V4 run, below), then `private.toml`, then `listed.toml`.

#### Observed Result

Default mode (`public.toml`, the same run as V1, V2 and V4):

```text
[10:36:12.234] $ curl -X POST -H "Authorization: Bearer MyAccessToken" http://127.0.0.1:41212/api/v1/key/60
HTTP 409 (curl exit 0)
{"status":"err","reason":"private capability is disabled by configuration"}
[10:36:12.242] done
[10:36:12.244] $ curl -X POST -H "Authorization: Bearer MyAccessToken" http://127.0.0.1:41212/api/v1/whitelist/9c38422213e30bff212b30c360d26f9a02136422
HTTP 409 (curl exit 0)
{"status":"err","reason":"listed capability is disabled by configuration"}
[10:36:12.252] done
```

`private = true` (`private.toml`):

```text
[10:36:12.807] $ target/debug/torrust-tracker --config-toml-path .tmp/2637-manual/private.toml
[10:36:13.324] API answering on http://127.0.0.1:41213
[10:36:13.326] $ curl -X POST -H "Authorization: Bearer MyAccessToken" http://127.0.0.1:41213/api/v1/key/60
HTTP 200 (curl exit 0)
{"key":"AiqnhY5rsOHCKTr0clwZSd3KDWv6Hgzn","valid_until":1794220633,"expiry_time":"2026-11-09 10:37:13 UTC"}
[10:36:13.347] done
[10:36:13.349] $ curl -X POST -H "Authorization: Bearer MyAccessToken" http://127.0.0.1:41213/api/v1/whitelist/9c38422213e30bff212b30c360d26f9a02136422
HTTP 409 (curl exit 0)
{"status":"err","reason":"listed capability is disabled by configuration"}
[10:36:13.358] done
```

`listed = true` (`listed.toml`):

```text
[10:36:13.873] $ target/debug/torrust-tracker --config-toml-path .tmp/2637-manual/listed.toml
[10:36:14.391] API answering on http://127.0.0.1:41214
[10:36:14.394] $ curl -X POST -H "Authorization: Bearer MyAccessToken" http://127.0.0.1:41214/api/v1/key/60
HTTP 409 (curl exit 0)
{"status":"err","reason":"private capability is disabled by configuration"}
[10:36:14.404] done
[10:36:14.406] $ curl -X POST -H "Authorization: Bearer MyAccessToken" http://127.0.0.1:41214/api/v1/whitelist/9c38422213e30bff212b30c360d26f9a02136422
HTTP 200 (curl exit 0)
{"status":"ok"}
[10:36:14.429] done
```

#### Conclusion

Met. In the default mode both operations answered `409` with the disabled reason; with `private = true` only the key
operation succeeded (`200`, a key); with `listed = true` only the whitelist operation succeeded (`200`).

### V4 - Statistics formats (M4)

- Goal: JSON and Prometheus text bodies for `/api/v1/stats` and `/api/v1/metrics`.
- Initial state: the tracker of V1.
- Status: `DONE`

#### Steps Performed

`curl` of `/api/v1/stats` and `/api/v1/metrics` with `format=json` and with `format=prometheus`, with the token.

#### Observed Result

```text
[10:36:12.254] $ curl -H "Authorization: Bearer MyAccessToken" http://127.0.0.1:41212/api/v1/stats?format=json
HTTP 200 (curl exit 0)
{"torrents":0,"seeders":0,"completed":0,"completed_in_session":0,"completed_persisted":0,"completed_persisted_enabled":false,"leechers":0,"tcp4_connections_hand
[10:36:12.261] done
[10:36:12.263] $ curl -H "Authorization: Bearer MyAccessToken" http://127.0.0.1:41212/api/v1/stats?format=prometheus
HTTP 200 (curl exit 0)
# HELP torrents Total number of torrents.
# TYPE torrents gauge
torrents 0
# HELP seeders Total number of seeders for all torrents.
# TYPE seeders gauge
seeders 0
# HELP completed Deprecated ambiguous completed-download total. Use completed_in_session and completed_persisted instead.
# TYPE completed counter
completed 0
# HELP c
[10:36:12.270] done
[10:36:12.272] $ curl -H "Authorization: Bearer MyAccessToken" http://127.0.0.1:41212/api/v1/metrics?format=json
HTTP 200 (curl exit 0)
{"metrics":[{"type":"counter","name":"udp_tracker_server_responses_sent_total","unit":"count","description":"Total number of UDP responses sent","samples":[]},{
[10:36:12.280] done
[10:36:12.282] $ curl -H "Authorization: Bearer MyAccessToken" http://127.0.0.1:41212/api/v1/metrics?format=prometheus
HTTP 200 (curl exit 0)
# HELP tracker_core_in_session_torrents_downloads_total The number of torrent downloads completed since this tracker process started.
# TYPE tracker_core_in_session_torrents_downloads_total counter
tracker_core_in_session_torrents_downloads_total 0

# HELP tracker_core_persistent_torrents_downloads_total Deprecated: use tracker_
[10:36:12.289] done
```

Then the shutdown of the tracker of V1-V4:

```text
[10:36:12.291] $ kill -INT 2747884
[10:36:12.799] tracker exited with code 0
--- last log lines (ANSI removed) ---
2026-11-09T10:36:12.292206Z  INFO HEALTH CHECK API: Stopped server running on: http://127.0.0.1:1313
2026-11-09T10:36:12.292223Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job completed after cooperative cancellation job=health_check_api
2026-11-09T10:36:12.292238Z  INFO torrust_tracker: Torrust tracker successfully shutdown.
```

#### Conclusion

Met. All four answered `200`; the `json` bodies are JSON (`{"torrents":...}`, `{"metrics":[...]}`) and the
`prometheus` bodies are Prometheus text (`# HELP`, `# TYPE`).

### V5 - TLS and graceful shutdown (M5)

- Goal: the API answers over HTTPS and the process exits cleanly on `SIGINT`.
- Initial state: `tls.toml` with `[http_api.tls_config]` and a self-signed certificate.
- Status: `DONE`

#### Steps Performed

`curl --insecure` of `https://127.0.0.1:41215/api/health_check` and of `/api/v1/stats` with the token; plain
`http://` to the same port; `kill -INT` of the tracker.

#### Observed Result

```text
[10:36:14.944] $ target/debug/torrust-tracker --config-toml-path .tmp/2637-manual/tls.toml
[10:36:15.526] API answering on https://127.0.0.1:41215
[10:36:15.529] $ curl --insecure https://127.0.0.1:41215/api/health_check
HTTP 200 (curl exit 0)
{"status":"Ok"}
[10:36:15.582] done
[10:36:15.584] $ curl --insecure -H "Authorization: Bearer MyAccessToken" https://127.0.0.1:41215/api/v1/stats
HTTP 200 (curl exit 0)
{"torrents":0,"seeders":0,"completed":0,"completed_in_session":0,"completed_persisted":0,"completed_persisted_enabled":false,"leechers":0,"tcp4_connections_hand
[10:36:15.637] done
## plain http to the TLS port
[10:36:15.640] $ curl http://127.0.0.1:41215/api/health_check
HTTP 000 (curl exit 1)
[10:36:15.648] done
```

```text
[10:36:15.650] $ kill -INT 2748579
[10:36:16.158] tracker exited with code 0
--- last log lines (ANSI removed) ---
2026-11-09T10:36:15.651339Z  INFO HEALTH CHECK API: Stopped server running on: http://127.0.0.1:1313
2026-11-09T10:36:15.651346Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job completed after cooperative cancellation job=health_check_api
2026-11-09T10:36:15.651352Z  INFO torrust_tracker: Torrust tracker successfully shutdown.
```

(`HTTP 000` with `curl` exit 1 is the plain `http://` request being refused by the TLS listener.)

#### Conclusion

Met. The API answered over HTTPS (`200`), a plain `http://` request did not succeed, and the process exited with
code 0 after logging `successfully shutdown`.

## Failures and Follow-up

No process failed. Observation, not a #2637 change: a missing or invalid token answers `500` with a plain-text
`Unhandled rejection` body, not `401`/`403`. The unit tests of the middleware pin this behavior. Changing it would be
a breaking REST API change; the maintainer decided it belongs to the REST API v2, postponed until after the tracker
4.0.0 release (recorded in `lessons.md`).
