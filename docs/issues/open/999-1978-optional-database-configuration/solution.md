---
semantic-links:
  related-artifacts:
    - docs/issues/open/999-1978-optional-database-configuration/ISSUE.md
    - docs/issues/open/999-1978-optional-database-configuration/analysis.md
    - docs/issues/open/1978-configuration-overhaul-epic/configuration-v2-to-v3-migration.md
---

# Phase 2 - Optional persistence solution

## Status

Pending Phase 1 evidence. Do not treat the preliminary direction below as an
approved implementation decision.

## Decision to Make

Select a design that allows v3 `[core.database]` to be omitted when persistence
is unused, while rejecting startup if an enabled persistence-backed capability
requires a database. The selected design must preserve v2 behaviour unchanged.

## Candidate Direction

The expected configuration representation is `Option<Database>` on v3 `Core`.
When absent, runtime construction must not create a driver, database file,
network connection, or migration side effect. This remains subject to Phase 1:
the analysis may identify a more suitable boundary or a prerequisite refactor.

The performance objective is explicit: a public UDP tracker that enables none
of the basic persistence-backed capabilities must run without a database. If at
least one such capability is enabled, the selected database is a normal shared
tracker dependency: initialize it once and apply the whole migration set. Do
not design separate schemas or migration streams per feature.

## Required Solution Content

### Configuration contract

- Define v3 TOML semantics for an omitted `[core.database]` section.
- Specify whether empty or partial database sections are rejected and how their
  errors are reported.
- Define the v2-to-v3 migration guidance and confirm v2 remains unchanged.

### Capability validation matrix

For every persistence-backed capability identified in Phase 1, define its
enablement condition and startup validation result when the database is absent.

| Capability | Enabled when | Database omitted result | Error text / code | Tests |
| ---------- | ------------ | ----------------------- | ----------------- | ----- |
| TODO       | TODO         | TODO                    | TODO              | TODO  |

When a requirement depends only on the relationship between configuration
options, implement it as a configuration-consistency rule through `Validator`
and `SemanticValidationError`. The precedent is
`UselessPrivateModeSection`, which rejects `[core.private_mode]` when
`core.private` is false. Follow the validation-layer policy in
`docs/adrs/20260723184019_separate_configuration_value_invariants_from_consistency_validation.md`:
field-local constraints belong in typed deserialization, and filesystem,
network, or deployment facts belong in runtime/environment validation.

Keep the rule centralized at the configuration/bootstrap boundary rather than
asking each repository or feature implementation to discover a missing database
at runtime. Phase 2 must select and document one owner for this check, including
why that owner matches the validation-layer policy.

### Runtime lifecycle

- Define the owner and timing of driver construction.
- Define the owner and timing of migration execution.
- Define repository/service construction when persistence is absent.
- State whether any constructor needs refactoring to remove migration side
  effects, based on Phase 1 evidence.
- Define the all-or-nothing migration contract: if a database is required,
  apply the complete shared schema migration set once; if none is required, do
  not construct a driver or execute any migration.
- Define container-entrypoint behaviour for deployments without persistence,
  including database-driver environment variables, default-database installation,
  and database-directory setup.

### REST API contract

For each route recorded in Phase 1, choose a deterministic outcome when
persistence is unavailable: absence because the feature is disabled, a
documented client error, or another explicitly justified response. The result
must never be an accidental driver or repository failure.

### EPIC ordering and activation decision

State one of the following and update EPIC #1978 accordingly:

1. #999 blocks #1980 and v3 activation because optional database configuration
   is part of the required v3 runtime contract.
2. #999 does not block activation, with documented evidence that activation
   remains correct without this optionality.

### Alternatives and trade-offs

Evaluate at least these alternatives against Phase 1 evidence:

- Keep the database mandatory in v3.
- Make database configuration optional but allow runtime failures for users of
  persistence-backed capabilities.
- Make database configuration optional and validate capability requirements at
  startup.

## Approval Record

Add the approved design, approver, UTC timestamp, decision rationale, and any
required ADR here before Phase 3 starts.
