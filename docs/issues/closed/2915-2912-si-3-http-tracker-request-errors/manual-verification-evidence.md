---
doc-type: manual-verification-evidence
issue-spec: docs/issues/closed/2915-2912-si-3-http-tracker-request-errors/ISSUE.md
last-updated-utc: "2026-11-19 11:35"
---

# Manual Verification Evidence

## Purpose

Record real, human-oriented verification of the completed behavior. This is
evidence from commands or interactions actually performed against the artifact;
do not invent commands, output, logs, or results.

## Preserving Verification Artifacts

The two tracker configurations and the request script lived in an agent scratchpad outside the
repository; they are embedded verbatim below. The captured responses and the test runs are stored
in the [`evidence/`](evidence/) folder of this issue:

- [`evidence/responses-before.txt`](evidence/responses-before.txt) and
  [`evidence/responses-after.txt`](evidence/responses-after.txt): the eleven requests and the
  raw bencoded responses, before and after the fix.
- [`evidence/tests-red-http-protocol.txt`](evidence/tests-red-http-protocol.txt),
  [`evidence/tests-red-axum-lib.txt`](evidence/tests-red-axum-lib.txt),
  [`evidence/tests-red-axum-integration.txt`](evidence/tests-red-axum-integration.txt): the red
  runs.
- [`evidence/tests-green.txt`](evidence/tests-green.txt): the green run.

In the stored output the Cargo home directory is replaced with `<cargo-home>` and thread ids are
removed; the rest of each line is verbatim.

## Environment and Prerequisites

- Date and time (UTC): 2026-11-19 11:22 to 11:35.
- Artifact under test: the `torrust-tracker` binary built from branch
  `2915-http-tracker-request-errors`. "Before": built at 11:22 from the unchanged tree of
  `develop` at `36b24a9c5`, before any edit on the branch. "After": built at 11:34 from the
  commit `fix(http-protocol): [#2915] replace LocatedError sources of info-hash and peer-id param errors`
  (`git diff --quiet HEAD` exited 0 before the build).
- Toolchain: stable Rust, `rustc 1.100.0 (0ef1bc386 2026-11-09)`, `cargo 1.100.0
  (3acc79c0c 2026-11-05)`. Formatting with the nightly Rust toolchain, `cargo 1.102.0-nightly
  (f93c676e3 2026-11-15)`. Client: `curl 8.18.0`.
- Operating system / environment: Linux 7.0.0-43-generic.
- Prerequisites and setup performed: two tracker instances started from the binary under test,
  one public (two HTTP trackers, the second behind a reverse proxy) and one private.

`public.toml`:

```toml
[metadata]
app = "torrust-tracker"
purpose = "configuration"
schema_version = "3.0.0"

[logging]
trace_filter = "info"

[core]
listed = false
private = false

[[http_trackers]]
bind_address = "127.0.0.1:17070"

[[http_trackers]]
bind_address = "127.0.0.1:17071"

[http_trackers.network]
on_reverse_proxy = true

[health_check_api]
bind_address = "127.0.0.1:17313"
```

`private.toml` (`<scratch>` is the scratch folder; the database file is deleted before each
run):

```toml
[metadata]
app = "torrust-tracker"
purpose = "configuration"
schema_version = "3.0.0"

[logging]
trace_filter = "info"

[core]
listed = false
private = true

[core.database]
driver = "sqlite3"
path = "<scratch>/private.db"

[[http_trackers]]
bind_address = "127.0.0.1:17072"

[health_check_api]
bind_address = "127.0.0.1:17314"
```

`run.sh`, run as `run.sh <tracker-binary> before` and `run.sh <tracker-binary> after`. It starts
both trackers, waits for the three HTTP ports, sends the eleven requests one after the other, and
stops the trackers with `SIGINT`:

```bash
#!/usr/bin/env bash
# Usage: run.sh <tracker-binary> <label>
set -u
DIR="$(cd "$(dirname "$0")" && pwd)"
BIN="$1"; LABEL="$2"
OUT="$DIR/$LABEL.txt"
IH='%3B%24U%04%CF%5F%11%BB%DB%E1%20%1C%EAjk%F4Z%EE%1B%C0'
PID_OK='-qB00000000000000001'
VALID="info_hash=$IH&peer_id=$PID_OK&port=17548"
rm -f "$DIR/private.db"
"$BIN" --config-toml-path "$DIR/public.toml" > "$DIR/$LABEL.public.log" 2>&1 & P1=$!
"$BIN" --config-toml-path "$DIR/private.toml" > "$DIR/$LABEL.private.log" 2>&1 & P2=$!
for port in 17070 17071 17072; do
  for _ in $(seq 1 100); do curl -s -o /dev/null "http://127.0.0.1:$port/" && break; sleep 0.1; done
done
req() { # name url [curl args...]
  local name="$1" url="$2"; shift 2
  printf '%s\n  GET %s\n  ' "$name" "${url#http://127.0.0.1}"
  curl -s "$@" "$url"; printf '\n'
}
{
  echo "date -u: $(date -u '+%Y-%m-%d %H:%M:%S')"
  req "R1 announce numwant=x" "http://127.0.0.1:17070/announce?$VALID&numwant=x"
  req "R2 announce without info_hash" "http://127.0.0.1:17070/announce?peer_id=$PID_OK&port=17548"
  req "R3 announce 19-byte peer_id" "http://127.0.0.1:17070/announce?info_hash=$IH&peer_id=-qB0000000000000000&port=17548"
  req "R4 announce 10-byte info_hash" "http://127.0.0.1:17070/announce?info_hash=0123456789&peer_id=$PID_OK&port=17548"
  req "R5 scrape 10-byte info_hash" "http://127.0.0.1:17070/scrape?info_hash=0123456789"
  req "R6 scrape without info_hash" "http://127.0.0.1:17070/scrape?foo=bar"
  req "R7 announce without query" "http://127.0.0.1:17070/announce"
  req "R8 scrape without query" "http://127.0.0.1:17070/scrape"
  req "R9 announce with unparsable query" "http://127.0.0.1:17070/announce?a=b=c"
  req "R10 private announce with malformed key" "http://127.0.0.1:17072/announce/invalid_key?$VALID"
  req "R11 reverse-proxy announce without X-Forwarded-For" "http://127.0.0.1:17071/announce?$VALID"
} > "$OUT"
kill -INT $P1 $P2; wait $P1 $P2 2>/dev/null
cat "$OUT"
```

A first "before" run at 11:22:33 got no response to R10, because both instances tried to bind the
default health check API port; the `[health_check_api]` sections above were added and the run
repeated at 11:22:40. That repeated run is the recorded one.

## Analysis

Re-checked against the tree before the first change (T1).
`grep -rn "location\|Located\|Location" packages/http-protocol/src` finds the 10 variants of the
spec's table and the three `Located(err).into()` sites; the line numbers differ from the spec's
(written against an older tree) but the set is the same. The `axum-http-server` extractors build
`MissingParams` (announce and scrape, `:66`), `InvalidKeyFormat` (`authentication_key.rs:109`,
`:118`) and `CannotExtractKeyParam` (`:122`).

Two test-only users outside the spec's file list build `PeerIpResolutionError::MissingClientIp`:
`packages/http-core/src/services/announce.rs` and `scrape.rs`. They change with the variant. Five
`axum-http-server` handler tests and one contract-test helper match the old peer-IP texts and change
with the wording.

Hypothesis: each format interpolates a `location` filled with `Location::caller()` in a function
that is not `#[track_caller]`, so it prints a fixed library line; the three `LocatedError` sources
print their own location, and `torrust_info_hash::ConversionError` prints a registry path as well.

## Verification Processes

### V1 - Malformed announce (scenario M1) and the other request-parsing failures

- Goal: the `failure reason` of each malformed request names the parameter and holds no `.rs:`
  path. R1 (`numwant=x`), R2 (no `info_hash`) and R3 (19-byte `peer_id`) are scenario M1; R6 to R9
  cover the other parsing failures.
- Initial state: the two trackers above.
- Status: `DONE`

#### Steps Performed

1. Before the fix, 11:22:40 UTC: `run.sh <before-binary> before`.
2. After the fix, 11:34:47 UTC: `run.sh <after-binary> after`.

#### Observed Result

Failure reasons before (from `evidence/responses-before.txt`, bencoding framing removed):

```text
R1 Bad request. Cannot parse query params for announce request: invalid param value x for numwant in packages/http-protocol/src/v1/requests/announce.rs:654:27
R2 Bad request. Cannot parse query params for announce request: missing param info_hash in packages/http-protocol/src/v1/requests/announce.rs:564:23
R3 Bad request. Cannot parse query params for announce request: invalid param value -qB0000000000000000 for peer_id in Peer id too short: expected 20 bytes, got 19, packages/http-protocol/src/v1/requests/announce.rs:576:38
R6 Bad request. Cannot parse query params for scrape request: missing param info_hash in packages/http-protocol/src/v1/requests/scrape.rs:93:23
R7 Bad request. Cannot parse query params for announce request: missing query params for announce request in packages/axum-http-server/src/v1/extractors/announce_request.rs:66:23
R8 Bad request. Cannot parse query params for scrape request: missing query params for scrape request in packages/axum-http-server/src/v1/extractors/scrape_request.rs:66:23
R9 Bad request. Cannot parse query params: invalid param a=b=c in packages/http-protocol/src/v1/query.rs:176:27
```

After (from `evidence/responses-after.txt`):

```text
R1 Bad request. Cannot parse query params for announce request: invalid param value x for numwant
R2 Bad request. Cannot parse query params for announce request: missing param info_hash
R3 Bad request. Cannot parse query params for announce request: invalid param value -qB0000000000000000 for peer_id: it must percent-decode to exactly 20 bytes
R6 Bad request. Cannot parse query params for scrape request: missing param info_hash
R7 Bad request. Cannot parse query params for announce request: missing query params for announce request: the URL has no query string
R8 Bad request. Cannot parse query params for scrape request: missing query params for scrape request: the URL has no query string
R9 Bad request. Cannot parse query params: invalid param a=b=c
```

#### Conclusion

Met. Every reason names the parameter (or the missing query) and the rejected value where there is
one; none holds a path.

### V2 - Short info-hash (scenario M2)

- Goal: an announce and a scrape with a 10-byte `info_hash` get a reason that names `info_hash` and
  holds no path, including no `.cargo/registry` path.
- Initial state: same trackers, same runs as V1.
- Status: `DONE`

#### Observed Result

Before:

```text
R4 Bad request. Cannot parse query params for announce request: invalid param value 0123456789 for info_hash in not enough bytes for infohash: got 10 bytes, expected 20 <cargo-home>/registry/src/index.crates.io-1949cf8c6b5b557f/torrust-info-hash-0.2.0/src/info_hash.rs:273:27, packages/http-protocol/src/v1/requests/announce.rs:559:42
R5 Bad request. Cannot parse query params for scrape request: invalid param value 0123456789 for info_hash in not enough bytes for infohash: got 10 bytes, expected 20 <cargo-home>/registry/src/index.crates.io-1949cf8c6b5b557f/torrust-info-hash-0.2.0/src/info_hash.rs:273:27, packages/http-protocol/src/v1/requests/scrape.rs:84:46
```

After:

```text
R4 Bad request. Cannot parse query params for announce request: invalid param value 0123456789 for info_hash: it must percent-decode to exactly 20 bytes
R5 Bad request. Cannot parse query params for scrape request: invalid param value 0123456789 for info_hash: it must percent-decode to exactly 20 bytes
```

#### Conclusion

Met. Before, the reason held two paths: the registry checkout of `torrust-info-hash` (with the
home directory of the account that built the tracker) and a workspace path. After, it holds
neither, and the inner error's text is no longer formatted into the reason.

### V3 - Key and proxy errors (scenario M3)

- Goal: on the private tracker, an announce with a malformed key in the URL (R10); on the
  reverse-proxy tracker, an announce without `X-Forwarded-For` (R11). The reasons hold no path.
- Initial state: same trackers, same runs as V1.
- Status: `DONE`

#### Observed Result

Before:

```text
R10 Tracker authentication error: Invalid format for authentication key param. Error in packages/axum-http-server/src/v1/extractors/authentication_key.rs:109:23
R11 Error resolving peer IP: missing or invalid the right most X-Forwarded-For IP (mandatory on reverse proxy tracker configuration) in packages/http-protocol/src/v1/services/peer_ip_resolver.rs:96:27
```

After:

```text
R10 Tracker authentication error: Invalid format for authentication key param: expected 32 alphanumeric characters
R11 Error resolving peer IP: missing or invalid right-most IP in the X-Forwarded-For header, which is mandatory when the tracker runs behind a reverse proxy
```

#### Conclusion

Met. Neither reason holds a path. R10 does not echo the key (a secret); it names the expected
format. `CannotExtractKeyParam` and `MissingClientIp` cannot be reached through the tracker's own
routes (the router always supplies the path params and the connection info), so they are covered
by unit tests only (see Regression Tests).

### T6 - Old and new failure reasons, for wording review

The text each error contributes to the `failure reason` (the prefix added by the `From`
conversion, for example `Bad request. Cannot parse query params for announce request:`, is
unchanged). `{…}` marks the interpolated input. The maintainer approves the wording in the PR
review.

| Error | Old text | New text |
| --- | --- | --- |
| `ParseQueryError::InvalidParam` | `invalid param {raw_param} in {location}` | `invalid param {raw_param}` |
| `ParseAnnounceQueryError::MissingParams` | `missing query params for announce request in {location}` | `missing query params for announce request: the URL has no query string` |
| `ParseAnnounceQueryError::MissingParam` | `missing param {param_name} in {location}` | `missing param {param_name}` |
| `ParseAnnounceQueryError::InvalidParam` | `invalid param value {param_value} for {param_name} in {location}` | `invalid param value {param_value} for {param_name}` |
| `ParseAnnounceQueryError::InvalidInfoHashParam` | `invalid param value {param_value} for {param_name} in {source}` (source with two locations) | `invalid param value {param_value} for {param_name}: it must percent-decode to exactly 20 bytes` |
| `ParseAnnounceQueryError::InvalidPeerIdParam` | `invalid param value {param_value} for {param_name} in {source}` (source with one location) | `invalid param value {param_value} for {param_name}: it must percent-decode to exactly 20 bytes` |
| `ParseScrapeQueryError::MissingParams` | `missing query params for scrape request in {location}` | `missing query params for scrape request: the URL has no query string` |
| `ParseScrapeQueryError::MissingParam` | `missing param {param_name} in {location}` | `missing param {param_name}` |
| `ParseScrapeQueryError::InvalidInfoHashParam` | `invalid param value {param_value} for {param_name} in {source}` (source with two locations) | `invalid param value {param_value} for {param_name}: it must percent-decode to exactly 20 bytes` |
| `PeerIpResolutionError::MissingRightMostXForwardedForIp` | `missing or invalid the right most X-Forwarded-For IP (mandatory on reverse proxy tracker configuration) in {location}` | `missing or invalid right-most IP in the X-Forwarded-For header, which is mandatory when the tracker runs behind a reverse proxy` |
| `PeerIpResolutionError::MissingClientIp` | `cannot get the client IP from the connection info in {location}` | `cannot get the client IP: the server did not provide the socket address of the connection` |
| `auth::Error::InvalidKeyFormat` | `Invalid format for authentication key param. Error in {location}` | `Invalid format for authentication key param: expected 32 alphanumeric characters` |
| `auth::Error::CannotExtractKeyParam` | `Cannot extract authentication key param from URL path. Error in {location}` | `Cannot extract authentication key param from URL path: expected /announce/<key> or /scrape/<key>` |

Notes for the review:

- The four formats that already printed their input only lose the trailing `in {location}`.
- The six that printed no input (the two `MissingParams`, the two `PeerIpResolutionError` and the
  two `auth::Error` variants) are reworded to say what failed. The two `MissingParams` and the
  two `auth::Error` texts keep the old leading words, so a client or test that matched the start
  of the old text still matches. The two `PeerIpResolutionError` texts do not: they keep only
  "missing or invalid" and "cannot get the client IP". Five `axum-http-server` handler tests and
  the contract-test helper
  `assert_could_not_find_remote_address_on_x_forwarded_for_header_error_response` matched the old
  starts and change with them (see Analysis).
- The info-hash and peer-id messages no longer print their source: the info-hash source text comes
  from `torrust-info-hash` (ADR rule 4), and the peer-id message uses the same wording so the two
  20-byte params read alike. The byte count the old source text gave ("got 19") is gone; the
  rejected value is still printed. The sources stay reachable through `Error::source()`.
- `32` in `InvalidKeyFormat` is `AUTH_KEY_LENGTH` of `tracker-core`, which `http-protocol` does
  not depend on, so the number is written in the message. A change of the key length would need a
  change here too.

## Regression Tests

### Boundary

As the spec's Regression Test Strategy says:

- Unit tests at the parse seams of `http-protocol`, through `str::parse::<Query>()`,
  `str::parse::<Event>()` and `Announce::try_from(Query)` / `Scrape::try_from(Query)` (the
  shapes real callers use), and through `resolve_remote_client_addr` for the peer-IP errors.
  Each test checks that the message holds no source path and pins the full message.
- Unit tests at the `axum-http-server` extractors that build `MissingParams`, `InvalidKeyFormat`
  and `CannotExtractKeyParam`. `CannotExtractKeyParam` is reached by calling the `Extract`
  extractor on a request that has no path params, as axum does when the route has none.
- One HTTP-server test per failure family in
  `packages/axum-http-server/tests/server/v1/contract/failure_reasons_hold_no_source_path.rs`,
  including the short `info_hash` (announce and scrape) and the short `peer_id`, the cases that went
  through `LocatedError`. They check the `failure reason` holds no source path and names what
  failed.

The helper `assert_names_no_source_path` forbids `.rs:`, `packages/`, `packages\`, `src/`, `src\`,
`.cargo/registry` and `rustlib/src`. It is `#[track_caller]`, so a failure names the test line, not
the helper's. There is one copy per test target (the `http-protocol` unit tests, the
`axum-http-server` unit tests and its integration tests), as in SI-2: the targets share no test
crate.

The new tests (25):

- `http-protocol` (10): `v1::query::tests::url_query::it_should_describe_an_unparsable_param_by_its_raw_text_and_no_source_path`;
  in `v1::requests::announce::tests::announce_request::describing_a_rejected_query`:
  `it_should_name_a_missing_mandatory_param_and_no_source_path`,
  `it_should_name_an_invalid_param_and_its_value_and_no_source_path`,
  `it_should_name_an_invalid_event_and_no_source_path_when_the_event_is_parsed_on_its_own`,
  `it_should_name_a_short_info_hash_and_the_expected_length_and_no_source_path`,
  `it_should_name_a_short_peer_id_and_the_expected_length_and_no_source_path`; in
  `v1::requests::scrape::tests::scrape_request::describing_a_rejected_query`:
  `it_should_name_the_missing_info_hash_param_and_no_source_path`,
  `it_should_name_a_short_info_hash_and_the_expected_length_and_no_source_path`; in
  `v1::services::peer_ip_resolver::tests`:
  `working_without_reverse_proxy::it_should_say_why_it_cannot_get_the_remote_client_ip_without_a_source_path`,
  `working_on_reverse_proxy_mode::it_should_name_the_missing_x_forwarded_for_ip_without_a_source_path`.
- `axum-http-server` unit tests (4): in `v1::extractors::announce_request::tests` and
  `v1::extractors::scrape_request::tests`,
  `it_should_say_that_a_request_without_query_params_has_no_query_string_and_name_no_source_path`;
  in `v1::extractors::authentication_key::tests`,
  `it_should_describe_an_invalid_path_key_by_the_expected_format_and_no_source_path` and
  `it_should_say_where_the_key_is_expected_and_name_no_source_path_when_the_request_has_no_path_params`.
- `axum-http-server` integration tests (11), module
  `server::v1::contract::failure_reasons_hold_no_source_path`: `an_announce_with_an_unparsable_query`,
  `an_announce_without_query_params`, `an_announce_without_the_info_hash_param`,
  `an_announce_with_an_invalid_numwant_param`, `an_announce_with_a_short_info_hash`,
  `an_announce_with_a_short_peer_id`, `a_scrape_without_query_params`,
  `a_scrape_without_the_info_hash_param`, `a_scrape_with_a_short_info_hash`,
  `an_announce_to_a_private_tracker_with_a_malformed_key`,
  `an_announce_to_a_tracker_behind_a_reverse_proxy_without_the_x_forwarded_for_header`.

Pinned messages: the 14 unit tests compare the whole message (or the whole `failure reason` in the
extractor tests) with the new text, so a change of wording fails them. The 11 HTTP-server tests
match a substring only, because the wording is pinned once at the unit level.

### Red (before the fix)

Run at 11:28:57 to 11:29:16 UTC on the commit
`test(http-protocol): [#2915] add red tests that HTTP tracker failure reasons name no source path`,
after `cargo +nightly fmt --all` and the commit (`git diff --quiet HEAD` exited 0 just before
the first command, whose run started at 11:28:57). Commands, in order (stable Rust, as above):

1. `cargo test -q -p torrust-tracker-http-protocol --lib` (no filter: all 82 unit tests ran):
   72 passed, 10 failed, the 10 new tests. Output: `evidence/tests-red-http-protocol.txt`.
2. `cargo test -q -p torrust-tracker-axum-http-server --lib` (no filter: all 53 unit tests ran):
   49 passed, 4 failed, the 4 new tests. Output: `evidence/tests-red-axum-lib.txt`.
3. `cargo test -q -p torrust-tracker-axum-http-server --test integration -- failure_reasons_hold_no_source_path`:
   the filter selects exactly the 11 new tests (63 filtered out); all 11 failed. Output:
   `evidence/tests-red-axum-integration.txt`.

Each of the 25 failures is the source-path assertion, at the test's own line, with the old text,
for example:

```text
thread 'v1::requests::announce::tests::announce_request::describing_a_rejected_query::it_should_name_a_short_info_hash_and_the_expected_length_and_no_source_path' panicked at packages/http-protocol/src/v1/requests/announce.rs:1029:17:
the error message must not hold a source path, but it contains ".rs:": "invalid param value 0123456789 for info_hash in not enough bytes for infohash: got 10 bytes, expected 20 <cargo-home>/registry/src/index.crates.io-1949cf8c6b5b557f/torrust-info-hash-0.2.0/src/info_hash.rs:273:27, packages/http-protocol/src/v1/requests/announce.rs:559:42"
```

The first attempt to commit these tests was stopped by the pre-commit hook (clippy
`redundant_pub_crate` on the helper's `pub(crate)`); the helper became `pub` and the commit was
made on the second attempt. `git log --format=%cI` records 11:27:44 as its author and committer
date; the `git commit` command, whose pre-commit hook runs in between, returned at 11:28:51 (a
`date -u` run right after it). No test ran on the rejected attempt.

### Between T3 and T4

After the commit `fix(http-protocol): [#2915] remove source locations from HTTP tracker request errors`,
`cargo test -q -p torrust-tracker-http-protocol` still failed the 3 tests of the `LocatedError`
cases (short info-hash in announce and scrape, short peer-id), as expected, and passed the other
79; those three were fixed by the next commit.

### Green (after the fix)

Run at 11:34:24 to 11:34:40 UTC on the commit
`fix(http-protocol): [#2915] replace LocatedError sources of info-hash and peer-id param errors`
(`git diff --quiet HEAD` exited 0 before the run), with the same three commands: 82 of 82, 53 of
53 and 11 of 11 passed. Output: `evidence/tests-green.txt`. Before that,
`cargo test -q -p torrust-tracker-http-protocol -p torrust-tracker-axum-http-server -p torrust-tracker-http-core`
passed on the same tree (unit, integration and doc tests of the three crates).

### Prose-first Arrange-Act-Assert review

Each test was written with temporary prose and compared with the code:

- `http-protocol` parse tests. Arrange: a query that differs from a valid one in exactly one
  param (missing `info_hash`, `numwant=x`, a 10-byte `info_hash`, a 19-byte `peer_id`). Act:
  parse it as callers do and render the error. Assert: no source path, and the whole new message.
  The single-use helper `rejection_message` names the action and hides only the parse and unwrap;
  the query stays visible in each test. The valid `info_hash` and `peer_id` are named constants so
  that the one changed param stands out. No prose was kept.
- The `event` test calls `str::parse::<Event>()` directly, the trait-dispatch shape the spec asks
  for; its name says so.
- Peer-IP tests. Arrange: a reverse-proxy mode and IP sources without the IP that mode needs. Act:
  resolve and render. Assert: as above. Written inline, like the nearby tests.
- Extractor tests. The `CannotExtractKeyParam` test needs a request with no path params; it builds
  `Parts` from a plain request and calls the extractor, which is what axum does for a route with no
  `{key}`. The Arrange, Act and Assert comments are kept, as in the nearby tests of that file.
- HTTP-server tests. `failure_reason_for(configuration, path_and_query)` starts a tracker, sends
  one request and returns the decoded `failure reason`; it hides only the environment mechanics. The
  configuration and the request stay in each test, which is all that differs between them.

## Failures and Follow-up

None for the scenarios. Seen while reading, out of scope:

- The extractor module docs of `announce_request.rs` and `scrape_request.rs` say the error
  response is `500`; the extractors return `200`.
- The `authentication_key.rs` module docs still show tracker-core sample texts with old paths
  (`Failed to read key`, `Key has expired`); those errors belong to SI-4 (#2916).
