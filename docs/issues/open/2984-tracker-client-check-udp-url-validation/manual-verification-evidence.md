---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2984-tracker-client-check-udp-url-validation/ISSUE.md
last-updated-utc: "2026-11-22 14:21"
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
# stdout goes to a pipe (the client refuses a terminal), stderr to a file
TORRUST_CHECKER_CONFIG='{"udp_trackers":["<url>"],"http_trackers":[],"health_checks":[]}' \
  timeout 60 ./target/debug/tracker_client check 2>stderr.txt | cat >stdout.txt
code=${PIPESTATUS[0]}   # the client's exit code, saved immediately after the pipeline
echo "exit=$code stdout_bytes=$(wc -c <stdout.txt)"
cat stdout.txt stderr.txt
```

Without `${PIPESTATUS[0]}`, the pipeline's status is `cat`'s, normally `0`, whatever the client returned, so the exit codes below come from `PIPESTATUS`, not from `$?`. `stdout.txt` and `stderr.txt` are scratch files in a temporary directory; the blocks below show their content. The first run (2026-11-19) summarized stderr by hand; the reruns recorded below used this procedure.

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
# stdout goes to a pipe (the client refuses a terminal), stderr to a file
TORRUST_CHECKER_CONFIG='{"udp_trackers":[],"http_trackers":["<url>"],"health_checks":[]}' \
  timeout 60 ./target/debug/tracker_client check 2>stderr.txt | cat >stdout.txt
code=${PIPESTATUS[0]}   # the client's exit code, saved immediately after the pipeline
echo "exit=$code stdout_bytes=$(wc -c <stdout.txt)"
cat stdout.txt stderr.txt
```

Same capture procedure as V1 (`${PIPESTATUS[0]}` for the exit code, stdout measured with `wc -c`, stderr kept separate), with the URL in `http_trackers` and, for the second list, in `health_checks`.

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

### V3 - Red regression tests (before the fix)

Run on `torrust/develop` at `6bc417424` with the new tests and only the test-enabling production changes (`resolve_socket_addr` returning a `Result` whose body still unwraps, the `ConfigurationError::InvalidTrackerUrl` type unused, and `panic_record` extracted). Command: `cargo test -p torrust-tracker-client --lib`, then `cargo test -p torrust-tracker-client --test output_contract`.

```text
test result: FAILED. 49 passed; 11 failed; 0 ignored
FAILED (all panicked with "the configuration should be rejected", except the last):
  health_checks::it_should_fail_when_a_health_check_url_has_the_ftp_scheme
  health_checks::it_should_fail_when_a_health_check_url_has_the_udp_scheme
  http_trackers::it_should_fail_when_a_tracker_http_url_has_the_ftp_scheme
  http_trackers::it_should_fail_when_a_tracker_http_url_has_the_udp_scheme
  udp_trackers::it_should_fail_when_a_tracker_udp_url_has_a_path_but_no_port
  udp_trackers::it_should_fail_when_a_tracker_udp_url_has_no_host
  udp_trackers::it_should_fail_when_a_tracker_udp_url_has_no_port
  udp_trackers::it_should_fail_when_a_tracker_udp_url_has_no_scheme_and_no_port
  udp_trackers::it_should_fail_when_a_tracker_udp_url_has_the_ftp_scheme
  udp_trackers::it_should_fail_when_a_tracker_udp_url_has_the_http_scheme
  checks::udp::tests::it_should_return_an_error_instead_of_panicking_when_the_host_cannot_be_resolved (panicked at udp.rs:111)

test it_should_report_a_udp_tracker_url_without_a_port_as_a_configuration_error_and_leave_stdout_empty ... FAILED
  left: Some(1)
 right: Some(2)
test result: FAILED. 9 passed; 1 failed
```

The tests that must keep passing (`localhost:6969`, `https://tracker.example.com` for the HTTP lists, the existing valid cases) passed in the same run. The three `panic_record` unit tests pass on the red tree because the extraction does not change behaviour; they are characterization tests of the panic-hook seam.

### V4 - Green run and like-for-like recheck (after the last production change)

Build: commit `fix(tracker-client): [#2984] reject unusable tracker URLs and fail the check on a DNS error`, `cargo build -p torrust-tracker-client --bin tracker_client`. `cargo test -p torrust-tracker-client` passes (60 unit tests, the integration tests, all green) and `linter all` passes in the pre-commit gate. The commands are those of V1 and V2, with the capture procedure described in V1.

| Section | Entry | Exit | stdout bytes | stderr / result |
| ------- | ----- | ---- | ------------ | --------------- |
| `udp_trackers` | `udp://127.0.0.1` | 2 | 0 | `invalid_configuration`: "Invalid URL `udp://127.0.0.1`: the URL has no port" |
| `udp_trackers` | `udp:///` | 2 | 0 | `invalid_configuration`: "Invalid URL `udp:///`: the URL has no host" |
| `udp_trackers` | `http://127.0.0.1` | 2 | 0 | `invalid_configuration`: "...: the URL scheme is not supported here" |
| `udp_trackers` | `ftp://127.0.0.1:6969` | 2 | 0 | `invalid_configuration`: "...: the URL scheme is not supported here" |
| `udp_trackers` | `udp://nonexistent.invalid:6969` | 0 | 258 | one `Udp` `Err` result, `remote_addr` null, `Setup` failed: "Failed to resolve the tracker address ...: failed to lookup address information: Name or service not known"; no panic |
| `udp_trackers` | `udp://127.0.0.1:6969`, `127.0.0.1:6969`, `udp://localhost:6969`, `localhost:6969` | 0 | 205 | checked as before (`Setup` ok, `Connect` timeout because nothing listens) |
| `http_trackers` and `health_checks` | `ftp://127.0.0.1` | 2 | 0 | `invalid_configuration`: "...: the URL scheme is not supported here" |
| `http_trackers` and `health_checks` | `udp://127.0.0.1:6969` | 2 | 0 | `invalid_configuration`: "...: the URL scheme is not supported here" |
| `http_trackers` and `health_checks` | `http:///` | 2 | 0 | `invalid_configuration`: "Invalid URL: empty host" (unchanged) |

Outcome: AC1 to AC5 hold on the real artifact. The result record now has `url` and `remote_addr: Option<SocketAddr>` (`null` when the host did not resolve); for a resolved tracker it carries the same address as before plus the new `url` field.
