---
semantic-links:
  related-artifacts:
    - ISSUE.md
    - implementation-retrospective.md
    - manual-verification-evidence.md
---

# Agent Review Reports: #2386 Minimum Self-Hosted Runner Capacity

## Reports

### 2026-10-02 07:18 UTC - Task Reviewer

- Invocation scope: Final pre-close review of #2386 AC1 to AC5, manual scenarios M1 to M4, the
  Decision section, the workflow checkpoints, the new implementation retrospective, and the
  uncommitted `open-pull-request` skill paragraph on closing keywords.
- Inputs: `ISSUE.md`, `manual-verification-evidence.md` V1 to V4, `implementation-retrospective.md`,
  `docs/self-hosted-runner.md`, EPIC #1840 rows 16 to 18, the PR #2383 and PR #2403 review audits,
  `.github/skills/dev/git-workflow/open-pull-request/SKILL.md`, and the working-tree diff on
  branch `2386-1840-runner-capacity-close-out` (based on `develop` with PRs #2389 and #2403).
- Evidence:
  - AC1: V1 and V2 record total build (639 s, 1065 s), `build 3/3` compile (614.1 s, 1029.7 s),
    memory and swap, and no out-of-memory kills. Build and window durations match their
    timestamps (10:39, 17:45, 84 minutes). The V1 commit `ec80aa0` is an ancestor of `develop`.
  - AC2: 639 + 162 = 801 s (13.4 minutes) and 1065 + 162 = 1227 s (20.5 minutes) against
    900 s; the 1.6-minute margin and 123 MiB free (126,192 KiB) match V1.
  - AC3: the Decision section dates the server size (2026-09-30) and the runner count and
    queueing (2026-10-01), and records resilience, recheck triggers, and the maintainer's
    rationale. Its replay figures (140 to 11 and 239 to 32 minutes; 13 and 63 for two runners)
    match V3, EPIC row 18, and the retrospective.
  - AC4: the guide has the server facts with the public IP marked as not recorded (the only
    address-like match is the firewall CIDR `0.0.0.0/0`), the Cost section with the 0.77 USD
    per run estimate and a break-even of about 100 runs per month, the one-runner-per-server
    causes, "Add Runner Capacity", and the queue-time and run-volume commands.
  - AC5: EPIC rows 16 (#2374, DONE) and 17 (#2386, IN_PROGRESS until close-out) exist.
  - M1 to M4 are `DONE` with commands, observed output, and conclusions in V1 to V4.
  - Retrospective: no commit SHAs; its claims match the spec, evidence, and audits, including
    the original pre-filter replay, which the evidence history shows was an `awk` one-liner.
  - Skill edit: the merge tool appends the whole body to the merge message
    (`contrib/dev-tools/git/github-merge.py` "Pull request description"); the scan command
    matches `Closes #2386` in backticks and `fixed: #12`, and not `Related to #2386`.
  - `linter markdown` and `linter cspell` exit `0` before and after this report. `linter all`
    was not rerun by this review; the change set is Markdown only.
- Findings:
  - Low: `ISSUE.md` line 351 lists PR #2389 as the implementation PR and omits PR #2403, which
    delivered T1 to T3; the Acceptance Verification rows at lines 287 to 288 cite only #2389.
  - Low: EPIC #1840 row 17 (`EPIC.md` line 88) cites only PR #2389 for the measurements and
    decision that PR #2403 merged.
  - Low: `manual-verification-evidence.md` line 4 keeps `last-updated-utc` 2026-09-30 15:17,
    although the PR #2403 review fixes changed the file on 2026-10-01.
  - Low: `docs/self-hosted-runner.md` lines 401 to 403 gate new capacity only on measured queue
    time and do not say the Decision tries cancelling superseded runs (#2402) first; the link
    points to the `docs/issues/open/` spec path, which breaks when the spec moves.
- Verdict: REVIEW PASSED
- Follow-up actions:
  - Close-out: add PR #2403 to the spec references, set EPIC row 17 to DONE with both PRs,
    refresh the evidence `last-updated-utc`, and point the guide's link to
    `docs/issues/closed/` when the spec moves.
  - #2402: state the cancel-first order in the guide's "Add Runner Capacity" section.
