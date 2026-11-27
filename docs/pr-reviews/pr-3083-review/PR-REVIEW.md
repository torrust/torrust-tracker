---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #3083 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/3083.

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

Review 6084545579 (`claude[bot]`) has no finding: it only says that automated review is disabled for a pull request from a fork.

| Finding ID | Review finding reference | Reviewer | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | -------- | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-3083-f1 | coderabbitai[bot] | Minor | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-3083-f2 | coderabbitai[bot] | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-3083-f3 | coderabbitai[bot] | Minor | correctness | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The ADR links the folder of the issue, whose path changes when the issue is closed

- PR number: 3083
- Source review ID: 6084598281
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3083#discussion_r4716532325
- Concern: The frontmatter of the ADR listed `docs/issues/open/2766-2758-daemon-json-logging/ISSUE.md` and `decisions.md`, and the body linked `decisions.md`; closing the issue moves the folder to `docs/issues/closed/` and the paths break. The reviewer cites the guideline to prefer the issue number.
- Solution: The two paths are removed from the frontmatter, and the body says that `decisions.md` stays with the specification of issue #2766 and that its path changes when the issue is closed, so it is not linked. This follows the stability warning of `docs/skills/semantic-skill-link-convention.md`.
- Current-tree verification: `grep -c "docs/issues" docs/adrs/20261127063102_write_the_daemon_logs_as_json_on_stderr_by_default.md` prints 0.
- Resolution reference: docs(adrs): [#2766] refer to the issue by number in the ADR of the daemon logs
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3083#discussion_r4716571391

### F2 - The reproduction setup of R0b, R0c and R1 is not recorded

- PR number: 3083
- Source review ID: 6084598281
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3083#discussion_r4716532334
- Concern: R0b and R0c need a `tracker.toml`, and R0c a listener on port 17071; the evidence has neither the configuration nor the listener command, so a reviewer cannot repeat the baseline or the recheck.
- Solution: A subsection "Setup of R0b, R0c and R1" holds the configuration (with the scratch directory as a placeholder), how the second configuration and `off.toml` differ, the commands that start the tracker and send the `SIGTERM`, the one-line listener and a note that nothing needs cleaning up.
- Current-tree verification: `grep -n "Setup of R0b, R0c and R1" docs/issues/open/2766-2758-daemon-json-logging/manual-verification-evidence.md` finds the heading before "Verification Processes".
- Resolution reference: docs(issues): [#2766] record the setup of the reproductions of the daemon logs
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3083#discussion_r4716571634

### F3 - R1 says the exit codes are unchanged, but the baseline exit code of the normal run was not recorded

- PR number: 3083
- Source review ID: 6084598281
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3083#discussion_r4716532339
- Concern: The R1 paragraph claimed the exit codes match R0b and R0c, including `0` after `SIGTERM`, while R0b says its exit code was not recorded.
- Solution: The claim is limited to the bind failure (`1` in R0c and in the recheck). The paragraph now says the normal run was not compared, that the recheck records `0` after `SIGTERM`, and that the lifecycle test `lifecycle-signals` asserts exit code `0` after `SIGTERM` and passes on the final tree.
- Current-tree verification: `grep -n "does not cover it" docs/issues/open/2766-2758-daemon-json-logging/manual-verification-evidence.md` finds the sentence; `grep -c "same exit codes"` on the same file prints 0.
- Resolution reference: docs(issues): [#2766] limit the claim of unchanged exit codes to the one that was recorded
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3083#discussion_r4716571879

## Processing Log

- 2026-11-27 06:53 UTC - Review 6084545579 (`claude[bot]`) arrived; it has no finding.
- 2026-11-27 07:00 UTC - CodeRabbit's review 6084598281 arrived with F1 to F3, on the first head.
- 2026-11-27 07:06 UTC - Pushed the three fix commits.
- 2026-11-27 07:06 UTC - Replied on the three threads of F1 to F3.
- 2026-11-27 07:07 UTC - A GraphQL listing showed the three threads resolved, each with my reply as its last comment.
- 2026-11-27 07:07 UTC - Created this record from the template. It was started after the replies were posted, not before the first reply as `process-pr-review` requires; the late start is logged here.

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
