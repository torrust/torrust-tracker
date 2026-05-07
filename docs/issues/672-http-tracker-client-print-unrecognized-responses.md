# Issue #672 — HTTP Tracker Client: Print Unrecognized Responses in JSON

## Overview

When the HTTP tracker client's `announce` or `scrape` command receives a response body that
cannot be deserialized into the expected Rust struct, the application currently panics with
an unhelpful message. The goal of this issue is to handle that failure gracefully: instead of
panicking, the client should attempt to convert the raw bencoded payload to a generic JSON
representation and print it. If even that conversion fails, the raw bytes should be printed.

- GitHub issue: <https://github.com/torrust/torrust-tracker/issues/672>
- Parent EPIC: <https://github.com/torrust/torrust-tracker/issues/669>
- Depends on: <https://github.com/torrust/torrust-tracker/issues/673> (bencode-to-JSON
  conversion — **already resolved**: `bencode2json` crate published at
  <https://crates.io/crates/bencode2json>)
- Related: <https://github.com/torrust/torrust-tracker/issues/671> (same feature for UDP client)

## Motivation

Real-world HTTP trackers often return valid but non-standard bencoded responses. For example,
the scrape response from `open.acgnxtracker.com` omits the `downloaded` field, which is
required by the Torrust `scrape::File` struct. This causes:

```text
thread 'main' panicked at src/shared/bit_torrent/tracker/http/client/responses/scrape.rs:143:60:
called `Result::unwrap()` on an `Err` value: MissingFileField { field_name: "downloaded" }
```

When testing the client against multiple trackers (e.g. from <https://newtrackon.com/>), any
non-standard response crashes the process without showing what the tracker actually sent.

## Current Behaviour

Both `announce_command` and `scrape_command` in
`console/tracker-client/src/console/clients/http/app.rs` use `.unwrap_or_else(|_| panic!(...))`:

```rust
// announce_command:
let announce_response: Announce = serde_bencode::from_bytes(&body)
    .unwrap_or_else(|_| panic!("response body should be a valid announce response, got: \"{body:#?}\""));

// scrape_command:
let scrape_response = scrape::Response::try_from_bencoded(&body)
    .unwrap_or_else(|_| panic!("response body should be a valid scrape response, got: \"{body:#?}\""));
```

`scrape::Response::try_from_bencoded` also panics internally via
`serde_bencode::from_bytes(bytes).expect(...)`.

## Proposed Behaviour

The two-step fallback strategy:

1. **Try to deserialize into the typed struct** (existing behaviour).
2. **On failure, convert the raw bencoded bytes to generic JSON** using the `bencode2json` crate
   and print that instead.
3. **If bencode-to-JSON conversion also fails**, print the raw bytes in their debug form so the
   developer can see what was received.

Example output when the response is non-standard but valid bencode:

```json
{
  "files": {
    "<info_hash_bytes>": {
      "incomplete": 0,
      "complete": 32
    }
  }
}
```

Example output when even bencode parsing fails (raw bytes):

```text
Warning: Could not deserialize HTTP tracker response. Raw bytes: [100, 56, ...]
```

## Goals

- [ ] Replace both `panic!(...)` / `.unwrap_or_else(|_| panic!(...))` calls in `app.rs` with
      graceful fallback logic
- [ ] Remove the `panic!` inside `scrape::Response::try_from_bencoded`; change the internal
      `expect(...)` to return `Err` properly
- [ ] Add `bencode2json` as a dependency of the `torrust-tracker-client` console crate
- [ ] On deserialization failure, print the raw bencoded payload as generic JSON (via
      `bencode2json`)
- [ ] If `bencode2json` conversion also fails, print a warning with the raw byte slice
- [ ] The process exits with a non-zero exit code when the response cannot be deserialized
      (print the fallback JSON/bytes to stdout, return an `Err` from the command function)
- [ ] `linter all` exits with code `0`
- [ ] `cargo machete` reports no unused dependencies
- [ ] All existing tests pass

## Implementation Plan

### Task 1: Fix `scrape::Response::try_from_bencoded` to not panic

In `packages/tracker-client/src/http/client/responses/scrape.rs`, replace the internal
`expect(...)` with a proper `?`-based propagation so callers can handle the error:

```rust
pub fn try_from_bencoded(bytes: &[u8]) -> Result<Self, BencodeParseError> {
    let scrape_response: DeserializedResponse = serde_bencode::from_bytes(bytes)
        .map_err(|e| BencodeParseError::DeserializationError { source: e })?;
    Self::try_from(scrape_response)
}
```

A new `BencodeParseError` variant may be needed for `serde_bencode::Error`.

### Task 2: Add `bencode2json` dependency

In `console/tracker-client/Cargo.toml`, add:

```toml
bencode2json = "0.1"   # adjust to the published version
```

### Task 3: Implement the two-step fallback helper

Add a private helper in `console/tracker-client/src/console/clients/http/app.rs`:

```rust
fn bencode_to_fallback_json(body: &[u8]) -> String {
    match bencode2json::to_json(body) {
        Ok(json) => json,
        Err(_) => format!("(raw bytes) {body:?}"),
    }
}
```

### Task 4: Replace panics in `announce_command`

```rust
let body = response.bytes().await?;

match serde_bencode::from_bytes::<Announce>(&body) {
    Ok(announce_response) => {
        let json = serde_json::to_string(&announce_response)
            .context("failed to serialize announce response into JSON")?;
        println!("{json}");
        Ok(())
    }
    Err(_) => {
        let fallback = bencode_to_fallback_json(&body);
        eprintln!("Warning: Could not deserialize HTTP tracker announce response.");
        println!("{fallback}");
        Err(anyhow::anyhow!("unrecognized announce response from tracker"))
    }
}
```

### Task 5: Replace panics in `scrape_command`

Apply the same two-step fallback to `scrape_command`, replacing the current
`.unwrap_or_else(|_| panic!(...))`.

### Task 6: Update the module doc comment in `app.rs`

Add examples showing the fallback output in the module-level doc comment.

## Acceptance Criteria

- [ ] Running the client against a tracker that returns a non-standard response prints the
      response as generic JSON (via `bencode2json`) and exits non-zero
- [ ] Running the client against a tracker that returns a completely unrecognized payload
      prints a warning with the raw bytes and exits non-zero
- [ ] Running the client against the Torrust Tracker still prints the typed JSON response
      and exits `0`
- [ ] No `panic!` or `.unwrap()` in the announce or scrape command paths
- [ ] `linter all` exits with code `0`
- [ ] `cargo machete` reports no unused dependencies
- [ ] All existing tests pass

## Key Files

| File                                                          | Role                                                |
| ------------------------------------------------------------- | --------------------------------------------------- |
| `console/tracker-client/src/console/clients/http/app.rs`      | Replace panics with two-step fallback — main change |
| `packages/tracker-client/src/http/client/responses/scrape.rs` | Fix `try_from_bencoded` to not panic internally     |
| `console/tracker-client/Cargo.toml`                           | Add `bencode2json` dependency                       |

## References

- Parent EPIC: <https://github.com/torrust/torrust-tracker/issues/669>
- Depends on: <https://github.com/torrust/torrust-tracker/issues/673>
  (bencode-to-JSON, resolved — `bencode2json` on crates.io)
- Related UDP issue: <https://github.com/torrust/torrust-tracker/issues/671>
- `bencode2json` crate: <https://crates.io/crates/bencode2json>
- `bencode2json` source: <https://github.com/torrust/bencode2json>
- BitTorrent scrape spec: <https://www.bittorrent.org/beps/bep_0048.html>
- List of public HTTP trackers: <https://newtrackon.com/>
