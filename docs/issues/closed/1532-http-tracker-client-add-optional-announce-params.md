---
doc-type: issue
issue-type: feature
status: done
priority: p2
github-issue: 1532
spec-path: docs/issues/closed/1532-http-tracker-client-add-optional-announce-params.md
branch: 1532-http-tracker-client-add-optional-announce-params
related-pr: null
last-updated-utc: null
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/README.md
    - console/tracker-client/
    - packages/tracker-client/
---

# Issue #1532 — HTTP Tracker Client: Add Optional Parameters to Announce Command

## Overview

The HTTP Tracker client's `announce` sub-command accepts only two arguments: the tracker URL and
the `info_hash`. All other announce query parameters (`event`, `uploaded`, `downloaded`, `left`,
`port`, `peer_addr`, `compact`, `peer_id`) are hard-coded with default values inside
`QueryBuilder::with_default_values()`.

This means that to simulate a state transition (e.g., a peer completing a download by sending
`event=completed`) a developer must edit the source, recompile, run, revert, recompile, and run
again. The goal of this issue is to make those parameters available as optional CLI flags.

- GitHub issue: <https://github.com/torrust/torrust-tracker/issues/1532>
- Parent EPIC: <https://github.com/torrust/torrust-tracker/issues/669>
- Related: <https://github.com/torrust/torrust-tracker/issues/1533> (same feature for UDP client)

## Motivation

The `downloads` counter on a tracker only increments when a peer transitions from `started` to
`completed`. Without being able to control the `event` field from the command line, testing this
behaviour requires source-level changes. An example of a test that triggered this pain:
<https://github.com/torrust/torrust-tracker/pull/1531>

## Current Behaviour

```console
cargo run -p torrust-tracker-client --bin http_tracker_client \
  announce http://127.0.0.1:7070 443c7602b4fde83d1154d6d9da48808418b181b6
```

All announce query parameters other than `info_hash` use defaults:

| Parameter    | Hard-coded default     |
| ------------ | ---------------------- |
| `event`      | `started`              |
| `uploaded`   | `0`                    |
| `downloaded` | `0`                    |
| `left`       | `0`                    |
| `port`       | `17548`                |
| `peer_addr`  | `192.168.1.88`         |
| `peer_id`    | `-qB00000000000000001` |
| `compact`    | `0` (not accepted)     |

## Proposed CLI

All announce-query parameters become optional flags. When omitted, the existing defaults apply.

```console
cargo run -p torrust-tracker-client --bin http_tracker_client announce \
  http://127.0.0.1:7070 443c7602b4fde83d1154d6d9da48808418b181b6 \
  --event completed \
  --uploaded 1234 \
  --downloaded 5678 \
  --left 0 \
  --port 6881 \
  --peer-addr 10.0.0.1 \
  '--peer-id=-RC00000000000000001' \
  --compact 1
```

Supported `--event` values: `started`, `stopped`, `completed` (case-insensitive).

`--peer-id` input contract:

- Accept a 20-character ASCII value.
- Reject any value that is not exactly 20 bytes.
- Surface validation errors as CLI argument errors.

## Goals

- [x] Add optional CLI flags to the `announce` sub-command in
      `console/tracker-client/src/console/clients/http/app.rs`:
      `--event`, `--uploaded`, `--downloaded`, `--left`, `--port`, `--peer-addr`,
      `--peer-id`, `--compact`
- [x] Parse each flag and map it into `announce::Query` values
- [x] Extend `QueryBuilder` with missing setters for
      `event`, `uploaded`, `downloaded`, `left`, and `port`
- [x] Defaults remain unchanged when a flag is omitted
- [x] Add CLI parsing for `Event` in the tracker-client layer
- [x] Pass `linter all` and `cargo machete` with zero warnings
- [x] Update the module-level doc comment in `app.rs` with new usage examples

## Implementation Plan

### Task 1: Add CLI parsing for `Event`

Use a CLI-facing enum (for example `CliEvent`) in
`console/tracker-client/src/console/clients/http/app.rs` and map it into
`bittorrent_tracker_client::http::client::requests::announce::Event`.

Do not rely on `packages/http-protocol` `Event`, which is a different type and
belongs to a different layer.

- [x] Implement `clap::ValueEnum` for the CLI-facing `event` type
- [x] Add explicit mapping from CLI event type to tracker-client request `Event`

### Task 2: Extend the `Announce` sub-command struct

In `console/tracker-client/src/console/clients/http/app.rs`:

- [x] Change the `Announce` variant of the `Command` enum to carry optional fields:

```rust
Announce {
    tracker_url: String,
    info_hash: String,
    #[arg(long)]
    event: Option<CliEvent>,
    #[arg(long)]
    uploaded: Option<u64>,
    #[arg(long)]
    downloaded: Option<u64>,
    #[arg(long)]
    left: Option<u64>,
    #[arg(long)]
    port: Option<u16>,
    #[arg(long = "peer-addr")]
    peer_addr: Option<IpAddr>,
    #[arg(long = "peer-id")]
    peer_id: Option<String>,
    #[arg(long)]
    compact: Option<CliCompact>,
}
```

`CliCompact` should accept only `0` and `1` and map to
`announce::Compact::{NotAccepted, Accepted}`.

### Task 3: Thread optional values through `announce_command`

- [x] Update `announce_command` signature to accept the optional parameters
- [x] Add missing `QueryBuilder` setters in
      `packages/tracker-client/src/http/client/requests/announce.rs`
- [x] Apply each `Some(value)` to the `QueryBuilder` chain before calling `.query()`
- [x] Parse and validate `--peer-id` into `bittorrent_udp_tracker_protocol::PeerId`

### Task 4: Update docs

- [x] Update the module-level doc comment in `app.rs` with the new extended usage example

## Manual Verification

This section is for manual validation after implementation is completed. It is a test plan only.

### Setup

Start the tracker locally with default development configuration:

```bash
cargo run
```

Expected startup log excerpt:

```text
Loading extra configuration from default configuration file: `./share/default/config/tracker.development.sqlite3.toml` ...
```

### Test 1: Default Announce (backward compatibility)

Command:

```bash
cargo run -p torrust-tracker-client --bin http_tracker_client announce \
      http://127.0.0.1:7070 443c7602b4fde83d1154d6d9da48808418b181b6
```

Example output (observed with current behaviour):

```json
{
  "complete": 1,
  "incomplete": 0,
  "interval": 120,
  "min interval": 120,
  "peers": []
}
```

Expected output (JSON):

- Response is valid announce JSON
- Existing defaults are used when flags are omitted
- The command succeeds without requiring optional flags

### Test 2: Announce with All Optional Parameters

Command:

```bash
cargo run -p torrust-tracker-client --bin http_tracker_client announce \
      http://127.0.0.1:7070 443c7602b4fde83d1154d6d9da48808418b181b6 \
      --event completed \
      --uploaded 1234 \
      --downloaded 5678 \
      --left 0 \
      --port 6881 \
      --peer-addr 10.0.0.1 \
      '--peer-id=-RC00000000000000001' \
      --compact 1
```

Note: Peer-id must be exactly 20 bytes. Use `--peer-id='...'` (with equals and quotes) for peer-ids that start with a dash (e.g., `-RC0...` style).

Observed output after implementation:

```json
{
  "complete": 1,
  "incomplete": 0,
  "interval": 120,
  "min interval": 120,
  "peers": []
}
```

Expected output (JSON):

- Response is valid announce JSON
- Request is accepted and processed by the tracker
- Query includes overridden values from flags (including `event=completed`)

Observed follow-up verification:

- Scrape transitioned from
  `{"complete":0,"downloaded":0,"incomplete":1}`
  to
  `{"complete":1,"downloaded":1,"incomplete":0}`
- Global stats transitioned from
  `"seeders":0,"completed":1,"leechers":1`
  to
  `"seeders":1,"completed":2,"leechers":0`

This confirms the started -> completed transition was applied and completed/download counters increased.

### Optional Negative-Path Checks

- `--peer-id` with length different from 20 bytes should fail with a CLI argument error
- Invalid `--event` value should fail and show allowed values
- Invalid `--compact` value (not `0` or `1`) should fail with a CLI argument error
- `--port 0` should fail with a CLI argument error

## Learnings

- Exposing `--compact 1` required the client to support compact HTTP announce response decoding,
  not only compact request generation. During manual verification, the client initially panicked
  because it only attempted to deserialize the dictionary-style announce response. The final
  implementation handles both response shapes.
- Manual verification is more reliable when comparing before/after deltas instead of assuming all
  tracker counters start at zero. Tracker state may persist across runs, so scrape/global stats
  transitions are the meaningful validation signal.
- For dash-prefixed peer IDs, the most reliable CLI form is
  `--peer-id=-RC00000000000000001` (typically quoted as a whole shell argument), combined with the
  explicit 20-byte validation enforced by the client.

## Acceptance Criteria

- [x] Running `announce ... --event completed` sends `event=completed` in the query string
- [x] Running `announce ...` without flags behaves exactly as today (defaults unchanged)
- [x] `linter all` exits with code `0`
- [x] `cargo machete` reports no unused dependencies
- [x] All existing tests pass

## Key Files

| File                                                           | Role                                                              |
| -------------------------------------------------------------- | ----------------------------------------------------------------- |
| `console/tracker-client/src/console/clients/http/app.rs`       | CLI entry point — add flags here                                  |
| `packages/tracker-client/src/http/client/requests/announce.rs` | `QueryBuilder`, `Event`, `Query` — add `ValueEnum`/`FromStr` here |

## References

- Parent EPIC: <https://github.com/torrust/torrust-tracker/issues/669>
- Related UDP issue: <https://github.com/torrust/torrust-tracker/issues/1533>
- PR that motivated this issue: <https://github.com/torrust/torrust-tracker/pull/1531>
- BitTorrent tracker spec: <https://wiki.theory.org/BitTorrentSpecification#Tracker_HTTP.2FHTTPS_Protocol>
