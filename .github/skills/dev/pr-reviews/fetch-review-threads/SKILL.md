---
name: fetch-review-threads
description: Fetch every GitHub pull request review thread, including resolved and outdated ones, for the torrust-tracker project. Use when asked to find open PR review threads, list review comments, collect thread IDs before resolving suggestions, audit earlier resolutions, or inspect Copilot review feedback. Triggers on "fetch review threads", "list unresolved PR comments", "get review thread IDs", or "find open review suggestions".
metadata:
  author: torrust
  version: "1.1"
  semantic-links:
    related-artifacts:
      - contrib/dev-tools/github/github-review-threads/Cargo.toml
---

# Fetching PR Review Threads

This is a component skill within the **process-pr-review** workflow.
Use this skill before processing review feedback. Its purpose is to collect every review thread,
including resolved and outdated ones, with enough evidence to detect re-raises and audit earlier
resolutions, and to select the unresolved threads that still need a reply or resolution.

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

Collect every thread, resolved or not. For each thread, capture:

- thread ID
- file path and `line`
- `isResolved` and `isOutdated`
- `resolvedBy` login
- each comment's author, body, and URL

`resolvedBy`, `path`, and `line` are self-audit evidence: they show who resolved an earlier thread
and where, so a new comment can be compared with what was already resolved. A null `resolvedBy`
or `line` means GitHub did not return one (for example, a deleted account or a rewritten line);
record it as unknown and never infer a value.

Filter to unresolved threads only when selecting threads for a reply or resolution action.

## Active PR Tool Workflow

1. Read the active PR.
2. Inspect the whole `reviewThreads` array, including resolved and outdated threads.
3. For reply or resolution actions only, select threads where `isResolved == false`.
4. Group them by file if you plan to address them in code.

## GitHub CLI GraphQL Fallback

Use GitHub CLI if you need to retrieve threads directly from the terminal.

## Review-Thread Tool

Run `cargo run --package github-review-threads --` from the repository root. Its `fetch`
subcommand writes the raw GraphQL response file consumed unchanged by the resolution workflow.
Its `list` and `show` subcommands emit one JSON object whose `threads` array holds every fetched
thread with `isResolved`, `isOutdated`, `resolvedBy`, `path`, and `line`; `show` adds the comments.
Pass `--unresolved-only` to either subcommand to select the threads that still need a reply or
resolution. `reply-status` always checks unresolved threads only, because it guards resolution.
Use `jq` to format or filter their result data.

The binary follows the CLI output contract: it refuses to run when stdout is a terminal (exit
code `2`, `tty_refusal` record on stderr), so every command below pipes or redirects stdout.
Usage errors and `--help` are JSON `usage_error` records on stderr with exit code `2`; the
subcommands and options are documented here instead.

Recommended usage:

```bash
# 1. Fetch all threads once
cargo run --package github-review-threads -- fetch \
  --pr-number 1707 \
  --output-file /tmp/pr_threads_1707.json | jq .

# 2. Read every thread, including resolved and outdated ones, with its comments
cargo run --package github-review-threads -- show \
  --threads-file /tmp/pr_threads_1707.json | jq '.threads[]'

# 3. Get compact IDs, paths, and resolution evidence for tracker population
cargo run --package github-review-threads -- list \
  --threads-file /tmp/pr_threads_1707.json | jq -c '.threads[]'

# 4. Select only the threads that still need a reply or resolution
cargo run --package github-review-threads -- list \
  --threads-file /tmp/pr_threads_1707.json --unresolved-only | jq -c '.threads[]'
```

Without the tool, request the same fields the tool's query requests:

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
            path
            line
            resolvedBy {
              login
            }
            isCollapsed
            comments(first: 20) {
              nodes {
                url
                body
                createdAt
                author {
                  login
                }
              }
            }
          }
        }
      }
    }
  }'
```

Keep every thread as evidence and filter to unresolved threads only for reply or resolution
actions. Prefer the Rust tool above for repository workflow automation.

## Practical Guidance

- Do not guess thread IDs.
- Do not resolve a thread immediately after fetching it. First confirm the fix exists.
- If a thread is outdated but unresolved, still read it before deciding what to do.
- Before treating a new comment as original, compare it with resolved threads on the same `path`
  and `line`; a match is a candidate re-raise for the audit.
- If there are more than 100 threads, paginate instead of assuming the first page is complete.

## Completion Checklist

- [ ] All thread IDs, including resolved and outdated threads, were collected from the current PR state
- [ ] Each thread has enough context for triage, including `resolvedBy`, `path`, and `line` evidence
- [ ] Only unresolved threads were selected for reply or resolution actions
- [ ] The result is ready to hand off to a fix or resolution workflow
