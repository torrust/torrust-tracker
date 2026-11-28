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

Review 6087950878 (`claude[bot]`) has no finding: it notes that automated review is disabled for a pull request from a fork. Issue comment 7217996387 (`coderabbitai[bot]`) is a walkthrough and pre-merge check summary with no new finding.

| Finding ID | Review finding reference | Reviewer | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | -------- | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-3084-f1 | coderabbitai[bot] | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-3084-f2 | da2ce7 | Minor | documentation | ORIGINAL | FIXED | OPEN |
| F3 | review-finding:pr-3084-f3 | da2ce7 | Minor | documentation | ORIGINAL | FIXED | OPEN |
| F4 | review-finding:pr-3084-f4 | da2ce7 | Minor | testing | ORIGINAL | FIXED | OPEN |
| F5 | review-finding:pr-3084-f5 | da2ce7 | Minor | documentation | ORIGINAL | FIXED | OPEN |
| F6 | review-finding:pr-3084-f6 | da2ce7 | Minor | formatting | ORIGINAL | FIXED | OPEN |
| F7 | review-finding:pr-3084-f7 | da2ce7 | Nit | correctness | ORIGINAL | FIXED | OPEN |
| F8 | review-finding:pr-3084-f8 | da2ce7 | Nit | testing | ORIGINAL | FIXED | OPEN |
| F9 | review-finding:pr-3084-f9 | da2ce7 | Nit | metadata | ORIGINAL | FIXED | OPEN |
| F10 | review-finding:pr-3084-f10 | da2ce7 | Suggestion | maintainability | ORIGINAL | FOLLOW_UP | OPEN |

## Finding Details

### F1 - Add required test-development review gates to T2

- PR number: 3084
- Source review ID: 6088001515
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3084#discussion_r4719271659
- Concern: T2 in `ISSUE.md` defined the regression test and red run but did not state the required `write-unit-test` loop: focused validation followed by the design review (causal-state, fixture-ownership, and visible Act/expected-result checks) and stopping for maintainer review after the final increment before final verification, commit, or PR.
- Solution: Added the `write-unit-test` design review notes to T2 in `ISSUE.md` in `fix(axum-server): [#3080] declare request_id field on request spans`, and refined T2 and `Commit Points` in `fix(axum-server): [#3080] address PR #3084 review findings` so the completed T2 row states what ran while keeping the maintainer review pending in `Workflow Checkpoints` and `Commit Points` (also resolving F5).
- Current-tree verification: `docs/issues/open/3080-record-request-ids-on-axum-request-spans/ISSUE.md` lists the `write-unit-test` design review checks on T2 and the pending maintainer-review checkpoint in `Workflow Checkpoints` and `Commit Points`; `frontmatter-validator` validates `ISSUE.md`.
- Resolution reference: fix(axum-server): [#3080] declare request_id field on request spans; fix(axum-server): [#3080] address PR #3084 review findings
- Follow-up PR URL: N/A
- Reply URL: N/A (thread resolved on GitHub after the first commit without an inline reply; post a follow-up reply referencing the fix commits before closing the review loop)

### F2 - Create the tracked PR review audit record for PR #3084

- PR number: 3084
- Source review ID: 6091805883
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3084#discussion_r4722394419
- Concern: The CodeRabbit thread on `ISSUE.md:87` (F1) was resolved without creating `docs/pr-reviews/pr-3084-review/PR-REVIEW.md` as required by `process-pr-review`.
- Solution: Created `docs/pr-reviews/pr-3084-review/PR-REVIEW.md` from `docs/templates/PR-REVIEW-TEMPLATE.md` covering F1 and all findings from review `6091805883` (`F2` to `F10`), plus the non-actionable review and issue comment.
- Current-tree verification: `docs/pr-reviews/pr-3084-review/PR-REVIEW.md` exists and passes offline structural checks matching `validate-audit-record.py` and `frontmatter-validator`. Full `validate-audit-record.py` verification against GitHub commits and replies requires `gh` CLI and must be run after pushing the follow-up commit and posting thread replies.
- Resolution reference: fix(axum-server): [#3080] address PR #3084 review findings
- Follow-up PR URL: N/A
- Reply URL: N/A (pending commit, push, and GitHub reply)

### F3 - Sync commit subjects in evidence and Commit Points with the branch

- PR number: 3084
- Source review ID: 6091805883
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3084#discussion_r4722394429
- Concern: `manual-verification-evidence.md` cited two commit subjects absent from the branch (`test(axum-server): [#3080] add regression test for request_id on request span` and `fix(axum-server): [#3080] declare empty request_id field on RedactedMakeSpan`), while `ISSUE.md` still planned separate T2/T3/T4 commits in `Commit Points`.
- Solution: Updated `manual-verification-evidence.md` (`Environment and Prerequisites`, `T3`, and `V2`) to cite the actual branch commit `fix(axum-server): [#3080] declare request_id field on request spans` (noting that the original T3/V2 runs executed on the uncommitted tree whose production diff matches that commit) and the review follow-up commit `fix(axum-server): [#3080] address PR #3084 review findings`, and updated `Commit Points` in `ISSUE.md` to match.
- Current-tree verification: Neither of the two non-existent commit subjects appears in `manual-verification-evidence.md` or `ISSUE.md`; both documents reference `fix(axum-server): [#3080] declare request_id field on request spans` and `fix(axum-server): [#3080] address PR #3084 review findings`.
- Resolution reference: fix(axum-server): [#3080] address PR #3084 review findings
- Follow-up PR URL: N/A
- Reply URL: N/A (pending commit, push, and GitHub reply)

### F4 - Re-record T2 red run on the final regression test form

- PR number: 3084
- Source review ID: 6091805883
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3084#discussion_r4722394431
- Concern: The recorded T2 failure showed an earlier rewrite (`request_log.rs:304:9`) rather than a red run of the final regression test.
- Solution: After removing the unused header in `it_should_record_the_request_id_in_the_span_of_a_request`, executed the `write-unit-test` mutate-then-restore cycle in Docker `rust:1-bookworm`: temporarily removed `use tracing::field::Empty;` and `request_id = Empty,`, ran `cargo test -p torrust-tracker-axum-server --lib request_log::tests` (`38 passed; 1 failed`, panicking at `packages/axum-server/src/request_log.rs:302:9`, matching final `assert!` line `304:9` once the two fix lines are restored), restored the fix, and recorded the verbatim failure in `manual-verification-evidence.md`.
- Current-tree verification: `manual-verification-evidence.md` section `T2` records the `2026-11-28T03:13:52Z` to `03:13:59Z` mutate-then-restore failure; after restoring the fix, `cargo test -p torrust-tracker-axum-server` passes all 57 tests (`48` unit + `9` integration).
- Resolution reference: fix(axum-server): [#3080] address PR #3084 review findings
- Follow-up PR URL: N/A
- Reply URL: N/A (pending commit, push, and GitHub reply)

### F5 - Keep only completed work in DONE T2 row and leave maintainer review pending

- PR number: 3084
- Source review ID: 6091805883
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3084#discussion_r4722394436
- Concern: `T2` had status `DONE` in `ISSUE.md`, yet its notes ended with imperative text to stop for maintainer review before final verification, commit, or PR even though that checkpoint remained unchecked and PR #3084 was already open.
- Solution: Updated the `T2` `DONE` row in `ISSUE.md` to describe only what was completed (regression test following `write-unit-test`, red run, and prose-first Arrange-Act-Assert design review record in `manual-verification-evidence.md`) and kept the pending maintainer review explicit in `Workflow Checkpoints` and `Commit Points`.
- Current-tree verification: `ISSUE.md` `T2` row describes only completed work and notes that pending maintainer review stays under `Workflow Checkpoints` (unchecked) and `Commit Points`.
- Resolution reference: fix(axum-server): [#3080] address PR #3084 review findings
- Follow-up PR URL: N/A
- Reply URL: N/A (pending commit, push, and GitHub reply)

### F6 - Replace local checkout path failing cspell spellcheck

- PR number: 3084
- Source review ID: 6091805883
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3084#discussion_r4722394443
- Concern: `manual-verification-evidence.md:154` contained the local host path with an unrecognized word, causing `cspell` to fail the `Docs Lint` and `Unit (stable)` CI jobs.
- Solution: Replaced the local host path in the `docker run` snippet in `manual-verification-evidence.md` with `"<checkout>:/workspace"`.
- Current-tree verification: `git grep -i` confirms zero occurrences of the local user directory name across `docs/` and `packages/`.
- Resolution reference: fix(axum-server): [#3080] address PR #3084 review findings
- Follow-up PR URL: N/A
- Reply URL: N/A (pending commit, push, and GitHub reply)

### F7 - Fix test total sum in T3 conclusion (449, not 448)

- PR number: 3084
- Source review ID: 6091805883
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3084#discussion_r4722394449
- Concern: The per-crate test counts in `manual-verification-evidence.md` (`48 + 9 + 13 + 13 + 1 + 53 + 77 + 1 + 170 + 63 + 1`) sum to `449`, not `448`.
- Solution: Corrected the T3 total count in `manual-verification-evidence.md` from `448` to `449`, alongside documenting the follow-up `torrust-tracker-axum-server` re-run (`57` tests).
- Current-tree verification: `manual-verification-evidence.md` states `449` and `448` no longer appears in the T3 conclusion.
- Resolution reference: fix(axum-server): [#3080] address PR #3084 review findings
- Follow-up PR URL: N/A
- Reply URL: N/A (pending commit, push, and GitHub reply)

### F8 - Remove unused x-request-id header from regression test Arrange

- PR number: 3084
- Source review ID: 6091805883
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3084#discussion_r4722394454
- Concern: `it_should_record_the_request_id_in_the_span_of_a_request` in `packages/axum-server/src/request_log.rs` set `.header("x-request-id", request_id)` even though `RedactedMakeSpan::make_span` reads no headers, violating `write-unit-test`'s causal initial-state rule.
- Solution: Removed `.header("x-request-id", request_id)` from `Request::builder()` in `packages/axum-server/src/request_log.rs` and updated the matching `Prose-first Arrange-Act-Assert Record` in `manual-verification-evidence.md`.
- Current-tree verification: `cargo test -p torrust-tracker-axum-server`, `cargo clippy -p torrust-tracker-axum-server --all-targets --all-features -- -D warnings`, and `cargo fmt --all -- --check` pass after the edit in Docker `rust:1-bookworm`.
- Resolution reference: fix(axum-server): [#3080] address PR #3084 review findings
- Follow-up PR URL: N/A
- Reply URL: N/A (pending commit, push, and GitHub reply)

### F9 - Update ISSUE.md frontmatter related-pr and status

- PR number: 3084
- Source review ID: 6091805883
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3084#discussion_r4722394457
- Concern: `docs/issues/open/3080-record-request-ids-on-axum-request-spans/ISSUE.md` had `related-pr: null` and `status: in-progress` while PR #3084 was open.
- Solution: Updated `ISSUE.md` frontmatter to `related-pr: 3084`, `status: in-review` (allowed in `docs/skills/semantic-skill-link-convention.md`), and refreshed `last-updated-utc`.
- Current-tree verification: `ISSUE.md` frontmatter carries `related-pr: 3084` and `status: in-review`, and passes `frontmatter-validator`.
- Resolution reference: fix(axum-server): [#3080] address PR #3084 review findings
- Follow-up PR URL: N/A
- Reply URL: N/A (pending commit, push, and GitHub reply)

### F10 - Track follow-up issue for duplicated request_id on response span lines

- PR number: 3084
- Source review ID: 6091805883
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3084#discussion_r4722394465
- Concern: Now that `request_id` is declared, each Axum server's `on_response` call to `span.record("request_id", ...)` appends a second `request_id` field on `response` lines because `SetRequestIdLayer` sits outside `TraceLayer` so `on_request` already fills the field. Dropping `on_response` records is out of scope for #3080 and warrants a linked follow-up issue.
- Solution: Kept the three Axum server crates unchanged in PR #3084 to preserve issue #3080 scope, noted the follow-up tracking in `ISSUE.md` (`Risks and Trade-offs`), and marked `F10` as `FOLLOW_UP` so a dedicated follow-up issue can be opened and linked without delaying #3080.
- Current-tree verification: `ISSUE.md` (`Out of Scope` and `Risks and Trade-offs`) and `manual-verification-evidence.md` (`Failures and Follow-up`) document why `on_response` stays unchanged in PR #3084 and needs a follow-up issue.
- Resolution reference: N/A
- Follow-up PR URL: N/A
- Reply URL: N/A (pending GitHub reply and follow-up issue creation)

## Processing Log

- 2026-11-27 12:13 UTC - Review 6087950878 (`claude[bot]`) arrived on first head; non-actionable notice that automated review is disabled for fork PRs.
- 2026-11-27 12:14 UTC - Issue comment 7217996387 (`coderabbitai[bot]`) posted walkthrough and pre-merge check summary; no new finding.
- 2026-11-27 12:17 UTC - CodeRabbit review 6088001515 arrived with F1 on `ISSUE.md:87`; T2 notes updated in `fix(axum-server): [#3080] declare request_id field on request spans` and thread resolved on GitHub before this audit record existed.
- 2026-11-27 17:57 UTC - Review 6091805883 (`da2ce7`) arrived on head `fix(axum-server): [#3080] declare request_id field on request spans` with nine inline threads (`F2` to `F10`).
- 2026-11-28 03:13 UTC - Removed unused `.header("x-request-id", request_id)` from `it_should_record_the_request_id_in_the_span_of_a_request` (F8) and ran the `write-unit-test` mutate-then-restore red run in `rust:1-bookworm` (`38 passed; 1 failed` at `request_log.rs:302:9`) (F4).
- 2026-11-28 03:15 UTC - Restored `use tracing::field::Empty;` and `request_id = Empty,` in `request_log.rs` and verified `cargo test -p torrust-tracker-axum-server` (`48` unit + `9` integration passed), Clippy with `-D warnings`, and `cargo fmt --all -- --check` in `rust:1-bookworm`.
- 2026-11-28 03:17 UTC - Created this audit record from `docs/templates/PR-REVIEW-TEMPLATE.md` covering `F1` to `F10` (F2) and updated `ISSUE.md` and `manual-verification-evidence.md` for `F3` to `F10`. Local `gh` CLI is unavailable and git/GitHub mutations are left for the maintainer/PR author, so `Reply URL` entries remain pending until commit `fix(axum-server): [#3080] address PR #3084 review findings` is committed manually, pushed, and replied to on GitHub.

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
