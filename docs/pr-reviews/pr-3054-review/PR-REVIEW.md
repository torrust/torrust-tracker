---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
    - "issue #3051"
---

<!-- skill-link: process-pr-review -->

# PR #3054 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/3054>.

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

Review 6059658951 (`claude[bot]`, 2026-11-25 14:36 UTC) posted one inline comment, F1. Review 6059932482 (`da2ce7`, `CHANGES_REQUESTED`) posted ten inline comments, F2 to F11; its body has no further actionable assertion. Review 6060302201 (`da2ce7`, `COMMENTED`, 2026-11-25 15:16 UTC) posted three more, F12 to F14. Review 6060493901 (`coderabbitai[bot]`, `COMMENTED`, 15:28 UTC) has one finding in its body, F15, with no thread. Review 6060544492 (`da2ce7`, 15:31 UTC, later dismissed) posted three inline comments, F16 to F18. CodeRabbit posted only its review-limit notice.

| Finding ID | Review finding reference | Reviewer | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | -------- | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-3054-f1 | claude[bot] | Minor (inferred) | correctness | ORIGINAL | FOLLOW_UP | RESOLVED |
| F2 | review-finding:pr-3054-f2 | da2ce7 | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-3054-f3 | da2ce7 | Minor | other | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-3054-f4 | da2ce7 | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-3054-f5 | da2ce7 | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-3054-f6 | da2ce7 | Nit | correctness | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-3054-f7 | da2ce7 | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-3054-f8 | da2ce7 | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-3054-f9 | da2ce7 | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-3054-f10 | da2ce7 | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-3054-f11 | da2ce7 | Nit | correctness | ORIGINAL | FIXED | RESOLVED |
| F12 | review-finding:pr-3054-f12 | da2ce7 | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F13 | review-finding:pr-3054-f13 | da2ce7 | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F14 | review-finding:pr-3054-f14 | da2ce7 | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F15 | review-finding:pr-3054-f15 | coderabbitai[bot] | Minor | maintainability | ORIGINAL | FIXED | NON_RESOLVABLE |
| F16 | review-finding:pr-3054-f16 | da2ce7 | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F17 | review-finding:pr-3054-f17 | da2ce7 | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F18 | review-finding:pr-3054-f18 | da2ce7 | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Durations whose expiry exceeds i64::MAX still return 500

- PR number: 3054
- Source review ID: 6059658951
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3054#discussion_r4696470914>
- Concern: The spec checks only clock overflow, but the stores convert the expiry with `i64::try_from`, so an admin who sends 2^63 seconds still gets a 500 instead of the 400. The reviewer marked it pre-existing and not blocking, and suggested bounding the check or replying.
- Solution: Verified in `sqlite/auth_key_store.rs:49` (the same conversion is in the other stores) and filed it as a separate issue, #3055: a different root cause (the persistence range, not the clock), no panic. Recorded under Out of Scope in the spec. The issue's tests use `u64::MAX`, which overflows the clock.
- Current-tree verification: `sed -n 44,56p packages/tracker-core/src/databases/driver/sqlite/auth_key_store.rs` shows `i64::try_from(value.as_secs())`; `gh issue view 3055` shows the issue open.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/3054#discussion_r4696525898>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3054#discussion_r4696525898>

### F2 - The generate-key endpoint cannot answer 400 through the adapter alone

- PR number: 3054
- Source review ID: 6059932482
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3054#discussion_r4696694169>
- Concern: `generate_auth_key_handler` answers every `AuthKeyError` with a 500 and a test pins it, so the spec's AC4 and M3 need a handler change that the scope did not list and the Out of Scope line excluded.
- Solution: Added the handler arm (400 through the existing `invalid_auth_key_duration_response`), its doc comment and handler tests to the scope, the Decision section and the test strategy; narrowed the Out of Scope line to status codes clients receive today. This widens the scope the maintainer approved; the reply says so.
- Current-tree verification: `grep -n "generate_auth_key_handler" docs/issues/open/3051-random-expiring-key-duration-overflow-panic/ISSUE.md` prints the Decision, In Scope and test-strategy lines.
- Resolution reference: docs(issues): [#3051] add the generate-key handler to the spec scope
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3054#discussion_r4696770393>

### F3 - The branch carries no PR-REVIEW.md audit record

- PR number: 3054
- Source review ID: 6059932482
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3054#discussion_r4696694182>
- Concern: `process-pr-review` has the author create the audit record before the first reply; the branch has none.
- Solution: Created this record, with a row for the earlier `claude[bot]` finding. The first reply (to the `claude[bot]` thread) was posted before the record existed; the Processing Log says so.
- Current-tree verification: `ls docs/pr-reviews/pr-3054-review/PR-REVIEW.md` lists the file.
- Resolution reference: docs(pr-reviews): [#3051] add the PR 3054 review audit
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3054#discussion_r4696770887>

### F4 - The test-design review was routed to the PR description

- PR number: 3054
- Source review ID: 6059932482
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3054#discussion_r4696694196>
- Concern: The template and `create-issue` record the review in task evidence or a file-local test plan, not in the PR description.
- Solution: The spec's Commit Points section names `test-design-review.md` in the issue folder as the place, after each passing increment and before maintainer review.
- Current-tree verification: `grep -n "test-design-review.md" docs/issues/open/3051-random-expiring-key-duration-overflow-panic/ISSUE.md` prints the line.
- Resolution reference: docs(issues): [#3051] add the generate-key handler to the spec scope
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3054#discussion_r4696771344>

### F5 - Request 5 carries no time

- PR number: 3054
- Source review ID: 6059932482
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3054#discussion_r4696694258>
- Concern: The claim that the tracker keeps serving depends on the order of the requests, but request 5 has no time and no end time is recorded.
- Solution: V1 step 5 now says it was not timestamped and why; V3 requires the start and the end of every request.
- Current-tree verification: `grep -n "Not timestamped\|Stamp the start" docs/issues/open/3051-random-expiring-key-duration-overflow-panic/manual-verification-evidence.md` prints both.
- Resolution reference: docs(issues): [#3051] add the generate-key handler to the spec scope
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3054#discussion_r4696772091>

### F6 - The i64::MAX bound is on the expiry, not the duration

- PR number: 3054
- Source review ID: 6059932482
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3054#discussion_r4696694274>
- Concern: The stores convert the expiry, so the 500 range starts at `i64::MAX` minus the current time.
- Solution: The spec's Out of Scope entry and the body of #3055 state the bound as an expiry above `i64::MAX` seconds.
- Current-tree verification: `grep -n "expiry (now plus the duration)" docs/issues/open/3051-random-expiring-key-duration-overflow-panic/ISSUE.md`; `gh issue view 3055 --json body`.
- Resolution reference: docs(issues): [#3051] add the generate-key handler to the spec scope
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3054#discussion_r4696772656>

### F7 - The panic quote is not verbatim

- PR number: 3054
- Source review ID: 6059932482
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3054#discussion_r4696694296>
- Concern: Each panic line in the log carries the thread id; the log file is an extract of two matches, and the command that made it is not named.
- Solution: The quote includes the thread id and the text names the extraction command and the `--` separator.
- Current-tree verification: `grep -n "4055249" docs/issues/open/3051-random-expiring-key-duration-overflow-panic/manual-verification-evidence.md` prints the quote.
- Resolution reference: docs(issues): [#3051] add the generate-key handler to the spec scope
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3054#discussion_r4696773251>

### F8 - Two randomly_generated modules in handler.rs

- PR number: 3054
- Source review ID: 6059932482
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3054#discussion_r4696694313>
- Concern: The temporary test names only `randomly_generated`, which exists in the expiring and in the permanent module.
- Solution: V2 names `handling_expiring_peer_keys::randomly_generated`.
- Current-tree verification: `grep -n "handling_expiring_peer_keys" docs/issues/open/3051-random-expiring-key-duration-overflow-panic/manual-verification-evidence.md`.
- Resolution reference: docs(issues): [#3051] add the generate-key handler to the spec scope
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3054#discussion_r4696773762>

### F9 - M1 and V1 name the requests differently

- PR number: 3054
- Source review ID: 6059932482
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3054#discussion_r4696694332>
- Concern: M1 says three requests and calls the pre-generated request the control; V1 runs five and calls request 1 the control.
- Solution: Only request 1 is the control; request 3 is the pre-generated comparison; M1 and M3 say five requests with the expected result per request number.
- Current-tree verification: `grep -n "five requests" docs/issues/open/3051-random-expiring-key-duration-overflow-panic/ISSUE.md` prints M1 and M3.
- Resolution reference: docs(issues): [#3051] add the generate-key handler to the spec scope
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3054#discussion_r4696774320>

### F10 - The completion-review conditions were dropped

- PR number: 3054
- Source review ID: 6059932482
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3054#discussion_r4696694351>
- Concern: The template's conditions for `implementation-retrospective.md` are missing from the spec.
- Solution: Added the conditions to the Implementation Completion Review section.
- Current-tree verification: `grep -n "implementation-retrospective.md" docs/issues/open/3051-random-expiring-key-duration-overflow-panic/ISSUE.md`.
- Resolution reference: docs(issues): [#3051] add the generate-key handler to the spec scope
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3054#discussion_r4696774832>

### F11 - The caller list is loose

- PR number: 3054
- Source review ID: 6059932482
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3054#discussion_r4696694364>
- Concern: The persistence-benchmark callers are a binary, not tests, and `generate_permanent_peer_key` and the permanent branch change with the error type.
- Solution: The In Scope list names the benchmark binary and adds `generate_permanent_peer_key` and the permanent branch of `add_peer_key`.
- Current-tree verification: `grep -n "generate_permanent_peer_key\|persistence_benchmark" docs/issues/open/3051-random-expiring-key-duration-overflow-panic/ISSUE.md`.
- Resolution reference: docs(issues): [#3051] add the generate-key handler to the spec scope
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3054#discussion_r4696775350>

### F12 - The validator fails on the persistence-range row

- PR number: 3054
- Source review ID: 6060302201
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3054#discussion_r4696993091>
- Concern: The F1 row had a bare comment id in `Resolution reference` and `Reply URL`, and its verification pointed at a validator result that the log says does not exist.
- Solution: Both fields hold the full discussion URL of the F1 reply, and the F3 verification no longer cites a validator result.
- Current-tree verification: `grep -n "4696525898" docs/pr-reviews/pr-3054-review/PR-REVIEW.md` prints only URL forms.
- Resolution reference: docs(pr-reviews): [#3051] record review 6060302201 in the PR 3054 audit
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3054#discussion_r4697016215>

### F13 - The generate-key handler test is not in the plan

- PR number: 3054
- Source review ID: 6060302201
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3054#discussion_r4696993105>
- Concern: T1, T2 and the red-run paragraph list only the `KeysHandler` and contract tests, and the existing handler test would no longer match its name once the arm lands.
- Solution: The spec lists the `generate_auth_key_handler` test in T1, T2 and the red-run paragraph, and plans the rename in T3.
- Current-tree verification: `grep -n "generate_auth_key_handler" docs/issues/open/3051-random-expiring-key-duration-overflow-panic/ISSUE.md` prints the plan lines.
- Resolution reference: docs(issues): [#3051] plan the handler test and name the log capture
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3054#discussion_r4697016795>

### F14 - The log capture is not shown in the start command

- PR number: 3054
- Source review ID: 6060302201
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3054#discussion_r4696993113>
- Concern: The panic extract reads `.tmp/tracker.log`, but the start command shows no redirect to it.
- Solution: The start command shows `> .tmp/tracker.log 2>&1`, and the text says what the file holds and that the folder was under `drafts/` when it ran.
- Current-tree verification: `grep -n "tracker.log 2>&1" docs/issues/open/3051-random-expiring-key-duration-overflow-panic/manual-verification-evidence.md`.
- Resolution reference: docs(issues): [#3051] plan the handler test and name the log capture
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3054#discussion_r4697017419>

### F15 - Cite issue 3051 by number, not by its open/ path

- PR number: 3054
- Source review ID: 6060493901
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3054#pullrequestreview-6060493901>
- Concern: the audit's `related-artifacts` cites the spec under `docs/issues/open/`, a path that goes stale when the issue closes and its spec moves to `closed/`. The reviewer marked the finding unverified.
- Solution: accepted. The audit `pr-3046-review` cites its issue as `"issue #3019"`, so the frontmatter now cites `"issue #3051"`.
- Current-tree verification: `sed -n 1,9p docs/pr-reviews/pr-3054-review/PR-REVIEW.md` shows `"issue #3051"` in `related-artifacts`, with no `docs/issues/open` path among the frontmatter lines.
- Resolution reference: docs(pr-reviews): [#3051] cite issue 3051 by number in the PR 3054 audit
- Follow-up PR URL: N/A
- Reply URL: N/A

### F16 - The F12 row credits the wrong commit

- PR number: 3054
- Source review ID: 6060544492
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3054#discussion_r4697191766>
- Concern: The F12 row's resolution reference names the commit that changed only the spec and the evidence file, and my reply named the same commit; the audit edit came from the commit that recorded review 6060302201.
- Solution: The F12 row now names `docs(pr-reviews): [#3051] record review 6060302201 in the PR 3054 audit`. The reply cannot be edited; the reply to F16 states the correction.
- Current-tree verification: `git log --format=%s -- docs/pr-reviews/pr-3054-review/PR-REVIEW.md` lists that subject, and the F12 detail entry cites it.
- Resolution reference: docs(pr-reviews): [#3051] record review 6060544492 in the PR 3054 audit
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3054#discussion_r4697424395>

### F17 - The evidence shows a command that did not run as written

- PR number: 3054
- Source review ID: 6060544492
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3054#discussion_r4697191787>
- Concern: The note that the folder was under `drafts/` when the tracker ran meant the command shown (with the `open/` path) was not the command that ran.
- Solution: The evidence shows the command as run, with the `drafts/` path, and says the folder has since moved to `open/`.
- Current-tree verification: `grep -n "issues/drafts/random-expiring" docs/issues/open/3051-random-expiring-key-duration-overflow-panic/manual-verification-evidence.md` prints the command.
- Resolution reference: docs(issues): [#3051] show the tracker start command as it ran
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3054#discussion_r4697458638>

### F18 - The PR description does not list the audit record

- PR number: 3054
- Source review ID: 6060544492
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3054#discussion_r4697191801>
- Concern: The description's Files list names only the issue folder, but the pull request also adds the audit record.
- Solution: The description's Files list and Summary now name the audit record. The fix is outside the repository tree (the pull request description), so the reply URL is the durable reference.
- Current-tree verification: `gh api repos/torrust/torrust-tracker/pulls/3054 --jq .body` contains `docs/pr-reviews/pr-3054-review/PR-REVIEW.md`.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/3054#discussion_r4697438606>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3054#discussion_r4697438606>

## Processing Log

- 2026-11-25 14:41 UTC - Late start: the reply to F1 (reply `created_at` 14:41:01Z) was posted, and its thread resolved, before this record existed. F1 had been handled with the commit `docs(issues): [#3051] record the persistence range as out of scope` (committed 14:38:12Z) and the new issue #3055.
- 2026-11-25 14:53 UTC - Review 6059932482 (`da2ce7`, `CHANGES_REQUESTED`) was submitted (`submitted_at` 14:53:42Z); F2 to F11 are its inline comments.
- 2026-11-25 14:55 UTC - The fixes for F2 and F4 to F11 were committed in one commit, `docs(issues): [#3051] add the generate-key handler to the spec scope`, and pushed (`git ls-remote` showed that head on the fork branch before the replies). #3055 was edited for F6.
- 2026-11-25 14:59 UTC - Replied on the ten `da2ce7` threads (reply `created_at` 14:59:22Z to 14:59:46Z) and resolved them. The reply to F3 said the record would be added in the next commit.
- 2026-11-25 15:01 UTC - Wrote this record for F3. `validate-audit-record.py` has not been run: it calls plain `gh`, which acts as `josecelano`; running it as the bot needs the maintainer's permission, asked for in the PR report. The maintainer's `ACK` of the earlier head is stale after the spec commit and after this commit; the new head needs a new `ACK`.
- 2026-11-25 15:16 UTC - Review 6060302201 (`da2ce7`, `COMMENTED`, `submitted_at` 15:16:08Z) arrived on head `docs(pr-reviews): [#3051] add the PR 3054 review audit`: F12 to F14. F12 found the F1 row's bare comment id; the replies to F12 to F14 were posted before the fix commit and say so.
- 2026-11-25 15:18 UTC - Wrote the rows F12 to F14 and fixed the F1 row. `validate-audit-record.py` was not run by this session (it calls plain `gh`); the reviewer's run reported the two failures that F12 fixes.
- 2026-11-25 15:28 UTC - Review 6060493901 (`coderabbitai[bot]`, `COMMENTED`, `submitted_at` 15:28:00Z) arrived on the head `docs(pr-reviews): [#3051] record review 6060302201 in the PR 3054 audit`: one finding in its body, F15 (no thread, so no reply).
- 2026-11-25 15:29 UTC - Fixed F15 in the commit `docs(pr-reviews): [#3051] cite issue 3051 by number in the PR 3054 audit`.
- 2026-11-25 15:31 UTC - Review 6060544492 (`da2ce7`, `submitted_at` 15:31:08Z, later dismissed) posted F16 to F18; the maintainer's `ACK` of 15:31:12Z named the head before the audit-only commit `docs(pr-reviews): [#3051] cite issue 3051 by number in the PR 3054 audit`.
- 2026-11-25 15:47 UTC - Edited the pull request description for F18 (`pulls/3054` PATCH).
- 2026-11-25 16:13 UTC - GitHub answered 5xx to most reply requests from 15:48 UTC, and some requests that looked failed had posted: the threads of F16 and F17 each have a second identical reply (`4697558202`, `4697504758`, posted 16:13:25Z and 16:05:11Z), which the bot wrapper cannot delete. The rows cite the first reply of each thread.
- 2026-11-25 16:14 UTC - Wrote the rows F16 to F18 and corrected the F12 resolution reference in the commit `docs(pr-reviews): [#3051] record review 6060544492 in the PR 3054 audit`. `validate-audit-record.py` was not run by this session (it calls plain `gh`).

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
