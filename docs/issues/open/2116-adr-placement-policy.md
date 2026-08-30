---
doc-type: issue
issue-type: task
status: open
priority: p2
epic: null
github-issue: 2116
spec-path: docs/issues/open/2116-adr-placement-policy.md
branch: "2116-adr-placement-policy"
related-pr: null
last-updated-utc: 2026-08-30 10:55
semantic-links:
  skill-links:
    - create-issue
    - create-adr
    - write-markdown-docs
  related-artifacts:
    - docs/AGENTS.md
    - docs/adrs/README.md
    - docs/adrs/index.md
    - docs/templates/ADR.md
    - .github/skills/dev/planning/create-adr/SKILL.md
    - .github/skills/dev/planning/create-issue/SKILL.md
    - console/tracker-client/docs/adrs/README.md
    - console/tracker-client/docs/adrs/20260512080000_define_tracker_cli_io_contract_and_error_handling.md
    - docs/adrs/20260519000000_define_global_cli_output_contract.md
---

<!-- skill-link: create-issue -->
<!-- skill-link: create-adr -->
<!-- skill-link: write-markdown-docs -->

# Issue #2116 - Define ADR Placement by Decision Scope

## Goal

Define where Architectural Decision Records (ADRs) belong based on the scope of the decision.
Package-owned decisions must remain with extractable packages, while repository-wide and
cross-package decisions remain in the root ADR collection.

## Background

The current guidance requires all ADRs to be created in `docs/adrs/`. That rule conflicts with
the repository's package-extraction direction: a decision that is solely owned by one package
loses its rationale when the package is extracted unless its ADR travels with it.

The tracker client is the existing precedent. Its local ADR collection under
`console/tracker-client/docs/adrs/` contains the original CLI I/O contract. The later root ADR
`20260519000000_define_global_cli_output_contract.md` explicitly records that the local decision
was intentionally separate because extraction was anticipated, then supersedes it with a
repository-wide contract.

This policy must distinguish decision scope from implementation-file location. A change that
touches one package can still govern shared configuration, a protocol, dependency policy, or
another inter-package contract and therefore belongs in the root collection.

## Scope

### In Scope

- Create a root ADR defining placement rules for root and package-local ADRs.
- Store package-owned ADRs in `packages/<package>/docs/adrs/` when their decisions are limited to
  that package and should travel with it after extraction.
- Keep repository-wide, multi-package, and inter-package-contract ADRs in `docs/adrs/`.
- Define local ADR collection structure: `README.md` for purpose and guidance, plus `index.md`
  for the local collection.
- Keep root and package ADR indexes separate; do not duplicate local ADR entries in
  `docs/adrs/index.md`.
- Define supersession: when a local decision becomes repository-wide, create a root ADR that
  links to and supersedes the local ADR while preserving the local ADR as historical context.
- Update ADR authoring guidance, templates, issue-authoring guidance, and documentation navigation
  to apply the policy consistently.
- Cite the tracker-client local ADR and the global CLI output ADR as the real placement and
  supersession example.

### Out of Scope

- Moving `20260829204258_use_exact_ip_counters_for_udp_banning.md` from `docs/adrs/` to
  `packages/udp-core/docs/adrs/`.
- Creating a package-local ADR collection for `udp-core`.
- Changing production code, benchmark behavior, or the UDP Bloom-filter removal work.
- Retroactively moving every existing ADR without a separately reviewed migration decision.

## Architectural Decisions

- Related ADRs:
  - `docs/adrs/20260519000000_define_global_cli_output_contract.md`
  - `console/tracker-client/docs/adrs/20260512080000_define_tracker_cli_io_contract_and_error_handling.md`
- ADRs to create: Define ADR placement by decision scope.

The policy ADR must state that architectural scope, rather than the paths of modified files,
determines placement. It must explicitly identify shared configuration, protocols, dependency
policy, and inter-package contracts as root-ADR criteria.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                              | Notes / Expected Output                                                                                                        |
| --- | ------ | --------------------------------- | ------------------------------------------------------------------------------------------------------------------------------ |
| T1  | TODO   | Create the root ADR               | Records placement criteria, rationale, alternatives, local-index boundary, extraction consequence, and supersession procedure. |
| T2  | TODO   | Update ADR guidance and template  | Align `docs/AGENTS.md`, `docs/adrs/README.md`, `docs/adrs/index.md`, `docs/templates/ADR.md`, and the `create-adr` skill.      |
| T3  | TODO   | Update issue-authoring guidance   | Ensure planned ADR references in issue specifications identify root or package-local placement by decision scope.              |
| T4  | TODO   | Update navigation and skill links | Make package-local discovery explicit and synchronize artifacts carrying `skill-link:` markers with their linked skills.       |
| T5  | TODO   | Validate documentation            | Run required linters and manually verify the tracker-client placement and supersession example remains accurate.               |

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted in `docs/issues/drafts/`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [ ] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [x] Focused specification validation completed (`linter markdown`, `linter cspell`, and `git diff --check`)
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded (status + evidence)
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-08-30 10:55 UTC - GitHub Copilot - Drafted from the ADR placement policy hand-off; awaiting maintainer approval before GitHub issue creation.
- 2026-08-30 10:56 UTC - GitHub Copilot - Maintainer approved the draft; created GitHub issue #2116 and moved this specification to `docs/issues/open/`.
- 2026-08-30 11:20 UTC - GitHub Copilot - Recovered after an interrupted session; verified GitHub issue #2116 and ran focused Markdown, spelling, and whitespace validation successfully.

## Acceptance Criteria

- [ ] AC1: A root ADR defines root versus package-local ADR placement according to decision scope.
- [ ] AC2: The policy explicitly treats shared configuration, protocols, dependency policy, and
      inter-package contracts as root-ADR criteria even when implementation changes are local.
- [ ] AC3: Package-local ADR collections require `README.md` and `index.md`, and local ADRs are
      not duplicated in the root ADR index.
- [ ] AC4: The policy defines how a root ADR supersedes a package-local ADR while retaining the
      local ADR as historical context.
- [ ] AC5: `docs/AGENTS.md`, the root ADR README and index, ADR template, ADR skill, and relevant
      issue-authoring guidance consistently describe the placement policy.
- [ ] AC6: The tracker-client ADR and the global CLI output ADR are cited as the existing local
      placement and root-supersession example.
- [ ] AC7: The UDP ADR migration is excluded from this policy change.
- [ ] `linter all` exits with code `0`.
- [ ] Relevant documentation checks pass.
- [ ] Manual verification scenarios are executed and documented (status + evidence).
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior.
- [ ] Documentation is updated when behavior or workflow changes.

## Verification Plan

Define verification before implementation starts and execute it before closing the issue.

### Automatic Checks

- `linter markdown`
- `linter cspell`
- `linter all`

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario               | Command/Steps                                                                         | Expected Result                                                   | Status | Evidence                                        |
| --- | ---------------------- | ------------------------------------------------------------------------------------- | ----------------------------------------------------------------- | ------ | ----------------------------------------------- |
| M1  | Verify scope criteria  | Review the root ADR and updated guidance for package-only and cross-package examples. | Package ownership and root criteria are unambiguous.              | TODO   | Review notes in this spec.                      |
| M2  | Verify local precedent | Read the tracker-client local ADR and the global CLI output ADR.                      | The local ADR is preserved and the root ADR records supersession. | TODO   | Links recorded in the ADR and updated guidance. |
| M3  | Verify index boundary  | Review root and a package-local ADR index after implementation.                       | Each ADR appears only in its owning collection's index.           | TODO   | Index links.                                    |

Notes:

- Manual verification is mandatory even when automated checks pass.
- If a scenario fails, record the failure and diagnosis in the progress log before proceeding.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                                                  |
| ----- | ---------------------- | --------------------------------------------------------- |
| AC1   | TODO                   | Root placement-policy ADR.                                |
| AC2   | TODO                   | Root placement-policy ADR and updated authoring guidance. |
| AC3   | TODO                   | Updated authoring guidance and local ADR structure.       |
| AC4   | TODO                   | Root placement-policy ADR.                                |
| AC5   | TODO                   | Updated canonical documentation and skills.               |
| AC6   | TODO                   | References to the tracker-client and global CLI ADRs.     |
| AC7   | TODO                   | Policy-only diff review.                                  |

## Risks and Trade-offs

- Local ADR collections are less visible from the root documentation, so each collection needs a
  purpose README and index, and package documentation must link to them.
- The placement assessment requires architectural judgment. Explicit root criteria reduce, but do
  not eliminate, the need for reviewer evaluation.
- Moving the current UDP ADR in this policy change would mix governance with implementation work;
  defer it to the UDP implementation PR after this policy is accepted.

## References

- GitHub issue: [#2116](https://github.com/torrust/torrust-tracker/issues/2116)
- Local precedent:
  `console/tracker-client/docs/adrs/20260512080000_define_tracker_cli_io_contract_and_error_handling.md`
- Root supersession example:
  `docs/adrs/20260519000000_define_global_cli_output_contract.md`
