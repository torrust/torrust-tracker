# Issue #671 — UDP Tracker Client: Print Unrecognized Responses

## Overview

When the UDP tracker client sends a request and receives bytes it cannot parse into a known
`Response` variant, the error currently surfaces as a deeply-nested `anyhow` chain that includes
the raw bytes in Rust `Debug` format. The result is technically correct but unreadable for the
developer trying to debug what the remote tracker sent.

The goal of this issue is to ensure that whenever a UDP response cannot be deserialized, the CLI
prints a clean, human-readable message that includes the raw bytes in decimal array notation,
matching the style expected by the caller:

```text
Error: Unrecognized UDP tracker response. Expected a valid UDP response, got: [0, 0, 0, 1]
```

- GitHub issue: <https://github.com/torrust/torrust-tracker/issues/671>
- Parent EPIC: <https://github.com/torrust/torrust-tracker/issues/669>
- Related: <https://github.com/torrust/torrust-tracker/issues/672> (same feature for HTTP client)

## Motivation

When testing against real-world public trackers (e.g. from <https://newtrackon.com/>), some
trackers respond with bytes that do not conform to the BEP 15 wire format. The developer should
be able to see those bytes immediately to understand what the tracker sent, without reaching for
`RUST_BACKTRACE=1` or a network sniffer.

## Current Behaviour

The error chain is constructed correctly — `Error::UnableToParseResponse` in
`packages/tracker-client/src/udp/mod.rs` already carries the raw `Vec<u8>` — but its `Display`
output is in `Debug` format:

```text
Error: Failed to receive a announce response, with error: Failed to parse response:
[0, 0, 0, 1], with error: failed to fill whole buffer
```

This is the result of the `thiserror` `#[error]` attribute using `{response:?}` rather than a
deliberately formatted byte list. The nesting also makes it hard to see which part is the raw
payload.

## Key Observation: Infrastructure Is Already in Place

The underlying `UdpTrackerClient::receive()` in
`packages/tracker-client/src/udp/client.rs` already returns
`Result<Response, Error>` where the `Err` variant carries the raw bytes:

```rust
Response::parse_bytes(&response, true)
    .map_err(|e| Error::UnableToParseResponse { err: e.into(), response })
```

No changes to `UdpClient` or `UdpTrackerClient` are required. The improvement is
**purely at the display/application layer**.

## Proposed Output

On a parse error the CLI should print to stderr and exit non-zero:

```text
Error: Unrecognized UDP tracker response. Expected a valid UDP response, got: [0, 0, 0, 1]
```

The decimal byte array (as formatted by `Vec<u8>`'s `Debug`) is acceptable; a hex representation
is a quality-of-life improvement but not required for the initial fix.

## Goals

- [ ] When a UDP response cannot be parsed, the CLI prints the raw bytes in a clean, readable
      message instead of a deeply-nested Rust error chain
- [ ] The exit code is non-zero on parse failure (already true via `anyhow` propagation;
      must not regress)
- [ ] Normal (valid) responses are unaffected
- [ ] `linter all` exits with code `0`
- [ ] `cargo machete` reports no unused dependencies
- [ ] All existing tests pass

## Implementation Plan

### Task 1: Improve the `UnableToParseResponse` error message

In `packages/tracker-client/src/udp/mod.rs`, update the `#[error(...)]` attribute on
`UnableToParseResponse` to emit a clean, developer-friendly message:

```rust
#[error("Unrecognized UDP tracker response. Expected a valid UDP response, got: {response:?}")]
UnableToParseResponse { err: Arc<std::io::Error>, response: Vec<u8> },
```

This change alone makes the top-level error message readable, because the wrapping
`UnableToReceiveAnnounceResponse` simply delegates to its inner `err`'s `Display`.

### Task 2: Simplify the wrapper error messages (optional polish)

In `console/tracker-client/src/console/clients/udp/mod.rs`, the wrapper variants such as
`UnableToReceiveAnnounceResponse` add a prefix that can obscure the root cause. Consider
simplifying them so the most important part (the bytes) is visible at the top level:

```rust
#[error("Failed to receive an announce response: {err}")]
UnableToReceiveAnnounceResponse { err: udp::Error },
```

### Task 3: Update the module doc comment in `app.rs`

In `console/tracker-client/src/console/clients/udp/app.rs`, add an example showing what
the error output looks like when an unrecognized response is received.

## Acceptance Criteria

- [ ] Running the client against a tracker that returns an invalid packet produces output
      matching:
      `Error: Unrecognized UDP tracker response. Expected a valid UDP response, got: [...]`
- [ ] Running the client against a well-behaved tracker still prints the JSON response and
      exits `0`
- [ ] `linter all` exits with code `0`
- [ ] `cargo machete` reports no unused dependencies
- [ ] All existing tests pass

## Key Files

| File                                                        | Role                                                    |
| ----------------------------------------------------------- | ------------------------------------------------------- |
| `packages/tracker-client/src/udp/mod.rs`                    | `Error` enum — improve `UnableToParseResponse` message  |
| `console/tracker-client/src/console/clients/udp/mod.rs`     | Wrapper `Error` enum — optional message polish          |
| `console/tracker-client/src/console/clients/udp/checker.rs` | Calls `UdpTrackerClient::receive()` — no changes needed |
| `console/tracker-client/src/console/clients/udp/app.rs`     | CLI entry point — update doc comment                    |
| `packages/tracker-client/src/udp/client.rs`                 | `UdpTrackerClient::receive()` — no changes needed       |

## References

- Parent EPIC: <https://github.com/torrust/torrust-tracker/issues/669>
- Related HTTP issue: <https://github.com/torrust/torrust-tracker/issues/672>
- Comment with context: <https://github.com/torrust/torrust-tracker/pull/814#issuecomment-2093272796>
- BEP 15 (UDP Tracker Protocol): <https://www.bittorrent.org/beps/bep_0015.html>
- List of public UDP trackers: <https://newtrackon.com/>
