---
semantic-links:
  skill-links:
    - create-adr
  related-artifacts:
    - docs/AGENTS.md
    - docs/adrs/README.md
    - docs/adrs/index.md
    - docs/templates/ADR.md
    - .github/skills/dev/planning/create-adr/SKILL.md
    - console/tracker-client/docs/adrs/20260512080000_define_tracker_cli_io_contract_and_error_handling.md
    - docs/adrs/20260519000000_define_global_cli_output_contract.md
---

# Place ADRs by Decision Scope

## Scope

Root ADR. This decision establishes a repository-wide policy for placing ADRs across root and
package-local collections.

## Description

The repository currently collects ADRs in `docs/adrs/`, but workspace packages are intended to
be independently extractable. An ADR whose decision is owned solely by one package must travel
with that package; otherwise, extraction separates the implementation from its rationale.

The paths changed by an implementation do not reliably determine this ownership. A change in one
package can establish a repository policy, alter shared configuration or a protocol, or define an
inter-package contract. Such decisions need one repository-level record even when their immediate
implementation is local.

The tracker client provides the established precedent. Its original CLI I/O decision lives in
`console/tracker-client/docs/adrs/`, because extraction was anticipated. The later root ADR,
`20260519000000_define_global_cli_output_contract.md`, expanded the contract to all first-party
binaries and superseded the local ADR without removing its historical context.

## Agreement

### Placement criteria

Place an ADR in `packages/<package>/docs/adrs/` when all of the following apply:

- The decision is limited to that package's architecture, behavior, or public contract.
- The package owns the decision and its rationale.
- The ADR should remain with the package when it is extracted into its own repository.

Place an ADR in `docs/adrs/` when the decision governs the repository, affects multiple packages,
or defines an inter-package contract. Root placement is required for decisions about shared
configuration, protocol behavior, dependency policy, workspace-wide conventions, or another
cross-package interface, even if the implementation change initially touches one package.

When scope is uncertain, use root placement or resolve the scope during review. Do not infer scope
solely from the paths of affected implementation files.

### Local ADR collections

Each package-local ADR collection must contain:

- `README.md`, describing the collection's package ownership and its relationship to root ADRs.
- `index.md`, listing ADRs owned by that package.
- Timestamp-prefixed ADR files using `YYYYMMDDHHMMSS_snake_case_title.md`.

Root and package-local indexes are separate. List each ADR only in its owning collection's index;
do not duplicate package-local ADR rows in `docs/adrs/index.md`. Package documentation should link
to its local collection so the ADRs remain discoverable from the package entry point.

The established `console/tracker-client/docs/adrs/` collection follows the same ownership model
for an extractable application that is not under `packages/`.

### Supersession

When a package-local decision becomes repository-wide, create a root ADR. The root ADR must link
to the local ADR and explain the expanded scope. Update the local ADR with a `Status: Superseded`
link to the root ADR, while retaining the local ADR and its local index entry as historical
context. Do not move or duplicate the local ADR merely because it was superseded.

## Alternatives Considered

**Keep every ADR in `docs/adrs/`.** Rejected because package extraction would separate
package-owned implementation from the decision rationale that explains it.

**Place ADRs by implementation-file location.** Rejected because local implementation can carry
repository-wide consequences, especially for configuration, protocols, and shared contracts.

**Copy package-local ADRs into the root index.** Rejected because duplicated registry entries
create ambiguous ownership and drift.

**Move a local ADR to the root when its scope expands.** Rejected because the original local
decision remains useful historical context and should remain with an extracted package.

## Consequences

Package-owned rationale remains portable with extractable packages. Contributors must make an
explicit scope judgment when authoring ADRs, and reviewers must verify that judgment. Root ADR
navigation does not enumerate every package-local decision, so package documentation must expose
its own ADR collection.

This ADR does not itself migrate existing ADRs. Existing migrations, including the UDP-core ADR,
are completed by their owning implementation work after this policy is accepted.

## Date

2026-08-30

## References

- Issue: [#2116](https://github.com/torrust/torrust-tracker/issues/2116)
- Tracker-client local precedent:
  `console/tracker-client/docs/adrs/20260512080000_define_tracker_cli_io_contract_and_error_handling.md`
- Root supersession example:
  `docs/adrs/20260519000000_define_global_cli_output_contract.md`
