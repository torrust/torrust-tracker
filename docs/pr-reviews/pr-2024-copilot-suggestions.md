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

Table value legend:

- `Decision`: `action` means a code or documentation change was applied; `no-action` means the suggestion was reviewed and no change was needed.
- `Status`: `DONE` means the suggestion has been processed; `OPEN` means processing remains.
- `Thread State`: `RESOLVED` means the PR thread has been resolved; `UNRESOLVED` means it remains open.

## Workflow

1. Download all review threads (including resolved/outdated state and thread IDs).
2. Add one row per thread in the Suggestions table.
3. Process suggestions one by one: decide `action` or `no-action`; if `action`, apply and validate the change; commit if needed; reply on the PR thread; then resolve it.
4. Set `Thread State` to `RESOLVED` once resolved in the PR.

## Processing Log

- 2026-07-22: Started processing five unresolved Copilot suggestions.
- 2026-07-22: Applied and pushed signed commit `4af6f8ca` for all five suggestions; replied to and resolved each thread.
- 2026-07-22: Completed the initial five-suggestion audit; later Copilot suggestions are tracked separately below.
- 2026-07-22: Applied and pushed signed commits `890c59f9`, `139f7f5c`, `10a6e06c`, and `a56b2b66` for four newer suggestions; replied to and resolved each thread.
- 2026-07-22: Verified the audit-tracker consistency correction in signed commit `f25e56d7`; replied to and resolved the related thread.
- 2026-07-22: Applied and pushed signed commit `722909ef` for the remaining path-consistency suggestion; replied to and resolved the related thread.
- 2026-07-22: Applied and pushed signed commit `651e49bb` to clarify the table value legend; replied to and resolved the related thread.
- 2026-07-22: Identified the exact unfiltered Copilot thread `PRRT_kwDOGp2yqc6TAiPx`; corrected the broken lifecycle-document links, validated the documentation, and will reply and resolve it after the signed commit is pushed.

## Suggestions

| #   | Thread ID             | Path                                                                                | URL                                                                         | Suggestion Summary                                          | Decision                                                                                                                                                                                   | Reply URL                                                                   | Status | Thread State |
| --- | --------------------- | ----------------------------------------------------------------------------------- | --------------------------------------------------------------------------- | ----------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | --------------------------------------------------------------------------- | ------ | ------------ |
| 1   | PRRT_kwDOGp2yqc6S90eA | .github/skills/dev/planning/create-issue/SKILL.md                                   | https://github.com/torrust/torrust-tracker/pull/2024#discussion_r3631407338 | Remove stale MCP issue-creation tool reference              | action: removed the unavailable tool name; the supported GitHub CLI command remains the repository-local workflow.                                                                         | https://github.com/torrust/torrust-tracker/pull/2024#discussion_r3631590602 | DONE   | RESOLVED     |
| 2   | PRRT_kwDOGp2yqc6S90ef | .github/skills/dev/planning/create-issue/SKILL.md                                   | https://github.com/torrust/torrust-tracker/pull/2024#discussion_r3631407376 | Reconcile single-file and folder-based spec layout guidance | action: clarified the canonical `docs/issues/open/AGENTS.md` guidance that both layouts are supported, selected by presence of issue-local artifacts.                                      | https://github.com/torrust/torrust-tracker/pull/2024#discussion_r3631594027 | DONE   | RESOLVED     |
| 3   | PRRT_kwDOGp2yqc6S90e8 | docs/AGENTS.md                                                                      | https://github.com/torrust/torrust-tracker/pull/2024#discussion_r3631407410 | Align open-spec placement guidance                          | action: clarified the folder-based path and aligned the open-issues convention with the existing single-file and folder-based layouts.                                                     | https://github.com/torrust/torrust-tracker/pull/2024#discussion_r3631596324 | DONE   | RESOLVED     |
| 4   | PRRT_kwDOGp2yqc6S90fi | docs/issues/open/2022-vendor-and-document-maintainer-merge-workflow/ISSUE.md        | https://github.com/torrust/torrust-tracker/pull/2024#discussion_r3631407458 | Correct inaccurate draft-status wording                     | action: changed the reference from “folder-style draft” to “folder-style specification” because this is an open issue specification.                                                       | https://github.com/torrust/torrust-tracker/pull/2024#discussion_r3631600857 | DONE   | RESOLVED     |
| 5   | PRRT_kwDOGp2yqc6S90f2 | docs/issues/open/2022-vendor-and-document-maintainer-merge-workflow/github-merge.py | https://github.com/torrust/torrust-tracker/pull/2024#discussion_r3631407484 | Include the referenced MIT license text                     | action: added the matching MIT `COPYING` file next to the immutable planning snapshot; the snapshot's recorded SHA-256 remains unchanged.                                                  | https://github.com/torrust/torrust-tracker/pull/2024#discussion_r3631607131 | DONE   | RESOLVED     |
| 6   | PRRT_kwDOGp2yqc6S-h5M | docs/issues/open/AGENTS.md                                                          | https://github.com/torrust/torrust-tracker/pull/2024#discussion_r3631668799 | Clarify folder-based heading hierarchy                      | action: reorganized the folder-based headings in signed commit `890c59f9`; the full lint and pre-commit gates passed.                                                                      | https://github.com/torrust/torrust-tracker/pull/2024#discussion_r3631780451 | DONE   | RESOLVED     |
| 7   | PRRT_kwDOGp2yqc6S-h5v | docs/issues/open/AGENTS.md                                                          | https://github.com/torrust/torrust-tracker/pull/2024#discussion_r3631668847 | Use an existing folder-based issue example                  | action: replaced the nonexistent example with the existing #2022 folder-based issue specification in signed commit `139f7f5c`; gates passed.                                               | https://github.com/torrust/torrust-tracker/pull/2024#discussion_r3631932871 | DONE   | RESOLVED     |
| 8   | PRRT_kwDOGp2yqc6S-h53 | docs/issues/open/AGENTS.md                                                          | https://github.com/torrust/torrust-tracker/pull/2024#discussion_r3631668862 | Use existing paths in the summary table                     | action: replaced fictional folder examples with current open or closed specifications in signed commit `10a6e06c`; gates passed.                                                           | https://github.com/torrust/torrust-tracker/pull/2024#discussion_r3631961855 | DONE   | RESOLVED     |
| 9   | PRRT_kwDOGp2yqc6S-h6B | docs/issues/open/AGENTS.md                                                          | https://github.com/torrust/torrust-tracker/pull/2024#discussion_r3631668877 | Document standalone EPIC layout                             | action: documented standalone EPIC layout with the existing #1978 EPIC specification in signed commit `a56b2b66`; gates passed.                                                            | https://github.com/torrust/torrust-tracker/pull/2024#discussion_r3632004367 | DONE   | RESOLVED     |
| 10  | PRRT_kwDOGp2yqc6S_eyk | docs/pr-reviews/pr-2024-copilot-suggestions.md                                      | https://github.com/torrust/torrust-tracker/pull/2024#discussion_r3632020033 | Keep the tracker log and table states consistent            | action: verified the correction in signed commit `f25e56d7`, which scopes the initial completion log to the first five suggestions and records the later completed suggestions separately. | https://github.com/torrust/torrust-tracker/pull/2024#discussion_r3632311033 | DONE   | RESOLVED     |
| 11  | PRRT_kwDOGp2yqc6S_ezE | docs/issues/open/AGENTS.md                                                          | https://github.com/torrust/torrust-tracker/pull/2024#discussion_r3632020084 | Use consistent paths in the summary table                   | action: removed redundant `docs/issues/open/` prefixes from open folder-based examples in signed commit `722909ef`; `linter all` and the mandatory pre-commit gate passed.                 | https://github.com/torrust/torrust-tracker/pull/2024#discussion_r3632393824 | DONE   | RESOLVED     |
| 12  | PRRT_kwDOGp2yqc6TAiMK | docs/pr-reviews/pr-2024-copilot-suggestions.md                                      | https://github.com/torrust/torrust-tracker/pull/2024#discussion_r3632412953 | Clarify the table value legend                              | action: renamed and clarified the legend for the Decision, Status, and Thread State columns in signed commit `651e49bb`; `linter all` and the mandatory pre-commit gate passed.            | https://github.com/torrust/torrust-tracker/pull/2024#discussion_r3632455274 | DONE   | RESOLVED     |
| 13  | PRRT_kwDOGp2yqc6TAiPx | .github/skills/dev/planning/create-issue/SKILL.md                                   | https://github.com/torrust/torrust-tracker/pull/2024#discussion_r3632519367 | Correct broken lifecycle-document relative links            | action: changed both lifecycle-document links from four to five parent-directory segments so they resolve from the skill directory to repository `docs/`.                                  | Pending                                                                     | OPEN   | UNRESOLVED   |

## Notes

- Keep this file as an audit log of review handling for the PR.
- Reply on every PR suggestion thread before resolving it so the decision is visible to reviewers.
