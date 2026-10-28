---
name: wait-for-pr-review
description: Wait for pull-request reviewers without spending tokens, using a deterministic Rust poller, then run the review loop on each event. Use after opening or updating a pull request when you must wait for Copilot or a maintainer review, instead of being told to wait. Triggers on "wait for review", "wait for da2ce7", "wait for Copilot review", "poll the PR", "block until reviewed", "review loop", or "wait-for-pr-review".
metadata:
  author: torrust
  version: "1.0"
  semantic-links:
    related-artifacts:
      - .github/skills/dev/pr-reviews/wait-for-pr-review/tool/Cargo.toml
      - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
      - .github/skills/dev/pr-reviews/fetch-review-threads/SKILL.md
      - .github/skills/dev/git-workflow/merge-pull-request/SKILL.md
      - docs/adrs/20260519000000_define_global_cli_output_contract.md
      - docs/issues/open/2581-2003-pr-review-wait-poller/ISSUE.md
---

# Waiting for Pull-Request Reviews

After you open or update a pull request, reviewers answer on their own schedule. Run the poller
instead of asking the contributor to tell you when to look, and instead of polling through an MCP
server, which spends inference tokens on every check. The poller is a Rust tool that calls no model,
holds no token, and returns only when something happens on the pull request.

This skill is the waiting half of the review loop. [`process-pr-review`](../process-pr-review/SKILL.md)
is the processing half.

## Prerequisites

- GitHub CLI (`gh`), authenticated. The poller refuses to run otherwise.
- The pull request number and the commit you last pushed.

## Run the Poller

Stdout carries the result and the poller refuses to run when stdout is a terminal. Redirect each
channel to a file, as the
[CLI output contract](../../../../../docs/adrs/20260519000000_define_global_cli_output_contract.md)
recommends for agents:

```sh
mkdir -p .tmp
cargo run -q -p wait-for-pr-review -- \
  --pr-number <PR> \
  --since-head-sha "$(git rev-parse HEAD)" \
  > .tmp/wait-for-pr-review.stdout 2> .tmp/wait-for-pr-review.stderr
echo "exit=$?"
cat .tmp/wait-for-pr-review.stdout
```

| Option                      | Default                                              | Meaning                                                                                                         |
| --------------------------- | ---------------------------------------------------- | --------------------------------------------------------------------------------------------------------------- |
| `--pr-number <N>`           | required                                             | Pull request to watch.                                                                                          |
| `--owner`, `--repository`   | `torrust`, `torrust-tracker`                         | Repository.                                                                                                     |
| `--reviewer <login>`        | `da2ce7`, `copilot-pull-request-reviewer[bot]`       | Whose reviews and comments count. Repeat the flag to watch several; passing it replaces the defaults.          |
| `--timeout-seconds <N>`     | 900 (15 minutes)                                     | Longest wait before a `timeout` result.                                                                         |
| `--poll-interval-seconds N` | 120 (2 minutes)                                      | Pause between two looks at the pull request.                                                                    |
| `--since-head-sha <sha>`    | the moment the poller starts                         | Count reviews and comments created after this commit was made. Pass the commit you just pushed.                 |

Pass `--since-head-sha "$(git rev-parse HEAD)"` after every push. Without it, a review that lands
between your push and the poller start is missed, because the baseline is the poller start.

If your shell tool caps a call below the 15-minute default, set `--timeout-seconds` under that cap
and run the poller again on a `timeout` result.

## Result

Stdout is exactly one JSON object whose `kind` says what happened. Every outcome, including a
timeout, exits `0`, so branch on `kind`.

| `kind`         | Meaning                                              | Other fields                                                                    |
| -------------- | ---------------------------------------------------- | ------------------------------------------------------------------------------- |
| `review`       | A watched reviewer submitted a review (any state).   | `reviewer`, `review_id`, `state`, `review_commit_id`, `url`, `head_sha`         |
| `comment`      | A watched reviewer commented.                        | `comment_kind` (`issue` or `review`), `reviewer`, `comment_id`, `url`, `head_sha` |
| `head_changed` | The head commit changed: someone pushed.             | `previous_head_sha`, `head_sha`                                                 |
| `pr_closed`    | The pull request was closed without merging.         | `head_sha`                                                                      |
| `pr_merged`    | The pull request was merged.                         | `head_sha`                                                                      |
| `timeout`      | Nothing happened within the wait.                    | `waited_seconds`, `head_sha`                                                    |

When several things happened at once, the poller reports the first in this order: closed or merged,
review, comment, head change. It reports one event; the review loop reads the whole pull request
afterward, so nothing is lost.

Stderr is NDJSON, one record per line, each with a `kind`: `wait_started` at the start, and on
failure one diagnostic.

| Exit code | Meaning                                                                                                                                         |
| --------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| `0`       | A result is on stdout (or help was printed).                                                                                                    |
| `1`       | Failure; stdout is empty. Diagnostic kinds: `gh_unavailable`, `gh_not_authenticated`, `gh_api_error`, `malformed_response`, `commit_date_error`, `deadline_exceeded`, `interrupted`, `output_error`. |
| `2`       | Usage error or TTY refusal (`usage_error`, `tty_refusal`).                                                                                      |

On exit `1`, read the `message` of the last stderr record. A signal (`SIGINT` or `SIGTERM`) stops the
poller and any `gh` call in flight and reports `interrupted`.

The `--timeout-seconds` budget is a hard bound. No request starts once it is spent, and a `gh` call
still running at that moment is killed. If that happens after the first look at the pull request, the
result is `timeout` (exit `0`). If it happens before, nothing was observed, so it is a
`deadline_exceeded` failure (exit `1`), not a `timeout` that would suggest a quiet pull request. The
last look is taken one poll interval or less before the budget ends, so rerun with `--since-head-sha`
and nothing in that gap is missed.

## The Review Loop

1. Push, then run the poller with `--since-head-sha` set to the pushed commit.
2. Branch on `kind`:
   - `review` or `comment`: run the [`process-pr-review`](../process-pr-review/SKILL.md) workflow.
     Fetch every thread with [`fetch-review-threads`](../fetch-review-threads/SKILL.md), read the new
     review body (a finding in a review body has no thread), fix, commit, push, and reply. Then
     re-request the review (see below) and go to step 1.
   - `head_changed`: someone else pushed. Pull the branch, then go to step 1 with the new head.
   - `timeout`: check the exit condition. If it is not met, go to step 1; after a few timeouts in a
     row, tell the contributor the reviewer has not answered.
   - `pr_closed` or `pr_merged`: stop.
3. Check the exit condition after each return. The loop is done when all of these hold:
   - Copilot's latest review is on the current head commit. Copilot never submits `APPROVED`, so its
     last review is `COMMENTED`; do not wait for an approval state.
   - The pull request has no unresolved actionable review thread anywhere, including threads on
     earlier commits. Count them over the whole pull request with `fetch-review-threads`.
   - The other maintainer has commented `ACK <full head commit hash>` on the head commit, or the
     maintainer has recorded a waiver, as the Merge Readiness section of
     [`merge-pull-request`](../../git-workflow/merge-pull-request/SKILL.md) describes.
4. When the exit condition holds, print the maintainer merge command from `merge-pull-request`. Do
   not merge, approve, or post `ACK` yourself; an agent never waives a check.

The poller returns events and does not decide the exit condition. A review-body finding with no
thread is not counted by the thread check, so read each new review body through the audit workflow.

## Re-Requesting a Review

After you push fixes for a reviewer, ask the same reviewer again. The poller never does this. This
call is the documented GitHub REST endpoint; it has not run against this repository yet.

```sh
gh api --method POST repos/torrust/torrust-tracker/pulls/<PR>/requested_reviewers \
  -f 'reviewers[]=da2ce7' > .tmp/re-request.stdout 2> .tmp/re-request.stderr
```

The Copilot re-review path is not verified. Record the verified command, or the confirmed
limitation, here once the manual verification of issue #2581 has run it.

## Skill Links

- `contrib/dev-tools/github/github-review-threads/` is the thread source the exit condition reads.
- `.github/skills/dev/pr-reviews/wait-for-pr-review/tool/src/main.rs` carries the option defaults and
  the exit codes this skill documents. Update both together.
