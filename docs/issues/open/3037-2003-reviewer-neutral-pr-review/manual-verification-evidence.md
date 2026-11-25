---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/3037-2003-reviewer-neutral-pr-review/ISSUE.md
last-updated-utc: "2026-11-24 18:41"
---

# Manual Verification Evidence

## Purpose

Record real, human-oriented verification of the completed behavior. This is
evidence from commands or interactions actually performed against the artifact;
do not invent commands, output, logs, or results.

## Environment and Prerequisites

- Date and time (UTC): 2026-11-24, 18:35 to 18:45
- Artifact under test: the implementation branch `3037-reviewer-neutral-pr-review`, with the
  commits "feat(wait-for-pr-review): [#3037] watch every account except the PR author by default",
  "docs(watch-pull-request-checks): [#3037] leave review-service check runs out of readiness" and
  "docs(process-pr-review): [#3037] name the reviewer by login in new audit records".
- Operating system / environment: Linux 7.0.0-44-generic; stable Rust `rustc 1.100.0`;
  `gh version 2.76.2`; Python 3 for `validate-audit-record.py`.
- Prerequisites and setup performed: the poller ran as `josecelano-bot` through `--gh-command gh-bot`
  (its `gh_identity` record). The validator ran with the bot token set inline for that one command
  (`gh api user` printed `josecelano-bot`). The pull requests read are open or merged pull requests
  of this repository; nothing was written to GitHub.

## Verification Processes

### V1 - The poller wakes on a reviewer it was not told about (M1)

- Goal: without `--reviewer`, a review service's comment wakes the poller, and the author's own
  reviews and comments do not.
- Initial state: open PR #3042 (author `josecelano-bot`, head `faa090a00`). Items after 18:06:00 UTC:
  Greptile's summary comment (18:06:53) and inline comments (18:06:56), the author's two thread-reply
  reviews and inline comments (18:16:03, 18:16:05), and a maintainer comment by `josecelano`
  (18:34:47).
- Status: `DONE`

#### Steps Performed

1. `cargo run -q -p wait-for-pr-review -- --pr-number 3042 --since-head-sha faa090a00e0cf9e704477641033f8ff867e24505 --gh-command gh-bot --timeout-seconds 30 --poll-interval-seconds 10 --since 2026-11-24T18:06:00Z`
2. The same command with `--since 2026-11-24T18:07:00Z`, after Greptile's items, so only the
   author's items and the maintainer comment remain.

#### Observed Result

```text
# step 1, exit 0
{"kind":"comment","comment_kind":"issue","reviewer":"greptile-apps[bot]","comment_id":7140177515,"url":"https://github.com/torrust/torrust-tracker/pull/3042#issuecomment-7140177515","head_sha":"faa090a00e0cf9e704477641033f8ff867e24505","cursor":"2026-11-24T18:06:53Z"}
# stderr
{"kind":"wait_started","gh_command":"gh-bot","pr_number":3042,"watch":"all_except_author","reviewers":[],"timeout_seconds":30,"poll_interval_seconds":10}

# step 2, exit 0
{"kind":"comment","comment_kind":"issue","reviewer":"josecelano","comment_id":7140841857,"url":"https://github.com/torrust/torrust-tracker/pull/3042#issuecomment-7140841857","head_sha":"faa090a00e0cf9e704477641033f8ff867e24505","cursor":"2026-11-24T18:34:47Z"}
```

#### Conclusion

Met. Step 1 reported `greptile-apps[bot]`, a login no flag named. Step 2 skipped the author's
reviews and inline comments of 18:16 and reported the next item by another account.

### V2 - `--reviewer` still narrows (M2)

- Goal: with `--reviewer da2ce7`, the review service's items do not wake the poller.
- Initial state: as V1.
- Status: `DONE`

#### Steps Performed

1. The V1 step 1 command with `--reviewer da2ce7` added.

#### Observed Result

```text
# exit 0
{"kind":"timeout","waited_seconds":30,"head_sha":"faa090a00e0cf9e704477641033f8ff867e24505"}
# stderr
{"kind":"wait_started","gh_command":"gh-bot","pr_number":3042,"watch":"listed","reviewers":["da2ce7"],"timeout_seconds":30,"poll_interval_seconds":10}
```

#### Conclusion

Met: no event for `greptile-apps[bot]` or `josecelano`; the wait timed out.

### V3 - Readiness leaves review-service check runs out (M3)

- Goal: the commands documented in `watch-pull-request-checks` "Review-Service Check Runs" count
  only workflow checks and list the others.
- Initial state: PR #3042 while its CI was running; PR #3039 after its CI passed.
- Status: `DONE`

#### Steps Performed

1. `gh-bot pr checks 3042 --repo torrust/torrust-tracker --json name,bucket,workflow --jq '[.[] | select(.workflow != "")] | group_by(.bucket) | map({(.[0].bucket): length}) | add'`
2. `gh-bot pr checks 3042 --repo torrust/torrust-tracker --json name,bucket,workflow --jq '.[] | select(.workflow == "") | "\(.name) \(.bucket)"'`
3. Both commands on PR #3039 (run while the documentation was written).

#### Observed Result

```text
# PR 3042, step 1
{"pass":13,"pending":26,"skipping":1}
# PR 3042, step 2
CodeRabbit skipping
Greptile Review pass
# PR 3039
{"pass":1}
CodeRabbit skipping
Greptile Review pass
```

#### Conclusion

Met: both review-service check runs have an empty `workflow` and are listed apart; the workflow
counts do not include them.

### V4 - Old and new audit records validate (M4)

- Goal: the contract check passes on the new template and skill text, and the existing audit
  records stay valid.
- Initial state: the branch after the `process-pr-review` commit; `docs/pr-reviews/pr-3027-review/PR-REVIEW.md`
  is an existing record with the `Author class` column.
- Status: `DONE`

#### Steps Performed

1. `cargo run -q -p agent-review-report-contract` on the branch.
2. A negative check: a line `- Author class: \`Copilot\`` added to the template, the check run
   again, the template restored from a copy.
3. `validate-audit-record.py --pr-number 3027 --base torrust/develop` with the bot token inline.
4. The validator's own `findings_table_rows` and `ROW_RE` applied, in a Python session, to the
   pr-3027 record and to a copy whose third column holds `coderabbitai[bot]`.

#### Observed Result

```text
# step 1, exit 0
Agent review report contract check passed.
# step 2, exit 1
expected `docs/templates/PR-REVIEW-TEMPLATE.md` not to contain `- Author class:`
# step 3, exit 0
{"status": "ok", "rows": 1, "log_entries": 4, "failures": 0}
# step 4
legacy rows parse: True 1
new-column rows parse: True 1
| F1 | review-finding:pr-3027-f1 | coderabbitai[bot] | Minor | maintainability | ORIGINAL | FIXED | RESOLVED |
```

#### Conclusion

Met. The validator reads the table by position, so it needed no change: a login in the third column
parses like a class. The implementation pull request's own audit record, if a review produces one,
is validated in the review loop.

### V5 - A fresh agent follows the new skills (M5)

- Goal: an agent that reads only the updated skills processes a review service's findings, requests
  no review and does not wait for Copilot.
- Initial state: a fresh Claude Code subagent (Claude Opus 5.5) with no conversation context, told
  to read only `AGENTS.md` "After opening a PR" and the `wait-for-pr-review`,
  `watch-pull-request-checks`, `merge-pull-request` (Merge Readiness) and `process-pr-review` skills
  on the branch, and to contact nothing.
- Status: `DONE`

#### Steps Performed

1. The agent got this scenario: PR 5000 by `contributor-bot`, head `abc123`; 14 workflow checks
   passed; `CodeRabbit` check run pending and `Greptile Review` failed, both without a workflow; one
   `COMMENTED` review by `coderabbitai[bot]` with two unresolved inline findings; no other review,
   Copilot neither reviewed nor requested; no ACK.
2. It answered six questions: readiness, next steps, whether it waits for Copilot, `da2ce7` or the
   review-service check runs, the audit column, what wakes the poller without `--reviewer`, and
   anything unclear.

#### Observed Result

Summary of its answers, with the sections it cited:

- Not ready: the two CodeRabbit findings are unprocessed, and there is no ACK or waiver. The checks
  are fine: the two review-service check runs do not count.
- Next: process the review with `process-pr-review`, push, reply, resolve, watch the workflow checks
  and run the poller again. It requests no review and posts no `@coderabbitai review`; if a new look
  would help, it tells the maintainer which reviewer to ask.
- It waits for neither Copilot nor `da2ce7` nor the two check runs.
- Audit column `Reviewer` with `coderabbitai[bot]`.
- Without `--reviewer`, every account except the author wakes the poller.
- Unclear points it raised: (a) `--watch --fail-fast` in "Watching the Checks" stops on the failed
  `Greptile Review` run, and only `watch-pull-request-checks` said to leave it out; (b) which Copilot
  login goes in `Reviewer`; (c) whether to report to the maintainer when only the ACK is missing;
  (d) the skills say "contributor" where the maintainer's own rules say "maintainer".

#### Conclusion

Met. Points (a) and (b) were fixed in the same branch: "Watching the Checks" step 3 now says a
failed review-service check run is not a failure, and `process-pr-review` step 2 says the login is
the one GitHub shows on the comment or review body. See Failures and Follow-up for (c) and (d).

## Failures and Follow-up

- V5 (c), when to send the hand-over report while only the ACK is missing, is the hand-over report
  of #2977, which this issue leaves to it (AC7). Reported to that issue.
- V5 (d), "contributor" in the skills: the skills are written for every contributor, and the person
  asked is the contributor who runs the agent. Not changed here.
