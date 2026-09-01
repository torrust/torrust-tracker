---
semantic-links:
  related-artifacts:
    - docs/issues/open/2122-expose-unambiguous-download-counter-semantics/ISSUE.md
    - docs/adrs/20260825193119_make_persistence_an_optional_application_composition_capability.md
    - packages/tracker-core/src/statistics/mod.rs
    - packages/tracker-core/src/statistics/event/handler.rs
    - packages/tracker-core/src/statistics/persisted/mod.rs
    - packages/rest-api-runtime-adapter/src/v1/adapters/stats.rs
    - packages/axum-rest-api-server/src/v1/routes.rs
---

# Define completed-download metric retention names

## Scope

This is a repository-level decision because completed-download retention is a
cross-package contract between tracker-core metrics and REST API v1 responses.
It is therefore recorded in `docs/adrs/`.

## Description

A completed-download counter is process-lifetime when persistence is disabled,
but is restored and maintained historically when persistent completed statistics
are enabled. The legacy REST `completed` field and
`tracker_core_persistent_torrents_downloads_total` metric do not identify this
conditional retention behavior. Their identifiers cannot change in API v1
without breaking consumers.

Issue #999 deliberately deferred a response-field model until retention and
persistence-free application composition were available. That deferral does not
prohibit an additive v1 bridge: a zero persisted count is unambiguous when a
separate authoritative availability boolean accompanies it.

## Agreement

1. `in_session` identifies a process-lifetime count. It starts at zero for each
   tracker process and is advanced by the in-memory completed-download event
   listener.
2. `persisted` identifies a count restored from and maintained in persistent
   storage. It is seeded from the stored aggregate at startup and advances only
   after the global persistent update succeeds.
3. #2107's independent in-memory and persistent listeners remain independent.
   The persisted listener receives the statistics repository only to update its
   distinct successful-persistence view; it does not own or alter the
   in-session listener.
4. `tracker_core_in_session_torrents_downloads_total` is available with tracker
   usage statistics. `tracker_core_persisted_torrents_downloads_total` is
   exported only when persistent completed statistics are enabled. Disabled
   persistence is omission, not a Prometheus zero-value sentinel.
5. API v1 retains `completed` and the legacy
   `tracker_core_persistent_torrents_downloads_total` identifier with their
   conditional values. Their descriptions deprecate them in favor of explicit
   views. API v2 removes these deprecated compatibility paths.
6. API v1 adds `completed_in_session`, `completed_persisted`, and
   `completed_persisted_enabled`. When disabled, the persisted number is zero
   and the boolean is false. When enabled, a numeric zero is an observed valid
   historical count. The REST composition root derives this boolean from the
   validated `persistent_torrent_completed_stat` configuration.

## Consequences

Consumers can migrate to explicit retention names before API v2 without a
breaking v1 change. Metrics users must treat absence of the persisted Prometheus
metric as disabled persistence and REST users must use the boolean rather than
infer availability from a zero value.

## Date

2026-09-01

## References

- Issue #2122
- Issue #999
- ADR [Make persistence an optional application-composition capability](20260825193119_make_persistence_an_optional_application_composition_capability.md)
- `docs/issues/closed/999-1978-optional-database-configuration/ISSUE.md`
