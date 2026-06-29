---
name: use-rest-api
description: Use the Torrust Tracker REST API. Covers authentication, all endpoints (stats, metrics, torrents, auth keys, whitelist), and making announce/scrape requests to verify API behaviour. Triggers on "use API", "test API", "call REST API", "query API", "API endpoint", "curl tracker", "tracker client", "announce request", or "verify API".
metadata:
  author: torrust
  version: "1.0"
---

# Use REST API

## Prerequisites

A running tracker with the REST API enabled. The default development config starts the API on port 1212:

```bash
cargo run
```

## Authentication

All API endpoints (except `/api/health_check`) require an access token.

### Header Method (preferred)

```bash
curl -H "Authorization: Bearer MyAccessToken" http://localhost:1212/api/v1/stats
```

### Query Parameter Method

```bash
curl "http://localhost:1212/api/v1/stats?token=MyAccessToken"
```

### Configuration

Tokens are defined in the TOML config file under `[http_api.access_tokens]`:

```toml
[http_api.access_tokens]
admin = "MyAccessToken"
```

Every token in the map has identical permissions — the label (`admin`) is just a human-readable name.

## Endpoints

All endpoints use `http://localhost:1212` as base (default dev config).

### Health Check

| Method | Endpoint            | Auth  |
| ------ | ------------------- | ----- |
| GET    | `/api/health_check` | ❌ No |

```bash
curl -s http://localhost:1212/api/health_check
```

### Stats

| Method | Endpoint          | Auth   |
| ------ | ----------------- | ------ |
| GET    | `/api/v1/stats`   | ✅ Yes |
| GET    | `/api/v1/metrics` | ✅ Yes |

```bash
curl -s http://localhost:1212/api/v1/stats -H "Authorization: Bearer MyAccessToken"
curl -s http://localhost:1212/api/v1/metrics -H "Authorization: Bearer MyAccessToken"
```

### Auth Keys

| Method | Endpoint                             | Auth   |
| ------ | ------------------------------------ | ------ |
| POST   | `/api/v1/key/{seconds_valid_or_key}` | ✅ Yes |
| DELETE | `/api/v1/key/{seconds_valid_or_key}` | ✅ Yes |
| GET    | `/api/v1/keys/reload`                | ✅ Yes |
| POST   | `/api/v1/keys`                       | ✅ Yes |

### Whitelist

| Method | Endpoint                        | Auth   |
| ------ | ------------------------------- | ------ |
| POST   | `/api/v1/whitelist/{info_hash}` | ✅ Yes |
| DELETE | `/api/v1/whitelist/{info_hash}` | ✅ Yes |
| GET    | `/api/v1/whitelist/reload`      | ✅ Yes |

### Torrents

| Method | Endpoint                      | Auth   |
| ------ | ----------------------------- | ------ |
| GET    | `/api/v1/torrent/{info_hash}` | ✅ Yes |
| GET    | `/api/v1/torrents`            | ✅ Yes |

## Making Announce Requests with the Tracker Client

The `tracker_client` binary can make BitTorrent announce requests to verify the tracker is working.

### UDP Announce

```bash
cargo run -p torrust-tracker-client --bin tracker_client -- udp announce udp://localhost:6969/announce 0123456789abcdef0123456789abcdef01234567
```

### HTTP Announce

```bash
cargo run -p torrust-tracker-client --bin tracker_client -- http announce http://localhost:7070/announce 0123456789abcdef0123456789abcdef01234567
```

### Scrape

```bash
cargo run -p torrust-tracker-client --bin tracker_client -- udp scrape udp://localhost:6969/announce 0123456789abcdef0123456789abcdef01234567
```

Output defaults to JSON. Use `--format text` for human-readable output.

## Verification Workflow

After making an announce request, verify the API reflects the activity:

1. Check stats changed:

   ```bash
   curl -s http://localhost:1212/api/v1/stats -H "Authorization: Bearer MyAccessToken"
   ```

   Expect `torrents` and `seeders` to increase.

2. Check metrics changed:

   ```bash
   curl -s http://localhost:1212/api/v1/metrics -H "Authorization: Bearer MyAccessToken"
   ```

   Expect protocol-specific counters to increase.

3. Check tracker console logs show the request was received:

   ```text
   active_peers_total=1 active_torrents_total=1
   ```
