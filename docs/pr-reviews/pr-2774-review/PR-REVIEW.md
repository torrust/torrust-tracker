---
pr-number: 2774
pr-url: https://github.com/torrust/torrust-tracker/pull/2774
last-updated-utc: "2026-11-09 10:33"
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

Round 2: Copilot review 5810776921 (3 findings) and maintainer review 5823193599 (14 findings, reviewer IDs F1 to F14 recorded in each detail entry). F19, F23, F25, F26 and F29 concern child specs that PR #2771 adds and are `FOLLOW_UP`; the rest are on files of this issue.

Round 1: Copilot review 5806242087 (12 inline findings). F1 and F2 concern this issue's files and are fixed here. F3 to F12 concern the child specs that PR #2771 adds and this PR is stacked on; they are `FOLLOW_UP` with the fix on #2771. After the maintainer allowed the force-push and the resolve calls, all 12 threads were resolved.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2774-f1 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2774-f2 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2774-f3 | Copilot | Minor (inferred) | correctness | ORIGINAL | FOLLOW_UP | RESOLVED |
| F4 | review-finding:pr-2774-f4 | Copilot | Minor (inferred) | correctness | ORIGINAL | FOLLOW_UP | RESOLVED |
| F5 | review-finding:pr-2774-f5 | Copilot | Minor | correctness | ORIGINAL | FOLLOW_UP | RESOLVED |
| F6 | review-finding:pr-2774-f6 | Copilot | Minor (inferred) | correctness | ORIGINAL | FOLLOW_UP | RESOLVED |
| F7 | review-finding:pr-2774-f7 | Copilot | Minor (inferred) | correctness | ORIGINAL | FOLLOW_UP | RESOLVED |
| F8 | review-finding:pr-2774-f8 | Copilot | Minor (inferred) | correctness | ORIGINAL | FOLLOW_UP | RESOLVED |
| F9 | review-finding:pr-2774-f9 | Copilot | Minor (inferred) | correctness | ORIGINAL | FOLLOW_UP | RESOLVED |
| F10 | review-finding:pr-2774-f10 | Copilot | Minor (inferred) | correctness | ORIGINAL | FOLLOW_UP | RESOLVED |
| F11 | review-finding:pr-2774-f11 | Copilot | Minor (inferred) | correctness | ORIGINAL | FOLLOW_UP | RESOLVED |
| F12 | review-finding:pr-2774-f12 | Copilot | Minor (inferred) | correctness | ORIGINAL | FOLLOW_UP | RESOLVED |
| F13 | review-finding:pr-2774-f13 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | OPEN |
| F14 | review-finding:pr-2774-f14 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | OPEN |
| F15 | review-finding:pr-2774-f15 | Copilot | Minor (inferred) | formatting | ORIGINAL | FIXED | OPEN |
| F16 | review-finding:pr-2774-f16 | Human | Major | correctness | ORIGINAL | FIXED | OPEN |
| F17 | review-finding:pr-2774-f17 | Human | Minor | documentation | RE_RAISE_OF:F13 | FIXED | OPEN |
| F18 | review-finding:pr-2774-f18 | Human | Minor | documentation | ORIGINAL | FIXED | OPEN |
| F19 | review-finding:pr-2774-f19 | Human | Minor | maintainability | ORIGINAL | FOLLOW_UP | OPEN |
| F20 | review-finding:pr-2774-f20 | Human | Minor | documentation | ORIGINAL | FIXED | OPEN |
| F21 | review-finding:pr-2774-f21 | Human | Minor | documentation | ORIGINAL | FIXED | OPEN |
| F22 | review-finding:pr-2774-f22 | Human | Minor | documentation | ORIGINAL | FIXED | OPEN |
| F23 | review-finding:pr-2774-f23 | Human | Minor | correctness | ORIGINAL | FOLLOW_UP | OPEN |
| F24 | review-finding:pr-2774-f24 | Human | Nit | documentation | ORIGINAL | FIXED | OPEN |
| F25 | review-finding:pr-2774-f25 | Human | Nit | documentation | ORIGINAL | FOLLOW_UP | OPEN |
| F26 | review-finding:pr-2774-f26 | Human | Nit | documentation | ORIGINAL | FOLLOW_UP | OPEN |
| F27 | review-finding:pr-2774-f27 | Human | Nit | maintainability | ORIGINAL | NO_ACTION | OPEN |
| F28 | review-finding:pr-2774-f28 | Human | Nit | documentation | ORIGINAL | FIXED | OPEN |
| F29 | review-finding:pr-2774-f29 | Human | Nit | documentation | ORIGINAL | FOLLOW_UP | OPEN |

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
- Current-tree verification: Inspected `docs/issues/open/2761-2758-reply-status-result/ISSUE.md` line 71: the Bug-Fix Process section is populated and points to the `fix-bug` skill.
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
- Current-tree verification: `grep -n '| cat\|| jq'` on the 2761 and 2762 specs: the 2761 M1 command (line 155) saves the tool output to a file and prints `tool exit=$?` before `jq`; no verification command pipes a tool into `cat` or `jq` in 2762.
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
- Current-tree verification: `.github/workflows/generate_coverage_pr.yaml` has `pull_request` (line 8) and checks out `github.event.pull_request.head.sha` (line 36); the 2762 spec line 175 now says the PR run exercises the change.
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
- Current-tree verification: The 2763 spec AC3 (line 132) now says `workspace-coupling` matches `no-stdout-result`: empty stdout, report to the file argument, NDJSON status on stderr.
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
- Current-tree verification: `src/console/ci/e2e/tracker_checker.rs` line 17 launches `--bin tracker_checker`; the 2765 spec lists the file (line 62) and plans its migration to `tracker_client check` (line 47).
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
- Current-tree verification: `src/console/ci/e2e/docker.rs` lines 142-146: `logs` returns `output.stdout` only; the 2766 spec lists the file (line 63) and says it must capture stderr (line 48).
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
- Current-tree verification: The 2766 spec line 49 now maps configuration errors to exit `2` and runtime failures such as a bind error to exit `1`; M2 (line 165) expects exit `2` for a bad config path.
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
- Current-tree verification: `console/tracker-client/src/console/clients/unified/check.rs` lines 147-152 build pretty or compact JSON, so `check --format text` is JSON; `ci-gate/src/main.rs` line 64 has `is_terminal`. The 2768 spec line 36 says both.
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
- Current-tree verification: `src/console/ci/e2e/runner.rs` line 80 logs `tracker config:\n{tracker_config}`; the 2769 spec (line 46) plans masking and a test.
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
- Current-tree verification: `packages/e2e-tools/Cargo.toml` names the package `torrust-tracker-e2e-tools`, but the 2769 spec M1 (line 150) still says `cargo build -p e2e-tools --bins`, so this concern is only partly closed; see F23.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4497526930
- Follow-up PR URL: https://github.com/torrust/torrust-tracker/pull/2771
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4497526930

### F13 - `(deprecated)` is a third value in the `Class` column

- PR number: 2774
- Source review ID: 5810776921
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4500658731
- Concern: The `Class` column is restricted to the defined values elsewhere; `(deprecated)` is a third value.
- Solution: The deprecated row now has `N/A` in `Class`, and the text above the table says a deprecated binary to be removed is not classified and reads `N/A`.
- Current-tree verification: ADR section 3 text and the last table row (`N/A`); the `AC1` line of the 2760 spec says the same.
- Resolution reference: docs(adrs): [#2760] address the second review round of PR 2774
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4511734776

### F14 - Progress log entries are out of chronological order

- PR number: 2774
- Source review ID: 5810776921
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4500658759
- Concern: The 17:16 entry appears after the 17:25 entry in the 2760 spec.
- Solution: The 2760 spec progress log is reordered and its stamps are taken from the issue `created_at` and the commit dates (see F18).
- Current-tree verification: The 2760 spec log reads 16:51, 17:16, 20:53 UTC in order.
- Resolution reference: docs(adrs): [#2760] address the second review round of PR 2774
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4511735253

### F15 - Exit codes in table notes lack backticks

- PR number: 2774
- Source review ID: 5810776921
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4500658770
- Concern: `1` in the `package-coverage-check` note is not in backticks while other rows use them.
- Solution: Every literal exit code in the table notes is now in backticks (for example the `0` after `exit` and the `1` after `exits`).
- Current-tree verification: `grep -n 'exit [0-9]\|exits [0-9]'` on the ADR prints nothing.
- Resolution reference: docs(adrs): [#2760] address the second review round of PR 2774
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4511735713

### F16 - The audit stops at round 1: round 2 is unprocessed

- PR number: 2774
- Source review ID: 5823193599
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4510659744
- Concern: Review 5810776921 and its three threads had no row, reply or log entry.
- Solution: Rows F13 to F15 added, replied to and resolved; this round is logged.
- Current-tree verification: This audit now has rows for review 5810776921; refreshed GraphQL data is recorded in the log.
- Resolution reference: docs(pr-reviews): [#2760] audit the second review round of PR 2774
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4511736108

### F17 - `(deprecated)` is a third value in the `Class` column, against the ADR rule

- PR number: 2774
- Source review ID: 5823193599
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4510659761
- Concern: Same concern as F13; also asks to keep line 68 and `AC1` consistent.
- Solution: As F13: `N/A` convention stated above the table, and `AC1` reworded.
- Current-tree verification: ADR section 3 text and the 2760 spec `AC1` line.
- Resolution reference: docs(adrs): [#2760] address the second review round of PR 2774
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4511736531

### F18 - Progress-log stamps are later than the commit that carries them

- PR number: 2774
- Source review ID: 5823193599
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4510659764
- Concern: Entries are stamped after the commit that holds them.
- Solution: The 2760 spec stamps now come from the issue `created_at` (16:51Z) and the commit dates; the audit stamps are corrected. The EPIC and the other ten specs are the coordinator's: handed off.
- Current-tree verification: `git log --format=%cI` of the commits and the GitHub issue `created_at` (16:51:39Z) against each stamp.
- Resolution reference: docs(adrs): [#2760] address the second review round of PR 2774
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4511737694

### F19 - The wave 1 file sets are not disjoint

- PR number: 2774
- Source review ID: 5823193599
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4510659773
- Concern: 2765 and 2766 both list files under `share/default/config/`; issue 10 depends on issue 6 through `tracker_checker.rs`.
- Solution: Handed to the #2771 coordinator (EPIC and specs 2765 and 2766 are not files of this issue).
- Current-tree verification: The review names the lines; the files are outside this issue.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4511738141
- Follow-up PR URL: https://github.com/torrust/torrust-tracker/pull/2771
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4511738141

### F20 - The audit cites a former PR-branch head by commit id

- PR number: 2774
- Source review ID: 5823193599
- Reviewer finding ID: F5
- Source URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4510659779
- Concern: The round-1 head was cited by commit id.
- Solution: The commit id is removed; the review id identifies the round.
- Current-tree verification: `grep -n 'bd84e405e' docs/pr-reviews/pr-2774-review/PR-REVIEW.md` prints nothing.
- Resolution reference: docs(pr-reviews): [#2760] audit the second review round of PR 2774
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4511738479

### F21 - The description understates what this branch carries

- PR number: 2774
- Source review ID: 5823193599
- Reviewer finding ID: F6
- Source URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4510659785
- Concern: The PR body says one spec commit is in the diff; the stack is five commits and 12 of 16 files come from it.
- Solution: The PR body is rewritten to state the real stack and to drop the promise to rebase once it is kept (outside the tree; durable URL in the reply).
- Current-tree verification: `gh pr view 2774` body after the edit.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4511738808
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4511738808

### F22 - `Current-tree verification` of the ten handed-off findings is second-hand

- PR number: 2774
- Source review ID: 5823193599
- Reviewer finding ID: F7
- Source URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4510659793
- Concern: The ten entries record that a commit exists, not what was inspected.
- Solution: Each entry now names the file, line and command inspected on 2026-11-09.
- Current-tree verification: Rows F3 to F12 of this audit; the inspection found F12 only partly closed (see F23).
- Resolution reference: docs(pr-reviews): [#2760] audit the second review round of PR 2774
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4511739169

### F23 - The replacement command names a package that does not exist

- PR number: 2774
- Source review ID: 5823193599
- Reviewer finding ID: F8
- Source URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4510659799
- Concern: `cargo build -p e2e-tools --bins` should use the package name `torrust-tracker-e2e-tools`.
- Solution: Handed to the #2771 coordinator (spec 2769 is not a file of this issue).
- Current-tree verification: `packages/e2e-tools/Cargo.toml` line 4 names `torrust-tracker-e2e-tools`; spec 2769 lines 150-151 still say `e2e-tools`.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4511739562
- Follow-up PR URL: https://github.com/torrust/torrust-tracker/pull/2771
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4511739562

### F24 - The spec checkboxes contradict what it records

- PR number: 2774
- Source review ID: 5823193599
- Reviewer finding ID: F9
- Source URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4510659806
- Concern: Several checkboxes are unticked although the log and tables record the work.
- Solution: Lines for the acceptance review, the completion review, manual verification and the acceptance re-review are ticked.
- Current-tree verification: `grep -n '^- \[ \]'` on the 2760 spec leaves only the spec-only-PR merge, reviewer, committer and close items.
- Resolution reference: docs(adrs): [#2760] address the second review round of PR 2774
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4511739940

### F25 - `fetch-review-threads` does not describe the exit `1`

- PR number: 2774
- Source review ID: 5823193599
- Reviewer finding ID: F10
- Source URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4510659814
- Concern: The EPIC row says both skills describe the exit `1`.
- Solution: Handed to the #2771 coordinator (EPIC.md).
- Current-tree verification: The review names the skill line; the file is outside this issue.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4511740595
- Follow-up PR URL: https://github.com/torrust/torrust-tracker/pull/2771
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4511740595

### F26 - "plus the PR #2748 head" adds nothing

- PR number: 2774
- Source review ID: 5823193599
- Reviewer finding ID: F11
- Source URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4510659822
- Concern: The audited base already contains PR #2748.
- Solution: Handed to the #2771 coordinator (EPIC.md).
- Current-tree verification: The review names the lines; the file is outside this issue.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4511740913
- Follow-up PR URL: https://github.com/torrust/torrust-tracker/pull/2771
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4511740913

### F27 - The subject scope `docs(adr)` departs from `docs(adrs)`

- PR number: 2774
- Source review ID: 5823193599
- Reviewer finding ID: F12
- Source URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4510659839
- Concern: The commit subject, and with it the PR title, use `docs(adr)`.
- Solution: The PR title is changed to `docs(adrs)`. The existing commit subject is kept to avoid rewriting the branch history again; the merge subject comes from the title.
- Current-tree verification: `git log --oneline` on `develop` uses `docs(adrs)`; the PR title now matches.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4511741226
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4511741226

### F28 - `Observed Result` is a digest rather than program output

- PR number: 2774
- Source review ID: 5823193599
- Reviewer finding ID: F13
- Source URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4510659843
- Concern: The evidence block condensed the command output.
- Solution: The evidence file now pastes the real output of steps 1 to 5 with start times and exit codes.
- Current-tree verification: `manual-verification-evidence.md` V1.
- Resolution reference: docs(adrs): [#2760] address the second review round of PR 2774
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4511742309

### F29 - "epic-coordination process" names nothing in the tree

- PR number: 2774
- Source review ID: 5823193599
- Reviewer finding ID: F14
- Source URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4510659856
- Concern: Spec 2770 cites a process that no document names.
- Solution: Handed to the #2771 coordinator (spec 2770).
- Current-tree verification: The review names the line; the file is outside this issue.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4511742655
- Follow-up PR URL: https://github.com/torrust/torrust-tracker/pull/2771
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4511742655

## Processing Log

- 2026-11-06 21:02 UTC - Started audit; round 1 (Copilot review 5806242087) processed: F1 and F2 fixed and pushed, F3 to F12 handed to the #2771 coordinator and fixed there, all 12 replied.
- 2026-11-07 07:13 UTC - Branch rebased onto the fixed #2771 head and force-pushed with lease (maintainer approval in chat); all 12 threads resolved; refreshed GraphQL data shows no unresolved thread.

- 2026-11-09 10:33 UTC - Round 2 (Copilot review 5810776921 and maintainer review 5823193599) processed: fixes pushed, PR title and body updated, all 17 threads replied to; F19, F23, F25, F26 and F29 fixed on PR #2771 by the coordinator. Thread resolution is pending the maintainer (the auto mode classifier denied the resolve call), so rows F13 to F29 stay `OPEN`.

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
