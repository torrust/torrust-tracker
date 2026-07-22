---
semantic-links:
  skill-links:
    - process-copilot-suggestions
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-copilot-suggestions/SKILL.md
---

<!-- cspell:disable -->

<!-- skill-link: process-copilot-suggestions -->

# PR #2020 Copilot Suggestions Tracking

Source: Copilot PR review threads for https://github.com/torrust/torrust-tracker/pull/2020

Status legend:

- `action`: code/docs change applied
- `no-action`: suggestion reviewed; no code change needed
- `resolved`: thread resolved in PR

## Workflow

1. Download all review threads (including resolved/outdated state and thread IDs).
2. Add one row per thread in the Suggestions table.
3. Process suggestions one by one: decide, implement and validate action items, reply on the PR thread, then resolve the thread.
4. Set `Thread State` to `resolved` once resolved in PR.

## Processing Log

- 2026-07-22: Started processing six Copilot suggestions.
- 2026-07-22: Applied the accepted fixes in signed commit `b917355c` and replied to and resolved all six original threads.

## Suggestions

| #   | Thread ID             | Path                                                               | URL                                                                         | Suggestion Summary                                           | Decision                                                               | Reply URL                                                                   | Status | Thread State |
| --- | --------------------- | ------------------------------------------------------------------ | --------------------------------------------------------------------------- | ------------------------------------------------------------ | ---------------------------------------------------------------------- | --------------------------------------------------------------------------- | ------ | ------------ |
| 1   | PRRT_kwDOGp2yqc6S2_XP | `contrib/dev-tools/git/tests/test-format-project-words.sh`         | https://github.com/torrust/torrust-tracker/pull/2020#discussion_r3628911184 | Ensure assertions fail the test script.                      | action: enabled fail-fast shell execution.                             | https://github.com/torrust/torrust-tracker/pull/2020#discussion_r3628954196 | DONE   | RESOLVED     |
| 2   | PRRT_kwDOGp2yqc6S2_Xn | `contrib/dev-tools/git/format-project-words.sh`                    | https://github.com/torrust/torrust-tracker/pull/2020#discussion_r3628911225 | Replace the dictionary atomically.                           | action: used a same-directory temporary file and `mv`.                 | https://github.com/torrust/torrust-tracker/pull/2020#discussion_r3628955657 | DONE   | RESOLVED     |
| 3   | PRRT_kwDOGp2yqc6S2_X4 | `contrib/dev-tools/git/hooks/pre-commit.sh`                        | https://github.com/torrust/torrust-tracker/pull/2020#discussion_r3628911249 | Do not mislabel formatter operational errors as changes.     | action: show restaging guidance only for formatter exit code 1.        | https://github.com/torrust/torrust-tracker/pull/2020#discussion_r3628957067 | DONE   | RESOLVED     |
| 4   | PRRT_kwDOGp2yqc6S2_YS | `.github/skills/dev/git-workflow/run-pre-commit-checks/SKILL.md`   | https://github.com/torrust/torrust-tracker/pull/2020#discussion_r3628911279 | Synchronize documented hook steps.                           | action: added `cargo deny check bans` and the current machete command. | https://github.com/torrust/torrust-tracker/pull/2020#discussion_r3628958264 | DONE   | RESOLVED     |
| 5   | PRRT_kwDOGp2yqc6S2_Yt | `docs/issues/open/2019-automatically-format-project-dictionary.md` | https://github.com/torrust/torrust-tracker/pull/2020#discussion_r3628911319 | Keep completed acceptance criteria consistent with evidence. | action: marked verified criteria complete.                             | https://github.com/torrust/torrust-tracker/pull/2020#discussion_r3628959344 | DONE   | RESOLVED     |
| 6   | PRRT_kwDOGp2yqc6S2_ZL | `docs/issues/open/2019-automatically-format-project-dictionary.md` | https://github.com/torrust/torrust-tracker/pull/2020#discussion_r3628911359 | Replace stale pending acceptance-verification entries.       | action: recorded completion evidence.                                  | https://github.com/torrust/torrust-tracker/pull/2020#discussion_r3628960975 | DONE   | RESOLVED     |

## Notes

- The linked `process-copilot-suggestions` skill was reviewed while updating this tracker; its workflow requires no change.
