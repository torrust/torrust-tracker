---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/3004-2758-tracker-client-mask-tracker-key/ISSUE.md
last-updated-utc: "2026-11-23 19:03"
---

<!-- cspell:ignore SECRETKEY OTHERKEY nothex Abcdefghijklmnopqrstuv Greptile -->

# Manual Verification Evidence

## Purpose

Record real, human-oriented verification of the defect and, after the fix, of the completed
behavior. Do not invent commands, output, logs, or results.

## Environment and Prerequisites

- Date and time (UTC): 2026-11-19 16:10 to 16:12
- Artifact under test: `tracker_client` built from `develop` at merge commit `7bc30738a`
  (`Merge torrust/torrust-tracker#2966`), with `cargo build -p torrust-tracker-client --bin tracker_client`
  (`dev` profile).
- Toolchain: `rustc 1.100.0 (0ef1bc386 2026-11-09)`, stable Rust toolchain.
- Operating system / environment: Linux workstation; no tracker listens on `127.0.0.1:1`, so every
  request fails with "connection refused".
- Prerequisites and setup performed: none. `SECRETKEYabc123` stands for a private tracker key.

## Verification Processes

### R0 - Before the fix: the key appears in the output

#### R0a - `http announce`, failed request

```console
$ target/debug/tracker_client http announce http://127.0.0.1:1/announce/SECRETKEYabc123 9c38422213e30bff212b30c360d26f9a02136422 > out.txt 2> err.txt
exit=1
stdout:
stderr: {"error":{"kind":"runtime_failure","source":"runtime","message":"Failed to get a response: reqwest::Error { kind: Request, url: \"http://127.0.0.1:1/announce/SECRETKEYabc123?info_hash=%9C8B%22%13%E3%0B%FF%21%2B0%C3%60%D2o%9A%02%13d%22&peer_id=%2DqB00000000000000001&port=17548&event=started&compact=0\", source: hyper_util::client::legacy::Error(Connect, ConnectError(\"tcp connect error\", 127.0.0.1:1, Os { code: 111, kind: ConnectionRefused, message: \"Connection refused\" })) }"}}
grep -c SECRETKEYabc123 err.txt -> 1
```

The record carries the key, the full query and the `Debug` text of `reqwest::Error`.

#### R0b - `http scrape`, failed request

```console
$ target/debug/tracker_client http scrape http://127.0.0.1:1/scrape/SECRETKEYabc123 9c38422213e30bff212b30c360d26f9a02136422 > out.txt 2> err.txt
exit=1
stdout:
stderr: {"error":{"kind":"runtime_failure","source":"runtime","message":"Failed to get a response: reqwest::Error { kind: Request, url: \"http://127.0.0.1:1/scrape/SECRETKEYabc123?info_hash=%9C8B%22%13%E3%0B%FF%21%2B0%C3%60%D2o%9A%02%13d%22\", source: hyper_util::client::legacy::Error(Connect, ConnectError(\"tcp connect error\", 127.0.0.1:1, Os { code: 111, kind: ConnectionRefused, message: \"Connection refused\" })) }"}}
grep -c SECRETKEYabc123 err.txt -> 1
```

#### R0c - `check` with a keyed HTTP tracker

```console
$ TORRUST_CHECKER_CONFIG='{"udp_trackers":[],"http_trackers":["http://127.0.0.1:1/announce/SECRETKEYabc123"],"health_checks":[]}' target/debug/tracker_client check > out.txt 2> err.txt
exit=0
stdout: {"results":[{"Http":{"Err":{"url":"http://127.0.0.1:1/announce/SECRETKEYabc123","results":[["Announce",{"Err":"Http request did not receive a response within the timeout: ResponseError { err: reqwest::Error { kind: Request, url: \"http://127.0.0.1:1/announce?info_hash=%9C8B%22%13%E3%0B%FF%21%2B0%C3%60%D2o%9A%02%13d%22&peer_id=%2DqB00000000000000001&port=17548&event=started&compact=0\", source: hyper_util::client::legacy::Error(Connect, ConnectError(\"tcp connect error\", 127.0.0.1:1, Os { code: 111, kind: ConnectionRefused, message: \"Connection refused\" })) } }"}],["Scrape",{"Err":"Http request did not receive a response within the timeout: ResponseError { err: reqwest::Error { kind: Request, url: \"http://127.0.0.1:1/scrape?info_hash=%9C8B%22%13%E3%0B%FF%21%2B0%C3%60%D2o%9A%02%13d%22\", source: hyper_util::client::legacy::Error(Connect, ConnectError(\"tcp connect error\", 127.0.0.1:1, Os { code: 111, kind: ConnectionRefused, message: \"Connection refused\" })) } }"}]]}}}]}
stderr:
matches in stdout: 1, in stderr: 0
```

The result echoes the configured URL with the key in its `url` field. The requests themselves go
to `/announce` and `/scrape` without the key, so the per-request texts do not carry it here; they
still carry the `Debug` text of `reqwest::Error`. Two side observations, not part of this defect:
the checker drops the key from the request path, and it reports a refused connection as "did not
receive a response within the timeout".

### R0 repeat - on the implementation branch, before the fix

- Date and time (UTC): 2026-11-23 16:32
- Artifact under test: `tracker_client` built from `develop` at merge commit `6bc417424`
  (`Merge torrust/torrust-tracker#3005`), with `cargo build -p torrust-tracker-client --bin tracker_client`
  (`dev` profile), `rustc 1.100.0 (0ef1bc386 2026-11-09)`.
- Commands: the R0a, R0b and R0c commands above, unchanged, with stdout and stderr redirected to
  files.

Result: byte-for-byte the same records as R0a, R0b and R0c (exit `1`, `1` and `0`; the key once on
stderr for announce and scrape, once on stdout for `check`). The defect is still present on the
branch.

### T1 probes - other output paths

Run on the same build, to complete the audit in the spec (section "Output Path Audit (T1)").

```console
$ TORRUST_CHECKER_CONFIG='{"udp_trackers":[],"http_trackers":"http://127.0.0.1:1/announce/SECRETKEYabc123","health_checks":[]}' target/debug/tracker_client check > /dev/null 2> e.txt
exit=2 {"error":{"kind":"invalid_configuration","source":"TORRUST_CHECKER_CONFIG","message":"JSON parse error: invalid type: string \"http://127.0.0.1:1/announce/SECRETKEYabc123\", expected a sequence at line 1 column 80"}}

$ TORRUST_CHECKER_CONFIG='{"udp_trackers":[],"http_trackers":["http://[::1/announce/SECRETKEYabc123"],"health_checks":[]}' target/debug/tracker_client check > /dev/null 2> e.txt
exit=2 {"error":{"kind":"invalid_configuration","source":"TORRUST_CHECKER_CONFIG","message":"Invalid URL: invalid IPv6 address"}}

$ target/debug/tracker_client http announce 'http://127.0.0.1:1/announce/SECRETKEYabc123?x=1' 9c38422213e30bff212b30c360d26f9a02136422 > /dev/null 2> e.txt
exit=1 {"error":{"kind":"runtime_failure","source":"runtime","message":"invalid tracker URL input: include only scheme, host, optional port, and optional path. Do not include query or fragment. Pass tracker request params using dedicated CLI arguments"}}

$ target/debug/tracker_client http announce http://127.0.0.1:1/announce/SECRETKEYabc123 nothex > /dev/null 2> e.txt
exit=1 {"error":{"kind":"runtime_failure","source":"runtime","message":"invalid infohash `nothex`. Example infohash: `9c38422213e30bff212b30c360d26f9a02136422`"}}

$ target/debug/tracker_client http announce http://127.0.0.1:1/announce/SECRETKEYabc123 9c38422213e30bff212b30c360d26f9a02136422 http://x/announce/OTHERKEY > /dev/null 2> e.txt
exit=2 {"error":{"kind":"usage_error","source":"arguments","message":"error: unexpected argument 'http://x/announce/OTHERKEY' found\n\nUsage: tracker_client http announce [OPTIONS] <TRACKER_URL> <INFO_HASH>\n\nFor more information, try '--help'.\n"}}
```

The first and the last record carry a key (P8 and P9 of the audit). The others do not echo the URL.

## Regression Tests (T2 to T4)

### Boundary

- Library seam (`packages/tracker-client/src/http/client/mod.rs`, module `tests::error_text`): the
  smallest boundary where the error text of a failed request is built (P1, P2 of the audit). It
  runs the real `reqwest` client against a closed local port or a local `404` responder.
- Masking functions (`packages/tracker-client/src/http/client/redaction.rs`): pure unit tests.
- Black-box tests (`console/tracker-client/tests/key_redaction.rs`): the console paths P3 to P9
  build their text in the binary (body read, checker result, health check, configuration and usage
  errors), and the contract is about what reaches stdout and stderr, so they run the real binary.

### Red runs

Full outputs are in `evidence/`.

- Library, 2026-11-23 16:56 UTC, `cargo test -q -p torrust-tracker-client-lib --lib http::client`,
  with the masking functions as identity stubs and the library error unchanged:
  10 failed, 11 passed. The 11 are the 8 URL-building tests that existed before and 3 new
  no-change tests of the masking functions, which pass against identity stubs; 13 tests were added
  in all. `evidence/t2-red-library.txt`. Each seam test failed on the key in the text, for example
  `Failed to get a response: reqwest::Error { kind: Request, url: "http://127.0.0.1:37803/announce/SECRETKEYabc123?...`.
- Console, 2026-11-23 16:57 UTC, the new tests run against the unchanged binary built from
  `develop` `6bc417424` (a copy kept before the branch changed any code):
  `NEXTEST_BIN_EXE_tracker_client=<copy> cargo test -q -p torrust-tracker-client --test key_redaction`.
  `cargo test` does not set `NEXTEST_BIN_EXE_*`, so the copy is the binary under test (the
  usage-error output names it: `Usage: tracker_client-develop-6bc417424 http announce`). 7 of 7
  failed; the health-check test, added after the first run, failed alone against the same copy at
  16:59. `evidence/t2-red-console-develop-binary.txt`.

### Green run

2026-11-23 16:59 UTC, after the fix: `cargo test -q -p torrust-tracker-client-lib --lib http::client`
21 passed; `cargo test -q -p torrust-tracker-client` all passed, `key_redaction` 8 passed.
`cargo clippy -p torrust-tracker-client -p torrust-tracker-client-lib --all-targets -- -D clippy::pedantic`
is clean.

The red runs used the key `SECRETKEYabc123`. The constant was then renamed to
`SecretTrackerKey123` in both test files, because the spell checker of the pre-commit gate does not
accept the first one in Rust sources; the tests are otherwise unchanged.

### T7 green run

2026-11-23 17:07 UTC, on `docs(tracker-client): [#3004] document the masking of the tracker key`
(the last commit with code is `fix(tracker-client): [#3004] do not write the tracker key in tracker_client output`):
`cargo test -p torrust-tracker-client -p torrust-tracker-client-lib` exit `0`, every test binary
`ok` (`key_redaction` 8 passed).

### Mutations of the console fixes

Each console file changed by the fix was restored to its `develop` content in turn, with the library
fix kept, and `key_redaction` was run (2026-11-23 16:59 UTC, `evidence/t4-console-mutations.txt`):

| Restored file | Failing tests |
| ------------- | ------------- |
| `unified/http.rs` (body read, P3) | `..._http_announce_response_body_is_cut_short` |
| `checker/checks/http.rs` (`Checks.url`, P4) | `..._in_the_check_result_when_the_tracker_gets_no_response`, `..._check_response_body_is_cut_short` |
| `clients/http/mod.rs` (checker errors, P5) | `..._check_response_body_is_cut_short` |
| `checker/checks/health.rs` (P6) | `..._health_check_gets_no_response` |
| `checker/config.rs` (P8) | `..._check_configuration_echoes_a_wrong_typed_url` |
| `checker/error.rs` (P9) | `..._usage_error_echoes_a_misplaced_url` |

Every run compiled; the failures are assertion failures. Not covered by a console test: the
`UnsuccessfulResponse` text of `health.rs` (a non-2xx health check); its library twin is covered by
the seam test with the `404` responder.

### Design review of each increment

Prose-first Arrange-Act-Assert comparison, done after each increment passed, before the next one.

1. Masking function tests (`redaction.rs`). Arrange: a URL whose key position is the one
   difference (announce, scrape, trailing slash with query, path prefix, password, no key, not a
   tracker endpoint). Act: `mask_tracker_key`. Assert: the expected URL written as a literal, not
   computed. The helper `masked` parses, masks and prints; it was kept because its name reads as the
   Act and it hides only parsing and printing. Prose removed: the test names and literals say it.
2. Text masking tests. Arrange: a text in the shape the third-party formatters write (a quoted URL
   after `unexpected argument`, a JSON-quoted URL). Act: `mask_tracker_keys_in_text`. Assert:
   the literal expected text. The no-key case keeps the formatter's real message, so it shows a URL
   without a key is left alone.
3. Library seam, failed request (announce, scrape). Arrange: a client for a keyed URL on a closed
   port; `closed_local_address` names the incidental mechanics (bind, read the port, drop). Act:
   `client.announce` or `client.scrape`, visible, inside a `TEST_DEADLINE` timeout. Assert: the
   key is absent and `/announce/****` (or `/scrape/****`) is present: the masked URL is stated
   independently, not derived from the production masker.
4. Library seam, non-2xx reply. Arrange: the local responder answers `404`; the test owns it
   through `AbortOnDrop`, which aborts the task when the test ends, also on a failed assertion.
   Assert: the key is absent, the status and the masked URL are present.
5. Library seam, `Debug` text. One causal difference from 3: the error is printed with `{:?}`.
6. Black-box, `http announce` and `http scrape` against a closed port. Arrange: a keyed URL on a
   closed port. Act: the command, visible as its arguments. Assert: exit `1`, no key on either
   stream, one stderr record whose message shows the masked URL, no `reqwest::Error` text. The
   assertions together state one observable result (the failure record), so they stay in one test.
7. Black-box, body cut short. Arrange: the fake tracker in its cut-short mode (extended with a
   `Reply` mode, not a second fake). The spec planned one test that runs `http announce` and
   `check`; the review split it in two, one Act each, because they guard different paths (P3 and
   P5) and different criteria (AC1 and AC2).
8. Black-box, `check` against a closed port. Assert: the result's `url` equals the keyed URL with
   the key replaced, written with `str::replace` in the test, not with the production masker.
9. Black-box, configuration and usage echoes (P8, P9). The comment above each Arrange says why the
   input is wrong, because that is the causal state and the code alone does not show it.
10. Black-box, health check (P6). Arrange: a health-check URL on a closed port. Assert: no
    `reqwest::Error` text. Added after the mutation plan showed `health.rs` had no guarding test.

### Ownership checkpoint

Reviewed against the code after the library slice was green, and again at the end:

- Closed port: as designed; no shared fixture.
- Local HTTP responder: the `JoinHandle` is held by `AbortOnDrop`, so the task is aborted when the
  test ends, also on a panic.
- Fake HTTP tracker: reused, with a `Reply` mode; it keeps its bounded reads and stop-and-join.
- Child process: `Command::output()` in every black-box test.
- Deadlines: the library tests use a 2 s client timeout and a 10 s `tokio::time::timeout`; the
  binary uses its 5 s network timeout; the fake bounds its request read at 5 s.

### Deviations from the spec

- The spec's commit table has a commit of the red tests (T2). The pre-commit gate runs the tests,
  so red tests cannot be committed; they are committed with the fix that makes them green, and the
  red runs are recorded here.
- The body-read test was split in two (see 7 above).
- Added beyond the spec: `mask_tracker_key` also masks the password of the URL user info (a
  credential in the same URL), and a test for the health-check path (10 above).

## After the fix: like-for-like recheck (M1 to M4)

- Date and time (UTC): 2026-11-23 17:08
- Artifact under test: `tracker_client` and `torrust-tracker` built with
  `cargo build -p torrust-tracker-client --bin tracker_client` and
  `cargo build --bin torrust-tracker` (`dev` profile) on
  `docs(tracker-client): [#3004] document the masking of the tracker key`,
  `rustc 1.100.0 (0ef1bc386 2026-11-09)`.
- Commands: the R0 commands, unchanged (stdout and stderr redirected to files).

### V1 - M1, `http announce`, failed request

```console
$ target/debug/tracker_client http announce http://127.0.0.1:1/announce/SECRETKEYabc123 9c38422213e30bff212b30c360d26f9a02136422 > out.txt 2> err.txt
exit=1
stdout:
stderr: {"error":{"kind":"runtime_failure","source":"runtime","message":"Failed to get a response: connection error for url (http://127.0.0.1:1/announce/****?info_hash=%9C8B%22%13%E3%0B%FF%21%2B0%C3%60%D2o%9A%02%13d%22&peer_id=%2DqB00000000000000001&port=17548&event=started&compact=0)"}}
grep -cF SECRETKEYabc123 err.txt -> 0
```

### V2 - M2, `http scrape`, failed request

```console
$ target/debug/tracker_client http scrape http://127.0.0.1:1/scrape/SECRETKEYabc123 9c38422213e30bff212b30c360d26f9a02136422 > out.txt 2> err.txt
exit=1
stdout:
stderr: {"error":{"kind":"runtime_failure","source":"runtime","message":"Failed to get a response: connection error for url (http://127.0.0.1:1/scrape/****?info_hash=%9C8B%22%13%E3%0B%FF%21%2B0%C3%60%D2o%9A%02%13d%22)"}}
grep -cF SECRETKEYabc123 err.txt -> 0
```

### V3 - M3, `check` with a keyed HTTP tracker

```console
$ TORRUST_CHECKER_CONFIG='{"udp_trackers":[],"http_trackers":["http://127.0.0.1:1/announce/SECRETKEYabc123"],"health_checks":[]}' target/debug/tracker_client check > out.txt 2> err.txt
exit=0
stdout: {"results":[{"Http":{"Err":{"url":"http://127.0.0.1:1/announce/****","results":[["Announce",{"Err":"Http request did not receive a response within the timeout: Failed to get a response: connection error for url (http://127.0.0.1:1/announce?info_hash=%9C8B%22%13%E3%0B%FF%21%2B0%C3%60%D2o%9A%02%13d%22&peer_id=%2DqB00000000000000001&port=17548&event=started&compact=0)"}],["Scrape",{"Err":"Http request did not receive a response within the timeout: Failed to get a response: connection error for url (http://127.0.0.1:1/scrape?info_hash=%9C8B%22%13%E3%0B%FF%21%2B0%C3%60%D2o%9A%02%13d%22)"}]]}}}]}
stderr:
matches in stdout: 0, in stderr: 0
```

The `url` field shows `****`, and the per-request texts carry no `reqwest::Error` text. The two side
observations of R0c are unchanged, as expected (out of scope).

### V4 - M4, success with a valid key

A private tracker on local ports, started with this configuration (`.tmp/m4/tracker.toml`):

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
path = "./.tmp/m4/sqlite3.db"

[[http_trackers]]
bind_address = "127.0.0.1:17070"

[http_api]
bind_address = "127.0.0.1:11212"

[http_api.access_tokens]
admin = "M4LocalAccessToken"

[health_check_api]
bind_address = "127.0.0.1:11313"
```

```console
$ target/debug/torrust-tracker --config-toml-path ./.tmp/m4/tracker.toml > .tmp/m4/tracker.log 2>&1 &
$ curl -s -X POST "http://127.0.0.1:11212/api/v1/keys?token=M4LocalAccessToken" -H "Content-Type: application/json" -d '{"key":"M4ValidKeyAbcdefghijklmnopqrstuv","seconds_valid":3600}'
{"key":"M4ValidKeyAbcdefghijklmnopqrstuv","valid_until":1795457303,"expiry_time":"2026-11-23 18:08:23 UTC"}

$ target/debug/tracker_client http announce http://127.0.0.1:17070/announce/M4ValidKeyAbcdefghijklmnopqrstuv 9c38422213e30bff212b30c360d26f9a02136422 > out.txt 2> err.txt
exit=0
stdout: {"complete":1,"incomplete":0,"interval":120,"min interval":120,"peers":[]}
stderr:
matches in stdout: 0, in stderr: 0

$ target/debug/tracker_client http scrape http://127.0.0.1:17070/scrape/M4ValidKeyAbcdefghijklmnopqrstuv 9c38422213e30bff212b30c360d26f9a02136422 > out.txt 2> err.txt
exit=0
stdout: {"9c38422213e30bff212b30c360d26f9a02136422":{"complete":1,"downloaded":0,"incomplete":0}}
stderr:
matches in stdout: 0, in stderr: 0
```

The keyed requests still reach the tracker and succeed (the key is masked only in output, not in
the request). The tracker was stopped with `SIGINT` afterwards. Its own request log holds the key
(`uri=/announce/<key>?...` at `INFO`); that is the server side, finding A3 of EPIC #2993, not
`tracker_client`.

### V5 - the configuration and usage echoes (P8, P9)

The T1 probes, repeated on the same build:

```console
$ TORRUST_CHECKER_CONFIG='{"udp_trackers":[],"http_trackers":"http://127.0.0.1:1/announce/SECRETKEYabc123","health_checks":[]}' target/debug/tracker_client check > /dev/null 2> e.txt
exit=2 {"error":{"kind":"invalid_configuration","source":"TORRUST_CHECKER_CONFIG","message":"JSON parse error: invalid type: string \"http://127.0.0.1:1/announce/****\", expected a sequence at line 1 column 80"}}

$ target/debug/tracker_client http announce http://127.0.0.1:1/announce/SECRETKEYabc123 9c38422213e30bff212b30c360d26f9a02136422 http://x/announce/OTHERKEY > /dev/null 2> e.txt
exit=2 {"error":{"kind":"usage_error","source":"arguments","message":"error: unexpected argument 'http://x/announce/****' found\n\nUsage: tracker_client http announce [OPTIONS] <TRACKER_URL> <INFO_HASH>\n\nFor more information, try '--help'.\n"}}
```

## Review round 1 of PR 3012: regression tests, fix and recheck

The review found that the text masker missed an echoed URL with a scheme in upper or mixed case, a
URL cut at a `)` or `'` inside it, and a URL that does not parse; that an authenticated library
client did not mask its own key outside `/announce/` and `/scrape/`; and that the kind-only error
text dropped the transport cause.

### Red run

2026-11-23 17:58 UTC, with the new tests added and the production code as in the reviewed head
(`docs(issues): [#3004] record the regression tests, the recheck and the retrospective`),
`evidence/r1-red.txt`:

- `cargo test -q -p torrust-tracker-client-lib --lib http::client`: 8 failed (four text-masker
  tests: upper-case scheme, `)` before the endpoint, a URL that does not parse, an apostrophe in
  the user info; three client-key tests: custom path with no response, custom path with a `404`,
  health check; and the transport-cause test), 23 passed. The two text tests for "one keyed URL"
  and "no URL", added for the spec's planned cases, pass before the fix too.
- `cargo test -q -p torrust-tracker-client --test key_redaction`: the four new echo tests (upper-case
  scheme and a URL that does not parse, for the configuration error and the usage error) failed,
  8 passed.

### Green run

2026-11-23 18:02 UTC on
`test(tracker-client): [#3004] cover upper-case and malformed URLs in the echo paths`:
`cargo test -p torrust-tracker-client -p torrust-tracker-client-lib` exit `0`; `key_redaction` 12
passed, the library unit tests 35 passed. `cargo clippy -p torrust-tracker-client -p torrust-tracker-client-lib --all-targets -- -D clippy::pedantic`
is clean.

### Recheck after the round 1 fix

2026-11-23 18:02 UTC, `tracker_client` and `torrust-tracker` built on
`test(tracker-client): [#3004] cover upper-case and malformed URLs in the echo paths`, the last
commit with code. The commands of V1 to V5, unchanged, with the same tracker configuration (a
fresh database). This recheck replaces V1 to V5 as the final one; the texts now end with the chain
of causes.

```console
$ target/debug/tracker_client http announce http://127.0.0.1:1/announce/SECRETKEYabc123 9c38422213e30bff212b30c360d26f9a02136422 > out.txt 2> err.txt
exit=1
stdout:
stderr: {"error":{"kind":"runtime_failure","source":"runtime","message":"Failed to get a response: connection error for url (http://127.0.0.1:1/announce/****?info_hash=%9C8B%22%13%E3%0B%FF%21%2B0%C3%60%D2o%9A%02%13d%22&peer_id=%2DqB00000000000000001&port=17548&event=started&compact=0): client error (Connect): tcp connect error: Connection refused (os error 111)"}}
grep -cF SECRETKEYabc123 err.txt -> 0

$ target/debug/tracker_client http scrape http://127.0.0.1:1/scrape/SECRETKEYabc123 9c38422213e30bff212b30c360d26f9a02136422 > out.txt 2> err.txt
exit=1
stdout:
stderr: {"error":{"kind":"runtime_failure","source":"runtime","message":"Failed to get a response: connection error for url (http://127.0.0.1:1/scrape/****?info_hash=%9C8B%22%13%E3%0B%FF%21%2B0%C3%60%D2o%9A%02%13d%22): client error (Connect): tcp connect error: Connection refused (os error 111)"}}
grep -cF SECRETKEYabc123 err.txt -> 0

$ TORRUST_CHECKER_CONFIG='{"udp_trackers":[],"http_trackers":["http://127.0.0.1:1/announce/SECRETKEYabc123"],"health_checks":[]}' target/debug/tracker_client check > out.txt 2> err.txt
exit=0
stdout: {"results":[{"Http":{"Err":{"url":"http://127.0.0.1:1/announce/****","results":[["Announce",{"Err":"Http request did not receive a response within the timeout: Failed to get a response: connection error for url (http://127.0.0.1:1/announce?info_hash=%9C8B%22%13%E3%0B%FF%21%2B0%C3%60%D2o%9A%02%13d%22&peer_id=%2DqB00000000000000001&port=17548&event=started&compact=0): client error (Connect): tcp connect error: Connection refused (os error 111)"}],["Scrape",{"Err":"Http request did not receive a response within the timeout: Failed to get a response: connection error for url (http://127.0.0.1:1/scrape?info_hash=%9C8B%22%13%E3%0B%FF%21%2B0%C3%60%D2o%9A%02%13d%22): client error (Connect): tcp connect error: Connection refused (os error 111)"}]]}}}]}
stderr:
matches in stdout: 0, in stderr: 0

$ curl -s -X POST "http://127.0.0.1:11212/api/v1/keys?token=M4LocalAccessToken" -H "Content-Type: application/json" -d '{"key":"M4ValidKeyAbcdefghijklmnopqrstuv","seconds_valid":3600}'
{"key":"M4ValidKeyAbcdefghijklmnopqrstuv","valid_until":1795460566,"expiry_time":"2026-11-23 19:02:46 UTC"}

$ target/debug/tracker_client http announce http://127.0.0.1:17070/announce/M4ValidKeyAbcdefghijklmnopqrstuv 9c38422213e30bff212b30c360d26f9a02136422 > out.txt 2> err.txt
exit=0
stdout: {"complete":1,"incomplete":0,"interval":120,"min interval":120,"peers":[]}
stderr:
matches in stdout: 0, in stderr: 0

$ target/debug/tracker_client http scrape http://127.0.0.1:17070/scrape/M4ValidKeyAbcdefghijklmnopqrstuv 9c38422213e30bff212b30c360d26f9a02136422 > out.txt 2> err.txt
exit=0
stdout: {"9c38422213e30bff212b30c360d26f9a02136422":{"complete":1,"downloaded":0,"incomplete":0}}
stderr:
matches in stdout: 0, in stderr: 0
```

The two echoes, each with a lowercase scheme, an upper-case scheme and a URL that does not parse
(`$U` is the URL in each line):

```console
$ TORRUST_CHECKER_CONFIG="{\"udp_trackers\":[],\"http_trackers\":\"$U\",\"health_checks\":[]}" target/debug/tracker_client check > /dev/null 2> e.txt
$ target/debug/tracker_client http announce http://127.0.0.1:1/announce 9c38422213e30bff212b30c360d26f9a02136422 "$U" > /dev/null 2> e.txt
exit=2 {"error":{"kind":"invalid_configuration","source":"TORRUST_CHECKER_CONFIG","message":"JSON parse error: invalid type: string \"http://127.0.0.1:1/announce/****\", expected a sequence at line 1 column 80"}}
exit=2 {"error":{"kind":"usage_error","source":"arguments","message":"error: unexpected argument 'http://127.0.0.1:1/announce/****' found\n\nUsage: tracker_client http announce [OPTIONS] <TRACKER_URL> <INFO_HASH>\n\nFor more information, try '--help'.\n"}}
exit=2 {"error":{"kind":"invalid_configuration","source":"TORRUST_CHECKER_CONFIG","message":"JSON parse error: invalid type: string \"HTTP://127.0.0.1:1/announce/****\", expected a sequence at line 1 column 80"}}
exit=2 {"error":{"kind":"usage_error","source":"arguments","message":"error: unexpected argument 'HTTP://127.0.0.1:1/announce/****' found\n\nUsage: tracker_client http announce [OPTIONS] <TRACKER_URL> <INFO_HASH>\n\nFor more information, try '--help'.\n"}}
exit=2 {"error":{"kind":"invalid_configuration","source":"TORRUST_CHECKER_CONFIG","message":"JSON parse error: invalid type: string \"http://127.0.0.1:bad/announce/****\", expected a sequence at line 1 column 82"}}
exit=2 {"error":{"kind":"usage_error","source":"arguments","message":"error: unexpected argument 'http://127.0.0.1:bad/announce/****' found\n\nUsage: tracker_client http announce [OPTIONS] <TRACKER_URL> <INFO_HASH>\n\nFor more information, try '--help'.\n"}}
```

No output contains a key. The tracker was stopped with `SIGINT` afterwards.

## Review round 2 of PR 3012: regression tests, fix and recheck

Greptile and CodeRabbit found four more ways the text masker left a key or a password in an echo: a
`\` separator (`/announce\<key>`), a key segment that starts with or contains a delimiter
(`/announce/(<key>`), an endpoint that starts at the `/` ending another one
(`/announce/scrape/<key>`), and a second URL's password when one authority runs into the next. They
also asked for a body-read test of `http scrape`.

### Red runs and the scrape mutation

- 2026-11-23 18:39 UTC, tests added, production code as in the round 1 head
  (`docs(pr-reviews): [#3012] record review round 1`), `evidence/r2-red.txt`: 4 library tests
  failed (backslash, delimiter in the segment, nested endpoint, two passwords) and the black-box
  usage-error test with a backslash failed. The `http scrape` body-read test passed, because the fix
  already covered that path.
- 2026-11-23 18:40 UTC, `evidence/r2-scrape-mutation.txt`: with `scrape_command` reading the body
  with `response.bytes().await?` again (announce unchanged), only the new `http scrape` body-read
  test failed. The file was restored with no diff.
- 2026-11-23 18:41 UTC, `evidence/r2-red-escaped-backslash.txt`: the recheck after the first round 2
  fix (`fix(tracker-client): [#3004] mask keys after a backslash, a delimiter or a nested endpoint`)
  still showed the key in the configuration echo with a backslash, because `serde_json` writes it
  as `\\` and the masker stopped at the second one. A unit test with `\\` and a black-box test of
  the configuration echo with a backslash were added and failed on that commit; the next commit
  (`fix(tracker-client): [#3004] mask the key after an escaped backslash`) makes the key the first
  non-empty segment after the endpoint.

### Green run

2026-11-23 18:42 UTC on `fix(tracker-client): [#3004] mask the key after an escaped backslash`:
`cargo test -q -p torrust-tracker-client-lib --lib` 40 passed; `cargo test -q -p torrust-tracker-client`
all binaries `ok` (`key_redaction` 15 passed); clippy with `-D clippy::pedantic` clean.

### Final recheck

2026-11-23 18:43 UTC, `tracker_client` and `torrust-tracker` built on
`fix(tracker-client): [#3004] mask the key after an escaped backslash`, the last commit with code.
The same commands as the round 1 recheck (a fresh database), and the two echoes now also with a
backslash URL. This recheck replaces the round 1 recheck as the final one. Every non-empty `exit=`,
`stdout:` and `stderr:` line of M1 to M4 and of the first three echo URLs equals a line recorded
in the round 1 recheck above (checked with `grep -qxF` against this file). The new lines:

```console
$ U='http://127.0.0.1:1/announce\SECRETKEYabc123'
$ TORRUST_CHECKER_CONFIG="{\"udp_trackers\":[],\"http_trackers\":\"${U//\\/\\\\}\",\"health_checks\":[]}" target/debug/tracker_client check > /dev/null 2> e.txt
exit=2 {"error":{"kind":"invalid_configuration","source":"TORRUST_CHECKER_CONFIG","message":"JSON parse error: invalid type: string \"http://127.0.0.1:1/announce\\\\****\", expected a sequence at line 1 column 81"}}
$ target/debug/tracker_client http announce http://127.0.0.1:1/announce 9c38422213e30bff212b30c360d26f9a02136422 "$U" > /dev/null 2> e.txt
exit=2 {"error":{"kind":"usage_error","source":"arguments","message":"error: unexpected argument 'http://127.0.0.1:1/announce\\****' found\n\nUsage: tracker_client http announce [OPTIONS] <TRACKER_URL> <INFO_HASH>\n\nFor more information, try '--help'.\n"}}
```

Across the whole recheck output, `SECRETKEYabc123` appears only in the input lines and the
`grep -cF` commands, which print `0`. The tracker was stopped with `SIGINT` afterwards.

## Review round 3 of PR 3012: recheck after the message casing change

Copilot asked to spell `HTTP` in upper case in three checker error messages
(`fix(tracker-client): [#3004] spell HTTP in upper case in the checker errors`). That changes
production text, so the recheck was repeated on that commit, the last commit with code, at
2026-11-23 19:03 UTC, with the same commands and configuration as the round 2 final recheck. This
recheck replaces it as the final one.

Every non-empty `exit=`, `stdout:` and `stderr:` line equals a line recorded above (checked with
`grep -qxF` against this file), except the `check` result (M3), which now reads:

```console
$ TORRUST_CHECKER_CONFIG='{"udp_trackers":[],"http_trackers":["http://127.0.0.1:1/announce/SECRETKEYabc123"],"health_checks":[]}' target/debug/tracker_client check > out.txt 2> err.txt
exit=0
stdout: {"results":[{"Http":{"Err":{"url":"http://127.0.0.1:1/announce/****","results":[["Announce",{"Err":"HTTP request did not receive a response within the timeout: Failed to get a response: connection error for url (http://127.0.0.1:1/announce?info_hash=%9C8B%22%13%E3%0B%FF%21%2B0%C3%60%D2o%9A%02%13d%22&peer_id=%2DqB00000000000000001&port=17548&event=started&compact=0): client error (Connect): tcp connect error: Connection refused (os error 111)"}],["Scrape",{"Err":"HTTP request did not receive a response within the timeout: Failed to get a response: connection error for url (http://127.0.0.1:1/scrape?info_hash=%9C8B%22%13%E3%0B%FF%21%2B0%C3%60%D2o%9A%02%13d%22): client error (Connect): tcp connect error: Connection refused (os error 111)"}]]}}}]}
stderr:
matches in stdout: 0, in stderr: 0
```

`SECRETKEYabc123` appears only in the input lines and the `grep -cF` commands, which print `0`.
The same commit adds a test that masking keeps the percent encoding of the other segments and the
query; it passed when it was added (the masker uses the serialized, percent-encoded segments).

## Outcome

Reproduced. R0 confirms the defect on `develop`: the key appears on stderr for `http announce` and
`http scrape`, and on stdout for `check`.

Fixed. After the fix, the like-for-like recheck (V1 to V3), the success case (V4) and the two
echoes (V5) write no key; every tracker URL in the output shows `****`. The recheck was repeated
after the review round 1, 2 and 3 fixes, on the last commit with code, with the same result.
