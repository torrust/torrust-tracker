---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2761-2758-reply-status-result/ISSUE.md
last-updated-utc: 2026-11-06 21:04
---

# Manual Verification Evidence

## Environment and Prerequisites

- Date and time (UTC): 2026-11-06 17:17 to 17:18
- Artifact under test: `feat(github-review-threads): [#2761] report a missing reply as a result with exit 0` (working tree before commit)
- Operating system / environment: Linux, `cargo build -p github-review-threads` (debug), stdout piped (the tool refuses a TTY)
- Prerequisites and setup performed: fixture `contrib/dev-tools/github/github-review-threads/tests/fixtures/review-threads.json` (`F` below); `B` is `target/debug/github-review-threads`.

## Verification Processes

### V1 - Thread without a reply (M1, AC1, AC2)

- Goal: a missing reply is a stdout result with exit 0, and `jq -e` can branch on `kind`.
- Initial state: fixture with two unresolved threads; login `author` replied to one, `reviewer` to both.
- Status: `DONE`

#### Steps Performed

1. `B reply-status --threads-file F --login author | jq -c .kind` and `${PIPESTATUS[0]}`.
2. `B reply-status --threads-file F --login author | jq -e '.kind=="missing_reply"'`.
3. `B reply-status --threads-file F --login reviewer | jq -c '{kind,summary}'`.
4. `B reply-status --threads-file F --login author | jq -c '.threads[]|select(.has_reply|not)|.thread_id'`.

#### Observed Result

```text
"missing_reply"
tool exit: 0
jq-e: 0
{"kind":"all_replied","summary":{"total":2,"with_reply":2,"without_reply":0}}
exit 0
"THREAD_UNRESOLVED_CURRENT"
```

#### Conclusion

Met: both kinds exit 0 with the result on stdout; `jq -e` branches on `kind`.

### V2 - Usage error and could-not-answer (M2, AC3)

- Goal: usage error exits 2 and an unreadable file exits 1, both with empty stdout and a JSON diagnostic.
- Initial state: none.
- Status: `DONE`

#### Steps Performed

1. `B reply-status --threads-file /nope --login a 2>e3 | wc -c`, then the exit status and `cat e3`.
2. `B reply-status > /dev/null 2>e2`, then the exit status and the start of `e2`.

#### Observed Result

```text
0
exit 1
{"kind":"reply_status_error","message":"cannot read review-thread response: No such file or directory (os error 2)"}
exit 2
{"kind":"usage_error","message":"error: the following required arguments were not provided:\n  --threads-file <THREADS_FILE>\n  --login <LOGIN> ...
```

#### Conclusion

Met: stdout was empty (0 bytes) in both cases; exit codes 1 and 2 with JSON diagnostics on stderr.

### V3 - Real PR capture (review finding F5)

- Goal: run `reply-status` against a real fetched PR response, not the synthetic fixture.
- Initial state: `github-review-threads fetch --pr-number 2775` (as `josecelano-bot`) wrote `/tmp/pr_threads_2775.json` at about 21:00 UTC, after Copilot's first review of PR #2775 left 6 unresolved threads and before any reply was posted.
- Status: `DONE`

#### Steps Performed

1. `reply-status --threads-file /tmp/pr_threads_2775.json --login josecelano-bot`.
2. The one login that authored comments in the capture was found with `jq` over `.author.login` (`copilot-pull-request-reviewer`), then `reply-status ... --login copilot-pull-request-reviewer`. Both ran at 21:02:17 UTC.

#### Observed Result

```text
{"kind":"missing_reply","summary":{"total":6,"with_reply":0,"without_reply":6}}   (login josecelano-bot, exit 0)
{"kind":"all_replied","summary":{"total":6,"with_reply":6,"without_reply":0}}     (login copilot-pull-request-reviewer, exit 0)
```

#### Conclusion

Met: on a real GraphQL capture both kinds are results with exit 0 and the counts match the 6 unresolved threads. The capture itself is not stored in the repository (it changes as the PR is reviewed).

## Failures and Follow-up

None.
