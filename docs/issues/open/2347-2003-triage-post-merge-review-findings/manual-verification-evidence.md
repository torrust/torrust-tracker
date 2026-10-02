---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2347-2003-triage-post-merge-review-findings/ISSUE.md
last-updated-utc: "2026-09-28 17:37"
---

# Manual Verification Evidence

<!-- cspell:ignore rawfile endswith -->

## Purpose

Record real, human-oriented verification of the completed behavior. This is
evidence from commands or interactions actually performed against the artifact;
do not invent commands, output, logs, or results.

## Environment and Prerequisites

- Date and time (UTC): 2026-09-28 10:56-10:57
- Artifact under test: the review threads of PRs #2290, #2293, #2300, #2313, and #2320 on GitHub,
  and the local #2347 branch at `docs(pr-reviews): [#2347] record PR #2320 post-merge findings`,
  rebased on `develop` at `8a953724`. It includes the five audit commits, from
  `docs(pr-reviews): [#2347] add PR #2290 post-merge review audit` through that one. They were
  pushed to PR #2363 after this verification.
- Operating system / environment: Linux local development workspace; `gh` authenticated as
  `josecelano`; stable Rust `1.98.1`, nightly Rust `1.100.0-nightly` (2026-09-23).
- Prerequisites and setup performed: the source comment IDs come from the spec's Finding
  Inventory, which is the source of record. `.tmp/c1e-replies.tsv`, the git-ignored T1 capture,
  held the same IDs.

## Verification Processes

### V1 - Tracking and Disposition Replies Visible (M1)

- Goal: Every one of the 32 threads shows a reply naming #2347.
- Initial state: T1 tracking replies were posted on 2026-09-26, and disposition replies on
  2026-09-28 at 10:28:56Z-10:29:54Z.
- Status: `DONE`

#### Steps Performed

1. For each PR, `cargo run -q --package github-review-threads -- fetch --pr-number <PR> --output-file .tmp/pr_threads_<PR>.json`,
   then `show --threads-file .tmp/pr_threads_<PR>.json > .tmp/show_<PR>.json`.
2. With `jq`, selected the threads whose first comment is one of the PR's source comment IDs,
   and counted three things: threads with a later `josecelano` comment containing `#2347`,
   threads with at least three comments, and resolved threads. `ids` holds the PR's source comment
   IDs, one per line:

   ```sh
   jq -r --arg pr "$pr" --rawfile ids <(printf '%s\n' $ids) '($ids|split("\n")|map(select(length>0))) as $want
     | [.threads[]|select((.comments[0].url|split("_r")[1]) as $c | $want|index($c))]
     | "\($pr): threads=\(length) with_2347_reply=\([.[]|select([.comments[1:][]|select(.author=="josecelano" and (.body|test("#2347")))]|length>=1)]|length) with_disposition_reply=\([.[]|select(.comments|length>=3)]|length) resolved=\([.[]|select(.isResolved)]|length)"' ".tmp/show_$pr.json"
   ```

#### Observed Result

```text
2290: threads=7 with_2347_reply=7 with_disposition_reply=7 resolved=0
2293: threads=8 with_2347_reply=8 with_disposition_reply=8 resolved=1
2300: threads=8 with_2347_reply=8 with_disposition_reply=8 resolved=0
2313: threads=7 with_2347_reply=7 with_disposition_reply=7 resolved=0
2320: threads=2 with_2347_reply=2 with_disposition_reply=2 resolved=0
```

#### Conclusion

All 32 threads show a reply naming #2347 and a disposition reply. The only resolved thread is
that of #2293 F3, whose fix was already on `develop` (T3 amendment 2). M1 passes.

### V2 - Thread States Match Dispositions (M2)

- Goal: After T7, only `FOLLOW_UP` threads are unresolved.
- Initial state: PR #2363 merged at 2026-09-28 17:00:27Z. Between 17:07:36Z and 17:07:57Z, josecelano posted the T7 reply on each of the 13 threads whose findings PR #2363 fixed. He then resolved those 13 threads and the 9 `NO_ACTION` threads. He had replied on the PR #2363 F15 thread and resolved it at 16:53:48Z.
- Status: `DONE`

#### Steps Performed

1. At 2026-09-28 17:19 UTC, the loop's orchestrator ran the GraphQL `repository.pullRequest.reviewThreads` query for PRs #2290, #2293, #2300, #2313, #2320, and #2363. It fetched each thread's node id, `isResolved`, path and line, and every comment's id, URL, author, time and body. The repository's `github-review-threads` tool was not run.
2. Each unresolved thread was mapped to its audit finding through the first comment id in the audit's `Source URL`. The owner named in its disposition reply was then read from the thread.

#### Observed Result

| PR | Threads | Unresolved | Unresolved threads (reviewer ID → audit ID, owner named in the disposition reply) |
| -- | ------- | ---------- | ------------------------------------------------------------------------------ |
| #2290 | 10 | 1 | loop F4 → F4, #2360 |
| #2293 | 16 | 2 | loop F1 → F1, #2361; loop F5 → F5, #2301 |
| #2300 | 12 | 1 | loop F10 → F10, #2278 order 8 |
| #2313 | 10 | 5 | loop F4 → F4, #2362; loop F5 → F5, #2362; loop F6 → F6, #2362; loop F7 → F7, #2362; loop F9 → F9, #2362 |
| #2320 | 29 | 0 | none |
| #2363 | 15 | 0 | none; F15 was resolved at 16:53:48Z |

#### Conclusion

The 9 unresolved threads are exactly the 9 `FOLLOW_UP` findings, and each has a disposition reply that names its owner. Every other one of the 32 threads is resolved: the 13 fixed findings, the 9 `NO_ACTION` findings, and #2293 F3. PR #2363 has no open thread. This matches the `FOLLOW_UP` set, and M2 passes.

### V3 - Live-Status Spot Check (M3)

- Goal: Re-run the recorded verification for three findings, one per disposition kind, and
  reproduce the recorded result.
- Initial state: selection by `random.Random(2347).choice(...)`, applied in order to three lists.
  The first list holds the 12 findings fixed in this PR with a documentation commit (excluding
  #2290 F5, whose fix is the audit itself, and #2293 F3, fixed upstream). The second holds the 9
  `NO_ACTION` findings, and the third the 9 findings owned by another issue. The picks were
  #2290 F6, #2300 F8, and #2290 F4.
- Status: `DONE`

#### Steps Performed

1. #2290 F6 and F4: removed the crate-level `clippy::empty_enums` allowance from
   `packages/udp-protocol/src/lib.rs` in the working tree, and ran
   `cargo +nightly clippy -p torrust-tracker-udp-protocol --all-targets --all-features --message-format=short -- -D warnings`
   and the same command with stable `cargo clippy`. Then restored the file with
   `git checkout -- packages/udp-protocol/src/lib.rs`; `git status --short` was empty.
2. #2300 F8: ran the script below at `478516cf` (the triage commit) and at `HEAD`. It extracts
   `## Status Values` from the template and from every `docs/pr-reviews/*/PR-REVIEW.md` at a
   revision, and lists the records whose section is byte-identical:

   ```python
   import re, subprocess, sys
   rev = sys.argv[1]
   def show(path):
       return subprocess.run(["git", "show", f"{rev}:{path}"], capture_output=True, text=True, check=True).stdout
   def section(text):
       m = re.search(r"^## Status Values\n(.*?)(?=^## |^<!--|\Z)", text, flags=re.M | re.S)
       return m.group(1).strip() if m else None
   paths = subprocess.run(["git", "ls-tree", "-r", "--name-only", rev, "docs/pr-reviews/"], capture_output=True, text=True, check=True).stdout.split()
   records = sorted(p for p in paths if p.endswith("/PR-REVIEW.md"))
   template = section(show("docs/templates/PR-REVIEW-TEMPLATE.md"))
   same = [p.split("/")[2] for p in records if section(show(p)) == template]
   print(f"{rev}: records={len(records)} byte-identical={len(same)}: {', '.join(same)}")
   ```

#### Observed Result

```text
nightly rc=101
      5 packages/udp-protocol/src/announce.rs
     11 packages/udp-protocol/src/common.rs
      1 packages/udp-protocol/src/connect.rs
      1 packages/udp-protocol/src/scrape.rs
18
stable rc=0
```

```text
478516cf: records=119 byte-identical=10: pr-2320-review, pr-2334-review, pr-2335-review, pr-2337-review, pr-2339-review, pr-2344-review, pr-2346-review, pr-2348-review, pr-2350-review, pr-2353-review
HEAD: records=124 byte-identical=14: pr-2290-review, pr-2293-review, pr-2320-review, pr-2334-review, pr-2335-review, pr-2337-review, pr-2339-review, pr-2344-review, pr-2346-review, pr-2348-review, pr-2350-review, pr-2352-review, pr-2353-review, pr-2355-review
```

#### Conclusion

- #2290 F6 and F4 reproduce: 18 unique nightly diagnostics in four modules, with `announce.rs`
  5, and a clean stable run.
- #2300 F8 reproduces at the triage commit: 10 records, `pr-2320` through `pr-2353`. At `HEAD`
  the two new audits and two later `develop` records also match, which is consistent with
  `NO_ACTION`: the finding is not live.

M3 passes.

## Automatic Checks

- `validate-audit-record.py` for each audit, at 10:57 UTC, with the first parent of each PR's
  merge commit as `--base`:
  - #2290: `--base 5c6dcb05^1` (`0ba8fa0f`): `{"status": "ok", "rows": 7, "log_entries": 6, "failures": 0}`
  - #2293: `--base bbcb34ff^1` (`63af2153`): `{"status": "ok", "rows": 8, "log_entries": 9, "failures": 0}`
  - #2300: `--base 3aeb62c5`: `{"status": "ok", "rows": 12, "log_entries": 6, "failures": 0}`
  - #2313: `--base 78d7e0fe`: `{"status": "ok", "rows": 10, "log_entries": 5, "failures": 0}`
  - #2320: `--base a110200d`: `{"status": "ok", "rows": 29, "log_entries": 32, "failures": 0}`
- The pre-commit hook (including `linter all` and the staged frontmatter check) passed on every
  commit. The pre-push hook passed on the first push to PR #2363.

## Failures and Follow-up

- The first run of the copied-section comparison with `sed` reported a false `DIFFERS`, because
  shell history expansion corrupted the `sed` range. A Python comparison replaced it. For the
  #2300 record, the `Status Values` and `Completion Rules` sections also differ at `develop`
  (they predate the current template); #2347 did not change them.
