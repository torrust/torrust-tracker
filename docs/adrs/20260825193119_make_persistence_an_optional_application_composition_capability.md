---
semantic-links:
  related-artifacts:
    - docs/issues/closed/999-1978-optional-database-configuration/ISSUE.md
    - docs/issues/closed/999-1978-optional-database-configuration/solution.md
    - packages/configuration/src/v3_0_0/core.rs
    - packages/tracker-core/src/container.rs
    - src/bootstrap/persistence.rs
---

# Make persistence an optional application-composition capability

## Description

The tracker historically supports an in-memory deployment, but the active v2 runtime always constructs a database driver and applies the complete shared migration set during application-container initialization. The configuration can omit the v2 `[core.database]` TOML table only because it defaults to SQLite; the runtime cannot operate without persistence.

Schema v3 makes the absence of `[core.database]` representable. The actual persistence-free runtime is delivered by the post-v3-activation follow-up: until then, bootstrap passes an explicit temporary database dependency to preserve current effective runtime behavior.

The management REST API exposes both in-memory tracker information and direct
persistence-backed capabilities. Issue #2107 makes it available without
persistence and supplies configuration-disabled responses for direct key and
whitelist operations. API #144 retains completed-metric provenance work.

## Agreement

The v3 application treats persistence as an optional **application-composition capability**.

1. `Option<Database>` represents configured persistence. An absent database means persistence is unavailable by configuration.
2. Issue #999 implements and unit-tests one reusable bootstrap-owned persistence-requirement check. The activation follow-up invokes it after v3 configuration is loaded and before application-container construction, once bootstrap receives actual `Option<Database>` rather than the temporary compatibility bridge. The same feature-to-persistence matrix must not be duplicated in repositories, route handlers, or `packages/configuration::Validator`.
3. Listing, private-mode keys, and persistent completed statistics require configured persistence. If one is enabled without `[core.database]`, startup fails with a diagnostic that names both the enabled capability and the missing database configuration.
4. Phase 3 resolves the optional database at the existing `TrackerCoreContainer` initialization seam. The `Some` branch retains tracker-core's driver, migration, and store setup, then passes required stores to persistence-backed consumers. The future `None` branch selects persistence-absent composition before those consumers are built.
5. Driver, schema, and migration implementation ownership remains in `tracker-core`. The selected composition seam changes where optionality is resolved; it does not move schema ownership or introduce feature-specific schemas, migration streams, or migration selection.
6. When no capability requires persistence, the activation follow-up constructs no persistence driver, store, database file, network connection, or migration side effect.
7. The management REST API starts without persistence. Direct key and whitelist
  operations whose capabilities are disabled return controlled HTTP 409
  responses; GitHub issue #144 owns only the next-major completed-metric
  provenance response model.
8. Persistence configuration is evaluated at process startup only. Disabling persistence never deletes or alters prior database state; re-enabling the same target reuses it, and changing targets never transfers data automatically.
9. The container entrypoint defers persistence selection to actual v3 configuration. It does not require or default a database driver when persistence is absent, and it never destructively alters mounted state during a persistence transition.

## Alternatives considered

### Inject optional initialized persistence services

Bootstrap or application composition could initialize a driver, migrations, and stores and pass `Option<PersistenceServices>` into tracker-core.

This remains a fallback if resolving `Option<Database>` in tracker-core requires optional container fields, optionality in unrelated consumers, duplicate initialization paths, or weakens required dependency invariants. It is not selected initially because it is more invasive and could make top-level composition own lifecycle details currently owned by tracker-core.

### Keep a mandatory database in v3

Rejected. It abandons the tracker’s explicit in-memory deployment capability and preserves unconditional persistence coupling.

### Make persistence optional but let consumers fail when accessed

Rejected. It makes configuration errors delayed runtime failures and spreads feature-to-persistence knowledge across consumers.

### Duplicate the capability matrix in configuration validation and bootstrap

Rejected. Two owners would drift as services and configuration evolve. Bootstrap is the application-composition boundary that knows which services are being constructed.

## Consequences

- **Positive:** #999 separates the optional v3 representation and optional composition API from the later runtime behavior change, allowing #1980 to activate v3 first.
- **Positive:** Optionality is localized at the initialization seam; services in the persistence-enabled branch keep required store dependencies rather than repeatedly handling `Option` values.
- **Positive:** The shared-schema lifecycle stays simple: zero drivers in persistence-free mode, exactly one driver and complete migrations otherwise.
- **Positive:** Missing persistence is detected deterministically before driver construction rather than through a late repository failure.
- **Negative:** The future `None` branch must construct a persistence-absent set of services before public runtime activation; Issue #999 deliberately does not activate that branch.
- **Negative:** Completed-metric provenance, the container entrypoint, and
  restart-transition verification require later work.
- **Negative:** State produced during a persistence-free interval is not recoverable when persistence is later re-enabled.

## Date

2026-08-25

## References

- Issue #999
- Configuration-overhaul EPIC #1978
- `docs/issues/closed/999-1978-optional-database-configuration/analysis.md`
- `docs/issues/closed/999-1978-optional-database-configuration/solution.md`
- `docs/adrs/20260723184019_separate_configuration_value_invariants_from_consistency_validation.md`
- GitHub issue #144
