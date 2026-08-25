---
doc-type: issue
issue-type: task
status: in-progress
priority: p2
epic: null
github-issue: 2095
spec-path: docs/issues/open/2095-organize-runtime-architecture-documentation.md
branch: "2095-organize-runtime-architecture-documentation"
related-pr: null
last-updated-utc: 2026-08-25 12:16
semantic-links:
  skill-links:
    - create-issue
    - write-markdown-docs
  related-artifacts:
    - docs/AGENTS.md
    - docs/index.md
    - docs/packages.md
    - docs/application-jobs.md
    - docs/architecture/README.md
    - docs/architecture/events.md
    - docs/architecture/tracker-instance-architecture.md
    - docs/adrs/20260727180000_shared_services_across_tracker_instances.md
    - docs/skills/semantic-skill-link-convention.md
    - .github/skills/dev/planning/create-issue/SKILL.md
    - .github/skills/dev/planning/write-markdown-docs/SKILL.md
---

# Issue #2095 - Organize Runtime Architecture Documentation

## Goal

Create a discoverable `docs/architecture/` documentation area that explains
the tracker runtime architecture. Move the event-topology guide into that area
and add a canonical guide describing the one-process, multiple-listener model,
including the boundary between shared services and listener-specific
configuration.

## Background

Torrust Tracker can expose multiple HTTP and UDP listener instances from one
process. This is intentionally not a supervisor for independent trackers:
listener instances share one logical tracker core, swarm data, policy
configuration, and selected protocol services. The design is recorded partly in
ADR-20260727180000 and the event-topology guide, but no central document
explains the full runtime composition and its configuration and deployment
consequences.

Recent configuration work correctly moved independently applicable settings,
such as network topology and metrics policy, to listener instances. This does
not make tracker instances independent. Values governing the shared tracker
core, including private mode, whitelist/listing authorization, announce policy,
and tracker policy, remain process-wide. A private HTTP listener and public UDP
listener cannot operate as independent trackers in one process. Operators need
separate processes for isolated swarm, authentication, whitelist, or policy
state.

The event-topology guide is an evolving architecture guide, not an ADR. Placing
it under `docs/architecture/` creates a coherent home for it and future runtime
explanations without mixing them with immutable decisions.

## Scope

### In Scope

- Create `docs/architecture/README.md` as the architecture-guide index.
- Place the event-topology guide at `docs/architecture/events.md`.
- Add `docs/architecture/tracker-instance-architecture.md` as the canonical
  runtime-composition guide.
- Describe shared services, listener-owned services and configuration,
  configuration-placement rules, and the boundary between multiple listeners
  and multiple tracker processes.
- Correct ADR-20260727180000's adapter ownership details and link it to the
  canonical guide.
- Update durable cross-references, documentation indexes, and semantic-link
  frontmatter affected by the move.

### Out of Scope

- Runtime, configuration-schema, dependency, or service-ownership changes.
- Migrating the active application runtime from configuration v2 to v3.
- Moving `docs/packages.md` or `docs/application-jobs.md`.
- Creating an ADR; this task documents accepted decisions rather than changing
  them.

## Architectural Decisions

- Related ADRs:
  - `docs/adrs/20260727180000_shared_services_across_tracker_instances.md`
  - `docs/adrs/20260727000000_events_are_objective_facts.md`
  - `docs/adrs/20260721000000_make_network_configuration_per_tracker_instance.md`
- ADRs to create: None known.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                    | Notes / Expected Output                                                                                            |
| --- | ------ | --------------------------------------- | ------------------------------------------------------------------------------------------------------------------ |
| T1  | DONE   | Create architecture documentation index | Added `docs/architecture/README.md` with scoped guide and related-document navigation.                             |
| T2  | DONE   | Move the event architecture guide       | Relocated it to `docs/architecture/events.md` and updated durable repository links.                                |
| T3  | DONE   | Document tracker-instance architecture  | Added canonical guide for shared state, listener responsibilities, configuration placement, and process isolation. |
| T4  | DONE   | Correct shared-services ADR             | Corrected adapter ownership, binding clarification, and guide links.                                               |
| T5  | DONE   | Update references and indexes           | Updated `docs/index.md`, `docs/AGENTS.md`, active/draft references, and semantic-link metadata.                    |
| T6  | DONE   | Validate documentation                  | `git diff --check`, Markdown, spelling, and full `linter all` checks passed; manual scenarios recorded below.      |
| T7  | DONE   | Re-review acceptance criteria           | Re-reviewed completed artifacts against every acceptance criterion.                                                |

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted in `docs/issues/drafts/`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [ ] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [x] Manual verification scenarios executed and recorded (status + evidence)
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [x] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-08-25 11:56 UTC - GitHub Copilot - Drafted specification from the architecture documentation review.
- 2026-08-25 12:00 UTC - GitHub Copilot - User approved the specification; created GitHub issue #2095 and implementation branch.
- 2026-08-25 12:16 UTC - GitHub Copilot - Created the architecture documentation area, relocated the event guide, added the tracker-instance guide, updated references and semantic links, and passed `linter all`.

## Acceptance Criteria

- [x] AC1: `docs/architecture/README.md` exists and indexes runtime architecture guides, relevant ADRs, package architecture, and job ownership documentation without duplicating them.
- [x] AC2: The event-topology guide is at `docs/architecture/events.md`, and durable repository references to the old path are updated.
- [x] AC3: A canonical tracker-instance guide explains that HTTP/UDP listeners in one process serve one logical tracker and identifies shared state/services and listener-owned concerns.
- [x] AC4: The new guide defines a configuration-placement rule and explains that isolated policies or swarm/authentication data require separate tracker processes.
- [x] AC5: ADR-20260727180000 accurately distinguishes shared state/services from per-listener HTTP and UDP protocol adapters and links to the guide.
- [x] AC6: Every new or modified Markdown artifact contains accurate YAML-frontmatter semantic links.
- [x] AC7: `linter all` exits with code `0`.
- [x] AC8: Manual verification scenarios are executed and documented with status and evidence.
- [x] AC9: Acceptance criteria are re-reviewed after implementation and reflect actual behavior.

## Verification Plan

### Automatic Checks

- `linter markdown`
- `linter cspell`
- `linter all`
- Search for stale references to the previous event-guide location and verify
  that no durable links remain.

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                                | Command/Steps                                                                                                    | Expected Result                                                                                                                               | Status | Evidence                                                                                                                      |
| --- | --------------------------------------- | ---------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------- | ------ | ----------------------------------------------------------------------------------------------------------------------------- |
| M1  | Navigate architecture documentation     | Read `docs/index.md`, then `docs/architecture/README.md`, then each listed guide.                                | A contributor can locate runtime composition, event topology, package boundaries, job ownership, and ADR records without guesswork.           | DONE   | Reviewed the documentation index and architecture index links after implementation.                                           |
| M2  | Verify instance-boundary explanation    | Compare the guide with `src/container.rs` and tracker, HTTP, UDP-core, and UDP-server container implementations. | The guide correctly separates shared services from listener-owned adapters/configuration and identifies the multi-process isolation boundary. | DONE   | Compared guide content with the documented container construction paths during the architecture review.                       |
| M3  | Verify path and semantic-link migration | Search for the old event-guide path and inspect frontmatter in every touched Markdown document.                  | No stale durable links remain; every semantic link targets a stable existing artifact or accepted issue/ADR reference.                        | DONE   | Repository search returned no `events-architecture.md` references; reviewed frontmatter for every modified Markdown artifact. |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                                                                               |
| ----- | ---------------------- | -------------------------------------------------------------------------------------- |
| AC1   | DONE                   | `docs/architecture/README.md`                                                          |
| AC2   | DONE                   | `docs/architecture/events.md`; stale-path repository search returned no results        |
| AC3   | DONE                   | `docs/architecture/tracker-instance-architecture.md`                                   |
| AC4   | DONE                   | Configuration Placement Rule and Multiple Listeners Versus Multiple Processes sections |
| AC5   | DONE                   | Updated ADR-20260727180000                                                             |
| AC6   | DONE                   | Frontmatter inspected in all added and modified Markdown documents                     |
| AC7   | DONE                   | `linter all` completed successfully at 2026-08-25 12:16 UTC                            |
| AC8   | DONE                   | M1 through M3 recorded above                                                           |
| AC9   | DONE                   | This completed acceptance-verification table                                           |

## Risks and Trade-offs

- Moving a canonical document can leave stale long-lived issue-specification
  links. Mitigation: search the complete repository and update durable links.
- A new guide could duplicate ADR, package, or job guidance. Mitigation: give
  every document a narrow responsibility and link rather than duplicate.
- Configuration v3 is not active. Mitigation: distinguish current shared
  topology from the intended v3 configuration boundary.

## References

- Shared-services decision: `docs/adrs/20260727180000_shared_services_across_tracker_instances.md`
- Events decision: `docs/adrs/20260727000000_events_are_objective_facts.md`
- Per-instance network decision: `docs/adrs/20260721000000_make_network_configuration_per_tracker_instance.md`
- Semantic-link convention: `docs/skills/semantic-skill-link-convention.md`
