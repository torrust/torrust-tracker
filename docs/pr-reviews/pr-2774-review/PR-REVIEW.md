---
pr-number: 2774
pr-url: https://github.com/torrust/torrust-tracker/pull/2774
last-updated-utc: "2026-11-09 12:11"
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

Round 4: maintainer review 5825509686 (4 findings, reviewer IDs recorded in each entry); all on files of this issue.

Round 3: da2ce7 reviews 5824868731 and 5825123886 (16 findings, reviewer IDs recorded in each entry) and Copilot review 5825128396 (5 findings). F43, F46, F49 and F50 concern child specs that PR #2771 adds and are `FOLLOW_UP`; F45 is dropped by the planned rebase; the rest are on files of this issue.

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
| F13 | review-finding:pr-2774-f13 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F14 | review-finding:pr-2774-f14 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F15 | review-finding:pr-2774-f15 | Copilot | Minor (inferred) | formatting | ORIGINAL | FIXED | RESOLVED |
| F16 | review-finding:pr-2774-f16 | Human | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F17 | review-finding:pr-2774-f17 | Human | Minor | documentation | RE_RAISE_OF:F13 | FIXED | RESOLVED |
| F18 | review-finding:pr-2774-f18 | Human | Minor | documentation | ORIGINAL | FOLLOW_UP | RESOLVED |
| F19 | review-finding:pr-2774-f19 | Human | Minor | maintainability | ORIGINAL | FOLLOW_UP | RESOLVED |
| F20 | review-finding:pr-2774-f20 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F21 | review-finding:pr-2774-f21 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F22 | review-finding:pr-2774-f22 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F23 | review-finding:pr-2774-f23 | Human | Minor | correctness | ORIGINAL | FOLLOW_UP | RESOLVED |
| F24 | review-finding:pr-2774-f24 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F25 | review-finding:pr-2774-f25 | Human | Nit | documentation | ORIGINAL | FOLLOW_UP | RESOLVED |
| F26 | review-finding:pr-2774-f26 | Human | Nit | documentation | ORIGINAL | FOLLOW_UP | RESOLVED |
| F27 | review-finding:pr-2774-f27 | Human | Nit | maintainability | ORIGINAL | FIXED | RESOLVED |
| F28 | review-finding:pr-2774-f28 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F29 | review-finding:pr-2774-f29 | Human | Nit | documentation | ORIGINAL | FOLLOW_UP | RESOLVED |
| F30 | review-finding:pr-2774-f30 | Human | Minor | documentation | RE_RAISE_OF:F18 | FIXED | RESOLVED |
| F31 | review-finding:pr-2774-f31 | Human | Minor | documentation | RE_RAISE_OF:F20 | FIXED | RESOLVED |
| F32 | review-finding:pr-2774-f32 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F33 | review-finding:pr-2774-f33 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F34 | review-finding:pr-2774-f34 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F35 | review-finding:pr-2774-f35 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F36 | review-finding:pr-2774-f36 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F37 | review-finding:pr-2774-f37 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F38 | review-finding:pr-2774-f38 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F39 | review-finding:pr-2774-f39 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F40 | review-finding:pr-2774-f40 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F41 | review-finding:pr-2774-f41 | Human | Minor | correctness | RE_RAISE_OF:F33 | FIXED | RESOLVED |
| F42 | review-finding:pr-2774-f42 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F43 | review-finding:pr-2774-f43 | Human | Minor | documentation | ORIGINAL | FOLLOW_UP | RESOLVED |
| F44 | review-finding:pr-2774-f44 | Human | Minor | documentation | RE_RAISE_OF:F21 | FIXED | RESOLVED |
| F45 | review-finding:pr-2774-f45 | Human | Nit | maintainability | ORIGINAL | FOLLOW_UP | OPEN |
| F46 | review-finding:pr-2774-f46 | Copilot | Minor | correctness | ORIGINAL | FOLLOW_UP | RESOLVED |
| F47 | review-finding:pr-2774-f47 | Copilot | Minor (inferred) | documentation | RE_RAISE_OF:F37 | FIXED | RESOLVED |
| F48 | review-finding:pr-2774-f48 | Copilot | Minor (inferred) | documentation | RE_RAISE_OF:F34 | FIXED | RESOLVED |
| F49 | review-finding:pr-2774-f49 | Copilot | Minor | testing | ORIGINAL | FOLLOW_UP | RESOLVED |
| F50 | review-finding:pr-2774-f50 | Copilot | Minor | correctness | RE_RAISE_OF:F10 | FOLLOW_UP | RESOLVED |
| F51 | review-finding:pr-2774-f51 | Human | Minor | correctness | RE_RAISE_OF:F32 | FIXED | RESOLVED |
| F52 | review-finding:pr-2774-f52 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F53 | review-finding:pr-2774-f53 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F54 | review-finding:pr-2774-f54 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

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
- Current-tree verification: At the time of the first inspection the 2761 M1 command saved the tool output to a file and printed `tool exit=$?` before `jq`. The merge of `develop` then took `develop`'s M1 row, which pipes the tool into `jq -e` again; the pipefail-safe command lives on PR #2771 and is the coordinator's to reconcile there. No verification command in the 2762 spec pipes a tool into `cat` or `jq`.
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
- Solution: The 2760 spec stamps and the audit stamps are corrected in this PR; the stamps of the EPIC and the other ten specs are PR #2771's files and are handed to the coordinator (disposition `FOLLOW_UP` for that part).
- Current-tree verification: `git log --format=%cI` of the commits and the GitHub issue `created_at` (16:51:39Z) against each stamp.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4511737694
- Follow-up PR URL: https://github.com/torrust/torrust-tracker/pull/2771
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4511737694

### F19 - The wave 1 file sets are not disjoint

- PR number: 2774
- Source review ID: 5823193599
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4510659773
- Concern: 2765 and 2766 both list files under `share/default/config/`; issue 10 depends on issue 6 through `tracker_checker.rs`.
- Solution: Handed to the #2771 coordinator (EPIC and specs 2765 and 2766 are not files of this issue).
- Current-tree verification: Fetched the PR #2771 branch on 2026-11-09 and read `git show` of its files: EPIC.md rows 103 and 104 now say wave 2, `after 6` and `after 6 and 7`; the 2766 spec line 64 excludes `share/default/config/tracker_checker.json`, which 2765 owns.
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
- Current-tree verification: Searched this audit for a former PR-branch commit id with a hash-like pattern: the round-1 sentence cites only the review id, and no verification quotes a commit id.
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
- Current-tree verification: `grep -n '^- \[ \]'` on the 2760 spec lists six unticked boxes: the spec-only PR merge, `Relevant tests pass` (a docs-only change has no test to run), `Documentation is updated when behavior or workflow changes`, the reviewer validation, the committer check and the issue close. The first, fourth, fifth and sixth belong to the process after review; the other two are left for the reviewer to judge.
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
- Current-tree verification: `git show` of the #2771 branch EPIC.md line 49: it now says `resolve-review-threads` describes the exit `1` and `fetch-review-threads` describes the tool and its exit text.
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
- Current-tree verification: `git show` of the #2771 branch EPIC.md: `grep -c 'PR #2748 head'` prints `0`.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4511740913
- Follow-up PR URL: https://github.com/torrust/torrust-tracker/pull/2771
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4511740913

### F27 - The subject scope `docs(adr)` departs from `docs(adrs)`

- PR number: 2774
- Source review ID: 5823193599
- Reviewer finding ID: F12
- Source URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4510659839
- Concern: The commit subject, and with it the PR title, use `docs(adr)`.
- Solution: The PR title is changed to `docs(adrs)`, which is what the finding asked for and becomes the merge subject; the fix is outside the tree. The existing commit subject is kept to avoid rewriting the branch history again.
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
- Current-tree verification: `git show` of the #2771 branch 2770 spec: `grep -n 'epic-coordination'` prints nothing.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4511742655
- Follow-up PR URL: https://github.com/torrust/torrust-tracker/pull/2771
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4511742655

### F30 - Re-raise: the stamps are corrected in part, and two of the new ones repeat the defect

- PR number: 2774
- Source review ID: 5824868731
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4511946371
- Concern: Two new stamps (spec `last-updated-utc` and the evidence file) are later than the commit that carries them; row F18 is `FIXED` while its entry hands part of the work off; the audit's first entry was rewritten in place.
- Solution: The spec and evidence `last-updated-utc` and log stamps now come from `git log`; the first audit entry is restored to its original text with a correction entry appended; row F18 is `FOLLOW_UP` for the part on PR #2771.
- Current-tree verification: `git log --format=%cI` of the commit that carries each stamp against the stamp; `git diff` of the audit log shows the restored entry.
- Resolution reference: docs(adrs): [#2760] address the third review round of PR 2774
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4512236087

### F31 - Re-raise: the former head's id is back on this line, and this grep does not print nothing

- PR number: 2774
- Source review ID: 5824868731
- Reviewer finding ID: F5
- Source URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4511946379
- Concern: The verification of row F20 quotes the former head's commit id.
- Solution: The verification no longer quotes an id; it describes the check.
- Current-tree verification: Row F20 of this audit; no verification quotes a commit id.
- Resolution reference: docs(adrs): [#2760] address the third review round of PR 2774
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4512236435

### F32 - The branch no longer merges into `develop`: this row conflicts

- PR number: 2774
- Source review ID: 5824868731
- Reviewer finding ID: F15
- Source URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4511946385
- Concern: The 2761 spec M1 row conflicted with `develop`.
- Solution: Merged `develop` into the branch; the conflicting rows take `develop`'s version.
- Current-tree verification: `git ls-files -u` prints nothing after the merge; `gh pr view 2774 --json mergeable` reports `MERGEABLE`.
- Resolution reference: Merge torrust/develop into 2760-adr-binary-table
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4512236778

### F33 - This note is false on arrival: `reply-status` no longer exits `1` for a missing reply

- PR number: 2774
- Source review ID: 5824868731
- Reviewer finding ID: F16
- Source URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4511946389
- Concern: The ADR row claims `reply-status` is non-compliant after #2775 fixed it.
- Solution: The clause and the #2761 reference are dropped from the row; it now says a missing reply is a result (exit `0`).
- Current-tree verification: `grep -n 'emit_json' contrib/dev-tools/github/github-review-threads/src/main.rs` shows the result path; the test `it_should_report_the_threads_without_a_reply_as_a_result_with_exit_zero` in `tests/cli.rs` covers it.
- Resolution reference: docs(adrs): [#2760] address the third review round of PR 2774
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4512237115

### F34 - This run is recorded nowhere

- PR number: 2774
- Source review ID: 5824868731
- Reviewer finding ID: F17
- Source URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4511946400
- Concern: The evidence file points at a run that the spec progress log does not record.
- Solution: The spec progress log now has an entry with the command, time and result of the 2026-11-09 run.
- Current-tree verification: `ISSUE.md` progress log of the 2760 spec.
- Resolution reference: docs(adrs): [#2760] address the third review round of PR 2774
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4512237419

### F35 - Four round-2 entries repeat what F7 raised: no inspection is recorded

- PR number: 2774
- Source review ID: 5824868731
- Reviewer finding ID: F21
- Source URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4511946405
- Concern: Rows F19, F25, F26 and F29 record no inspection.
- Solution: Each of the four entries now names what was read on the PR #2771 branch.
- Current-tree verification: Rows F19, F25, F26 and F29 of this audit.
- Resolution reference: docs(adrs): [#2760] address the third review round of PR 2774
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4512237711

### F36 - An earlier commit rewrote this log entry in place

- PR number: 2774
- Source review ID: 5824868731
- Reviewer finding ID: F22
- Source URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4511946410
- Concern: The round-2 log entry was rewritten after it was committed.
- Solution: The original sentence is restored and the later entry records what followed.
- Current-tree verification: `git show` of the earlier commit of the audit against the restored line.
- Resolution reference: docs(adrs): [#2760] address the third review round of PR 2774
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4512238061

### F37 - The stamps below contradict "in this order"

- PR number: 2774
- Source review ID: 5824868731
- Reviewer finding ID: F18
- Source URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4511946413
- Concern: Step 1 started after steps 2 to 5.
- Solution: The evidence file says step 1 was rerun at 10:26:01 after steps 2 to 5 at 10:24:28.
- Current-tree verification: `manual-verification-evidence.md` Steps Performed.
- Resolution reference: docs(adrs): [#2760] address the third review round of PR 2774
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4512238432

### F38 - This grep prints two lines more than the sentence lists

- PR number: 2774
- Source review ID: 5824868731
- Reviewer finding ID: F19
- Source URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4511946422
- Concern: Two more unticked boxes were not listed.
- Solution: Row F24 lists all six and says why each stays open.
- Current-tree verification: Row F24 of this audit.
- Resolution reference: docs(adrs): [#2760] address the third review round of PR 2774
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4512238835

### F39 - `NO_ACTION` does not match what was done for this finding

- PR number: 2774
- Source review ID: 5824868731
- Reviewer finding ID: F20
- Source URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4511946431
- Concern: Row F27 was `NO_ACTION` although the title was changed.
- Solution: Row F27 is now `FIXED` with the reply URL as reference (the fix is outside the tree).
- Current-tree verification: Row F27 of this audit.
- Resolution reference: docs(adrs): [#2760] address the third review round of PR 2774
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4512239104

### F40 - The last review round has no replies, rows or log entry

- PR number: 2774
- Source review ID: 5825123886
- Reviewer finding ID: F30
- Source URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4512148108
- Concern: The review of 2026-11-09 10:55Z had no rows, replies or log entry.
- Solution: Rows F30 to F50 added, replied to and logged in this round.
- Current-tree verification: Rows F30 to F50 and the log of this audit.
- Resolution reference: docs(pr-reviews): [#2760] audit the third review round of PR 2774
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4512239455

### F41 - Re-raise: this note is now false in this very tree

- PR number: 2774
- Source review ID: 5825123886
- Reviewer finding ID: F16
- Source URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4512148116
- Concern: Same concern as F33.
- Solution: As F33: the clause is dropped.
- Current-tree verification: As F33.
- Resolution reference: docs(adrs): [#2760] address the third review round of PR 2774
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4512239830

### F42 - This verification is false at this head: the merge put the pipe back

- PR number: 2774
- Source review ID: 5825123886
- Reviewer finding ID: F31
- Source URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4512148123
- Concern: The F4 verification said the command saves the output to a file; the merge restored the piped row.
- Solution: The F4 verification now says the merge restored `develop`'s row and where the safe command lives.
- Current-tree verification: Row F4 of this audit.
- Resolution reference: docs(adrs): [#2760] address the third review round of PR 2774
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4512240142

### F43 - `It fails today` is false in this tree

- PR number: 2774
- Source review ID: 5825123886
- Reviewer finding ID: F32
- Source URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4512148135
- Concern: The 2761 spec describes work that PR #2775 already did.
- Solution: Handed to the #2771 coordinator, who fixed it in the commit `docs(issues): [#2758] address the round 3 review of PR 2774 on the child specs`.
- Current-tree verification: `grep -n 'it_should_report_the_threads_without_a_reply_as_a_result_with_exit_zero' contrib/dev-tools/github/github-review-threads/tests/cli.rs` finds the test, so the claim is stale.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4512240486
- Follow-up PR URL: https://github.com/torrust/torrust-tracker/pull/2771
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4512240486

### F44 - Re-raise: the description's counts no longer match the branch

- PR number: 2774
- Source review ID: 5825123886
- Reviewer finding ID: F6
- Source URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4512148149
- Concern: The PR body quoted file counts that changed with the merge.
- Solution: The PR body no longer quotes counts and points to `git diff --stat` (outside the tree; the reply is the durable record).
- Current-tree verification: `gh pr view 2774` body after the edit.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4512240837
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4512240837

### F45 - The merge keeps git's default subject

- PR number: 2774
- Source review ID: 5825123886
- Reviewer finding ID: F33
- Source URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4512148157
- Concern: The merge commit has the default subject and no body.
- Solution: The merge commit disappears when the branch is rebased onto `develop` after PR #2771 merges; no history rewrite now.
- Current-tree verification: The finding itself says to reword only if the branch is rewritten.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4512241098
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4512241098

### F46 - Capture child stdout in the E2E helper

- PR number: 2774
- Source review ID: 5825128396
- Reviewer finding ID: F35
- Source URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4512151660
- Concern: Spec 2765 omits capturing the child stdout of the e2e `tracker_checker` helper.
- Solution: Handed to the #2771 coordinator, who fixed it in the commit `docs(issues): [#2758] address the round 3 review of PR 2774 on the child specs`.
- Current-tree verification: `src/console/ci/e2e/tracker_checker.rs` launches the child; the spec 2765 line 47 does not mention stdout capture.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4512241479
- Follow-up PR URL: https://github.com/torrust/torrust-tracker/pull/2771
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4512241479

### F47 - Correct the execution-order claim to match recorded times

- PR number: 2774
- Source review ID: 5825128396
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4512151728
- Concern: The evidence file claims an order its stamps contradict.
- Solution: As F37.
- Current-tree verification: As F37.
- Resolution reference: docs(adrs): [#2760] address the third review round of PR 2774
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4512241801

### F48 - The claimed final-run record is missing

- PR number: 2774
- Source review ID: 5825128396
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4512151765
- Concern: The evidence file cites a final run the spec does not record.
- Solution: As F34.
- Current-tree verification: As F34.
- Resolution reference: docs(adrs): [#2760] address the third review round of PR 2774
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4512242118

### F49 - Capture the executable's stderr, not Cargo's

- PR number: 2774
- Source review ID: 5825128396
- Reviewer finding ID: R1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4512151800
- Concern: The 2762 verification captures `cargo run` stderr.
- Solution: Handed to the #2771 coordinator, who fixed it in the commit `docs(issues): [#2758] address the round 3 review of PR 2774 on the child specs`.
- Current-tree verification: Read the 2762 spec on the PR #2771 branch on 2026-11-09: M1 (line 159) now says `cargo build -p package-coverage-check` and then runs `target/debug/package-coverage-check bogus > out 2> err`, not `cargo run`.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4512242447
- Follow-up PR URL: https://github.com/torrust/torrust-tracker/pull/2771
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4512242447

### F50 - `unified/check.rs` serializes a Vec, so both formats emit an array

- PR number: 2774
- Source review ID: 5825128396
- Reviewer finding ID: F10
- Source URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4512151838
- Concern: Spec 2768 and EPIC.md:52 do not mention the array shape against the single JSON object the ADR requires.
- Solution: Handed to the #2771 coordinator, who fixed it in the commit `docs(issues): [#2758] address the round 3 review of PR 2774 on the child specs`.
- Current-tree verification: `console/tracker-client/src/console/clients/unified/check.rs` lines 147-152 build `json!(check_results)` from a `Vec`.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4512242882
- Follow-up PR URL: https://github.com/torrust/torrust-tracker/pull/2771
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4512242882

### F51 - Re-raise: the forge reports this pull request as not mergeable again

- PR number: 2774
- Source review ID: 5825509686
- Reviewer finding ID: F15
- Source URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4512454151
- Concern: `develop` archived the 2761 spec to `docs/issues/closed/`, and this branch still modified it at the old path.
- Solution: Merged `develop` again; the spec stays at the archived path with `develop`'s content, so the stack's edits to the old path give way. The remaining wording in that archived file is `develop`'s and not changed by this PR.
- Current-tree verification: `git ls-files -u` prints nothing after the merge; `git status` shows the rename to `docs/issues/closed/`; `gh pr view 2774 --json mergeable` is checked after the push.
- Resolution reference: chore(merge): [#2760] update the branch with develop and keep the archived 2761 spec
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4512518745

### F52 - Six of these seven rows cite a commit that does not carry their fix

- PR number: 2774
- Source review ID: 5825509686
- Reviewer finding ID: F34
- Source URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4512454173
- Concern: Rows F31, F35, F36, F38, F39 and F42 cite the audit commit, but their fixes are in the commit of the third round fixes.
- Solution: The six rows now cite `docs(adrs): [#2760] address the third review round of PR 2774`; F40 keeps the audit subject because its fix is split.
- Current-tree verification: `git show` of the two commits: each fix is present in the fixes commit and absent before it.
- Resolution reference: docs(pr-reviews): [#2760] audit the fourth review round of PR 2774
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4512519075

### F53 - This entry records no inspection

- PR number: 2774
- Source review ID: 5825509686
- Reviewer finding ID: F35
- Source URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4512454182
- Concern: Row F49 says the coordinator fixed it but records no inspection.
- Solution: Row F49 now names what was read on the PR #2771 branch.
- Current-tree verification: Row F49 of this audit.
- Resolution reference: docs(pr-reviews): [#2760] audit the fourth review round of PR 2774
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4512519397

### F54 - This correction says `by this commit`, but the restoration is in the commit before it

- PR number: 2774
- Source review ID: 5825509686
- Reviewer finding ID: F36
- Source URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4512454188
- Concern: The correction entry names the wrong commit and omits the dropped sentence of the first entry.
- Solution: A later correction entry names both: the restoration is in the commit of the third round fixes, and the dropped sentence of the first entry was `Threads left open, awaiting the maintainer.`. The old entry is not rewritten.
- Current-tree verification: The log of this audit.
- Resolution reference: docs(pr-reviews): [#2760] audit the fourth review round of PR 2774
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2774#discussion_r4512519916

## Processing Log

- 2026-11-06 21:05 UTC - Started audit; round 1 (Copilot review 5806242087) processed: F1 and F2 fixed and pushed, F3 to F12 handed to the #2771 coordinator and fixed there, all 12 replied. Threads left open, awaiting the maintainer.
- 2026-11-07 07:13 UTC - Branch rebased onto the fixed #2771 head and force-pushed with lease (maintainer approval in chat); all 12 threads resolved; refreshed GraphQL data shows no unresolved thread.

- 2026-11-09 10:33 UTC - Round 2 (Copilot review 5810776921 and maintainer review 5823193599) processed: fixes pushed, PR title and body updated, all 17 threads replied to; F19, F23, F25, F26 and F29 fixed on PR #2771 by the coordinator. Thread resolution is pending the maintainer (the auto mode classifier denied the resolve call), so rows F13 to F29 stay `OPEN`.

- 2026-11-09 10:35 UTC - The maintainer allowed the resolve calls in chat; all 17 round-2 threads were resolved; refreshed GraphQL data lists no unresolved thread.

- 2026-11-09 11:32 UTC - Correction: the first entry's `21:05` stamp is later than the commit that carries it (dated 21:02:32Z), so the real time of that entry is before 21:02:32Z. It had been rewritten in place to `21:02` and the later `10:33` entry's sentence on pending resolution had been replaced; both are restored to their original text by this commit.
- 2026-11-09 11:32 UTC - Round 3 (maintainer reviews 5824868731 and 5825123886, Copilot review 5825128396) processed: fixes pushed, PR body updated, all 21 threads replied to; F43, F46, F49 and F50 fixed on PR #2771 by the coordinator. Thread resolution is pending the maintainer (resolve calls need approval), so the rows still `OPEN` here stay open until then; F45 stays open for the rebase.

- 2026-11-09 11:41 UTC - The maintainer allowed the resolve calls in chat; the 20 round-3 threads other than F45 were resolved; refreshed GraphQL data lists one unresolved thread, F45, which stays open until the branch is rebased onto `develop`.

- 2026-11-09 12:06 UTC - Correction to the entry of 2026-11-09 11:32: the restoration of the first entry and of the `10:33` entry's sentence is in the commit `docs(adrs): [#2760] address the third review round of PR 2774`, not in the commit that carries that entry; the sentence dropped from the first entry was `Threads left open, awaiting the maintainer.`. That entry is not rewritten.
- 2026-11-09 12:06 UTC - Round 4 (maintainer review 5825509686) processed: the branch was updated with `develop` (the 2761 spec stays at its archived path), the audit fixed, the four threads replied to. Thread resolution is pending the maintainer (the auto mode classifier denied the resolve call), so rows F51 to F54 stay `OPEN`.

- 2026-11-09 12:11 UTC - The maintainer allowed the resolve calls in chat; the four round-4 threads were resolved; refreshed GraphQL data lists one unresolved thread, F45, which stays open until the rebase onto `develop`.

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
