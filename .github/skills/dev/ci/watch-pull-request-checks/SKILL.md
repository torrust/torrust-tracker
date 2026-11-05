---
name: watch-pull-request-checks
description: Wait for the CI checks of a pull request to finish with the GitHub CLI, then read which ones failed. Use after pushing to a pull request when you need to know whether CI passed, instead of asking the contributor or polling by hand. Triggers on "watch CI", "wait for checks", "wait for CI", "did CI pass", "gh pr checks", or "watch pull request checks".
metadata:
  author: torrust
  version: "1.0"
  semantic-links:
    related-artifacts:
      - .github/skills/dev/pr-reviews/wait-for-pr-review/SKILL.md
      - .github/skills/dev/git-workflow/open-pull-request/SKILL.md
---

# Watching Pull-Request Checks

`gh pr checks` already does the waiting, so this repository ships no tool for it. Use it after you
push, and use [`wait-for-pr-review`](../../pr-reviews/wait-for-pr-review/SKILL.md) for reviewer
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

## When a Check Fails

1. Open the failing job's `link` and read its log before changing anything.
2. If the failure is in the change, fix it, commit, push, and watch again.
3. If the log shows a cause outside the change, such as a cache or runner outage, re-run the failed
   jobs instead of committing. See "CI Failures Caused Outside the Change" in
   [`update-dependencies`](../../maintenance/update-dependencies/SKILL.md).
4. A check that failed on a new toolchain release can pass locally on the default toolchain. Run the
   same command on the toolchain the failing job used (for example `cargo +nightly clippy`) and
   cover the whole workspace, not one package.
5. Do not loop forever: the review loop's
   [Round Limit](../../pr-reviews/wait-for-pr-review/SKILL.md#round-limit) stops and asks the
   contributor when more than 5 head commits of the pull request have failed checks.
