---
semantic-links:
  skill-links:
    - process-copilot-suggestions
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-copilot-suggestions/SKILL.md
---

<!-- skill-link: process-copilot-suggestions -->

# PR #<PR_NUMBER> Copilot Suggestions Tracking

Source: Copilot PR review threads for <PR_URL>

Status legend:

- `action`: code/docs change applied
- `no-action`: suggestion reviewed; no code change needed
- `resolved`: thread resolved in PR

## Workflow

1. Download all review threads (including resolved/outdated state and thread IDs).
2. Add one row per thread in the Suggestions table.
3. Process suggestions one by one:
   - decide `action` or `no-action`
   - if `action`, apply change and validate
   - if needed, commit changes
   - resolve the PR thread
4. Set `Thread State` to `resolved` once resolved in PR.

## Processing Log

- <YYYY-MM-DD>: Started processing suggestions.
- <YYYY-MM-DD>: Completed processing suggestions.

## Suggestions

| #   | Thread ID   | Path        | URL           | Suggestion Summary | Decision              | Status         | Thread State       |
| --- | ----------- | ----------- | ------------- | ------------------ | --------------------- | -------------- | ------------------ |
| 1   | <THREAD_ID> | <FILE_PATH> | <COMMENT_URL> | <SHORT_SUMMARY>    | <ACTION_OR_NO_ACTION> | <OPEN_OR_DONE> | <OPEN_OR_RESOLVED> |

## Notes

- Keep this file as an audit log of review handling for the PR.
- Prefer concise decisions with explicit rationale.
- If no code changes are needed, explain why in `Decision`.
