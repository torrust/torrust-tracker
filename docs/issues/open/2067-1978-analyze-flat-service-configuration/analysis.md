# Analysis Report: Flat Heterogeneous Service Configuration

> **Status:** Planned
>
> **Issue contract:** [ISSUE.md](ISSUE.md)
>
> **Evidence ledger:** [evidence.md](evidence.md)

This is the final decision record for the analysis-only issue. It must recommend exactly one
outcome: reject the change, defer the change, or create a separate implementation issue. It must
not describe unapproved production work as implemented.

## Executive Decision

| Field                  | Result |
| ---------------------- | ------ |
| Recommendation         | TODO   |
| Decision status        | TODO   |
| Rationale              | TODO   |
| Required prerequisites | TODO   |
| Proposed follow-up     | TODO   |

## Current-State Baseline

TODO: Describe the v3 configuration shape, cardinality/defaulting, startup phases, container and
registry behavior, `ConfigurationInstanceId`, shared UDP behavior, and configuration redaction.

## Candidate Representations

TODO: Compare at least two TOML/Rust representations. Include the adjacent-tagged form and the
rejected or less-preferred alternatives with their operator-facing trade-offs.

## Feasibility Results

TODO: Summarize TOML parsing, Serde round-trip behavior, Figment defaults and environment
overrides, discriminator validation, and any prototype constraints. Link each conclusion to an
evidence record.

## Runtime and Normalization Model

TODO: Explain the proposed normalization boundary, role-specific views, startup dependencies,
singleton/default behavior, shared UDP policies, and observability compatibility invariants.

## Identity, Ordering, and Migration

TODO: Define the `ServiceKind` to `ServiceRole` mapping, compare role-local and global positions,
document the v3 cross-role ordering limitation, and state any canonical migration-order rule.

## Schema Lifecycle, Security, and Compatibility

TODO: State the future version-loading/transition recommendation, #1980/#1490 relationship,
redaction requirements, affected external consumers, and compatibility constraints.

## Cost, Risks, and Recommendation

TODO: Estimate the affected modules and effort, list remaining risks, and provide the final
recommendation with exact proposed scope for any follow-up implementation issue.

## Evidence Index

TODO: Link each material conclusion above to the relevant section of [evidence.md](evidence.md).
