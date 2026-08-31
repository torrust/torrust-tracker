---
doc-type: issue
issue-type: feature
status: open
priority: p2
github-issue: 2023
spec-path: docs/issues/open/2023-1978-expose-configured-public-urls-in-runtime-observability.md
branch: 2023-expose-configured-public-urls
related-pr: null
depends-on:
  - docs/issues/closed/1417-1978-add-public-service-url-to-configuration.md
  - docs/issues/open/1980-1978-configuration-overhaul-final-cleanup.md
last-updated-utc: 2026-08-31 00:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/configuration/src/v3_0_0/public_url.rs
    - packages/axum-health-check-api-server/
    - packages/http-core/src/event.rs
    - packages/udp-core/src/event.rs
    - packages/axum-http-server/
    - packages/axum-rest-api-server/
    - src/bootstrap/
---

# Issue #2023 - Expose Configured Public URLs in Runtime Observability

## Goal

Use the v3 `public_url` configuration values introduced by #1417 in health-check responses,
metrics, and runtime logs without conflating them with a service's configured bind address or
its post-bind `ServiceBinding`.

## Background

Each service has three distinct concepts:

| Concept                 | Source                                          | Meaning                                                                                                                                                                                                     |
| ----------------------- | ----------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Configured bind address | `bind_address` configuration                    | The requested local socket bind target. It may be wildcard (`0.0.0.0` or `[::]`) and may use port `0`.                                                                                                      |
| Service binding         | `ServiceBinding` created after the socket binds | The protocol plus the actual local socket address. An OS-assigned ephemeral port replaces configured port `0`, but a wildcard address remains wildcard. It is an identity, not necessarily a reachable URL. |
| Public URL              | Optional v3 `public_url` configuration          | The operator-declared external endpoint. It may differ completely from the bind address and service binding because of reverse proxies, NAT, TLS termination, or DNS.                                       |

`internal_service_url` is a possible future concept. It is not implemented, must not be added by
this issue, and cannot be inferred reliably from a wildcard service binding because a wildcard
listener can be reachable through multiple interfaces.

Issue #1417 stores and validates typed v3 `public_url` values but deliberately does not consume them at
runtime. #1980 migrates runtime consumers to explicit v3 configuration imports. This issue must
follow both changes.

## Scope

### In Scope

- Add an optional public-URL representation to health-check service details while preserving the
  existing `service_binding`, `binding`, and `service_type` fields.
- Add an optional `public_url` label to relevant per-service metrics when an operator configures
  a public URL.
- Add the configured `public_url`, when present, to relevant service startup and request logs;
  retain the service binding as the local service identity.
- Define and test the absent-value behavior: services without `public_url` remain valid and do
  not claim a public endpoint.
- Test that `public_url`, configured `bind_address`, and post-bind `ServiceBinding` remain
  distinguishable, including a wildcard bind address with an OS-assigned port.

### Out of Scope

- Changing how #1417 validates or stores v3 `public_url` values.
- Changing `ServiceBinding` or adding an `internal_service_url` type.
- Choosing a concrete reachable interface for wildcard listeners.
- Modifying the v2 configuration schema or supporting a v2 runtime fallback.
- Changing BitTorrent protocol behavior or URL path routing.

## Compatibility Decisions

| Surface      | Required behavior                                                                                                                     |
| ------------ | ------------------------------------------------------------------------------------------------------------------------------------- |
| Health check | Add an optional `public_url` field. Retain `service_binding`, `binding`, and `service_type` unchanged.                                |
| Metrics      | Add `public_url` only when configured. Confirm and document the resulting Prometheus series/cardinality effect.                       |
| Logs         | Emit `service_binding` as the local identity and optional `public_url` as the operator-declared endpoint. Neither replaces the other. |

## Maintainer Decisions

- Health-check responses always include `public_url`; the field is `null` when the service has no
  configured public URL.
- Emit `public_url` only in service startup logs, not in per-request logs.
- Add `public_url` to every per-service metric family that already includes service-binding labels.
  Do not add it to metric families that lack those labels.
- Document the Prometheus series/cardinality effect next to the relevant metric definitions.

## Implementation Plan

| ID  | Status  | Task                                                                  | Notes                                                            |
| --- | ------- | --------------------------------------------------------------------- | ---------------------------------------------------------------- |
| T1  | DONE    | Review v3 runtime configuration access after #1980                    | Consumes v3 typed configuration only; no v2 fallback added.      |
| T2  | DONE    | Extend health-check contract                                          | Adds nullable `public_url` while preserving existing fields.     |
| T3  | DONE    | Extend per-service metric labels                                      | Adds the label where request contexts have binding labels.       |
| T4  | DONE    | Extend startup logging                                                | Records `service_binding` and nullable `public_url` separately.  |
| T5  | DONE    | Add focused tests                                                     | Covers HTTP, UDP, absent values, wildcard binding, and port `0`. |
| T6  | PARTIAL | Run automatic and manual verification                                 | Automatic evidence recorded; manual scenarios remain TODO.       |
| T7  | N/A     | Update migration guide if this subissue affects the config public API | No configuration public API changed.                             |

## Progress Tracking

### Workflow Checkpoints

- [x] Specification drafted and approved by user/maintainer
- [x] GitHub issue created: #2023
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests)
- [ ] Manual verification scenarios executed and recorded
- [ ] Acceptance criteria reviewed after implementation
- [ ] Issue closed and specification moved to `docs/issues/closed/`

### Progress Log

- 2026-07-22 13:15 UTC - agent - Drafted as an EPIC #1978 follow-up after maintainer
  clarification that `public_url`, `ServiceBinding`, and the future `internal_service_url` are
  separate concepts.
- 2026-07-22 13:35 UTC - agent - Maintainer approved the specification and created GitHub issue
  #2023.
- 2026-08-31 00:00 UTC - maintainer - Confirmed nullable health-check output, startup-only log
  fields, and metric coverage wherever service-binding labels already exist.
- 2026-08-31 00:00 UTC - agent - Implemented optional configured public URLs in runtime metadata,
  health-check responses, per-service metrics, and service startup logs. Automatic verification
  passed; evidence is recorded in
  `2023-1978-expose-configured-public-urls-in-runtime-observability-verification.md`.

## Acceptance Criteria

- [x] AC1: A configured v3 `public_url` is exposed as an optional health-check field without
      replacing existing service-identity fields.
- [x] AC2: Relevant per-service metrics expose `public_url` only when configured.
- [x] AC3: Relevant startup logs identify the local service with `service_binding` and,
      independently, the configured `public_url` when present.
- [x] AC4: A wildcard bind address with configured port `0` demonstrates three separate values:
      configured bind address, post-bind service binding, and configured public URL.
- [x] AC5: Services without `public_url` preserve existing health-check, metric, and logging
      behavior.
- [x] AC6: No `internal_service_url` implementation or `torrust-net-primitives` change is made.
- [x] AC7: `linter all` and relevant tests pass.
- [ ] AC8: Manual verification evidence records both configured and absent `public_url` cases.

## Verification Plan

### Automatic Checks

- `linter all`
- Focused tests for changed server, health-check, and metrics packages
- `cargo test --workspace`

### Manual Verification Scenarios

| ID  | Scenario                                                                                                                                               | Expected Result                                                                                                         | Status | Evidence |
| --- | ------------------------------------------------------------------------------------------------------------------------------------------------------ | ----------------------------------------------------------------------------------------------------------------------- | ------ | -------- |
| M1  | Start a local v3 tracker with `bind_address = "0.0.0.0:0"` and `public_url = "https://tracker.example.test/announce"`; call the health-check endpoint. | The response distinguishes the configured public URL from the post-bind wildcard service binding with OS-assigned port. | TODO   |          |
| M2  | Send an HTTP announce to that local service and query Prometheus metrics.                                                                              | The matching metric has `public_url="https://tracker.example.test/announce"` and retains its `server_binding_*` labels. | TODO   |          |
| M3  | Repeat with no `public_url` configured.                                                                                                                | Existing fields remain; no public URL is claimed or labelled.                                                           | TODO   |          |

## Risks and Trade-offs

- **Metric cardinality**: public URLs can increase Prometheus time-series cardinality. Restrict the
  label to configured per-service metric series and document the behavior.
- **Consumer compatibility**: health-check response additions must be optional and additive.
- **Identity confusion**: logs and API fields must name `service_binding` and `public_url`
  explicitly so an operator does not mistake either for an internal reachable URL.

## References

- #1417 - typed v3 public URL configuration
- #1415 - service binding identity
- #1980 - explicit v3 consumer migration
- EPIC #1978 - configuration overhaul
