---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #3084 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/3084.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A

## Status Values

- Relationship: `ORIGINAL`, `RE_RAISE_OF:<FindingId>`
- Disposition: `FIXED`, `NO_ACTION`, `SUPERSEDED`, `FOLLOW_UP`
- Thread state: `OPEN`, `RESOLVED`, `NON_RESOLVABLE`, `SUPERSEDED`. `OPEN` applies prospectively
  to audits created or updated for approved follow-up work; historical audits remain valid without
  bulk migration.
- Severity: `Blocker`, `Major`, `Minor`, `Nit`, `Suggestion`; append `(inferred)` when derived
  from free prose.
- Reviewer: the author's GitHub login, for example `da2ce7` or `coderabbitai[bot]`; `unknown`
  when GitHub shows no author
- Category: `link-integrity`, `formatting`, `metadata`, `testing`, `correctness`,
  `documentation`, `maintainability`, `security`, `other`
- An outdated thread whose concern was fixed is `FIXED`/`RESOLVED`, even when GitHub marks the
  original thread outdated after the push. For in-PR feedback, use `NO_ACTION`/`SUPERSEDED` only
  for a duplicate, superseded, or no-change concern. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work.
- When the current identity cannot resolve a thread (`viewerCanResolve` is `false`), keep the
  disposition established from the current tree and record `Thread state=OPEN`. The reviewer or PR
  author resolves it; permission says nothing about whether the concern was fixed.

## Findings

Review 6087950878 (`claude[bot]`) has no finding: it notes that automated review is disabled for a pull request from a fork. Issue comment 7217996387 (`coderabbitai[bot]`) is a walkthrough and pre-merge check summary with no new finding. PR conversation comment 7233148375 (by the PR author) summarized the follow-up commit and did not replace per-thread replies (addressed in `F11`). Review 6096915465 (`coderabbitai[bot]`) posted a reply (`discussion_r4726346283`) on the `F1` thread asking for explicit T2 gate ordering; that follow-up reply is logged below and awaits maintainer disposition.

| Finding ID | Review finding reference | Reviewer | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | -------- | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-3084-f1 | coderabbitai[bot] | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-3084-f2 | da2ce7 | Minor | documentation | ORIGINAL | FIXED | OPEN |
| F3 | review-finding:pr-3084-f3 | da2ce7 | Minor | documentation | ORIGINAL | FIXED | OPEN |
| F4 | review-finding:pr-3084-f4 | da2ce7 | Minor | testing | ORIGINAL | FIXED | OPEN |
| F5 | review-finding:pr-3084-f5 | da2ce7 | Minor | documentation | ORIGINAL | FIXED | OPEN |
| F6 | review-finding:pr-3084-f6 | da2ce7 | Minor | formatting | ORIGINAL | FIXED | OPEN |
| F7 | review-finding:pr-3084-f7 | da2ce7 | Nit | correctness | ORIGINAL | FIXED | OPEN |
| F8 | review-finding:pr-3084-f8 | da2ce7 | Nit | testing | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-3084-f9 | da2ce7 | Nit | metadata | ORIGINAL | FIXED | OPEN |
| F10 | review-finding:pr-3084-f10 | da2ce7 | Suggestion | maintainability | ORIGINAL | FOLLOW_UP | OPEN |
| F11 | review-finding:pr-3084-f11 | da2ce7 | Minor | documentation | ORIGINAL | FIXED | OPEN |
| F12 | review-finding:pr-3084-f12 | da2ce7 | Minor | documentation | ORIGINAL | FIXED | OPEN |
| F13 | review-finding:pr-3084-f13 | da2ce7 | Nit | other | ORIGINAL | FIXED | OPEN |
| F14 | review-finding:pr-3084-f14 | da2ce7 | Nit | documentation | ORIGINAL | FIXED | OPEN |
| F15 | review-finding:pr-3084-f15 | da2ce7 | Nit | documentation | ORIGINAL | FIXED | OPEN |

## Finding Details

### F1 - Add required test-development review gates to T2

- PR number: 3084
- Source review ID: 6088001515
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3084#discussion_r4719271659
- Concern: T2 in `ISSUE.md` defined the regression test and red run but did not state the required `write-unit-test` loop: focused validation followed by the design review (causal-state, fixture-ownership, and visible Act/expected-result checks) and stopping for maintainer review after the final increment before final verification, commit, or PR.
- Solution: Added the `write-unit-test` design review notes to T2 in `ISSUE.md` in `fix(axum-server): [#3080] declare request_id field on request spans`, refined T2 and `Commit Points` in `fix(axum-server): [#3080] address PR #3084 review findings` so the completed T2 row states what ran while keeping the maintainer review pending in `Workflow Checkpoints` and `Commit Points` (also addressing F5), and replied directly on the thread. Note: CodeRabbit later replied (`discussion_r4726346283`, review `6096915465`) asking to state the gate order explicitly in T2.
- Current-tree verification: `docs/issues/open/3080-record-request-ids-on-axum-request-spans/ISSUE.md` lists the `write-unit-test` design review checks on T2 and the pending maintainer-review checkpoint in `Workflow Checkpoints` and `Commit Points`; `frontmatter-validator` validates `ISSUE.md`.
- Resolution reference: fix(axum-server): [#3080] declare request_id field on request spans; fix(axum-server): [#3080] address PR #3084 review findings
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3084#discussion_r4726342364

### F2 - Create the tracked PR review audit record for PR #3084

- PR number: 3084
- Source review ID: 6091805883
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3084#discussion_r4722394419
- Concern: The CodeRabbit thread on `ISSUE.md:87` (F1) was resolved without creating `docs/pr-reviews/pr-3084-review/PR-REVIEW.md` as required by `process-pr-review`.
- Solution: Created `docs/pr-reviews/pr-3084-review/PR-REVIEW.md` from `docs/templates/PR-REVIEW-TEMPLATE.md` covering `F1` to `F10` in `fix(axum-server): [#3080] address PR #3084 review findings`, then updated it in `docs(pr-3084): [#3080] record review replies and follow-up issue` with actual GitHub `Reply URL` links, issue `#3087`, and second-round findings `F11` to `F15`, and replied directly on the thread.
- Current-tree verification: `docs/pr-reviews/pr-3084-review/PR-REVIEW.md` exists with all 15 rows (`F1` to `F15`), passes `frontmatter-validator`, and passes all `validate-audit-record.py` row, detail, log-order, comment-matching, `Reply URL`, and `Resolution reference` checks against the captured PR #3084 GitHub REST data once `docs(pr-3084): [#3080] record review replies and follow-up issue` is committed.
- Resolution reference: fix(axum-server): [#3080] address PR #3084 review findings; docs(pr-3084): [#3080] record review replies and follow-up issue
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3084#discussion_r4726344459

### F3 - Sync commit subjects in evidence and Commit Points with the branch

- PR number: 3084
- Source review ID: 6091805883
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3084#discussion_r4722394429
- Concern: `manual-verification-evidence.md` cited two commit subjects absent from the branch (`test(axum-server): [#3080] add regression test for request_id on request span` and `fix(axum-server): [#3080] declare empty request_id field on RedactedMakeSpan`), while `ISSUE.md` still planned separate T2/T3/T4 commits in `Commit Points`.
- Solution: Updated `manual-verification-evidence.md` (`Environment and Prerequisites`, `T3`, and `V2`) to cite the actual branch commit `fix(axum-server): [#3080] declare request_id field on request spans` (noting that the original T3/V2 runs executed on the uncommitted tree whose production diff matches that commit) and the review follow-up commit `fix(axum-server): [#3080] address PR #3084 review findings`, updated `Commit Points` in `ISSUE.md` to match, and replied on the thread.
- Current-tree verification: Neither of the two non-existent commit subjects appears in `manual-verification-evidence.md` or `ISSUE.md`; both documents reference `fix(axum-server): [#3080] declare request_id field on request spans` and `fix(axum-server): [#3080] address PR #3084 review findings`.
- Resolution reference: fix(axum-server): [#3080] address PR #3084 review findings
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3084#discussion_r4726345210

### F4 - Re-record T2 red run on the final regression test form

- PR number: 3084
- Source review ID: 6091805883
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3084#discussion_r4722394431
- Concern: The recorded T2 failure showed an earlier rewrite (`request_log.rs:304:9`) rather than a red run of the final regression test.
- Solution: After removing the unused header in `it_should_record_the_request_id_in_the_span_of_a_request`, executed the `write-unit-test` mutate-then-restore cycle in Docker `rust:1-bookworm`: temporarily removed `use tracing::field::Empty;` and `request_id = Empty,`, ran `cargo test -p torrust-tracker-axum-server --lib request_log::tests` (`38 passed; 1 failed`, panicking at `packages/axum-server/src/request_log.rs:302:9`, matching final `assert!` line `304:9` once the two fix lines are restored), restored the fix, recorded the verbatim failure in `manual-verification-evidence.md`, and replied on the thread.
- Current-tree verification: `manual-verification-evidence.md` section `T2` records the `2026-11-28T03:13:52Z` to `03:13:59Z` mutate-then-restore failure; after restoring the fix, `cargo test -p torrust-tracker-axum-server` passes all 57 tests (`48` unit + `9` integration).
- Resolution reference: fix(axum-server): [#3080] address PR #3084 review findings
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3084#discussion_r4726346553

### F5 - Keep only completed work in DONE T2 row and leave maintainer review pending

- PR number: 3084
- Source review ID: 6091805883
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3084#discussion_r4722394436
- Concern: `T2` had status `DONE` in `ISSUE.md`, yet its notes ended with imperative text to stop for maintainer review before final verification, commit, or PR even though that checkpoint remained unchecked and PR #3084 was already open.
- Solution: Updated the `T2` `DONE` row in `ISSUE.md` to describe only what was completed (regression test following `write-unit-test`, red run, and prose-first Arrange-Act-Assert design review record in `manual-verification-evidence.md`), kept the pending maintainer review explicit in `Workflow Checkpoints` and `Commit Points`, and replied on the thread.
- Current-tree verification: `ISSUE.md` `T2` row describes only completed work and notes that pending maintainer review stays under `Workflow Checkpoints` (unchecked) and `Commit Points`.
- Resolution reference: fix(axum-server): [#3080] address PR #3084 review findings
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3084#discussion_r4726349037

### F6 - Replace local checkout path failing cspell spellcheck

- PR number: 3084
- Source review ID: 6091805883
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3084#discussion_r4722394443
- Concern: `manual-verification-evidence.md:154` contained the local host path with an unrecognized word, causing `cspell` to fail the `Docs Lint` and `Unit (stable)` CI jobs.
- Solution: Replaced the local host path in the `docker run` snippet in `manual-verification-evidence.md` with `"<checkout>:/workspace"` and replied on the thread.
- Current-tree verification: `git grep -i` confirms zero occurrences of the unrecognized local directory name across `docs/`, `packages/`, and `.claude/`.
- Resolution reference: fix(axum-server): [#3080] address PR #3084 review findings
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3084#discussion_r4726350352

### F7 - Fix test total sum in T3 conclusion (449, not 448)

- PR number: 3084
- Source review ID: 6091805883
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3084#discussion_r4722394449
- Concern: The per-crate test counts in `manual-verification-evidence.md` (`48 + 9 + 13 + 13 + 1 + 53 + 77 + 1 + 170 + 63 + 1`) sum to `449`, not `448`.
- Solution: Corrected the T3 total count in `manual-verification-evidence.md` from `448` to `449`, documented the follow-up `torrust-tracker-axum-server` re-run (`57` tests), and replied on the thread.
- Current-tree verification: `manual-verification-evidence.md` states `449` and `448` no longer appears in the T3 conclusion.
- Resolution reference: fix(axum-server): [#3080] address PR #3084 review findings
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3084#discussion_r4726351590

### F8 - Remove unused x-request-id header from regression test Arrange

- PR number: 3084
- Source review ID: 6091805883
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3084#discussion_r4722394454
- Concern: `it_should_record_the_request_id_in_the_span_of_a_request` in `packages/axum-server/src/request_log.rs` set `.header("x-request-id", request_id)` even though `RedactedMakeSpan::make_span` reads no headers, violating `write-unit-test`'s causal initial-state rule.
- Solution: Removed `.header("x-request-id", request_id)` from `Request::builder()` in `packages/axum-server/src/request_log.rs`, updated the matching `Prose-first Arrange-Act-Assert Record` in `manual-verification-evidence.md`, replied on the thread, and the thread was resolved on GitHub.
- Current-tree verification: `cargo test -p torrust-tracker-axum-server`, `cargo clippy -p torrust-tracker-axum-server --all-targets --all-features -- -D warnings`, and `cargo fmt --all -- --check` pass after the edit in Docker `rust:1-bookworm`, and the GitHub thread is resolved.
- Resolution reference: fix(axum-server): [#3080] address PR #3084 review findings
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3084#discussion_r4726353151

### F9 - Update ISSUE.md frontmatter related-pr and status

- PR number: 3084
- Source review ID: 6091805883
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3084#discussion_r4722394457
- Concern: `docs/issues/open/3080-record-request-ids-on-axum-request-spans/ISSUE.md` had `related-pr: null` and `status: in-progress` while PR #3084 was open.
- Solution: Updated `ISSUE.md` frontmatter to `related-pr: 3084`, `status: in-review` (allowed in `docs/skills/semantic-skill-link-convention.md`), refreshed `last-updated-utc`, and replied on the thread.
- Current-tree verification: `ISSUE.md` frontmatter carries `related-pr: 3084` and `status: in-review`, and passes `frontmatter-validator`.
- Resolution reference: fix(axum-server): [#3080] address PR #3084 review findings
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3084#discussion_r4726354815

### F10 - Track follow-up issue for duplicated request_id on response span lines

- PR number: 3084
- Source review ID: 6091805883
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3084#discussion_r4722394465
- Concern: Now that `request_id` is declared, each Axum server's `on_response` call to `span.record("request_id", ...)` appends a second `request_id` field on `response` lines because `SetRequestIdLayer` sits outside `TraceLayer` so `on_request` already fills the field. Dropping `on_response` records is out of scope for #3080 and warrants a linked follow-up issue.
- Solution: Kept the three Axum server crates unchanged in PR #3084 to preserve issue #3080 scope, opened follow-up issue #3087 (https://github.com/torrust/torrust-tracker/issues/3087), linked `#3087` in `ISSUE.md` (`Risks and Trade-offs`), and replied on the thread with the issue URL.
- Current-tree verification: https://github.com/torrust/torrust-tracker/issues/3087 is open, `ISSUE.md` (`Out of Scope` and `Risks and Trade-offs`) and `manual-verification-evidence.md` (`Failures and Follow-up`) document the scope boundary and link `#3087`, and the reply URL `https://github.com/torrust/torrust-tracker/pull/3084#discussion_r4726356941` states the follow-up.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3084#discussion_r4726356941
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3084#discussion_r4726356941

### F11 - Reply directly on every review thread and record actual Reply URLs

- PR number: 3084
- Source review ID: 6096846531
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3084#discussion_r4726283405
- Concern: None of the nine threads from review `6091805883` nor the earlier resolved CodeRabbit thread (`discussion_r4719271659`) had a per-thread reply, all ten `Reply URL` fields in `PR-REVIEW.md` stayed pending (`N/A (pending ...)`), and PR conversation comment `7233148375` did not name review IDs or per-finding dispositions (`validate-audit-record.py` failed with `expected exactly one discussion Reply URL, found 0`).
- Solution: Replied directly on every review thread (`F1` to `F15`) on GitHub with its disposition, verification, and resolution reference, replaced every pending `Reply URL` in `PR-REVIEW.md` with its actual GitHub comment `html_url`, and replied on this thread.
- Current-tree verification: Every finding `F1` to `F15` in `PR-REVIEW.md` holds its verified GitHub `Reply URL` (`https://github.com/torrust/torrust-tracker/pull/3084#discussion_r...`) matching the PR #3084 REST review comments.
- Resolution reference: docs(pr-3084): [#3080] record review replies and follow-up issue
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3084#discussion_r4726357730

### F12 - Record durable resolution reference for F10, open #3087, and fix line 83 claim

- PR number: 3084
- Source review ID: 6096846531
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3084#discussion_r4726283409
- Concern: `F10` (`FOLLOW_UP`) had `Resolution reference: N/A` instead of a durable reply URL (`process-pr-review` `SKILL.md:282-283`), the linked follow-up issue did not yet exist on GitHub, and line 83 of `PR-REVIEW.md` inaccurately claimed the prior revision passed offline structural checks matching `validate-audit-record.py`.
- Solution: Opened follow-up issue `#3087` (https://github.com/torrust/torrust-tracker/issues/3087), replied on the `F10` thread with the issue URL, set `F10`'s `Resolution reference` to that durable reply URL (`https://github.com/torrust/torrust-tracker/pull/3084#discussion_r4726356941`), removed the inaccurate offline-validation claim about the earlier `8c3a40e` revision from line 83 (`F2` `Current-tree verification`), and replied on this thread.
- Current-tree verification: Issue `#3087` exists on GitHub, `F10` records `https://github.com/torrust/torrust-tracker/pull/3084#discussion_r4726356941` as `Resolution reference`, `ISSUE.md` links `#3087`, and line 83 reflects only the updated audit state.
- Resolution reference: docs(pr-3084): [#3080] record review replies and follow-up issue
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3084#discussion_r4726358031

### F13 - Use docs type with commit body for subsequent review commits

- PR number: 3084
- Source review ID: 6096846531
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3084#discussion_r4726283412
- Concern: Commit `8c3a40e` changed documents and one test line with no production change yet used the `fix` type without a commit body; the reviewer noted `8c3a40e` did not need rewriting and advised using `docs(...)` or `test(...)` with an explanatory body for the next such commit.
- Solution: Kept `8c3a40e` untouched on the branch as advised, specified the subsequent follow-up commit as `docs(pr-3084): [#3080] record review replies and follow-up issue` with a descriptive commit body in `ISSUE.md` (`Commit Points`), and replied on the thread.
- Current-tree verification: `ISSUE.md` (`Commit Points`) records `docs(pr-3084): [#3080] record review replies and follow-up issue` as the follow-up docs/audit commit.
- Resolution reference: docs(pr-3084): [#3080] record review replies and follow-up issue
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3084#discussion_r4726358820

### F14 - List PR-REVIEW.md and review commits in the PR #3084 description

- PR number: 3084
- Source review ID: 6096846531
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3084#discussion_r4726283416
- Concern: The `Changes` section of PR #3084's description omitted `docs/pr-reviews/pr-3084-review/PR-REVIEW.md` and the review-fix commit, even though the merge commit copies the full PR description.
- Solution: Updated the PR #3084 description (`Changes` section) on GitHub to include `docs/pr-reviews/pr-3084-review/PR-REVIEW.md`, follow-up issue `#3087`, and the review-fix commits, tracked the update in `PR-REVIEW.md`, and replied on the thread.
- Current-tree verification: `https://api.github.com/repos/torrust/torrust-tracker/pulls/3084` confirms the PR description's `Changes` section lists `docs/pr-reviews/pr-3084-review/PR-REVIEW.md`, issue `#3087`, and `docs(pr-3084): [#3080] record review replies and follow-up issue`.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3084#discussion_r4726359144; docs(pr-3084): [#3080] record review replies and follow-up issue
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3084#discussion_r4726359144

### F15 - Name the agent model on the 2026-11-28 Progress Log entry

- PR number: 3084
- Source review ID: 6096846531
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3084#discussion_r4726283417
- Concern: The `2026-11-28 03:17 UTC` Progress Log entry in `ISSUE.md` named `Antigravity` without its model, contrary to `docs/templates/ISSUE.md:152-154`.
- Solution: Updated the `2026-11-28 03:17 UTC` Progress Log entry in `ISSUE.md` to `Antigravity (Gemini 4 Hydrogen)`, used the same format on the new `2026-11-28 09:24 UTC` Progress Log entry, and replied on the thread.
- Current-tree verification: Both `2026-11-28` Progress Log entries in `ISSUE.md` read `Antigravity (Gemini 4 Hydrogen)`.
- Resolution reference: docs(pr-3084): [#3080] record review replies and follow-up issue
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3084#discussion_r4726359769

## Processing Log

- 2026-11-27 12:13 UTC - Review 6087950878 (`claude[bot]`) arrived on first head; non-actionable notice that automated review is disabled for fork PRs.
- 2026-11-27 12:14 UTC - Issue comment 7217996387 (`coderabbitai[bot]`) posted walkthrough and pre-merge check summary; no new finding.
- 2026-11-27 12:17 UTC - CodeRabbit review 6088001515 arrived with F1 on `ISSUE.md:87`; T2 notes updated in `fix(axum-server): [#3080] declare request_id field on request spans` and thread resolved on GitHub before this audit record existed.
- 2026-11-27 17:57 UTC - Review 6091805883 (`da2ce7`) arrived on head `fix(axum-server): [#3080] declare request_id field on request spans` with nine inline threads (`F2` to `F10`).
- 2026-11-28 03:13 UTC - Removed unused `.header("x-request-id", request_id)` from `it_should_record_the_request_id_in_the_span_of_a_request` (F8) and ran the `write-unit-test` mutate-then-restore red run in `rust:1-bookworm` (`38 passed; 1 failed` at `request_log.rs:302:9`) (F4).
- 2026-11-28 03:15 UTC - Restored `use tracing::field::Empty;` and `request_id = Empty,` in `request_log.rs` and verified `cargo test -p torrust-tracker-axum-server` (`48` unit + `9` integration passed), Clippy with `-D warnings`, and `cargo fmt --all -- --check` in `rust:1-bookworm`.
- 2026-11-28 03:17 UTC - Created this audit record from `docs/templates/PR-REVIEW-TEMPLATE.md` covering `F1` to `F10` (F2) and updated `ISSUE.md` and `manual-verification-evidence.md` for `F3` to `F10`. Local `gh` CLI is unavailable and git/GitHub mutations are left for the maintainer/PR author, so `Reply URL` entries remain pending until commit `fix(axum-server): [#3080] address PR #3084 review findings` is committed manually, pushed, and replied to on GitHub.
- 2026-11-28 03:24 UTC - Commit `fix(axum-server): [#3080] address PR #3084 review findings` created on the branch; general PR conversation comment 7233148375 posted at `03:31 UTC`.
- 2026-11-28 08:54 UTC - Review 6096846531 (`da2ce7`) arrived on head `fix(axum-server): [#3080] address PR #3084 review findings` with five inline threads (`F11` to `F15`).
- 2026-11-28 09:09 UTC - Follow-up issue #3087 (`https://github.com/torrust/torrust-tracker/issues/3087`) opened for duplicated `on_response` `request_id` recording (`F10`, `F12`), and PR #3084 description updated (`F14`).
- 2026-11-28 09:16 UTC - PR author finished posting direct per-thread replies on all 15 review threads (`F1` at `09:11 UTC`, `F2`–`F10` at `09:12`–`09:15 UTC`, `F11`–`F15` at `09:16 UTC`); `F8` thread marked `RESOLVED` on GitHub. In the same window (`09:13 UTC`), CodeRabbit review 6096915465 (`discussion_r4726346283`) replied on the `F1` thread requesting explicit T2 gate order wording.
- 2026-11-28 09:24 UTC - Recorded actual `Reply URL` values across `F1` to `F10`, `F8` resolved state, durable reply `Resolution reference` for `F10` with issue `#3087`, corrected `F2` verification text, and added `F11` to `F15` for commit `docs(pr-3084): [#3080] record review replies and follow-up issue`; updated `ISSUE.md` Progress Log model names, `#3087` link, and `Commit Points`.

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- For an outdated thread whose concern was fixed, record `Disposition=FIXED` and
  `Thread state=RESOLVED`, even if GitHub marks the thread outdated after the push. For a
  duplicate, superseded, or no-change in-PR thread, reply exactly
  `Superseded by <FindingId>: <reason>.`, record `Disposition=NO_ACTION` and
  `Thread state=SUPERSEDED`, then resolve it. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work. When the current identity cannot resolve a thread
  (`viewerCanResolve` is `false`), still reply, keep the disposition established from the current
  tree, and record `Thread state=OPEN`; the reviewer or PR author resolves it.
- A consolidated PR conversation response may cover multiple review rounds only when it names
  every review ID and every finding ID with its disposition and resolution reference. Record its
  durable URL in each related row.
- Cite a fix by its unique Conventional Commit subject or durable reply URL, never by a branch SHA
  that can change after a rebase.
- Refresh review threads using GraphQL and confirm that no unresolved actionable thread remains.
