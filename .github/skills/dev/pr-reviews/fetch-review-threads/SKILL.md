---
name: fetch-review-threads
description: Fetch unresolved GitHub pull request review thread IDs for the torrust-tracker project. Use when asked to find open PR review threads, list unresolved review comments, collect thread IDs before resolving suggestions, or inspect Copilot review feedback. Triggers on "fetch review threads", "list unresolved PR comments", "get review thread IDs", or "find open review suggestions".
metadata:
  author: torrust
  version: "1.0"
  semantic-links:
    related-artifacts:
      - contrib/dev-tools/github/github-review-threads/Cargo.toml
---

# Fetching PR Review Threads

This is a component skill within the **process-pr-review** workflow.
Use this skill before resolving review feedback. Its purpose is to collect the unresolved
review thread IDs and enough context to decide whether each thread should stay open or be closed.

**Part of larger workflow**: See **process-pr-review** for the full end-to-end process.

## Preferred Sources

Use one of these approaches:

1. GitHub CLI GraphQL — reliable for all PRs, including fork-based PRs (see note below).
2. Active pull request tools when they are available in the environment and the PR is not fork-based.

> **Fork-based PR limitation**: The VS Code `currentActivePullRequest` and `pullRequestInViewport`
> tools do **not** detect PRs opened from a fork (e.g. `contributor:branch` → `upstream/repo`).
> In this repository all contributor PRs are fork-based, so the GitHub CLI GraphQL approach
> is the reliable primary path. Use the VS Code tools only when you know the branch lives in
> the same repository as the target.

## What to Collect

For each unresolved thread, capture:

- thread ID
- file path
- `isResolved`
- `canResolve`
- comment author
- comment body

Only unresolved threads should be considered for follow-up work.

## Active PR Tool Workflow

1. Read the active PR.
2. Inspect the `reviewThreads` array.
3. Filter to threads where `isResolved == false`.
4. Group them by file if you plan to address them in code.

## GitHub CLI GraphQL Fallback

Use GitHub CLI if you need to retrieve threads directly from the terminal.

## Review-Thread Tool

Run `cargo run --package github-review-threads --` from the repository root. Its `fetch`
subcommand writes the raw GraphQL response file consumed unchanged by the resolution workflow.
Its `list` and `show` subcommands emit JSON arrays of unresolved threads; use `jq` to format or
filter their result data.

Recommended usage:

```bash
# 1. Fetch all threads once
cargo run --package github-review-threads -- fetch \
  --pr-number 1707 \
  --output-file /tmp/pr_threads_1707.json

# 2. Read full suggestion bodies
cargo run --package github-review-threads -- show \
  --threads-file /tmp/pr_threads_1707.json

# 3. Get compact IDs/paths for tracker population
cargo run --package github-review-threads -- list \
  --threads-file /tmp/pr_threads_1707.json
```

```bash
gh api graphql \
  -F owner=torrust \
  -F repo=torrust-tracker \
  -F pullNumber=1707 \
  -f query='query($owner: String!, $repo: String!, $pullNumber: Int!) {
    repository(owner: $owner, name: $repo) {
      pullRequest(number: $pullNumber) {
        reviewThreads(first: 100) {
          nodes {
            id
            isResolved
            isOutdated
            comments(first: 20) {
              nodes {
                author {
                  login
                }
                body
                path
              }
            }
          }
        }
      }
    }
  }'
```

Then filter for unresolved threads. Prefer the Rust tool above for repository workflow automation.

## Practical Guidance

- Do not guess thread IDs.
- Do not resolve a thread immediately after fetching it. First confirm the fix exists.
- If a thread is outdated but unresolved, still read it before deciding what to do.
- If there are more than 100 threads, paginate instead of assuming the first page is complete.

## Completion Checklist

- [ ] Unresolved thread IDs were collected from the current PR state
- [ ] Each thread has enough context for triage
- [ ] Already resolved threads were excluded from action items
- [ ] The result is ready to hand off to a fix or resolution workflow
