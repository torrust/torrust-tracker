# Prose-First Test Review - Issue #2417

Record of the mandatory prose-first Arrange-Act-Assert comparison
(`.github/skills/dev/testing/write-unit-test/SKILL.md`) for every test this issue
added or materially changed. For each test, the temporary prose below was the
specification; the code was compared with it, and the code was changed where
it did not express the prose. The temporary prose was then removed from the
code: the names, visible Acts, and assertions now carry it. Kept comments are
the module-level reason and ADR link, short per-test doc comments that state
the reason a limit test exists (S1, W1, US1, C1, K1), and the one-line W1
context noted below.

## Changes the Comparison Required

| Test | Mismatch with the prose | Change |
| --- | --- | --- |
| H1-H5 | The Act helper was named `scrape`, but the Act is parsing | Renamed to `parse_scrape` |
| H4 | Prose says the request fails *because of the invalid `info_hash`*; the code accepted any error | Asserts `ParseScrapeQueryError::InvalidInfoHashParam` |
| U1, U2 | Encoding (setup) was hidden inside the Act | Encoding moved to Arrange; the Act is `parse_scrape_info_hashes` |
| W1 | Disabling connection ID validation looked like part of the behavior | One-line comment: the sample ID has no valid cookie, and validation is not under test |
| K1 | Reason given but no ADR link (AC3) | ADR link added |
| K8 | Array destructuring obscured "the first hash is repeated" | Built as `[first, second, first]` by index (done before commit) |
| K6, K8 | Empty stderr was asserted as equality with an empty `Vec<Value>` | `assert!(stderr_records.is_empty(), ...)` (done before commit) |

## HTTP Parser (`packages/http-protocol`, `Scrape::try_from`)

| ID | Arrange | Act | Assert | Result |
| --- | --- | --- | --- | --- |
| H1 | A query with exactly 100 distinct `info_hash` params, the limit | Parse it into a `Scrape` | All 100 are kept, in request order | Matches after rename |
| H2 | A query with 101 distinct params | Parse it | Only the first 100 are kept, in request order | Matches after rename |
| H3 | 100 valid params followed by an invalid 101st | Parse it | Parsing succeeds and keeps the 100 valid hashes; the 101st is never validated | Matches after rename |
| H4 | An invalid param first, then 100 valid ones (101 in total) | Parse it | Parsing fails with the invalid `info_hash` error | Changed: specific error asserted |
| H5 | 101 params where the first hash is sent twice | Parse it | The first 100 params as sent are kept (99 distinct): duplicates count | Matches after rename |

The query string is built with `scrape_builder::Query` (percent-encoding is
incidental mechanics); the expected values are the literal input slices, not
values derived from the parser.

## HTTP Server (`packages/axum-http-server`)

| ID | Arrange | Act | Assert | Result |
| --- | --- | --- | --- | --- |
| S1 | A public tracker with no peers; 101 distinct hashes | Scrape them through the HTTP client | The response has zeroed files for exactly the first 100 | Matches |

## UDP Parser (`packages/udp-protocol`)

| ID | Arrange | Act | Assert | Result |
| --- | --- | --- | --- | --- |
| U1 | A scrape request with 74 distinct hashes, encoded | Parse it with `MAX_SCRAPE_INFO_HASHES` | All 74 are kept | Changed: encoding moved to Arrange |
| U2 | A scrape request with 75 distinct hashes, encoded | Parse it with `MAX_SCRAPE_INFO_HASHES` | The first 74 are kept, in order | Changed: encoding moved to Arrange |
| U3 | Scrape requests with 74 and with 75 hashes | Encode both | 74 fit in `MAX_PACKET_SIZE` and 75 do not | Matches |

## UDP Server (`packages/udp-server`)

| ID | Arrange | Act | Assert | Result |
| --- | --- | --- | --- | --- |
| W1 | A 75-hash scrape payload handed to the handler without a socket | `handle_packet` | A scrape response with 74 entries | Changed: context comment |
| US1 | A running tracker, a connected client, and a valid connection ID | Send a 75-hash scrape over the socket | 74 torrent stats | Matches |

## Tracker Core (`packages/tracker-core`)

| ID | Arrange | Act | Assert | Result |
| --- | --- | --- | --- | --- |
| C1 | A public scrape handler; 101 distinct hashes | `handle_scrape` | 101 files: core has no cap | Matches |

## Tracker Client (`console/tracker-client`)

| ID | Arrange | Act | Assert | Result |
| --- | --- | --- | --- | --- |
| K1 | `udp scrape` arguments with 75 hashes | Parse the CLI arguments | Accepted, with all 75 hashes | Changed: ADR link |
| K2 | 75 requested, 74 returned | Build the truncation warning | One record with kind and both counts | Matches |
| K3 | 74 requested, 74 returned | Build the truncation warning | No warning | Matches |
| K4 | Three hashes, the first repeated | Count distinct hashes | 2 | Matches |
| K5 | A fake UDP tracker that keeps 2 hashes | Run `tracker_client udp scrape` with 3 | Exit 0; 2 entries on stdout; stderr is exactly one warning (3, 2) | Matches |
| K6 | A fake UDP tracker that answers everything | Run it with 3 hashes | Exit 0; 3 entries; empty stderr | Changed before commit (empty-stderr assertion) |
| K7 | A fake HTTP tracker that keeps 2 | Run `tracker_client http scrape` with 3 | Exit 0; 2 files; stderr is exactly one warning (3, 2) | Matches |
| K8 | A fake HTTP tracker that answers everything | Run it with the first hash repeated | Exit 0; 2 files; empty stderr | Changed before commit (arrangement, empty-stderr assertion) |

`Run` and `stderr_warnings()` are incidental mechanics: they split the output
channels and fail on any non-JSON line or unexpected stderr record, so the
assertions read as the prose.

## Monitor (`console/tracker-client`, ride-along T9)

| Test | Arrange | Act | Assert | Result |
| --- | --- | --- | --- | --- |
| Success path | A fake UDP tracker that answers | Monitor it for two seconds | Exit 0; every probe is `ok` with a latency; the summary counts them, with no timeouts and populated min/max/average/last | Matches |
| Timeout path | A silent fake UDP tracker | Monitor it for two seconds | Exit 0; the summary names the monitored URL; every probe times out; the summary counts them, `timeout_percent` is 100, and latencies are null | Matches |

## Accepted Exception: Timing in Binary-Level Tests

The fake trackers poll a stop flag every 10 ms, and the monitor tests run the
binary for a fixed two seconds with a one-second probe timeout. The success
path assumes a loopback round trip finishes within one second. This is
accepted: these tests specify the CLI contract of a real process over real
sockets, which a clock abstraction cannot reach without changing production
code. Assertions do not depend on exact timing (they check that every probe has
the expected status and that counts agree). Three consecutive runs of
`cargo test --manifest-path console/tracker-client/Cargo.toml --test tracker_checker --test tracker_client`
passed (11 and 7 tests each run).
