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
| `--since-head-sha <sha>`    | the head at the start of the wait                    | The commit you have checked out. A pull request head that differs from it is reported as `head_changed`. Unless `--since` is given, reviews and comments created after this commit was made count as new. |
| `--since <utc time>`        | the moment the poller starts                         | Count reviews and comments created strictly after this time. Pass the `cursor` of the previous result. Combine it with `--since-head-sha`, so a head that moved is not adopted silently. |

Always pass `--since-head-sha "$(git rev-parse HEAD)"`, and add `--since <cursor>` when resuming
after an event that needed no push (see [The Review Loop](#the-review-loop)). `--since-head-sha` tells
the poller which commit your checkout has, so it reports `head_changed` when the pull request is
ahead of you. Without it, the baseline is the head at the poller start, a push that happened before
that is never reported, and a review that landed between your push and the poller start is missed.

If your shell tool caps a call below the 15-minute default, set `--timeout-seconds` under that cap
and run the poller again on a `timeout` result.

## Result

Stdout is exactly one JSON object whose `kind` says what happened. Every outcome, including a
timeout, exits `0`, so branch on `kind`.

| `kind`         | Meaning                                              | Other fields                                                                    |
| -------------- | ---------------------------------------------------- | ------------------------------------------------------------------------------- |
| `review`       | A watched reviewer submitted a review (any state).   | `reviewer`, `review_id`, `state`, `review_commit_id`, `url`, `head_sha`, `cursor`, `head_moved` |
| `comment`      | A watched reviewer commented.                        | `comment_kind` (`issue` or `review`), `reviewer`, `comment_id`, `url`, `head_sha`, `cursor`, `head_moved` |
| `head_changed` | The head commit changed: someone pushed.             | `previous_head_sha`, `head_sha`                                                 |
| `pr_closed`    | The pull request was closed without merging.         | `head_sha`                                                                      |
| `pr_merged`    | The pull request was merged.                         | `head_sha`                                                                      |
| `timeout`      | Nothing happened within the wait.                    | `waited_seconds`, `head_sha`                                                    |

`head_moved` is present and `true` on a `review` or `comment` result when the pull request head is
not the commit you passed with `--since-head-sha` (or, without it, the head at the start). The
feedback may then be about commits your checkout does not have; [synchronize](#the-review-loop)
before processing it.

`cursor` is the GitHub time of the item the result reports. `--since <cursor>` counts only items
created strictly after it, so the reported item and everything before it are skipped. An item created
in the same second as the cursor is skipped too; the review loop reads the whole pull request after
every event, so it is still read.

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

1. Run the poller with `--since-head-sha "$(git rev-parse HEAD)"`. After an event that needed no
   push, add `--since <cursor>` from that result.
2. Branch on `kind`:
   - `review` or `comment`: first make sure you are working on the commit the feedback is about.
     If `head_moved` is `true`, or `head_sha` differs from `git rev-parse HEAD`, run `git fetch` and
     bring the branch up to date with `git pull --ff-only` (or rebase your unpushed commits onto it)
     before processing; if that cannot be done safely, stop and tell the contributor. Then run the
     [`process-pr-review`](../process-pr-review/SKILL.md) workflow. Fetch every thread with
     [`fetch-review-threads`](../fetch-review-threads/SKILL.md) and read the new review body (a
     finding in a review body has no thread).
     - If you fixed something, commit, push, reply, and re-request the review (see below). Then go
       to step 1 with the commit you pushed.
     - If you pushed nothing, because the review was clean or the comment was informational, do not
       re-request the review. Go to step 1 with your `HEAD` and the `cursor` of this result, so this
       event is not reported again and nothing after it is missed.
   - `head_changed`: someone else pushed. Pull the branch, then go to step 1 with the new `HEAD`.
   - `timeout`: check the exit condition. If it is not met, run the poller again with the same flags
     as the run that timed out, since nothing was processed in between. After a few timeouts in a
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

After you push fixes for a reviewer, ask the same reviewer again. Do it only when new fixes need a
new look; a clean review needs no re-request. The poller never does this. The same REST endpoint
works for a maintainer and for Copilot; the Copilot reviewer is a bot, so its login ends in `[bot]`:

```sh
gh api --method POST repos/torrust/torrust-tracker/pulls/<PR>/requested_reviewers   -f 'reviewers[]=da2ce7' > .tmp/re-request.stdout 2> .tmp/re-request.stderr

gh api --method POST repos/torrust/torrust-tracker/pulls/<PR>/requested_reviewers   -f 'reviewers[]=copilot-pull-request-reviewer[bot]' > .tmp/re-request.stdout 2> .tmp/re-request.stderr
```

The Copilot call was verified on PR #2683 (2026-10-28): it exited `0`, GitHub recorded a
`review_requested` timeline event for `Copilot` one second later, and Copilot submitted a review
about 40 minutes after the request. Three things follow:

- The `requested_reviewers` list of the pull request never shows a bot, so do not use it to check
  that the request worked. Read the timeline (`gh api repos/torrust/torrust-tracker/issues/<PR>/timeline`,
  event `review_requested`) or the GraphQL `reviewRequests` of the pull request, which lists the bot
  while the request is pending.
- Copilot can take 40 minutes or more, longer than the poller's 15-minute default. Run the poller
  again with the same flags after a `timeout`; a second request while the first is pending adds
  nothing.
- Pushing does not request a review by itself: the earlier push on that pull request produced none.

## Skill Links

- `contrib/dev-tools/github/github-review-threads/` is the thread source the exit condition reads.
- `.github/skills/dev/pr-reviews/wait-for-pr-review/tool/src/main.rs` carries the option defaults and
  the exit codes this skill documents. Update both together.
