---
semantic-links:
  skill-links:
    - process-copilot-suggestions
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-copilot-suggestions/SKILL.md
---

<!-- cspell:disable -->

<!-- skill-link: process-copilot-suggestions -->

# PR #2027 Copilot Suggestions Tracking

Source: Copilot PR review threads for https://github.com/torrust/torrust-tracker/pull/2027

## Processing Log

- 2026-07-23: Started processing five unresolved Copilot suggestions.

## Suggestions

| #   | Thread ID             | Path                                                          | URL                                                                         | Suggestion Summary                                                               | Decision                                                                                                                                        | Reply URL                                                                   | Status | Thread State |
| --- | --------------------- | ------------------------------------------------------------- | --------------------------------------------------------------------------- | -------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------- | ------ | ------------ |
| 1   | PRRT_kwDOGp2yqc6TMx43 | `contrib/dev-tools/git/merge-pull-request.sh`                 | https://github.com/torrust/torrust-tracker/pull/2027#discussion_r3637020386 | Make repository-config failures actionable and distinguish unset from incorrect. | action: add distinct remediation messages with the required Git command.                                                                        | https://github.com/torrust/torrust-tracker/pull/2027#discussion_r3637101935 | DONE   | RESOLVED     |
| 2   | PRRT_kwDOGp2yqc6TMx5E | `contrib/dev-tools/git/merge-pull-request.sh`                 | https://github.com/torrust/torrust-tracker/pull/2027#discussion_r3637020415 | Include the signing-key configuration command in the preflight failure.          | action: include the exact configuration command.                                                                                                | https://github.com/torrust/torrust-tracker/pull/2027#discussion_r3637123557 | DONE   | RESOLVED     |
| 3   | PRRT_kwDOGp2yqc6TMx5X | `contrib/dev-tools/git/merge-pull-request.sh`                 | https://github.com/torrust/torrust-tracker/pull/2027#discussion_r3637020442 | Validate the vendored tool and Python interpreter before delegation.             | action: add explicit non-dry-run availability checks.                                                                                           | https://github.com/torrust/torrust-tracker/pull/2027#discussion_r3637127241 | DONE   | RESOLVED     |
| 4   | PRRT_kwDOGp2yqc6TMx5y | `.github/skills/dev/git-workflow/merge-pull-request/SKILL.md` | https://github.com/torrust/torrust-tracker/pull/2027#discussion_r3637020491 | Do not assume a contributor-local upstream remote name.                          | action: use an explicit placeholder and describe how to select it.                                                                              | https://github.com/torrust/torrust-tracker/pull/2027#discussion_r3637131984 | DONE   | RESOLVED     |
| 5   | PRRT_kwDOGp2yqc6TMx6A | `.github/skills/dev/git-workflow/merge-pull-request/SKILL.md` | https://github.com/torrust/torrust-tracker/pull/2027#discussion_r3637020516 | Do not list an unused branch config as a wrapper prerequisite.                   | action: state that the wrapper passes `develop` directly.                                                                                       | https://github.com/torrust/torrust-tracker/pull/2027#discussion_r3637137050 | DONE   | RESOLVED     |
| 6   | PRRT_kwDOGp2yqc6TND_U | `project-words.txt`                                           | https://github.com/torrust/torrust-tracker/pull/2027#discussion_r3637126309 | Keep dictionary entries in deterministic `LC_ALL=C` order.                       | no-action: `LC_ALL=C sort -cu project-words.txt` and the project formatter confirm the current `ghtoken` then `githubmerge` order is canonical. | https://github.com/torrust/torrust-tracker/pull/2027#discussion_r3637380968 | DONE   | RESOLVED     |
| 7   | PRRT_kwDOGp2yqc6TND_z | `contrib/dev-tools/git/tests/test-merge-pull-request.sh`      | https://github.com/torrust/torrust-tracker/pull/2027#discussion_r3637126356 | Isolate the unset repository fixture from global and system Git configuration.   | action: disable both configuration scopes for this assertion.                                                                                   | https://github.com/torrust/torrust-tracker/pull/2027#discussion_r3637413029 | DONE   | RESOLVED     |

## Completion

- 2026-07-23: All seven Copilot threads were replied to and resolved. Signed commits `83ff6ddad88df276797678aedccf03ead2faa6ea` and `8eccc7594558d6feb67ffbd87a279b11ac249bd6` contain the action items; thread 6 was verified as no-action. A final refresh is required after committing this audit update.
