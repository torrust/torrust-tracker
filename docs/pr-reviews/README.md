---
semantic-links:
  skill-links:
    - process-copilot-suggestions
  related-artifacts:
    - docs/index.md
    - docs/templates/COPILOT-SUGGESTIONS-TEMPLATE.md
    - .github/skills/dev/pr-reviews/process-copilot-suggestions/SKILL.md
---

# PR Copilot Suggestions Review Workflow

This directory contains tools and templates for managing GitHub Copilot code review suggestions on pull requests.

## Files

- [docs/templates/COPILOT-SUGGESTIONS-TEMPLATE.md](../templates/COPILOT-SUGGESTIONS-TEMPLATE.md) — Reusable template for tracking and processing Copilot suggestions on any PR. Copy and customize for each new PR.
- **pr-1733-copilot-suggestions.md** — Example of a completed suggestion review for PR #1733, showing how to document decisions, process suggestions, and track resolutions.

## Workflow

1. **Setup** — Copy [docs/templates/COPILOT-SUGGESTIONS-TEMPLATE.md](../templates/COPILOT-SUGGESTIONS-TEMPLATE.md) to a new file named `pr-<PR_NUMBER>-copilot-suggestions.md` in `docs/pr-reviews/`.

2. **Download threads** — Use `bash .github/skills/dev/pr-reviews/fetch-review-threads/scripts/get-pr-review-threads.sh --pr-number <PR_NUMBER> --output-file /tmp/pr_threads_<PR_NUMBER>.json` to fetch all review threads.

3. **List and analyze** — Use `bash .github/skills/dev/pr-reviews/fetch-review-threads/scripts/list-unresolved-threads.sh --threads-file /tmp/pr_threads_<PR_NUMBER>.json` to see unresolved suggestions, then review each one to determine if code/doc changes are needed.

4. **Apply changes** — For `action` items, apply fixes, validate with linters/tests, and commit.

5. **Resolve threads** — Use `bash .github/skills/dev/pr-reviews/resolve-review-threads/scripts/resolve-all-unresolved-threads.sh --threads-file /tmp/pr_threads_<PR_NUMBER>.json` to mark all processed suggestions as resolved in GitHub.

6. **Document** — Update the tracker file with decisions and thread states, then commit as part of the PR documentation.

## Example

See `pr-1733-copilot-suggestions.md` for a complete example where all 26 Copilot suggestions were reviewed, processed, and resolved.
