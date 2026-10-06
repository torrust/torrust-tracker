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

### 2026-10-05 21:07 UTC - Task Reviewer

- Invocation scope: read-only re-review of the full branch diff against `torrust/develop` (21 commits, clean tree), focused on the remediation commits `test: [#2417] align scrape limit tests with their prose-first specification` and `docs(issues): [#2417] record the prose-first test review and the first Task Reviewer report`; resolution of the 18:46 findings, the prose-first record, AC1-AC4, spec consistency, and commit-id hygiene. Persisted here by the implementer.
- Inputs: `ISSUE.md`, `prose-first-test-review.md`, this file, `manual-verification-evidence.md`, `implementation-retrospective.md`, the `write-unit-test` prose-first procedure, the `review-task` Test Design checklist, and the final code of all 21 tests, the fake UDP tracker, and `ServerThread`.
- Evidence:
  - The remediation diff matches its claims (`parse_scrape`; H4 asserts `InvalidInfoHashParam`; U1/U2 encode in Arrange; W1 context comment; K1 ADR link); no production code changed.
  - All 21 tests were compared with their review rows: Arrange, Act, and Assert match; expected values are literal and independent of production code; every production Act is visible; no parameter-bag fixtures.
  - The timing exception is recorded and justified.
  - All 18:46 findings are resolved, except the optional `MAX_INFO_HASHES_PER_QUERY` comment and the PR-body commit-scope note, deferred as stated.
  - AC1-AC4 still pass; the linter/tests criterion and the final checkpoint are correctly left unchecked.
  - The only commit id in the issue folder is a `develop` ancestor.
  - Focused tests passed: `http-protocol --lib limiting` (5), `udp-protocol --lib limiting` (3), `udp-server --lib first_74` (1), `tracker-client --tests` (lib 50, `tracker_checker` 11, `tracker_client` 7).
- Findings:
  - MINOR: the retrospective does not record the late prose-first comparison as a lesson, and calls the fake-tracker tests "deterministic", conflicting with the timing exception.
  - NIT: the review intro omits the kept per-test doc comments (S1, W1, US1, C1, K1).
  - NIT: the timeout-path monitor row omits the URL and total-count assertions.
  - NIT: the timing exception does not record the repeated-run command.
- Verdict: REVIEW PASSED
- Follow-up actions:
  - Optionally fix the MINOR and NITs in a focused `docs(issues)` commit.
  - Run pre-push; tick the linter/tests criterion and the final checkpoint only after it passes.
  - Mention in the PR body that `fix(http-protocol): [#2417] cap scrape info hashes per protocol` also changes `udp-protocol`, `udp-server`, and `tracker-core`, including removing the public `MAX_SCRAPE_TORRENTS` constant.

### 2026-10-06 06:14 UTC - Implementation Follow-up

- Invocation scope: the optional findings of the 21:07 Task Reviewer report.
- Inputs: the report above.
- Evidence: the retrospective records the late prose-first comparison as lesson 4 and no longer calls the timing-based tests deterministic; `prose-first-test-review.md` lists the kept per-test doc comments, adds the URL and total-count assertions to the timeout-path row, and records the repeated-run command. The PR body will carry the commit-scope note.
- Findings:
  - None.
- Verdict: REVIEW PASSED
- Follow-up actions:
  - Run pre-push, then tick the linter/tests criterion.
