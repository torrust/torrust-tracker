---
semantic-links:
  skill-links:
    - process-copilot-suggestions
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-copilot-suggestions/SKILL.md
---

<!-- cspell:disable -->

<!-- skill-link: process-copilot-suggestions -->

# PR #2176 Copilot Suggestions Tracking

Source: Copilot PR review threads for <https://github.com/torrust/torrust-tracker/pull/2176>

Status legend:

- `action`: code/docs change applied
- `no-action`: suggestion reviewed; no code change needed
- `resolved`: thread resolved in PR

## Processing Log

- 2026-09-08: Started processing suggestions.
- 2026-09-08: Assessed the reported doubled leading pipe in the specification's tables as a false positive; no row begins with one and markdownlint passes, so no change was made; reply posted and thread resolved.
- 2026-09-08: Assessed the `last-updated-utc` timestamp format as correct for this repository; the value follows `docs/templates/ISSUE.md` and every other open specification, so no change was made; reply posted and thread resolved.
- 2026-09-08: Copilot re-reviewed the pull request after the push and opened four further threads.
- 2026-09-08: Dismissed threads 3 and 4, duplicate doubled-pipe reports against this tracker, on byte-level evidence; replies posted and threads resolved.
- 2026-09-08: Dismissed thread 5, the same report against the specification, on the same evidence; reply posted and thread resolved.
- 2026-09-08: Accepted thread 6 and parameterized the manifest path across the specification in commit `d3769ecb`; reply and resolution pending the push.

## Suggestions

| #   | Thread ID               | Path                                                          | URL                                                                           | Suggestion Summary                                                                            | Decision                                                                                                                                                                                                                                                                                    | Reply URL                                                                     | Status | Thread State |
| --- | ----------------------- | ------------------------------------------------------------- | ----------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------- | ------ | ------------ |
| 1   | `PRRT_kwDOGp2yqc6gWynQ` | `docs/issues/open/2175-merge-tool-symlink-exceptions/ISSUE.md` | <https://github.com/torrust/torrust-tracker/pull/2176#discussion_r3960866610> | Table rows reportedly start with a doubled leading pipe, adding an empty first column.        | no-action — verified false positive: no row in the file begins with a doubled pipe, every table uses the single-leading-pipe form the other open specifications use, and markdownlint passes on the file.                                                                                     | <https://github.com/torrust/torrust-tracker/pull/2176#discussion_r3961188601> | DONE   | RESOLVED     |
| 2   | `PRRT_kwDOGp2yqc6gWynt` | `docs/issues/open/2175-merge-tool-symlink-exceptions/ISSUE.md` | <https://github.com/torrust/torrust-tracker/pull/2176#discussion_r3960866654> | `last-updated-utc` carries no explicit UTC designator; consider an ISO-8601 timestamp instead. | no-action — the value follows the repository template exactly and matches every other open specification, with the timezone carried by the field name rather than by the value. Adopting an ISO-8601 form would be a template-level decision affecting all specifications, not a single-spec change. | <https://github.com/torrust/torrust-tracker/pull/2176#discussion_r3961189091> | DONE   | RESOLVED     |
| 3   | `PRRT_kwDOGp2yqc6gYMxO` | `docs/copilot-pr-reviews/pr-2176-copilot-suggestions.md`       | <https://github.com/torrust/torrust-tracker/pull/2176#discussion_r3961420756> | The tracker reportedly contradicts itself by using doubled leading pipes in its own table. | no-action — verified at byte level: grepping row starts for a doubled pipe returns zero matches, and a raw-byte listing shows one pipe followed by a space on every row. The tracker's own tables are well formed. | <https://github.com/torrust/torrust-tracker/pull/2176#discussion_r3961438542> | DONE    | RESOLVED     |
| 4   | `PRRT_kwDOGp2yqc6gYMx1` | `docs/copilot-pr-reviews/pr-2176-copilot-suggestions.md`       | <https://github.com/torrust/torrust-tracker/pull/2176#discussion_r3961420820> | Duplicate of thread 3: the same doubled leading pipe claim against the tracker.            | no-action — duplicate of row 3 and dismissed on the same byte-level evidence. | <https://github.com/torrust/torrust-tracker/pull/2176#discussion_r3961439089> | DONE    | RESOLVED     |
| 5   | `PRRT_kwDOGp2yqc6gYMyX` | `docs/issues/open/2175-merge-tool-symlink-exceptions/ISSUE.md` | <https://github.com/torrust/torrust-tracker/pull/2176#discussion_r3961420875> | The specification's tables reportedly begin with doubled pipes.                            | no-action — same byte-level verification as row 3 against the specification, and markdownlint passes on the file. | <https://github.com/torrust/torrust-tracker/pull/2176#discussion_r3961439649> | DONE    | RESOLVED     |
| 6   | `PRRT_kwDOGp2yqc6gYMyx` | `docs/issues/open/2175-merge-tool-symlink-exceptions/ISSUE.md` | <https://github.com/torrust/torrust-tracker/pull/2176#discussion_r3961420915> | The rules hard-code the manifest path while `--symlinks <path>` makes it overridable.      | action — parameterized in commit `d3769ecb`: the rules, background, ownership review, and AC2 now read the declaration from the path the argument names, defaulting to the documented file name. | Pending | Pending | OPEN |

## Notes

- Keep this file as an audit log of review handling for the PR.
- Reply on every PR suggestion thread before resolving it so the decision is visible to reviewers.
- Suggestion 1 evidence: no table row in the specification begins with a doubled pipe; verified by grepping row starts, which returned zero matches, and by inspecting the raw bytes of the row starts, which show one pipe followed by a space. No change was made.
- Suggestion 2 evidence: `docs/templates/ISSUE.md` line 11 defines the field as `last-updated-utc: YYYY-MM-DD HH:MM`, and the open specifications use that shape. No change was made.
- Fix commit for suggestion 6: `d3769ecb` — every read of the declaration now names the tree path `--symlinks` supplies, with the documented default when the argument is omitted; no hard-coded read of the manifest remains in the specification.
- The doubled-pipe reports in rows 1, 3, 4 and 5 all trace to the same false positive. This file previously quoted the literal in a shell command; that quotation has been reworded so the literal appears nowhere in the changed files.
