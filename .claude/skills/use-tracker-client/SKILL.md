---
name: use-tracker-client
description: Use the Torrust Tracker Client CLI to make BitTorrent announce and scrape requests against UDP and HTTP trackers. Covers the unified `tracker_client` binary, all subcommands, options, and output formats. Triggers on "tracker client", "use tracker client", "announce request", "scrape request", "http announce", "udp announce", "tracker_client", "test tracker", or "verify tracker".
metadata:
  author: torrust
  version: "1.0"
---

# Use Tracker Client

## Prerequisites

A running tracker. The default development config starts UDP trackers on ports 6969 and 6868,
HTTP trackers on ports 7070 and 7171:

```bash
cargo run
```

## Skill Links

This skill depends on these artifacts. If any of them change, review this skill.

- `console/tracker-client/src/console/clients/unified/app.rs`
- `console/tracker-client/src/console/clients/unified/http.rs`
- `console/tracker-client/src/console/clients/unified/udp.rs`
- `console/tracker-client/Cargo.toml`
- `packages/http-protocol/src/v1/requests/announce.rs`
- `packages/http-protocol/src/v1/responses/announce/`

Use the marker `skill-link: use-tracker-client` in affected artifacts.

## Quick Start

The unified `tracker_client` binary is in the `torrust-tracker-client` package:

```bash
cargo run -p torrust-tracker-client --bin tracker_client -- <protocol> <command> <args...>
```

The binary supports three top-level subcommands:

| Subcommand | Description                         |
| ---------- | ----------------------------------- |
| `http`     | HTTP tracker announce and scrape    |
| `udp`      | UDP tracker announce and scrape     |
| `check`    | Tracker checker (health monitoring) |

## HTTP Client

### HTTP Announce

```bash
cargo run -p torrust-tracker-client --bin tracker_client -- http announce <tracker_url> <info_hash>
```

**Example**:

```bash
cargo run -p torrust-tracker-client --bin tracker_client -- http announce http://127.0.0.1:7070 9c38422213e30bff212b30c360d26f9a02136422
```

**Response** (JSON):

```json
{
  "complete": 1,
  "incomplete": 0,
  "interval": 120,
  "min interval": 120,
  "peers": []
}
```

**Options**:

| Option         | Type   | Description                          |
| -------------- | ------ | ------------------------------------ |
| `--event`      | enum   | `started`, `stopped`, `completed`    |
| `--uploaded`   | u64    | Bytes uploaded                       |
| `--downloaded` | u64    | Bytes downloaded                     |
| `--left`       | u64    | Bytes left to download               |
| `--port`       | u16    | Client port (non-zero)               |
| `--peer-addr`  | IpAddr | Peer IP address                      |
| `--peer-id`    | PeerId | 20-byte hex-encoded peer ID          |
| `--compact`    | enum   | `0` (not accepted) or `1` (accepted) |
| `--format`     | enum   | `json` (default) or `text`           |

**Example with options**:

```bash
cargo run -p torrust-tracker-client --bin tracker_client -- http announce \
  http://127.0.0.1:7070 \
  9c38422213e30bff212b30c360d26f9a02136422 \
  --event started \
  --uploaded 0 \
  --downloaded 0 \
  --left 1000 \
  --port 6881 \
  --compact 1
```

### HTTP Scrape

```bash
cargo run -p torrust-tracker-client --bin tracker_client -- http scrape <tracker_url> <info_hash> [info_hash...]
```

**Example**:

```bash
cargo run -p torrust-tracker-client --bin tracker_client -- http scrape http://127.0.0.1:7070 9c38422213e30bff212b30c360d26f9a02136422
```

**Response** (JSON):

```json
{
  "9c38422213e30bff212b30c360d26f9a02136422": {
    "complete": 1,
    "downloaded": 0,
    "incomplete": 0
  }
}
```

**Options**:

| Option     | Type | Description                |
| ---------- | ---- | -------------------------- |
| `--format` | enum | `json` (default) or `text` |

Multiple info hashes can be provided (space-separated):

```bash
cargo run -p torrust-tracker-client --bin tracker_client -- http scrape \
  http://127.0.0.1:7070 \
  9c38422213e30bff212b30c360d26f9a02136422 \
  aabbccddeeff00112233445566778899aabbccdd
```

## UDP Client

### UDP Announce

```bash
cargo run -p torrust-tracker-client --bin tracker_client -- udp announce <host:port> <info_hash>
```

**Example**:

```bash
cargo run -p torrust-tracker-client --bin tracker_client -- udp announce 127.0.0.1:6969 9c38422213e30bff212b30c360d26f9a02136422
```

**Response** (JSON):

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

**Options**:

| Option           | Type     | Description                               |
| ---------------- | -------- | ----------------------------------------- |
| `--event`        | enum     | `none`, `started`, `stopped`, `completed` |
| `--uploaded`     | u64      | Bytes uploaded                            |
| `--downloaded`   | u64      | Bytes downloaded                          |
| `--left`         | u64      | Bytes left to download                    |
| `--port`         | u16      | Client port (non-zero)                    |
| `--ip-address`   | Ipv4Addr | Peer IPv4 address                         |
| `--peer-id`      | hex      | 20-byte hex-encoded peer ID               |
| `--key`          | i32      | Client key                                |
| `--peers-wanted` | i32      | Number of peers wanted                    |
| `--format`       | enum     | `json` (default) or `text`                |

### UDP Scrape

```bash
cargo run -p torrust-tracker-client --bin tracker_client -- udp scrape <host:port> <info_hash> [info_hash...]
```

**Example**:

```bash
cargo run -p torrust-tracker-client --bin tracker_client -- udp scrape 127.0.0.1:6969 9c38422213e30bff212b30c360d26f9a02136422
```

**Response** (JSON):

```json
{
  "Scrape": {
    "transaction_id": -888840697,
    "torrent_stats": [{ "seeders": 1, "completed": 0, "leechers": 0 }]
  }
}
```

## Output Formats

All commands support `--format`:

| Value  | Description                          |
| ------ | ------------------------------------ |
| `json` | Compact JSON (default)               |
| `text` | Pretty-printed JSON (human-readable) |

## Tracker Checker

The `check` subcommand runs health checks against configured trackers:

```bash
TORRUST_CHECKER_CONFIG='{
    "udp_trackers": ["127.0.0.1:6969"],
    "http_trackers": ["http://127.0.0.1:7070"],
    "health_checks": ["http://127.0.0.1:1212/api/health_check"]
}' cargo run -p torrust-tracker-client --bin tracker_client -- check
```

## Verification Workflow

A typical manual verification workflow:

1. **Start the tracker**:

   ```bash
   cargo run
   ```

2. **Send an HTTP announce**:

   ```bash
   cargo run -p torrust-tracker-client --bin tracker_client -- http announce http://127.0.0.1:7070 9c38422213e30bff212b30c360d26f9a02136422
   ```

   Expected: JSON response with `complete`, `incomplete`, `interval`, `min interval`, `peers`.

3. **Send an HTTP scrape**:

   ```bash
   cargo run -p torrust-tracker-client --bin tracker_client -- http scrape http://127.0.0.1:7070 9c38422213e30bff212b30c360d26f9a02136422
   ```

   Expected: JSON response with per-infohash stats.

4. **Send a UDP announce**:

   ```bash
   cargo run -p torrust-tracker-client --bin tracker_client -- udp announce 127.0.0.1:6969 9c38422213e30bff212b30c360d26f9a02136422
   ```

   Expected: JSON response with `AnnounceIpv4` containing `transaction_id`, `announce_interval`, `leechers`, `seeders`, `peers`.

5. **Send a UDP scrape**:

   ```bash
   cargo run -p torrust-tracker-client --bin tracker_client -- udp scrape 127.0.0.1:6969 9c38422213e30bff212b30c360d26f9a02136422
   ```

   Expected: JSON response with `Scrape` containing `transaction_id` and `torrent_stats`.

## Troubleshooting

### "no bin target named `tracker_client`"

Use the full package specification:

```bash
cargo run -p torrust-tracker-client --bin tracker_client -- ...
```

Not:

```bash
cargo run --bin tracker_client -- ...
```

### Tracker not responding

Ensure the tracker is running (`cargo run` in another terminal). Check the default ports:

- UDP tracker 1: `6969`
- UDP tracker 2: `6868`
- HTTP tracker 1: `7070`
- HTTP tracker 2: `7171`

### Port already in use

If the tracker fails to start because ports are in use, kill any lingering processes:

```bash
pkill -f "target/debug/torrust-tracker"
```
