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

The tracker’s target architecture permits a persistence-free runtime. GitHub
issue #2107 delivered capability-aware key and whitelist route composition plus
configuration-disabled responses. The REST API still exposes completed counters
documented as historical values even when a value is only known for the current
tracker process.

The current code conflates distinct states:

1. A configured database fails operationally after startup.
2. A current/session metric exists, while its historical counterpart is
   unavailable.

A session-only completed count must not be documented or serialized as an
undifferentiated lifetime count.

Issue #999 records the source-level inventory and #2107 delivery status in
`persistence-unavailable-scenarios.md`.

## Goal

Make completed-metric responses explicitly distinguish session values from
historical values without confusing either state with an operational database
failure.

## Approved Contract Direction

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

- Define and implement next-major explicit completed-metric provenance/history
  semantics for stats and torrent responses.
- Update REST API client models, contract tests, documentation, and migration
  guidance for the new major API contract.

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
| `rest-api-protocol`        | Define next-major completed-metric provenance DTOs.                                     |
| `rest-api-application`     | Preserve provenance/history state through use cases.                                    |
| `rest-api-runtime-adapter` | Map in-memory and restored data to the next-major response model.                       |
| `axum-rest-api-server`     | Update stats and torrent response contracts.                                            |
| `rest-api-client`          | Update next-major client DTOs and migration guidance.                                   |

## Verification

- [ ] Stats/torrent responses explicitly describe current versus historical
      completed values.
- [ ] No response uses a negative numeric sentinel for unavailable history.
- [ ] REST API client and user-facing migration documentation are updated.
- [ ] `linter all` and relevant workspace tests pass.

## References

- GitHub EPIC issue #144
- Issue #999
- `docs/issues/open/999-1978-optional-database-configuration/solution.md`
- `docs/issues/open/999-1978-optional-database-configuration/persistence-unavailable-scenarios.md`
- `docs/issues/open/999-1978-optional-database-configuration/persistence-free-runtime-activation-draft.md`
