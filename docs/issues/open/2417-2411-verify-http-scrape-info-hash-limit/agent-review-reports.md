---
semantic-links:
  related-artifacts:
    - .github/agents/task-reviewer.agent.md
    - docs/agents/orchestration.md
---

# Agent Review Reports - Issue #2417 HTTP Scrape Info-Hash Limit

> Append one completed independent-review entry at a time. Do not modify, reorder, or remove
> earlier entries. A correction is a new entry that names the earlier conclusion.

## Reports

### 2026-10-05 18:46 UTC - Task Reviewer

- Invocation scope: independent pre-PR review of the branch diff against `torrust/develop` (16 signed commits, clean tree), all acceptance criteria, the test matrix, repository conventions, and spec consistency. Read-only; persisted here by the implementer, with PR-branch commit ids replaced by commit subjects.
- Inputs: `ISSUE.md`, `manual-verification-evidence.md`, `implementation-retrospective.md`, ADR `20261005124222`, the ADR index, the global CLI output contract ADR, the `review-task` Test Design checklist, and all changed code, test, and doc files.
- Evidence:
  - Focused tests passed: `http-protocol --lib scrape` (11), `udp-protocol --lib` (12), `tracker-core --lib scrape_handler` (7), `udp-server` (212 unit, 12 integration; W1 and US1 confirmed with `--list`), `axum-http-server --test integration receiving_an_scrape_request` (18, S1 confirmed), `tracker-client` (lib 50, `tracker_checker` 11, `tracker_client` 7).
  - `cargo clippy` for `tracker-client` with pedantic warnings, and nightly rustdoc with `-D warnings` for the four changed protocol and server crates, were clean.
  - `MAX_SCRAPE_TORRENTS` remains only in the ADR, this spec, and historical reports; `1..=74` only in the frozen legacy client.
  - The only commit id in the issue folder is a `develop` ancestor.
  - No recorded prose-first Arrange-Act-Assert comparison was found.
  - AC1, AC2, AC3, and AC4 pass; the linter/tests criterion is correctly pending. The stderr warning meets the global CLI output contract. V3 and V4 ran against the final production code (later commits touch only tests and docs).
- Findings:
  - BLOCKER: none of the 21 changed tests (H1-H5, S1, U1-U3, W1, US1, C1, K1-K8, and the two monitor tests) has a recorded prose-first Arrange-Act-Assert comparison, which the issue plans and the Test Design checklist requires.
  - MINOR: the fake trackers and monitor tests rely on polling and wall-clock timing; record it as an accepted exception with its reason.
  - MINOR: K1's doc comment names the reason but does not link the ADR.
  - MINOR: the AC1 evidence cell says "final implementation acceptance remains pending" despite status DONE.
  - MINOR: the "automatic checks" workflow checkpoint is ticked while the linter/tests criterion is pending.
  - NIT: the T8 row gives the fakes' old location; the M2 row does not mention the V4 rerun; the evidence file's `last-updated-utc` is stale.
  - NIT: the commit `fix(http-protocol): [#2417] cap scrape info hashes per protocol` also changes `udp-protocol`, `udp-server`, and `tracker-core`, including the public constant removal; mention this in the PR body, since history is not rewritten.
  - NIT (optional): the `MAX_INFO_HASHES_PER_QUERY` doc comment mentions only UDP's 74.
- Verdict: REVIEW FAILED
- Follow-up actions:
  - Record the prose-first comparison for every changed test, with the timing exception, and request a re-review.
  - Apply the MINOR and NIT spec fixes and the K1 ADR link.
  - Run pre-commit and pre-push; tick the linter/tests criterion only after both pass.

### 2026-10-05 21:05 UTC - Implementation Follow-up

- Invocation scope: remediation of the 18:46 Task Reviewer report.
- Inputs: the report above, the `write-unit-test` prose-first procedure, and the changed tests.
- Evidence:
  - The comparison is recorded in [prose-first-test-review.md](prose-first-test-review.md), with the accepted timing exception.
  - It required the commit `test: [#2417] align scrape limit tests with their prose-first specification`: H4 asserts the specific invalid `info_hash` error; U1 and U2 encode in Arrange; W1 explains the disabled validation; K1 links the ADR; the H parse helper is `parse_scrape`. Focused tests and clippy pass.
  - Spec fixes: AC1 evidence cell, checkpoint reworded to "focused checks", T8 and M2 rows, evidence `last-updated-utc`.
  - Not changed: the optional `MAX_INFO_HASHES_PER_QUERY` comment (still accurate). The commit-scope NIT goes in the PR body.
- Findings:
  - Pending independent re-review.
- Verdict: REVIEW PENDING
- Follow-up actions:
  - Request a Task Reviewer re-review, then run pre-push.
