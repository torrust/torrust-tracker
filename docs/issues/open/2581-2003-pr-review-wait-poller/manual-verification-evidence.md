---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2581-2003-pr-review-wait-poller/ISSUE.md
last-updated-utc: "2026-10-28 11:55"
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
- Not yet run: V6 and V7 need this feature's pull request, which is not open yet.

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
{"kind":"timeout","waited_seconds":3,"head_sha":"0f5b7876f4b8e798fb9e889bd0172c417fa401fe"}
```

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
- Initial state: this feature's pull request is not open yet.
- Status: `TODO`

### V7 - Copilot re-review request path

- Goal: find out whether `POST repos/{owner}/{repo}/pulls/{n}/requested_reviewers` re-requests a
  Copilot review, and record the verified command or the limitation in the skill.
- Initial state: this feature's pull request is not open yet.
- Status: `TODO`

## Failures and Follow-up

No failure so far. V6 and V7 remain after the pull request opens.
