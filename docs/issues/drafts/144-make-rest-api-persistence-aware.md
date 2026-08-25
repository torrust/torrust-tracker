---
doc-type: issue
issue-type: enhancement
status: draft
priority: p2
epic: 144
github-issue: null
spec-path: docs/issues/drafts/144-make-rest-api-persistence-aware.md
branch: null
related-pr: null
last-updated-utc: 2026-08-25 00:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/open/999-1978-optional-database-configuration/solution.md
    - docs/issues/open/999-1978-optional-database-configuration/persistence-unavailable-scenarios.md
    - docs/issues/open/999-1978-optional-database-configuration/persistence-free-runtime-activation-draft.md
    - packages/rest-api-protocol/
    - packages/rest-api-application/
    - packages/rest-api-runtime-adapter/
    - packages/axum-rest-api-server/
---

# Make the REST API persistence-aware

## Subissue of EPIC #144 — next-major REST API work

## Problem

The tracker’s target architecture permits a persistence-free runtime, but the
current REST API assumes persistence-backed whitelist and key services exist.
It also exposes completed counters documented as historical values even when a
value is only known for the current tracker process.

The current code conflates distinct states:

1. A feature is disabled deliberately by configuration.
2. A configured database fails operationally after startup.
3. A current/session metric exists, while its historical counterpart is
   unavailable.

A direct whitelist or key operation for a disabled capability must not become a
misleading database failure. Likewise, a session-only completed count must not
be documented or serialized as an undifferentiated lifetime count.

Issue #999 records the source-level inventory and the approved target behavior
in `persistence-unavailable-scenarios.md`. Its small activation follow-up keeps
`http_api` persistence-required until this next-major REST API contract work is
complete.

## Goal

Make the REST API explicitly capability-aware and persistence-aware so it can
start in a persistence-free tracker deployment without confusing intentional
configuration, operational failures, session values, and historical values.

## Approved Contract Direction

### Disabled direct capabilities

When a client calls a whitelist route while `core.listed = false`, or a key
route while `core.private = false`, return HTTP `409 Conflict` using the
existing `ActionStatus::Err` response shape:

```json
{
  "status": "err",
  "reason": "Whitelist capability is disabled by configuration (`core.listed = false`)."
}
```

The protocol/application boundary must carry a distinct
`DisabledByConfiguration` error. It must not reuse database error variants.

Configured database failures remain a distinct operational state and retain
server-error handling; they are not configuration-disabled responses.

### Completed metric semantics

Keep in-memory routes available when their data is meaningful. Do not use a
negative numeric sentinel for missing history. Replace the ambiguous historical
meaning of `completed: u64` with an explicit next-major response model that can
distinguish at least:

- session-only value;
- restored/persisted historical value; and
- unavailable historical value.

The final DTO names and migration policy require review during implementation.

## Scope

### In Scope

- Add capability-aware composition for REST API services and routes.
- Add the HTTP 409 configuration-disabled response for direct whitelist and key
  operations.
- Preserve the distinction between configuration-disabled and operational
  database failure across protocol, application, runtime-adapter, and Axum
  layers.
- Define and implement next-major explicit completed-metric provenance/history
  semantics for stats and torrent responses.
- Update REST API client models, contract tests, documentation, and migration
  guidance for the new major API contract.
- Enable the persistence-free runtime activation follow-up to remove its
  temporary `http_api` persistence requirement.

### Out of Scope

- Changing v2 tracker configuration behavior.
- Replacing the tracker persistence schema or creating feature-specific
  migration streams.
- Hiding supported routes with an accidental 404 or reporting disabled
  capabilities as authorization failures.
- Using numeric sentinels for missing historical values.

## Implementation Considerations

| Area                       | Expected work                                                                           |
| -------------------------- | --------------------------------------------------------------------------------------- |
| `rest-api-protocol`        | Add a distinct disabled-by-configuration error and next-major response DTOs.            |
| `rest-api-application`     | Preserve the disabled capability state through use cases.                               |
| `rest-api-runtime-adapter` | Compose optional capability services and provide in-memory adapters where appropriate.  |
| `axum-rest-api-server`     | Map disabled capability errors to HTTP 409 and update route contract tests.             |
| `rest-api-client`          | Update next-major client DTOs and error handling.                                       |
| Tracker activation         | Remove `http_api` from the persistence-required matrix once this contract is available. |

## Verification

- [ ] Contract tests distinguish disabled capability (409) from operational
      database failure (server error).
- [ ] Whitelist/key disabled routes do not attempt persistence access.
- [ ] Stats/torrent responses explicitly describe current versus historical
      completed values.
- [ ] No response uses a negative numeric sentinel for unavailable history.
- [ ] REST API starts in the persistence-free tracker scenario after its
      composition dependencies are updated.
- [ ] REST API client and user-facing migration documentation are updated.
- [ ] `linter all` and relevant workspace tests pass.

## References

- GitHub EPIC issue #144
- Issue #999
- `docs/issues/open/999-1978-optional-database-configuration/solution.md`
- `docs/issues/open/999-1978-optional-database-configuration/persistence-unavailable-scenarios.md`
- `docs/issues/open/999-1978-optional-database-configuration/persistence-free-runtime-activation-draft.md`
