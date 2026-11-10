---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2812 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2812.

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
- Author class: `Copilot`, `Human`, `Unknown`
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

Reviews of da2ce7, in order: 5846465906 (F1 to F8), 5846866430 (F9 to F14), 5847274601 (F15 to F17), 5847680359 (F18) and 5847973119 (approval with F19; its row is added in the commit after its reply). Copilot review 5846485416 reported no open issues and has no row.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2812-f1 | Human | Major | metadata | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2812-f2 | Human | Major | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2812-f3 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2812-f4 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2812-f5 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2812-f6 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2812-f7 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2812-f8 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-2812-f9 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-2812-f10 | Human | Minor | other | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-2812-f11 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F12 | review-finding:pr-2812-f12 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F13 | review-finding:pr-2812-f13 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F14 | review-finding:pr-2812-f14 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F15 | review-finding:pr-2812-f15 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F16 | review-finding:pr-2812-f16 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F17 | review-finding:pr-2812-f17 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F18 | review-finding:pr-2812-f18 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - AC2 ticked, but 13 crates' keywords repeat a word of their crate name

- PR number: 2812
- Source review ID: 5846465906
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2812#discussion_r4528296861
- Concern: The metadata rule said no keyword repeats a word of the crate name, yet the approved keyword table did.
- Solution: The maintainer had approved the table, so the rule was amended and the approved table recorded in the spec.
- Current-tree verification: Compared `cargo metadata` keywords with the spec table and the rule text at the fixing commit. Reply text cites the branch head by id (`cb8b8f5`), which was the PR head when the reply was posted; its subject is the Resolution reference.
- Resolution reference: docs(issues): [#2576] record the approved keyword table and amend the keyword rule
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2812#discussion_r4528332619

### F2 - M1 marked DONE without recorded evidence

- PR number: 2812
- Source review ID: 5846465906
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2812#discussion_r4528296867
- Concern: The evidence file did not record the M1 read-through, did not follow the template and pointed ACs at sections that did not exist.
- Solution: Rewrote `manual-verification-evidence.md` with goal, steps, observed result and conclusion per process, the full V1 command, the URL checks and per-AC evidence.
- Current-tree verification: Read the evidence file; ran the URL checks it records.
- Resolution reference: docs(packages): [#2576] address the PR review findings F2 to F8
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2812#discussion_r4528385995

### F3 - Audit attributes a decision the spec does not record

- PR number: 2812
- Source review ID: 5846465906
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2812#discussion_r4528296873
- Concern: The audit said examples were optional and none were needed, but the spec's decision and quality-bar item 5 asked reusable crates for an example.
- Solution: Made item 5 optional (no example is added in this PR) and aligned the audit and progress log.
- Current-tree verification: grep of the spec, audit and log for the example wording.
- Resolution reference: docs(packages): [#2576] address the PR review findings F2 to F8
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2812#discussion_r4528386403

### F4 - Template misses two create-markdown-template steps

- PR number: 2812
- Source review ID: 5846465906
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2812#discussion_r4528296882
- Concern: The template was not in `docs/index.md`, had no frontmatter and no skill pointed to it.
- Solution: Added the index row, a step in the `add-workspace-member` skill and a comment on why the template has no frontmatter.
- Current-tree verification: Inspected the three files.
- Resolution reference: docs(packages): [#2576] address the PR review findings F2 to F8
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2812#discussion_r4528386743

### F5 - Two tier N audit notes claim fixes this PR did not make

- PR number: 2812
- Source review ID: 5846465906
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2812#discussion_r4528296888
- Concern: `e2e-tools` and `persistence-benchmark` were unchanged, and some tier U/P READMEs are out of scope.
- Solution: Marked the two rows unchanged, named the real fixes for `torrent-repository-benchmarking` and wrote "every in-scope".
- Current-tree verification: Inspected the audit rows against `git diff --stat`.
- Resolution reference: docs(packages): [#2576] address the PR review findings F2 to F8
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2812#discussion_r4528387074

### F6 - README points at deprecated re-exports

- PR number: 2812
- Source review ID: 5846465906
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2812#discussion_r4528296896
- Concern: The `primitives` README listed `PeerId` and `DurationSinceUnixEpoch`, which are deprecated re-exports.
- Solution: Removed both from the purpose and Main API.
- Current-tree verification: grep of the README for the two names.
- Resolution reference: docs(packages): [#2576] address the PR review findings F2 to F8
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2812#discussion_r4528387441

### F7 - Spec bookkeeping lags the implementation

- PR number: 2812
- Source review ID: 5846465906
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2812#discussion_r4528296901
- Concern: Spec-only PR checkpoint open, stale frontmatter, AC6 ticked before the merge.
- Solution: Ticked the checkpoint, updated the frontmatter and unticked AC6.
- Current-tree verification: Inspected the spec frontmatter and checkpoints.
- Resolution reference: docs(packages): [#2576] address the PR review findings F2 to F8
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2812#discussion_r4528387831

### F8 - License section names another product

- PR number: 2812
- Source review ID: 5846465906
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2812#discussion_r4528296907
- Concern: The REST API client README named the client library product and wrote "prosperity".
- Solution: Named the REST API client and wrote "posterity" in the three LGPL READMEs.
- Current-tree verification: grep for `prosperity` and the product name.
- Resolution reference: docs(packages): [#2576] address the PR review findings F2 to F8
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2812#discussion_r4528388265

### F9 - Rule's word list misses three repeats of the approved table

- PR number: 2812
- Source review ID: 5846866430
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2812#discussion_r4528602650
- Concern: The amended rule named four words, but the approved table also keeps `application`, `adapter` and `runtime`.
- Solution: The rule now lets the approved table list the exceptions; V1's conclusion says the same.
- Current-tree verification: Compared the table keywords with the rule text.
- Resolution reference: docs(issues): [#2576] fix the keyword rule, M1 and M3 pointers, stamps and baseline wording
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2812#discussion_r4528668680

### F10 - F2 to F8 replies posted before the commit they cite

- PR number: 2812
- Source review ID: 5846866430
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2812#discussion_r4528602662
- Concern: The seven F2 to F8 replies were posted 17:44:10 to 17:44:21 UTC, before the commit they cite was created (17:45:21 UTC), and the F1 reply cites a branch commit id.
- Solution: Replies cannot be edited, so the Processing Log records the timing as a correction and the F1 detail entry maps the cited commit id `cb8b8f5` to its subject. Process change: reply only after the commit is pushed.
- Current-tree verification: Compared the reply `created_at` times from the GitHub API with `git log --format=%cI`; the validator exits 0.
- Resolution reference: docs(pr-reviews): [#2576] add the review audit record for PR 2812
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2812#discussion_r4528708383

### F11 - No review audit record on the branch

- PR number: 2812
- Source review ID: 5846866430
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2812#discussion_r4528602667
- Concern: The threads were answered and resolved without the tracked audit record that `process-pr-review` requires.
- Solution: Added `docs/pr-reviews/pr-2812-review/PR-REVIEW.md` with one row and one detail entry per finding.
- Current-tree verification: `python3 .github/skills/dev/pr-reviews/process-pr-review/scripts/validate-audit-record.py --pr-number 2812` exits 0.
- Resolution reference: docs(pr-reviews): [#2576] add the review audit record for PR 2812
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2812#discussion_r4528708746

### F12 - last-updated-utc stamps postdate their commit; log entries lack times

- PR number: 2812
- Source review ID: 5846866430
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2812#discussion_r4528602675
- Concern: The stamps said 18:10 while the commit was made at 17:45, and two log entries had no time.
- Solution: Stamps now carry the time of the edit and all four log entries of this PR carry `HH:MM UTC` from `git log --format=%cI`.
- Current-tree verification: Compared the stamps with `git log --format=%cI`.
- Resolution reference: docs(issues): [#2576] fix the keyword rule, M1 and M3 pointers, stamps and baseline wording
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2812#discussion_r4528669023

### F13 - M3 row points at V3 after the renumbering

- PR number: 2812
- Source review ID: 5846866430
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2812#discussion_r4528602683
- Concern: M3 pointed at V3 (now the read-through) and M1's steps no longer matched what ran.
- Solution: M3 points at V4 and M1's steps describe what ran.
- Current-tree verification: Inspected the manual verification table and the evidence sections.
- Resolution reference: docs(issues): [#2576] fix the keyword rule, M1 and M3 pointers, stamps and baseline wording
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2812#discussion_r4528669382

### F14 - Baseline says every member inherited documentation

- PR number: 2812
- Source review ID: 5846866430
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2812#discussion_r4528602691
- Concern: Only tracker packages inherited the field; the developer tools did not.
- Solution: Changed the baseline to "every tracker package".
- Current-tree verification: grep of the evidence file.
- Resolution reference: docs(issues): [#2576] fix the keyword rule, M1 and M3 pointers, stamps and baseline wording
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2812#discussion_r4528669734

### F15 - The Findings introduction names both review heads by branch commit id

- PR number: 2812
- Source review ID: 5847274601
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2812#discussion_r4528912432
- Concern: The introduction cited two abbreviated branch commit ids, which a rebase would invalidate.
- Solution: Named the rounds by review ID only.
- Current-tree verification: Ran the commit-id check from `open-pull-request`: the only id left is the one quoted from the F1 reply, mapped to its subject.
- Resolution reference: docs(pr-reviews): [#2576] drop branch commit ids from the audit introduction and reword F1
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2812#discussion_r4529059134

### F16 - The F1 entry says the cited commit was later pushed

- PR number: 2812
- Source review ID: 5847274601
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2812#discussion_r4528912441
- Concern: "Later pushed" suggested the F1 reply had the same timing fault as F2 to F8, which the forge data does not show.
- Solution: Reworded: the commit was the PR head when the reply was posted.
- Current-tree verification: Read the F1 entry and the Processing Log entry for 17:39 UTC.
- Resolution reference: docs(pr-reviews): [#2576] drop branch commit ids from the audit introduction and reword F1
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2812#discussion_r4529059492

### F17 - The 18:05 progress-log entry answers F10 and F11 before the audit record existed

- PR number: 2812
- Source review ID: 5847274601
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2812#discussion_r4528912451
- Concern: The 18:05 entry names the audit record's F10 and F11 answers, but that record was added at 18:10:49 UTC and its F10 and F11 rows at 18:13:46 UTC.
- Solution: Left the entry as written and appended a progress-log entry for the audit commits, with times from `git log --format=%cI`.
- Current-tree verification: Read the progress log of `ISSUE.md` and compared the times with `git log --format=%cI`.
- Resolution reference: docs(issues): [#2576] append the audit-record entry to the progress log and name F17 in the audit
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2812#discussion_r4529263246

### F18 - The record leaves F17 out of review 5847274601 and its thread was resolved without a reply

- PR number: 2812
- Source review ID: 5847680359
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2812#discussion_r4529228166
- Concern: Review 5847274601 carried F15, F16 and F17, but the record named two, had no row for F17, and the F17 thread was resolved with no reply.
- Solution: Named F17 in the Findings introduction, replied on the F17 thread (late), added rows and detail entries for F17 and F18, and appended a correction to the Processing Log (the 18:32 entry stays, as the log is append only).
- Current-tree verification: Ran the audit validator; compared the review's comment list with the rows.
- Resolution reference: docs(pr-reviews): [#2576] record findings F17 and F18 in the PR 2812 audit
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2812#discussion_r4529263606

## Processing Log

- 2026-11-10 17:35 UTC - Round 1 review of da2ce7 received (review 5846465906, eight findings F1 to F8).
- 2026-11-10 17:37 UTC - Copilot review 5846485416 received: no open issues, its agentic review timed out; no row.
- 2026-11-10 17:39 UTC - F1 fixed in commit `docs(issues): [#2576] record the approved keyword table and amend the keyword rule` (created 17:37:09 UTC and pushed), reply posted 17:39:16 UTC. The reply cites the branch head by id `cb8b8f5`, a commit id; its Conventional Commit subject is the resolution reference above.
- 2026-11-10 17:45 UTC - F2 to F8 fixed in commit `docs(packages): [#2576] address the PR review findings F2 to F8` (created 17:45:21 UTC, pushed after a failed first attempt). The seven replies were posted between 17:44:10 and 17:44:21 UTC, before the commit existed; they were wrong until that commit was pushed. Correction recorded here and in round 2 finding F10.
- 2026-11-10 18:03 UTC - Round 2 review of da2ce7 received (review 5846866430, six findings F9 to F14).
- 2026-11-10 18:05 UTC - F9, F12, F13 and F14 fixed in commit `docs(issues): [#2576] fix the keyword rule, M1 and M3 pointers, stamps and baseline wording` (created 18:05:41 UTC) and pushed.
- 2026-11-10 18:09 UTC - F9, F12, F13 and F14 replies posted (18:09:43 to 18:09:49 UTC), after the push.
- 2026-11-10 18:13 UTC - Audit commit `docs(pr-reviews): [#2576] add the review audit record for PR 2812` (created 18:10:49 UTC) pushed; F10 and F11 replies posted at 18:13:18 and 18:13:20 UTC, after the push.
- 2026-11-10 18:32 UTC - Round 3 review of da2ce7 received (review 5847274601: approval at the then head, with F15 and F16).
- 2026-11-10 18:46 UTC - F15 and F16 fixed in commit `docs(pr-reviews): [#2576] drop branch commit ids from the audit introduction and reword F1` (created 18:35:23 UTC) and pushed (the first push attempt failed when GitHub closed the SSH connection during the pre-push hook); replies posted at 18:46:33 and 18:46:35 UTC, after the push.
- 2026-11-10 19:02 UTC - Round 4 review of da2ce7 received (review 5847680359: F18; it records F15 and F16 as fixed).
- 2026-11-10 19:03 UTC - Correction: the 18:32 UTC entry above lists F15 and F16, but review 5847274601 also carried F17 (on `ISSUE.md`). I resolved the F17 thread at 18:46 UTC without a reply, because my resolve loop closed every open thread. F17 was fixed in commit `docs(issues): [#2576] append the audit-record entry to the progress log and name F17 in the audit` (created 19:03:06 UTC) and pushed.
- 2026-11-10 19:05 UTC - F17 and F18 replies posted (19:05:44 and 19:05:46 UTC), after the push.
- 2026-11-10 19:22 UTC - Round 5 review of da2ce7 received (review 5847973119: approval at head `93bf0e094`, with F19).
- 2026-11-10 19:29 UTC - Correction: the 19:05 entry above says both replies came after the push. That holds for the F17 reply (19:05:44 UTC), which cites a pushed commit. The F18 reply (19:05:46 UTC) says the F17 and F18 rows follow "in the next commit", and that commit (`docs(pr-reviews): [#2576] record findings F17 and F18 in the PR 2812 audit`) was created at 19:06:01 UTC, 15 s later; the part of the F18 fix that the reply described as already done (the introduction naming F17) was pushed before it.

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
