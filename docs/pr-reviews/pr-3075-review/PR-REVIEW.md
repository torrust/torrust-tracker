---
pr-number: 3075
pr-url: https://github.com/torrust/torrust-tracker/pull/3075
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
last-updated-utc: "2026-11-26 17:53"
---

<!-- skill-link: process-pr-review -->

# PR #3075 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/3075.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and do not own this record. A reply file that an issue-linked
review task names is a separate artifact, which the reviewer writes (see the `review-pr` skill).

- Post-merge workflow approval: N/A (pre-merge review).

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

Round 1: CodeRabbit review 6077575588 on the head of commit `docs(issues): [#144] record EPIC state and inputs reported by SI-B` (F1). The review of `claude[bot]` (6077541485) says that automated review is disabled for fork pull requests and carries no finding.

Round 2: `da2ce7`'s review 6077981724 on the head of commit `docs(issues): [#144] address CodeRabbit: align invalid-token status candidates` (F2 to F8); F7 and F8 are findings of its review body without an inline thread.

Round 3: `da2ce7`'s review 6078167335 on the head of commit `docs(issues): [#144] address da2ce7 review: state today's v1 behavior in R3, R4, R5` (F9 to F12); F10 to F12 are findings of its review body without an inline thread, and F10 re-raises F2 for a head that predates the record.

Round 4: `da2ce7`'s review 6078600500 (an approval) on the head of commit `docs(pr-reviews): [#144] record the third review round on pull request 3075` (F13, F14).

| Finding ID | Review finding reference | Reviewer | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | -------- | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-3075-f1 | coderabbitai[bot] | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-3075-f2 | da2ce7 | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-3075-f3 | da2ce7 | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-3075-f4 | da2ce7 | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-3075-f5 | da2ce7 | Nit | correctness | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-3075-f6 | da2ce7 | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-3075-f7 | da2ce7 | Minor | documentation | ORIGINAL | FIXED | NON_RESOLVABLE |
| F8 | review-finding:pr-3075-f8 | da2ce7 | Nit | other | ORIGINAL | NO_ACTION | NON_RESOLVABLE |
| F9 | review-finding:pr-3075-f9 | da2ce7 | Nit | correctness | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-3075-f10 | da2ce7 | Minor | documentation | RE_RAISE_OF:F2 | FIXED | NON_RESOLVABLE |
| F11 | review-finding:pr-3075-f11 | da2ce7 | Minor | documentation | ORIGINAL | FIXED | NON_RESOLVABLE |
| F12 | review-finding:pr-3075-f12 | da2ce7 | Nit | documentation | ORIGINAL | FIXED | NON_RESOLVABLE |
| F13 | review-finding:pr-3075-f13 | da2ce7 | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F14 | review-finding:pr-3075-f14 | da2ce7 | Nit | metadata | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The invalid-token candidates of R2 and the skeleton note drop 404

- PR number: 3075
- Source review ID: 6077575588
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3075#discussion_r4711129948
- Concern: The error-model row allows `404` for an invalid token, while R2 and the skeleton sub-issue note limited token failures to `401` or `403`, so the candidate lists conflicted.
- Solution: R2 and the skeleton note now name `401`, `403` or `404`, matching the error-model row.
- Current-tree verification: Read R2 and the sub-issue 3 note of `EPIC.md` after the edit.
- Resolution reference: docs(issues): [#144] address CodeRabbit: align invalid-token status candidates
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3075#discussion_r4711178118

### F2 - The head has no PR-REVIEW.md audit record

- PR number: 3075
- Source review ID: 6077981724
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3075#discussion_r4711430398
- Concern: The thread of F1 was answered and resolved, but the head had no `docs/pr-reviews/pr-3075-review/PR-REVIEW.md`, which `process-pr-review` requires before the first reply.
- Solution: This record was created from the template of a recent audit. The reply to F1 was posted before it existed: the late start is logged in the Processing Log. The frontmatter was aligned with the template afterwards (see F14 and the Processing Log).
- Current-tree verification: `ls docs/pr-reviews/pr-3075-review/PR-REVIEW.md` and `validate-audit-record.py --pr-number 3075` after the commit.
- Resolution reference: docs(pr-reviews): [#144] record the review rounds on pull request 3075
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3075#discussion_r4711597665

### F3 - R5 puts the #3055 bound on the duration instead of the expiry

- PR number: 3075
- Source review ID: 6077981724
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3075#discussion_r4711430407
- Concern: The `500` of #3055 starts when the expiry is above `i64::MAX`; a duration that overflows the clock already answers `400`.
- Solution: R5 now says the `500` starts when the expiry (now plus the duration) is above `i64::MAX` seconds while the duration still fits the clock, and that a clock overflow already answers `400`.
- Current-tree verification: Read `auth_key_store.rs:49` (`i64::try_from` on the expiry) and `auth_key/handlers.rs:42` and `:80` (`DurationOverflow` maps to `invalid_auth_key_duration_response`).
- Resolution reference: docs(issues): [#144] address da2ce7 review: state today's v1 behavior in R3, R4, R5
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3075#discussion_r4711488325

### F4 - R4 lists the planned text policy as today's v1 behavior

- PR number: 3075
- Source review ID: 6077981724
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3075#discussion_r4711430416
- Concern: The column is "v1 behavior today", but no v1 error body carries a request id: the request id is only the `x-request-id` header.
- Solution: R4 now states today's `500` text, says the request id is only the header, and marks the fixed text with the request id as the planned v1 policy of #3000.
- Current-tree verification: Read `responses.rs:92-99` (`unhandled_rejection_response`) after the edit.
- Resolution reference: docs(issues): [#144] address da2ce7 review: state today's v1 behavior in R3, R4, R5
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3075#discussion_r4711488732

### F5 - R3 attributes the key and info-hash text to the framework

- PR number: 3075
- Source review ID: 6077981724
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3075#discussion_r4711430426
- Concern: A malformed key or info-hash answers `400` with the tracker's own text; only the JSON body extractor gives the mixed `400`, `422` and `415` statuses with library text.
- Solution: R3 now says so. The query claim is dropped because it was not verified.
- Current-tree verification: Read `responses.rs:53-63` (`invalid_info_hash_param_response`, `invalid_auth_key_param_response`).
- Resolution reference: docs(issues): [#144] address da2ce7 review: state today's v1 behavior in R3, R4, R5
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3075#discussion_r4711489097

### F6 - The sub-issue 3 note change left last-updated-utc and the progress log unchanged

- PR number: 3075
- Source review ID: 6077981724
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3075#discussion_r4711430433
- Concern: The fix of F1 changed the spec but left `last-updated-utc` and added no Progress Log entry.
- Solution: `last-updated-utc` is now the time of this round and the Progress Log has an entry for this round that also cites the fix of F1.
- Current-tree verification: Read the frontmatter and the last Progress Log entry of `EPIC.md` after the edit.
- Resolution reference: docs(issues): [#144] address da2ce7 review: state today's v1 behavior in R3, R4, R5
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3075#discussion_r4711489480

### F7 - The PR description gives 401/403 and lacks the required items

- PR number: 3075
- Source review ID: 6077981724
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3075#pullrequestreview-6077981724
- Concern: The description still gave the v2 token statuses as `401`/`403` and lacked the files-touched and validation items that `open-pull-request` requires.
- Solution: The PR description was edited: the statuses are `401`, `403` or `404`, and it has "Files touched" and "Validation" sections.
- Current-tree verification: Read the PR description on GitHub after the edit.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3075#issuecomment-7198533402
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3075#issuecomment-7198533402

### F8 - The branch name lacks the -spec suffix

- PR number: 3075
- Source review ID: 6077981724
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3075#pullrequestreview-6077981724
- Concern: The branch `144-epic-intake` lacks the `-spec` suffix of `create-feature-branch`.
- Solution: No change: renaming the branch would close this pull request and open a new one for a naming nit. The next spec branch uses the suffix.
- Current-tree verification: Not applicable: nothing changed.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3075#issuecomment-7198533402
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3075#issuecomment-7198533402

### F9 - R5 names the duration where the clock check is on the expiry

- PR number: 3075
- Source review ID: 6078167335
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3075#discussion_r4711563297
- Concern: `now_add` checks now plus the duration, so the `500` needs the expiry to fit the clock; an expiry that overflows the clock answers `400`.
- Solution: R5 now says a key whose expiry is above `i64::MAX` seconds but still fits the clock returns `500`, and an expiry that overflows the clock already answers `400`.
- Current-tree verification: Read `handler.rs:102` and `key/mod.rs:111` (`now_add` of the duration).
- Resolution reference: docs(issues): [#144] address da2ce7 third round: bound the R5 500 by the expiry
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3075#discussion_r4711710887

### F10 - The head has no PR-REVIEW.md and its thread has no reply

- PR number: 3075
- Source review ID: 6078167335
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3075#pullrequestreview-6078167335
- Concern: The reviewed head had no audit record while five threads were answered; this re-raises F2.
- Solution: No new change: the record was pushed after that head (see F2).
- Current-tree verification: `ls docs/pr-reviews/pr-3075-review/PR-REVIEW.md` and `validate-audit-record.py --pr-number 3075`.
- Resolution reference: docs(pr-reviews): [#144] record the review rounds on pull request 3075
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3075#issuecomment-7199057391

### F11 - The description lists a record the branch lacked and keeps the old #3055 line

- PR number: 3075
- Source review ID: 6078167335
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3075#pullrequestreview-6078167335
- Concern: The description listed the record under files touched but the reviewed head did not contain it, and its #3055 line still said a key duration above `i64::MAX` returns `500`.
- Solution: The record is on the branch (F2). The #3055 line of the description now names the expiry.
- Current-tree verification: Read the PR description on GitHub after the edit.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3075#issuecomment-7199057391
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3075#issuecomment-7199057391

### F12 - Replies cite branch commit ids

- PR number: 3075
- Source review ID: 6078167335
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3075#pullrequestreview-6078167335
- Concern: Replies 4711178118, 4711488325, 4711488732, 4711489097 and 4711489480 cited branch commit ids instead of commit subjects.
- Solution: The five replies, and reply 4711597665 which had the same fault, were edited to cite commit subjects only.
- Current-tree verification: Fetched each reply body after the edit and found no commit id in it.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3075#issuecomment-7199057391
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3075#issuecomment-7199057391

### F13 - The round paragraph of review 6078167335 is out of order

- PR number: 3075
- Source review ID: 6078600500
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3075#discussion_r4711891666
- Concern: The Round 3 paragraph sat between the paragraphs for the first two reviews.
- Solution: The paragraphs now follow the order the reviews arrived: rounds 1, 2, 3, 4.
- Current-tree verification: Read the Findings introduction of this record after the edit.
- Resolution reference: docs(pr-reviews): [#144] address da2ce7 fourth round: order the rounds and add the template frontmatter
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3075#discussion_r4711936923

### F14 - The record does not follow the template form

- PR number: 3075
- Source review ID: 6078600500
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3075#discussion_r4711891675
- Concern: The record was created from an earlier audit, so its frontmatter lacks the template's `semantic-links` and its Ownership paragraph is not the template's text.
- Solution: The frontmatter now has the template's `semantic-links`. The Ownership paragraph stays as it is: the template's sentence says reviewers have no repository-artifact obligation, which `review-pr` contradicts (the same contradiction was found on PR 3070), so aligning the template and the records is a change of its own. The late start stays recorded in F2.
- Current-tree verification: Read the frontmatter of this record after the edit; ran `validate-audit-record.py`.
- Resolution reference: docs(pr-reviews): [#144] address da2ce7 fourth round: order the rounds and add the template frontmatter
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3075#discussion_r4711937236

## Processing Log

- 2026-11-26 16:34 UTC - Replied to F1 on its thread and resolved it, before this record existed (late start, F2).
- 2026-11-26 17:00 UTC - `da2ce7`'s review 6077981724 arrived (F2 to F8).
- 2026-11-26 17:07 UTC - Replied on the threads of F3 to F6 after pushing their fix (`docs(issues): [#144] address da2ce7 review: state today's v1 behavior in R3, R4, R5`), and resolved them.
- 2026-11-26 17:07 UTC - Created this record from the template (late start, F2), with every finding of both rounds. F2's reply URL is recorded by the commit that follows its reply.
- 2026-11-26 17:14 UTC - `da2ce7`'s review 6078167335 arrived (F9 to F12).
- 2026-11-26 17:17 UTC - Replied to F2 on its thread and resolved it; recorded its reply URL.
- 2026-11-26 17:27 UTC - Replied to F9 on its thread and resolved it, posted the conversation response for F10 to F12, and recorded both reply URLs.
- 2026-11-26 17:48 UTC - `da2ce7`'s review 6078600500 (approval) arrived (F13, F14).
- 2026-11-26 17:49 UTC - Reordered the round paragraphs and added the template's `semantic-links` to the frontmatter (F13, F14).
- 2026-11-26 17:53 UTC - Replied to F13 and F14 on their threads, resolved them and recorded the reply URLs.

Append entries only. If an entry was rewritten in place, restore the prior entries, then append a correction that names the rewritten content; do not rewrite the record again.

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
