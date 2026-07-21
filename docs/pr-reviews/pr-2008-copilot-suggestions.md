---
semantic-links:
  skill-links:
    - process-copilot-suggestions
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-copilot-suggestions/SKILL.md
    - .github/workflows/upload_coverage_pr.yaml
---

<!-- skill-link: process-copilot-suggestions -->

# PR #2008 Copilot Suggestions Tracking

Source: Copilot PR review threads for https://github.com/torrust/torrust-tracker/pull/2008

<!-- cspell:ignore PRRT_kwDOGp2yqc6SVFOl PRRT_kwDOGp2yqc6SVFPE PRRT_kwDOGp2yqc6SWYB_ PRRT_kwDOGp2yqc6SWYCe PRRT_kwDOGp2yqc6SWYC3 PRRT_kwDOGp2yqc6SX7AO PRRT_kwDOGp2yqc6SX7Ah PRRT_kwDOGp2yqc6SX7A1 PRRT_kwDOGp2yqc6SfBQ1 PRRT_kwDOGp2yqc6SfecS PRRT_kwDOGp2yqc6SgaMi PRRT_kwDOGp2yqc6SgaM_ SWYB subshell -->

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
- 2026-07-20: Started processing three newly received Copilot suggestions.
- 2026-07-21: Replied to and resolved all three newly processed Copilot review threads.
- 2026-07-21: Started processing an additional Copilot suggestion.
- 2026-07-21: Replied to and resolved the final two processed Copilot review threads.
- 2026-07-21: Started processing two newly received Copilot suggestions.

## Suggestions

| #   | Thread ID             | Path                                                            | URL                                                                         | Suggestion Summary                                                                          | Decision                                                                                                                                                  | Status | Thread State |
| --- | --------------------- | --------------------------------------------------------------- | --------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------- | ------ | ------------ |
| 1   | PRRT_kwDOGp2yqc6SVFOl | `.github/workflows/upload_coverage_pr.yaml`                     | https://github.com/torrust/torrust-tracker/pull/2008#discussion_r3616316716 | Extract fork-produced artifact archives into a dedicated directory and strip archive paths. | action: use `unzip -j` in `coverage_artifacts` and upload the report from that directory; `linter yaml` passed.                                           | DONE   | RESOLVED     |
| 2   | PRRT_kwDOGp2yqc6SVFPE | `.github/workflows/upload_coverage_pr.yaml`                     | https://github.com/torrust/torrust-tracker/pull/2008#discussion_r3616316757 | Remove unsupported Codecov `working-directory` input and use `directory` instead.           | no-action: Codecov v7 documents `working-directory` as an input; retaining it ensures the uploader runs from the trusted checkout containing `.git`.      | DONE   | RESOLVED     |
| 3   | PRRT_kwDOGp2yqc6SWYB_ | `.github/workflows/upload_coverage_pr.yaml`                     | https://github.com/torrust/torrust-tracker/pull/2008#discussion_r3616793119 | Reject symlinked files extracted from fork-produced artifact archives.                      | action: require the three expected artifact paths to be regular files and reject symlinks before reading or uploading them.                               | DONE   | RESOLVED     |
| 4   | PRRT_kwDOGp2yqc6SWYCe | `docs/pr-reviews/pr-2008-copilot-suggestions.md`                | https://github.com/torrust/torrust-tracker/pull/2008#discussion_r3616793156 | Add all opaque thread IDs to the scoped cspell ignore directive.                            | action: added all current tracker thread IDs to the file-scoped cspell ignore directive.                                                                  | DONE   | RESOLVED     |
| 5   | PRRT_kwDOGp2yqc6SWYC3 | `docs/issues/open/2006-fix-fork-pr-coverage-upload-workflow.md` | https://github.com/torrust/torrust-tracker/pull/2008#discussion_r3616793184 | Align the relevant-tests acceptance checkbox with recorded verification.                    | action: update the acceptance criterion because the pre-push suite completed successfully.                                                                | DONE   | RESOLVED     |
| 6   | PRRT_kwDOGp2yqc6SX7AO | `.github/workflows/upload_coverage_pr.yaml`                     | https://github.com/torrust/torrust-tracker/pull/2008#discussion_r3617362462 | Prevent archive-entry collisions across untrusted artifact ZIPs.                            | action: allowlist one expected entry per archive, extract each archive in an isolated temporary directory, then move that file into `coverage_artifacts`. | DONE   | RESOLVED     |
| 7   | PRRT_kwDOGp2yqc6SX7Ah | `.github/workflows/upload_coverage_pr.yaml`                     | https://github.com/torrust/torrust-tracker/pull/2008#discussion_r3617362491 | Make artifact-directory creation idempotent.                                                | action: create `coverage_artifacts` with `mkdir -p`.                                                                                                      | DONE   | RESOLVED     |
| 8   | PRRT_kwDOGp2yqc6SX7A1 | `docs/pr-reviews/pr-2008-copilot-suggestions.md`                | https://github.com/torrust/torrust-tracker/pull/2008#discussion_r3617362517 | Restore the exact opaque thread ID in row 3.                                                | action: restored `PRRT_kwDOGp2yqc6SWYB_` and retained it in the scoped cspell ignore directive.                                                           | DONE   | RESOLVED     |
| 9   | PRRT_kwDOGp2yqc6SfBQ1 | `docs/pr-reviews/pr-2008-copilot-suggestions.md`                | https://github.com/torrust/torrust-tracker/pull/2008#discussion_r3620043755 | Add the tracker skill-link marker.                                                          | action: added `<!-- skill-link: process-copilot-suggestions -->` for the governing review workflow.                                                       | DONE   | RESOLVED     |
| 10  | PRRT_kwDOGp2yqc6SfecS | `.github/workflows/upload_coverage_pr.yaml`                     | https://github.com/torrust/torrust-tracker/pull/2008#discussion_r3620210121 | Validate untrusted artifact metadata before writing step outputs.                           | action: require a numeric PR number and 40-character hexadecimal SHA before emitting Codecov metadata outputs.                                            | DONE   | RESOLVED     |
| 11  | PRRT_kwDOGp2yqc6SgaMi | `docs/pr-reviews/pr-2008-copilot-suggestions.md`                | https://github.com/torrust/torrust-tracker/pull/2008#discussion_r3620551890 | Remove the duplicate processing-log entry.                                                  | action: removed the repeated event so each review-batch milestone appears once.                                                                            | DONE   | OPEN         |
| 12  | PRRT_kwDOGp2yqc6SgaM_ | `.github/workflows/upload_coverage_pr.yaml`                     | https://github.com/torrust/torrust-tracker/pull/2008#discussion_r3620551932 | Clean temporary extraction directories on all paths.                                        | action: run extraction in a subshell with an `EXIT` trap that removes its temporary directory.                                                             | DONE   | OPEN         |

## Notes

- Keep this file as an audit log of review handling for the PR.
- Prefer concise decisions with explicit rationale.
- If no code changes are needed, explain why in `Decision`.
