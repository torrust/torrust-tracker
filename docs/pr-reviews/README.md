# PR Copilot Suggestions Review Workflow

This directory contains tools and templates for managing GitHub Copilot code review suggestions on pull requests.

## Files

- **COPILOT-SUGGESTIONS-TEMPLATE.md** — Reusable template for tracking and processing copilot suggestions on any PR. Copy and customize for each new PR.
- **pr-1733-copilot-suggestions.md** — Example of a completed suggestion review for PR #1733, showing how to document decisions, process suggestions, and track resolutions.

## Workflow

1. **Setup** — Copy `COPILOT-SUGGESTIONS-TEMPLATE.md` to a new file named `pr-<PR_NUMBER>-copilot-suggestions.md`.

2. **Download threads** — Use `contrib/dev-tools/github-api-scripts/get-pr-review-threads.sh <PR_NUMBER>` to fetch all review threads.

3. **List and analyze** — Use `list-unresolved-threads.sh` to see unresolved suggestions, then review each one to determine if code/doc changes are needed.

4. **Apply changes** — For `action` items, apply fixes, validate with linters/tests, and commit.

5. **Resolve threads** — Use `resolve-all-unresolved-threads.sh` to mark all processed suggestions as resolved in GitHub.

6. **Document** — Update the tracker file with decisions and thread states, then commit as part of the PR documentation.

## Example

See `pr-1733-copilot-suggestions.md` for a complete example where all 26 Copilot suggestions were reviewed, processed, and resolved.
