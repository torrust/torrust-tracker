---
semantic-links:
  skill-links:
    - process-copilot-suggestions
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-copilot-suggestions/SKILL.md
---

<!-- cspell:disable -->

<!-- skill-link: process-copilot-suggestions -->

# PR #2013 Copilot Suggestions Tracking

Source: Copilot PR review threads for https://github.com/torrust/torrust-tracker/pull/2013

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

- 2026-07-21: Started processing Copilot suggestions.
- 2026-07-21: Updated stale issue-spec references, validated the documentation change, pushed commit `01a4843d`, replied with the fix summary, and resolved the Copilot thread.
- 2026-07-21: Completed processing first suggestion.
- 2026-07-21: Added `<!-- cspell:disable -->` to tracker and template, committed in `2410d52d`, replied and resolved thread `PRRT_kwDOGp2yqc6Si2c6`.
- 2026-07-21: All suggestions processed.
- 2026-07-21: Three new threads opened by Copilot after last push. Fixed reply URL validation in `reply-to-thread.sh` and `reply-and-resolve-thread.sh`, and replaced `printf` JSON construction with `jq -n`/`--argjson` in `check-thread-reply-status.sh`. Committed `3039b382`, replied and resolved threads `PRRT_kwDOGp2yqc6Sj3Ce`, `PRRT_kwDOGp2yqc6Sj3Cy`, `PRRT_kwDOGp2yqc6Sj3DE`. All threads resolved.

## Suggestions

| #   | Thread ID               | Path                                                                                       | URL                                                                                    | Suggestion Summary                                                                                            | Decision                                                                                                                         | Reply URL                                                                            | Status | Thread State |
| --- | ----------------------- | ------------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------ | ------ | ------------ |
| 1   | `PRRT_kwDOGp2yqc6SitOP` | `docs/issues/open/1875-review-lto-fat-in-dev-profile/ISSUE.md`                             | [comment](https://github.com/torrust/torrust-tracker/pull/2013#discussion_r3621393047) | Update stale references to the standalone issue-spec path.                                                    | action — updated the EPIC's direct references and migrated the open-issues naming convention to folder specs.                    | [reply](https://github.com/torrust/torrust-tracker/pull/2013#discussion_r3621442150) | DONE   | RESOLVED     |
| 2   | `PRRT_kwDOGp2yqc6Si2c6` | `docs/copilot-pr-reviews/pr-2013-copilot-suggestions.md`                                   | [comment](https://github.com/torrust/torrust-tracker/pull/2013#discussion_r3621444815) | Add `<!-- cspell:disable -->` to avoid spell-check failures on opaque thread IDs.                             | action — added `<!-- cspell:disable -->` to the tracker file and to the template so future PR trackers include it automatically. | [reply](https://github.com/torrust/torrust-tracker/pull/2013#discussion_r3621514178) | DONE   | RESOLVED     |
| 3   | `PRRT_kwDOGp2yqc6SjBkm` | `docs/copilot-pr-reviews/pr-2013-copilot-suggestions.md`                                   | [comment](https://github.com/torrust/torrust-tracker/pull/2013#discussion_r3621507603) | Workflow section missing the explicit reply-before-resolve step.                                              | action — added the reply step to the workflow list in this file to match the template and skill.                                 | [reply](https://github.com/torrust/torrust-tracker/pull/2013#discussion_r3621586425) | DONE   | RESOLVED     |
| 4   | `PRRT_kwDOGp2yqc6Sjapq` | `docs/copilot-pr-reviews/pr-2013-copilot-suggestions.md`                                   | [comment](https://github.com/torrust/torrust-tracker/pull/2013#discussion_r3621648952) | Missing blank line before step 4 in Workflow list causes unreliable rendering.                                | action — added blank line before step 4.                                                                                         | [reply](https://github.com/torrust/torrust-tracker/pull/2013#discussion_r3621692387) | DONE   | RESOLVED     |
| 5   | `PRRT_kwDOGp2yqc6Sj3Ce` | `.github/skills/dev/pr-reviews/resolve-review-threads/scripts/reply-to-thread.sh`          | [comment](https://github.com/torrust/torrust-tracker/pull/2013#discussion_r3621808021) | `gh api graphql --jq` can return empty/`null` while exiting 0; validate `REPLY_URL` before reporting success. | action — added guard after GraphQL call: exit 1 if `REPLY_URL` is empty or `null`.                                               | [reply](https://github.com/torrust/torrust-tracker/pull/2013#discussion_r3621845797) | DONE   | RESOLVED     |
| 6   | `PRRT_kwDOGp2yqc6Sj3Cy` | `.github/skills/dev/pr-reviews/fetch-review-threads/scripts/check-thread-reply-status.sh`  | [comment](https://github.com/torrust/torrust-tracker/pull/2013#discussion_r3621808055) | `printf` with `%s` emits `"null"` string when url is JSON null; use `jq -n`/`--argjson` for correct types.    | action — replaced `printf` with `jq -n --argjson` to preserve JSON types.                                                        | [reply](https://github.com/torrust/torrust-tracker/pull/2013#discussion_r3621847390) | DONE   | RESOLVED     |
| 7   | `PRRT_kwDOGp2yqc6Sj3DE` | `.github/skills/dev/pr-reviews/resolve-review-threads/scripts/reply-and-resolve-thread.sh` | [comment](https://github.com/torrust/torrust-tracker/pull/2013#discussion_r3621808078) | GraphQL mutation can return empty/`null` URL while exiting 0; validate before proceeding to resolve.          | action — added guard after GraphQL call: exit 1 if `REPLY_URL` is empty or `null`.                                               | [reply](https://github.com/torrust/torrust-tracker/pull/2013#discussion_r3621849094) | DONE   | RESOLVED     |

## Notes

- Keep this file as an audit log of review handling for the PR.
- Prefer concise decisions with explicit rationale.
- If no code changes are needed, explain why in `Decision`.
