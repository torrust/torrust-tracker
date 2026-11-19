---
doc-type: manual-verification-evidence
issue-spec: docs/issues/closed/2768-2758-tracker-client-json-only/ISSUE.md
last-updated-utc: "2026-11-19 09:45"
---

# Manual Verification Evidence

## Environment and Prerequisites

- Date and time (UTC): 2026-11-09 12:11 to 12:13; the pseudo-terminal re-run shown below was done after Copilot's comment of 12:34:57 UTC and before the commit of 12:40:44 UTC (the exact minute was not recorded)
- Artifact under test: the unchanged tool, built from the tree of the commit `Merge torrust/develop into 2758-cli-output-contract-tools-migration-spec` (develop with the EPIC specs; no code of this issue yet)
- Operating system / environment: Linux; stable Rust toolchain 1.99.1 (`rustc 1.99.1`, `cargo 1.99.1`); binaries built with `cargo build` (debug) and run directly from `target/debug/`, stdout and stderr redirected to files (stdout is not a terminal)
- Prerequisites and setup performed: none beyond the build

## Environment of the implementation runs (sections R1, G1 and V1 to V4)

- Date and time (UTC): R1 to V4 ran between 19:10 and 19:21 on 2026-11-18, except where a section gives a time. The linters passed at 19:18:53. The minute of each command was not recorded.
- Artifact under test: the working tree of the patch `feat(tracker-client): [#2768] refuse a terminal and write JSON only`, on top of `torrust/develop` at `7e8df95ec`. Binaries built with `cargo build` (debug), run as `target/debug/tracker_client`.
- Operating system / environment: Linux; rustc 1.100.0, cargo 1.100.0; `script(1)` gives the pseudo-terminal in V2 and V4; the real tracker is `target/debug/torrust-tracker` from the same tree with `share/default/config/tracker.development.sqlite3.toml` (UDP `0.0.0.0:6969`, HTTP `0.0.0.0:7070`, API `0.0.0.0:1212`), run from a scratch directory so the SQLite file is outside the repository.

## Verification Processes

### R0 - Reproduction of the deviation before the change (tracker_client)

> Historical: this section records the tool before the change of issue #2768. The sections below record the changed tool.

- Goal: show the current behavior that the issue changes, as `fix-bug` requires, before the code changes.
- Status: `DONE` (reproduction only; the red regression test, the fix and the final like-for-like recheck are recorded when the issue is implemented)

#### Steps Performed

```text
$ TORRUST_CHECKER_CONFIG='{"udp_trackers":[],"http_trackers":[],"health_checks":[]}' target/debug/tracker_client check > out 2> err
exit=0
stdout (3 bytes):
[]
stderr (0 bytes):
(empty)
```

```text
$ TORRUST_CHECKER_CONFIG='{"udp_trackers":[],"http_trackers":[],"health_checks":[]}' script -qec 'target/debug/tracker_client check; rc=$?; echo "rc=$rc"; exit "$rc"' /dev/null
[]
rc=0
script exit status: 0
```

#### Observed Result

`check` writes a top-level JSON array, not a single JSON object (Copilot's and da2ce7's finding), and with stdout on a pseudo-terminal it runs and exits `0` instead of refusing with exit `2`. The plain-text `tracing` on stdout and the direct prints of `http`/`udp` were not run here (they need a tracker); they are read from `unified/app.rs` (`tracing_subscriber::fmt()` writes to stdout) and the `print_*` allows in the console layer.

### R1 - Red run of the new tests against the unchanged tool

- Goal: show that the regression tests fail before the fix, as `fix-bug` requires.
- Status: `DONE`

#### Steps Performed

The new test file `console/tracker-client/tests/output_contract.rs` and the crate `Cargo.toml` (with the `rustix` dev-dependency) were copied into a detached worktree of `torrust/develop` (`7e8df95ec`, the tool without the change), the old `tracker_client` was built there, and the file was run with `cargo nextest run -p torrust-tracker-client --test output_contract --no-fail-fast`. The worktree was removed afterwards. The first attempt set `NEXTEST_BIN_EXE_tracker_client` to the old binary from outside, but nextest overwrites that variable, so all nine tests ran against the new binary and passed; that attempt proves nothing and is not used.

```text
FAIL it_should_report_a_usage_error_as_a_json_record_with_exit_2
FAIL it_should_report_help_as_a_stderr_record_and_leave_stdout_empty
FAIL it_should_write_the_check_results_as_one_json_object_in_both_formats
FAIL it_should_report_an_unrecognized_http_response_on_stderr_and_leave_stdout_empty
FAIL it_should_write_the_check_results_of_a_failing_tracker_inside_the_object
FAIL tty_refusal::it_should_refuse_before_parsing_the_arguments
FAIL tty_refusal::it_should_refuse_to_run_the_check_subcommand_when_stdout_is_a_terminal
FAIL tty_refusal::it_should_refuse_to_run_an_http_command_when_stdout_is_a_terminal
PASS it_should_report_a_runtime_failure_as_a_json_record_and_leave_stdout_empty
Summary: 9 tests run: 1 passed, 8 failed
```

#### Observed Result

Eight tests are red on the old tool. The ninth passes: a runtime failure on stderr as JSON with empty stdout already held, so that test guards it. The tests were written after the production change, not before it, so this red run was done afterwards against the old code, not as a step of the change.

### G1 - Green run

- Goal: the crate tests pass on the changed tool.
- Status: `DONE`

```text
$ cargo nextest run -p torrust-tracker-client
Summary [   2.032s] 71 tests run: 71 passed, 0 skipped
$ linter all
INFO torrust_linting::cli: All linters passed
$ cargo clippy --workspace --all-targets
Finished `dev` profile
```

The pseudo-terminal tests ran under nextest with its own `NEXTEST_BIN_EXE_tracker_client`. The first run of them hung because the test kept a copy of the pseudo-terminal slave open, so reading the master never ended; the test now drops every copy before it reads.

### V1 - Announce and scrape against a real tracker (M1)

- Status: `DONE`

```text
$ H=9c38422213e30bff212b30c360d26f9a02136422
$ target/debug/tracker_client udp announce udp://127.0.0.1:6969 $H > out 2> err
exit=0 stdout_bytes=107 stderr_lines=0
{"AnnounceIpv4":{"transaction_id":-888840697,"announce_interval":120,"leechers":0,"seeders":1,"peers":[]}}
$ target/debug/tracker_client udp scrape udp://127.0.0.1:6969 $H
exit=0 stdout_bytes=100 stderr_lines=0
{"Scrape":{"transaction_id":-888840697,"torrent_stats":[{"seeders":1,"completed":0,"leechers":0}]}}
$ target/debug/tracker_client http announce http://127.0.0.1:7070 $H
exit=0 stdout_bytes=178 stderr_lines=0
{"complete":2,"incomplete":0,"interval":120,"min interval":120,"peers":[{"ip":"127.0.0.1","peer id":[45,82,...],"port":56507}]}
$ target/debug/tracker_client http scrape http://127.0.0.1:7070 $H
exit=0 stdout_bytes=90 stderr_lines=0
{"9c38422213e30bff212b30c360d26f9a02136422":{"complete":2,"downloaded":0,"incomplete":0}}
$ jq -e 'type=="object"' <each stdout file>
true (four times)
```

Each stdout is one JSON object and each stderr is empty. The piped form `| jq .` of the README is the same, because stdout is then not a terminal.

### V2 - Terminal refusal (M2)

- Status: `DONE`

```text
$ script -qec "target/debug/tracker_client udp scrape udp://127.0.0.1:6969 $H 2>err; echo rc=\$?" /dev/null
rc=2
$ cat err
{"kind":"tty_refusal","message":"stdout is a TTY; pipe the output to consume result data"}
```

Nothing reached the terminal; the exit code is `2`; stderr holds the one record of the ADR.

### V3 - `check`, `monitor`, help, usage error (M3)

- Status: `DONE`

```text
$ CFG='{"udp_trackers":["127.0.0.1:6969"],"http_trackers":["http://127.0.0.1:7070"],"health_checks":["http://127.0.0.1:1212/api/health_check"]}'
$ TORRUST_CHECKER_CONFIG="$CFG" target/debug/tracker_client check --format json > out 2> err   # and --format text
fmt=json exit=0 stderr_bytes=0
fmt=text exit=0 stderr_bytes=0
$ jq -e 'type=="object" and (.results|type=="array")' out
true (both formats)
$ head -c 120 out
{"results":[{"Udp":{"Ok":{"remote_addr":"127.0.0.1:6969","results":[["Setup",{"Ok":null}],...

$ target/debug/tracker_client check monitor udp --url udp://127.0.0.1:6969 --interval 1 --timeout 1 --duration 2
exit=0
stdout: {"udp_trackers":[{"url":"udp://127.0.0.1:6969","status":{"code":"ok","message":"monitor completed","stats":{"total":2,...}}}]}
stderr: {"event":"probe","sequence":1,"url":"udp://127.0.0.1:6969","status":"ok","elapsed_ms":0}
        {"event":"probe","sequence":2,"url":"udp://127.0.0.1:6969","status":"ok","elapsed_ms":0}

$ target/debug/tracker_client --help
exit=0 stdout=0 bytes; stderr: {"kind":"help","message":"A collection of console clients ... \n\nUsage: tracker_client <COMMAND>\n..."}
$ target/debug/tracker_client bogus
exit=2 stdout=0 bytes; stderr: {"error":{"kind":"usage_error","source":"arguments","message":"error: unrecognized subcommand 'bogus' ..."}}
```

`check` writes one object, not an array, in both formats. The unrecognized-response path was exercised by an automated test with a fake server (G1); an attempt to serve a malformed answer with `nc` produced a transport error instead and is not evidence.

### V4 - The e2e runner in an interactive terminal (M4)

- Status: `DONE`

```text
$ script -qec 'cargo run -p torrust-tracker-e2e-tools --bin e2e_tests_runner -- --config-toml-path ./share/default/config/tracker.e2e.container.sqlite3.toml --tracker-image torrust-tracker:jobs-16 --skip-build; echo "runner_rc=$?"' /dev/null
... INFO ...::tracker_checker: stdout:
{"results":[{"Udp":{"Ok":{"remote_addr":"127.0.0.1:6969","results":[["Setup",{"Ok":null}],...
... INFO ...::tracker_checker: stderr:
Running `target/debug/tracker_client check`
... INFO ...::tracker_container: Removing docker tracker container: ...
runner_rc=0
```

The runner ran under a pseudo-terminal (`script`). The checker child exited `0`, its output came through the pipe, and the runner finished with exit `0` and no panic. The image was `torrust-tracker:jobs-16`, an existing local image of the tracker (not built from this tree), used with `--skip-build`: the step under test is the runner's launch of `tracker_client check`, not the tracker build. The run took about eight seconds after the build (19:20:49 to 19:20:57 UTC in the runner log).

### Observed deviations and leftovers

- The runtime-failure record of a failed request repeats the request URL in its message (seen while trying `nc`). It was like that before this change; the ADR redaction rule (section 7) suggests removing the URL. Not changed here (out of scope); reported as a follow-up.
