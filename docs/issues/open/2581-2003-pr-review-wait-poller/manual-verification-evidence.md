---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2581-2003-pr-review-wait-poller/ISSUE.md
last-updated-utc: "2026-10-28 16:40"
---

# Manual Verification Evidence

## Purpose

Record real, human-oriented verification of the completed behavior. This is
evidence from commands or interactions actually performed against the artifact;
do not invent commands, output, logs, or results.

## Environment and Prerequisites

- Date and time (UTC): 2026-10-28 11:52 to 11:55
- Artifact under test: the #2581 implementation on branch `2581-pr-review-wait-poller`, up to the
  commit `docs(skills): [#2581] add the wait-for-pr-review skill and list the poller`
- Operating system / environment: Linux; GitHub CLI 2.76.2 authenticated as `josecelano`
- Rust toolchain: stable `rustc 1.99.1 (58cf8f9c3 2026-10-13)`
- Prerequisites and setup performed: `cargo build -p wait-for-pr-review`. Every run below is
  read-only against the GitHub API. The binary refuses a terminal on stdout, so each run redirects
  stdout and stderr to files under `.tmp/`; the relevant output is copied here.
- V6 and V7 need this feature's pull request. It is open now (PR #2683); their status is below.

## Verification Processes

### V1 - A merged pull request returns at once

- Goal: the poller reports `pr_merged` without waiting.
- Initial state: PR #2584 is merged.
- Status: `DONE`

#### Steps Performed

1. `target/debug/wait-for-pr-review --pr-number 2584 > .tmp/m1.out 2> .tmp/m1.err; echo "exit=$?"`

#### Observed Result

```text
exit=0
{"kind":"pr_merged","head_sha":"0f6ae32356c27cbd0f9dad53e5ddf19e367b2e35"}
{"kind":"wait_started","pr_number":2584,"reviewers":["da2ce7","copilot-pull-request-reviewer[bot]"],"timeout_seconds":900,"poll_interval_seconds":120}
```

The first line is stdout; the second is stderr.

#### Conclusion

Met: one JSON object on stdout, exit `0`, and the `wait_started` record on stderr.

### V2 - A real Copilot review is detected

- Goal: the poller returns a real `copilot-pull-request-reviewer[bot]` review, with state
  `COMMENTED`, when the baseline is an early commit of the pull request.
- Initial state: open PR #2678 with Copilot reviews on it.
- Status: `DONE`

#### Steps Performed

1. `first=$(gh api repos/torrust/torrust-tracker/pulls/2678/commits --jq '.[0].sha')` gave
   `f8fae454cccd3dc3e93626cc7f0c8d2ed28f8294`.
2. `target/debug/wait-for-pr-review --pr-number 2678 --since-head-sha $first --timeout-seconds 30 > .tmp/m2.out 2> .tmp/m2.err; echo "exit=$?"`

#### Observed Result

```text
exit=0
{"kind":"review","reviewer":"copilot-pull-request-reviewer[bot]","review_id":5680948571,"state":"COMMENTED","review_commit_id":"669411a583a3709543c92ff00c87c3a0f195c67f","url":"https://github.com/torrust/torrust-tracker/pull/2678#pullrequestreview-5680948571","head_sha":"991f75bc413a83534b6895fd7d0fff0864053e5a"}
```

#### Conclusion

Met: the review is returned as one `review` result with `state` `COMMENTED`, which also confirms
the `--since-head-sha` baseline reaches reviews created after that commit. This run covers the
Copilot half of the "real review" scenario; V6 covers `da2ce7`.

### V3 - A quiet pull request times out as a result

- Goal: the poller reports `timeout` with exit `0` when nothing happens.
- Initial state: open PR #2622 (a Dependabot update); `--reviewer nobody-at-all` makes the
  watched set match no real account, so no review or comment can end the wait.
- Status: `DONE`

#### Steps Performed

1. `target/debug/wait-for-pr-review --pr-number 2622 --reviewer nobody-at-all --poll-interval-seconds 1 --timeout-seconds 3 > .tmp/m3.out 2> .tmp/m3.err; echo "exit=$?"`

#### Observed Result

```text
exit=0
{"kind":"timeout","waited_seconds":3,"head_sha":"<head commit of PR #2622, elided>"}
```

The `head_sha` value is replaced by a placeholder because a PR head commit id is not durable; the rest of the line is verbatim.

#### Conclusion

Met: the timeout is a result on stdout with exit `0`, as the maintainer decided.

### V4 - An API failure is a failure

- Goal: an unusable `--since-head-sha` commit exits `1` with empty stdout and a JSON diagnostic.
- Initial state: a well-formed SHA that does not exist in the repository.
- Status: `DONE`

#### Steps Performed

1. `target/debug/wait-for-pr-review --pr-number 2622 --since-head-sha deadbeefdeadbeefdeadbeefdeadbeefdeadbeef --timeout-seconds 3 > .tmp/m4.out 2> .tmp/m4.err; echo "exit=$?"; cat .tmp/m4.out; tail -n1 .tmp/m4.err`

#### Observed Result

```text
exit=1
{"kind":"gh_api_error","message":"`gh api repos/torrust/torrust-tracker/commits/deadbeefdeadbeefdeadbeefdeadbeefdeadbeef` failed: gh: No commit found for SHA: deadbeefdeadbeefdeadbeefdeadbeefdeadbeef (HTTP 422)"}
```

`cat .tmp/m4.out` printed nothing; the record above is the last stderr line.

#### Conclusion

Met: exit `1`, empty stdout, and the `gh` error in a `gh_api_error` record.

### V5 - TTY refusal, usage error, and signals

- Goal: stdout on a terminal exits `2`; a missing argument exits `2`; `SIGTERM` and `SIGINT` stop a
  real wait promptly with `interrupted` and exit `1`.
- Initial state: a Python script gave the binary a pseudo-terminal as stdout for the first step and
  pipes for the rest. The signal steps ran against PR #2622 with `--reviewer nobody`, so the wait
  was still running when the signal arrived.
- Status: `DONE`

#### Steps Performed

1. Ran the binary with `--pr-number 2622` and stdout set to a pty.
2. Started the binary with `--pr-number 2622 --reviewer nobody`, waited 3 seconds, sent `SIGTERM`,
   and collected the exit code, stdout, and last stderr line. Repeated with `SIGINT`.
3. Ran the binary with no arguments.

#### Observed Result

```text
--- TTY refusal (stdout on a pty)
exit 2
stderr {"kind":"tty_refusal","message":"stdout is a TTY; pipe the output to consume result data"}
--- SIGTERM during a real wait
exit 1 stdout bytes 0 stopped after 3.1s
{"kind":"interrupted","message":"interrupted by a signal"}
--- SIGINT during a real wait
exit 1 stdout bytes 0 stopped after 3.0s
{"kind":"interrupted","message":"interrupted by a signal"}
--- usage error
exit 2 {"kind":"usage_error","message":"error: the following required arguments were not provided:\n  --pr-number <PR_NUMBER>\n
```

#### Conclusion

Met: exit codes `2`, `1`, `1`, and `2`; stdout empty on every failure; every stderr line is JSON.
The automated suite also covers a hung `gh` call being killed on a signal
(`it_should_not_wait_for_a_hung_gh_call_when_a_signal_arrives`).

### V6 - A real `da2ce7` review

- Goal: the poller returns on a real `da2ce7` review of this feature's pull request.
- Initial state: PR #2683 is open and `da2ce7` is a requested reviewer (GitHub timeline event
  `review_requested` at 2026-10-28T12:19:58Z). No `da2ce7` review exists yet.
- Status: `TODO`

### V7 - Copilot re-review request path

- Goal: find out whether `POST repos/{owner}/{repo}/pulls/{n}/requested_reviewers` re-requests a
  Copilot review, and record the verified command or the limitation in the skill.
- Initial state: PR #2683 had one Copilot review (5682635980, submitted 2026-10-28T12:34:22Z).
  The fixes for its four findings were pushed before the request.
- Status: `DONE`

#### Steps Performed

1. At 12:54:25 UTC: `gh api --method POST repos/torrust/torrust-tracker/pulls/2683/requested_reviewers -f 'reviewers[]=copilot-pull-request-reviewer[bot]'` (exit `0`, JSON of the pull request returned).
2. `gh api repos/torrust/torrust-tracker/pulls/2683 --jq '{requested:[.requested_reviewers[].login]}'` right after.
3. `gh api repos/torrust/torrust-tracker/issues/2683/timeline --paginate --jq '.[]|select(.event=="review_requested")|...'`.
4. Two poller runs with `--since-head-sha` set to the pushed head, one after the other: 12:54:43 to 13:09:43 UTC and 13:09:55 to 13:24:55 UTC.
5. At 13:25:15 UTC, the GraphQL route that earlier audits of PRs #2671, #2676, and #2679 name: the
   `requestReviews` mutation with `botIds` set to the node ID of the `copilot-pull-request-reviewer`
   bot (read from the existing Copilot review's author) and `union: true`. Then a GraphQL `reviewRequests` query.

#### Observed Result

```text
step 2: {"requested":["da2ce7"]}
step 3 (last events): 2026-10-28T12:16:59Z maintainers / 2026-10-28T12:19:58Z da2ce7 / 2026-10-28T12:54:26Z Copilot
step 4: {"kind":"timeout","waited_seconds":900,"head_sha":"<head of PR #2683, elided>"}   (both runs)
step 5: {"data":{"requestReviews":{"pullRequest":{"number":2683}}}}
step 5, reviewRequests: Team maintainers, User da2ce7, Bot copilot-pull-request-reviewer
```

#### Later observations

Copilot submitted review 5683463670 at 13:34:16 UTC, 39 minutes 50 seconds after the REST request at
12:54:26 UTC, on the head that was current when the request was made. Both poller windows had ended
by then (13:09:43 and 13:24:55 UTC), so neither returned it; it was read from GitHub at 14:01 UTC. No
Copilot review exists on the head of the earlier push of the spec-refinement commit, so the push alone
did not produce one. A second request, made through the REST endpoint at 14:13:25 UTC after the next
push, was recorded as a `review_requested` event for `Copilot` as well.

#### Conclusion

Met. `POST repos/{owner}/{repo}/pulls/{n}/requested_reviewers` with
`reviewers[]=copilot-pull-request-reviewer[bot]` requests a Copilot review: the timeline records it,
the GraphQL `reviewRequests` list shows the bot while the request is pending, and Copilot answered. The
REST `requested_reviewers` list never shows bots, so it cannot confirm the request. The latency was
about 40 minutes, longer than the poller's 15-minute default. The GraphQL `requestReviews` mutation
that earlier audits used was also run once (13:25:15 UTC, `union: true`); it returned success and
added no second event while the first request was pending, so it was not needed. The skill records the
verified command and these limits.

### V8 - Unrelated CI failure: `base_source_precedence` fails with `Address already in use`

This is not part of the poller. The Testing workflow of PR #2683 failed in `Unit (stable)` on a test
that this pull request does not touch, so the cause was investigated and fixed here in its own commit,
following the `fix-bug` skill.

- Goal: the executable-boundary configuration scenarios no longer fail because another process holds
  the port they hard-code.
- Initial state: `tests/configuration/cli_configuration/` hard-coded `43152` to `43154`
  (`base_source_precedence`) and `43155` and `43156` (`per_value_overrides`).
- Status: `DONE`

#### Symptom and hypothesis

Run 40691707586 (workflow Testing, job `Unit (stable)`, 2026-10-28) failed on
`base_source_precedence::it_should_select_the_cli_configuration_when_both_child_environment_base_sources_are_set`
with `tracker exited before readiness with exit status: 1` and the tracker's own error `Could not bind
the health check API listener ... Address already in use (os error 98)`. The same test failed with the
same error in run 38925732709 (2026-10-16, a push to `develop`) and run 37060706317 (2026-10-02, a pull
request), so it is not caused by PR #2683, which adds no tracker code. The only port the scenario binds
is the winning CLI one, `43152`. It lies inside Linux's default ephemeral range (`32768` to `60999`,
read from `/proc/sys/net/ipv4/ip_local_port_range` on the machine used here), which the kernel hands
to port-zero bindings and to the source side of outgoing connections. Hypothesis: something on the
runner holds the port when the tracker binds it. The holder in the CI runs was not identified.

#### Reproduction (before the fix)

Toolchain `rustc 1.99.1 (58cf8f9c3 2026-10-13)`, code at the commit `docs(pr-reviews): [#2581] record
the first Copilot review of PR 2683` (tests unchanged). A Python process bound `127.0.0.1:43152` and
ran `cargo test -p torrust-tracker --test cli-configuration`.

```text
baseline, port free:   test result: ok. 25 passed; 0 failed
port 43152 held:       base_source_precedence::it_should_select_the_cli_configuration_when_both_child_environment_base_sources_are_set ... FAILED
                       ... Could not start the health check API: Could not bind the health check API listener. Check that its bind address is available: Address already in use (os error 98)
                       test result: FAILED. 24 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

Outcome: **Reproduced** with an induced collision; the failure and its error text match the three CI
runs. What held the port in CI is not known.

#### Regression-test boundary

The defect is that a scenario names a port that anything may hold. The smallest deterministic
maintained tests are unit tests of a shared fixture helper, `reserve_free_ports`, in
`tests/common/native_tracker/ports.rs`, plus the scenarios themselves using it:

- `it_should_reserve_only_ports_below_the_default_ephemeral_range` guards the cause: it fails if the
  helper draws from the range the kernel assigns by default.
- `it_should_reserve_the_requested_number_of_distinct_ports`, `it_should_skip_a_candidate_that_a_live_listener_holds`,
  `it_should_return_nothing_when_there_are_no_candidates`, and
  `it_should_refuse_to_reserve_more_ports_than_it_can_search_for` guard the helper contract.

A first design reserved ports from the kernel with port zero. It was rejected before commit: a
port-zero port comes from the same ephemeral range, so it only narrows the window, and its tests
raced the other tests in the process (one run failed with `reserved port 33495 should be free to bind:
Address already in use`). The helper now draws random ports from `10000..32768`, below the range, and
probes each by binding it.

No maintained test stops someone from writing a literal port in a new scenario; that stays a review
matter.

#### Red proof

With `SCENARIO_PORTS` mutated to `40_000..50_000` (not staged, then restored byte for byte; the test
had the name `it_should_reserve_only_ports_below_the_ephemeral_range` then and was renamed later):

```text
test native_tracker::ports::tests::it_should_reserve_only_ports_below_the_ephemeral_range ... FAILED
port 44324 is outside the scenario range and could be handed out by the kernel
test result: FAILED. 28 passed; 1 failed
```

The test `it_should_refuse_to_reserve_more_ports_than_it_can_search_for`, added for a review finding,
failed before its fix with the panic message `the loopback interface should have a free port below the
ephemeral range` where `cannot reserve 1001 ports with a budget of 1000 draws` was expected.

The scenario tests were also run against the first design with the old ports as its return value and
`43152` held: `it_should_reserve_ports_that_can_be_bound_once_the_call_has_returned` and
`base_source_precedence` failed. That test was later removed with the design.

#### Green and like-for-like recheck (after the last change to the fix)

The fixture changed again after a Copilot review (the commit
`fix(tests): [#2581] name the real cause when a port reservation cannot be satisfied`), so the recheck
was repeated on the build that includes it.

```text
cargo test -p torrust-tracker --test cli-configuration      test result: ok. 30 passed; 0 failed
cargo test -p torrust-tracker --test lifecycle-signals      test result: ok. 25 passed; 0 failed
cargo clippy -p torrust-tracker --tests                     no warning or error
```

The reproduction was repeated: ports `43152` to `43156` all held by another process, then the same
`cargo test -p torrust-tracker --test cli-configuration`: 30 passed, 0 failed, with both scenarios `ok`.
The full suite was then run 40 more times with the old ports held: 0 failed runs. Before the fix,
holding `43152` alone failed the run both times it was tried.

#### CI after the fix

The Testing workflow re-ran on the pushed fix (run 40698289978) and `Unit (stable)` passed, with every
other check on the pull request. One passing run does not show that a flake is gone; the local runs
above, with the old ports held, are the evidence for the fix.

#### Conclusion

Met for the failure that was reproduced and for the cause the evidence supports. Not shown: that the
ephemeral range is what caused the failures in CI, since the holder there was not observed. A port can
still be taken between the probe and the tracker's bind by a process that listens on that exact port;
the range is below what the kernel assigns on a default Linux configuration, and concurrent runs draw
different random ports. A host that lowers `net.ipv4.ip_local_port_range` below `32768` narrows that
guarantee; the fixture documents it and does not read the setting.

## Failures and Follow-up

No failure of the poller remains open. Copilot's three reviews raised twelve findings (ten distinct, two duplicates), all addressed (see the audit in `docs/pr-reviews/pr-2683-review/PR-REVIEW.md`). V6 is open: it waits for a `da2ce7` review. V8 is an unrelated pre-existing flake fixed in its own commit.
