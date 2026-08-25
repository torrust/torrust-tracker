---
status: draft
purpose: persistence-unavailable-scenario-catalog
related-issue: 999
related-github-issue: 144
last-updated-utc: 2026-08-25 00:00
semantic-links:
  related-artifacts:
    - docs/issues/open/999-1978-optional-database-configuration/analysis.md
    - docs/issues/open/999-1978-optional-database-configuration/solution.md
    - docs/issues/open/999-1978-optional-database-configuration/persistence-free-runtime-activation-draft.md
    - docs/issues/open/999-1978-optional-database-configuration/persistence-awareness-epic-draft.md
      - docs/issues/drafts/144-make-rest-api-persistence-aware.md
---

# Persistence-unavailable scenario catalog

> **Planning catalog:** This is a case log for Issue #999 and its follow-ups.
> It distinguishes intentional absence of persistence from operational database
> failure. Update it when implementation finds a new case; do not substitute it
> for authoritative API contracts, tests, or issue specifications.

## State vocabulary

| State                | Meaning                                                                |
| -------------------- | ---------------------------------------------------------------------- |
| Persistence absent   | V3 `[core.database]` is omitted and the activation path honors `None`. |
| Capability disabled  | A feature is intentionally off in configuration.                       |
| Persistence required | An enabled capability needs a configured database.                     |
| Operational failure  | A configured database fails after startup or during an operation.      |
| Session-only data    | A value exists only for the current process lifetime.                  |
| Historical data      | A value is restored from or maintained in persistence.                 |

## Scenario catalog

| ID  | Situation                                                                     | Required behavior                                                                                                                   | Delivery owner                                             | Status                    |
| --- | ----------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------- | ------------------------- |
| S1  | `core.listed = true`, but persistence is absent                               | Bootstrap reports `ListedRequiresDatabase` before container or driver construction.                                                 | #999 implements/tests; activation follow-up invokes.       | Approved                  |
| S2  | `core.private = true`, but persistence is absent                              | Bootstrap reports `PrivateRequiresDatabase` before container or driver construction.                                                | #999 implements/tests; activation follow-up invokes.       | Approved                  |
| S3  | Persistent completed metrics enabled, but persistence is absent               | Bootstrap reports `PersistentTorrentCompletedStatRequiresDatabase` before container or driver construction.                         | #999 implements/tests; activation follow-up invokes.       | Approved                  |
| S4  | No persistence-required capability is enabled, and persistence is absent      | Public UDP/HTTP tracker starts with no driver, file, connection, migration, or persistence stores.                                  | Activation follow-up.                                      | Planned                   |
| S5  | Direct whitelist route called while listing is disabled                       | Do not attempt a database operation. Target contract: HTTP 409 plus `ActionStatus::Err` and `DisabledByConfiguration`.              | Draft `144-make-rest-api-persistence-aware.md`.            | Approved target; deferred |
| S6  | Direct key route called while private mode is disabled                        | Do not attempt a database operation. Target contract: HTTP 409 plus `ActionStatus::Err` and `DisabledByConfiguration`.              | Draft `144-make-rest-api-persistence-aware.md`.            | Approved target; deferred |
| S7  | A configured database fails during whitelist/key operation                    | Preserve operational database-failure behavior; do not report this as configuration-disabled.                                       | Existing behavior; review when API #144 changes responses. | Current                   |
| S8  | Stats/torrent endpoint returns a current value with no historical persistence | Keep the endpoint available, but do not represent a session-only count as an undifferentiated lifetime count. No negative sentinel. | Draft `144-make-rest-api-persistence-aware.md`.            | Approved target; deferred |
| S9  | Stats/torrent endpoint returns a value restored from persistence              | Represent historical/provenance semantics explicitly and consistently with S8.                                                      | Draft `144-make-rest-api-persistence-aware.md`.            | Approved target; deferred |
| S10 | `http_api` configured before API #144 is delivered                            | Temporary activation constraint: require persistence; do not claim API works without it.                                            | Activation follow-up.                                      | Planned                   |
| S11 | `http_api` configured after API #144 is delivered                             | API may start without persistence; direct disabled capabilities follow S5/S6.                                                       | API #144 work and later activation review.                 | Planned                   |
| S12 | Operator restarts from persistence-enabled to persistence-free configuration  | Do not open, migrate, write, delete, or otherwise alter the previously selected database target.                                    | Activation follow-up and operational docs.                 | Approved                  |
| S13 | Operator restarts from persistence-free to persistence-required configuration | Require a selected database; initialize its complete shared schema and reuse data if the target already exists.                     | Activation follow-up and operational docs.                 | Approved                  |
| S14 | Operator changes database driver or location                                  | Initialize/migrate the new target; never copy or delete historical data automatically.                                              | Activation follow-up and operational docs.                 | Approved                  |
| S15 | Container starts with persistence absent                                      | Do not require a driver override or install/create persistence-specific SQLite configuration, file, or directory.                   | Activation follow-up container work.                       | Approved                  |
| S16 | Container starts with persistence configured                                  | Follow actual v3 configuration; retain non-destructive driver-specific setup only when selected.                                    | Activation follow-up container work.                       | Approved                  |

## Rules for new discoveries

1. Classify the new case using the state vocabulary.
2. Add it here with source evidence and an owner.
3. If it is a persistence-required capability, also add it to the centralized
   bootstrap requirement matrix and focused tests.
4. If it changes a public REST contract, coordinate it with
   `docs/issues/drafts/144-make-rest-api-persistence-aware.md` and GitHub
   issue #144.
5. Never reuse an operational database error for an intentionally disabled
   capability.
