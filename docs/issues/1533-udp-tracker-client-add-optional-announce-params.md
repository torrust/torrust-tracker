# Issue #1533 — UDP Tracker Client: Add Optional Parameters to Announce Command

## Overview

The UDP Tracker client's `announce` sub-command accepts only two arguments: the tracker socket
address and the `info_hash`. All other announce request parameters (`event`, `uploaded`,
`downloaded`, `left`, `port`, `peer_id`, `ip_address`, `key`, `peers_wanted`) are hard-coded
with default values directly inside `checker::Client::send_announce_request()`.

This is the UDP counterpart of issue
[#1532](https://github.com/torrust/torrust-tracker/issues/1532), which adds the same capability
to the HTTP Tracker client.

- GitHub issue: <https://github.com/torrust/torrust-tracker/issues/1533>
- Parent EPIC: <https://github.com/torrust/torrust-tracker/issues/669>
- Related: <https://github.com/torrust/torrust-tracker/issues/1532> (same feature for HTTP client)

## Motivation

Same motivation as #1532. The `downloads` counter only increments when a peer transitions from
`started` to `completed`. Without control over the `event` field at the command line, testing
this behaviour requires source-level edits, recompilation, and manual repetition.

## Current Behaviour

```console
cargo run -p torrust-tracker-client --bin udp_tracker_client \
  announce 127.0.0.1:6969 9c38422213e30bff212b30c360d26f9a02136422
```

All announce request fields other than `info_hash` use hard-coded defaults (from
`console/tracker-client/src/console/clients/udp/checker.rs`):

| Parameter          | Hard-coded default           |
| ------------------ | ---------------------------- |
| `event`            | `AnnounceEvent::Started`     |
| `bytes_uploaded`   | `0`                          |
| `bytes_downloaded` | `0`                          |
| `bytes_left`       | `0`                          |
| `port`             | socket's local port (random) |
| `ip_address`       | `0.0.0.0` (unspecified)      |
| `peer_id`          | `-qB00000000000000001`       |
| `key`              | `0`                          |
| `peers_wanted`     | `1`                          |

## Proposed CLI

All announce request parameters become optional flags. When omitted, the existing defaults apply.

```console
cargo run -p torrust-tracker-client --bin udp_tracker_client announce \
  127.0.0.1:6969 443c7602b4fde83d1154d6d9da48808418b181b6 \
  --event completed \
  --uploaded 1234 \
  --downloaded 5678 \
  --left 0 \
  --port 6881 \
  --ip-address 10.0.0.1 \
  --peer-id "-RC0000000000000001" \
  --key 42 \
  --peers-wanted 50
```

Supported `--event` values: `none`, `completed`, `started`, `stopped` (matching
`bittorrent_udp_tracker_protocol::AnnounceEvent` variants, case-insensitive).

`--peer-id` input contract:

- Accept a 20-character ASCII value.
- Reject any value that is not exactly 20 bytes.
- Surface validation errors as CLI argument errors.

## Goals

- [ ] Add optional CLI flags to the `Announce` variant in
      `console/tracker-client/src/console/clients/udp/app.rs`:
      `--event`, `--uploaded`, `--downloaded`, `--left`, `--port`, `--ip-address`,
      `--peer-id`, `--key`, `--peers-wanted`
- [ ] Thread the optional values from the CLI into `handle_announce` and then into
      `checker::Client::send_announce_request()`
- [ ] Add `clap::ValueEnum` (or `FromStr`) for `AnnounceEvent` so it can be parsed from the
      command line — implement directly on the in-house type or introduce a thin wrapper in
      the CLI layer for clean separation of concerns
- [ ] Defaults remain unchanged when a flag is omitted
- [ ] Pass `linter all` and `cargo machete` with zero warnings
- [ ] Update the module-level doc comment in `app.rs` with new usage examples

## Implementation Plan

### Task 1: Add `clap` parsing for `AnnounceEvent`

`AnnounceEvent` is now an in-house type defined in `packages/udp-protocol/src/announce.rs`
(re-exported by `bittorrent_udp_tracker_protocol`), so the foreign-trait constraint no longer
applies. Two implementation paths are available:

- Implement `clap::ValueEnum` directly on `AnnounceEvent` in `packages/udp-protocol` by
  adding `clap` as an optional feature-gated dependency there.
- Introduce a thin `CliAnnounceEvent` wrapper enum in the CLI crate that implements
  `clap::ValueEnum`, then map it to `AnnounceEvent` when building the request. This keeps
  `clap` out of the protocol crate and preserves clean separation of concerns.

The wrapper approach is recommended to avoid leaking CLI concerns into the protocol layer.

- [ ] Choose and implement one of the above in the CLI layer
      (`console/tracker-client/src/console/clients/udp/`)

### Task 2: Extend the `Announce` sub-command struct

In `console/tracker-client/src/console/clients/udp/app.rs`:

- [ ] Change the `Announce` variant of the `Command` enum to carry optional fields:

```rust
Announce {
    #[arg(value_parser = parse_socket_addr)]
    tracker_socket_addr: SocketAddr,
    #[arg(value_parser = parse_info_hash)]
    info_hash: TorrustInfoHash,
    #[arg(long)]
    event: Option<CliAnnounceEvent>,
    #[arg(long)]
    uploaded: Option<i64>,
    #[arg(long)]
    downloaded: Option<i64>,
    #[arg(long)]
    left: Option<i64>,
    #[arg(long)]
    port: Option<u16>,
    #[arg(long = "ip-address")]
    ip_address: Option<Ipv4Addr>,
    #[arg(long = "peer-id")]
    peer_id: Option<String>,
    #[arg(long)]
    key: Option<i32>,
    #[arg(long = "peers-wanted")]
    peers_wanted: Option<i32>,
}
```

### Task 3: Thread optional values through `handle_announce`

- [ ] Update `handle_announce` to accept the new optional parameters and pass them to
      `checker::Client::send_announce_request()`
- [ ] Update `send_announce_request` in `checker.rs` to accept an optional parameter struct
      (or individual `Option` arguments) and apply overrides when `Some`
- [ ] Validate and parse `--peer-id` into `bittorrent_udp_tracker_protocol::PeerId`
- [ ] Reject negative values for `uploaded`, `downloaded`, and `left` at the CLI layer

### Task 4: Update docs

- [ ] Update the module-level doc comment in `app.rs` with the new extended usage example

## Acceptance Criteria

- [ ] Running `announce ... --event completed` sends `event=completed` in the UDP packet
- [ ] Running `announce ...` without flags behaves exactly as today (defaults unchanged)
- [ ] `linter all` exits with code `0`
- [ ] `cargo machete` reports no unused dependencies
- [ ] All existing tests pass

## Key Files

| File                                                        | Role                                               |
| ----------------------------------------------------------- | -------------------------------------------------- |
| `console/tracker-client/src/console/clients/udp/app.rs`     | CLI entry point — add flags here                   |
| `console/tracker-client/src/console/clients/udp/checker.rs` | `send_announce_request` — propagate overrides here |

## References

- Parent EPIC: <https://github.com/torrust/torrust-tracker/issues/669>
- Related HTTP issue: <https://github.com/torrust/torrust-tracker/issues/1532>
- `bittorrent_udp_tracker_protocol::AnnounceEvent`: `packages/udp-protocol/src/announce.rs`
- `bittorrent_peer_id::PeerId`: `packages/peer-id/src/peer_id.rs`
- UDP tracker protocol spec (BEP 15): <https://www.bittorrent.org/beps/bep_0015.html>
