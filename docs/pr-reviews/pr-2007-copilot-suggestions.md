---
semantic-links:
  skill-links:
    - process-copilot-suggestions
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-copilot-suggestions/SKILL.md
---

<!-- cspell:disable -->
<!-- skill-link: process-copilot-suggestions -->

# PR #2007 Copilot Suggestions Tracking

Source: Copilot PR review threads for https://github.com/torrust/torrust-tracker/pull/2007

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

- 2026-07-20: Started processing suggestions.
- 2026-07-20: Completed processing suggestions (batch 1 — YAML + README).
- 2026-07-20: Completed processing suggestions (batch 2 — APT cache cleanup).
- 2026-07-20: Completed processing suggestions (batch 3 — cargo-nextest pinning, cspell, security README).
- 2026-07-21: Completed processing suggestions (batch 4 — broken link no-action, GCC casing fix); added explanatory replies to all batch 1–2 threads.

## Suggestions

| #   | Thread ID               | Path                                                                      | URL                                                                                    | Suggestion Summary                                                                                                                | Decision  | Status | Thread State |
| --- | ----------------------- | ------------------------------------------------------------------------- | -------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------- | --------- | ------ | ------------ |
| 1   | `PRRT_kwDOGp2yqc6SUumy` | `.github/skills/dev/maintenance/run-manual-docker-security-scan/SKILL.md` | [comment](https://github.com/torrust/torrust-tracker/pull/2007#discussion_r3616188738) | YAML frontmatter `related-artifacts` list is malformed: `docs/security/analysis/build/` is not indented under `related-artifacts` | action    | DONE   | resolved     |
| 2   | `PRRT_kwDOGp2yqc6SUunN` | `docs/security/analysis/README.md`                                        | [comment](https://github.com/torrust/torrust-tracker/pull/2007#discussion_r3616188775) | README describes `review-date` but actual CVE docs use `date-analyzed`                                                            | action    | DONE   | resolved     |
| 3   | `PRRT_kwDOGp2yqc6SWeDs` | `Containerfile` (chef stage)                                              | [comment](https://github.com/torrust/torrust-tracker/pull/2007#discussion_r3616827507) | Missing `apt-get clean` in chef stage APT layer — .deb archives remain in the image                                               | action    | DONE   | resolved     |
| 4   | `PRRT_kwDOGp2yqc6SWeED` | `Containerfile` (tester stage)                                            | [comment](https://github.com/torrust/torrust-tracker/pull/2007#discussion_r3616827539) | Missing `apt-get clean` in tester stage APT layer — .deb archives remain                                                          | action    | DONE   | resolved     |
| 5   | `PRRT_kwDOGp2yqc6SWeEV` | `Containerfile` (gcc stage)                                               | [comment](https://github.com/torrust/torrust-tracker/pull/2007#discussion_r3616827560) | Missing `apt-get clean` in gcc stage APT layer — .deb archives remain                                                             | action    | DONE   | resolved     |
| 6   | `PRRT_kwDOGp2yqc6SW8iK` | `Containerfile` (chef stage)                                              | [comment](https://github.com/torrust/torrust-tracker/pull/2007#discussion_r3616827507) | `cargo-nextest` installed without pinned version — non-reproducible build                                                         | action    | DONE   | resolved     |
| 7   | `PRRT_kwDOGp2yqc6SW8in` | `project-words.txt`                                                       | [comment](https://github.com/torrust/torrust-tracker/pull/2007#discussion_r3616827507) | `Uumy` is an opaque thread ID fragment, not a stable project term — pollutes dictionary                                           | action    | DONE   | resolved     |
| 8   | `PRRT_kwDOGp2yqc6SW8jK` | `docs/pr-reviews/pr-2007-copilot-suggestions.md`                          | [comment](https://github.com/torrust/torrust-tracker/pull/2007#discussion_r3616827507) | Tracker file should use `<!-- cspell:disable -->` instead of adding ID fragments to global dictionary                             | action    | DONE   | resolved     |
| 9   | `PRRT_kwDOGp2yqc6SW8jn` | `docs/security/README.md`                                                 | [comment](https://github.com/torrust/torrust-tracker/pull/2007#discussion_r3616827507) | Security overview still lists old build-stage base images (`rust:trixie`, `gcc:trixie`)                                           | action    | DONE   | resolved     |
| 10  | `PRRT_kwDOGp2yqc6SW8j-` | `Containerfile` (tester stage)                                            | [comment](https://github.com/torrust/torrust-tracker/pull/2007#discussion_r3616827507) | `cargo-nextest` in tester stage also unpinned — non-reproducible test execution                                                   | action    | DONE   | resolved     |
| 11  | `PRRT_kwDOGp2yqc6SX-aD` | `docs/security/docker/scans/build-images.md`                              | [comment](https://github.com/torrust/torrust-tracker/pull/2007#discussion_r3616827507) | Link targets `non-affecting/` path which does not exist after catalog reorganization                                              | no-action | DONE   | resolved     |
| 12  | `PRRT_kwDOGp2yqc6SX-aS` | `docs/security/docker/scans/README.md`                                    | [comment](https://github.com/torrust/torrust-tracker/pull/2007#discussion_r3616827507) | Stage column uses uppercase `GCC` — inconsistent with `gcc` in Containerfile and scan report                                      | action    | DONE   | resolved     |

## Notes

- Keep this file as an audit log of review handling for the PR.
- Prefer concise decisions with explicit rationale.
- If no code changes are needed, explain why in `Decision`.
