---
semantic-links:
  skill-links:
    - process-copilot-suggestions
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-copilot-suggestions/SKILL.md
---

<!-- cspell:disable -->

<!-- skill-link: process-copilot-suggestions -->

# PR #2173 Copilot Suggestions Tracking

Source: Copilot PR review threads for <https://github.com/torrust/torrust-tracker/pull/2173>

Status legend:

- `action`: code/docs change applied
- `no-action`: suggestion reviewed; no code change needed
- `resolved`: thread resolved in PR

## Processing Log

- 2026-09-08: Started processing suggestions.
- 2026-09-08: Replaced the `script(1)` existence check with a functional probe of the `--command` and `--return` flags the suite uses, so an incompatible implementation fails by name before the tests run; reply and thread resolution still pending.

## Suggestions

| #   | Thread ID | Path                                                       | URL     | Suggestion Summary                                                                                                                          | Decision | Reply URL | Status | Thread State |
| --- | --------- | ---------------------------------------------------------- | ------- | -------------------------------------------------------------------------------------------------------------------------------------------- | -------- | --------- | ------ | ------------ |
| 1   | Pending   | `contrib/dev-tools/git/tests/test-merge-pull-request.sh`   | Pending | `require_pseudo_terminal_support` checked only that `script` exists; implementations that reject `--command`/`--return` failed later with a confusing usage error. | action   | Pending   | OPEN   | OPEN         |

## Notes

- Keep this file as an audit log of review handling for the PR.
- Reply on every PR suggestion thread before resolving it so the decision is visible to reviewers.
- Fix commit: `dfbc9f48` — probes `script --quiet --return --command true /dev/null`, the exact flags `run_in_pseudo_terminal` uses, and fails fast with a named reason. Verified by running the suite (passes) and by re-running it with a stub `script` that rejects `--command` (fails immediately with the named error).
- Thread ID and comment URL are pending the review-thread fetch; fill them in with the reply URL when the thread is answered and resolved.
