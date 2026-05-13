# Issue #1562 — HTTP Tracker Client: Add Option to Show Response in Pretty JSON

## Overview

The HTTP tracker client currently prints JSON as a single compact line.
Developers often pipe output to `jq` to make it readable.

This issue adds a CLI output formatting option so users can request pretty JSON
without external tools.

- GitHub issue: <https://github.com/torrust/torrust-tracker/issues/1562>
- Parent EPIC: <https://github.com/torrust/torrust-tracker/issues/669>
- Related: <https://github.com/torrust/torrust-tracker/issues/1563>

## Motivation

A common workflow is:

```text
cargo run -p torrust-tracker-client --bin http_tracker_client announce \
  https://tracker.torrust-demo.com \
  000620bbc6c52d5a96d98f6c0f1dfa523a40df82 | jq
```

Needing `jq` is not ideal for quick local debugging, CI scripts, or machines
where the tool is not installed.

## Current Behaviour

In `console/tracker-client/src/console/clients/http/app.rs`, both
`announce_command` and `scrape_command` serialize with:

- `serde_json::to_string(...)`

So output is compact JSON only. There is no output-format CLI option.

## Proposed Behaviour

Add `--format` to HTTP commands with the following values:

- `compact` (default)
- `pretty`

Formatting applies to both typed responses and fallback JSON generated for
unrecognized responses (from #672). Raw-byte fallback remains plain text and is
not reformatted.

Defaulting to `compact` is intentional because:

- It is better for shell pipelines and machine parsing.
- It keeps logs and CI output smaller and easier to scan.
- It provides a consistent default that can be shared by both HTTP and UDP
  clients.

Examples:

```text
# Existing behavior (still default)
cargo run -p torrust-tracker-client --bin http_tracker_client announce \
  https://tracker.torrust-demo.com \
  000620bbc6c52d5a96d98f6c0f1dfa523a40df82
```

```text
# New behavior
cargo run -p torrust-tracker-client --bin http_tracker_client announce \
  https://tracker.torrust-demo.com \
  000620bbc6c52d5a96d98f6c0f1dfa523a40df82 \
  --format pretty
```

## Goals

- [ ] Add a `--format` option to HTTP `announce` and `scrape`
- [ ] Keep default output as `compact` for script and CI friendliness
- [ ] Support `pretty` output using `serde_json::to_string_pretty`
- [ ] Update CLI docs/examples for both commands
- [ ] `linter all` exits with code `0`
- [ ] `cargo machete` reports no unused dependencies
- [ ] Existing tests keep passing

## Implementation Plan

### Task 1: Define output format enum

In `console/tracker-client/src/console/clients/http/app.rs`:

- Add a small `OutputFormat` enum deriving `clap::ValueEnum`
- Values: `Compact`, `Pretty`

### Task 2: Add `--format` to CLI subcommands

Extend both `Command::Announce` and `Command::Scrape` variants with:

- `format: OutputFormat`

Use clap defaults so current command lines remain valid and default to compact.

### Task 3: Centralize JSON serialization helper

Add helper:

- `fn serialize_json<T: serde::Serialize>(value: &T, format: OutputFormat) -> anyhow::Result<String>`

Use:

- `serde_json::to_string` for `Compact`
- `serde_json::to_string_pretty` for `Pretty`

### Task 4: Wire format through command handlers

Pass selected format from the parsed subcommand into:

- `announce_command`
- `scrape_command`

Replace direct `serde_json::to_string(...)` calls with the helper.

### Task 5: Update module docs

Update examples in `app.rs` module docs to include `--format pretty` usage.

## Acceptance Criteria

- [ ] `announce --format pretty` prints multiline indented JSON
- [ ] `scrape --format pretty` prints multiline indented JSON
- [ ] Omitting `--format` still produces compact single-line JSON
- [ ] When fallback JSON is produced, `--format pretty` prints indented JSON and
      default output remains compact
- [ ] Invalid format values are rejected by clap with usage guidance
- [ ] `linter all` exits with code `0`
- [ ] `cargo machete` reports no unused dependencies
- [ ] Existing tests pass

## Key Files

| File                                                     | Role                                      |
| -------------------------------------------------------- | ----------------------------------------- |
| `console/tracker-client/src/console/clients/http/app.rs` | Main CLI parsing and output serialization |

## References

- Parent EPIC: <https://github.com/torrust/torrust-tracker/issues/669>
- Related UDP issue: <https://github.com/torrust/torrust-tracker/issues/1563>
- HTTP client CLI source: `console/tracker-client/src/console/clients/http/app.rs`
