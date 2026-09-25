---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2333-2278-fetch-all-review-threads/ISSUE.md
last-updated-utc: "2026-09-24 18:46"
---

# Manual Verification Evidence

## Purpose

Record real, human-oriented verification of the completed behavior. This is
evidence from commands or interactions actually performed against the artifact;
do not invent commands, output, logs, or results.

## Environment and Prerequisites

- Date and time (UTC): 2026-09-24 18:40
- Artifact under test: #2333 implementation on branch `2333-2278-fetch-all-review-threads`, commits
  `feat(dev-tools): return all review threads with resolution evidence` and
  `docs(pr-reviews): collect all review threads as self-audit evidence`
- Operating system / environment: Linux; GitHub CLI authenticated as `josecelano`
- Rust toolchain: stable `rustc 1.98.1 (48a229cea 2026-09-01)`
- Prerequisites and setup performed: chose merged PR #2320 because a GraphQL survey of the last 40
  pull requests showed it holds every required state: 29 threads, 27 resolved, 2 unresolved, and
  21 outdated. Every command below is read-only; the resolver ran with `--dry-run`.

## Verification Processes

### V1 - Capture All Threads (M1)

- Goal: `fetch` and default `list` return resolved, unresolved, and outdated threads.
- Initial state: PR #2320 on GitHub with the thread counts above.
- Status: `DONE`

#### Steps Performed

1. `cargo run -q --package github-review-threads -- fetch --pr-number 2320 --output-file .tmp/pr_threads_2320.json`
2. `cargo run -q --package github-review-threads -- list --threads-file .tmp/pr_threads_2320.json`,
   counted with `jq`.

#### Observed Result

```text
{"status":"ok","pr_number":2320,"output_file":".tmp/pr_threads_2320.json"}
{"total":29,"resolved":27,"unresolved":2,"outdated":21,"resolvedAndOutdated":21}
```

#### Conclusion

Met. The default view holds all 29 threads, matching the survey's counts for resolved, unresolved,
and outdated threads.

### V2 - Inspect Resolution Evidence (M2)

- Goal: `show` identifies who resolved a thread and where; absent values are explicit nulls.
- Initial state: the V1 capture.
- Status: `DONE`

#### Steps Performed

1. `cargo run -q --package github-review-threads -- show --threads-file .tmp/pr_threads_2320.json`,
   selecting the first resolved thread, the first resolved outdated thread, and summary counts with `jq`.

#### Observed Result

```text
{"id":"PRRT_kwDOGp2yqc6lKZ2q","isResolved":true,"isOutdated":false,"resolvedBy":"josecelano","path":"packages/udp-server/benches/udp_tracker_server_benchmark.rs","line":70,"comments":2}
{"id":"PRRT_kwDOGp2yqc6lKZ3H","isOutdated":true,"resolvedBy":"josecelano","path":"packages/udp-server/benches/udp_tracker_server_benchmark.rs","line":null}
{"resolvers":["josecelano"],"nullResolver":0,"nullLine":21}
```

#### Conclusion

Met. A current resolved thread reports its resolver, path, and line. Each of the 21 outdated
threads reports `line: null` rather than a guessed line, and keeps its path and resolver. No
thread in this capture had a null resolver; that case is covered by the
`it_should_keep_a_missing_resolver_as_an_explicit_null` fixture test.

### V3 - Action-Only Views (M3)

- Goal: `list --unresolved-only` and `reply-status` return only unresolved threads.
- Initial state: the V1 capture.
- Status: `DONE`

#### Steps Performed

1. `cargo run -q --package github-review-threads -- list --threads-file .tmp/pr_threads_2320.json --unresolved-only`
2. `cargo run -q --package github-review-threads -- reply-status --threads-file .tmp/pr_threads_2320.json --login josecelano`,
   with stdout and stderr redirected to files.

#### Observed Result

```text
[{"id":"PRRT_kwDOGp2yqc6liGT6","isResolved":false,"isOutdated":false,"line":500},{"id":"PRRT_kwDOGp2yqc6liGUA","isResolved":false,"isOutdated":false,"line":64}]
reply-status exit=1
{"kind":"missing_reply","total":2,"ids":["PRRT_kwDOGp2yqc6liGT6","PRRT_kwDOGp2yqc6liGUA"]}
```

#### Conclusion

Met. Both action views contain exactly the two unresolved threads. `reply-status` exits `1` with a
`missing_reply` diagnostic because neither of those threads has a `josecelano` comment; it checked
2 threads, not 29.

### V4 - Downstream Consumer (M4)

- Goal: the bulk resolver accepts the new response file.
- Initial state: the V1 capture, which now carries `line` and `resolvedBy` on each thread.
- Status: `DONE`

#### Steps Performed

1. `bash .github/skills/dev/pr-reviews/resolve-review-threads/scripts/resolve-all-unresolved-threads.sh --dry-run --threads-file .tmp/pr_threads_2320.json`

#### Observed Result

```text
{"status":"dry-run","thread_id":"PRRT_kwDOGp2yqc6liGT6"}
{"status":"dry-run","thread_id":"PRRT_kwDOGp2yqc6liGUA"}
script exit=0
```

#### Conclusion

Met. The resolver reads the extended file unchanged and lists the same two unresolved thread IDs.

## Failures and Follow-up

No process failed. The survey found merged PRs that still hold unresolved threads: #2290 (7),
PR #2293 (8), #2300 (8), #2313 (7), and #2320 (2). They were only read here. Any action on them
falls under the post-merge review rule in `process-pr-review` and needs maintainer approval.
