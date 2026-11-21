---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2984-tracker-client-check-udp-url-validation/ISSUE.md
last-updated-utc: "2026-11-21 13:15"
---

# Manual Verification Evidence

## Environment and Prerequisites

- Date and time (UTC): 2026-11-19 16:05
- Artifact under test: `tracker_client` built from `torrust/develop` at the commit that contains PR #2956 (no patches).
- Operating system / environment: Linux; `cargo build -p torrust-tracker-client --bin tracker_client` (default toolchain).
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

Control cases (a URL that fails to parse), recorded at the first run on 2026-11-19 16:05 UTC and not rerun:

| `<url>`                 | Exit | Stderr                                                                                                                              |
| ----------------------- | ---- | ----------------------------------------------------------------------------------------------------------------------------------- |
| `invalid URL`           | 2    | `{"error":{"kind":"invalid_configuration","source":"TORRUST_CHECKER_CONFIG","message":"Invalid URL: invalid domain character"}}` |
| `udp://[::1`            | 2    | `{"error":{"kind":"invalid_configuration","source":"TORRUST_CHECKER_CONFIG","message":"Invalid URL: invalid IPv6 address"}}`     |

Stdout was empty (0 bytes) in every case.

#### Conclusion

Reproduced. The `unwrap` at `checks/udp.rs:107` is reached for every UDP URL without a resolvable address; URLs that fail to parse already get the expected exit `2`.
