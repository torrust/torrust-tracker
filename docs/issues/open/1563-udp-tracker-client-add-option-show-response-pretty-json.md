# Issue #1563 — UDP Tracker Client: Add Option to Show Response in Pretty JSON

## Overview

The UDP tracker client already prints pretty JSON by default. This issue adds an
explicit `--format` option so output style is user-controlled and aligned with
the HTTP client UX.

This spec intentionally changes the default to `compact` for consistency with
HTTP and better machine-oriented ergonomics.

- GitHub issue: <https://github.com/torrust/torrust-tracker/issues/1563>
- Parent EPIC: <https://github.com/torrust/torrust-tracker/issues/669>
- Related: <https://github.com/torrust/torrust-tracker/issues/1562>

## Motivation

The issue request asks for native pretty JSON output without piping to `jq`:

```text
cargo run -p torrust-tracker-client --bin udp_tracker_client announce \
  udp://tracker.torrust-demo.com:6969/announce \
  000620bbc6c52d5a96d98f6c0f1dfa523a40df82 | jq
```

In the current codebase, this output is already pretty-printed. The missing
piece is an explicit formatting option and parity with HTTP client CLI options.

## Current Behaviour

In `console/tracker-client/src/console/clients/udp/responses/json.rs`,
`ToJson::to_json_string()` always calls:

- `serde_json::to_string_pretty(...)`

So there is no way to request compact output, and no `--format` flag in
`console/tracker-client/src/console/clients/udp/app.rs`.

## Proposed Behaviour

Add `--format` to UDP commands with values:

- `compact` (default)
- `pretty`

Formatting applies to both typed responses and fallback JSON generated for
unrecognized responses (from #671 style behavior). Raw-byte fallback remains
plain text and is not reformatted.

Defaulting to `compact` is intentional because:

- It is better for shell pipelines and machine parsing.
- It keeps logs and CI output smaller and easier to scan.
- It aligns default behavior across HTTP and UDP clients.

Even though this changes current UDP default behavior, it is acceptable at this
stage because the client is still internal and not yet published.

Examples:

```text
# New default behavior
cargo run -p torrust-tracker-client --bin udp_tracker_client announce \
  udp://tracker.torrust-demo.com:6969/announce \
  000620bbc6c52d5a96d98f6c0f1dfa523a40df82
```

```text
# New explicit pretty behavior
cargo run -p torrust-tracker-client --bin udp_tracker_client announce \
  udp://tracker.torrust-demo.com:6969/announce \
  000620bbc6c52d5a96d98f6c0f1dfa523a40df82 \
  --format pretty
```

```text
# Explicit compact behavior
cargo run -p torrust-tracker-client --bin udp_tracker_client announce \
  udp://tracker.torrust-demo.com:6969/announce \
  000620bbc6c52d5a96d98f6c0f1dfa523a40df82 \
  --format compact
```

## Goals

- [x] Add a `--format` option to UDP `announce` and `scrape`
- [x] Change default output to `compact`
- [x] Support `pretty` output for human-readable inspection
- [x] Keep response DTO conversion unchanged
- [x] Update CLI docs/examples
- [x] `linter all` exits with code `0`
- [x] `cargo machete` reports no unused dependencies
- [x] Existing tests keep passing

## Implementation Plan

### Task 1: Define output format enum for UDP app

In `console/tracker-client/src/console/clients/udp/app.rs`:

- Add `OutputFormat` enum deriving `clap::ValueEnum`
- Values: `Compact`, `Pretty`
- Default to `Compact`

### Task 2: Add `--format` argument to subcommands

Extend both `Command::Announce` and `Command::Scrape` with:

- `format: OutputFormat`

### Task 3: Make JSON serializer format-aware

In `console/tracker-client/src/console/clients/udp/responses/json.rs`:

- Replace `to_json_string()` with one that accepts format, or add a new method
  such as `to_json_string_with_format(format)`
- Use:
  - `serde_json::to_string(...)` for `Compact`
  - `serde_json::to_string_pretty(...)` for `Pretty`

### Task 4: Thread format through command execution

In `udp/app.rs`, pass selected format to response serialization before printing.

### Task 5: Update module docs

Update examples to show both default and explicit `--format pretty` usage.

## Acceptance Criteria

- [x] Running UDP `announce --format pretty` prints multiline JSON
- [x] Running UDP `announce --format compact` prints single-line JSON
- [x] Running UDP `scrape --format pretty` prints multiline JSON
- [x] Omitting `--format` produces compact single-line JSON
- [ ] When fallback JSON is produced, `--format pretty` prints indented JSON and
      default output remains compact
- [x] Invalid format values are rejected by clap with usage guidance
- [x] `linter all` exits with code `0`
- [x] `cargo machete` reports no unused dependencies
- [x] Existing tests pass

## Manual Verification

Environment used:

- Local tracker started with default development config (`tracker.development.sqlite3.toml`)
- Command target: `udp://127.0.0.1:6969/scrape`
- Info hash: `000620bbc6c52d5a96d98f6c0f1dfa523a40df82`

### Compact output

Command:

```text
./target/debug/udp_tracker_client scrape \
  udp://127.0.0.1:6969/scrape \
  000620bbc6c52d5a96d98f6c0f1dfa523a40df82 \
  --format compact
```

Captured output:

```json
{"Scrape":{"transaction_id":-888840697,"torrent_stats":[{"seeders":0,"completed":0,"leechers":0}]}}
```

### Pretty output

Command:

```text
./target/debug/udp_tracker_client scrape \
  udp://127.0.0.1:6969/scrape \
  000620bbc6c52d5a96d98f6c0f1dfa523a40df82 \
  --format pretty
```

Captured output:

```json
{
  "Scrape": {
    "transaction_id": -888840697,
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

### Additional checks

Command:

```text
./target/debug/udp_tracker_client announce \
  udp://127.0.0.1:6969/announce \
  000620bbc6c52d5a96d98f6c0f1dfa523a40df82 \
  --format compact
```

Captured output:

```json
{"AnnounceIpv4":{"transaction_id":-888840697,"announce_interval":120,"leechers":0,"seeders":1,"peers":[]}}
```

Command:

```text
./target/debug/udp_tracker_client announce \
  udp://127.0.0.1:6969/announce \
  000620bbc6c52d5a96d98f6c0f1dfa523a40df82 \
  --format pretty
```

Captured output:

```json
{
  "AnnounceIpv4": {
    "transaction_id": -888840697,
    "announce_interval": 120,
    "leechers": 0,
    "seeders": 2,
    "peers": ["0.0.0.0:46251"]
  }
}
```

Command:

```text
./target/debug/udp_tracker_client scrape \
  udp://127.0.0.1:6969/scrape \
  000620bbc6c52d5a96d98f6c0f1dfa523a40df82
```

Captured output:

```json
{
  "Scrape": {
    "transaction_id": -888840697,
    "torrent_stats": [{ "seeders": 2, "completed": 0, "leechers": 0 }]
  }
}
```

Command:

```text
./target/debug/udp_tracker_client scrape \
  udp://127.0.0.1:6969/scrape \
  000620bbc6c52d5a96d98f6c0f1dfa523a40df82 \
  --format invalid
```

Captured output:

```text
error: invalid value 'invalid' for '--format <FORMAT>'
  [possible values: compact, pretty]

For more information, try '--help'.
```

## Key Files

| File                                                               | Role                                  |
| ------------------------------------------------------------------ | ------------------------------------- |
| `console/tracker-client/src/console/clients/udp/app.rs`            | CLI parsing and command wiring        |
| `console/tracker-client/src/console/clients/udp/responses/json.rs` | JSON serialization strategy by format |

## References

- Parent EPIC: <https://github.com/torrust/torrust-tracker/issues/669>
- Related HTTP issue: <https://github.com/torrust/torrust-tracker/issues/1562>
- UDP app source: `console/tracker-client/src/console/clients/udp/app.rs`
- UDP JSON response helper: `console/tracker-client/src/console/clients/udp/responses/json.rs`
