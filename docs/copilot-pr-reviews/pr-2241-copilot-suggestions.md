---
semantic-links:
  skill-links:
    - process-copilot-suggestions
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-copilot-suggestions/SKILL.md
    - .github/skills/dev/pr-reviews/fetch-review-threads/SKILL.md
    - .github/skills/dev/pr-reviews/resolve-review-threads/SKILL.md
---

<!-- cspell:disable -->

<!-- skill-link: process-copilot-suggestions -->

# PR #2241 Copilot Suggestions Tracking

Source: Copilot PR review threads for https://github.com/torrust/torrust-tracker/pull/2241

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
   - reply on the PR thread with the fix commit and outcome, or the no-action rationale
   - resolve the PR thread

4. Set `Thread State` to `resolved` once resolved in PR.

## Processing Log

- 2026-09-16: Started processing Copilot suggestions for the spec-only review.
- 2026-09-16: Applied the documentation fixes and resolved each thread after posting a reply on the PR.

## Suggestions

| # | Thread ID | Path | URL | Suggestion Summary | Decision | Reply URL | Status | Thread State |
| --- | ----------- | ---- | --- | ------------------ | -------- | --------- | ------ | ------------ |
| 1 | PRRT_kwDOGp2yqc6jB-Vj | docs/issues/open/2226-fix-stale-inactivity-cutoff-in-activity-metrics-updater/evidence.md | https://github.com/torrust/torrust-tracker/pull/2241#discussion_r4028609405 | Clarify that the tracker command must run in a separate terminal because it blocks the foreground session | action | pending | DONE | RESOLVED |
| 2 | PRRT_kwDOGp2yqc6jB-Wm | docs/issues/open/2230-add-fix-bug-skill-and-bug-spec-guardrails/ISSUE.md | https://github.com/torrust/torrust-tracker/pull/2241#discussion_r4028609508 | Fix the leading double-pipe table formatting in the Markdown tables | action | pending | DONE | RESOLVED |
| 3 | PRRT_kwDOGp2yqc6jB-XM | docs/issues/open/2230-add-fix-bug-skill-and-bug-spec-guardrails/ISSUE.md | https://github.com/torrust/torrust-tracker/pull/2241#discussion_r4028609563 | Align the skill taxonomy under `.github/skills/dev/...` and avoid mixed `add-new-skill` paths | action | pending | DONE | RESOLVED |

## Notes

- The review feedback was documentation-only and therefore fixed in place without changing the spec scope.
- Each thread was replied to before resolution so the rationale is visible in the PR history.
- The final resolution was confirmed by re-fetching the review threads and confirming `list-unresolved-threads.sh` returned no output.
