---
name: wait-for-pr-review
description: Wait for reviews from any reviewer with a deterministic Rust poller that spends no tokens, then run the review loop. Use when waiting for a review of an opened or updated PR.
metadata:
  author: torrust
  version: "1.5"
  semantic-links:
    related-artifacts:
      - .claude/skills/wait-for-pr-review/tool/Cargo.toml
      - .claude/skills/watch-pull-request-checks/SKILL.md
      - .claude/skills/process-pr-review/SKILL.md
      - .claude/skills/fetch-review-threads/SKILL.md
      - .claude/skills/merge-pull-request/SKILL.md
      - docs/adrs/20260519000000_define_global_cli_output_contract.md
      - docs/issues/closed/2581-2003-pr-review-wait-poller/ISSUE.md
      - docs/adrs/20261124182709_review_pull_requests_without_a_mandatory_reviewer.md
---

# Waiting for Pull-Request Reviews

After you open or update a pull request, reviewers answer on their own schedule. Run the poller
instead of asking the contributor to tell you when to look, and instead of polling through an MCP
server, which spends inference tokens on every check. The poller is a Rust tool that calls no model,
holds no token, and returns only when something happens on the pull request.

This skill is the waiting half of the review loop. [`process-pr-review`](../process-pr-review/SKILL.md)
is the processing half.

## Prerequisites

- A GitHub CLI command, authenticated as the account you act as. By default that is `gh`, and the poller
  refuses to run when `gh auth status` fails. To run as another account, name its command with
  `--gh-command`; never put a stand-in named `gh` on `PATH`, because one that calls `gh` again calls
  itself without end.
- The pull request number and the commit you last pushed.

## Run the Poller

Stdout carries the result and the poller refuses to run when stdout is a terminal. Redirect each
channel to a file, as the
[CLI output contract](../../../docs/adrs/20260519000000_define_global_cli_output_contract.md)
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
| `--reviewer <login>`        | every account except the pull request's author      | Whose reviews and comments count. Repeat the flag to watch only those logins. Without it, any reviewer, person or bot, wakes the poller, and the author's own replies do not. |
| `--timeout-seconds <N>`     | 900 (15 minutes)                                     | Longest wait before a `timeout` result. At most 86400 (one day); a larger value is a usage error, exit `2`.    |
| `--poll-interval-seconds N` | 120 (2 minutes)                                      | Pause between two looks at the pull request.                                                                    |
| `--since-head-sha <sha>`    | the head at the start of the wait                    | The commit you have checked out. A pull request head that differs from it is reported as `head_changed`. Unless `--since` is given, reviews and comments created after this commit was made count as new. |
| `--gh-command <command>`    | `gh`                                                 | The GitHub CLI command run for every call: one program name or path, never split on spaces and never run through a shell. It must accept the `gh` command line (`api ...`). `gh auth status` runs only when the value is `gh`. An empty value is a usage error, exit `2`. |
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

`head_sha` is fetched after the reviews and comments in the same result, so it never predates a push
that the feedback was already about when it was fetched. A push made after the head was fetched is
not in `head_sha`; the next poll reports it as `head_changed`, unless that poll has new feedback to
report first, which comes first (see the order below) with `head_moved` true. A push between the feedback
fetch and the head fetch makes `head_moved` true for feedback about the older head: a false alarm,
never a miss.

`cursor` is the GitHub time of the item the result reports. `--since <cursor>` counts only items
created strictly after it, so the reported item and everything before it are skipped. An item created
in the same second as the cursor is skipped too; the review loop reads the whole pull request after
every event, so it is still read.

When several things happened at once, the poller reports a closed or merged pull request first, then
the earliest new review or comment by GitHub time (a review before a conversation comment before an
inline comment when made in the same second), then a head change. Reporting the earliest keeps the
`cursor` honest: resuming from it never skips an item that was still unreported. The poller reports
one event; the review loop reads the whole pull request afterward, so nothing is lost.

Without `--reviewer`, no reviewer has to be named, so a new reviewer or review bot needs no
configuration. The cost is more wake-ups: a review bot that answers your thread replies submits an
empty `COMMENTED` review each time, and any comment, for example a maintainer's `ACK`, wakes the
poller. Such an event needs no push: resume with `--since <cursor>`. When you pass `--reviewer`,
list every login the reviewer uses: one reviewer's reviews and inline comments can come from
different logins (Copilot reviews as `copilot-pull-request-reviewer[bot]` and comments inline as
`Copilot`).

Stderr is NDJSON, one record per line, each with a `kind`: `wait_started` at the start (with the
`gh_command` that will run, `watch` set to `all_except_author` or `listed`, and the listed
`reviewers`), then `gh_identity`, and on failure one diagnostic. `gh_identity` carries
`gh_command` and `gh_login`, the account that command is authenticated as (from `api user`), or `null`
when the lookup fails or ends at its bound, `min(10 s, timeout / 4)`. Check `gh_login` against the account
you act as before you act on the result. The lookup runs before the baseline snapshot, so without
`--since-head-sha` a push that lands while it stalls is absorbed into the baseline (a review or comment is
absorbed without `--since-head-sha` or `--since`); always pass `--since-head-sha`.

| Exit code | Meaning                                                                                                                                         |
| --------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| `0`       | A result is on stdout (or help was printed).                                                                                                    |
| `1`       | Failure; stdout is empty. Diagnostic kinds: `gh_unavailable`, `gh_not_authenticated`, `gh_api_error`, `malformed_response`, `commit_date_error`, `deadline_exceeded`, `interrupted`, `output_error`. |
| `2`       | Usage error or TTY refusal (`usage_error`, `tty_refusal`).                                                                                      |

On exit `1`, read the `message` of the last stderr record. A signal (`SIGINT` or `SIGTERM`) stops the
poller and any `gh` call in flight and reports `interrupted`.

The `--timeout-seconds` budget is a hard bound. No request starts once it is spent, and a `gh` call
still running at that moment is killed. The bound also covers reading the output of a `gh` call that has
already exited: if a process `gh` started still holds its output pipes open when the budget ends or a signal
arrives, the call fails and no partial output is used. A signal always ends the run with an `interrupted`
failure (exit `1`). If the budget ends after the first look at the pull request, the result is
`timeout` (exit `0`). If it ends before, nothing was observed, so it is a
`deadline_exceeded` failure (exit `1`), not a `timeout` that would suggest a quiet pull request. The
last look is taken one poll interval or less before the budget ends, so rerun with `--since-head-sha`
and nothing in that gap is missed.

## The Review Loop

Who runs a round: in Claude Code, the session that owns the pull request can delegate the processing
of one `review` or `comment` event (step 2, from `process-pr-review` to the push, the replies and the
resolved threads) to the [`pr-review-handler`](../../agents/pr-review-handler.md) agent, which runs
on its pinned model and returns a short report. The session keeps the rest: the poller, bringing the
branch to the commit the event is about before it delegates (the start of step 2), the checks of the
new head, the round count, the exit condition and the watching after it. A
session that does not delegate, or runs in another tool, runs step 2 itself.

1. When a session starts or takes over the pull request, first list every review already on it,
   from every author, because the poller reports only reviews created after the commit you pass:

   ```sh
   gh api repos/torrust/torrust-tracker/pulls/<PR>/reviews --paginate \
     --jq '.[] | "\(.id) \(.user.login) \(.state) \(.commit_id[0:9]) \(.submitted_at)"'
   ```

   Process each review, other than your own replies, that the audit record does not cover yet, as
   in step 2. Then run the poller with `--since-head-sha "$(git rev-parse HEAD)"`. After an event
   that needed no push, add `--since <cursor>` from that result.
2. Branch on `kind`:
   - `review` or `comment`: first make sure you are working on the commit the feedback is about.
     If `head_moved` is `true`, or `head_sha` differs from `git rev-parse HEAD`, run `git fetch` and
     bring the branch up to date with `git pull --ff-only` (or rebase your unpushed commits onto it)
     before processing; if that cannot be done safely, stop and tell the contributor. Then open
     [`process-pr-review`](../process-pr-review/SKILL.md) and run its workflow, including its rule
     for when to create the audit record `PR-REVIEW.md` (the first event that needs a reply or a
     fix, before replying, from the template). Fetch every thread with
     [`fetch-review-threads`](../fetch-review-threads/SKILL.md) and read the new review body (a
     finding in a review body has no thread).
     - If you fixed something, commit, push, and reply. Then
       [watch the checks](#watching-the-checks) of the new head and fix any failure, then go to
       step 1 with the commit you pushed. Do not request a new review yourself (see
       [A Review a Maintainer Requested](#a-review-a-maintainer-requested)).
     - If you pushed nothing, because the review was clean or the comment was informational, go to
       step 1 with your `HEAD` and the `cursor` of this result, so this event is not reported again
       and nothing after it is missed.
   - `head_changed`: someone else pushed. Pull the branch, then go to step 1 with the new `HEAD`.
   - `timeout`: check the exit condition. If it holds, go to step 5. If it is not met, run the
     poller again with the same flags as the run that timed out, since nothing was processed in
     between. After a few timeouts in a row, tell the contributor which condition is still unmet.
   - `pr_closed` or `pr_merged`: stop watching and tell the contributor which one happened.
3. Count the round (see [Round Limit](#round-limit)) after each push, and stop and ask the
   contributor when the limit is exceeded.
4. Check the exit condition after each return. The loop is done when all of these hold:
   - Every review received so far is processed with `process-pr-review`: list the reviews as in
     step 1, and each one either has its findings in the audit record or has none. No review from a named
     reviewer is required and none is waited for: a pull request with no review at all can be
     ready. A review that arrives later reopens the loop (step 6). Reviewers often submit only
     `COMMENTED` reviews; do not wait for an approval state.
   - Every workflow check of the head commit is `pass` or `skipping`, as
     [`watch-pull-request-checks`](../watch-pull-request-checks/SKILL.md) reports; a cancelled
     check needs attention like a failed one. A check run that a review service posts from its own GitHub
     App, for example `CodeRabbit` or `Greptile Review`, does not count: see
     [Review-Service Check Runs](../watch-pull-request-checks/SKILL.md#review-service-check-runs).
   - The pull request has no unresolved actionable review thread anywhere, including threads on
     earlier commits. Count them over the whole pull request with `fetch-review-threads`.
   - The other maintainer has commented `ACK <full head commit hash>` on the head commit, or the
     maintainer has recorded a waiver, as the Merge Readiness section of
     [`merge-pull-request`](../merge-pull-request/SKILL.md) describes. Its subsection
     [An ACK Names One Commit](../merge-pull-request/SKILL.md#an-ack-names-one-commit) says what
     happens to an `ACK` after a commit that changes only the audit record.
5. When the exit condition holds, print the maintainer merge command from `merge-pull-request`. Do
   not merge, approve, or post `ACK` yourself; an agent never waives a check.
6. Keep watching until the pull request is merged or closed; do not end the session when the exit
   condition first holds, and do not wait for the contributor to say it was merged. Run the poller
   again with `--since-head-sha "$(git rev-parse HEAD)"` and the `cursor` of the last result.
   - `pr_merged` or `pr_closed`: stop and report it.
   - `timeout`: run it again. Waiting costs no tokens, so there is no limit on repeats, but use the
     longest `--timeout-seconds` your shell tool allows to keep the number of calls small.
   - `review`, `comment` or `head_changed`: the pull request changed after it looked ready. Go back
     to step 2, because the exit condition no longer holds until it is checked again.

The poller returns events and does not decide the exit condition. A review-body finding with no
thread is not counted by the thread check, so read each new review body through the audit workflow.

## Watching the Checks

CI is part of the loop, not a separate task. Checks often finish before a reviewer answers, so run
them first:

1. After the first push, and after every later push, wait for the workflow checks with the loop in
   [Review-Service Check Runs](../watch-pull-request-checks/SKILL.md#review-service-check-runs),
   which leaves out review services' own check runs; `gh pr checks <PR> --watch --fail-fast`, as
   [`watch-pull-request-checks`](../watch-pull-request-checks/SKILL.md) describes, also waits for
   them. Both spend no tokens while they wait.
2. If no check starts and the runs say `action_required`, a maintainer has to approve them (see
   "When Checks Never Start" in that skill). Ask for it and wait; this is not a failure and does not
   count toward the check limit.
3. If a check failed, first see whether it is a workflow check: a review service's own check run
   (no workflow) is not a failure, even though `--watch --fail-fast` stops on it (see
   [Review-Service Check Runs](../watch-pull-request-checks/SKILL.md#review-service-check-runs)).
   For a failed workflow check, follow "When a Check Fails" in that skill: read the log, fix the
   change (or re-run the jobs when the cause is outside it), push, and watch again.
4. Run the poller while the checks run, so a review that arrives first is not missed. Do not
   request a review (see [A Review a Maintainer Requested](#a-review-a-maintainer-requested)).

A review that arrives while checks are failing is still processed, but its fixes and the check fixes
go in the same push when possible, so the checks run once.

## Round Limit

A round is one push of fixes made in response to a review event, whoever the reviewer was. Stop the loop and ask the
contributor before starting round 6, that is, when more than 5 rounds of fixes have been pushed.
Rounds that ended with a clean review and no push do not count, and neither do timeouts.

Failed check runs have their own limit of 5, because a check failure is not a review event. Count
the distinct head commits of the pull request whose checks failed. Each one is a push that did not
yet make the checks pass, so the count measures failed fixing attempts, not successful fixes. A
re-run of the same commit does not add to it. Ask the contributor when more than 5 head commits have
failed checks. Pass a large `--limit`, because `gh run list` returns only the 20 latest runs by
default and a lower count would let the loop run past the limit:

```sh
gh run list --repo torrust/torrust-tracker --branch <branch> --event pull_request --limit 1000 \
  --json headSha,conclusion \
  --jq '[.[] | select(.conclusion == "failure") | .headSha] | unique | length'
```

A loop that keeps finding new things after 5 rounds usually means something is wrong with the agent
or its harness (for example a fix that regresses another finding, or a reviewer reacting to the
agent's own replies). A person should look before more tokens are spent.

Keep the count where any agent can recompute it, not in the session memory: it is the number of
distinct reviews in the audit record
(`docs/pr-reviews/pr-<PR_NUMBER>-review/PR-REVIEW.md`) that have at least one `FIXED` finding. A
restarted session, or a different agent or harness taking over the pull request, therefore resumes
with the right count instead of starting at zero. When asking, say how many rounds ran, which
findings keep coming back, and what you suggest. The contributor can raise the limit for that pull
request; record the new limit in the pull request conversation.

## A Review a Maintainer Requested

No review from a named reviewer is required
([ADR](../../../docs/adrs/20261124182709_review_pull_requests_without_a_mandatory_reviewer.md)).
Reviewers come in two kinds, and their reviews are processed the same way, with
[`process-pr-review`](../process-pr-review/SKILL.md):

- **Reviewers that review by themselves**, for example a review service installed on the repository
  that reviews each push. Nothing is requested: process each review it submits, including the new
  findings it posts after your fixes.
- **Reviewers that review only when asked**: a maintainer, or Copilot. Only a maintainer requests
  their review, and decides whether to request it again after a round of fixes.

An AI agent never triggers a review: no request through `requested_reviewers`, no review-service
command in a comment (such as `@coderabbitai review`), and no `@`-mention that asks for one. An
account without triage access cannot request reviewers anyway. When a new look after your fixes
would help, tell the maintainer which reviewer to ask and whether the findings were substantive or
minor; the maintainer decides. A person contributing may still ask a reviewer to look.

What to know about a Copilot review that a maintainer requests (verified on PR #2683, 2026-10-28:
the request was recorded as a `review_requested` timeline event one second later, and Copilot
reviewed about 40 minutes after it):

- Copilot reviews the head as of the request, not as of the moment it answers. On PR #2683 it was
  requested at 14:13:25 UTC with head `d000f8881`, two commits were pushed at 14:18, and its review at
  14:58 was of `d000f8881`. When you push after a request, tell the maintainer that the new commits
  are not covered.
- It can take 40 minutes or more, longer than the poller's 15-minute default. Pass
  `--timeout-seconds 2700` (45 minutes) if your shell tool allows a call that long, or run the poller
  again with the same flags after a `timeout`.
- The `requested_reviewers` list of a pull request never shows a bot. To see a pending request,
  read the timeline (`gh api repos/torrust/torrust-tracker/issues/<PR>/timeline`, event
  `review_requested`) or the GraphQL `reviewRequests` of the pull request.
- Pushing does not request a review by itself.

For reference, the maintainer's request uses the same REST endpoint for a person and for Copilot,
whose reviewer login ends in `[bot]`:

```sh
gh api --method POST repos/torrust/torrust-tracker/pulls/<PR>/requested_reviewers \
  -f 'reviewers[]=copilot-pull-request-reviewer[bot]' > .tmp/request.stdout 2> .tmp/request.stderr
```

## Skill Links

- `contrib/dev-tools/github/github-review-threads/` is the thread source the exit condition reads.
- `.claude/skills/wait-for-pr-review/tool/src/main.rs` carries the option defaults and
  the exit codes this skill documents. Update both together.
