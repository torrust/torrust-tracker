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
thread 'main' panicked at packages/tracker-client/src/http/client/responses/scrape.rs:143:60:
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

The scrape parser path also contains nested `.unwrap()` calls while iterating
decoded file dictionaries. Those must be removed from reachable runtime paths.

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
- [ ] Remove panic/unwrap usage from the scrape decode path:
      `expect(...)` in `try_from_bencoded` and nested `.unwrap()` calls in
      parser helpers
- [ ] Add `bencode2json` as a dependency of the `torrust-tracker-client` console crate
- [ ] On deserialization failure, print the raw bencoded payload as generic JSON (via
      `bencode2json`)
- [ ] If `bencode2json` conversion also fails, print a warning with the raw byte slice
- [ ] The process exits with a non-zero exit code when the response cannot be deserialized
      (print the fallback JSON/bytes to stdout, return an `Err` from the command function)
- [ ] Fallback JSON output is compact by default in this issue; once `--format`
      is introduced in #1562, fallback JSON must respect the selected format
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

Also replace nested `.unwrap()` calls in scrape parsing helpers with proper
error propagation into `BencodeParseError`.

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

## Manual Verification

This section is a living test plan and result log for validating fallback behavior against real
HTTP trackers and a forced malformed local response.

### Goal

- Confirm normal typed JSON output for well-behaved HTTP trackers.
- Confirm non-standard but valid bencoded responses are printed as generic JSON and exit non-zero.
- Confirm completely unrecognized payloads print raw bytes and exit non-zero.

### Step 1: Collect stable HTTP trackers

- Query the newtrackon HTTP endpoint: <https://newtrackon.com/api#get-/http>
- Record the sampled tracker list used for this verification run.
- Note date/time and any filtering criteria.

### Step 2: Probe sampled public trackers

- Run `announce` and/or `scrape` against sampled trackers.
- Record whether each response is typed JSON or fallback JSON.
- Record exit code for each probe.

### Step 3: Record results

Use this table to track outcomes:

| Tracker     | Command     | Output mode | Exit code   | Notes       |
| ----------- | ----------- | ----------- | ----------- | ----------- |
| _(pending)_ | _(pending)_ | _(pending)_ | _(pending)_ | _(pending)_ |

### Step 4: Local malformed-response verification

If public trackers do not produce an unrecognized payload, force one locally to verify the raw
bytes fallback:

1. Apply a temporary local patch to the HTTP tracker response path to return malformed payload bytes.
2. Run the tracker locally.
3. Run `http_tracker_client announce` or `scrape` against the local tracker.
4. Verify fallback prints raw bytes and command exits non-zero.

Record command lines and observed output in this section. If a temporary local patch was used,
state explicitly that it is not part of the committed implementation.

## Acceptance Criteria

- [ ] Running the client against a tracker that returns a non-standard response prints the
      response as generic JSON (via `bencode2json`) and exits non-zero
- [ ] Running the client against a tracker that returns a completely unrecognized payload
      prints a warning with the raw bytes and exits non-zero
- [ ] Running the client against the Torrust Tracker still prints the typed JSON response
      and exits `0`
- [ ] No `panic!` or `.unwrap()` in the announce or scrape command paths
- [ ] No reachable panic/unwrap remains in the scrape decoding path
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
