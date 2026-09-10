---
semantic-links:
  skill-links:
    - process-copilot-suggestions
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-copilot-suggestions/SKILL.md
    - docs/issues/open/2190-maintenance-frictions-cleanup/EPIC.md
    - docs/templates/ISSUE.md
    - docs/templates/EPIC.md
---

<!-- cspell:disable -->

<!-- skill-link: process-copilot-suggestions -->

# PR #2193 Copilot Suggestions Tracking

Source: Copilot PR review threads for <https://github.com/torrust/torrust-tracker/pull/2193>

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

- 2026-09-10: Reviewed the `last-updated-utc` timestamp-format suggestion against the repository templates and every existing specification. `docs/templates/ISSUE.md:11`, `docs/templates/EPIC.md:7`, `docs/templates/REFACTOR-PLAN.md:6`, and `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md:4` all prescribe `YYYY-MM-DD HH:MM`, and all 68 `last-updated-utc` values under `docs/issues/open/` and `docs/issues/drafts/` use that form; none uses an ISO-8601 designator. Decided `no-action`: the field follows the repository template, and changing one specification alone would make it the only file out of line with the convention.

## Suggestions

| #   | Thread ID               | Path                                                              | URL                                                                           | Suggestion Summary                                                                             | Decision                                                                                                                     | Reply URL   | Status | Thread State |
| --- | ----------------------- | ----------------------------------------------------------------- | ----------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------ | ----------- | ------ | ------------ |
| 1   | `PRRT_kwDOGp2yqc6g_jtS` | `docs/issues/open/2190-maintenance-frictions-cleanup/ISSUE.md`    | <https://github.com/torrust/torrust-tracker/pull/2193#discussion_r3977110619> | Use an ISO-8601 UTC form for `last-updated-utc` instead of `YYYY-MM-DD HH:MM`.                 | `no-action`: the format is the one the repository templates prescribe and every one of the 68 existing specifications uses. | {reply URL} | DONE   | TODO         |

## Notes

- The commented path is the specification's path before this pull request renamed it to `EPIC.md`; the field itself is unchanged and is now at `docs/issues/open/2190-maintenance-frictions-cleanup/EPIC.md:11`.
- A repository-wide move to ISO-8601 timestamps would be a convention change across the templates and every existing specification, not a change to one file. If it is wanted, it belongs in its own issue.
