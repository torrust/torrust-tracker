---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2858 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2858.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: NA (not merged)

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

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2858-f1 | Human | Major | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2858-f2 | Human | Major | metadata | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2858-f3 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2858-f4 | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2858-f5 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2858-f6 | Human | Nit | formatting | ORIGINAL | FIXED | RESOLVED |
| C1 | review-finding:pr-2858-c1 | Copilot | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2858-f7 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2858-f8 | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-2858-f9 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - HTTP `Error` doc describes removed variants

- PR number: 2858
- Source review ID: 5899149477
- Source URL: https://github.com/torrust/torrust-tracker/pull/2858#discussion_r4569554532
- Concern: The doc comment of the HTTP `Error` still listed startup-notification and stop failures.
- Solution: Replaced it with a doc naming only the startup errors the remaining variants carry.
- Current-tree verification: Read `packages/axum-http-server/src/server.rs` doc of `Error`.
- Resolution reference: fix(servers): [#2844] update Error docs and verification evidence after review
- Follow-up PR URL: NA
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2858#discussion_r4569601018

### F2 - PR title is not a Conventional Commits line

- PR number: 2858
- Source review ID: 5899149477
- Source URL: https://github.com/torrust/torrust-tracker/pull/2858#discussion_r4569554534
- Concern: The title had no `type(scope):`.
- Solution: Retitled to `refactor(servers): [#2844] remove unused public shutdown error variants`.
- Current-tree verification: `gh pr view 2858` title.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2858#discussion_r4569601682
- Follow-up PR URL: NA
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2858#discussion_r4569601682

### F3 - REST `Error` doc says "or stopping"

- PR number: 2858
- Source review ID: 5899149477
- Source URL: https://github.com/torrust/torrust-tracker/pull/2858#discussion_r4569554539
- Concern: The doc still said starting or stopping.
- Solution: Reworded to starting only.
- Current-tree verification: Read `packages/axum-rest-api-server/src/server.rs`.
- Resolution reference: fix(servers): [#2844] update Error docs and verification evidence after review
- Follow-up PR URL: NA
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2858#discussion_r4569601364

### F4 - M2 and M3 evidence lacks commands and output

- PR number: 2858
- Source review ID: 5899149477
- Source URL: https://github.com/torrust/torrust-tracker/pull/2858#discussion_r4569554544
- Concern: Evidence recorded results without commands, stamps or output; "five listeners" instead of six.
- Solution: Reran M2 and M3 and rewrote the evidence with commands, stamps, six listeners, bind error and toolchain.
- Current-tree verification: Read `manual-verification-evidence.md`.
- Resolution reference: fix(servers): [#2844] update Error docs and verification evidence after review
- Follow-up PR URL: NA
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2858#discussion_r4569601931

### F5 - `related-pr` still null

- PR number: 2858
- Source review ID: 5899149477
- Source URL: https://github.com/torrust/torrust-tracker/pull/2858#discussion_r4569554552
- Concern: Frontmatter lacked the implementation PR.
- Solution: Set `related-pr: 2858`.
- Current-tree verification: Read ISSUE.md frontmatter.
- Resolution reference: fix(servers): [#2844] update Error docs and verification evidence after review
- Follow-up PR URL: NA
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2858#discussion_r4569602197

### F6 - Progress-log entry after a blank line

- PR number: 2858
- Source review ID: 5899149477
- Source URL: https://github.com/torrust/torrust-tracker/pull/2858#discussion_r4569554555
- Concern: The new entry sat after the blank line, making a loose list.
- Solution: Moved it into the tight list.
- Current-tree verification: Read the Progress Log.
- Resolution reference: fix(servers): [#2844] update Error docs and verification evidence after review
- Follow-up PR URL: NA
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2858#discussion_r4569602416

### C1 - Document runtime verification commands and output

- PR number: 2858
- Source review ID: 5899177819
- Source URL: https://github.com/torrust/torrust-tracker/pull/2858#discussion_r4569573991
- Concern: Same concern as F4, reviewed at the earlier head.
- Solution: Already fixed with F4; replied and resolved.
- Current-tree verification: Read `manual-verification-evidence.md`.
- Resolution reference: fix(servers): [#2844] update Error docs and verification evidence after review
- Follow-up PR URL: NA
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2858#discussion_r4569608127

### F7 - No review audit record at this head

- PR number: 2858
- Source review ID: 5899281528
- Source URL: https://github.com/torrust/torrust-tracker/pull/2858#discussion_r4569656048
- Concern: `PR-REVIEW.md` was missing although replies were posted.
- Solution: Created this record from the template, logged the late start; replies citing a branch commit id map to `fix(servers): [#2844] update Error docs and verification evidence after review`.
- Current-tree verification: This file exists.
- Resolution reference: docs(issues): [#2844] add review audit record and tighten verification evidence
- Follow-up PR URL: NA
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2858#discussion_r4569668175

### F8 - M2 block shows edited output as command output

- PR number: 2858
- Source review ID: 5899281528
- Source URL: https://github.com/torrust/torrust-tracker/pull/2858#discussion_r4569656058
- Concern: Trimmed output and stamps were presented as verbatim.
- Solution: Stated above the block what is trimmed and how stamps were taken; listed the two HTTP log lines separately.
- Current-tree verification: Read `manual-verification-evidence.md`.
- Resolution reference: docs(issues): [#2844] add review audit record and tighten verification evidence
- Follow-up PR URL: NA
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2858#discussion_r4569668450

### F9 - `last-updated-utc` not moved

- PR number: 2858
- Source review ID: 5899281528
- Source URL: https://github.com/torrust/torrust-tracker/pull/2858#discussion_r4569656070
- Concern: The stamp lagged the new log entry.
- Solution: Moved to 23:10.
- Current-tree verification: Read ISSUE.md frontmatter.
- Resolution reference: docs(issues): [#2844] add review audit record and tighten verification evidence
- Follow-up PR URL: NA
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2858#discussion_r4569668697

## Processing Log

- 2026-11-13 22:49 UTC - First review (da2ce7) arrived; replies to F1 to F6 posted from 22:55 UTC without this record.
- 2026-11-13 23:06 UTC - Late start: record created after the round-2 review (F7); all threads answered and resolved.

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
