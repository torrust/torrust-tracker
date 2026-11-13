---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

# PR #2856 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2856.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A

This record was created late (round 2), after round 1's ten findings were already fixed and
replied to. Round 1's nine `da2ce7`-thread replies and all ten resolutions cite commit
`f2f0e4e73` (a branch commit id) rather than its Conventional Commit subject, `docs(issues):
[#2855] address PR #2856 review findings F1-F7`. That subject is the durable resolution
reference for F1-F7 and F15-F17 below; the id-based replies predate this record and are not
edited.

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

Round 1 is `da2ce7`'s review `5889698232` (F1 to F7) and `copilot-pull-request-reviewer`'s review
`5889723390` (F15 to F17, no reviewer-provided ids), both submitted on the head whose last commit
had the subject `docs(issues): add issue specification for #2855`. Round 2 is `da2ce7`'s review
`5890192507` (F8 to F14), submitted on the head whose last commit has the subject `docs(issues):
[#2855] address PR #2856 review findings F1-F7`, and `copilot-pull-request-reviewer`'s review
`5891738510` (F18, reviewer label `F8`, reassigned on collision with `da2ce7`'s F8), submitted on
the same head.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2856-f1 | Human | Major | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2856-f2 | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2856-f3 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2856-f4 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2856-f5 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2856-f6 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2856-f7 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2856-f8 | Human | Major | documentation | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-2856-f9 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-2856-f10 | Human | Minor | maintainability | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-2856-f11 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F12 | review-finding:pr-2856-f12 | Human | Minor | maintainability | ORIGINAL | FIXED | RESOLVED |
| F13 | review-finding:pr-2856-f13 | Human | Nit | testing | ORIGINAL | FIXED | RESOLVED |
| F14 | review-finding:pr-2856-f14 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F15 | review-finding:pr-2856-f15 | Copilot | Minor (inferred) | documentation | RE_RAISE_OF:F6 | FIXED | RESOLVED |
| F16 | review-finding:pr-2856-f16 | Copilot | Minor (inferred) | formatting | ORIGINAL | NO_ACTION | RESOLVED |
| F17 | review-finding:pr-2856-f17 | Copilot | Nit (inferred) | documentation | RE_RAISE_OF:F7 | FIXED | RESOLVED |
| F18 | review-finding:pr-2856-f18 | Copilot | Minor | testing | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - V4 output not produced by the named command

- PR number: 2856
- Source review ID: 5889698232
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4562390058
- Concern: V4's block named `dpkg -l | grep coreutils; grep -h -B2 -A3 rust-coreutils /var/log/apt/history.log*` but showed a `readlink`/`timeout --version`-shaped line neither command can print, and silently dropped dpkg columns and the `-A3` context.
- Solution: Split V4 into three separate commands actually run over SSH on `torrust-runner-01`, each with its own real output (`dpkg -l | grep coreutils`, `readlink -f $(command -v timeout); timeout --version`, `grep -h -B2 -A3 "rust-coreutils" /var/log/apt/history.log*`).
- Current-tree verification: `manual-verification-evidence.md` V4 now shows three fenced commands whose output matches each command's real behavior; no elided columns.
- Resolution reference: docs(issues): [#2855] address PR #2856 review findings F1-F7
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4562663093

### F2 - V1/M3 do not record a repeatable invocation

- PR number: 2856
- Source review ID: 5889698232
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4562390070
- Concern: V1's script had no `RUN_TRACKER_STATUS` line and no recorded `IMAGE_TAG`/edit, and M3 did not say how the script was pointed at GNU `timeout`.
- Solution: Recorded the exact edit (now a patch that can be reapplied, see F13/F18) and the exact command including `IMAGE_TAG=tt:repro BUILD_IMAGE=false`; M3/V3 now name the `PATH`-prepended-symlink mechanism used to select GNU `timeout` without editing the script.
- Current-tree verification: `manual-verification-evidence.md` V1 and V3 both show the exact command line used.
- Resolution reference: docs(issues): [#2855] address PR #2856 review findings F1-F7
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4562663455

### F3 - Background step 3 states the mechanism as fact

- PR number: 2856
- Source review ID: 5889698232
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4562390077
- Concern: Step 3 said the docker client "never forwards the signal", but V2's `kill -INT <docker client pid>` row shows the client does forward a SIGINT it receives; no recorded probe shows whether the signal reaches the client at all.
- Solution: Reworded step 3 to state the observed symptom as fact and the forwarding/delivery mechanism as an explicit, documented hypothesis.
- Current-tree verification: `ISSUE.md` Background step 3 now reads "...is a hypothesis...; the fix below is correct under either reading."
- Resolution reference: docs(issues): [#2855] address PR #2856 review findings F1-F7
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4562663893

### F4 - Impact omits the leftover container

- PR number: 2856
- Source review ID: 5889698232
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4562390087
- Concern: The Impact section did not mention that each failed run leaves its tracker container running on the self-hosted runner.
- Solution: Added the leftover-container explanation to Impact, backed by new evidence V5.
- Current-tree verification: `ISSUE.md` Impact paragraph 2 now describes the leftover-container mechanism, citing V5.
- Resolution reference: docs(issues): [#2855] address PR #2856 review findings F1-F7
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4562664191

### F5 - PR description missing required sections, head/base inverted

- PR number: 2856
- Source review ID: 5889698232
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4562390092
- Concern: The description lacked the `open-pull-request` skill's required sections and inverted head and base.
- Solution: Rewrote the description with Summary, Files touched, and Validation performed sections, and corrected the head/base wording.
- Current-tree verification: `gh-bot pr view 2856 --json body` shows all three required sections and the corrected wording.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4562664586
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4562664586

### F6 - T2 status inconsistent with its own note

- PR number: 2856
- Source review ID: 5889698232
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4562390098
- Concern: T2 was `TODO` while its note said the red run was already captured in V1, and the scenario table listed M1, M2, M4, M3 out of order with an M4 steps cell that did not match the command V4 actually ran.
- Solution: Marked T2 `DONE` against V1, reordered the scenario table to M1-M5, and corrected the M4 steps cell to the actual V4 commands.
- Current-tree verification: `ISSUE.md` Implementation Plan T2 is `DONE`; the Manual Verification Scenarios table lists M1 through M5 in order.
- Resolution reference: docs(issues): [#2855] address PR #2856 review findings F1-F7
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4562664901

### F7 - Progress-log timestamp format and dropped checkpoints

- PR number: 2856
- Source review ID: 5889698232
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4562390105
- Concern: The second progress-log entry had no `HH:MM`, `last-updated-utc` predated it, and the workflow checkpoints dropped three template items (reviewer validation, independent reviewer reports, committer check) that peer open specs keep.
- Solution: Added `HH:MM` to every progress-log entry, advanced `last-updated-utc`, and restored the full template checkpoint list.
- Current-tree verification: `ISSUE.md` Workflow Checkpoints has all thirteen template items; every Progress Log entry carries `HH:MM`.
- Resolution reference: docs(issues): [#2855] address PR #2856 review findings F1-F7
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4562665264

### F8 - V5 listing shows output `head -5` cannot print; "oldest"/"none stopped" unproven

- PR number: 2856
- Source review ID: 5890192507
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4562747484
- Concern: `docker ps -a | head -5` keeps the five newest containers and cannot produce the oldest line shown with an ellipsis; "none stopped" was not shown by any recorded command either.
- Solution: Re-ran on `torrust-runner-01` (maintainer-approved SSH): a full `docker ps -a --format ...` listing with no lines elided except an explicitly marked ellipsis over the identical middle rows, a separate `docker ps -a --filter status=exited -q | wc -l` returning `0`, and a `docker inspect` loop over every running container identifying all 31 by image (`torrust-tracker:local`) and this script's `/tmp/test-mounted-no-persistence-configuration.<random>/{etc,lib,log}` mount pattern.
- Current-tree verification: `manual-verification-evidence.md` V5 now shows the full listing, the exited-count command, and the image/mount identification, with the true oldest (`2212f3f2e8ef`, 2026-11-11 09:45:50 UTC) and count (31) stated from that listing.
- Resolution reference: docs(issues): [#2855] address PR #2856 review findings F8-F14
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4562747484

### F9 - Impact states an unproven cause as fact

- PR number: 2856
- Source review ID: 5890192507
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4562747491
- Concern: Impact read as if all 26 (now 31) leftover containers were caused by this bug, while V5 only said the count was consistent with, not proven to be caused by, it; the oldest container predates the last green run.
- Solution: Reworded Impact to state that the oldest leftover predates the last known-green run (so it is not explained by this regression) and that the rest are consistent with, but not individually proven to be caused by, it.
- Current-tree verification: `ISSUE.md` Impact paragraph 2 now carries this hedge, matching V5's own wording.
- Resolution reference: docs(issues): [#2855] address PR #2856 review findings F8-F14
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4562747491

### F10 - No review audit record; replies cite a branch commit id

- PR number: 2856
- Source review ID: 5890192507
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4562747500
- Concern: All ten round-1 threads were answered and resolved without the `process-pr-review` audit record ever being created, and nine replies cited the branch commit id `f2f0e4e73` instead of its Conventional Commit subject.
- Solution: Created this record (late; the late start and the id-to-subject mapping are logged in Ownership and the Processing Log below). The id-based replies already posted on GitHub are not edited; this record states their actual subject.
- Current-tree verification: `docs/pr-reviews/pr-2856-review/PR-REVIEW.md` exists at this head with the Ownership note naming the nine id-based replies' actual subject.
- Resolution reference: docs(pr-reviews): [#2855] create the PR 2856 review audit record
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4562747500

### F11 - PR description still states the mechanism as fact

- PR number: 2856
- Source review ID: 5890192507
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4562747509
- Concern: The description's summary still said uutils `timeout` "never forwards SIGINT to the `docker run` client" as fact, while `ISSUE.md` (F3) now calls that a hypothesis.
- Solution: Reworded the description to "the tracker never receives SIGINT" and named the forwarding-vs-delivery question as a documented hypothesis, matching `ISSUE.md`.
- Current-tree verification: `gh-bot pr view 2856 --json body` shows the reworded sentence.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2856#issuecomment-6857306524
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4562747509

### F12 - Spec-only PR records unmerged-implementation results; checkpoint order

- PR number: 2856
- Source review ID: 5890192507
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4562747514
- Concern: T3/T4/AC1/AC2 stated implementation results without naming the branch they ran on, and the checkpoints checked "Implementation completed" while "Spec-only PR merged into `develop` before implementation" stayed unchecked, against the spec-first sequence in `create-issue`; "Manual verification scenarios executed" also stayed unchecked although M1-M5 were `DONE`.
- Solution: Named the fix by its Conventional Commit subject and PR number in V3 and the Implementation Plan; marked T3/T4/AC1-AC3 explicitly as validated on PR #2857's branch (not yet on `develop`); checked "Manual verification scenarios executed"; recorded the spec-first sequence deviation (implementation PR opened before this spec-only PR merged) in the Progress Log.
- Current-tree verification: `ISSUE.md` Implementation Plan and Acceptance Verification name PR #2857 and its commit subject; the 2026-11-13 11:20 UTC progress-log entry states the deviation.
- Resolution reference: docs(issues): [#2855] address PR #2856 review findings F8-F14
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4562747514

### F13 - V1 edit has a no-op pair and omits the `PROJECT_ROOT` change

- PR number: 2856
- Source review ID: 5890192507
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4562747522
- Concern: The recorded V1 diff removed and re-added the identical `timeout` line (a no-op) and did not show the stated `PROJECT_ROOT` change, so the edit could not be reapplied exactly as written.
- Solution: Replaced the diff with a complete unified patch that can be reapplied with `patch(1)` (correct hunk headers) covering both real changes: the `PROJECT_ROOT` pin and the added status-echo line; see also F18, which asked for the same ability to reconstruct the script from a fresh checkout.
- Current-tree verification: `manual-verification-evidence.md` V1 shows a `cp` + `patch <<'PATCH'` setup block with two hunks, no no-op lines.
- Resolution reference: docs(issues): [#2855] address PR #2856 review findings F8-F14
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4562747522

### F14 - Backfilled progress-log time is later than its own commit

- PR number: 2856
- Source review ID: 5890192507
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4562747535
- Concern: The 10:40 UTC progress-log entry was already in the first commit, which was committed at 10:13:58 UTC, so the entry's own stamp postdated its commit.
- Solution: Changed the stamp to 10:12 UTC, no later than the 10:13:58 UTC commit that carries it.
- Current-tree verification: `git log --format=%cI` for `9796436047f042efb5d2800d89ad4ec973706a81` shows `2026-11-13T10:13:58Z`; the entry now reads `10:12 UTC`.
- Resolution reference: docs(issues): [#2855] address PR #2856 review findings F8-F14
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4562747535

### F15 - Copilot re-raise: T2 status inconsistency

- PR number: 2856
- Source review ID: 5889723390
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4562409597
- Concern: Same concern as F6 (T2 marked `TODO` while its note says the evidence is already captured).
- Solution: Fixed together with F6.
- Current-tree verification: Same as F6.
- Resolution reference: docs(issues): [#2855] address PR #2856 review findings F1-F7
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4562667223

### F16 - Copilot: `||` markdown-table syntax (false positive)

- PR number: 2856
- Source review ID: 5889723390
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4562409666
- Concern: Table rows allegedly used `||` instead of `|`, which could render incorrectly in Markdown.
- Solution: No action: inspected the raw file bytes (`cat -A`, `grep '||'`) and found no double-pipe table syntax anywhere; the only `||` in the files is shell's logical-or inside a fenced code block. Replied explaining this. The evidence file was rewritten anyway for F1/F2/F8/F13, which keeps all tables single-pipe.
- Current-tree verification: `grep -n '||' docs/issues/open/2855-container-regression-timeout-sigint/*.md` matches only the shell `|| exit_status=$?` line inside a code fence.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4562667618
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4562667618

### F17 - Copilot re-raise: progress-log timestamp format

- PR number: 2856
- Source review ID: 5889723390
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4562409692
- Concern: Same concern as F7 (second progress-log entry missing `HH:MM`).
- Solution: Fixed together with F7.
- Current-tree verification: Same as F7.
- Resolution reference: docs(issues): [#2855] address PR #2856 review findings F1-F7
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4562667984

### F18 - Copilot: reproduction cannot be rerun from a fresh checkout

- PR number: 2856
- Source review ID: 5891738510
- Reviewer finding ID: F8
- Source URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4563793717
- Concern: V1's `./instrumented-copy.sh` is not in the issue folder, and the shown diff omitted the stated `PROJECT_ROOT` change, so a fresh checkout could not execute or reconstruct the reproduction.
- Solution: Same fix as F13: replaced the diff with a complete `cp` + `patch` block, reapplied with `patch(1)`, naming both real changes, so the instrumented copy can be reconstructed exactly without retaining it as a separate issue-local artifact.
- Current-tree verification: Same as F13.
- Resolution reference: docs(issues): [#2855] address PR #2856 review findings F8-F14
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4563793717

## Processing Log

- 2026-11-13 11:20 UTC - `da2ce7`'s review `5889698232` (7 findings, F1 to F7) and `copilot-pull-request-reviewer`'s review `5889723390` (3 findings, F15 to F17) processed before this record existed: fixed in commit `f2f0e4e73` (branch id cited in the nine replies instead of this subject) and replied on all ten threads, then resolved.
- 2026-11-13 14:00 UTC - Started this record late, after round 2 arrived. Processing round 2: `da2ce7`'s review `5890192507` (7 findings, F8 to F14) and `copilot-pull-request-reviewer`'s review `5891738510` (1 finding, F18, reviewer label `F8`, reassigned on collision with `da2ce7`'s F8).
