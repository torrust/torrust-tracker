---
semantic-links:
  related-artifacts:
    - docs/issues/open/999-1978-optional-database-configuration/ISSUE.md
    - packages/tracker-core/
    - packages/configuration/src/v2_0_0/
    - packages/configuration/src/v3_0_0/
    - src/container.rs
---

# Phase 1 - Persistence dependency analysis

## Purpose

Establish evidence for the current persistence lifecycle and every dependency
that assumes a database exists. This phase must not select the implementation
solution or change runtime behaviour.

## Required Investigation

### Configuration and startup lifecycle

- Trace how v2 configuration currently requires and supplies database settings.
- Trace v3 `Core.database` parsing, defaults, and all v3-to-runtime handoff
  paths planned under #1980.
- Locate each database-driver construction path and identify its owner.
- Locate each migration invocation and determine whether it is coupled to
  construction, connection, or an explicit bootstrap operation.
- Inspect `share/container/entry_script_sh`, the container image, and related
  configuration/install paths. Record directory creation, default-database
  installation, required database-driver environment variables, and any other
  database side effect before the tracker process starts.
- Reconcile the source-level lifecycle findings with the baseline end-to-end
  evidence in `baseline-e2e-verification.md`.
- Trace the configuration-consistency validation path from
  `packages/configuration/src/validator.rs` through `Configuration::validate()`
  and bootstrap. Record whether each database requirement is expressible as a
  cross-field configuration rule or instead needs runtime/environment
  validation.
- Record separate SQLite, MySQL, and PostgreSQL behaviour, including file or
  connection side effects and migration preconditions.

### Persistence consumer inventory

For every consumer, record the source location, enablement condition, repository
or service dependency, startup dependency, runtime failure mode, and tests.

| Domain / consumer                     | Enablement configuration | Persistence operations | REST API coupling | Findings |
| ------------------------------------- | ------------------------ | ---------------------- | ----------------- | -------- |
| Whitelist                             | TODO                     | TODO                   | TODO              | TODO     |
| Torrent completion metrics            | TODO                     | TODO                   | TODO              | TODO     |
| Private-tracker keys                  | TODO                     | TODO                   | TODO              | TODO     |
| Other direct `tracker-core` consumers | TODO                     | TODO                   | TODO              | TODO     |
| Indirect consumers and jobs           | TODO                     | TODO                   | TODO              | TODO     |

The consumer inventory identifies requirements; it is not a plan to create
independent schemas or migration streams. The tracker has one small shared
persistence schema. Phase 1 must confirm every migration invocation, but the
working constraint is all or nothing: no required consumers means no driver and
no migrations; one or more required consumers means one initialized driver and
the complete migration set.

### Management REST API inventory

For every route that reads or writes a persistence-backed domain, record the
route, authorization policy, feature dependency, current response when the
underlying domain is unavailable, and desired contract candidates. Do not decide
the final response in this phase.

| Route / operation | Domain | Current dependency path | Current unavailable behaviour | Evidence |
| ----------------- | ------ | ----------------------- | ----------------------------- | -------- |
| TODO              | TODO   | TODO                    | TODO                          | TODO     |

### Compatibility and activation inventory

- Confirm that v2 must remain unchanged and continues requiring a database.
- Identify every default configuration, helper, example, benchmark, E2E setup,
  and deployment document that will be affected if v3 `Core.database` becomes
  optional.
- Identify container entrypoint changes needed to support a no-persistence v3
  deployment without requiring a database-driver environment variable or
  installing a default SQLite database.
- Identify the concrete #1980 consumer-migration tasks that may need to depend
  on this issue.

## Evidence Requirements

- Link source paths, tests, logs, or focused experiment results for every
  finding.
- Distinguish verified findings from hypotheses.
- Record contradictory evidence and unresolved questions explicitly.
- Update `solution.md` only after this document provides enough evidence to
  evaluate the alternatives.
