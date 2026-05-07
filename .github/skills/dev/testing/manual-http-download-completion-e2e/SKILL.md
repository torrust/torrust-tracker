---
name: manual-http-download-completion-e2e
description: Manual end-to-end verification of started -> completed peer lifecycle using the HTTP tracker announce/scrape endpoints with curl (or browser for stats). Use when contributors want a fast, transparent simulation of download completion without containerized clients. Triggers on "manual http e2e", "http announce completed test", "simulate completion with curl", or "verify completed counter http".
metadata:
  author: torrust
  version: "1.0"
---

# Manual HTTP Download-Completion E2E

## Purpose

This skill verifies manually that an HTTP peer transition from `started` to `completed`
updates tracker state correctly:

- announce response changes from leecher view to seeder view
- scrape stats change (`incomplete -> complete`, `downloaded` increments)
- global tracker stats change (`seeders` and `completed` increment)

This is a fast diagnostic workflow. It complements automated E2E (for example,
`src/bin/qbittorrent_e2e_runner.rs`).

This same started-to-completed scenario can also be exercised with the HTTP tracker client,
similar to the UDP workflow in
`.github/skills/dev/testing/manual-udp-download-completion-e2e/SKILL.md`.
This skill intentionally documents a generic HTTP client approach (curl/browser),
so contributors can reproduce the flow without relying on a specific tracker client binary.

## Prerequisites

Run all commands from repository root.

- HTTP tracker: `http://127.0.0.1:7070`
- Stats API: `http://127.0.0.1:1212/api/v1/stats?token=MyAccessToken`

Optional clean baseline:

```bash
rm -f ./storage/tracker/lib/database/sqlite3.db
```

## 1. Start tracker

In terminal A:

```bash
cargo run
```

Expected startup excerpt:

```text
Loading extra configuration from default configuration file: `./share/default/config/tracker.development.sqlite3.toml` ...
... HTTP TRACKER: Started on: http://0.0.0.0:7070
... API: Started on: http://0.0.0.0:1212
```

## 2. Define test values

In terminal B:

```bash
INFO_HASH='TTTTTTTTTTTTTTTTTTTT'
PEER_ID='HTTPCLIENTPEERID0000'
BASE='http://127.0.0.1:7070'
STATS='http://127.0.0.1:1212/api/v1/stats?token=MyAccessToken'
```

Notes:

- `INFO_HASH` must be exactly 20 bytes in this curl workflow.
- `PEER_ID` must be exactly 20 bytes.

## 3. Baseline checks

### 3.1 Global stats

Command:

```bash
curl -s "$STATS"
```

Output captured during validation:

```json
{
  "torrents": 0,
  "seeders": 0,
  "completed": 0,
  "leechers": 0,
  "tcp4_connections_handled": 0,
  "tcp4_announces_handled": 0,
  "tcp4_scrapes_handled": 0,
  "tcp6_connections_handled": 0,
  "tcp6_announces_handled": 0,
  "tcp6_scrapes_handled": 0,
  "udp_requests_aborted": 0,
  "udp_requests_banned": 0,
  "udp_banned_ips_total": 0,
  "udp_avg_connect_processing_time_ns": 0,
  "udp_avg_announce_processing_time_ns": 0,
  "udp_avg_scrape_processing_time_ns": 0,
  "udp4_requests": 0,
  "udp4_connections_handled": 0,
  "udp4_announces_handled": 0,
  "udp4_scrapes_handled": 0,
  "udp4_responses": 0,
  "udp4_errors_handled": 0,
  "udp6_requests": 0,
  "udp6_connections_handled": 0,
  "udp6_announces_handled": 0,
  "udp6_scrapes_handled": 0,
  "udp6_responses": 0,
  "udp6_errors_handled": 0
}
```

### 3.2 Torrent scrape

Command:

```bash
curl -sG "$BASE/scrape" --data-urlencode "info_hash=$INFO_HASH"
```

Output captured during validation:

```text
d5:filesd20:TTTTTTTTTTTTTTTTTTTTd8:completei0e10:downloadedi0e10:incompletei0eeee
```

## 4. Announce started

Command:

```bash
curl -sG "$BASE/announce" \
  --data-urlencode "info_hash=$INFO_HASH" \
  --data-urlencode "peer_id=$PEER_ID" \
  --data-urlencode "port=6881" \
  --data-urlencode "uploaded=0" \
  --data-urlencode "downloaded=0" \
  --data-urlencode "left=1000" \
  --data-urlencode "event=started" \
  --data-urlencode "compact=1" \
  --data-urlencode "numwant=0"
```

Output captured during validation:

```text
d8:completei0e10:incompletei1e8:intervali120e12:min intervali120e5:peers0:6:peers60:e
```

Then verify scrape and global stats:

```bash
curl -sG "$BASE/scrape" --data-urlencode "info_hash=$INFO_HASH"
curl -s "$STATS"
```

Outputs captured during validation:

```text
d5:filesd20:TTTTTTTTTTTTTTTTTTTTd8:completei0e10:downloadedi0e10:incompletei1eeee
```

```json
{
  "torrents": 1,
  "seeders": 0,
  "completed": 0,
  "leechers": 1,
  "tcp4_connections_handled": 3,
  "tcp4_announces_handled": 1,
  "tcp4_scrapes_handled": 2,
  "tcp6_connections_handled": 0,
  "tcp6_announces_handled": 0,
  "tcp6_scrapes_handled": 0,
  "udp_requests_aborted": 0,
  "udp_requests_banned": 0,
  "udp_banned_ips_total": 0,
  "udp_avg_connect_processing_time_ns": 0,
  "udp_avg_announce_processing_time_ns": 0,
  "udp_avg_scrape_processing_time_ns": 0,
  "udp4_requests": 0,
  "udp4_connections_handled": 0,
  "udp4_announces_handled": 0,
  "udp4_scrapes_handled": 0,
  "udp4_responses": 0,
  "udp4_errors_handled": 0,
  "udp6_requests": 0,
  "udp6_connections_handled": 0,
  "udp6_announces_handled": 0,
  "udp6_scrapes_handled": 0,
  "udp6_responses": 0,
  "udp6_errors_handled": 0
}
```

Expected meaning:

- `incomplete` became `1`
- global `leechers` became `1`
- global `completed` still `0`

## 5. Announce completed

Command:

```bash
curl -sG "$BASE/announce" \
  --data-urlencode "info_hash=$INFO_HASH" \
  --data-urlencode "peer_id=$PEER_ID" \
  --data-urlencode "port=6881" \
  --data-urlencode "uploaded=0" \
  --data-urlencode "downloaded=1000" \
  --data-urlencode "left=0" \
  --data-urlencode "event=completed" \
  --data-urlencode "compact=1" \
  --data-urlencode "numwant=0"
```

Output captured during validation:

```text
d8:completei1e10:incompletei0e8:intervali120e12:min intervali120e5:peers0:6:peers60:e
```

Then verify scrape and global stats:

```bash
curl -sG "$BASE/scrape" --data-urlencode "info_hash=$INFO_HASH"
curl -s "$STATS"
```

Outputs captured during validation:

```text
d5:filesd20:TTTTTTTTTTTTTTTTTTTTd8:completei1e10:downloadedi1e10:incompletei0eeee
```

```json
{
  "torrents": 1,
  "seeders": 1,
  "completed": 1,
  "leechers": 0,
  "tcp4_connections_handled": 5,
  "tcp4_announces_handled": 2,
  "tcp4_scrapes_handled": 3,
  "tcp6_connections_handled": 0,
  "tcp6_announces_handled": 0,
  "tcp6_scrapes_handled": 0,
  "udp_requests_aborted": 0,
  "udp_requests_banned": 0,
  "udp_banned_ips_total": 0,
  "udp_avg_connect_processing_time_ns": 0,
  "udp_avg_announce_processing_time_ns": 0,
  "udp_avg_scrape_processing_time_ns": 0,
  "udp4_requests": 0,
  "udp4_connections_handled": 0,
  "udp4_announces_handled": 0,
  "udp4_scrapes_handled": 0,
  "udp4_responses": 0,
  "udp4_errors_handled": 0,
  "udp6_requests": 0,
  "udp6_connections_handled": 0,
  "udp6_announces_handled": 0,
  "udp6_scrapes_handled": 0,
  "udp6_responses": 0,
  "udp6_errors_handled": 0
}
```

Expected meaning:

- scrape `complete`: `0 -> 1`
- scrape `downloaded`: `0 -> 1`
- scrape `incomplete`: `1 -> 0`
- global `seeders`: `0 -> 1`
- global `completed`: `0 -> 1`

## 6. Browser option

You can open global stats directly in a browser:

```text
http://127.0.0.1:1212/api/v1/stats?token=MyAccessToken
```

Use page refresh between steps to observe the counter changes.

## Troubleshooting

If announce fails with peer-id validation, check peer-id length.

Example failure output captured during validation (peer_id had 21 bytes):

```text
d14:failure reason269:Bad request. Cannot parse query params for announce request: invalid param value HTTPCLIENTPEERID00001 for peer_id in too many bytes for peer id: got 21 bytes, expected 20 ...e
```

## Related

- Automated real-client E2E: `src/bin/qbittorrent_e2e_runner.rs`
- Manual UDP equivalent: `.github/skills/dev/testing/manual-udp-download-completion-e2e/SKILL.md`
