---
semantic-links:
  skill-links:
    - process-copilot-suggestions
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-copilot-suggestions/SKILL.md
---

<!-- cspell:disable -->

<!-- skill-link: process-copilot-suggestions -->

# PR #2184 Copilot Suggestions Tracking

Source: Copilot PR review threads for <https://github.com/torrust/torrust-tracker/pull/2184>

Status legend:

- `action`: code/docs change applied
- `no-action`: suggestion reviewed; no code change needed
- `resolved`: thread resolved in PR

## Processing Log

- 2026-09-09 13:45 UTC: Started processing suggestions (three unresolved threads, all on `contrib/dev-tools/git/github-merge.py`).
- 2026-09-09 14:04 UTC: Accepted thread 3 and escaped every path, target, and reason the symbolic-link report prints, in commit `a0b42f36`.
- 2026-09-09 14:06 UTC: Accepted thread 2 and matched declared links against tree bytes in commit `a4e6a026`, which also removed a second decoding defect the same evidence exposed on the path side.
- 2026-09-09 14:06 UTC: Accepted thread 1 in part and reused link contents across the range in commit `ba60e693`, after measuring what the suggested batch reader would add on top of it.
- 2026-09-09 14:07 UTC: Stated the declaration path as a repository convention in the vendoring README, the merge skill, and the specification, in commit `a0f69c75`.
- 2026-09-09 14:12 UTC: Gated the four commits on the build server: the symbolic-link suite (23 tests), the wrapper suite, `linter all`, and the pre-commit script all pass, each commit green on its own.
- 2026-09-09 14:15 UTC: Pushed the four commits and replied on each thread with its fix commit and validation, resolving each immediately after its reply.
- 2026-09-09 14:17 UTC: Re-fetched the pull request's review threads and confirmed all three are resolved and none remains open.

## Suggestions

| #   | Thread ID               | Path                                     | URL                                                                           | Suggestion Summary                                                                                                                    | Decision                                                                                                                                                                                                                                                                                                                     | Reply URL | Status | Thread State |
| --- | ----------------------- | ---------------------------------------- | ----------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | --------- | ------ | ------------ |
| 1   | `PRRT_kwDOGp2yqc6gnX_H` | `contrib/dev-tools/git/github-merge.py` | <https://github.com/torrust/torrust-tracker/pull/2184#discussion_r3967484438> | The range walk runs one `git ls-tree` per checked commit plus one `git cat-file` per symbolic-link entry; consider a batch blob reader. | action — commit `ba60e693` reads each distinct link content once across the whole range instead of once per commit that carries it, which is where the cost grew. Over a fifty-commit range that is 262 ms to 143 ms with one link per commit and 2778 ms to 188 ms with twenty. The suggested batch reader was measured and declined: it saves at most a further 8 ms per hundred reads against an irreducible 357 ms of tree listing. | <https://github.com/torrust/torrust-tracker/pull/2184#discussion_r3969468318> | DONE | RESOLVED |
| 2   | `PRRT_kwDOGp2yqc6gnX_l` | `contrib/dev-tools/git/github-merge.py` | <https://github.com/torrust/torrust-tracker/pull/2184#discussion_r3967484484> | The rules require a byte-exact target match while the implementation decodes the blob with `errors='replace'`, collapsing distinct byte sequences. | action — the report is correct, and commit `a4e6a026` matches paths and targets against the bytes the tree carries. A target that is not valid UTF-8 now has no declaration it can equal, and the same evidence exposed a second decoding defect on the path side: the listing was git's rendering of the path, so a declaration naming a path outside ASCII could not match the link it describes.                     | <https://github.com/torrust/torrust-tracker/pull/2184#discussion_r3969465246> | DONE | RESOLVED |
| 3   | `PRRT_kwDOGp2yqc6gnX_-` | `contrib/dev-tools/git/github-merge.py` | <https://github.com/torrust/torrust-tracker/pull/2184#discussion_r3967484517> | `path` and `target` come from tree content and are interpolated into messages, so control characters can forge misleading output.       | action — commit `a0b42f36` quotes and escapes every path and target the report prints and sanitizes the declaration's reason, which is tree content on the same footing. Ordinary values render exactly as before, so the documented message shapes are unchanged, and the escaping never reaches the comparison.                     | <https://github.com/torrust/torrust-tracker/pull/2184#discussion_r3969466844> | DONE | RESOLVED |

## Notes

- Keep this file as an audit log of review handling for the PR.
- Reply on every PR suggestion thread before resolving it so the decision is visible to reviewers.
- Suggestion 1 evidence: measured on the gate host with git 2.43.0, best of three runs of the check itself over a synthetic fifty-commit range. Listing one tree per commit costs 357 ms for fifty commits and cannot be removed, because whether a commit carries a link is a property of its own tree. Fifty separate blob reads cost 197 ms, of which 139 ms is bare process startup; the same fifty through one long-lived batch reader cost 8 ms. Reusing contents already read collapses the reads to one per distinct content, which is what the multiplicative term was, and leaves a remainder the batch reader would improve by less than the measurement noise of the tree listing.
- Suggestion 2 evidence: two bytes that are not valid UTF-8 decode to two replacement characters under a replacing decoder, so a declaration naming those two characters admitted the link. `it_should_refuse_a_link_whose_target_is_not_valid_utf8` fixes that reading in place. `it_should_accept_a_declared_target_outside_ascii` and `it_should_accept_a_declared_path_outside_ascii` cover the values that must still match.
- Suggestion 3 evidence: `it_should_escape_a_control_character_in_an_accepted_target` and `it_should_escape_a_control_character_in_a_declaration_reason` each declare a value carrying a newline followed by the text of a refusal, and assert that no such line appears in the report.
- Validation for all three: the symbolic-link suite (23 tests) and the wrapper suite pass, `linter all` exits 0, and the pre-commit script passes all six steps, each of the four commits green on its own.
- All three threads were replied to before being resolved, one at a time, and a re-fetch of the pull request's review threads after the round reports three threads and none unresolved.
