---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/3004-2758-tracker-client-mask-tracker-key/ISSUE.md
last-updated-utc: "2026-11-23 16:52"
---

<!-- cspell:ignore SECRETKEY OTHERKEY nothex -->

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

## Outcome

Reproduced. R0 confirms the defect on `develop`: the key appears on stderr for `http announce` and
`http scrape`, and on stdout for `check`. After the fix, repeat R0a to R0c like for like as
scenarios M1 to M3 of the spec and record them below.
