---
name: process-copilot-suggestions
description: End-to-end workflow for processing and resolving all Copilot code review suggestions on a pull request in torrust-tracker. Use when asked to handle PR review feedback, process all copilot suggestions, audit and resolve review comments, or manage copilot-generated review threads. Triggers on "process copilot suggestions", "handle all PR feedback", "resolve copilot review", "audit PR suggestions", or "close all copilot comments".
metadata:
  author: torrust
  version: "1.0"
  semantic-links:
    related-artifacts:
      - docs/templates/COPILOT-SUGGESTIONS-TEMPLATE.md
---

# Processing Copilot PR Suggestions

This is the primary workflow for handling all Copilot code review suggestions on a pull request.
It combines decision-making, implementation, tracking, and resolution into a structured end-to-end process.

## Overview

Copilot generates suggestions that fall into two categories:

- **action** — Code or documentation changes needed; implement, validate, commit
- **no-action** — Already handled, false positive, or intentionally declined; explain reasoning and mark resolved

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
   docs/pr-reviews/pr-<PR_NUMBER>-copilot-suggestions.md
```

Open the tracker file and fill in:

- `<PR_NUMBER>` and `<PR_URL>` at the top
- Placeholder columns in the Suggestions table

### 2. Fetch All Review Threads

Use the **fetch-review-threads** skill or the helper script:

```bash
contrib/dev-tools/github-api-scripts/get-pr-review-threads.sh <PR_NUMBER>
```

This saves all review threads (resolved, unresolved, outdated) to `/tmp/pr_threads_<PR_NUMBER>.json`.

### 3. Populate the Tracker

Extract unresolved threads from the JSON:

```bash
contrib/dev-tools/github-api-scripts/list-unresolved-threads.sh /tmp/pr_threads_<PR_NUMBER>.json
```

Add one row per thread to your tracker file with:

- Thread ID
- File path
- Comment URL
- Brief summary of the suggestion

### 4. Analyze and Decide

For each suggestion, decide:

- **action** — The suggestion identifies a real fix needed:
  - Apply the code/doc change
  - Run `linter all` and targeted tests
  - Commit with clear message
  - Update tracker with `action` status
- **no-action** — The suggestion is already handled or not needed:
  - Document the reason (e.g., "outdated after later commits", "false positive verified by tests")
  - Update tracker with `no-action` status and rationale

**Key principle**: Do not resolve a thread just because a suggestion exists. Only resolve when the concern is genuinely addressed or explicitly declined with documented reasoning.

### 5. Implement Fixes

For each `action` item:

1. Read the suggestion carefully
2. Apply the minimal fix
3. Validate:

   ```bash
   linter all                          # Full lint gate
   cargo test -p <affected-package>    # Targeted tests
   ```

4. Commit with GPG signature:

   ```bash
   git add <files>
   git commit -S -m "chore(review): <concise description>"
   ```

5. Update tracker with `action` status

### 6. Batch Resolve All Threads

After all decisions are made and `action` items are committed:

```bash
contrib/dev-tools/github-api-scripts/get-pr-review-threads.sh <PR_NUMBER>
contrib/dev-tools/github-api-scripts/resolve-all-unresolved-threads.sh /tmp/pr_threads_<PR_NUMBER>.json
```

This resolves all unresolved threads (both `action` and `no-action` categories).

### 7. Final Documentation

Update the tracker file with completion notes:

- Add timestamps to the Processing Log
- Mark all threads as `resolved` in the Thread State column
- Commit the tracker and all helper scripts as final documentation

```bash
git add docs/pr-reviews/pr-<PR_NUMBER>-copilot-suggestions.md
git add contrib/dev-tools/github-api-scripts/
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

Located in `contrib/dev-tools/github-api-scripts/`:

- **get-pr-review-threads.sh** — Fetch all threads for a PR
- **list-unresolved-threads.sh** — Filter to unresolved threads only
- **resolve-all-unresolved-threads.sh** — Resolve all unresolved threads via GraphQL

See `contrib/dev-tools/github-api-scripts/README.md` for details.

## Related Skills

- **fetch-review-threads** — Deep dive on collecting thread metadata
- **resolve-review-threads** — Deep dive on resolving threads via GraphQL

Both are integrated into this workflow automatically.

## Example

See `docs/pr-reviews/pr-1733-copilot-suggestions.md` for a complete worked example
with all 26 Copilot suggestions processed, decided, and resolved.

## Completion Checklist

- [ ] Tracker file created from template with PR number and URL
- [ ] All review threads fetched and added to tracker table
- [ ] Each thread categorized as `action` or `no-action` with rationale
- [ ] All `action` items implemented, validated, and committed
- [ ] All threads resolved in GitHub (via batch script or one-by-one)
- [ ] Tracker file updated with Processing Log and Thread State column
- [ ] Tracker and helper scripts committed as documentation
- [ ] No uncommitted changes remain
