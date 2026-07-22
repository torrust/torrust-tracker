---
semantic-links:
  skill-links:
    - process-copilot-suggestions
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-copilot-suggestions/SKILL.md
---

<!-- cspell:disable -->

<!-- skill-link: process-copilot-suggestions -->

# PR #2024 Copilot Suggestions Tracking

Source: Copilot PR review threads for https://github.com/torrust/torrust-tracker/pull/2024

Status legend:

- `action`: code/docs change applied
- `no-action`: suggestion reviewed; no code change needed
- `resolved`: thread resolved in PR

## Workflow

1. Download all review threads (including resolved/outdated state and thread IDs).
2. Add one row per thread in the Suggestions table.
3. Process suggestions one by one: decide `action` or `no-action`; if `action`, apply and validate the change; commit if needed; reply on the PR thread; then resolve it.
4. Set `Thread State` to `RESOLVED` once resolved in the PR.

## Processing Log

- 2026-07-22: Started processing five unresolved Copilot suggestions.
- 2026-07-22: Applied and pushed signed commit `4af6f8ca` for all five suggestions; replied to and resolved each thread.
- 2026-07-22: Completed the initial Copilot suggestion audit; a final PR refresh follows this tracker commit.

## Suggestions

| #   | Thread ID             | Path                                                                                | URL                                                                         | Suggestion Summary                                          | Decision                                                                                                                                              | Reply URL                                                                   | Status | Thread State |
| --- | --------------------- | ----------------------------------------------------------------------------------- | --------------------------------------------------------------------------- | ----------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------- | ------ | ------------ |
| 1   | PRRT_kwDOGp2yqc6S90eA | .github/skills/dev/planning/create-issue/SKILL.md                                   | https://github.com/torrust/torrust-tracker/pull/2024#discussion_r3631407338 | Remove stale MCP issue-creation tool reference              | action: removed the unavailable tool name; the supported GitHub CLI command remains the repository-local workflow.                                    | https://github.com/torrust/torrust-tracker/pull/2024#discussion_r3631590602 | DONE   | RESOLVED     |
| 2   | PRRT_kwDOGp2yqc6S90ef | .github/skills/dev/planning/create-issue/SKILL.md                                   | https://github.com/torrust/torrust-tracker/pull/2024#discussion_r3631407376 | Reconcile single-file and folder-based spec layout guidance | action: clarified the canonical `docs/issues/open/AGENTS.md` guidance that both layouts are supported, selected by presence of issue-local artifacts. | https://github.com/torrust/torrust-tracker/pull/2024#discussion_r3631594027 | DONE   | RESOLVED     |
| 3   | PRRT_kwDOGp2yqc6S90e8 | docs/AGENTS.md                                                                      | https://github.com/torrust/torrust-tracker/pull/2024#discussion_r3631407410 | Align open-spec placement guidance                          | action: clarified the folder-based path and aligned the open-issues convention with the existing single-file and folder-based layouts.                | https://github.com/torrust/torrust-tracker/pull/2024#discussion_r3631596324 | DONE   | RESOLVED     |
| 4   | PRRT_kwDOGp2yqc6S90fi | docs/issues/open/2022-vendor-and-document-maintainer-merge-workflow/ISSUE.md        | https://github.com/torrust/torrust-tracker/pull/2024#discussion_r3631407458 | Correct inaccurate draft-status wording                     | action: changed the reference from “folder-style draft” to “folder-style specification” because this is an open issue specification.                  | https://github.com/torrust/torrust-tracker/pull/2024#discussion_r3631600857 | DONE   | RESOLVED     |
| 5   | PRRT_kwDOGp2yqc6S90f2 | docs/issues/open/2022-vendor-and-document-maintainer-merge-workflow/github-merge.py | https://github.com/torrust/torrust-tracker/pull/2024#discussion_r3631407484 | Include the referenced MIT license text                     | action: added the matching MIT `COPYING` file next to the immutable planning snapshot; the snapshot's recorded SHA-256 remains unchanged.             | https://github.com/torrust/torrust-tracker/pull/2024#discussion_r3631607131 | DONE   | RESOLVED     |
| 6   | PRRT_kwDOGp2yqc6S-h5M | docs/issues/open/AGENTS.md                                                          | https://github.com/torrust/torrust-tracker/pull/2024#discussion_r3631668799 | Clarify folder-based heading hierarchy                      | action: reorganized the folder-based headings in signed commit `890c59f9`; the full lint and pre-commit gates passed.                                 | https://github.com/torrust/torrust-tracker/pull/2024#discussion_r3631780451 | DONE   | RESOLVED     |
| 7   | PRRT_kwDOGp2yqc6S-h5v | docs/issues/open/AGENTS.md                                                          | https://github.com/torrust/torrust-tracker/pull/2024#discussion_r3631668847 | Use an existing folder-based issue example                  | action: replaced the nonexistent example with the existing #2022 folder-based issue specification; pending validation and PR reply.                   | —                                                                           | OPEN   | UNRESOLVED   |
| 8   | PRRT_kwDOGp2yqc6S-h53 | docs/issues/open/AGENTS.md                                                          | https://github.com/torrust/torrust-tracker/pull/2024#discussion_r3631668862 | Use existing paths in the summary table                     | pending review                                                                                                                                        | —                                                                           | OPEN   | UNRESOLVED   |
| 9   | PRRT_kwDOGp2yqc6S-h6B | docs/issues/open/AGENTS.md                                                          | https://github.com/torrust/torrust-tracker/pull/2024#discussion_r3631668877 | Document standalone EPIC layout                             | pending review                                                                                                                                        | —                                                                           | OPEN   | UNRESOLVED   |

## Notes

- Keep this file as an audit log of review handling for the PR.
- Reply on every PR suggestion thread before resolving it so the decision is visible to reviewers.
