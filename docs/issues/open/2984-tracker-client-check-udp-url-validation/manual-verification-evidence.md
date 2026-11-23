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

### V5 - Test design reviews (prose-first Arrange/Act/Assert)

The Test-development loop in `ISSUE.md` requires a design review for every test-producing increment. It was not recorded when T3 was marked done; Copilot's review of the implementation pull request found that gap. The reviews below were done afterwards, by re-reading each test against the loop's checklist: the one causal initial-state difference, a fixture that owns only incidental mechanics, and a visible production Act with an independently specified expected result. A refactoring pattern from `docs/testing/refactoring-patterns/` was not needed in any increment; none of the tests has duplicated setup that a catalog pattern would remove.

| Increment | Arrange (one causal difference) | Act | Assert (specified independently of the code) | Helper boundary | Outcome |
| --------- | ------------------------------- | --- | -------------------------------------------- | --------------- | ------- |
| UDP config cases (`udp_trackers`: no port, no scheme and no port, path but no port, no host, `http`, `ftp`; plus upper-case `UDP://` and a query string that contains `://`) | One entry in `udp_trackers`, nothing else in the configuration; the entry is the only difference between tests | `Configuration::try_from(plain_config)`, inside the helper `rejection` for the rejecting cases and visible in the allowing cases | The rejected entry and its `UrlRejection` reason, written as literals from the spec (AC1, AC2), or the parsed URL for the allowed forms (AC4) | `rejection(plain_config)` hides only the match on `ConfigurationError::InvalidTrackerUrl`; the test names the entry and reason. The Act is inside the helper, which is named for what it returns; accepted because the alternative repeats a six-line match in eleven tests | Kept. One test per entry, so a failure names the URL |
| HTTP config cases (`http_trackers` and `health_checks`: `ftp`, `udp`; `https` without a port allowed) | One entry in one list | Same helper and `try_from` as above | `(entry, UnsupportedScheme)` from AC5; the allowed `https` entry only asserts `is_ok` because AC5 names no parsed form | Same helper, shared by the three list modules | Kept |
| Resolver test (`it_should_return_an_error_instead_of_panicking_when_the_host_cannot_be_resolved`) | The reserved `.invalid` host | `resolve_socket_addr(&url)` | `is_err()` only, as the spec's strategy says: it goes through the system resolver and does not claim to be network-free | None | Kept; its Result signature was added only so it could be written (V3) |
| Failed-check test on `run` (added for Copilot finding F2) | Two UDP URLs without a port | `run(urls, timeout)` | Two failed results, each with its URL, a null `remote_addr` and exactly one failed `Setup`; no DNS or network | None | Kept |
| End-to-end test (`it_should_report_a_udp_tracker_url_without_a_port_as_a_configuration_error_and_leave_stdout_empty`) | The configuration `udp://127.0.0.1` in `TORRUST_CHECKER_CONFIG` | Run the real binary with `check` | Exit `2`, empty stdout, one `invalid_configuration` record that names the entry and the port, as the output-contract ADR specifies | `tracker_client`, `stdout_text` and `stderr_records` are the file's existing helpers | Kept |
| Panic-record tests (`panic_record`, three payload kinds) | One payload kind: `&str`, `String` or non-string | `panic_record(payload, location)` | The `kind`, `message` and `location` fields of the JSON record; the fixed message for a non-string payload comes from the previous inline code | `panic_record_value` parses the record and checks that it is one line | Kept; they pass on the red tree because the extraction does not change behaviour |

Weakness found by the review and left as it is: the allowed HTTP forms only assert `is_ok`, because the specification does not say which URL they parse to.

### V6 - Recheck after the review fixes (last production change)

Production code changed after V4 to answer review findings (the scheme detection, the IP-literal tests and the redaction of the rejected entry), so the recheck was repeated. Build: commit `fix(tracker-client): [#2984] do not echo the path, query or credentials of a rejected tracker URL`, `cargo build -p torrust-tracker-client --bin tracker_client`; `cargo test -p torrust-tracker-client` passes (68 unit tests and the integration tests) and the pre-commit gate passed. The commands are those of V1 and V2, with the capture procedure of V1; the table gives the exit code, the stdout size and the error message (`invalid_configuration` record) or the check result.

| Section | Entry | Exit | stdout bytes | Message or result |
| ------- | ----- | ---- | ------------ | ----------------- |
| `udp_trackers` | `udp://127.0.0.1` | 2 | 0 | "Invalid URL `udp://127.0.0.1`: the URL has no port" |
| `udp_trackers` | `udp:///` | 2 | 0 | "Invalid URL `udp://`: the URL has no host" |
| `udp_trackers` | `http://127.0.0.1`, `ftp://127.0.0.1:6969` | 2 | 0 | "...: the URL scheme is not supported here" (entry as typed) |
| `udp_trackers` | `udp://nonexistent.invalid:6969` | 0 | 258 | `Udp` `Err`, `remote_addr` null, failed `Setup`; no panic |
| `udp_trackers` | `udp://127.0.0.1:6969`, `127.0.0.1:6969`, `UDP://localhost:6969`, `localhost:6969` | 0 | 205 | checked as before (`remote_addr` `127.0.0.1:6969`) |
| `udp_trackers` | `http://127.0.0.1:7070/announce/PRIVATE_KEY` | 2 | 0 | "Invalid URL `http://127.0.0.1:7070/****`: the URL scheme is not supported here" |
| `udp_trackers` | `udp://127.0.0.1?key=PRIVATE_KEY` | 2 | 0 | "Invalid URL `udp://127.0.0.1/****`: the URL has no port" |
| `udp_trackers` | `ftp://user:PRIVATE_KEY@127.0.0.1:6969` | 2 | 0 | "Invalid URL `ftp://127.0.0.1:6969/****`: the URL scheme is not supported here" |
| `http_trackers` and `health_checks` | `ftp://127.0.0.1`, `udp://127.0.0.1:6969` | 2 | 0 | "...: the URL scheme is not supported here" |
| `http_trackers` and `health_checks` | `ftp://127.0.0.1/announce/PRIVATE_KEY` | 2 | 0 | "Invalid URL `ftp://127.0.0.1/****`: the URL scheme is not supported here" |
| `http_trackers` and `health_checks` | `http:///` | 2 | 0 | "Invalid URL: empty host" (unchanged) |

No `PRIVATE_KEY` text appears in any output. V4 remains the record of the first recheck; V6 supersedes it as the final one.
