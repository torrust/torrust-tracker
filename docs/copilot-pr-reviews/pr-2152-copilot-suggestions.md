---
semantic-links:
  skill-links:
    - process-copilot-suggestions
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - docs/issues/open/1347-overhaul-packages-testing/EPIC.md
---

<!-- cspell:disable -->

<!-- skill-link: process-copilot-suggestions -->

# PR #2152 Copilot Suggestions Tracking

Source: Copilot PR review threads for https://github.com/torrust/torrust-tracker/pull/2152

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

- 2026-09-07 10:33 UTC: Started processing two Copilot suggestions.

## Suggestions

| # | Thread ID | Path | URL | Suggestion Summary | Decision | Reply URL | Status | Thread State |
| - | --------- | ---- | --- | ------------------ | -------- | --------- | ------ | ------------ |
| 1 | `PRRT_kwDOGp2yqc6f4Fpo` | `.github/skills/dev/planning/create-issue/SKILL.md` | https://github.com/torrust/torrust-tracker/pull/2152#discussion_r3948837445 | Clarify how `Commit Points` apply to EPIC specifications. | action | Pending | OPEN | OPEN |
| 2 | `PRRT_kwDOGp2yqc6f4FqS` | `docs/issues/open/1347-overhaul-packages-testing/EPIC.md` | https://github.com/torrust/torrust-tracker/pull/2152#discussion_r3948837510 | Use a UTC timestamp for the new EPIC progress-log entry. | action | Pending | OPEN | OPEN |

## Notes

- This is a spec-only PR; the review fixes are limited to the existing planning skill and EPIC specification.
- Every suggestion is replied to before it is resolved.
