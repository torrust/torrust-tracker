---
name: fetch-review-threads
description: Fetch unresolved GitHub pull request review thread IDs for the torrust-tracker project. Use when asked to find open PR review threads, list unresolved review comments, collect thread IDs before resolving suggestions, or inspect Copilot review feedback. Triggers on "fetch review threads", "list unresolved PR comments", "get review thread IDs", or "find open review suggestions".
metadata:
  author: torrust
  version: "1.0"
---

# Fetching PR Review Threads

Use this skill before resolving review feedback. Its purpose is to collect the unresolved
review thread IDs and enough context to decide whether each thread should stay open or be closed.

## Preferred Sources

Use one of these approaches:

1. Active pull request tools when they are available in the environment.
2. GitHub CLI GraphQL when you need a terminal-based fallback.

Prefer the active PR tools first because they provide thread metadata together with file paths,
resolution state, and comments.

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

Then filter for unresolved threads.

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
