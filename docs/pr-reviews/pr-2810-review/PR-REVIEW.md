---
pr-number: 2810
pr-url: https://github.com/torrust/torrust-tracker/pull/2810
last-updated-utc: "2026-11-11 09:25"
---

# PR #2810 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2810.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

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

Round 1: da2ce7 review 5846429188 (CHANGES_REQUESTED) on the head that carries the commit `docs(issues): [#2762] record verification evidence and progress`: one Major, five Minor, two Nit. All eight are valid and fixed in the commit `fix(package-coverage-check): [#2762] keep pipefail on the summary step and correct the evidence`.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2810-f1 | Human | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2810-f2 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2810-f3 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2810-f4 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2810-f5 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2810-f6 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2810-f7 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2810-f8 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-2810-f9 | Copilot | Major | correctness | RE_RAISE_OF:F1 | FIXED | RESOLVED |
| F10 | review-finding:pr-2810-f10 | Copilot | Minor | documentation | RE_RAISE_OF:F2 | FIXED | RESOLVED |
| F11 | review-finding:pr-2810-f11 | Human | Minor | documentation | RE_RAISE_OF:F3 | FIXED | RESOLVED |
| F12 | review-finding:pr-2810-f12 | Human | Nit | metadata | RE_RAISE_OF:F8 | FIXED | RESOLVED |
| F13 | review-finding:pr-2810-f13 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F14 | review-finding:pr-2810-f14 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F15 | review-finding:pr-2810-f15 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F16 | review-finding:pr-2810-f16 | Human | Minor | documentation | RE_RAISE_OF:F11 | FIXED | RESOLVED |
| F17 | review-finding:pr-2810-f17 | Human | Nit | metadata | RE_RAISE_OF:F12 | FIXED | RESOLVED |
| F18 | review-finding:pr-2810-f18 | Human | Nit | metadata | RE_RAISE_OF:F15 | FIXED | RESOLVED |
| F19 | review-finding:pr-2810-f19 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F20 | review-finding:pr-2810-f20 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F21 | review-finding:pr-2810-f21 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F22 | review-finding:pr-2810-f22 | Human | Nit | documentation | RE_RAISE_OF:F19 | FIXED | RESOLVED |
| F23 | review-finding:pr-2810-f23 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F24 | review-finding:pr-2810-f24 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F25 | review-finding:pr-2810-f25 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F26 | review-finding:pr-2810-f26 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |

Round 1 follow-up: Copilot review 5846504036, on the earlier head, raised the F1 and F2 concerns again as F9 and F10 (reviewer IDs F1 and F2); both were already fixed. Round 2: da2ce7 review 5846796508 on the head after round 1: F11 to F15 (reviewer IDs F3, F8, F9, F10, F11), fixed in the round-2 commit. Round 3: da2ce7 review 5847265767 (APPROVED) on the head after round 2: F16 to F21 (reviewer IDs F3, F8, F11, F12, F13, F14), fixed in the round-3 commit. Round 4: da2ce7 review 5847623438 (APPROVED) on the head after round 3: F22 and F23 (reviewer IDs F12 and F15), fixed in the round-4 commit. Round 5: da2ce7 review 5850003910 (APPROVED) on the head after round 4: F24 to F26 (reviewer IDs F16, F17, F18), fixed in the close-out PR.

## Finding Details

### F1 - The jq pipe masks a failing summary: the step has no pipefail.

- PR number: 2810
- Source review ID: 5846429188
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2810#discussion_r4528268634
- Concern: The jq pipe masks a failing summary: the step has no pipefail.
- Solution: Valid. Set shell: bash on the step so a failing summary fails it; V3 corrected.
- Current-tree verification: git diff shows shell: bash on the Write Report-Only Summary step.
- Resolution reference: fix(package-coverage-check): [#2762] keep pipefail on the summary step and correct the evidence
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2810#discussion_r4528303412

### F2 - TTY exit=2 record cannot come from the binary: $? expanded by the outer shell.

- PR number: 2810
- Source review ID: 5846429188
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2810#discussion_r4528268643
- Concern: TTY exit=2 record cannot come from the binary: $? expanded by the outer shell.
- Solution: Valid. Reran with single quotes and recorded the verbatim output.
- Current-tree verification: Evidence V1 shows the single-quoted command and exit=2.
- Resolution reference: fix(package-coverage-check): [#2762] keep pipefail on the summary step and correct the evidence
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2810#discussion_r4528303842

### F3 - AC5 cites a PR-run confirmation that did not exist; M3 and T4 DONE against actionlint that was not run.

- PR number: 2810
- Source review ID: 5846429188
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2810#discussion_r4528268653
- Concern: AC5 cites a PR-run confirmation that did not exist; M3 and T4 DONE against actionlint that was not run.
- Solution: Valid. AC5, M3 and T4 now record what ran.
- Current-tree verification: ISSUE.md AC5 row, M3 row and T4 row.
- Resolution reference: fix(package-coverage-check): [#2762] keep pipefail on the summary step and correct the evidence
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2810#discussion_r4528304204

### F4 - Evidence lacks the quoted red output, green output, unit-test mutation and the build per recheck.

- PR number: 2810
- Source review ID: 5846429188
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2810#discussion_r4528268662
- Concern: Evidence lacks the quoted red output, green output, unit-test mutation and the build per recheck.
- Solution: Valid. Added R1 quote, mutation, G1 green output and the Environment builds.
- Current-tree verification: Evidence sections R1, G1 and Environment.
- Resolution reference: fix(package-coverage-check): [#2762] keep pipefail on the summary step and correct the evidence
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2810#discussion_r4528304669

### F5 - T1 DONE for TTY tests but no TTY test exists; the reason is not in the spec.

- PR number: 2810
- Source review ID: 5846429188
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2810#discussion_r4528268673
- Concern: T1 DONE for TTY tests but no TTY test exists; the reason is not in the spec.
- Solution: Valid. The Regression Test Strategy and T1 record why it is manual-only and the follow-up.
- Current-tree verification: ISSUE.md Regression Test Strategy and T1 row.
- Resolution reference: fix(package-coverage-check): [#2762] keep pipefail on the summary step and correct the evidence
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2810#discussion_r4528305114

### F6 - linter all checkpoint ticked while the acceptance item stays open, no run recorded.

- PR number: 2810
- Source review ID: 5846429188
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2810#discussion_r4528268683
- Concern: linter all checkpoint ticked while the acceptance item stays open, no run recorded.
- Solution: Valid. Ran linter all, recorded as L1, checked both.
- Current-tree verification: Evidence section L1; ISSUE.md checkboxes.
- Resolution reference: fix(package-coverage-check): [#2762] keep pipefail on the summary step and correct the evidence
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2810#discussion_r4528305549

### F7 - The TTY early return is in run, not main.

- PR number: 2810
- Source review ID: 5846429188
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2810#discussion_r4528268698
- Concern: The TTY early return is in run, not main.
- Solution: Valid. Comment corrected.
- Current-tree verification: tests/cli.rs header says run.
- Resolution reference: fix(package-coverage-check): [#2762] keep pipefail on the summary step and correct the evidence
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2810#discussion_r4528305930

### F8 - Log entry and last-updated-utc say 12:00 but R1 names a 16:49 run.

- PR number: 2810
- Source review ID: 5846429188
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2810#discussion_r4528268706
- Concern: Log entry and last-updated-utc say 12:00 but R1 names a 16:49 run.
- Solution: Valid. Times corrected to the recording time.
- Current-tree verification: ISSUE.md progress log and both last-updated-utc fields.
- Resolution reference: fix(package-coverage-check): [#2762] keep pipefail on the summary step and correct the evidence
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2810#discussion_r4528306362

### F9 - The summary step has no pipefail, so a failing summary passes (re-raise of F1).

- PR number: 2810
- Source review ID: 5846504036
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2810#discussion_r4528327554
- Concern: The summary step has no pipefail, so a failing summary passes (re-raise of F1).
- Solution: Already fixed by the F1 change (shell: bash); replied and resolved.
- Current-tree verification: Workflow step sets shell: bash; the hosted log shows pipefail.
- Resolution reference: fix(package-coverage-check): [#2762] keep pipefail on the summary step and correct the evidence
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2810#discussion_r4528344636

### F10 - The TTY exit status is expanded by the outer shell (re-raise of F2).

- PR number: 2810
- Source review ID: 5846504036
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2810#discussion_r4528327650
- Concern: The TTY exit status is expanded by the outer shell (re-raise of F2).
- Solution: Already fixed by the F2 change (single quotes); replied and resolved.
- Current-tree verification: Evidence V1 shows the single-quoted command.
- Resolution reference: fix(package-coverage-check): [#2762] keep pipefail on the summary step and correct the evidence
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2810#discussion_r4528345067

### F11 - T4, AC5 and V3 cite a hosted-run note the PR does not carry.

- PR number: 2810
- Source review ID: 5846796508
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/2810#discussion_r4528549737
- Concern: T4, AC5 and V3 cite a hosted-run note the PR does not carry.
- Solution: Valid. Recorded the hosted run as H1 (job, run, what it showed and what was not read) and reworded T4, AC5 and V3.
- Current-tree verification: Evidence H1; ISSUE.md T4 and AC5 rows.
- Resolution reference: docs(pr-reviews): [#2762] address PR 2810 review round 2
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2810#discussion_r4528657876

### F12 - Corrected stamps are later than the commits carrying them.

- PR number: 2810
- Source review ID: 5846796508
- Reviewer finding ID: F8
- Source URL: https://github.com/torrust/torrust-tracker/pull/2810#discussion_r4528549751
- Concern: Corrected stamps are later than the commits carrying them.
- Solution: Valid. Stamps now come from the commit times (16:54 and 17:35).
- Current-tree verification: git log --format=%cI of the fix commits.
- Resolution reference: docs(pr-reviews): [#2762] address PR 2810 review round 2
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2810#discussion_r4528656446

### F13 - The audit has no rows for the two findings of review 5846504036.

- PR number: 2810
- Source review ID: 5846796508
- Reviewer finding ID: F9
- Source URL: https://github.com/torrust/torrust-tracker/pull/2810#discussion_r4528549758
- Concern: The audit has no rows for the two findings of review 5846504036.
- Solution: Valid. Added F9 and F10 with their details.
- Current-tree verification: This file.
- Resolution reference: docs(pr-reviews): [#2762] address PR 2810 review round 2
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2810#discussion_r4528656880

### F14 - Evidence and audit cite PR-branch commits by id.

- PR number: 2810
- Source review ID: 5846796508
- Reviewer finding ID: F10
- Source URL: https://github.com/torrust/torrust-tracker/pull/2810#discussion_r4528549762
- Concern: Evidence and audit cite PR-branch commits by id.
- Solution: Valid. Cited by subject; V1 and V2 are placed on the first fix commit build.
- Current-tree verification: grep for the ids in the spec folder and this audit prints nothing.
- Resolution reference: docs(pr-reviews): [#2762] address PR 2810 review round 2
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2810#discussion_r4528657554

### F15 - Audit log entry and stamp postdate its commit; resolution times unrecorded.

- PR number: 2810
- Source review ID: 5846796508
- Reviewer finding ID: F11
- Source URL: https://github.com/torrust/torrust-tracker/pull/2810#discussion_r4528549772
- Concern: Audit log entry and stamp postdate its commit; resolution times unrecorded.
- Solution: Valid. Stamps from the commit time (17:37); the resolution times are stated as not captured.
- Current-tree verification: git log --format=%cI of the audit commit.
- Resolution reference: docs(pr-reviews): [#2762] address PR 2810 review round 2
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2810#discussion_r4528657210

### F16 - AC5 stays checked while the hosted summary was not read; the description records no run.

- PR number: 2810
- Source review ID: 5847265767
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/2810#discussion_r4528905298
- Concern: AC5 stays checked while the hosted summary was not read; the description records no run.
- Solution: Valid. The hosted summary is not readable here; H1 and the AC5 row state what was observed and what was not; the PR description records the run.
- Current-tree verification: Evidence H1; ISSUE.md AC5 row.
- Resolution reference: docs(pr-reviews): [#2762] address PR 2810 review round 3
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2810#discussion_r4528939229

### F17 - The 18:15 stamps postdate the commit carrying them.

- PR number: 2810
- Source review ID: 5847265767
- Reviewer finding ID: F8
- Source URL: https://github.com/torrust/torrust-tracker/pull/2810#discussion_r4528905308
- Concern: The 18:15 stamps postdate the commit carrying them.
- Solution: Valid. Round-2 entry stamped 18:10; last-updated fields set before the commit.
- Current-tree verification: ISSUE.md progress log and last-updated-utc fields.
- Resolution reference: docs(pr-reviews): [#2762] address PR 2810 review round 3
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2810#discussion_r4528938352

### F18 - The audit last-updated-utc and the new entry postdate their commit.

- PR number: 2810
- Source review ID: 5847265767
- Reviewer finding ID: F11
- Source URL: https://github.com/torrust/torrust-tracker/pull/2810#discussion_r4528905314
- Concern: The audit last-updated-utc and the new entry postdate their commit.
- Solution: Valid. Correction entry appended; last-updated-utc set before the commit.
- Current-tree verification: Processing Log of this file.
- Resolution reference: docs(pr-reviews): [#2762] address PR 2810 review round 3
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2810#discussion_r4528940218

### F19 - The log says replies follow the push although they were created before the commit.

- PR number: 2810
- Source review ID: 5847265767
- Reviewer finding ID: F12
- Source URL: https://github.com/torrust/torrust-tracker/pull/2810#discussion_r4528905323
- Concern: The log says replies follow the push although they were created before the commit.
- Solution: Valid. A correction entry records the replies with their created_at times; resolution times are not captured.
- Current-tree verification: Processing Log of this file.
- Resolution reference: docs(pr-reviews): [#2762] address PR 2810 review round 3
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2810#discussion_r4528938744

### F20 - The round-1 log entry was rewritten in place.

- PR number: 2810
- Source review ID: 5847265767
- Reviewer finding ID: F13
- Source URL: https://github.com/torrust/torrust-tracker/pull/2810#discussion_r4528905332
- Concern: The round-1 log entry was rewritten in place.
- Solution: Valid. The 17:55 entry is restored and a correction entry names the rewrite.
- Current-tree verification: Processing Log of this file.
- Resolution reference: docs(pr-reviews): [#2762] address PR 2810 review round 3
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2810#discussion_r4528940640

### F21 - Ten older replies still name PR-branch commits by id.

- PR number: 2810
- Source review ID: 5847265767
- Reviewer finding ID: F14
- Source URL: https://github.com/torrust/torrust-tracker/pull/2810#discussion_r4528905342
- Concern: Ten older replies still name PR-branch commits by id.
- Solution: Valid. The ten replies were edited to cite the Conventional Commit subjects.
- Current-tree verification: The ten replies no longer contain a commit id.
- Resolution reference: docs(pr-reviews): [#2762] address PR 2810 review round 3
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2810#discussion_r4528939707

### F22 - The round-3 log entry again says replies follow the push.

- PR number: 2810
- Source review ID: 5847623438
- Reviewer finding ID: F12
- Source URL: https://github.com/torrust/torrust-tracker/pull/2810#discussion_r4529184349
- Concern: The round-3 log entry again says replies and resolutions follow the push, though the six round-3 replies were created before the commit.
- Solution: Valid. A correction entry records the round-3 reply times (18:34:58Z to 18:35:12Z); the resolution times are not captured.
- Current-tree verification: Processing Log of this file.
- Resolution reference: docs(pr-reviews): [#2762] add the PR 2810 review retrospective
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2810#discussion_r4536112206

### F23 - Round sentences out of order and a wrong "below".

- PR number: 2810
- Source review ID: 5847623438
- Reviewer finding ID: F15
- Source URL: https://github.com/torrust/torrust-tracker/pull/2810#discussion_r4529184361
- Concern: The round-3 sentence sits between the round-1 follow-up and round 2, and the round-3 log entry says "below" for rows that are above it.
- Solution: Valid. The sentences are in round order and the log entry text now says "above" (named in the correction entry).
- Current-tree verification: The paragraph after the findings table and the round-3 log entry.
- Resolution reference: docs(pr-reviews): [#2762] add the PR 2810 review retrospective
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2810#discussion_r4536112506

### F24 - Rows F22 and F23 are FIXED and RESOLVED, but their Reply URLs point to replies that defer the fix; the F22 Solution overstates the log entry.

- PR number: 2810
- Source review ID: 5850003910
- Reviewer finding ID: F16
- Source URL: https://github.com/torrust/torrust-tracker/pull/2810#discussion_r4530967267
- Concern: Rows F22 and F23 are FIXED and RESOLVED, but their Reply URLs point to replies that defer the fix; the F22 Solution overstates the log entry.
- Solution: Valid. Replies posted on both threads after the fix commit, the reply URLs point to them, and the F22 Solution now matches the log entry.
- Current-tree verification: Rows F22 and F23 and the F22 detail.
- Resolution reference: docs(issues): [#2762] close out the package-coverage-check issue
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2810#discussion_r4530990398

### F25 - "4 audit-only" does not match git show --name-status: only one commit changes the audit alone.

- PR number: 2810
- Source review ID: 5850003910
- Reviewer finding ID: F17
- Source URL: https://github.com/torrust/torrust-tracker/pull/2810#discussion_r4530967278
- Concern: "4 audit-only" does not match git show --name-status: only one commit changes the audit alone.
- Solution: Valid. The retrospective now says 1 audit-only commit and names the three that also change ISSUE.md and the evidence.
- Current-tree verification: Review Summary of the retrospective.
- Resolution reference: docs(issues): [#2762] close out the package-coverage-check issue
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2810#discussion_r4530990807

### F26 - The retrospective says the validator checks the table shape only, which understates validate-audit-record.py.

- PR number: 2810
- Source review ID: 5850003910
- Reviewer finding ID: F18
- Source URL: https://github.com/torrust/torrust-tracker/pull/2810#discussion_r4530967285
- Concern: The retrospective says the validator checks the table shape only, which understates validate-audit-record.py.
- Solution: Valid. The retrospective now lists what the validator checks and what it lacks.
- Current-tree verification: Root Causes and Improvements in the retrospective.
- Resolution reference: docs(issues): [#2762] close out the package-coverage-check issue
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2810#discussion_r4530991145

## Processing Log

- 2026-11-10 17:55 UTC - Started audit; round 1 (da2ce7 review 5846429188) processed: all eight findings fixed in one commit and replied. All eight threads resolved after the replies.
- 2026-11-10 18:15 UTC - Round 1 follow-up (Copilot 5846504036) and round 2 (da2ce7 5846796508) added as F9 to F15; replies and resolutions follow the push.
- 2026-11-10 18:36 UTC - Correction of the entries above (round 3, da2ce7 review 5847265767). The 17:55 entry was rewritten in place in the round-2 commit (to 17:37, with new wording); it is restored above. Both stamps are later than the commits that carry them: the round-1 audit commit is dated 17:37:50Z and the round-2 commit 18:10:38Z, so the events they describe happened at about 17:37 and 18:10. Event times from the GitHub capture: the round-1 replies were created 17:36:28Z to 17:36:45Z, the two replies to Copilot's review 17:40:24Z and 17:40:26Z, the round-2 replies 18:08:33Z to 18:08:41Z (all before the commits that cite them). The resolution times of the threads are not captured.
- 2026-11-10 18:36 UTC - Round 3 (da2ce7 review 5847265767, APPROVED with six record findings, F16 to F21 above): the ten older replies that named commits by id were edited to cite the subjects (edits at 18:34:19Z to 18:34:32Z), the stamps were corrected and the hosted run was recorded; replies and resolutions follow the push.
- 2026-11-10 21:33 UTC - Round 4 (da2ce7 review 5847623438, APPROVED with two Nit findings F22, F23) and a retrospective requested by the maintainer's retrospective session. Corrections: the round-3 replies were created 18:34:58Z to 18:35:12Z, before the round-3 commit (18:36:39Z), so the round-3 entry's "replies and resolutions follow the push" was wrong; the round-3 entry's "F16 to F21 below" was edited in place to "above" and the round sentences were reordered. The two round-4 replies (20:00:06Z and 20:00:09Z) said the change would wait for the archive PR; this commit makes it now instead, at the maintainer's request. The resolution times of the threads are not captured. The retrospective is `docs/pr-reviews/pr-2810-review/review-retrospective.md`.
- 2026-11-11 09:25 UTC - Review 5850003910 (da2ce7, APPROVED, F24 to F26) answered with the merged PR: the three replies were posted on 2026-11-10 (after the round-4 head was ACKed) and said the fixes would come in the close-out PR; this commit makes them. The replies to the F22 and F23 threads that this audit cites were posted after the merge, on 2026-11-11 (created_at in the capture), citing the close-out commit subject. The resolution times of the threads are not captured.

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
