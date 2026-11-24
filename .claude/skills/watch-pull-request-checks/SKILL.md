---
name: watch-pull-request-checks
description: Wait for a pull request's CI checks to finish with gh pr checks, then read which failed. Use when you need to know whether CI passed after pushing to a PR.
metadata:
  author: torrust
  version: "1.1"
  semantic-links:
    related-artifacts:
      - .claude/skills/wait-for-pr-review/SKILL.md
      - .claude/skills/open-pull-request/SKILL.md
      - docs/adrs/20261124182709_review_pull_requests_without_a_mandatory_reviewer.md
---

# Watching Pull-Request Checks

`gh pr checks` already does the waiting, so this repository ships no tool for it. Use it after you
push, and use [`wait-for-pr-review`](../wait-for-pr-review/SKILL.md) for reviewer
events, which `gh pr checks` does not report.

## Prerequisites

- GitHub CLI (`gh`), authenticated as the identity that should read the checks. Pass the pull request
  number and `--repo torrust/torrust-tracker`, because a fork-based pull request lives upstream.

## Wait for the Checks

```sh
gh pr checks <PR> --repo torrust/torrust-tracker --watch --fail-fast --interval 60 \
  > .tmp/pr-checks.txt 2>&1
echo "exit=$?"
```

`--watch` repaints the whole table every interval, so redirect it to a file and read the last table,
not the stream. `--fail-fast` stops at the first failing check instead of waiting for the slow ones.
CI jobs take minutes, so an interval under 60 seconds only adds API calls.

| Exit code | Meaning                                                      |
| --------- | ------------------------------------------------------------ |
| `0`       | Every check finished and none failed (skipped checks do not count as failures). |
| `1`       | At least one check failed, or the command failed.            |
| `8`       | Checks are still pending. Without `--watch` this is the normal answer to a single look. |

Run the watcher in the background when your shell tool caps a call, and read the file when it exits.
Right after a push the checks may not exist yet; if the command reports none, wait a few seconds
and run it again.

## Read the Result Without Watching

A single look is enough when you only need the state. The `bucket` field is one of `pass`, `fail`,
`pending`, `skipping` or `cancel`:

```sh
gh pr checks <PR> --repo torrust/torrust-tracker --json name,bucket \
  --jq 'group_by(.bucket) | map({(.[0].bucket): length}) | add'
gh pr checks <PR> --repo torrust/torrust-tracker --json name,bucket,link \
  --jq '.[] | select(.bucket == "fail") | "\(.name) \(.link)"'
```

## Review-Service Check Runs

A review service installed on the repository can post its own check run, for example `CodeRabbit`
or `Greptile Review`. Such a check run comes from the service's GitHub App, not from a workflow of
this repository, and it does not count for readiness: it neither blocks a pull request nor is waited
for ([ADR](../../../docs/adrs/20261124182709_review_pull_requests_without_a_mandatory_reviewer.md)).
Its review, if it posts one, goes through the review loop like any other.

`gh pr checks` reports an empty `workflow` for these check runs, so they can be left out without
naming any service. `--watch` waits for every check run, theirs included, so it can keep waiting
for a slow review service after CI has finished, and its exit code counts a failed review-service
check as a failure. To wait for the workflow checks alone, poll them instead. The loop spends no
tokens. It ends when at least one workflow check exists and none is pending, or when the workflow
runs of the head commit wait for a maintainer's approval (see
[When Checks Never Start](#when-checks-never-start)), or when a `gh` call fails:

```sh
head=$(git rev-parse HEAD)   # the commit you pushed
while :; do
  if ! ready=$(gh pr checks <PR> --repo torrust/torrust-tracker --json bucket,workflow \
    --jq '[.[] | select(.workflow != "")] | length > 0 and all(.bucket != "pending")' \
    2> .tmp/pr-checks-error.txt); then
    grep -q "^no checks reported" .tmp/pr-checks-error.txt \
      || { cat .tmp/pr-checks-error.txt; echo "gh pr checks failed" >&2; break; }
    ready=false   # no check exists yet: see whether the runs wait for approval
  fi
  [ "$ready" = true ] && { echo "workflow checks finished"; break; }
  waiting=$(gh run list --repo torrust/torrust-tracker --commit "$head" --json conclusion \
    --jq 'any(.[]; .conclusion == "action_required")') \
    || { echo "gh run list failed; see its error above" >&2; break; }
  [ "$waiting" = true ] && { echo "workflow runs of $head wait for a maintainer's approval"; break; }
  sleep 60
done > .tmp/pr-checks-wait.txt 2>&1
```

Read the last line of the file when the loop exits. With `--json`, `gh pr checks` exits `0` whether
the checks pass, fail or are pending. It exits non-zero with `no checks reported on the '<branch>'
branch` when the head has no check at all, which the loop treats as "not yet"; any other non-zero
exit is a real error, such as an expired login, denied access or a network failure. The loop then stops and keeps the error instead of retrying
without end: fix the cause before waiting again. When the runs wait for approval, ask for it as
"When Checks Never Start" says and run the loop again once they start. `--commit` needs the full
commit hash.

Then decide with the workflow checks alone, and list the review-service check runs that were left
out, for the report:

```sh
gh pr checks <PR> --repo torrust/torrust-tracker --json name,bucket,workflow \
  --jq '[.[] | select(.workflow != "")] | group_by(.bucket) | map({(.[0].bucket): length}) | add'
gh pr checks <PR> --repo torrust/torrust-tracker --json name,bucket,workflow \
  --jq '.[] | select(.workflow == "") | "\(.name) \(.bucket)"'
```

The workflow checks pass only when every one of them is `pass` or `skipping`. A `cancel` needs
attention, like a `fail`: the job did not check the change. A `pending` needs more waiting.

## When Checks Never Start

A pull request from a fork can stop at `conclusion: action_required` until a maintainer approves its
workflow runs. This depends on the author's account, not on the change: a contributor without write
access to the repository is affected, and a maintainer pushing from a branch in the repository is not.
`gh pr checks` then reports no checks, or none that finish. It is not a failure, and the author
cannot approve the run. Confirm it with:

```sh
gh run list --repo torrust/torrust-tracker --branch <branch> --limit 5 \
  --json headSha,conclusion --jq '.[] | select(.conclusion == "action_required") | .headSha'
```

Do not push an empty commit or re-run jobs to get around it. Ask a maintainer to use "Approve and run
workflows" on the pull request, say which head commit is waiting, and watch the checks again once
they approve. Every new push can need a new approval, so batch fixes into one push where possible.

## When a Check Fails

1. Open the failing job's `link` and read its log before changing anything.
2. If the failure is in the change, fix it, commit, push, and watch again.
3. If the log shows a cause outside the change, such as a cache or runner outage, re-run the failed
   jobs instead of committing. See "CI Failures Caused Outside the Change" in
   [`update-dependencies`](../update-dependencies/SKILL.md).
4. A check that failed on a new toolchain release can pass locally on the default toolchain. Run the
   same command on the toolchain the failing job used (for example `cargo +nightly clippy`) and
   cover the whole workspace, not one package.
5. Do not loop forever: the review loop's
   [Round Limit](../wait-for-pr-review/SKILL.md#round-limit) stops and asks the
   contributor when more than 5 head commits of the pull request have failed checks.
