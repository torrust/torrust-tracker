---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2856 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2856.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A

This record was created late (round 2), after round 1's ten findings were already fixed and
replied to. Round 1's six `da2ce7`-thread replies and three automated-reviewer-thread replies
(nine total) and all ten resolutions cite commit `f2f0e4e73` (a branch commit id) rather than its
Conventional Commit subject, `docs(issues): [#2855] address PR #2856 review findings F1-F7`. That
subject is the durable resolution reference for F1-F7 and F15-F17 below; the id-based replies
predate this record and are not edited.

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
the same head. Round 3 is `da2ce7`'s APPROVED review `5893913692` (F19 to F24, reviewer labels
F15-F20, reassigned on collision with the audit's own F15-F18 and with round 2's reviewer-label
usage) and `copilot-pull-request-reviewer`'s review `5894073993` (F25 to F27, reviewer labels
F19-F21 in each comment's own title, reassigned on collision with `da2ce7`'s round-3 F19-F21),
both submitted on the head whose last commit has the subject `docs(issues): [#2855] address
PR #2856 review findings F8-F14`. F26 and F27 are the automated reviewer's independent re-raises of
`da2ce7`'s F21 and F23 (same concerns, found separately by both reviewers in the same round, not
one reviewer repeating the other's finding). Round 4 is `da2ce7`'s APPROVED review `5899089435`
(F28 to F31, reviewer labels F21-F24, reassigned on collision with round-3 usage), submitted on
the head whose last commit has the subject `docs(issues): [#2855] address PR #2856 review
findings F19-F27`.

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
| F18 | review-finding:pr-2856-f18 | Copilot | Minor | testing | RE_RAISE_OF:F13 | FIXED | RESOLVED |
| F19 | review-finding:pr-2856-f19 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F20 | review-finding:pr-2856-f20 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F21 | review-finding:pr-2856-f21 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F22 | review-finding:pr-2856-f22 | Human | Nit | testing | ORIGINAL | FIXED | RESOLVED |
| F23 | review-finding:pr-2856-f23 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F24 | review-finding:pr-2856-f24 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F25 | review-finding:pr-2856-f25 | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F26 | review-finding:pr-2856-f26 | Copilot | Nit | documentation | RE_RAISE_OF:F21 | FIXED | RESOLVED |
| F27 | review-finding:pr-2856-f27 | Copilot | Minor | documentation | RE_RAISE_OF:F23 | FIXED | RESOLVED |
| F28 | review-finding:pr-2856-f28 | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F29 | review-finding:pr-2856-f29 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F30 | review-finding:pr-2856-f30 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F31 | review-finding:pr-2856-f31 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

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
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4565221088

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
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4565224903

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
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4565225743

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
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4565232871

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
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4565227131

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
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4565227755

### F14 - Backfilled progress-log time is later than its own commit

- PR number: 2856
- Source review ID: 5890192507
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4562747535
- Concern: The 10:40 UTC progress-log entry was already in the first commit, which was committed at 10:13:58 UTC, so the entry's own stamp postdated its commit.
- Solution: Changed the stamp to 10:12 UTC, no later than the 10:13:58 UTC commit that carries it.
- Current-tree verification: `git log --format=%cI` for the commit with subject `docs(issues): add issue specification for #2855` shows `2026-11-13T10:13:58Z`; the entry now reads `10:12 UTC`.
- Resolution reference: docs(issues): [#2855] address PR #2856 review findings F8-F14
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4565228445

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
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4565229077

### F19 - Audit record missing the skill-link marker

- PR number: 2856
- Source review ID: 5893913692
- Reviewer finding ID: F15
- Source URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4565388335
- Concern: The template and the audit records for PR #2826 and PR #2830 carry `<!-- skill-link: process-pr-review -->` between the frontmatter and the heading; this record did not.
- Solution: Added the marker in the same position.
- Current-tree verification: `docs/pr-reviews/pr-2856-review/PR-REVIEW.md` lines 1-9 now show the marker between the frontmatter's closing `---` and the `# PR #2856 Review Audit` heading.
- Resolution reference: docs(issues): [#2855] address PR #2856 review findings F19-F27
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4569357597

### F20 - Commit cited by id; Ownership note's reply count attributed all nine to `da2ce7`

- PR number: 2856
- Source review ID: 5893913692
- Reviewer finding ID: F16
- Source URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4565388346
- Concern: F14's current-tree verification cited the first commit by its full id rather than its Conventional Commit subject, and the F14 reply on GitHub cited it by a short id; separately, the Ownership note's "nine `da2ce7`-thread replies" counted six `da2ce7`-thread replies plus three automated-reviewer-thread replies as all belonging to `da2ce7`.
- Solution: Changed F14's current-tree verification to cite the commit by subject (`docs(issues): add issue specification for #2855`); posted a new reply on the F14 thread correcting the citation without editing the published comment; reworded the Ownership note to "six `da2ce7`-thread replies and three automated-reviewer-thread replies (nine total)".
- Current-tree verification: `docs/pr-reviews/pr-2856-review/PR-REVIEW.md` F14 entry and Ownership section.
- Resolution reference: docs(issues): [#2855] address PR #2856 review findings F19-F27
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4569336328

### F21 - V5 elided-line count does not add up to the reported total

- PR number: 2856
- Source review ID: 5893913692
- Reviewer finding ID: F17
- Source URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4565388350
- Concern: Five displayed rows plus "27 more lines" makes 32 containers, but `docker ps -a -q | wc -l` reported 31.
- Solution: Recaptured V5 on the runner (read-only, maintainer-approved) with a fresh run: 34 containers total, 0 exited; the listing shows 3 newest rows, elides 29, then shows the 2 oldest rows (3 + 29 + 2 = 34), matching `wc -l`.
- Current-tree verification: `manual-verification-evidence.md` V5: `docker ps -a -q | wc -l` prints 34; the `docker ps -a --format ...` listing shows 3 rows, "[... 29 more lines, newest to oldest ...]", then 2 rows (3 + 29 + 2 = 34).
- Resolution reference: docs(issues): [#2855] address PR #2856 review findings F19-F27
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4569358576

### F22 - V1 patch's second hunk header miscounted; `cp` location wrongly described

- PR number: 2856
- Source review ID: 5893913692
- Reviewer finding ID: F18
- Source URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4565388359
- Concern: The second hunk header said `@@ -37,2 +37,3 @@` (2 old, 3 new) but the body has 3 old lines and 4 new, so a strict applier (`git apply`) rejects it; separately, the prose said the copy "runs directly beside the unmodified script" when the `cp` command's relative source path actually puts the copy in the repository root (the working directory), which is why pinning `PROJECT_ROOT` to the copy's own directory works.
- Solution: Corrected the header to `@@ -37,3 +37,4 @@`; reworded the prose to describe the `cp` command's actual effect (puts the copy in the repository root because its source path is relative).
- Current-tree verification: Applied the corrected patch locally with `patch(1)` against an unmodified copy of the script; it applies and produces the two intended edits (confirmed both the `PROJECT_ROOT` line and the added echo line are present, nothing else changed).
- Resolution reference: docs(issues): [#2855] address PR #2856 review findings F19-F27
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4569359397

### F23 - Backfilled 14:00 stamps predate the V5 rerun they report

- PR number: 2856
- Source review ID: 5893913692
- Reviewer finding ID: F19
- Source URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4565388371
- Concern: Both `last-updated-utc` fields and the ISSUE.md progress-log entry said 14:00 UTC, but the V5 listing's newest container ("Up 2 hours" at 13:46:16 creation) could not have been captured before roughly 15:16, and the commit carrying it was committed at 15:23:59.
- Solution: Recaptured V5 on the runner at a recorded real time (2026-11-13 22:18:22 UTC) and set `last-updated-utc` in both files, plus a new progress-log entry, to match.
- Current-tree verification: `manual-verification-evidence.md` frontmatter and V5 "Capture time" line; `ISSUE.md` frontmatter and progress log.
- Resolution reference: docs(issues): [#2855] address PR #2856 review findings F19-F27
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4569360166

### F24 - `linter all` criterion unchecked; description's Files touched list stale

- PR number: 2856
- Source review ID: 5893913692
- Reviewer finding ID: F20
- Source URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4565388384
- Concern: The `linter all` acceptance criterion was unchecked while the checkpoint on the line above marked automatic verification complete and the description said `linter all` passes; separately, the description's Files touched list omitted `docs/pr-reviews/pr-2856-review/PR-REVIEW.md`.
- Solution: Ran `linter all` (exits 0) and checked the criterion; added the audit record to the description's Files touched list via `gh-bot pr edit`.
- Current-tree verification: `ISSUE.md` Acceptance Criteria; PR #2856 description.
- Resolution reference: docs(issues): [#2855] address PR #2856 review findings F19-F27
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4569361000

### F25 - Copilot: F18 should be classified as a re-raise of F13

- PR number: 2856
- Source review ID: 5894073993
- Reviewer finding ID: F19
- Source URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4565509280
- Concern: F18's detail explicitly says it requests the same reconstruction fix as F13 and uses the same verification, but the table labeled it `ORIGINAL`, double-counting one underlying concern.
- Solution: Changed F18's Relationship to `RE_RAISE_OF:F13`.
- Current-tree verification: `docs/pr-reviews/pr-2856-review/PR-REVIEW.md` F18 row.
- Resolution reference: docs(issues): [#2855] address PR #2856 review findings F19-F27
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4569305288

### F26 - Copilot: re-raise of F21 (elided container count)

- PR number: 2856
- Source review ID: 5894073993
- Reviewer finding ID: F20
- Source URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4565509225
- Concern: Same concern as F21 (elided-line count did not add up to the reported total).
- Solution: Fixed together with F21.
- Current-tree verification: Same as F21.
- Resolution reference: docs(issues): [#2855] address PR #2856 review findings F19-F27
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4569306256

### F27 - Copilot: re-raise of F23 (V5 capture time)

- PR number: 2856
- Source review ID: 5894073993
- Reviewer finding ID: F21
- Source URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4565509158
- Concern: Same concern as F23 (backfilled stamp predates the V5 rerun it reports).
- Solution: Fixed together with F23.
- Current-tree verification: Same as F23.
- Resolution reference: docs(issues): [#2855] address PR #2856 review findings F19-F27
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4569307085

### F28 - V1 recipe and "line 39" describe the pre-fix script without naming it

- PR number: 2856
- Source review ID: 5899089435
- Reviewer finding ID: F21
- Source URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4569510300
- Concern: PR #2857's fix merged into `develop`, so the working tree's script no longer has the `test` line V1's recipe and ISSUE.md's "line 39" describe (it is now a `case` block); a `cp` of the working tree and an unqualified "line 39" no longer name any real state.
- Solution: Named the pre-fix state by its durable `develop` commit id, `ec1f58ec9` (the last commit before the merge): V1's setup now reads `git show ec1f58ec9:...` instead of `cp` of the working tree, and both "line 39" references now say "line 39 at `ec1f58ec9`".
- Current-tree verification: Applied the patch to `git show ec1f58ec9:contrib/dev-tools/containers/tests/test-mounted-no-persistence-configuration.sh` locally with `patch(1)`; it applies and produces the two intended edits.
- Resolution reference: docs(issues): [#2855] address PR #2856 review findings F28-F31
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4569539309

### F29 - Round-2 progress-log entry's container count and time were wrong

- PR number: 2856
- Source review ID: 5899089435
- Reviewer finding ID: F22
- Source URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4569510320
- Concern: The round-4 recapture shows exactly 2 containers newer than the round-2 capture's newest (13:46:16); since containers cannot be created with a past creation time, the round-2 capture held at least 34 - 2 = 32, not the 31 its progress-log entry stated (which matched its own listing's row count, 3 + 27 + 2, but not the `wc -l` it also recorded). Separately, the entry's 14:00 stamp still predates the capture it reports (no earlier than roughly 15:16, per F19/F23's prior reasoning).
- Solution: Corrected the round-2 progress-log entry's count to 32 and its timestamp to "15:20 UTC (backfilled; no later than the 15:23:59 UTC commit that carries it)", with a note that the entry originally stated 31.
- Current-tree verification: `ISSUE.md` progress log, the round-2 entry.
- Resolution reference: docs(issues): [#2855] address PR #2856 review findings F28-F31
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4569540164

### F30 - F14's correction reply landed on the wrong thread; F25-F27 wrongly marked "(inferred)"

- PR number: 2856
- Source review ID: 5899089435
- Reviewer finding ID: F23
- Source URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4569510328
- Concern: The reply correcting F14's commit-id citation was posted on F16's own thread instead of F14's; separately, F25-F27 carried "(inferred)" although their source comments open with explicit severity brackets (`[Minor][F19]`, `[Nit][F20]`, `[Minor][F21]`), not free prose.
- Solution: Posted a new reply directly on F14's thread with the correction; removed "(inferred)" from F25, F26 and F27.
- Current-tree verification: `docs/pr-reviews/pr-2856-review/PR-REVIEW.md` F25-F27 rows.
- Resolution reference: docs(issues): [#2855] address PR #2856 review findings F28-F31
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4569541157

### F31 - 22:18 progress-log entry cited the wrong finding IDs and omitted fixes

- PR number: 2856
- Source review ID: 5899089435
- Reviewer finding ID: F24
- Source URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4569510342
- Concern: The entry cited "F19-F21", the automated reviewer's own labels, while in this audit those findings are F25-F27 and F19-F21 are other, unrelated findings; it also omitted several of the same commit's fixes (the skill-link marker, the commit cited by subject, the V1 hunk header and `cp` prose, the `linter all` criterion).
- Solution: Rewrote the entry to cite the audit IDs (F19-F27) and list all of that commit's fixes.
- Current-tree verification: `ISSUE.md` progress log, the 22:18 entry.
- Resolution reference: docs(issues): [#2855] address PR #2856 review findings F28-F31
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2856#discussion_r4569542129

## Processing Log

- 2026-11-13 11:20 UTC - `da2ce7`'s review `5889698232` (7 findings, F1 to F7) and `copilot-pull-request-reviewer`'s review `5889723390` (3 findings, F15 to F17) processed before this record existed: fixed in commit `f2f0e4e73` (branch id cited in the nine replies instead of this subject) and replied on all ten threads, then resolved.
- 2026-11-13 14:00 UTC - Started this record late, after round 2 arrived. Processing round 2: `da2ce7`'s review `5890192507` (7 findings, F8 to F14) and `copilot-pull-request-reviewer`'s review `5891738510` (1 finding, F18, reviewer label `F8`, reassigned on collision with `da2ce7`'s F8).
- 2026-11-13 22:30 UTC - Processing round 3: `da2ce7`'s APPROVED review `5893913692` (6 findings, F19 to F24, reviewer labels F15-F20) and `copilot-pull-request-reviewer`'s review `5894073993` (3 findings, F25 to F27, reviewer labels F19-F21). F26 and F27 are independent re-raises of F21 and F23 (same underlying concerns, found separately by both reviewers in this round). Fixed in commit `docs(issues): [#2855] address PR #2856 review findings F19-F27`: added the skill-link marker (F19); cited the first commit by subject and corrected the Ownership note's reply-count attribution (F20); recaptured V5 on torrust-runner-01 (read-only, maintainer-approved) with a fresh, internally-consistent listing (F21, F26); corrected the V1 patch's second hunk header and the `cp` location prose (F22); recorded the real V5 capture time throughout (F23, F27); ran `linter all`, checked the criterion, and added the audit record to the PR description's Files touched list (F24); reclassified F18 as `RE_RAISE_OF:F13` (F25). Replied on all nine threads, then resolved them.
- 2026-11-13 22:45 UTC - Processing round 4: `da2ce7`'s APPROVED review `5899089435` (4 findings, F28 to F31, reviewer labels F21-F24). Fixed in commit `docs(issues): [#2855] address PR #2856 review findings F28-F31`: named the pre-fix script by its durable `develop` commit id `ec1f58ec9` instead of a `cp` of the (now fixed) working tree or an unqualified line number (F28); corrected the round-2 progress-log entry's container count (31 to 32) and timestamp, noting the original was wrong (F29); posted the F14 correction on F14's own thread instead of F16's, and removed the incorrect "(inferred)" tag from F25-F27 (F30); rewrote the round-3 progress-log entry to cite audit IDs instead of the automated reviewer's own labels and to list all of that commit's fixes (F31). Replied on all four threads, then resolved them.
