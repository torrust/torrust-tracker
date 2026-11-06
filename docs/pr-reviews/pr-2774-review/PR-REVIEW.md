---
pr-number: 2774
pr-url: https://github.com/torrust/torrust-tracker/pull/2774
last-updated-utc: "2026-11-06 21:05"
---

# PR #2774 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2774.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository artifact obligation.

- Post-merge workflow approval: N/A

## Status Values

- Relationship: `ORIGINAL`, `RE_RAISE_OF:<FindingId>`
- Disposition: `FIXED`, `NO_ACTION`, `SUPERSEDED`, `FOLLOW_UP`
- Thread state: `OPEN`, `RESOLVED`, `NON_RESOLVABLE`, `SUPERSEDED`
- Severity: `Blocker`, `Major`, `Minor`, `Nit`, `Suggestion`; append `(inferred)` when derived
  from free prose.
- Author class: `Copilot`, `Human`, `Unknown`
- Category: `link-integrity`, `formatting`, `metadata`, `testing`, `correctness`,
  `documentation`, `maintainability`, `security`, `other`

## Findings

Round 1: Copilot review 5806242087 on the head `bd84e405e` (12 inline findings). F1 and F2 concern this issue's files and are fixed here. F3 to F12 concern the child specs that PR #2771 adds and this PR is stacked on; they are `FOLLOW_UP` with the fix on #2771. Thread resolution is pending the maintainer (the auto mode classifier denied the resolve call), so every thread is `OPEN`.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2774-f1 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | OPEN |
| F2 | review-finding:pr-2774-f2 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | OPEN |
| F3 | review-finding:pr-2774-f3 | Copilot | Minor (inferred) | correctness | ORIGINAL | FOLLOW_UP | OPEN |
| F4 | review-finding:pr-2774-f4 | Copilot | Minor (inferred) | correctness | ORIGINAL | FOLLOW_UP | OPEN |
| F5 | review-finding:pr-2774-f5 | Copilot | Minor | correctness | ORIGINAL | FOLLOW_UP | OPEN |
| F6 | review-finding:pr-2774-f6 | Copilot | Minor (inferred) | correctness | ORIGINAL | FOLLOW_UP | OPEN |
| F7 | review-finding:pr-2774-f7 | Copilot | Minor (inferred) | correctness | ORIGINAL | FOLLOW_UP | OPEN |
| F8 | review-finding:pr-2774-f8 | Copilot | Minor (inferred) | correctness | ORIGINAL | FOLLOW_UP | OPEN |
| F9 | review-finding:pr-2774-f9 | Copilot | Minor (inferred) | correctness | ORIGINAL | FOLLOW_UP | OPEN |
| F10 | review-finding:pr-2774-f10 | Copilot | Minor (inferred) | correctness | ORIGINAL | FOLLOW_UP | OPEN |
| F11 | review-finding:pr-2774-f11 | Copilot | Minor (inferred) | correctness | ORIGINAL | FOLLOW_UP | OPEN |
| F12 | review-finding:pr-2774-f12 | Copilot | Minor (inferred) | correctness | ORIGINAL | FOLLOW_UP | OPEN |

## Finding Details

### F1 - Completion review is missing before issue closure

- PR number: 2774
- Source review ID: 5806242087
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4497020548
- Concern: The spec says `Retrospective: Not yet assessed` while the PR closes #2760.
- Solution: Recorded the completion review and why no retrospective is needed.
- Current-tree verification: `ISSUE.md` Implementation Completion Review and the progress log entry now say so.
- Resolution reference: docs(issues): [#2760] record the implementation completion review
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4497522977

### F2 - Evidence file lacks required frontmatter and verification details

- PR number: 2774
- Source review ID: 5806242087
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4497020627
- Concern: The evidence file had no frontmatter and no performed-step/observed-result structure.
- Solution: Rewrote the file to the template with real commands and output.
- Current-tree verification: `manual-verification-evidence.md` has the frontmatter, V1 and V2 sections with steps and observed output.
- Resolution reference: docs(issues): [#2760] rewrite the manual verification evidence to the template
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4497523380

### F3 - Migration specs omit required bug-fix evidence

- PR number: 2774
- Source review ID: 5806242087
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4497020690
- Concern: 2761 ISSUE.md bug-fix sections say `Not applicable`. The file belongs to PR #2771, which this PR is stacked on, and is outside the files issue #2760 may touch.
- Solution: The coordinator session fixed it on PR #2771 in the commit `docs(issues): [#2758] address the Copilot review of PR 2774 on the child specs`; this branch was rebased onto that head.
- Current-tree verification: The commit is the parent of this PR's own commits after the rebase (`git log --oneline` shows it before the #2760 commits); the coordinator reported checking each finding against the code first.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4497523700
- Follow-up PR URL: https://github.com/torrust/torrust-tracker/pull/2771
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4497523700

### F4 - Pipeline masks producer exit status without pipefail

- PR number: 2774
- Source review ID: 5806242087
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4497020766
- Concern: 2762 and 2761 verification commands pipe the tool into `cat`/`jq`. The file belongs to PR #2771, which this PR is stacked on, and is outside the files issue #2760 may touch.
- Solution: The coordinator session fixed it on PR #2771 in the commit `docs(issues): [#2758] address the Copilot review of PR 2774 on the child specs`; this branch was rebased onto that head.
- Current-tree verification: The commit is the parent of this PR's own commits after the rebase (`git log --oneline` shows it before the #2760 commits); the coordinator reported checking each finding against the code first.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4497524097
- Follow-up PR URL: https://github.com/torrust/torrust-tracker/pull/2771
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4497524097

### F5 - Coverage workflow can verify the change before merge

- PR number: 2774
- Source review ID: 5806242087
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4497020834
- Concern: 2762 claims the workflow cannot run before merge. The file belongs to PR #2771, which this PR is stacked on, and is outside the files issue #2760 may touch.
- Solution: The coordinator session fixed it on PR #2771 in the commit `docs(issues): [#2758] address the Copilot review of PR 2774 on the child specs`; this branch was rebased onto that head.
- Current-tree verification: The commit is the parent of this PR's own commits after the rebase (`git log --oneline` shows it before the #2760 commits); the coordinator reported checking each finding against the code first.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4497524510
- Follow-up PR URL: https://github.com/torrust/torrust-tracker/pull/2771
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4497524510

### F6 - AC3 permits stdout JSON contrary to the no-stdout-result decision

- PR number: 2774
- Source review ID: 5806242087
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4497020895
- Concern: 2763 AC3 accepts a JSON stdout result for `workspace-coupling`. The file belongs to PR #2771, which this PR is stacked on, and is outside the files issue #2760 may touch.
- Solution: The coordinator session fixed it on PR #2771 in the commit `docs(issues): [#2758] address the Copilot review of PR 2774 on the child specs`; this branch was rebased onto that head.
- Current-tree verification: The commit is the parent of this PR's own commits after the rebase (`git log --oneline` shows it before the #2760 commits); the coordinator reported checking each finding against the code first.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4497524916
- Follow-up PR URL: https://github.com/torrust/torrust-tracker/pull/2771
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4497524916

### F7 - E2E caller still launches the binary slated for removal

- PR number: 2774
- Source review ID: 5806242087
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4497020967
- Concern: 2765 omits `src/console/ci/e2e/tracker_checker.rs`. The file belongs to PR #2771, which this PR is stacked on, and is outside the files issue #2760 may touch.
- Solution: The coordinator session fixed it on PR #2771 in the commit `docs(issues): [#2758] address the Copilot review of PR 2774 on the child specs`; this branch was rebased onto that head.
- Current-tree verification: The commit is the parent of this PR's own commits after the rebase (`git log --oneline` shows it before the #2760 commits); the coordinator reported checking each finding against the code first.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4497525284
- Follow-up PR URL: https://github.com/torrust/torrust-tracker/pull/2771
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4497525284

### F8 - Docker log helper drops daemon stderr

- PR number: 2774
- Source review ID: 5806242087
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4497021021
- Concern: 2766 omits `Docker::logs`, which returns only stdout. The file belongs to PR #2771, which this PR is stacked on, and is outside the files issue #2760 may touch.
- Solution: The coordinator session fixed it on PR #2771 in the commit `docs(issues): [#2758] address the Copilot review of PR 2774 on the child specs`; this branch was rebased onto that head.
- Current-tree verification: The commit is the parent of this PR's own commits after the rebase (`git log --oneline` shows it before the #2760 commits); the coordinator reported checking each finding against the code first.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4497525563
- Follow-up PR URL: https://github.com/torrust/torrust-tracker/pull/2771
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4497525563

### F9 - Scenario conflates configuration errors with runtime startup failures

- PR number: 2774
- Source review ID: 5806242087
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4497021093
- Concern: 2766 expects exit 1 for both invalid configuration and a busy port. The file belongs to PR #2771, which this PR is stacked on, and is outside the files issue #2760 may touch.
- Solution: The coordinator session fixed it on PR #2771 in the commit `docs(issues): [#2758] address the Copilot review of PR 2774 on the child specs`; this branch was rebased onto that head.
- Current-tree verification: The commit is the parent of this PR's own commits after the rebase (`git log --oneline` shows it before the #2760 commits); the coordinator reported checking each finding against the code first.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4497525833
- Follow-up PR URL: https://github.com/torrust/torrust-tracker/pull/2771
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4497525833

### F10 - Text formatting and TTY support are described wrongly

- PR number: 2774
- Source review ID: 5806242087
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4497021161
- Concern: 2768 calls `check --format text` plain text and overstates the missing TTY refusal. The file belongs to PR #2771, which this PR is stacked on, and is outside the files issue #2760 may touch.
- Solution: The coordinator session fixed it on PR #2771 in the commit `docs(issues): [#2758] address the Copilot review of PR 2774 on the child specs`; this branch was rebased onto that head.
- Current-tree verification: The commit is the parent of this PR's own commits after the rebase (`git log --oneline` shows it before the #2760 commits); the coordinator reported checking each finding against the code first.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4497526144
- Follow-up PR URL: https://github.com/torrust/torrust-tracker/pull/2771
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4497526144

### F11 - Tracker TOML logging can expose credentials

- PR number: 2774
- Source review ID: 5806242087
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4497021226
- Concern: 2769 does not cover masking of the logged tracker TOML. The file belongs to PR #2771, which this PR is stacked on, and is outside the files issue #2760 may touch.
- Solution: The coordinator session fixed it on PR #2771 in the commit `docs(issues): [#2758] address the Copilot review of PR 2774 on the child specs`; this branch was rebased onto that head.
- Current-tree verification: The commit is the parent of this PR's own commits after the rebase (`git log --oneline` shows it before the #2760 commits); the coordinator reported checking each finding against the code first.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4497526528
- Follow-up PR URL: https://github.com/torrust/torrust-tracker/pull/2771
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4497526528

### F12 - Cargo run output contaminates runners' NDJSON checks

- PR number: 2774
- Source review ID: 5806242087
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4497021273
- Concern: 2769 captures `cargo run` output, which includes cargo messages. The file belongs to PR #2771, which this PR is stacked on, and is outside the files issue #2760 may touch.
- Solution: The coordinator session fixed it on PR #2771 in the commit `docs(issues): [#2758] address the Copilot review of PR 2774 on the child specs`; this branch was rebased onto that head.
- Current-tree verification: The commit is the parent of this PR's own commits after the rebase (`git log --oneline` shows it before the #2760 commits); the coordinator reported checking each finding against the code first.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4497526930
- Follow-up PR URL: https://github.com/torrust/torrust-tracker/pull/2771
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4497526930

## Processing Log

- 2026-11-06 21:05 UTC - Started audit; round 1 (Copilot review 5806242087) processed: F1 and F2 fixed and pushed, F3 to F12 handed to the #2771 coordinator and fixed there, all 12 replied. Threads left open, awaiting the maintainer.

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- For an outdated thread whose concern was fixed, record `Disposition=FIXED` and
  `Thread state=RESOLVED`, even if GitHub marks the thread outdated after the push. For a
  duplicate, superseded, or no-change in-PR thread, reply exactly
  `Superseded by <FindingId>: <reason>.`, record `Disposition=NO_ACTION` and
  `Thread state=SUPERSEDED`, then resolve it. When the current identity cannot resolve a thread,
  still reply, keep the disposition established from the current tree, and record
  `Thread state=OPEN`; the reviewer or PR author resolves it.
- Cite a fix by its unique Conventional Commit subject or durable reply URL, never by a branch SHA
  that can change after a rebase.
- Refresh review threads using GraphQL and confirm that no unresolved actionable thread remains.
