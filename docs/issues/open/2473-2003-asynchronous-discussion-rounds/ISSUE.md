---
schema-version: 1
doc-type: issue
issue-type: task
status: in-progress
priority: p3
epic: 2003
github-issue: 2473
spec-path: docs/issues/open/2473-2003-asynchronous-discussion-rounds/ISSUE.md
branch: "2473-asynchronous-discussion-rounds"
related-pr: null
last-updated-utc: "2026-10-08 06:44"
semantic-links:
  skill-links:
    - create-issue
    - create-markdown-template
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - .github/skills/dev/planning/create-markdown-template/SKILL.md
    - docs/discussions/AGENTS.md
    - docs/templates/README.md
    - docs/index.md
    - "issue #2003"
---

<!-- skill-link: create-issue -->

# Issue #2473 - Run Design Discussions as Asynchronous, Attributed Rounds

**Parent EPIC:** #2003 - Overhaul: Automation Tools and AI Agent Guardrails

## Goal

Let several participants contribute to a design discussion in `docs/discussions/` through short,
independently merged pull requests ("rounds"), each participant writing only their own positions,
recorded per topic and attributed, so that only the final decision needs agreement.

## Background

[`docs/discussions/AGENTS.md`](../../../discussions/AGENTS.md) describes a single-PR lifecycle:
draft the discussion on a branch, open a pull request, request review from the owner of the
affected work, then record the outcome in the discussion's `README.md`. In practice the discussion
itself then happens in the pull request:

- In PR #2428, the owner of EPIC #2003 answered the open questions of the three 2026-10-03
  discussions in review bodies, and the author transcribed the answers into each `## Outcome`
  section, two of them "for him to confirm against the review text".
- In PR #2467 (AI model provenance in commits), the EPIC owner posted a labelled draft position
  (A1-A5, B1-B6, C1-C7) as a PR conversation comment, separate from his review of the document. The
  maintainer declined to transcribe it in that PR and proposed the model this issue specifies; the
  response is <https://github.com/torrust/torrust-tracker/pull/2467#issuecomment-6041029813>.

The single-PR model has these costs:

1. **Someone writes for someone else.** The PR author becomes the transcriber of other
   participants' views, and the transcription then needs the original author's confirmation.
2. **The PR author leads the discussion.** The pull request stays open until everyone has spoken,
   and it carries both document-quality review and substantive disagreement.
3. **Positions are lost or flattened.** Views posted in PR comments do not enter history (the merge
   subject takes only the title), and a synthesis loses who held which position and why.
4. **It does not scale.** Each additional participant lengthens the same pull request.

## Proposed Model

A discussion is a document that grows through rounds:

- **Round.** One participant's pull request that adds or revises only that participant's own
  entries in a discussion. Opening the discussion is the first round.
- **Own words only.** Nobody transcribes another participant's position. A position posted
  elsewhere (a PR comment, an issue comment) is linked, not copied, until its author adds it in a
  round.
- **Merge gate for a round.** Document quality only: links, linters, frontmatter, attribution, and
  the accuracy of factual claims about the repository. Reviewers do not block a round because they
  disagree with an opinion; they reply in their own round.
- **Per-topic, attributed, append-only.** Each topic (open question) has a stable ID (`Q1`, `Q2`,
  ...). Under each topic, positions are appended as entries naming the participant, the date, and
  the round's pull request. A participant changes position by adding a new entry; they may correct
  only the wording of their own earlier entry. No one edits another participant's entry.
- **Only the decision needs agreement.** The decision owner (the owner of the affected issue or
  EPIC) records `## Outcome` in a separate decision pull request that cites the entries it relies
  on. That pull request is the one where agreement is required. Unchanged from today, the Outcome
  links the canonical document (issue or EPIC specification, ADR) that carries the decision.

## Scope

### In Scope

- Rewrite the Lifecycle section of `docs/discussions/AGENTS.md` for the round model: opening round,
  contribution rounds, decision round, merge gates, and the rules above.
- Add a canonical discussion template, `docs/templates/DISCUSSION.md`, with a metadata table,
  context sections owned by the opening author, a `## Topics` section with one subsection per
  `Q<n>` holding its question and its `Positions` entries, and a `## Outcome` section owned by the
  decision owner.
- Register the template in `docs/templates/README.md` and `docs/index.md`, and link it from
  `docs/discussions/AGENTS.md`.
- Define how AI-assisted entries are attributed: an entry belongs to the human participant who
  submits it; AI assistance is stated in the entry, consistent with the Participants rows of the
  existing discussions.

### Out of Scope

- Restructuring the three 2026-10-03 discussions under #2003. Their Outcomes are already recorded
  and they remain as historical records.
- Restructuring the AI model provenance discussion merged in PR #2467 in this issue. Its
  participants adopt the template in its next round, after this issue merges.
- Deciding the open questions of any existing discussion.
- A new skill or an automated check for discussion structure. A follow-up may propose one if the
  template is adopted and attribution drift appears.

## Architectural Decisions

- Related ADRs:
  `docs/adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md`
  (the record must live in tracked files, not in a tool's state or a pull-request thread).
- ADRs to create: None known. The discussion lifecycle is documentation process owned by
  `docs/discussions/AGENTS.md`; create an ADR only if review shows the round model changes how
  decisions are made repository-wide.

## Design and Ownership Review

| Part of a discussion            | Owner                                    | Changed in                    |
| ------------------------------- | ---------------------------------------- | ----------------------------- |
| Metadata table and context      | Opening author                           | Opening round; later by owner |
| Topic list (`Q<n>` and wording) | Opening author; others may add topics    | Any round, append-only        |
| A position entry                | The participant named in the entry       | That participant's rounds     |
| `## Outcome`                    | Decision owner of the affected issue/EPIC | Decision round                |

No child processes, asynchronous I/O, network readiness, or test fixtures are involved.

## Bug-Fix Process

Not applicable.

## Regression Test Strategy

Not applicable.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                         | Notes / Expected Output                                                                         |
| --- | ------ | -------------------------------------------- | ----------------------------------------------------------------------------------------------- |
| T1  | TODO   | Add `docs/templates/DISCUSSION.md`           | Template with metadata, context, `## Topics` with `Q<n>` and `Positions`, and `## Outcome`.      |
| T2  | TODO   | Register the template                        | Rows in `docs/templates/README.md` and `docs/index.md`, per the `create-markdown-template` skill. |
| T3  | TODO   | Rewrite the discussion lifecycle             | `docs/discussions/AGENTS.md` describes rounds, gates, ownership, and links the template.        |
| T4  | TODO   | Manual verification and completion review    | Scenarios M1-M2 recorded in `manual-verification-evidence.md`; AC review; completion review.    |

## Commit Points

| Task | Coherent change set                                         | Commit policy                                        |
| ---- | ----------------------------------------------------------- | ---------------------------------------------------- |
| T1   | The new template file                                       | Commit after focused validation and required review. |
| T2   | Template catalog rows                                       | Commit after focused validation and required review. |
| T3   | The lifecycle rewrite in `docs/discussions/AGENTS.md`       | Commit after focused validation and required review. |
| T4   | `manual-verification-evidence.md` and spec progress updates | Commit after focused validation and required review. |

Record a justified no-change decision in the task's evidence without creating an empty commit. Use
a Conventional Commit message with the narrow affected scope (`docs(templates)`,
`docs(discussions)`, `docs(issues)`), and sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2003-asynchronous-discussion-rounds/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [x] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all` and the pre-commit gate)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-10-07 15:32 UTC - AI assistant (Copilot SDK in VS Code) - Drafted the specification at the
  maintainer's request, following the round model proposed in
  <https://github.com/torrust/torrust-tracker/pull/2467#issuecomment-6041029813>.
- 2026-10-07 16:15 UTC - AI assistant (Copilot SDK in VS Code) - The maintainer approved the
  specification; created issue #2473, linked it under EPIC #2003, and moved the specification to
  `docs/issues/open/` for a spec-only pull request.
- 2026-10-07 16:40 UTC - AI assistant (Copilot SDK in VS Code) - Reordered the manual scenarios
  so the scratch discussion from M1 is kept for M2 and deleted only after V2 is recorded
  (`review-finding:pr-2474-f1`).
- 2026-10-08 06:44 UTC - AI assistant (Copilot SDK in VS Code) - Spec-only PR #2474 merged. Asked the EPIC
  owner for objections to the round model
  (<https://github.com/torrust/torrust-tracker/issues/2473#issuecomment-6054107392>); the
  implementation PR waits for his review. Started implementation on
  `2473-asynchronous-discussion-rounds`.

## Acceptance Criteria

- [ ] AC1: `docs/discussions/AGENTS.md` defines the opening, contribution, and decision rounds, and
      states that a contribution round's merge gate is document quality, not agreement.
- [ ] AC2: The rules state that participants write only their own entries, that positions are
      appended per topic with participant, date, and pull-request attribution, and that a change of
      position is a new entry.
- [ ] AC3: `## Outcome` is owned by the decision owner, written in a decision round, cites the
      entries it relies on, and links the canonical document that carries the decision.
- [ ] AC4: `docs/templates/DISCUSSION.md` exists with a metadata table, context sections,
      `## Topics` with per-`Q<n>` `Positions`, and `## Outcome`, and states where concrete
      discussions belong.
- [ ] AC5: The template is listed in `docs/templates/README.md` and `docs/index.md`, and
      `docs/discussions/AGENTS.md` links to it instead of repeating its skeleton.
- [ ] AC6: The rules state how AI-assisted entries are attributed.
- [ ] AC7: The three 2026-10-03 discussions are unchanged.
- [ ] `linter all` exits with code `0`
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

### Automatic Checks

- `linter all`
- `./contrib/dev-tools/git/hooks/pre-commit.sh` (includes staged frontmatter validation)

No Rust code changes, so no tests are added or required.

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                               | Human-oriented command/steps                                                                                                                                                                                                                                               | Expected Result                                                                                           | Status | Evidence                                     |
| --- | -------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------- | ------ | -------------------------------------------- |
| M1  | Open a discussion from the template    | Following only `docs/discussions/AGENTS.md`, copy the template into a scratch discussion folder, fill two topics, and run `linter markdown`, `linter cspell`, and `linter lychee`. Keep the scratch folder for M2.                                                         | The instructions suffice without other sources; linters exit 0.                                           | TODO   | `manual-verification-evidence.md` section V1 |
| M2  | Simulate a contribution and a decision | In the scratch discussion, add a second participant's entries under both topics and an Outcome citing one entry per topic; ask a reader who did not write it to answer "who held which position on `Q1`, and in which PR?". After recording V2, delete the scratch folder. | The answer comes from the `Q1` section alone; no entry was edited by another participant; linters exit 0. | TODO   | `manual-verification-evidence.md` section V2 |

Notes:

- Manual verification is mandatory even when automated checks pass.
- Create `manual-verification-evidence.md` from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`
  and record the actual commands, linter versions, output, and outcome.
- If a scenario fails, record the failure and diagnosis in the progress log before proceeding.

### Disposable Verification Scripts

None planned.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | TODO                   |          |
| AC2   | TODO                   |          |
| AC3   | TODO                   |          |
| AC4   | TODO                   |          |
| AC5   | TODO                   |          |
| AC6   | TODO                   |          |
| AC7   | TODO                   |          |

## Risks and Trade-offs

- **The discussion is spread across many pull requests.** Mitigation: the document, not the PRs,
  is the record; each entry links its round.
- **Low-quality or off-topic rounds merge because opinions are not gated.** Mitigation: factual
  claims about the repository are still gated, and the decision owner is not bound by any entry.
- **Concurrent rounds conflict.** Mitigation: append-only entries in separate topic sections keep
  conflicts small and textual.
- **No synthesis until the Outcome.** Readers must read the entries per topic. This is deliberate:
  the decision owner synthesizes once, in the decision round.

## Implementation Completion Review

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from
  `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this issue specification's directory.
- If no retrospective is needed, add a concise progress-log entry explaining why the work had no
  material discovery.
- When an independent reviewer receives this folder-style specification, it records its result in
  `agent-review-reports.md` using `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- Related issues: #2003
- Related PRs: #2428, #2467
- Related ADRs:
  `docs/adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md`
