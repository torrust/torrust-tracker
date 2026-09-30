---
schema-version: 1
doc-type: issue
issue-type: task
status: draft
priority: p2
epic: 2003
github-issue: null
spec-path: docs/issues/drafts/2003-mine-ai-agent-memories/ISSUE.md
branch: "chore/mine-ai-agent-memories-spec"
related-pr: null
last-updated-utc: "2026-09-30 11:25"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - AGENTS.md
    - docs/adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md
    - docs/issues/drafts/2003-mine-ai-agent-memories/github-copilot-memory-snapshot.md
    - docs/issues/drafts/2003-mine-ai-agent-memories/vscode-copilot-local-memory-snapshot.md
    - docs/issues/drafts/mine-pr-review-audit-records/ISSUE.md
    - docs/issues/open/2003-overhaul-guardrails-and-automation/EPIC.md
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Mine AI Agent Memories to Improve Repository Guidance

**Parent EPIC:** #2003 - Overhaul: Automation Tools and AI Agent Guardrails (EPIC owner:
@da2ce7). The maintainer placed this issue under #2003 on 2026-09-30.

## Goal

Review everything AI agents have stored in their memories about this repository, and use it to
improve the tracked guidance they should have found instead: `AGENTS.md` files, skills,
templates, documentation, and deterministic checks. Then clean up the memories so that stale
or contradictory records no longer mislead agents.

## Background

AI agents working on this repository keep memories outside Git. Each memory is a short fact or
rule the agent decided to remember. The maintainer wants to treat these memories as a dataset:
why an agent needed to remember something is evidence of where tracked guidance is missing,
hard to find, or wrong.

The repository policy already says memories must not be the source of truth. `AGENTS.md`
engineering policy 7 and the
[AI agent context portability ADR](../../../adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md)
require repository knowledge to live in Git-tracked documentation, scripts, and tests. They
treat provider-specific retained state, which includes memories, as an optional adapter. This
issue checks how well the repository meets that policy today.

This issue covers two memory stores, captured as issue-local snapshots:

| Store                                  | Written by                                        | Scope                                   | Snapshot                                                                           | Records |
| -------------------------------------- | ------------------------------------------------- | --------------------------------------- | ---------------------------------------------------------------------------------- | ------- |
| GitHub Copilot Memory (repository)     | Copilot code review (`copilot-code-review`)       | The `torrust/torrust-tracker` repository | [github-copilot-memory-snapshot.md](github-copilot-memory-snapshot.md)             | 110     |
| VS Code GitHub Copilot Chat local memory | The Copilot Chat agent on the maintainer's workstation | User scope and one workspace clone      | [vscode-copilot-local-memory-snapshot.md](vscode-copilot-local-memory-snapshot.md) | 50      |

### Preliminary Observations

A first read during drafting suggests the following. These are hypotheses for T2 to verify,
not conclusions.

- **All 110 GitHub memories come from Copilot code review.** 90 were written by
  `gpt-5.6-luna`, 17 by `gpt-5.2`, and 3 by `gpt-5.6-sol`. The reviewer learns repository
  conventions from the PRs it reviews, so its memories describe what reviewers keep having to
  check.
- **Many GitHub memories are near-duplicates.** Examples of clusters: Containerfile sync for new
  workspace members (`G047`, `G062`, `G063`, `G069`, `G070`); the `stdout-result-data` output
  contract (`G023`, `G044`, `G046`); append-only review reports (`G004`, `G007`, `G090`);
  frontmatter first-line detection (`G029`, `G030`, `G039`); and `last-updated-utc` format
  (`G001`, `G031`, `G035`, `G036`, `G042`).
- **Some GitHub memories contradict each other.** Examples: whether `open` is a valid issue
  status (`G008`, `G073`, `G076` against `G027`, `G105`); where PR review audits live (`G038`
  against `G077` against `G103`); where the review-report contract check lives (`G015` against
  `G019`); which key links manual evidence to its issue (`G003` against `G075`); and whether
  `last-updated-utc` is quoted (`G001`, `G031` against `G035`, `G036`). The page ranks some
  stale-looking records highly; for example, `G008` is ranked 8th.
- **Frontmatter and issue-spec metadata dominate the GitHub memories.** That suggests the
  frontmatter contract is hard to discover, or that the validator does not run early enough to
  catch mistakes before review.
- **Several local memories are lessons from specific mistakes** (`U05`, `U08`, `U09`, `U12`,
  `U13`, `U22`, `U24`). Each could become a skill guardrail or a deterministic check.
- **Some local memories are already redundant or stale.** `U06` says its rules are now in
  `AGENTS.md`. `R05` repeats `U14` and `AGENTS.md` essential rule 2. `R04` mentions
  squash-merged branches, although the maintainer merge tool creates merge commits. `R07` is a
  time-bound note from 2026-09-08.

## Scope

### In Scope

- The two snapshots above, committed unchanged with their provenance.
- A classification ledger covering every record in both snapshots, with each record checked
  against the current `develop` branch.
- An analysis report that groups the findings and proposes concrete changes to tracked
  guidance and checks, each linked to the memory IDs that motivate it.
- The maintainer-approved guidance changes that are small enough for this issue. Larger ones
  become follow-up issues.
- Memory hygiene after the approved guidance changes merge: delete or correct the stale,
  contradictory, duplicate, and newly redundant records in both stores, and record what changed.

### Out of Scope

- Memories of other contributors, other local clones, and other agent products (for example
  Claude Code, Codex, or Cursor). A follow-up can repeat this process for them.
- Automating the export of GitHub Copilot Memory. No documented API is known; the snapshot is a
  manual copy of the settings page.
- Memories that GitHub hides because Copilot "learned not to use" them. The settings page does
  not list them.
- Changing how GitHub Copilot Memory or the VS Code memory tool work.
- Implementing new deterministic checks. The report proposes them; each needs its own issue.

## Architectural Decisions

- Related ADRs:
  [`docs/adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md`](../../../adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md)
  (memories are provider-specific adapters, not sources of truth).
- ADRs to create: none known. T5 decides whether a periodic memory review becomes a documented
  process. If it does, the process belongs in a skill; it needs an ADR only if it changes the
  portability policy.

During implementation, stop and create an ADR when a decision affects project architecture or
design patterns, selects an approach among meaningful alternatives, or has consequences future
contributors need to understand.

### Coordination

- **Draft `mine-pr-review-audit-records`** mines PR review audit records. Copilot code review
  writes the GitHub memories while reviewing the same PRs, so the two analyses may point to the
  same guidance gaps. Cross-reference shared findings rather than proposing the same change
  twice.
- **EPIC #2003** owns agent guardrails and automation, and **#2264** owns frontmatter and
  semantic-link conventions. Proposals that touch their areas are raised with their owners, not
  implemented unilaterally here.

## Design and Ownership Review

Not applicable. This is documentation and analysis work. It adds no child processes,
asynchronous I/O, network readiness, resource cleanup, or reusable test fixtures.

## Bug-Fix Process

Not applicable. This is analysis work, not a defect fix. Stale guidance found during the
analysis is reported as a proposal; if a proposal is a bug, its follow-up uses the fix-bug
skill.

## Regression Test Strategy

Not applicable. This is not bug work.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                              | Notes / Expected Output |
| --- | ------ | --------------------------------- | ----------------------- |
| T1  | DONE   | Capture the memory snapshots      | Both snapshots exist in this folder with provenance, stable record IDs, and unchanged record text. Captured during drafting on 2026-09-30. |
| T2  | TODO   | Classify every record             | Issue-local `memory-classification.md` lists every snapshot record once. For each record it gives the classification (see below), a duplicate cluster ID when one applies, whether the record is true on current `develop` with evidence, the canonical tracked source when one exists, and the proposed target artifact. |
| T3  | TODO   | Write the analysis report         | Issue-local `memory-mining-report.md` summarizes counts per classification and store, groups the records into themes, and lists ranked proposals. Each proposal names the target file or check, the change, and the motivating record IDs. |
| T4  | TODO   | Apply approved guidance changes   | The maintainer approves, rejects, or defers each proposal, and the report records the decision. Approved small documentation, skill, template, or `AGENTS.md` changes are made here; larger ones and new checks become follow-up issues linked from the report. |
| T5  | TODO   | Clean up memories and decide on recurrence | After T4's changes merge, the maintainer deletes or corrects the targeted GitHub memories in the settings page, and the agent updates local memories. The report records every removed or changed record. The maintainer decides whether to repeat this review periodically; if so, the process is documented in a skill. |

### Classification Values for T2

Proposed values for T2 to settle. A record gets one primary value.

| Value          | Meaning |
| -------------- | ------- |
| `DOCUMENTED`   | True, and already stated in a tracked canonical source. The memory is redundant; T2 checks whether that source is easy to find. |
| `UNDOCUMENTED` | True and useful, but not stated in any tracked source. Candidate for a guidance change. |
| `STALE`        | No longer true on current `develop`. |
| `CONTRADICTED` | Conflicts with another record. T2 resolves it against the repository and marks the true side with another value. |
| `DUPLICATE`    | Restates another record in the same cluster. The ledger names the cluster's representative record. |
| `PERSONAL`     | A contributor preference or local-environment fact, such as remote names. It belongs in personal memory or contributor docs, not in repository-wide guidance. |
| `INCIDENT`     | A lesson from a specific mistake. Candidate for a skill guardrail or a deterministic check. |
| `CODE_FACT`    | Describes code that is easy to read from the source, such as a type alias. Usually no action, or a Rust doc comment. |

## Commit Points

| Task | Coherent change set                                                          | Commit policy                                        |
| ---- | ---------------------------------------------------------------------------- | ---------------------------------------------------- |
| T1   | Issue specification and both snapshots                                       | Commit with the spec after maintainer review.        |
| T2   | `memory-classification.md`                                                   | Commit after focused validation and required review. |
| T3   | `memory-mining-report.md`                                                    | Commit after focused validation and required review. |
| T4   | One commit per approved guidance change, scoped to the file or skill it changes | Commit after focused validation and required review. |
| T5   | Report update recording the memory cleanup and the recurrence decision, plus the skill if one is approved | Commit after focused validation and required review. |

Record a justified no-change decision in the task's evidence without creating an empty commit.
Use a Conventional Commit message with the narrow affected scope, and sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2003-mine-ai-agent-memories/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] Maintainer approved publishing both snapshots in the public repository
- [x] Parent EPIC decided: #2003
- [ ] Draft merged into `develop` through a spec-only PR
- [ ] EPIC #2003 owner (@da2ce7) told about the draft so it can be included in the EPIC plan
- [ ] GitHub issue created, linked as a sub-issue of #2003, and issue number added to this spec
- [ ] EPIC #2003 subissue table updated with this issue
- [ ] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-09-30 10:38 UTC - GitHub Copilot - Drafted from the maintainer's request to mine agent
  memories. Captured the GitHub Copilot Memory page (110 records) and the local VS Code memory
  (50 records) as issue-local snapshots. No parent EPIC assigned; #2003 suggested.
- 2026-09-30 10:55 UTC - GitHub Copilot - Maintainer placed the issue under EPIC #2003; renamed
  the draft folder to `2003-mine-ai-agent-memories` and set `epic: 2003`. Redacted `U11` and
  `U19` after a privacy review.
- 2026-09-30 11:00 UTC - GitHub Copilot - Maintainer approved the spec and publishing both
  snapshots. Sequence agreed: merge this draft through a spec-only PR, then tell the EPIC #2003
  owner so the draft can be included in the EPIC plan. The GitHub issue is created afterwards.

## Acceptance Criteria

- [ ] AC1: Both snapshots are committed with provenance, and their record text matches the
  captured sources except for the text redacted in each snapshot's privacy review.
- [ ] AC2: `memory-classification.md` lists each of the 160 snapshot records exactly once, each
  with a classification and evidence checked against current `develop`.
- [ ] AC3: Every contradiction found in T2 is resolved: the ledger names the true side and cites
  the tracked source, code, or check that proves it.
- [ ] AC4: `memory-mining-report.md` lists ranked proposals, each linked to its motivating record
  IDs and target artifact, with the maintainer's decision recorded for each.
- [ ] AC5: Every approved proposal is either merged or tracked by a linked follow-up issue.
- [ ] AC6: The memory cleanup is done after the guidance changes merge, and the report records
  each deleted or changed record, or the maintainer's reason for deferring the cleanup.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

### Automatic Checks

- `linter all`
- `frontmatter-validator` on this issue folder
- Tests for any check or tool changed by an approved proposal

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                              | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | ------------------------------------- | ---------------------------- | --------------- | ------ | -------- |
| M1  | Spot-check the classification         | The maintainer picks ten ledger rows, including at least two contradictions, and checks each against the repository | Every checked row's classification and evidence hold; any error is fixed and the whole ledger is rechecked for the same mistake | TODO | `manual-verification-evidence.md` section V1 |
| M2  | Guidance is findable without memory   | For five facts that were memory-only before T4, ask a fresh agent session the question the memory answered, with memory unavailable or ignored | The agent finds the answer in tracked guidance and cites the file | TODO | `manual-verification-evidence.md` section V2 |
| M3  | Memory cleanup took effect            | After T5, the maintainer reopens the GitHub memory settings page and the agent views the local memory files | Records marked for deletion are gone, corrected records show the new text, and the counts match the report | TODO | `manual-verification-evidence.md` section V3 |

Notes:

- Manual verification is mandatory even when automated tests pass.
- Record the toolchain for every command result where it can affect behavior.
- Create `manual-verification-evidence.md` from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`.

### Disposable Verification Scripts

None planned.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence           |
| ----- | ---------------------- | ------------------ |
| AC1   | TODO                   | {test/log/PR link} |
| AC2   | TODO                   | {test/log/PR link} |
| AC3   | TODO                   | {test/log/PR link} |
| AC4   | TODO                   | {test/log/PR link} |
| AC5   | TODO                   | {test/log/PR link} |
| AC6   | TODO                   | {test/log/PR link} |

## Risks and Trade-offs

- **Publishing memories.** The GitHub memories are visible only to repository administrators,
  and the local memories are private to one workstation. Committing them makes them public.
  A privacy review on 2026-09-30 found no secrets. It redacted one operational note about
  remote server administration (`U19`) and one contributor's first name (`U11`); each snapshot
  records its review. The maintainer approved publishing both snapshots on 2026-09-30, after
  that review.
- **Snapshots age quickly.** Copilot keeps adding and re-ranking memories. The snapshots are
  frozen on 2026-09-30, and T2 classifies them against `develop` at the time of analysis. T5
  compares the live page with the snapshot before deleting anything, so it does not remove a
  record that changed after capture.
- **Cleaning up too early removes guidance agents still rely on.** T5 runs only after T4's
  changes merge, so each deleted memory has a tracked replacement.
- **One contributor's local memory is a small sample.** It shows one workflow. Other
  contributors' memories are out of scope; if T3 finds the local store valuable, a follow-up can
  collect more samples.

## Implementation Completion Review

After implementation, compare the result with this specification. Record
invalidated assumptions, material design changes, unexpected validation
findings, and reusable lessons.

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from the repository
  template at `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this issue
  specification's directory.
- If no retrospective is needed, add a concise progress-log entry explaining
  why the work had no material discovery.
- When an independent reviewer receives this folder-style specification, it
  records its result in `agent-review-reports.md` using
  `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- Related issues: #2003 (parent EPIC), #2264 (frontmatter conventions)
- Related drafts: [`mine-pr-review-audit-records`](../mine-pr-review-audit-records/ISSUE.md)
- Related ADRs:
  [`docs/adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md`](../../../adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md)
