---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2883 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2883.

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

Review 5921522069 by da2ce7 (`APPROVED`, 2026-11-16 14:20:43 UTC, at the head ending in
`chore(issues): [#2393] archive the closed issue spec and mark it DONE in EPIC #2392`) raised three
inline findings, F1 to F3. Its body's other paragraphs are a summary and checks, with no request.

Review 5921551645 by Copilot (`COMMENTED`, 2026-11-16 14:22:36 UTC, same head) raised two inline
findings, F4 and F5. Its body is an overview with no other request.

Review 5922953773 by da2ce7 (`APPROVED`, 2026-11-16 15:52:16 UTC, at the head ending in
`docs(pr-reviews): [#2393] add the PR 2883 review audit`) raised three inline findings. Their reviewer
IDs F4 to F6 collide with this audit's F4 and F5, so they are recorded as F6 to F8. Its body's other
paragraphs are a summary and checks, with no request.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2883-f1 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2883-f2 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2883-f3 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2883-f4 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2883-f5 | Copilot | Nit (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2883-f6 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2883-f7 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2883-f8 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The EPIC keeps three statements that wait on order 1

- PR number: 2883
- Source review ID: 5921522069
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2883#discussion_r4586905336
- Concern: With order 1 DONE, the EPIC's Phase 1 state still said order 1 stays open for T7 to T11, the AC2 row still waited on #2393 T10, and the lost-coverage risk still waited on T9.
- Solution: Phase 1's state now says done, citing PRs 2859 and 2874 and V6. The AC2 row cites V6 (150 of 150 runs with no #2393 test ignored) and says AC2 stays TODO until the order 2 audit (AC4) confirms no other shared resource makes the suite fail. The risk is marked resolved: the restored tests run on Linux and are ignored on other systems. A progress-log entry records the sync.
- Current-tree verification: `rg -n 'stays open|stays TODO until #2393|untested until' docs/issues/open/2392-test-isolation/EPIC.md` prints nothing.
- Resolution reference: docs(issues): [#2392] mark Phase 1 done and sync AC2 and the coverage risk with #2393
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2883#discussion_r4587848050

### F2 - A closing keyword before #2393 in the commit body

- PR number: 2883
- Source review ID: 5921522069
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2883#discussion_r4586905345
- Concern: The commit body opened with `PR 2874 merged and closed #2393.`, which matches the closing-keyword pattern and would land on `develop` unchanged.
- Solution: With the maintainer's approval, the commit was reworded. The body now opens with `Issue #2393 is closed after PR 2874 merged.`
- Current-tree verification: `git log -1 --format=%B` of the commit named below shows the new sentence; `git log --format=%B torrust/develop..HEAD | rg -i '(close[sd]?|fix(e[sd])?|resolve[sd]?) #[0-9]+'` prints nothing.
- Resolution reference: chore(issues): [#2393] archive the closed issue spec and mark it DONE in EPIC #2392
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2883#discussion_r4587848509

### F3 - The new stamps are later than the commit that carries them

- PR number: 2883
- Source review ID: 5921522069
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/2883#discussion_r4586905354
- Concern: Six stamps said 2026-11-16 13:56 UTC, but the commit carrying them was made at 13:54:31 UTC.
- Solution: All six were set to 13:54, no later than that commit, whose author time 13:54:31 the reword kept. The two `last-updated-utc` values of the #2393 spec and of the EPIC moved on with later edits.
- Current-tree verification: `rg -n '2026-11-16 13:56' docs/issues/open/2392-test-isolation docs/issues/open/2394-2392-audit-shared-test-resources docs/issues/closed/2393-2392-test-port-allocation-races` prints nothing; `TZ=UTC git log --format='%ad %s' --date=format-local:%T` shows the archive commit authored at 13:54:31.
- Resolution reference: docs(issues): [#2393] stamp the archive entries with the archive commit time
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2883#discussion_r4587848943

### F4 - T11's row reads as instructions after it is DONE

- PR number: 2883
- Source review ID: 5921551645
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2883#discussion_r4586927128
- Concern: T11's Expected Output cell is written as future instructions and names PR 2862, which the review reads as unrelated to the close-out.
- Solution: The cell is now in the past tense and says which merged PRs the close-out followed: the implementation PR 2874 and the EPIC sync PR 2862. PR 2862 stays: it is the coordinator's EPIC update, and T11 waited for it so that this PR would not conflict with it.
- Current-tree verification: `rg -n '^\| T11' docs/issues/closed/2393-2392-test-port-allocation-races/ISSUE.md` prints the past-tense row.
- Resolution reference: docs(issues): [#2393] state the T11 close-out in the past tense
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2883#discussion_r4587849520

### F5 - "Random" and "unique" loopback address used for the same thing

- PR number: 2883
- Source review ID: 5921551645
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2883#discussion_r4586927236
- Concern: Order 1's row says "random loopback address" while other text says "unique loopback address".
- Solution: Order 1's row and the lost-coverage risk now say "unique loopback address", the EPIC Decisions' term, and add that it is picked at random (from `127.0.0.0/8` in the row).
- Current-tree verification: `rg -n 'unique loopback' docs/issues/open/2392-test-isolation/EPIC.md` prints order 1's row, the lost-coverage risk and the Decisions entry; the one remaining `random loopback address` is the Decisions entry describing what the helper returns.
- Resolution reference: docs(issues): [#2392] name the restored tests' loopback address the way the EPIC does
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2883#discussion_r4587850043

### F6 - The description does not say the history was rewritten

- PR number: 2883
- Source review ID: 5922953773
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/2883#discussion_r4588003679
- Concern: The F2 fix reworded the first commit, but the PR description did not say the history was rewritten, as the `open-pull-request` skill's history rules ask.
- Solution: The description now has a "History rewritten" note: the first commit's body was reworded for F2 with its tree unchanged, and the head `e12f3b0db` that the first round and its ACKs cite is that commit before the reword.
- Current-tree verification: `gh pr view 2883 --json body --jq .body | grep -c 'History rewritten'` prints `1`.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2883#discussion_r4588072127
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2883#discussion_r4588072127

### F7 - The resolved coverage risk gives every non-Linux system the same reason

- PR number: 2883
- Source review ID: 5922953773
- Reviewer finding ID: F5
- Source URL: https://github.com/torrust/torrust-tracker/pull/2883#discussion_r4588003691
- Concern: The resolved lost-coverage risk said every non-Linux system configures only `127.0.0.1`, while the #2393 spec gives that reason for macOS and the BSDs only and calls the ignore conservative.
- Solution: The risk now says the tests are conservatively ignored off Linux: macOS and the BSDs configure only `127.0.0.1`, and CI runs no tests off Linux.
- Current-tree verification: `rg -n 'conservatively ignored' docs/issues/open/2392-test-isolation/EPIC.md` prints the risk.
- Resolution reference: docs(issues): [#2392] give the restored tests' non-Linux ignore the spec's reason
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2883#discussion_r4588072641

### F8 - Two of the six stamps no longer read 13:54

- PR number: 2883
- Source review ID: 5922953773
- Reviewer finding ID: F6
- Source URL: https://github.com/torrust/torrust-tracker/pull/2883#discussion_r4588003702
- Concern: F3's Solution said all six stamps now say 13:54, but the `last-updated-utc` values of the #2393 spec and of the EPIC moved on with later fix commits.
- Solution: F3's Solution now says all six were set to 13:54 and that those two values moved on with later edits.
- Current-tree verification: F3's Solution in this record reads "All six were set to 13:54".
- Resolution reference: docs(pr-reviews): [#2393] record da2ce7's second-round findings in the PR 2883 audit
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2883#discussion_r4588073176

## Processing Log

- 2026-11-16 15:38 UTC - Started audit for reviews 5921522069 and 5921551645; F1 and F3 to F5 fixed in four commits, and F2 by rewording the first commit with the maintainer's approval.
- 2026-11-16 15:39 UTC - Replied on the five threads before pushing the fixes.
- 2026-11-16 15:57 UTC - Recorded review 5922953773 as F6 to F8, fixed them (F6 in the PR description, F7 and F8 in commits), and replied on the three threads before pushing.

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
