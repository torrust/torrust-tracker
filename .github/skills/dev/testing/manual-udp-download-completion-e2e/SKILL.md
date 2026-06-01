---
name: manual-udp-download-completion-e2e
description: Manual end-to-end verification of started -> completed peer lifecycle using tracker_client (unified) and tracker stats API. Use when contributors need to simulate a peer completing a download without running containerized qBittorrent E2E. Triggers on "manual e2e", "simulate peer completion", "udp started completed test", or "verify downloads increment manually".
metadata:
  author: torrust
  version: "1.0"
---

# Manual UDP Download-Completion E2E

## Purpose

This skill verifies, manually and quickly, that a single peer transition from `started` to
`completed` updates tracker state correctly:

- seeders/leechers transition as expected
- torrent completed/download count increments
- global completed/download count increments

This workflow is a **diagnostic complement** to automated E2E (for example, `qbittorrent_e2e_runner`).

## Prerequisites

Run commands from repository root.

- Tracker config: `./share/default/config/tracker.development.sqlite3.toml`
- UDP tracker endpoint: `127.0.0.1:6969`
- Stats API endpoint: `http://127.0.0.1:1212/api/v1/stats?token=MyAccessToken`

Optional (recommended for deterministic baseline):

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
... API: Started on: http://0.0.0.0:1212
... UDP TRACKER: Started on: udp://0.0.0.0:6969
```

## 2. Define test values

In terminal B:

```bash
INFO_HASH=1111111111111111111111111111111111111111
PEER_ID=ABCDEFGHIJKLMNOPQRST
TRACKER=127.0.0.1:6969
STATS_URL='http://127.0.0.1:1212/api/v1/stats?token=MyAccessToken'
```

## 3. Capture baseline

### 3.1 Global stats

```bash
curl -s "$STATS_URL"
```

Example output:

```json
{"torrents":0,"seeders":0,"completed":0,"leechers":0,...}
```

### 3.2 Torrent-specific stats (scrape)

```bash
cargo run -q -p torrust-tracker-client --bin tracker_client -- udp scrape "$TRACKER" "$INFO_HASH"
```

Example output:

```json
{
  "Scrape": {
    "transaction_id": -214458979,
    "torrent_stats": [
      {
        "seeders": 0,
        "completed": 0,
        "leechers": 0
      }
    ]
  }
}
```

## 4. Send started announce

```bash
cargo run -q -p torrust-tracker-client --bin tracker_client -- udp announce \
  "$TRACKER" "$INFO_HASH" \
  --event started \
  --uploaded 0 \
  --downloaded 0 \
  --left 1000 \
  --port 6881 \
  --peer-id "$PEER_ID" \
  --key 1 \
  --peers-wanted 0
```

Example output:

```json
{
  "AnnounceIpv4": {
    "transaction_id": -888840697,
    "announce_interval": 120,
    "leechers": 1,
    "seeders": 0,
    "peers": []
  }
}
```

Verify after `started`:

```bash
cargo run -q -p torrust-tracker-client --bin tracker_client -- udp scrape "$TRACKER" "$INFO_HASH"
curl -s "$STATS_URL"
```

Expected checks:

- scrape `leechers` is `1`
- scrape `seeders` is `0`
- global `leechers` increased by `1`
- global `completed` unchanged

## 5. Send completed announce

```bash
cargo run -q -p torrust-tracker-client --bin tracker_client -- udp announce \
  "$TRACKER" "$INFO_HASH" \
  --event completed \
  --uploaded 0 \
  --downloaded 1000 \
  --left 0 \
  --port 6881 \
  --peer-id "$PEER_ID" \
  --key 1 \
  --peers-wanted 0
```

Example output:

```json
{
  "AnnounceIpv4": {
    "transaction_id": -888840697,
    "announce_interval": 120,
    "leechers": 0,
    "seeders": 1,
    "peers": []
  }
}
```

Verify after `completed`:

```bash
cargo run -q -p torrust-tracker-client --bin tracker_client -- udp scrape "$TRACKER" "$INFO_HASH"
curl -s "$STATS_URL"
```

Expected checks:

- scrape `seeders` changed `0 -> 1`
- scrape `completed` changed `0 -> 1`
- scrape `leechers` changed `1 -> 0`
- global `seeders` increased by `1`
- global `completed` increased by `1`

## 6. Optional output formatting with jq (human-friendly)

If `jq` is available, use these helpers:

```bash
curl -s "$STATS_URL" | jq '{torrents, seeders, completed, leechers}'

cargo run -q -p torrust-tracker-client --bin tracker_client -- udp scrape "$TRACKER" "$INFO_HASH" \
  | jq '.Scrape.torrent_stats[0]'
```

## Troubleshooting

- Peer ID must be exactly 20 bytes.
- Use a fresh `INFO_HASH` to avoid contamination from previous runs.
- If baseline numbers are non-zero, either reset SQLite DB or compare deltas instead of absolute values.
- Confirm tracker/API are listening on `6969/udp` and `1212/tcp`.

## Related

- Automated E2E runner: `src/bin/qbittorrent_e2e_runner.rs`
- Local tracker run workflow: `.github/skills/dev/environment-setup/run-tracker-locally/SKILL.md`
