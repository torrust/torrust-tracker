---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2984-tracker-client-check-udp-url-validation/ISSUE.md
last-updated-utc: "2026-11-21 15:20"
---

# Manual Verification Evidence

## Environment and Prerequisites

- Date and time (UTC): see Dates below
- Artifact under test: `tracker_client` built from `develop` commit `7bc30738a` (`Merge torrust/torrust-tracker#2966: docs(issues): close out the tracker_client JSON-only issue`), the merge base of this PR and, per `git diff 7bc30738a -- console packages` (empty), identical to this branch in the code it builds. No patches.
- Toolchain: `rustc 1.100.0 (0ef1bc386 2026-11-09)`, `cargo 1.100.0 (3acc79c0c 2026-11-05)`, as reported by `rustc --version` and `cargo --version` on 2026-11-21; the binary used was `target/debug/tracker_client` in this worktree (file modification time 2026-11-21 13:19 UTC, built from the code described above).
- Dates: first run 2026-11-19 16:05 UTC (summarized the stderr); rerun with verbatim capture 2026-11-21 15:15 UTC with the same commands. The rerun printed the same messages as the first run. The toolchain of the first run was not recorded.
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

Control cases (a URL that fails to parse), recorded at the first run on 2026-11-19 16:05 UTC and not rerun:

| `<url>`                 | Exit | Stderr                                                                                                                              |
| ----------------------- | ---- | ----------------------------------------------------------------------------------------------------------------------------------- |
| `invalid URL`           | 2    | `{"error":{"kind":"invalid_configuration","source":"TORRUST_CHECKER_CONFIG","message":"Invalid URL: invalid domain character"}}` |
| `udp://[::1`            | 2    | `{"error":{"kind":"invalid_configuration","source":"TORRUST_CHECKER_CONFIG","message":"Invalid URL: invalid IPv6 address"}}`     |

Stdout was empty (0 bytes) in every case.

#### Conclusion

Reproduced. The `unwrap` at `checks/udp.rs:107` is reached for every UDP URL without a resolvable address; URLs that fail to parse already get the expected exit `2`.
