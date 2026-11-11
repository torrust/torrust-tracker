---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2834 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2834.

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

Round 1 is da2ce7's review `5860290788` (F1 to F5) and Copilot's review `5860342629` (F6 to F8, numbered after F5), both submitted on the head whose last commit has the subject `docs(issues): [#2824] record the S1 PeerRole fix in the spec`. F6 and F7 re-raise F1 and F3.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2834-f1 | Human | Major | testing | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2834-f2 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2834-f3 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2834-f4 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2834-f5 | Human | Nit | testing | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2834-f6 | Copilot | Minor (inferred) | documentation | RE_RAISE_OF:F1 | FIXED | RESOLVED |
| F7 | review-finding:pr-2834-f7 | Copilot | Minor (inferred) | documentation | RE_RAISE_OF:F3 | FIXED | RESOLVED |
| F8 | review-finding:pr-2834-f8 | Copilot | Minor (inferred) | testing | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - T2a is marked DONE but the manual verification evidence file is missing

- PR number: 2834
- Source review ID: 5860290788
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2834#discussion_r4539093566
- Concern: T2a was marked DONE while the evidence file that the spec makes a merge precondition for S1 (`manual-verification-evidence.md`) did not exist; the red run was asserted only in a commit body and a log entry.
- Solution: Added `manual-verification-evidence.md` from the template: the F2 reproduction at the nearest seam with its classification, the red output of both tests on the `test(primitives)` commit, the green output after the `fix(primitives)` commit and the head recheck, with start and end times and the toolchain.
- Current-tree verification: The runs listed in the file were executed on 2026-11-11 from detached checkouts of the three commits; the file names each command and its output. `ls docs/issues/open/2824-1347-add-tests-primitives/` lists the file.
- Resolution reference: docs(issues): [#2824] add the manual verification evidence for the S1 fix
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2834#discussion_r4539187928

### F2 - Tracking state left stale or outside the spec's vocabulary

- PR number: 2834
- Source review ID: 5860290788
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2834#discussion_r4539093576
- Concern: The S1 row had no PR and used the status `IN REVIEW`, which is not in the spec's status values; the S0 row and the S0 checkpoint were stale; the F2 register row said `FIXED (PR pending)`; the frontmatter kept `planned`, the S0 branch and PR, and an old stamp.
- Solution: Set the S1 row to `#2834` and `IN_PROGRESS`, the S0 row to `DONE`, ticked the S0 checkpoint, wrote `FIXED through #2834` in the F2 register row, and updated the frontmatter (`status: in-progress`, branch, `related-pr`, `last-updated-utc`).
- Current-tree verification: `grep -n "IN REVIEW\|FIXED (PR pending)\|status: planned" docs/issues/open/2824-1347-add-tests-primitives/ISSUE.md` prints nothing; the S0 and S1 slice rows and the frontmatter were read after the edit.
- Resolution reference: docs(issues): [#2824] update the spec tracking state after S0 merged
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2834#discussion_r4539188470

### F3 - Per-PR measurement and EPIC updates are missing

- PR number: 2834
- Source review ID: 5860290788
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2834#discussion_r4539093586
- Concern: The slice PR touched neither the slice measurement in `coverage-evidence.md` nor the EPIC "Latest" values, and the EPIC row stayed `TODO`.
- Solution: Re-measured the package with the baseline command (49 unit tests; unit-only lines 468 / 746 = 62.73%, regions 70.29%, functions 59.49%), added an S1 section to `coverage-evidence.md`, updated both EPIC "Latest" tables and the Change cells, set the EPIC row to `IN_PROGRESS` and added an EPIC log entry. The note says the change includes merges to `develop` since the baseline, not only this slice's two tests.
- Current-tree verification: `cargo llvm-cov -p torrust-tracker-primitives --all-features --lib --json` ran after `cargo llvm-cov clean --workspace`; the totals were filtered to `packages/primitives/src/` with a script and match the figures written.
- Resolution reference: docs(issues): [#2824] record the S1 coverage measurement and the EPIC update
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2834#discussion_r4539188907

### F4 - Log entry stamped later than its commit

- PR number: 2834
- Source review ID: 5860290788
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2834#discussion_r4539093593
- Concern: The S1 log entry was stamped 14:10 UTC, later than the 14:07:54 UTC commit that adds it.
- Solution: Restamped the entry 14:07 UTC. Swept the records of this PR for the same mistake and found three more (the spec frontmatter, the EPIC `last-updated-utc` and its new log entry, and the evidence file's `last-updated-utc`, stamped 14:40 and 14:35 against commits at 14:31 and 14:32) and restamped them in a second commit.
- Current-tree verification: `git log --format=%cI` gives the commit times; `grep -rn "14:40\|14:35"` over the issue folder and the EPIC prints nothing after the second commit.
- Resolution reference: docs(issues): [#2824] update the spec tracking state after S0 merged
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2834#discussion_r4539189276

### F5 - The test names claim a Display match that the assertions do not check

- PR number: 2834
- Source review ID: 5860290788
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2834#discussion_r4539093599
- Concern: Both tests are named "like its display format" but compare a literal, so a change to `Display` alone would leave them green.
- Solution: Both tests now also assert the JSON value equals `PeerRole::<role>.to_string()`.
- Current-tree verification: `cargo test -p torrust-tracker-primitives --lib peer_role_serialization` ran: 2 passed; the full package run passes 49 unit tests.
- Resolution reference: test(primitives): [#2824] compare the PeerRole JSON with its Display form
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2834#discussion_r4539189654

### F6 - Manual-verification evidence is absent (re-raise of F1)

- PR number: 2834
- Source review ID: 5860342629
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2834#discussion_r4539135457
- Concern: Copilot reports that the S1 `manual-verification-evidence.md` required by the spec is absent and asks for the commands and outputs.
- Solution: Same request as F1, already fixed by the evidence file (see F1).
- Current-tree verification: `ls docs/issues/open/2824-1347-add-tests-primitives/` lists `manual-verification-evidence.md`.
- Resolution reference: docs(issues): [#2824] add the manual verification evidence for the S1 fix
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2834#discussion_r4539208689

### F7 - Slice coverage and EPIC tables not updated (re-raise of F3)

- PR number: 2834
- Source review ID: 5860342629
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2834#discussion_r4539135527
- Concern: Copilot reports that `coverage-evidence.md` holds only the baseline and both EPIC "Latest" entries say "Not measured yet".
- Solution: Same request as F3, already fixed by the S1 measurement and the EPIC update (see F3).
- Current-tree verification: `grep -n "Not measured yet" docs/issues/open/1347-overhaul-packages-testing/EPIC.md` shows no `primitives` row.
- Resolution reference: docs(issues): [#2824] record the S1 coverage measurement and the EPIC update
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2834#discussion_r4539209273

### F8 - No recorded prose-first AAA design review for the two new tests

- PR number: 2834
- Source review ID: 5860342629
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2834#discussion_r4539135613
- Concern: Copilot reports that `docs/templates/ISSUE.md` requires the prose-first Arrange-Act-Assert review to be recorded for each test-producing increment and that none is recorded for the two `PeerRole` tests.
- Solution: Recorded the review in `test-refactor-plans/s1-peer-role-serialization.md`: the prose per section against the code, the causal initial state, no fixture, the independent expected result and the two-function layout. Both tests are `clean` after it, so no test changed.
- Current-tree verification: The file exists in the issue folder; each test's Act and assertions were compared with its table row when writing it.
- Resolution reference: docs(issues): [#2824] record the prose-first AAA review of the PeerRole tests
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2834#discussion_r4539209752

## Processing Log

- 2026-11-11 14:35 UTC - Started audit after da2ce7's review `5860290788` (changes requested, 5 findings).
- 2026-11-11 14:36 UTC - Replied on the five threads of da2ce7's review (F1 to F5) against the head whose last commit has the subject `docs(issues): [#2824] record the S1 coverage measurement and the EPIC update`; the audit validator passed (5 rows).
- 2026-11-11 14:38 UTC - Copilot's review `5860342629` (3 findings on the same head) processed: F6 and F7 re-raise F1 and F3, F8 is new. Replied on its three threads and resolved all eight threads; the fixes for F4 and F8 and this audit are in the next push.

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
