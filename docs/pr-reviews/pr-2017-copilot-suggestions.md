---
semantic-links:
  skill-links:
    - process-copilot-suggestions
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-copilot-suggestions/SKILL.md
---

<!-- cspell:disable -->

<!-- skill-link: process-copilot-suggestions -->

# PR #2017 Copilot Suggestions Tracking

Source: Copilot PR review threads for https://github.com/torrust/torrust-tracker/pull/2017

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

- 2026-07-21: Started processing suggestions (9 threads across 2 pushes).
- 2026-07-21: Completed processing all suggestions. All 9 threads resolved.

## Suggestions

| #   | Thread ID             | Path                                        | URL                                                                         | Suggestion Summary                                                                       | Decision | Reply URL                                                                   | Status | Thread State |
| --- | --------------------- | ------------------------------------------- | --------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------- | -------- | --------------------------------------------------------------------------- | ------ | ------------ |
| 1   | PRRT_kwDOGp2yqc6Sq55a | project-words.txt                           | https://github.com/torrust/torrust-tracker/pull/2017#discussion_r3624391767 | `dport` placed before `datagram`; breaks alphabetical order                              | action   | https://github.com/torrust/torrust-tracker/pull/2017#discussion_r3624598630 | DONE   | RESOLVED     |
| 2   | PRRT_kwDOGp2yqc6Sq55o | project-words.txt                           | https://github.com/torrust/torrust-tracker/pull/2017#discussion_r3624391788 | `HDRINCL` placed after `Hydranode`; should be after `hasher`                             | action   | https://github.com/torrust/torrust-tracker/pull/2017#discussion_r3624605903 | DONE   | RESOLVED     |
| 3   | PRRT_kwDOGp2yqc6Sq56B | project-words.txt                           | https://github.com/torrust/torrust-tracker/pull/2017#discussion_r3624391826 | `middlewares` before `middlebox`; b < w                                                  | action   | https://github.com/torrust/torrust-tracker/pull/2017#discussion_r3624636696 | DONE   | RESOLVED     |
| 4   | PRRT_kwDOGp2yqc6Sq56P | project-words.txt                           | https://github.com/torrust/torrust-tracker/pull/2017#discussion_r3624391850 | `sendto` before `savepath`; should be after `Seedable`                                   | action   | https://github.com/torrust/torrust-tracker/pull/2017#discussion_r3624638336 | DONE   | RESOLVED     |
| 5   | PRRT_kwDOGp2yqc6Sq56p | packages/udp-server/src/server/processor.rs | https://github.com/torrust/torrust-tracker/pull/2017#discussion_r3624391885 | Fixed `sleep(50ms)` can be flaky; use bounded wait                                       | action   | https://github.com/torrust/torrust-tracker/pull/2017#discussion_r3624647118 | DONE   | RESOLVED     |
| 6   | PRRT_kwDOGp2yqc6Sq561 | packages/udp-server/src/server/processor.rs | https://github.com/torrust/torrust-tracker/pull/2017#discussion_r3624391900 | Assertion message misleading; received counter always 0 in unit test (launcher bypassed) | action   | https://github.com/torrust/torrust-tracker/pull/2017#discussion_r3624659670 | DONE   | RESOLVED     |
| 7   | PRRT_kwDOGp2yqc6SrCCM | project-words.txt                           | https://github.com/torrust/torrust-tracker/pull/2017#discussion_r3624438710 | `recvfrom` before `recognised`; outdated thread but issue persisted                      | action   | https://github.com/torrust/torrust-tracker/pull/2017#discussion_r3624698072 | DONE   | RESOLVED     |
| 8   | PRRT_kwDOGp2yqc6Srh5b | project-words.txt                           | https://github.com/torrust/torrust-tracker/pull/2017#discussion_r3624629672 | `nmap`/`nping` before `new*` words; e < m < p                                            | action   | https://github.com/torrust/torrust-tracker/pull/2017#discussion_r3624711012 | DONE   | RESOLVED     |
| 9   | PRRT_kwDOGp2yqc6Srh5u | packages/udp-server/src/server/processor.rs | https://github.com/torrust/torrust-tracker/pull/2017#discussion_r3624629708 | Doc comment says port 0 "invalid"; RFC 768 permits it; real issue is OS EINVAL           | action   | https://github.com/torrust/torrust-tracker/pull/2017#discussion_r3624712519 | DONE   | RESOLVED     |

## Notes

- Keep this file as an audit log of review handling for the PR.
- Prefer concise decisions with explicit rationale.
- If no code changes are needed, explain why in `Decision`.
- Reply on every PR suggestion thread before resolving it so the decision is visible to reviewers.
