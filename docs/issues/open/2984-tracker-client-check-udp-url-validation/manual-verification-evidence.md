---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2984-tracker-client-check-udp-url-validation/ISSUE.md
last-updated-utc: "2026-11-22 10:18"
---

# Manual Verification Evidence

## Environment and Prerequisites

- Date and time (UTC): see Dates below
- Artifact under test: `tracker_client` built from `develop` commit `7bc30738a` (`Merge torrust/torrust-tracker#2966: docs(issues): close out the tracker_client JSON-only issue`), the merge base of this PR and, per `git diff 7bc30738a -- console packages` (empty), identical to this branch in the code it builds. No patches.
- Toolchain: `rustc 1.100.0 (0ef1bc386 2026-11-09)`, `cargo 1.100.0 (3acc79c0c 2026-11-05)`, as reported by `rustc --version` and `cargo --version` on 2026-11-21; the binary used was `target/debug/tracker_client` in this worktree (file modification time 2026-11-21 13:19 UTC, built from the code described above).
- Dates: first run 2026-11-19 16:05 UTC (summarized the stderr); rerun with verbatim capture 2026-11-21 15:15 UTC with the same commands (the two parse-failure controls were rerun at 15:57 UTC). The rerun printed the same messages as the first run. The toolchain of the first run was not recorded.
- Operating system / environment: Linux.
- Setup: none; no tracker is needed because the failure happens before any network traffic. Stdout is piped (`| cat`) because `tracker_client` refuses a terminal.

## Verification Processes

### V1 - Initial reproduction (before the fix)

- Goal: observe the wrong outcome for UDP URLs without a usable address.
- Initial state: `torrust/develop` after PR #2956.
- Status: `DONE`
- Outcome: **Reproduced** (the wrong outcome, a panic and exit `1`, was observed).

#### Steps Performed

For each `<url>`:

```bash
TORRUST_CHECKER_CONFIG='{"udp_trackers":["<url>"],"http_trackers":[],"health_checks":[]}' \
  ./target/debug/tracker_client check | cat
```

#### Observed Result

Each block below is the exit code, stdout size and stderr of one run, copied verbatim from the rerun at 2026-11-21 15:15:02 UTC (see the environment section). The `panic` record is the first stderr line and the `runtime_failure` record the second.

`udp://127.0.0.1`:

```text
exit=1 stdout_bytes=0
{"kind":"panic","message":"called `Result::unwrap()` on an `Err` value: Custom { kind: InvalidData, error: \"No port number in the URL\" }","location":"console/tracker-client/src/console/clients/checker/checks/udp.rs:107"}
{"error":{"kind":"runtime_failure","source":"runtime","message":"task 33 panicked with message \"called `Result::unwrap()` on an `Err` value: Custom { kind: InvalidData, error: \\\"No port number in the URL\\\" }\""}}
```

`udp:///`:

```text
exit=1 stdout_bytes=0
{"kind":"panic","message":"called `Result::unwrap()` on an `Err` value: Custom { kind: InvalidData, error: \"No host name in the URL\" }","location":"console/tracker-client/src/console/clients/checker/checks/udp.rs:107"}
{"error":{"kind":"runtime_failure","source":"runtime","message":"task 33 panicked with message \"called `Result::unwrap()` on an `Err` value: Custom { kind: InvalidData, error: \\\"No host name in the URL\\\" }\""}}
```

`ftp://127.0.0.1:6969`:

```text
exit=1 stdout_bytes=0
{"kind":"panic","message":"called `Result::unwrap()` on an `Err` value: Custom { kind: InvalidData, error: \"No port number in the URL\" }","location":"console/tracker-client/src/console/clients/checker/checks/udp.rs:107"}
{"error":{"kind":"runtime_failure","source":"runtime","message":"task 33 panicked with message \"called `Result::unwrap()` on an `Err` value: Custom { kind: InvalidData, error: \\\"No port number in the URL\\\" }\""}}
```

`http://127.0.0.1`:

```text
exit=1 stdout_bytes=0
{"kind":"panic","message":"called `Result::unwrap()` on an `Err` value: Custom { kind: InvalidData, error: \"No port number in the URL\" }","location":"console/tracker-client/src/console/clients/checker/checks/udp.rs:107"}
{"error":{"kind":"runtime_failure","source":"runtime","message":"task 33 panicked with message \"called `Result::unwrap()` on an `Err` value: Custom { kind: InvalidData, error: \\\"No port number in the URL\\\" }\""}}
```

`udp://nonexistent.invalid:6969`:

```text
exit=1 stdout_bytes=0
{"kind":"panic","message":"called `Result::unwrap()` on an `Err` value: Custom { kind: Uncategorized, error: \"failed to lookup address information: Name or service not known\" }","location":"console/tracker-client/src/console/clients/checker/checks/udp.rs:107"}
{"error":{"kind":"runtime_failure","source":"runtime","message":"task 33 panicked with message \"called `Result::unwrap()` on an `Err` value: Custom { kind: Uncategorized, error: \\\"failed to lookup address information: Name or service not known\\\" }\""}}
```

Control cases (a URL that fails to parse), rerun with the same binary at 2026-11-21 15:57 UTC, verbatim:

`invalid URL`:

```text
exit=2 stdout_bytes=0
{"error":{"kind":"invalid_configuration","source":"TORRUST_CHECKER_CONFIG","message":"Invalid URL: invalid domain character"}}
```

`udp://[::1`:

```text
exit=2 stdout_bytes=0
{"error":{"kind":"invalid_configuration","source":"TORRUST_CHECKER_CONFIG","message":"Invalid URL: invalid IPv6 address"}}
```

Stdout was empty (0 bytes) in every case above.

#### Conclusion

Reproduced. The `unwrap` at `checks/udp.rs:107` is reached for every UDP URL without a resolvable address; URLs that fail to parse already get the expected exit `2`.

### V2 - Reproduction of the HTTP forms (before the fix)

- Goal: observe what `http_trackers` and `health_checks` entries with a wrong scheme or no host do today (decision 2 of the spec).
- Initial state: the same binary and code as V1 (`develop` commit `7bc30738a`; `git diff --stat 7bc30738a -- console packages` prints nothing). Run at 2026-11-22 10:17:57 UTC.
- Status: `DONE`
- Outcome: **Reproduced**: an entry with a wrong scheme that parses is accepted and checked (exit `0`) instead of being rejected as a configuration error (exit `2`). An entry with no host is already rejected with exit `2`.

#### Steps Performed

For each list (`http_trackers`, `health_checks`) and each `<url>` below, with the URL in that list and the other two lists empty:

```bash
TORRUST_CHECKER_CONFIG='{"udp_trackers":[],"http_trackers":["<url>"],"health_checks":[]}' \
  ./target/debug/tracker_client check | cat
```

#### Observed Result

Each block is the exit code, stdout size, stdout and stderr of one run, verbatim.

`http_trackers ftp://127.0.0.1`:

```text
exit=0 stdout_bytes=640
--stdout
{"results":[{"Http":{"Err":{"url":"ftp://127.0.0.1/","results":[["Announce",{"Err":"Http request did not receive a response within the timeout: ResponseError { err: reqwest::Error { kind: Builder, url: \"ftp://127.0.0.1/announce?info_hash=%9C8B%22%13%E3%0B%FF%21%2B0%C3%60%D2o%9A%02%13d%22&peer_id=%2DqB00000000000000001&port=17548&event=started&compact=0\", source: BadScheme } }"}],["Scrape",{"Err":"Http request did not receive a response within the timeout: ResponseError { err: reqwest::Error { kind: Builder, url: \"ftp://127.0.0.1/scrape?info_hash=%9C8B%22%13%E3%0B%FF%21%2B0%C3%60%D2o%9A%02%13d%22\", source: BadScheme } }"}]]}}}]}
--stderr
```

`http_trackers udp://127.0.0.1:6969`:

```text
exit=0 stdout_bytes=654
--stdout
{"results":[{"Http":{"Err":{"url":"udp://127.0.0.1:6969","results":[["Announce",{"Err":"Http request did not receive a response within the timeout: ResponseError { err: reqwest::Error { kind: Builder, url: \"udp://127.0.0.1:6969/announce?info_hash=%9C8B%22%13%E3%0B%FF%21%2B0%C3%60%D2o%9A%02%13d%22&peer_id=%2DqB00000000000000001&port=17548&event=started&compact=0\", source: BadScheme } }"}],["Scrape",{"Err":"Http request did not receive a response within the timeout: ResponseError { err: reqwest::Error { kind: Builder, url: \"udp://127.0.0.1:6969/scrape?info_hash=%9C8B%22%13%E3%0B%FF%21%2B0%C3%60%D2o%9A%02%13d%22\", source: BadScheme } }"}]]}}}]}
--stderr
```

`http_trackers http:///`:

```text
exit=2 stdout_bytes=0
--stdout
--stderr
{"error":{"kind":"invalid_configuration","source":"TORRUST_CHECKER_CONFIG","message":"Invalid URL: empty host"}}
```

`health_checks ftp://127.0.0.1`:

```text
exit=0 stdout_bytes=196
--stdout
{"results":[{"Health":{"Err":{"url":"ftp://127.0.0.1/","result":{"Err":"Heath check failed to get a response: reqwest::Error { kind: Builder, url: \"ftp://127.0.0.1/\", source: BadScheme }"}}}}]}
--stderr
```

`health_checks udp://127.0.0.1:6969`:

```text
exit=0 stdout_bytes=204
--stdout
{"results":[{"Health":{"Err":{"url":"udp://127.0.0.1:6969","result":{"Err":"Heath check failed to get a response: reqwest::Error { kind: Builder, url: \"udp://127.0.0.1:6969\", source: BadScheme }"}}}}]}
--stderr
```

`health_checks http:///`:

```text
exit=2 stdout_bytes=0
--stdout
--stderr
{"error":{"kind":"invalid_configuration","source":"TORRUST_CHECKER_CONFIG","message":"Invalid URL: empty host"}}
```

#### Conclusion

Wrong-scheme entries that parse (`ftp://`, `udp://`) are checked as if they were HTTP and fail at run time with exit `0`; `http:///` is already a configuration error with exit `2` (`Invalid URL: empty host`). AC5 therefore changes the first case and must keep the second.
