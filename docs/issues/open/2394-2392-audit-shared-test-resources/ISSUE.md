---
schema-version: 1
doc-type: issue
issue-type: task
status: planned
priority: p2
epic: 2392
github-issue: 2394
spec-path: docs/issues/open/2394-2392-audit-shared-test-resources/ISSUE.md
branch: "2392-test-isolation-spec"
related-pr: 2397
last-updated-utc: "2026-09-30 14:58"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - docs/issues/open/2392-test-isolation/EPIC.md
    - docs/issues/open/2393-2392-test-port-allocation-races/ISSUE.md
    - docs/issues/open/2386-1840-self-hosted-runner-minimum-capacity/ISSUE.md
    - docs/analysis/20260909-cli-config-path-test-isolation/README.md
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
| Network ports | Fixed ports; port `0` selected and released before use; fixed ports inside the ephemeral range |
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
`.github/skills/dev/debugging/fix-bug/SKILL.md`.

## Regression Test Strategy

Not applicable.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | TODO | Define the search method | Repeatable search commands per resource class, recorded in `inventory.md`. |
| T2 | TODO | Inventory network ports | Findings with dispositions; cross-check with #2393 T5 to avoid duplicate work. |
| T3 | TODO | Inventory container resources | Findings with dispositions. |
| T4 | TODO | Inventory filesystem paths | Findings with dispositions. |
| T5 | TODO | Inventory process-global state and external services | Findings with dispositions. |
| T6 | TODO | Draft subissues for new problems | One draft per confirmed problem (or cohesive group), linked from the EPIC after maintainer review. |

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
- [ ] Inventory completed and reviewed
- [ ] Subissues drafted for new problems
- [ ] Acceptance criteria reviewed
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-09-30 14:40 UTC - GitHub Copilot - Drafted as a subissue of the test isolation EPIC to
  cover the resource classes #2386 left unaudited.
- 2026-09-30 14:58 UTC - GitHub Copilot - Maintainer approved the draft. Created #2394 as a
  sub-issue of EPIC #2392 and moved the specification to `docs/issues/open/`.

## Acceptance Criteria

- [ ] AC1: `inventory.md` lists findings for every resource class in scope, with the search
  commands used, so the audit can be repeated.
- [ ] AC2: Every finding has one disposition: isolated, deliberate exception, covered, or new
  problem.
- [ ] AC3: Every new problem has a subissue draft linked from the EPIC.
- [ ] `linter all` exits with code `0`

## Verification Plan

### Automatic Checks

- `linter all`

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| M1 | Repeat the search | Rerun the recorded search commands on the final commit. | Results match the inventory. | TODO | `inventory.md` |
| M2 | Spot-check dispositions | Maintainer reviews a sample of findings from each class. | Dispositions are accepted. | TODO | Review comment |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| --- | --- | --- |
| AC1 | TODO | `inventory.md` |
| AC2 | TODO | `inventory.md` |
| AC3 | TODO | EPIC subissue table |

## Risks and Trade-offs

- **False positives.** A fixed value is not always shared; record why a finding is safe instead of
  opening an issue for it.
- **Stale inventory.** The inventory reflects one commit. The documented isolation rules (EPIC
  Phase 4) prevent new problems; the inventory does not.

## Implementation Completion Review

- Retrospective: `Not yet assessed`
- Add a progress-log entry explaining whether a retrospective is needed.

## References

- Parent EPIC: #2392, `docs/issues/open/2392-test-isolation/EPIC.md`
- Unaudited resources: #2386
- Earlier isolation analysis: `docs/analysis/20260909-cli-config-path-test-isolation/README.md`
