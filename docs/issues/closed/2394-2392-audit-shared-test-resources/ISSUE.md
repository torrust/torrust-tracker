---
schema-version: 1
doc-type: issue
issue-type: task
status: done
priority: p2
epic: 2392
github-issue: 2394
spec-path: docs/issues/closed/2394-2392-audit-shared-test-resources/ISSUE.md
branch: "2392-test-isolation-spec"
related-pr: 2397
last-updated-utc: "2026-11-18 13:39"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .claude/skills/create-issue/SKILL.md
    - docs/issues/open/2392-test-isolation/EPIC.md
    - docs/issues/closed/2393-2392-test-port-allocation-races/ISSUE.md
    - docs/issues/closed/2386-1840-self-hosted-runner-minimum-capacity/ISSUE.md
    - docs/analysis/20260909-cli-config-path-test-isolation/README.md
    - docs/issues/open/2906-2392-fixed-port-metrics-test-concurrent-runs/ISSUE.md
    - docs/issues/closed/2907-2392-remove-leaked-test-temporary-files/ISSUE.md
    - docs/issues/open/2908-2392-compose-cleanup-after-failed-start/ISSUE.md
---

<!-- skill-link: create-issue -->

# Issue #2394 - Audit Shared Test Resources

Parent EPIC: #2392 - Test Isolation (`docs/issues/open/2392-test-isolation/EPIC.md`)

## Goal

Produce an inventory of every resource that tests or test runners share with other tests or other
runs, classify each finding, and turn every confirmed isolation problem into a subissue of the Test
Isolation EPIC. The audit changes no code.

## Background

Known shared resources were found one at a time: a flaky port fixture while repairing a container
build (#2298), fixed E2E host ports and a shared image tag while sizing the self-hosted runner
(#2386), and environment-variable configuration while making main-application tests parallel
(#1419). #2386 states that fixed container names, networks, and `/tmp` paths were not audited.
Without an inventory, the next isolation problem will again be found by a flaky failure.

## Scope

### In Scope

Resource classes to inventory in Rust tests, test fixtures, E2E runners, Compose files, container
test scripts, and CI workflow steps that run tests:

| Class | Examples to look for |
| --- | --- |
| Network ports | Fixed ports; port `0` selected and released before use; fixed ports inside the ephemeral range; a released port rebound to prove release; a released port handed back within one test; a released port probed and expected dead |
| Container resources | Fixed image tags, container names, network names, volume names, Compose project names |
| Filesystem paths | Fixed paths under `storage/`, `/tmp`, or the repository; shared database files |
| Process-global state | `std::env::set_var`, global tracing subscribers, statics, current directory changes |
| External services | Shared database instances or schemas used by more than one test |

For each finding, record: location, resource, who else can use it, and a disposition:

- **Isolated**: already safe (for example port `0` kept bound, or a random Compose project name).
- **Deliberate exception**: the test contract needs the fixed resource; confirm it is documented.
- **Covered**: an existing subissue owns it (for example #2393).
- **New problem**: draft a subissue under the EPIC.

### Out of Scope

- Fixing any finding in this issue.
- Resources shared only across CI jobs on separate virtual machines.
- Test speed and coverage (EPIC #1840, EPIC #1347).

## Architectural Decisions

- Related ADRs: none known.
- ADRs to create: none expected.

## Design and Ownership Review

Not applicable. The audit changes no code.

## Bug-Fix Process

Not applicable. This is an inventory task; confirmed defects become bug subissues that follow
`.claude/skills/fix-bug/SKILL.md`.

## Regression Test Strategy

Not applicable.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | DONE | Define the search method | Repeatable search commands per resource class, recorded in `inventory.md`. |
| T2 | DONE | Inventory network ports | Reuse #2393's per-test inventory for Rust tests: mark **Covered** only the rows #2393 owns (its ignored and rewritten tests). #2393 covers races within one suite run, so still audit Rust tests for collisions between concurrent runs on one host (for example the fixed ports `17091`-`17094` in `tests/metrics/fixed_ports.rs`, and the `reserve_free_ports` users in `tests/configuration/`, which #2393 left unchanged), and audit E2E runners, Compose files, container test scripts, and CI workflow steps. |
| T3 | DONE | Inventory container resources | Findings with dispositions. |
| T4 | DONE | Inventory filesystem paths | Findings with dispositions. |
| T5 | DONE | Inventory process-global state and external services | Findings with dispositions. |
| T6 | DONE | Draft subissues for new problems | One draft per confirmed problem (or cohesive group), linked from the EPIC after maintainer review. Done: D1 (P2), D2 (F2) and D3 (C9) under `docs/issues/drafts/`, merged in PR #2891 and linked from the EPIC's order 2 row. The EPIC coordinator created their GitHub issues (#2906, #2907, #2908) and EPIC rows (orders 6 to 8). |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T1-T5 | `inventory.md` | Documentation-only commit after maintainer review. |
| T6 | New subissue drafts and EPIC table rows | Documentation-only commit after maintainer review. |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/audit-shared-test-resources/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created, linked to the EPIC, and issue number added to this spec
- [x] Inventory completed and reviewed
- [x] Subissues drafted for new problems
- [x] Acceptance criteria reviewed
- [x] Spec moved from `docs/issues/open/` to `docs/issues/closed/`
- [ ] GitHub issue #2394 closed (pending: the close-out PR #2898 has merged; the maintainer closes it)

### Progress Log

- 2026-09-30 14:40 UTC - GitHub Copilot - Drafted as a subissue of the test isolation EPIC to
  cover the resource classes #2386 left unaudited.
- 2026-09-30 14:58 UTC - GitHub Copilot - Maintainer approved the draft. Created #2394 as a
  sub-issue of EPIC #2392 and moved the specification to `docs/issues/open/`.
- 2026-11-14 22:47 UTC - Claude Code (Claude Opus 5.5), EPIC coordinator - After #2393's PR #2859
  merged: T2 reuses #2393's port inventory for Rust tests, and the port class lists the two racing
  shapes #2393 found besides bind-and-release.
- 2026-11-15 12:11 UTC - Claude Code (Claude Opus 5.5), EPIC coordinator - After the first review
  round on PR #2862: T2 marks Covered only the rows #2393 owns and still audits Rust tests for
  collisions between concurrent runs; the port class adds the released-port-probed-dead shape.
- 2026-11-16 17:30 UTC - Claude Code (Claude Opus 5.5), #2394 session - T1 to T5 done at
  `b009b6425`: `inventory.md` records 37 findings (29 isolated, 1 deliberate exception, 5 covered,
  2 new problems). The maintainer decided the three judgement calls: P2 (fixed-port metrics test)
  and F2 (leaked temporary files) are new problems, drafted for T6; C3 (local default image tags)
  extends #2396's scope.
- 2026-11-16 22:06 UTC - Claude Code (Claude Opus 5.5), #2394 session - First review round on PR
  #2891 (da2ce7 approved with F1-F6; Copilot four findings): corrected where the fixed-port test
  runs in CI, the P8 enumeration and the drafts' skill paths and template sections; added
  reproduction evidence to D1 and D2; the maintainer classified Copilot's failed-`up` Compose
  cleanup as a new problem (C9, draft D3). The inventory now has 38 findings (29 isolated,
  1 deliberate exception, 5 covered, 3 new problems).
- 2026-11-17 08:52 UTC - Claude Code (Claude Opus 5.5), #2394 session - PR #2891 merged
  (approved by da2ce7 after three review rounds). T6 done: the EPIC's order 2 row links drafts
  D1, D2 and D3; the EPIC coordinator creates their GitHub issues and EPIC rows and extends
  #2396's scope (C3). Spec moved to `docs/issues/closed/`; #2394 is still open, and the
  maintainer closes it when this close-out merges. No retrospective file: the audit invalidated no
  assumption of this spec, and the one reusable lesson fits here. The T1 searches looked for
  shared names and values, not for resources released only on the success path, so Copilot's
  review found C9 (a failed `docker compose up` leaves the stack running) that the inventory's
  first version had marked as torn down on drop. A future audit also checks each resource's
  failure and drop paths (EPIC rule 1). The other review findings were corrections to the audit
  documents, recorded in `docs/pr-reviews/pr-2891-review/PR-REVIEW.md`.
- 2026-11-17 12:35 UTC - Claude Code (Claude Opus 5.5), EPIC #2392 coordinator - Created the
  GitHub issues for D1 (#2906), D2 (#2907) and D3 (#2908), moved their specs to
  `docs/issues/open/`, and added them to the EPIC as orders 6 to 8.
- 2026-11-17 15:22 UTC - Claude Code (Claude Opus 5.5), EPIC #2392 coordinator - The issue-closed checkpoint
  and inventory row C3 now reflect the state after PR #2898 merged and #2396's scope was extended.

## Acceptance Criteria

- [x] AC1: `inventory.md` lists findings for every resource class in scope, with the search
  commands used, so the audit can be repeated.
- [x] AC2: Every finding has one disposition: isolated, deliberate exception, covered, or new
  problem.
- [x] AC3: Every new problem has a subissue draft linked from the EPIC.
- [x] `linter all` exits with code `0`

## Verification Plan

### Automatic Checks

- `linter all`

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| M1 | Repeat the search | Rerun the recorded search commands on the final commit. | Results match the inventory. | DONE | `inventory.md` |
| M2 | Spot-check dispositions | Maintainer reviews a sample of findings from each class. | Dispositions are accepted. | DONE | Maintainer decisions Q1-Q4 in `inventory.md`; PR #2891 approved and merged |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| --- | --- | --- |
| AC1 | DONE | `inventory.md` (search commands per class, rerun on the merge of `develop` `0373f05f8`) |
| AC2 | DONE | `inventory.md`: 38 findings, 29 isolated, 1 deliberate exception, 5 covered, 3 new problems |
| AC3 | DONE | EPIC order 2 row links drafts D1, D2 and D3; their GitHub issues #2906, #2907 and #2908 are EPIC orders 6 to 8 |

## Risks and Trade-offs

- **False positives.** A fixed value is not always shared; record why a finding is safe instead of
  opening an issue for it.
- **Stale inventory.** The inventory reflects one commit. The documented isolation rules (EPIC
  Phase 4) prevent new problems; the inventory does not.

## Implementation Completion Review

- Retrospective: `Not needed` (see the progress log)
- Add a progress-log entry explaining whether a retrospective is needed.

## References

- Parent EPIC: #2392, `docs/issues/open/2392-test-isolation/EPIC.md`
- Unaudited resources: #2386
- Earlier isolation analysis: `docs/analysis/20260909-cli-config-path-test-isolation/README.md`
