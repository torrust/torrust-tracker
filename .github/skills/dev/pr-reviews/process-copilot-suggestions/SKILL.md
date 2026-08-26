---
name: process-copilot-suggestions
description: End-to-end workflow for processing and resolving all Copilot code review suggestions on a pull request in torrust-tracker. Use when asked to handle PR review feedback, process all copilot suggestions, audit and resolve review comments, or manage copilot-generated review threads. Triggers on "process copilot suggestions", "handle all PR feedback", "resolve copilot review", "audit PR suggestions", or "close all copilot comments".
metadata:
  author: torrust
  version: "1.0"
  semantic-links:
    related-artifacts:
      - docs/templates/COPILOT-SUGGESTIONS-TEMPLATE.md
      - .github/skills/dev/pr-reviews/fetch-review-threads/scripts/get-pr-review-threads.sh
      - .github/skills/dev/pr-reviews/fetch-review-threads/scripts/list-unresolved-threads.sh
      - .github/skills/dev/pr-reviews/fetch-review-threads/scripts/show-unresolved-thread-bodies.sh
      - .github/skills/dev/pr-reviews/fetch-review-threads/scripts/check-thread-reply-status.sh
      - .github/skills/dev/pr-reviews/resolve-review-threads/scripts/reply-to-thread.sh
      - .github/skills/dev/pr-reviews/resolve-review-threads/scripts/reply-and-resolve-thread.sh
      - .github/skills/dev/pr-reviews/resolve-review-threads/scripts/resolve-all-unresolved-threads.sh
---

# Processing Copilot PR Suggestions

This is the primary workflow for handling all Copilot code review suggestions on a pull request.
It combines decision-making, implementation, tracking, and resolution into a structured end-to-end process.

## Overview

Copilot generates suggestions that fall into two categories:

- **action** — Code or documentation changes needed; implement, validate, commit
- **no-action** — Already handled, false positive, or intentionally declined; explain reasoning and mark resolved

## Two Absolute Rules

**Rule 1 — Always reply before resolving.**
Every thread must have a comment explaining what was done (or why nothing was done) before it
is marked resolved. Resolving a thread without a reply makes the decision invisible to reviewers
and future contributors reading the PR.

**Rule 2 — Resolve promptly, one thread at a time.**
Copilot re-reviews the PR on every push and opens new suggestion threads. If old threads are
left unresolved, they become indistinguishable from the newly opened ones. Resolve each thread
immediately after posting the reply — do not accumulate a backlog of open threads.

## Prerequisites

- Target PR number
- Write access to branch (to apply fixes and push)
- Access to GitHub CLI (`gh`)
- Ability to run linters and tests locally

## Full Workflow

### 1. Setup Tracking File

Copy the template to create a tracker for this PR:

```bash
cp docs/templates/COPILOT-SUGGESTIONS-TEMPLATE.md \
  docs/copilot-pr-reviews/pr-<PR_NUMBER>-copilot-suggestions.md
```

Open the tracker file and fill in:

- `<PR_NUMBER>` and `<PR_URL>` at the top
- Placeholder columns in the Suggestions table

### 2. Fetch All Review Threads

Use the **fetch-review-threads** skill or the helper script:

```bash
bash ../fetch-review-threads/scripts/get-pr-review-threads.sh \
  --pr-number <PR_NUMBER> \
  --output-file /tmp/pr_threads_<PR_NUMBER>.json
```

This saves all review threads (resolved, unresolved, outdated) to `/tmp/pr_threads_<PR_NUMBER>.json`.

### 3. Populate the Tracker

Read the full suggestion bodies to understand each thread:

```bash
bash ../fetch-review-threads/scripts/show-unresolved-thread-bodies.sh \
  --threads-file /tmp/pr_threads_<PR_NUMBER>.json
```

Then extract the compact list for populating the tracker table:

```bash
bash ../fetch-review-threads/scripts/list-unresolved-threads.sh \
  --threads-file /tmp/pr_threads_<PR_NUMBER>.json
```

Add one row per thread to your tracker file with:

- Thread ID
- File path
- Comment URL
- Brief summary of the suggestion

### 4. Process Each Thread (Decide → Implement → Reply → Resolve)

Handle suggestions **one at a time**, completing each thread fully before moving to the next.
**Post a reply and resolve the thread before touching the next one.** This keeps already-addressed
threads visibly separated from new suggestions Copilot may open on the next push.

For each unresolved thread:

#### Step A — Decide

- **`action`** — The suggestion identifies a real fix needed. Apply it.
- **`no-action`** — Already handled, false positive, or intentionally declined. Document the reason.

**Key principle**: Do not resolve a thread just because a suggestion exists. Only resolve when
the concern is genuinely addressed or explicitly declined with documented reasoning.

#### Step B — Implement (action only)

1. Apply the minimal fix.
2. Validate:

   ```bash
   linter all                          # Full lint gate
   cargo test -p <affected-package>    # Targeted tests
   ```

3. Commit with GPG signature:

   ```bash
   git add <files>
   git commit -S -m "fix(review): <concise description>"
   ```

#### Step C — Reply and resolve

Use the `reply-and-resolve-thread.sh` script to post a reply **and** resolve in one operation:

```bash
bash ../resolve-review-threads/scripts/reply-and-resolve-thread.sh \
  --thread-id <THREAD_ID> \
  --body "<explanation>"
```

For an `action` reply, include:

- the commit that contains the fix,
- the files or behaviour changed, and
- the validation performed (when useful to establish correctness).

For a `no-action` reply, state the reason it was declined (for example, it was already
addressed, is outdated, or is a verified false positive).

The script outputs `{"reply_url": "...", "resolved": true}`. Copy the `reply_url` into the
tracker row.

#### Step D — Update tracker

- Set `Reply URL` to the reply URL from the script output.
- Set `Status` to `DONE`.
- Set `Thread State` to `RESOLVED`.

Repeat steps A–D for every thread before moving on.

### 5. Verify All Threads Are Resolved

After processing all threads, refresh and verify no unresolved threads remain:

```bash
bash ../fetch-review-threads/scripts/get-pr-review-threads.sh \
  --pr-number <PR_NUMBER> \
  --output-file /tmp/pr_threads_<PR_NUMBER>.json

bash ../fetch-review-threads/scripts/list-unresolved-threads.sh \
  --threads-file /tmp/pr_threads_<PR_NUMBER>.json
```

If any threads remain (Copilot may post new suggestions as you push commits), process them
using the same per-thread loop (Step 4).

#### Batch resolver — emergency cleanup only

If some threads need bulk-resolving, first confirm every thread already has a user reply:

```bash
bash ../fetch-review-threads/scripts/check-thread-reply-status.sh \
  --threads-file /tmp/pr_threads_<PR_NUMBER>.json
```

This script exits with code 1 if any thread lacks a reply. Only proceed with the batch resolver
once it exits 0:

```bash
bash ../resolve-review-threads/scripts/resolve-all-unresolved-threads.sh \
  --threads-file /tmp/pr_threads_<PR_NUMBER>.json
```

### 6. Final Documentation

Update the tracker file with completion notes:

- Add timestamps to the Processing Log.
- Confirm all rows have `Status = DONE` and `Thread State = RESOLVED`.

Commit the tracker as final documentation:

```bash
git add docs/copilot-pr-reviews/pr-<PR_NUMBER>-copilot-suggestions.md
git commit -S -m "docs(review): document PR #<PR_NUMBER> copilot suggestions audit"
```

## Decision Matrix

| Suggestion Type                           | Has Fix? | Tests Pass? | Decision  | Action                    |
| ----------------------------------------- | -------- | ----------- | --------- | ------------------------- |
| Clear code bug                            | Yes      | Yes         | action    | Apply + commit + resolve  |
| Outdated (already fixed in later commits) | N/A      | N/A         | no-action | Document reason + resolve |
| False positive (verified by tests)        | N/A      | Pass        | no-action | Document why + resolve    |
| Good suggestion but low priority          | No       | N/A         | no-action | Document reason + resolve |
| Docs improvement                          | Yes      | Yes         | action    | Apply + commit + resolve  |

## Helper Scripts Reference

### Fetch & inspect threads

- `../fetch-review-threads/scripts/get-pr-review-threads.sh` — Fetch all threads for a PR
- `../fetch-review-threads/scripts/list-unresolved-threads.sh` — Filter to unresolved threads only
- `../fetch-review-threads/scripts/show-unresolved-thread-bodies.sh` — Show full body of each unresolved thread
- `../fetch-review-threads/scripts/check-thread-reply-status.sh` — Report which unresolved threads are missing a reply (exits 1 if any are missing)

### Reply & resolve threads

- `../resolve-review-threads/scripts/reply-and-resolve-thread.sh` — Post a reply then resolve a single thread (preferred per-thread operation)
- `../resolve-review-threads/scripts/reply-to-thread.sh` — Post a reply on a thread without resolving it
- `../resolve-review-threads/scripts/resolve-all-unresolved-threads.sh` — Bulk-resolve all unresolved threads (use only after `check-thread-reply-status.sh` exits 0)

## Related Skills

- **fetch-review-threads** — Deep dive on collecting thread metadata
- **resolve-review-threads** — Deep dive on resolving threads via GraphQL

Both are integrated into this workflow automatically.

## Example

See `docs/copilot-pr-reviews/EXAMPLE-COMPLETED.md` for a complete worked example
with all 26 Copilot suggestions processed, decided, and resolved.

## Completion Checklist

- [ ] Tracker file created from template with PR number and URL
- [ ] All review threads fetched and added to tracker table
- [ ] Each thread categorized as `action` or `no-action` with rationale
- [ ] All `action` items implemented, validated, and committed
- [ ] Every thread replied to with `reply-and-resolve-thread.sh` (reply URL recorded in tracker)
- [ ] All threads resolved in GitHub (`list-unresolved-threads.sh` returns no output)
- [ ] Tracker file updated with Processing Log and Thread State column
- [ ] Tracker committed as documentation
- [ ] No uncommitted changes remain
