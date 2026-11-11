---
pr-number: 2827
pr-url: https://github.com/torrust/torrust-tracker/pull/2827
last-updated-utc: "2026-11-11 13:25"
---

# PR #2827 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2827.

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

Round 1: da2ce7 review 5858428287 (CHANGES_REQUESTED) on the head `refactor(primitives): [#2823] remove unused AddAssign for AggregateActiveSwarmMetadata` (six inline findings F1 to F6, one Major, four Minor, one Nit): fixed in the commit `docs(issues): [#2823] address review round 1 of the spec`.

Round 2: da2ce7 review 5859120491 (COMMENTED) on the head `docs(issues): [#2823] address review round 1 of the spec` (four inline findings F7 to F10, three Minor, one Nit): fixed in `docs(issues): [#2823] correct spec stamps, commit citation and wording` and `docs(pr-reviews): [#2827] add the PR 2827 review audit`. The Copilot review (`copilot-pull-request-reviewer`, COMMENTED) recorded no findings.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2827-f1 | Human | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2827-f2 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2827-f3 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2827-f4 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2827-f5 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2827-f6 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2827-f7 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2827-f8 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-2827-f9 | Human | Minor | maintainability | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-2827-f10 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The Risks bullet calls the crate internal, but it is published at 3.1.0, so removing the impl is a breaking change.

- PR number: 2827
- Source review ID: 5858428287
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2827#discussion_r4537644927
- Concern: The Risks bullet calls the crate internal, but it is published at 3.1.0, so removing the impl is a breaking change.
- Solution: Valid. The Risks bullet says the crate is publishable (3.1.0), the removal is a breaking change, and the next publish is a major version under the versioning ADR, already required by the `Peer: Ord` removal (#2798). The ADR is listed under Related ADRs.
- Current-tree verification: grep -n "3.1.0\|major" in the spec Risks section; `packages/primitives/Cargo.toml` has `publish.workspace = true` and version 3.1.0.
- Resolution reference: docs(issues): [#2823] address review round 1 of the spec
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2827#discussion_r4538056955

### F2 - Two counts and one history statement are wrong.

- PR number: 2827
- Source review ID: 5858428287
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2827#discussion_r4537644932
- Concern: Two counts and one history statement are wrong.
- Solution: Valid. The spec says ten `get_metrics` impls in eight modules, eleven production copies with the registry plus the test body, and that `9a43815d0` added the impl.
- Current-tree verification: `grep -rn "fn get_metrics" packages/torrent-repository-benchmarking/src` and `grep -rn "total_complete +=" --include=*.rs .` give the ten impls in eight modules, the registry and the test.
- Resolution reference: docs(issues): [#2823] address review round 1 of the spec
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2827#discussion_r4538057355

### F3 - The spec progress records lag what the branch delivers.

- PR number: 2827
- Source review ID: 5858428287
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/2827#discussion_r4537644940
- Concern: The spec progress records lag what the branch delivers.
- Solution: Valid. T1 lists the commands that ran with the stable toolchain, the checkpoints and criteria are ticked, `related-pr` is 2827 and the Acceptance Verification table exists.
- Current-tree verification: Inspection of the spec frontmatter, Implementation Plan, checkpoints and Acceptance Verification table.
- Resolution reference: docs(issues): [#2823] address review round 1 of the spec
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2827#discussion_r4538057761

### F4 - `epic: 1347` makes the spec a subissue the EPIC does not list.

- PR number: 2827
- Source review ID: 5858428287
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/2827#discussion_r4537644951
- Concern: `epic: 1347` makes the spec a subissue the EPIC does not list.
- Solution: Valid. Followed the sibling specs of #2798 and #2804: `epic: null`, folder `2823-addassign-aggregate-swarm-metadata`, and a sentence saying the analysis register is not published and the issue is related to the EPIC, not a listed subissue.
- Current-tree verification: `git ls-files docs/issues/open | grep 2823` shows the folder without the `1347-` prefix; frontmatter has `epic: null`.
- Resolution reference: docs(issues): [#2823] address review round 1 of the spec
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2827#discussion_r4538058282

### F5 - The live summation keeps the same release-mode wrap, so the commit body and description overstate what the removal does.

- PR number: 2827
- Source review ID: 5858428287
- Reviewer finding ID: F5
- Source URL: https://github.com/torrust/torrust-tracker/pull/2827#discussion_r4537644963
- Concern: The live summation keeps the same release-mode wrap, so the commit body and description overstate what the removal does.
- Solution: Valid in part. The spec (Background, Out of Scope, lessons note) and the PR description now say the live summations keep the same unchecked `u64 +=` and that this is out of scope. The body of the first commit still has the old sentence: amending it needs a force-push, which was blocked and is not done without the maintainer's OK. Review round 2 notes this as partly fixed.
- Current-tree verification: The PR description and the spec state it; `git log -1 --format=%B` of the first commit still has the old sentence.
- Resolution reference: docs(issues): [#2823] address review round 1 of the spec
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2827#discussion_r4538058647

### F6 - T3 and its criterion address a session, not a record.

- PR number: 2827
- Source review ID: 5858428287
- Reviewer finding ID: F6
- Source URL: https://github.com/torrust/torrust-tracker/pull/2827#discussion_r4537644973
- Concern: T3 and its criterion address a session, not a record.
- Solution: Valid. T3 and its acceptance criterion are removed, and the Out of Scope text no longer names a session.
- Current-tree verification: grep -n "primitives tests" in the spec prints nothing.
- Resolution reference: docs(issues): [#2823] address review round 1 of the spec
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2827#discussion_r4538059104

### F7 - The progress-log entry claims a reword the bytes contradict, and has no time.

- PR number: 2827
- Source review ID: 5859120491
- Reviewer finding ID: F7
- Source URL: https://github.com/torrust/torrust-tracker/pull/2827#discussion_r4538183021
- Concern: The progress-log entry claims a reword the bytes contradict, and has no time.
- Solution: Valid. The entry now says the PR description was corrected and that the first commit body was not amended; both log entries carry times from `git log`, and `last-updated-utc` is updated.
- Current-tree verification: grep -n "^- 2026" in the spec shows the two entries with times; the first commit body is unchanged.
- Resolution reference: docs(issues): [#2823] correct spec stamps, commit citation and wording
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2827#discussion_r4538344331

### F8 - The AC3 evidence cites a PR-branch commit id.

- PR number: 2827
- Source review ID: 5859120491
- Reviewer finding ID: F8
- Source URL: https://github.com/torrust/torrust-tracker/pull/2827#discussion_r4538183034
- Concern: The AC3 evidence cites a PR-branch commit id.
- Solution: Valid. The AC3 row cites the commit subject. A search of the spec for other 9-digit hex ids finds only the history commits that are on develop.
- Current-tree verification: grep -n -E "[0-9a-f]{9}" in the spec prints only `9a43815d0`, `144a338b9` and `3d6fc651d`.
- Resolution reference: docs(issues): [#2823] correct spec stamps, commit citation and wording
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2827#discussion_r4538344778

### F9 - All six threads are answered and resolved, but the head has no review audit record.

- PR number: 2827
- Source review ID: 5859120491
- Reviewer finding ID: F9
- Source URL: https://github.com/torrust/torrust-tracker/pull/2827#discussion_r4538183037
- Concern: All six threads are answered and resolved, but the head has no review audit record.
- Solution: Valid. This record is added for F1 to F10; the replies that cite a branch id map to commit subjects here.
- Current-tree verification: `validate-audit-record.py --pr-number 2827` exit code exit 0 (run before the push).
- Resolution reference: docs(pr-reviews): [#2827] add the PR 2827 review audit
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2827#discussion_r4538355726

### F10 - Two wording slips: a repeated command and a call of the test body a test helper.

- PR number: 2827
- Source review ID: 5859120491
- Reviewer finding ID: F10
- Source URL: https://github.com/torrust/torrust-tracker/pull/2827#discussion_r4538183048
- Concern: Two wording slips: a repeated command and a call of the test body a test helper.
- Solution: Valid. The repeated command is removed and the copy is named as the body of the test `it_should_get_metrics`. Correction to the reply: the reply says three places named it a helper; only one did (the lessons note), and the Background and the lessons note now name the test.
- Current-tree verification: grep -n "helper\|it_should_get_metrics" in the spec shows the test name in lines 49 and 180 and no "test helper".
- Resolution reference: docs(issues): [#2823] correct spec stamps, commit citation and wording
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2827#discussion_r4538345161

## Processing Log

- 2026-11-11 12:05 UTC - Review round 1 submitted by da2ce7 (F1 to F6); Copilot review at 12:08 UTC with no findings.
- 2026-11-11 12:49 UTC - Replied to F1 to F6 (12:48 to 12:49 UTC); the six threads were resolved afterwards.
- 2026-11-11 13:01 UTC - Review round 2 submitted by da2ce7 (F7 to F10).
- 2026-11-11 13:17 UTC - Replied to F7, F8 and F10 after the fix commit was pushed.
- 2026-11-11 13:18 UTC - Replied to F9 after committing this record locally; `validate-audit-record.py --pr-number 2827` then exited 0 (10 rows, 0 failures).
