---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/open/2867-udp-health-check-panics-on-invalid-response/ISSUE.md
---

<!-- skill-link: process-pr-review -->

# PR #2868 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2868>.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: <https://github.com/torrust/torrust-tracker/pull/2868#issuecomment-6912858266> (retrospective record, created 2026-11-16 09:41 UTC, of the maintainer's chat decision before the merge at 08:11 UTC to defer F7 and F8 to the implementation PR)

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

F1 comes from the first Copilot review (COMMENTED, Lite effort; Copilot noted its full agentic review did not start before its timeout), on the head ending in `docs(issues): [#2867] move the UDP health check panic spec to open`. F2 to F6 come from the first da2ce7 review (APPROVED, findings not blocking), on the head ending in `docs(pr-reviews): [#2867] log the resolution of the first PR 2868 review round`. The reviewer's own IDs F1 to F5 collide with the audit's F1, so they are kept as `Reviewer finding ID`.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2868-f1 | Copilot | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2868-f2 | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2868-f3 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2868-f4 | Human | Nit | correctness | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2868-f5 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2868-f6 | Human | Nit | correctness | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2868-f7 | Human | Minor | documentation | ORIGINAL | FOLLOW_UP | OPEN |
| F8 | review-finding:pr-2868-f8 | Human | Nit | documentation | ORIGINAL | FOLLOW_UP | OPEN |

## Finding Details

### F1 - UDP launcher missing from the spec's related artifacts

- PR number: 2868
- Source review ID: 5912844466
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2868#discussion_r4580164404>
- Concern: The spec cites `packages/udp-server/src/server/launcher.rs` in its step-by-step explanation and References, but its `semantic-links.related-artifacts` did not list it. Copilot marked it low severity.
- Solution: Added `packages/udp-server/src/server/launcher.rs` to `semantic-links.related-artifacts`, next to the other code files the spec cites.
- Current-tree verification: `grep -n "launcher.rs" docs/issues/open/2867-udp-health-check-panics-on-invalid-response/ISSUE.md` prints the new frontmatter line and the existing body mentions.
- Resolution reference: docs(issues): [#2867] list the UDP launcher among the spec's related artifacts
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2868#discussion_r4580176772>

### F2 - AC2 has no planned maintained test

- PR number: 2868
- Source review ID: 5913009678
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2868#discussion_r4580308600>
- Concern: `fix-bug` asks that the acceptance criteria match the planned tests. AC2 (a junk-answering UDP service through `GET /health_check`) was covered only by the manual M3 recheck, and the spec kept the V1 test only "if the reviewer wants". The reviewer asked to keep it.
- Solution: The Regression Test Strategy now has a third maintained test, the V1 test kept as a UDP contract test, as plan row T3. It comes before the `check` fix (now T4), because that fix alone turns it green; its red run is the V1 output. AC1, AC2 and AC3 name their tests (T2, T3, T5), and the plan and commit points are renumbered.
- Current-tree verification: `grep -n "test T3" docs/issues/open/2867-udp-health-check-panics-on-invalid-response/ISSUE.md` prints the AC2 line; the plan table has rows T1 to T7.
- Resolution reference: docs(issues): [#2867] plan the junk-reply UDP contract test that covers AC2
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2868#discussion_r4580332848>

### F3 - PR description no longer matches the branch

- PR number: 2868
- Source review ID: 5913009678
- Reviewer finding ID: F2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2868#discussion_r4580308604>
- Concern: The PR description said the pre-commit hook passes "on both commits", but the branch has more, and its Files list did not name this audit record. The merge tool copies the description into the merge commit.
- Solution: Edited the PR description. Its Files list names this audit record and the V3 evidence files. Its Validation section says the hook passes on every commit of the branch and adds the V3 rerun. Its Summary plans three regression tests.
- Current-tree verification: `gh-bot pr view 2868 --repo torrust/torrust-tracker --json body` shows the `PR-REVIEW.md` line and "on every commit of the branch".
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2868#issuecomment-6901716295>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2868#discussion_r4580332743>

### F4 - Not every await in `check` is bounded by the 2 s sleep

- PR number: 2868
- Source review ID: 5913009678
- Reviewer finding ID: F3
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2868#discussion_r4580308611>
- Concern: Only the receive races the 2 s `select!` sleep; bind, connect and send are bounded by the 5 s `DEFAULT_UDP_TIMEOUT`. The conclusion holds, but the spec named only one deadline.
- Solution: The Design and Ownership Review now names both deadlines.
- Current-tree verification: `grep -n "DEFAULT_UDP_TIMEOUT" docs/issues/open/2867-udp-health-check-panics-on-invalid-response/ISSUE.md` prints the new sentence.
- Resolution reference: docs(issues): [#2867] name both deadlines that bound the awaits in check
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2868#discussion_r4580332951>

### F5 - V1 ran with only the junk test applied

- PR number: 2868
- Source review ID: 5913009678
- Reviewer finding ID: F4
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2868#discussion_r4580308614>
- Concern: The V1 log reports `11 filtered out` and the V2 log `12 filtered out`, so V1 ran before the closed-port test existed, while the stored diff adds both. The closed-port test's copied comment says the peer answers with junk bytes.
- Solution: V1 now says it ran with only the junk test added, and that the stored diff is what V2 ran, with the V1 test byte for byte as its first test. The copied comment is not edited inside `evidence/reproduction-tests.diff`, which records what V2 ran (an edit would also break its index line); V2 names the wrong comment, and the V3 diff has it corrected. The reply states this deviation.
- Current-tree verification: `grep -n "11 filtered out" docs/issues/open/2867-udp-health-check-panics-on-invalid-response/manual-verification-evidence.md` prints the V1 step; `grep -n "nothing listens" docs/issues/open/2867-udp-health-check-panics-on-invalid-response/evidence/closed-port-value-check.diff` prints the corrected comment.
- Resolution reference: docs(issues): [#2867] state which temporary tests V1 and V2 ran with
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2868#discussion_r4580333067>

### F6 - V2 shows an error result, not the `Timed Out` text

- PR number: 2868
- Source review ID: 5913009678
- Reviewer finding ID: F5
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2868#discussion_r4580308617>
- Concern: The V2 test asserted only `is_err()`, so the `Timed Out` value in V2, in ISSUE.md and in the M2 row was inferred.
- Solution: Reran the closed-port test asserting `Err("Timed Out")` on the same base and toolchain (new V3, passed in 3.01 s). V2 says it only asserted `is_err()`; the ISSUE.md Reproduction paragraph and the M2 row cite V3, and V3 cites the maintained stopped-UDP contract test. The planned M3 evidence section moved from V3 to V4.
- Current-tree verification: `grep -n "Timed Out" docs/issues/open/2867-udp-health-check-panics-on-invalid-response/evidence/closed-port-value-check.diff` prints the `assert_eq!`; `evidence/closed-port-value-check-run.log` shows `1 passed`.
- Resolution reference: docs(issues): [#2867] assert the Timed Out value of the closed-port check
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2868#discussion_r4580333203>

### F7 - The V3 rebuild rests on a build line that no stored file carries

- PR number: 2868
- Source review ID: 5915664909
- Reviewer finding ID: F6
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2868#discussion_r4582464465>
- Concern: The stored V1 to V3 logs start at `Finished`, so no kept file shows the `Compiling` line that V3 and the F6 Solution cite as proof that the value assertion was rebuilt.
- Solution: Deferred by the maintainer to the implementation PR. There, `manual-verification-evidence.md` says that the stored V1 to V3 logs start at `Finished` and that the V3 rebuild was not kept, and points to the maintained stopped-UDP test run whose full log is kept; every new log keeps its build lines. The F6 Solution above no longer claims the rebuild.
- Current-tree verification: on the follow-up branch, `grep -n "Finished" docs/issues/open/2867-udp-health-check-panics-on-invalid-response/manual-verification-evidence.md` prints the statements about the V1 to V3 logs and the V3 rebuild.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2868#discussion_r4583253808>
- Follow-up PR URL: <https://github.com/torrust/torrust-tracker/pull/2879>
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2868#discussion_r4583253808>

### F8 - The artifact under test still names only the V1/V2 diff

- PR number: 2868
- Source review ID: 5915664909
- Reviewer finding ID: F7
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2868#discussion_r4582464475>
- Concern: V3 applied `evidence/closed-port-value-check.diff`, but the artifact-under-test line names only `evidence/reproduction-tests.diff`.
- Solution: Deferred by the maintainer to the implementation PR, where the line names `evidence/reproduction-tests.diff` (V1, V2) and `evidence/closed-port-value-check.diff` (V3).
- Current-tree verification: on the follow-up branch, `grep -n "(V3) applied" docs/issues/open/2867-udp-health-check-panics-on-invalid-response/manual-verification-evidence.md` prints the artifact-under-test line.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2868#discussion_r4583254064>
- Follow-up PR URL: <https://github.com/torrust/torrust-tracker/pull/2879>
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2868#discussion_r4583254064>

## Processing Log

- 2026-11-15 20:47 UTC - Started audit. Copilot reviewed the head ending in `docs(issues): [#2867] move the UDP health check panic spec to open` at 20:43:53Z (COMMENTED, one inline thread, F1). Fixed F1 in its own commit, replied on the thread, and recorded spec-only PR 2868 in the spec frontmatter in a separate commit.
- 2026-11-15 21:34 UTC - da2ce7 reviewed the head ending in `docs(pr-reviews): [#2867] log the resolution of the first PR 2868 review round` at 21:27:05Z (APPROVED, five inline threads, none blocking), recorded as F2 to F6. Each in-tree finding was fixed in its own commit; F3 was fixed by editing the PR description. Replied on all five threads before the push.
- 2026-11-16 09:58 UTC - da2ce7 reviewed the head ending in `docs(pr-reviews): [#2867] log the resolution of the first da2ce7 review of PR 2868` at 06:01:56Z (APPROVED, two inline threads, none blocking), recorded as F7 and F8 because the reviewer's F6 collides with the audit's F6. The maintainer chose to merge as is (merged 08:11:29Z) and defer both to the implementation PR 2879. Replied on both threads, recorded the approval on PR 2868, and reopened both threads, which stay `OPEN` until PR 2879 merges.

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
