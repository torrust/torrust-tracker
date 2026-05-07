# Issue #1561 — HTTP Tracker Client: Avoid Duplicating the `announce` Suffix

## Overview

The HTTP tracker client currently assumes the user passes a tracker base URL
without the request path suffix. When the user provides a full tracker URL that
already ends in `/announce`, the client appends another `announce` segment and
sends the request to an invalid endpoint.

This is a bug in the HTTP client URL construction logic. The client should
accept both forms:

- base URL, for example `https://tracker.torrust-demo.com/`
- full announce URL, for example `https://tracker.torrust-demo.com/announce`

The `/announce` suffix is common in public tracker lists (for example
newtrackon), but not guaranteed by protocol-level requirements. The client
should therefore support a mixed strategy:

- If the input URL path is empty (domain only) or exactly `/`, append
  `/announce`.
- If the input URL already contains a path segment, keep it as provided.

- GitHub issue: <https://github.com/torrust/torrust-tracker/issues/1561>
- Parent EPIC: <https://github.com/torrust/torrust-tracker/issues/669>

## Motivation

A user naturally expects the HTTP client to accept the same long-form tracker
URL that appears in torrent metadata and public tracker lists.

Today this command fails:

```text
cargo run -p torrust-tracker-client --bin http_tracker_client announce \
  https://tracker.torrust-demo.com/announce \
  000620bbc6c52d5a96d98f6c0f1dfa523a40df82
```

Because the final request URL becomes:

```text
https://tracker.torrust-demo.com/announceannounce?...query...
```

That produces a `404 Not Found` even though the provided tracker URL is valid.

## Current Behaviour

The console binary parses the user input URL and passes it unchanged into the
package client in `console/tracker-client/src/console/clients/http/app.rs`.

The actual bug is in
`packages/tracker-client/src/http/client/mod.rs`, where request URLs are built
by plain string concatenation:

```rust
fn build_announce_path_and_query(&self, query: &announce::Query) -> String {
    format!("{}?{query}", self.build_path("announce"))
}

fn build_url(&self, path: &str) -> String {
    let base_url = self.base_url();
    format!("{base_url}{path}")
}
```

If `base_url` already ends in `announce`, the client still appends `announce`
again. The same risk exists for `scrape` if a full scrape URL is passed.

## Proposed Behaviour

The HTTP client should normalize the request URL before sending requests.

Expected accepted inputs for announce:

- `https://tracker.torrust-demo.com`
- `https://tracker.torrust-demo.com/`
- `https://tracker.torrust-demo.com/announce`
- `https://tracker.torrust-demo.com/custom-tracker-endpoint`

Expected final request path for announce:

- exactly one effective endpoint path, resolved by the rule below

Path resolution rule for `announce`:

- Input path empty or `/` -> resolve to `/announce`
- Input path non-empty (for example `/announce`, `/foo`, `/foo/bar`) -> keep it
  unchanged

The client should not rely on callers pre-trimming or pre-normalizing the URL.

Scope note: this issue is about tracker protocol endpoints (`announce` and
`scrape`). The `health_check` endpoint is out of scope.

## Goals

- [ ] Accept both bare tracker base URLs and full announce URLs in the HTTP
      client
- [ ] Append `/announce` only for bare URLs (`host` or `host/`)
- [ ] Keep provided path unchanged when a non-empty path already exists
- [ ] Avoid duplicating the `announce` path suffix in the final request URL
- [ ] Keep authenticated path handling working, including URLs that append the
      authentication key after the endpoint path
- [ ] Preserve existing behaviour for valid base URLs
- [ ] Add tests covering the supported input forms
- [ ] Keep `health_check` behaviour unchanged in this issue
- [ ] `linter all` exits with code `0`
- [ ] `cargo machete` reports no unused dependencies
- [ ] Existing tests pass

## Implementation Plan

### Task 1: Replace string concatenation with URL-aware path building

In `packages/tracker-client/src/http/client/mod.rs`, stop constructing request
URLs through `format!("{base_url}{path}")`.

Instead, add a helper that derives a normalized endpoint URL from the parsed
`reqwest::Url`, for example by:

- inspecting the current path segments
- detecting whether the last segment is already `announce` or `scrape`
- replacing or appending path segments as needed
- preserving scheme, host, port, and query construction

The key rule is: the final URL must contain the endpoint suffix exactly once.

### Task 2: Apply base-URL detection for announce

For announce requests:

- If the input URL path is empty or `/`, append `announce`
- Otherwise, keep the original path unchanged

Do not append `announce` when any path segment already exists.

### Task 3: Preserve authenticated endpoint support

`build_path()` currently appends the optional authentication key as:

```rust
announce/<key>
```

or

```rust
scrape/<key>
```

The normalization logic must preserve this behaviour without producing broken
paths like:

- `/announce/announce/<key>`
- `/announce/<key>/<key>`

### Task 4: Add focused unit tests for URL building

Add tests in `packages/tracker-client/src/http/client/mod.rs` covering at least:

- base URL without trailing slash + announce
- base URL with trailing slash + announce
- full `/announce` URL + announce
- full custom path URL + announce (path unchanged)
- authenticated announce path with a full `/announce` base URL

The tests should assert the exact final URL string.

### Task 5: Update HTTP client docs/examples

Update the module docs in
`console/tracker-client/src/console/clients/http/app.rs` or package docs so the
accepted URL forms are explicit.

### Task 6: Keep `health_check` out of scope

Do not change `health_check` behavior as part of this bug fix. If endpoint
normalization is later generalized to all methods, that should be handled in a
separate issue with dedicated tests.

## Acceptance Criteria

- [ ] Passing `https://tracker.torrust-demo.com` to the announce command sends
      the request to `/announce`
- [ ] Passing `https://tracker.torrust-demo.com/announce` to the announce
      command also sends the request to `/announce`
- [ ] Passing a URL with a non-empty path (for example `/foo`) keeps `/foo`
      unchanged and does not append `announce`
- [ ] Authenticated requests still generate correct URLs
- [ ] No duplicated endpoint suffix appears in final request URLs
- [ ] `linter all` exits with code `0`
- [ ] `cargo machete` reports no unused dependencies
- [ ] Existing tests pass

## Key Files

| File                                                     | Role                                      |
| -------------------------------------------------------- | ----------------------------------------- |
| `packages/tracker-client/src/http/client/mod.rs`         | Main bug location and URL normalization   |
| `console/tracker-client/src/console/clients/http/app.rs` | Console entry point that accepts user URL |

## References

- Parent EPIC: <https://github.com/torrust/torrust-tracker/issues/669>
- GitHub issue: <https://github.com/torrust/torrust-tracker/issues/1561>
- HTTP client package: `packages/tracker-client/src/http/client/`
- HTTP client console app: `console/tracker-client/src/console/clients/http/app.rs`
