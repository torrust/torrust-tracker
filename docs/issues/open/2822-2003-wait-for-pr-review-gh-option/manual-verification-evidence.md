---
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/open/2822-2003-wait-for-pr-review-gh-option/ISSUE.md
---

# Manual verification evidence for #2822

Toolchain: stable `rustc 1.99.1` for builds and tests; nightly `cargo +nightly fmt`. The real release binary
was run with `PATH=/usr/bin:/bin`, so the fake command was reachable only through `--gh-command`. The fake is a
disposable shell script kept in this folder as `evidence/fake-gh-command.sh` (shown below): it answers `api user`,
the pull request endpoint and the paginated feedback endpoints, and logs each call. Setup: copy it to a
directory outside `PATH`, `chmod +x` it, and pass its path to `--gh-command`; `touch user-hangs` or
`touch user-fails` next to it to make `api user` stall for 30 seconds or fail. No `gh` stand-in is on `PATH`.

```sh
#!/bin/sh
dir="$(dirname "$0")"
echo "$*" >> "$dir/calls"
case "$*" in
  "api user") if [ -f "$dir/user-hangs" ]; then sleep 30; fi; if [ -f "$dir/user-fails" ]; then exit 1; fi; echo '{"login": "some-account"}' ;;
  "api repos/torrust/torrust-tracker/pulls/7") echo '{"head": {"sha": "abc1234"}, "state": "closed", "merged": true}' ;;
  "api --paginate --slurp"*) echo "[[]]" ;;
  *) echo "unexpected: $*" >&2; exit 1 ;;
esac
```

## Red run (T2)

`cargo test --test cli` with the new tests and the updated existing scenarios, before the option existed:
23 passed, 14 failed. The failing tests were the 10 new ones (`--gh-command` was an unknown option, exit 2) and
the 4 existing scenarios that now expect `gh_identity`.

## Green run (T3)

`cargo test` after the implementation: 67 unit tests and 37 `tests/cli.rs` scenarios passed;
`cargo clippy --all-targets` and `cargo +nightly fmt --all -- --check` clean.

Flakiness note: the fake-`gh` scenarios of `tests/cli.rs` are known to be flaky (a separate issue). Over 7 full
runs of `cargo test --test cli` after the change, one run had one failing scenario (a new one, whose fake
command failed to start); the other 6 passed. The new scenarios use the same fake-script mechanism as the
existing ones and add no new waiting; the cause is not changed here.

## Manual scenarios (T4)

| ID | Command (stdout and stderr redirected) | Result |
| -- | -------------------------------------- | ------ |
| M1 | `wait-for-pr-review --pr-number 7 --gh-command <fake> --timeout-seconds 5` | `exit=0`, stdout `{"kind":"pr_merged","head_sha":"abc1234"}`. Calls logged: `api user`, three paginated feedback calls, `api repos/.../pulls/7`; no `auth status`. stderr: `wait_started` with `gh_command`, then `gh_identity` with `gh_login: "some-account"`. |
| M2 | `... --gh-command /nonexistent --timeout-seconds 5` | `exit=1`, empty stdout. stderr: `wait_started`, `gh_identity` with `gh_login: null`, then `gh_unavailable`: "`/nonexistent` cannot be started: No such file or directory (os error 2)". |
| M3 | M1 with the fake's `api user` failing | `exit=0`, the same result; `gh_identity` has `gh_login: null`. |
| M4 | M1 with `user-hangs` set (the fake's `api user` sleeps 30 s), `--timeout-seconds 60` | `exit=0`, `elapsed=10.31s`, stdout `{"kind":"pr_merged","head_sha":"abc1234"}`; stderr `wait_started` then `gh_identity` with `gh_login: null`. The lookup ended at its 10 s bound and the wait continued to the unchanged result. |
| M4b | The same with `--timeout-seconds 10` | `exit=0`, `elapsed=2.81s`, the same stdout; `gh_login: null`. The lookup ended at its 2.5 s bound. |

M4 and M4b were first run with an incomplete version of the fake that ended them in `gh_api_error` after the
lookup (the fake, not the poller); they were rerun with the script above and the table shows the rerun. The
`gh_api_error` message names the command, as D2 says.

## Signal during the lookup

`tests/cli.rs` `it_should_stop_on_sigterm_during_the_lookup_without_running_another_command` starts the poller
with the stalled lookup, sends `SIGTERM`, and asserts exit 1, one `interrupted` record, empty stdout, and that
the fake served no pull request call and no `auth status`. It was added after review and passes; it was not run
against the unfixed code.

## Test design review (prose-first Arrange-Act-Assert)

Done after the tests passed, before maintainer review. Each new scenario in `tests/cli.rs` was judged against its
prose and the code was left as it is where it already matched.

- Arrange: the `Sandbox` fixture still owns only incidental mechanics (the fake script, the pull request JSON,
  flag files). The new helpers are `fake_path` (where the script is), `calls` (what it was asked) and
  `command_without_fake_on_path` (a `PATH` that cannot reach it); each is a named action, not a copy of setup.
  The one causal difference of each scenario is visible in the test body: a flag file (`auth-fails`,
  `user-hangs`, `api-fails`), a `user.json`, or the `--gh-command` value.
- Act: always one run of the real binary with the arguments written in the test, so the option under test stays
  in sight.
- Assert: the expected exit code, the record kinds or the one field that matters, written in the test, not in a
  helper.
- Existing scenarios: only the expected stderr kinds (two scenarios) and the signal helper
  `start_and_wait_until_running` (it now also reads `gh_identity`, which the fake answers at once) changed.
