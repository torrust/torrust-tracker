---
semantic-links:
  skill-links:
    - process-copilot-suggestions
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-copilot-suggestions/SKILL.md
    - .github/workflows/upload_coverage_pr.yaml
---

# PR #2008 Copilot Suggestions Tracking

Source: Copilot PR review threads for https://github.com/torrust/torrust-tracker/pull/2008

<!-- cspell:ignore PRRT_kwDOGp2yqc6SVFOl PRRT_kwDOGp2yqc6SVFPE PRRT_kwDOGp2yqc6SWYB_ PRRT_kwDOGp2yqc6SWYCe PRRT_kwDOGp2yqc6SWYC3 SWYB -->

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
   - resolve the PR thread
4. Set `Thread State` to `resolved` once resolved in PR.

## Processing Log

- 2026-07-20: Started processing Copilot suggestions.
- 2026-07-20: Reviewed two unresolved Copilot suggestions; hardened artifact extraction and documented one false positive.
- 2026-07-20: Resolved both processed Copilot review threads in the PR.
- 2026-07-20: Started processing three newly received Copilot suggestions.
- 2026-07-20: Replied to and resolved all three newly processed Copilot review threads.

## Suggestions

| #   | Thread ID             | Path                                                            | URL                                                                         | Suggestion Summary                                                                          | Decision                                                                                                                                             | Status | Thread State |
| --- | --------------------- | --------------------------------------------------------------- | --------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------- | ------ | ------------ |
| 1   | PRRT_kwDOGp2yqc6SVFOl | `.github/workflows/upload_coverage_pr.yaml`                     | https://github.com/torrust/torrust-tracker/pull/2008#discussion_r3616316716 | Extract fork-produced artifact archives into a dedicated directory and strip archive paths. | action: use `unzip -j` in `coverage_artifacts` and upload the report from that directory; `linter yaml` passed.                                      | DONE   | RESOLVED     |
| 2   | PRRT_kwDOGp2yqc6SVFPE | `.github/workflows/upload_coverage_pr.yaml`                     | https://github.com/torrust/torrust-tracker/pull/2008#discussion_r3616316757 | Remove unsupported Codecov `working-directory` input and use `directory` instead.           | no-action: Codecov v7 documents `working-directory` as an input; retaining it ensures the uploader runs from the trusted checkout containing `.git`. | DONE   | RESOLVED     |
| 3   | PRRT*kwDOGp2yqc6SWYB* | `.github/workflows/upload_coverage_pr.yaml`                     | https://github.com/torrust/torrust-tracker/pull/2008#discussion_r3616793119 | Reject symlinked files extracted from fork-produced artifact archives.                      | action: require the three expected artifact paths to be regular files and reject symlinks before reading or uploading them.                          | DONE   | RESOLVED     |
| 4   | PRRT_kwDOGp2yqc6SWYCe | `docs/pr-reviews/pr-2008-copilot-suggestions.md`                | https://github.com/torrust/torrust-tracker/pull/2008#discussion_r3616793156 | Add all opaque thread IDs to the scoped cspell ignore directive.                            | action: added all current tracker thread IDs to the file-scoped cspell ignore directive.                                                             | DONE   | RESOLVED     |
| 5   | PRRT_kwDOGp2yqc6SWYC3 | `docs/issues/open/2006-fix-fork-pr-coverage-upload-workflow.md` | https://github.com/torrust/torrust-tracker/pull/2008#discussion_r3616793184 | Align the relevant-tests acceptance checkbox with recorded verification.                    | action: update the acceptance criterion because the pre-push suite completed successfully.                                                           | DONE   | RESOLVED     |

## Notes

- Keep this file as an audit log of review handling for the PR.
- Prefer concise decisions with explicit rationale.
- If no code changes are needed, explain why in `Decision`.
