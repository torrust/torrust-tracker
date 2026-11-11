---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2826 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2826.

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

R1 to R6 are Copilot's review 5858141026 (R1, R5 and R6 are its own IDs; R2 and R4 are audit-assigned to its unlabelled findings). F1 to F16 are da2ce7's review 5858207086, F17 to F23 his review 5859095235 and F24 to F30 his review 5859464376. Every row is an inline thread.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| R1 | review-finding:pr-2826-r1 | Copilot | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| R2 | review-finding:pr-2826-r2 | Copilot | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| R5 | review-finding:pr-2826-r5 | Copilot | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| R4 | review-finding:pr-2826-r4 | Copilot | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| R6 | review-finding:pr-2826-r6 | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-2826-f11 | Human | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F12 | review-finding:pr-2826-f12 | Human | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F13 | review-finding:pr-2826-f13 | Human | Major | documentation | ORIGINAL | FIXED | RESOLVED |
| F1 | review-finding:pr-2826-f1 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2826-f2 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2826-f3 | Human | Minor | correctness | RE_RAISE_OF:R4 | FIXED | RESOLVED |
| F4 | review-finding:pr-2826-f4 | Human | Minor | testing | RE_RAISE_OF:R5 | FIXED | RESOLVED |
| F14 | review-finding:pr-2826-f14 | Human | Minor | correctness | RE_RAISE_OF:R2 | FIXED | RESOLVED |
| F15 | review-finding:pr-2826-f15 | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2826-f5 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2826-f6 | Human | Minor | metadata | RE_RAISE_OF:R1 | FIXED | RESOLVED |
| F7 | review-finding:pr-2826-f7 | Human | Minor | formatting | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2826-f8 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-2826-f9 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-2826-f10 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F16 | review-finding:pr-2826-f16 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F17 | review-finding:pr-2826-f17 | Human | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F18 | review-finding:pr-2826-f18 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F20 | review-finding:pr-2826-f20 | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F19 | review-finding:pr-2826-f19 | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F22 | review-finding:pr-2826-f22 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F21 | review-finding:pr-2826-f21 | Human | Nit | formatting | ORIGINAL | FIXED | RESOLVED |
| F23 | review-finding:pr-2826-f23 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F24 | review-finding:pr-2826-f24 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F25 | review-finding:pr-2826-f25 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F26 | review-finding:pr-2826-f26 | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F27 | review-finding:pr-2826-f27 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F28 | review-finding:pr-2826-f28 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F29 | review-finding:pr-2826-f29 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F30 | review-finding:pr-2826-f30 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### R1 - Use the reserved -spec branch name

- PR number: 2826
- Source review ID: 5858141026
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4537413997
- Concern: `branch:` names the implementation branch in a spec-only PR.
- Solution: `branch:` is the `-spec` name.
- Current-tree verification: `grep '^branch:'` on the spec shows the `-spec` name.
- Resolution reference: docs(issues): [#2822] add issue specification for the wait-for-pr-review --gh-command option
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4538029384

### R2 - Define timeout and cancellation behavior for the optional lookup

- PR number: 2826
- Source review ID: 5858141026
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4537414064
- Concern: The `api user` lookup shares one deadline, so a stall can consume the wait budget; no budget, cancellation or scenario was specified.
- Solution: The lookup has its own bound, shares the `interrupted` flag, and is covered by M4.
- Current-tree verification: Design and Ownership Review and M4 read in the spec at this head.
- Resolution reference: docs(issues): [#2822] add issue specification for the wait-for-pr-review --gh-command option
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4538029920

### R5 - Make the required test-design review explicit

- PR number: 2826
- Source review ID: 5858141026
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4537414120
- Concern: The plan adds tests but omits the progressive test-design gate.
- Solution: The Implementation Plan names `write-unit-test`, the prose-first AAA review per increment and the maintainer stop.
- Current-tree verification: The paragraph after the Implementation Plan table.
- Resolution reference: docs(issues): [#2822] add issue specification for the wait-for-pr-review --gh-command option
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4538030484

### R4 - Capture poller output and exit status to test gh_unavailable

- PR number: 2826
- Source review ID: 5858141026
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4537414157
- Concern: M2 run in a terminal ends in `tty_refusal`, exit 2.
- Solution: M2 redirects stdout and stderr to files and prints the exit status.
- Current-tree verification: M2 row of the manual scenarios table.
- Resolution reference: docs(issues): [#2822] add issue specification for the wait-for-pr-review --gh-command option
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4538031056

### R6 - Include validation findings and reusable lessons in the retrospective

- PR number: 2826
- Source review ID: 5858141026
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4537414197
- Concern: The completion review restricted the retrospective to plan deviations.
- Solution: The section covers deviations, validation findings and lessons, with a progress-log explanation otherwise.
- Current-tree verification: Implementation Completion Review section.
- Resolution reference: docs(issues): [#2822] add issue specification for the wait-for-pr-review --gh-command option
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4538031568

### F11 - Option A compares --gh-command with itself

- PR number: 2826
- Source review ID: 5858207086
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4537466767
- Concern: The rename touched every occurrence, including the old name in the pros-and-cons table.
- Solution: Option A reads "instead of `--gh`" and "A1 (`--gh`)".
- Current-tree verification: D1 table row A.
- Resolution reference: docs(issues): [#2822] add issue specification for the wait-for-pr-review --gh-command option
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4538032074

### F12 - AC3 and AC6 cannot both pass

- PR number: 2826
- Source review ID: 5858207086
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4537466779
- Concern: AC3 kept stderr unchanged while AC6 added fields.
- Solution: AC3 pins exit codes, existing record kinds and JSON-only stderr; the fields have their own AC4.
- Current-tree verification: Acceptance Criteria.
- Resolution reference: docs(issues): [#2822] add issue specification for the wait-for-pr-review --gh-command option
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4538032630

### F13 - PR title, description and EPIC row 20 describe the spec before its decisions

- PR number: 2826
- Source review ID: 5858207086
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4537466785
- Concern: Title, description and row 20 still said `--gh` and open questions.
- Solution: Title and description were edited on the PR; row 20 says `--gh-command`.
- Current-tree verification: `gh pr view 2826 --json title,body`; row 20 in `EPIC.md`.
- Resolution reference: docs(issues): [#2822] add issue specification for the wait-for-pr-review --gh-command option
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4538033322

### F1 - Backlog item labels name nothing in the tree

- PR number: 2826
- Source review ID: 5858207086
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4537466795
- Concern: A-labels referred to a document outside the repository.
- Solution: The items are named in words and the backlog is described as external.
- Current-tree verification: Background and Out of Scope read in the spec.
- Resolution reference: docs(issues): [#2822] add issue specification for the wait-for-pr-review --gh-command option
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4538033919

### F2 - The CLI output contract ADR is decided, not under decision

- PR number: 2826
- Source review ID: 5858207086
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4537466805
- Concern: Stale status of the ADR and #2715.
- Solution: The ADRs are linked by path and described as decided; the risk is restated.
- Current-tree verification: Architectural Decisions section; both ADR files exist under `docs/adrs/`.
- Resolution reference: docs(issues): [#2822] add issue specification for the wait-for-pr-review --gh-command option
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4538034487

### F3 - M2 ends in tty_refusal, not gh_unavailable

- PR number: 2826
- Source review ID: 5858207086
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4537466814
- Concern: Same as R4.
- Solution: Same fix as R4.
- Current-tree verification: M2 row.
- Resolution reference: docs(issues): [#2822] add issue specification for the wait-for-pr-review --gh-command option
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4538035029

### F4 - The plan omits the progressive test-development loop

- PR number: 2826
- Source review ID: 5858207086
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4537466820
- Concern: Missing loop, design-review checkpoint and template sections.
- Solution: The loop, the checkpoint, and `Not applicable` sections were added.
- Current-tree verification: Implementation Plan and Design and Ownership Review.
- Resolution reference: docs(issues): [#2822] add issue specification for the wait-for-pr-review --gh-command option
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4538035559

### F14 - A best-effort lookup cannot keep its promise under one deadline

- PR number: 2826
- Source review ID: 5858207086
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4537466824
- Concern: Shared deadline, lookup before `wait_started`, and the unusable-command attribution.
- Solution: Own bound, `wait_started` first, a separate `gh_identity` record, attribution to the first non-lookup call.
- Current-tree verification: Design and Ownership Review and D2.
- Resolution reference: docs(issues): [#2822] add issue specification for the wait-for-pr-review --gh-command option
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4538036112

### F15 - AC6 has no manual scenario and no task documents the new fields

- PR number: 2826
- Source review ID: 5858207086
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4537466827
- Concern: No scenario or task covered the fields.
- Solution: AC4, M3, M4, T4 and AC6 cover them and the ACs are reordered.
- Current-tree verification: Acceptance Criteria and the scenario table.
- Resolution reference: docs(issues): [#2822] add issue specification for the wait-for-pr-review --gh-command option
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4538036676

### F5 - Progress-log stamps postdate the commits

- PR number: 2826
- Source review ID: 5858207086
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4537466833
- Concern: Stamps were later than the commits that carry them.
- Solution: The commits were squashed; stamps were set to the event times (issue creation from GitHub, decisions from the commit time).
- Current-tree verification: `git log --format=%cI`; the Progress Log.
- Resolution reference: docs(issues): [#2822] align the spec with the identity-record design and the lookup bound
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4538037380

### F6 - branch: names the implementation branch

- PR number: 2826
- Source review ID: 5858207086
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4537466845
- Concern: Same as R1.
- Solution: Same fix as R1.
- Current-tree verification: Spec frontmatter.
- Resolution reference: docs(issues): [#2822] add issue specification for the wait-for-pr-review --gh-command option
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4538037920

### F7 - Commit subjects have no scope and the PR title departs from the house form

- PR number: 2826
- Source review ID: 5858207086
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4537466854
- Concern: Unscoped subjects and a number suffix in the title.
- Solution: One scoped commit with a body, and the PR retitled `docs(issues): [#2822] ...`.
- Current-tree verification: `git log -1 --format=%s`; `gh pr view 2826 --json title`.
- Resolution reference: docs(issues): [#2822] add issue specification for the wait-for-pr-review --gh-command option
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4538038461

### F8 - EPIC progress log has no line for row 20

- PR number: 2826
- Source review ID: 5858207086
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4537466858
- Concern: No log line.
- Solution: A line was added to the EPIC progress log.
- Current-tree verification: Last EPIC progress-log lines.
- Resolution reference: docs(issues): [#2822] add issue specification for the wait-for-pr-review --gh-command option
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4538038963

### F9 - D2 message-change wording is half done

- PR number: 2826
- Source review ID: 5858207086
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4537466862
- Concern: Which messages change was unclear.
- Solution: D2 says `gh_api_error` is changed and `gh_unavailable` loses its fixed wording.
- Current-tree verification: D2 in the spec.
- Resolution reference: docs(issues): [#2822] add issue specification for the wait-for-pr-review --gh-command option
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4538039474

### F10 - Checkpoints and T1 policy are stale

- PR number: 2826
- Source review ID: 5858207086
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4537466867
- Concern: Mixed done/open checkpoint and a stale T1 policy.
- Solution: The checkpoint is split and T1 reads "Done in the spec PR".
- Current-tree verification: Workflow Checkpoints and Commit Points.
- Resolution reference: docs(issues): [#2822] add issue specification for the wait-for-pr-review --gh-command option
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4538040055

### F16 - Decisions rest on an unlinked instruction

- PR number: 2826
- Source review ID: 5858207086
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4537466879
- Concern: The maintainer's instruction has no link.
- Solution: The concern is stated on its merits and the progress log says the instruction was given in the session chat and has no durable link.
- Current-tree verification: Decisions intro and Progress Log.
- Resolution reference: docs(issues): [#2822] align the spec with the identity-record design and the lookup bound
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4538040664

### F17 - In Scope still puts gh_login in wait_started

- PR number: 2826
- Source review ID: 5859095235
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4538163544
- Concern: In Scope and the ADR text contradicted D2.
- Solution: Both name the `gh_identity` record.
- Current-tree verification: Scope and Architectural Decisions sections.
- Resolution reference: docs(issues): [#2822] align the spec with the identity-record design and the lookup bound
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4538212209

### F18 - AC4's guarantee fails at its boundary and for events during a stall

- PR number: 2826
- Source review ID: 5859095235
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4538163564
- Concern: The bound failed at timeout 10 and a stall can absorb an event.
- Solution: The bound is `min(10 s, timeout / 4)`; the baseline limit is recorded as known, with `--since-head-sha` as the caller's answer.
- Current-tree verification: Budget and Baseline bullets; AC4.
- Resolution reference: docs(issues): [#2822] align the spec with the identity-record design and the lookup bound
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4538212715

### F20 - Two existing scenarios cannot stay as they are

- PR number: 2826
- Source review ID: 5859095235
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4538163577
- Concern: The new record changes the exact kinds those scenarios assert.
- Solution: D2 says their expected sequences gain `gh_identity`.
- Current-tree verification: D2 test bullet.
- Resolution reference: docs(issues): [#2822] align the spec with the identity-record design and the lookup bound
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4538213273

### F19 - M4 ends in deadline_exceeded, and no task runs it

- PR number: 2826
- Source review ID: 5859095235
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4538163584
- Concern: M4 used a 5 s timeout and T4 ran only M1 to M3.
- Solution: M4 uses 60 s (and a second run with 10 s); T4 runs M1 to M4.
- Current-tree verification: M4 row and T4.
- Resolution reference: docs(issues): [#2822] align the spec with the identity-record design and the lookup bound
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4538213811

### F22 - The review round is unrecorded

- PR number: 2826
- Source review ID: 5859095235
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4538163594
- Concern: No audit record, four threads said to lack a reply, replies cite a branch id.
- Solution: This record was added; all 21 round-1 threads had a reply and were resolved when the review was read, as the thread data shows, and the replies now cite commit subjects.
- Current-tree verification: GraphQL review threads: every thread has a reply and is resolved.
- Resolution reference: docs(pr-reviews): [#2822] add the review audit for PR #2826
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4538275633

### F21 - Decisions nest under Regression Test Strategy

- PR number: 2826
- Source review ID: 5859095235
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4538163605
- Concern: Section order.
- Solution: The two sections are after `### Decisions`.
- Current-tree verification: Heading order in the spec.
- Resolution reference: docs(issues): [#2822] align the spec with the identity-record design and the lookup bound
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4538214349

### F23 - fails on the first call is no longer exact

- PR number: 2826
- Source review ID: 5859095235
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4538163613
- Concern: Risk wording.
- Solution: The Risk says first non-lookup call.
- Current-tree verification: Risks section.
- Resolution reference: docs(issues): [#2822] align the spec with the identity-record design and the lookup bound
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4538214957

### F24 - AC4 and the Budget bullet promise a stalled lookup never changes the result

- PR number: 2826
- Source review ID: 5859464376
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4538446476
- Concern: The spec promised no change while the Baseline bullet and the Risks said otherwise, and a spent quarter of the budget can end in `deadline_exceeded`.
- Solution: The Budget bullet and AC4 state what holds: no error kind or exit code of its own, ends at its bound, spends up to a quarter of the budget, and the baseline limit applies.
- Current-tree verification: Budget bullet, AC4 and Risks in the spec.
- Resolution reference: docs(issues): [#2822] state the lookup budget, the baseline limit and the scenarios it changes
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4538503387

### F25 - The baseline limit names the wrong condition for a push

- PR number: 2826
- Source review ID: 5859464376
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4538446489
- Concern: `--since` alone does not close the push case; only `--since-head-sha` does.
- Solution: The Baseline bullet and the Risks say reviews and comments are absorbed without either flag, and a push without `--since-head-sha`.
- Current-tree verification: Baseline bullet; `wait.rs:100-104` and the `--since` help in `main.rs`.
- Resolution reference: docs(issues): [#2822] state the lookup budget, the baseline limit and the scenarios it changes
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4538503961

### F26 - Four more existing scenarios are changed by gh_identity

- PR number: 2826
- Source review ID: 5859464376
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4538446501
- Concern: The signal scenarios and the scenario that parses the rest of stderr as one JSON value are affected, and the `/dev/full` scenario depends on best-effort writing.
- Solution: D2 names every affected scenario, and a Best effort bullet says `gh_identity` is written best effort like `wait_started`.
- Current-tree verification: D2 test bullet and the Best effort bullet.
- Resolution reference: docs(issues): [#2822] state the lookup budget, the baseline limit and the scenarios it changes
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4538504503

### F27 - The first Processing Log entry cites a rewritten branch commit by id

- PR number: 2826
- Source review ID: 5859464376
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4538446509
- Concern: The log cited a branch id and folded the audit start under another stamp.
- Solution: The log says "the first round's head" without the id, and the audit start has its own entry.
- Current-tree verification: The Processing Log of this record.
- Resolution reference: docs(pr-reviews): [#2822] correct the audit after the third review of PR #2826
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4538505125

### F28 - The count of edited replies is 21, the forge shows 20

- PR number: 2826
- Source review ID: 5859464376
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4538446517
- Concern: The reply on F7's thread cites no commit id and needed no edit.
- Solution: The log says 20 replies were edited and that F7's reply needed no edit.
- Current-tree verification: The Processing Log of this record.
- Resolution reference: docs(pr-reviews): [#2822] correct the audit after the third review of PR #2826
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4538505859

### F29 - The replies on F5 and F16 describe the first commit's fix

- PR number: 2826
- Source review ID: 5859464376
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4538446521
- Concern: The replies cited the first commit while their rows cite the second.
- Solution: Both replies were edited to cite the second commit and its content.
- Current-tree verification: The F5 and F16 replies read on the forge.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4538037380; https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4538040664
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4538506500

### F30 - R1 severity is stated, not inferred; N/A for audit-assigned IDs

- PR number: 2826
- Source review ID: 5859464376
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4538446528
- Concern: R1's source opens with `[Minor][R1]`, and R2 and R4 should record `N/A`.
- Solution: R1 is `Minor`; R2 and R4 record `N/A`.
- Current-tree verification: Rows and entries of R1, R2 and R4 in this record.
- Resolution reference: docs(pr-reviews): [#2822] correct the audit after the third review of PR #2826
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2826#discussion_r4538507026

## Processing Log

- 2026-11-11 11:39 UTC - Review 5858141026 (Copilot) submitted on the first round's head; review 5858207086 (da2ce7) followed at 11:45 UTC.
- 2026-11-11 12:46 UTC - Replied to and resolved the 21 threads of those reviews after the squashed commit `docs(issues): [#2822] add issue specification for the wait-for-pr-review --gh-command option` (the reply times are the GitHub `created_at` of the replies).
- 2026-11-11 12:59 UTC - Review 5859095235 (da2ce7) submitted on the new head: F17 to F23.
- 2026-11-11 13:04 UTC - Replied to and resolved F17 to F21 and F23 after the commit `docs(issues): [#2822] align the spec with the identity-record design and the lookup bound` was pushed; the six round-1 replies that cited a branch id were edited to cite the commit subject.
- 2026-11-11 13:05 UTC - Started the audit record from the GitHub review data.
- 2026-11-11 13:10 UTC - Replied to and resolved F22. The reply was posted before the audit commit reached GitHub: the first push of it at about 13:08 was blocked by two failing fake-`gh` scenarios of the poller's `tests/cli.rs` (unrelated to this change), and the retry succeeded at about 13:12 UTC.
- 2026-11-11 13:27 UTC - Review 5859464376 (da2ce7, `APPROVED`) submitted on the second round's head: F24 to F30, none blocking.
- 2026-11-11 13:32 UTC - Replied to and resolved the seven threads of that review and edited the replies on F5 and F16. The replies on F24 to F26 cite the pushed commit `docs(issues): [#2822] state the lookup budget, the baseline limit and the scenarios it changes`; the replies on F27, F28 and F30 were posted just before the commit that fixes them was pushed.

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
