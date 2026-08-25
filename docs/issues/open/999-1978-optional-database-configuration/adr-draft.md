---
status: approved-draft
intended-destination: docs/adrs/
related-issue: 999
semantic-links:
  related-artifacts:
    - docs/issues/open/999-1978-optional-database-configuration/ISSUE.md
    - docs/issues/open/999-1978-optional-database-configuration/analysis.md
    - docs/issues/open/999-1978-optional-database-configuration/solution.md
    - docs/adrs/20260723184019_separate_configuration_value_invariants_from_consistency_validation.md
---

# Draft ADR - Make persistence an optional application-composition capability

> **Approved Phase 2 draft:** Copy this artifact to `docs/adrs/` with its final
> timestamped filename during Phase 3. Reconcile it with the implemented code,
> tests, API contract, and review outcome before treating it as a final ADR.

## Description

The tracker historically supports an in-memory deployment, but the active v2
runtime always constructs a database driver and applies the complete shared
migration set during application-container initialization. The configuration
can omit the v2 `[core.database]` TOML table only because it defaults to
SQLite; the runtime cannot operate without persistence.

Schema v3 makes the absence of `[core.database]` representable. The actual
persistence-free runtime is delivered by the post-v3-activation follow-up:
until then, bootstrap passes an explicit temporary database dependency to
preserve current effective runtime behavior.

The management REST API exposes both in-memory tracker information and direct
persistence-backed capabilities. It must remain usable in a persistence-free
deployment, without representing a disabled capability as an accidental
database failure.

## Agreement

The v3 application treats persistence as an optional **application-composition
capability**.

1. `Option<Database>` represents configured persistence. An absent database
   means persistence is unavailable by configuration.
2. Issue #999 implements and unit-tests one reusable bootstrap-owned
   persistence-requirement check. The activation follow-up invokes it after v3
   configuration is loaded and before application-container construction, once
   bootstrap receives actual `Option<Database>` rather than the temporary
   compatibility bridge. The same feature-to-persistence matrix must not be
   duplicated in repositories, route handlers, or
   `packages/configuration::Validator`.
3. Listing, private-mode keys, and persistent completed statistics require
   configured persistence. If one is enabled without `[core.database]`, startup
   fails with a diagnostic that names both the enabled capability and the
   missing database configuration.
4. When any capability requires persistence, bootstrap constructs one selected
   driver and applies the complete shared migration set once. Feature-specific
   schemas, migration streams, and migration selection are prohibited. Feature
   configuration controls code behavior rather than database fragments.
5. When no capability requires persistence, the activation follow-up's application composition constructs
   only in-memory services and no persistence driver or migration side effect.
6. The management REST API may start without persistence only after the
   next-major API work tracked by GitHub issue #144 implements the approved
   configuration-disabled response model. Until then, it remains
   persistence-required at activation.
7. The GitHub issue #144 API work must ensure fields do not silently present
   session values as historical persisted values and must not use negative
   numeric sentinels for unavailable history.
8. Persistence configuration is evaluated at process startup only. Disabling
   persistence never deletes or alters prior database state; re-enabling the
   same target reuses it, and changing targets never transfers data
   automatically.
9. The container entrypoint defers persistence selection to actual v3
   configuration. It does not require or default a database driver when
   persistence is absent, and it never destructively alters mounted state
   during a persistence transition.

## Alternatives Considered

### Keep a mandatory database in v3

Rejected. It abandons the tracker’s explicit in-memory deployment capability
and preserves unconditional persistence coupling.

### Make persistence optional but let consumers fail when accessed

Rejected. It makes configuration errors delayed runtime failures and spreads
feature-to-persistence knowledge across consumers.

### Make the REST API require persistence

Rejected as the target architecture, but retained as a temporary activation
constraint until GitHub issue #144 provides the compatibility-breaking REST
response-model changes.

### Duplicate the capability matrix in configuration validation and bootstrap

Rejected. Two owners would drift as services and configuration evolve.
Bootstrap is the application-composition boundary that knows which services are
being constructed.

## Consequences

- **Positive:** #999 separates optional representation/container dependencies
  from the later runtime behavior change, allowing #1980 to activate v3 first.
- **Positive:** After the activation follow-up, public UDP/HTTP tracker
  services can run without a database when persistence-backed capabilities are
  disabled. The management API joins that mode only after API #144 implements
  its approved next-major contract.
- **Positive:** Missing persistence is detected deterministically before driver
  construction rather than through a late repository failure.
- **Positive:** The shared-schema lifecycle stays simple: zero drivers in
  persistence-free mode, exactly one driver and complete migrations otherwise.
- **Positive:** Future features avoid conditional schema upgrade and
  compatibility paths even when their current persistence tables look
  independent.
- **Positive:** Operators can change persistence configuration without risking
  automatic data deletion or unexpected cross-driver migration.
- **Negative:** Application containers, REST API composition, route behavior,
  response models, test helpers, and the container entrypoint require changes.
- **Negative:** Some API consumers may need to adapt to explicit
  configuration-disabled or historical-data-unavailable semantics.
- **Negative:** State produced during a persistence-free interval is not
  recoverable when persistence is later re-enabled.

## Date

Approved as a draft on 2026-08-25; finalize during Issue #999 Phase 3.

## References

- Issue #999
- Configuration-overhaul EPIC #1978
- `docs/issues/open/999-1978-optional-database-configuration/analysis.md`
- `docs/issues/open/999-1978-optional-database-configuration/solution.md`
- `docs/adrs/20260723184019_separate_configuration_value_invariants_from_consistency_validation.md`
- GitHub issue #144
