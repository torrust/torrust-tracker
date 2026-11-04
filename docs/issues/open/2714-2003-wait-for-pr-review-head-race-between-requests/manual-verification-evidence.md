---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2714-2003-wait-for-pr-review-head-race-between-requests/ISSUE.md
last-updated-utc: "2026-11-04 13:07"
---

# Manual Verification Evidence

## Purpose

Record real, human-oriented verification of the completed behavior. This is
evidence from commands or interactions actually performed against the artifact;
do not invent commands, output, logs, or results.

## Environment and Prerequisites

- Date and time (UTC): 2026-11-04 11:50
- Artifact under test: `wait-for-pr-review` as built from `develop` at `f7fecdf14`
- Operating system / environment: Linux 7.0.0-38-generic
- Rust toolchain: stable `rustc 1.99.1 (58cf8f9c3 2026-10-13)`
- Prerequisites and setup performed: `cargo build -p wait-for-pr-review`. The fake `gh` is
  `evidence/fake-gh-head-race.sh`, copied as `gh` into a temporary directory placed first on `PATH`, with
  `FAKE_GH_STATE` pointing at a temporary directory that holds its call counter. It answers every call the poller
  makes and never reaches the GitHub API. **The timing is scripted:** the script returns what GitHub could
  answer if a push and a review of that push happened between two of the poller's sequential calls. The race
  itself was not observed on GitHub.

## Verification Processes

### V1 - A review of a newer commit than the reported head (reproduction, before the fix)

- Goal: show whether a `review` result can carry a `head_sha` older than the commit the review is about, and
  whether `head_moved` then stays false for a checkout of that older head.
- Initial state: unfixed code. The fake `gh` reports head `aaaa…` (40 characters) in every read of the pull
  request, an empty review list on the first snapshot, and, from the second snapshot on, one review by `da2ce7`
  of commit `bbbb…` (40 characters).
- Status: `DONE`
- Outcome: **Reproduced**, with the API timing scripted. The wrong outcome itself was observed: the result.

#### Steps Performed

1. `wait-for-pr-review --pr-number 7 --poll-interval-seconds 1 --timeout-seconds 30` with the fake `gh` first on
   `PATH`; stdout to a file. No `--since-head-sha`, so the baseline head is the first read, `aaaa…`.
2. A short script printed the fields of the result.

#### Observed Result

```text
exit=0
{"kind":"review","reviewer":"da2ce7","review_id":1,"state":"COMMENTED","review_commit_id":"bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb","url":"https://example.test/review/1","head_sha":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","cursor":"2026-11-04T10:00:00Z"}
kind: review
head_sha            : aaaaaaaa ...
review_commit_id    : bbbbbbbb ...
head_moved present  : False (absent means false)
review is of the head the result reports: False
review-list calls the poller made: 2
```

#### Conclusion

The result reports head `aaaa…` and a review of commit `bbbb…`, and `head_moved` is absent, which means false.
For a checkout of `aaaa…`, the review loop's test ("`head_moved` is true, or `head_sha` differs from
`git rev-parse HEAD`") passes, so the loop would process a review of `bbbb…` on a tree that does not contain it.
The output is wrong for the scripted API responses; whether GitHub produces those responses in that order in
practice depends on timing that was not observed.

### V2 - The same scenario after the fix

- Goal: repeat V1 on the final build.
- Status: `TODO`

## Failures and Follow-up

None yet.
