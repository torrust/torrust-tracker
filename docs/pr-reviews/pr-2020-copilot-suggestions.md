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

Column legend:

- **Decision**: `action` means a code or documentation change was applied; `no-action` means the suggestion was reviewed and declined with a documented rationale.
- **Status**: `OPEN` while a thread is being processed; `DONE` after it has been handled.
- **Thread State**: `OPEN` until the thread is resolved in the PR; `RESOLVED` after resolution.

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
| 7   | PRRT_kwDOGp2yqc6S3Lr0 | `contrib/dev-tools/git/format-project-words.sh`                    | https://github.com/torrust/torrust-tracker/pull/2020#discussion_r3628981491 | Report temporary-file creation failures explicitly.          | action: added the diagnostic and focused test coverage.                | https://github.com/torrust/torrust-tracker/pull/2020#discussion_r3629010458 | DONE   | RESOLVED     |
| 8   | PRRT_kwDOGp2yqc6S3LsT | `docs/issues/open/2019-automatically-format-project-dictionary.md` | https://github.com/torrust/torrust-tracker/pull/2020#discussion_r3628981533 | Align the issue specification with the documented layout.    | action: moved the spec to its documented `ISSUE.md` folder layout.     | https://github.com/torrust/torrust-tracker/pull/2020#discussion_r3629239329 | DONE | RESOLVED |
| 9   | PRRT_kwDOGp2yqc6S3T8_ | `contrib/dev-tools/git/hooks/pre-commit.sh`                        | https://github.com/torrust/torrust-tracker/pull/2020#discussion_r3629029141 | Retain the exact failed step exit code.                       | action: captured the `run_step` exit code directly.                    | https://github.com/torrust/torrust-tracker/pull/2020#discussion_r3629242405 | DONE | RESOLVED |
| 10  | PRRT_kwDOGp2yqc6S3dJZ | `contrib/dev-tools/git/format-project-words.sh`                    | https://github.com/torrust/torrust-tracker/pull/2020#discussion_r3629081831 | Support non-GNU local toolchains.                             | action: replaced GNU-only options with portable equivalents.           | https://github.com/torrust/torrust-tracker/pull/2020#discussion_r3629243737 | DONE | RESOLVED |
| 11  | PRRT_kwDOGp2yqc6S3k60 | `docs/pr-reviews/pr-2020-copilot-suggestions.md`                   | https://github.com/torrust/torrust-tracker/pull/2020#discussion_r3629126625 | Make tracker column meanings unambiguous.                     | action: replaced the ambiguous legend with column-specific definitions. | Pending | OPEN | OPEN |

## Notes

- The linked `process-copilot-suggestions` skill was reviewed while updating this tracker; its workflow requires no change.
