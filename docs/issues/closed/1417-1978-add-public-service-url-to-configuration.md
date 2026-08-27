---
doc-type: issue
issue-type: enhancement
status: done
priority: p3
github-issue: 1417
spec-path: docs/issues/closed/1417-1978-add-public-service-url-to-configuration.md
branch: "1417-add-public-service-url"
related-pr: null
last-updated-utc: 2026-07-22 00:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - issue #1640
    - issue torrust/torrust-tracker-deployer
    - issue torrust/torrust-tracker-deployer docs/ai-training/dataset/environment-configs/02-full-stack-lxd.json
    - packages/configuration/src/v3_0_0/http_tracker.rs
    - packages/configuration/src/v3_0_0/udp_tracker.rs
    - packages/configuration/src/v3_0_0/tracker_api.rs
    - packages/configuration/src/v3_0_0/health_check_api.rs
---

# Issue #1417 - Include public service URL in configuration

> **EPIC position**: Subissue #4 of 11. Depends on #1640 (subissue #3) for the `Network` block placement decision — `public_url` stays flat (not inside `Network`). Implements after #1640 is complete.

## Goal

Add an optional `public_url` field to each tracker instance (`HttpTracker`, `UdpTracker`) and API service (`HttpApi`) so the application knows the public-facing URL for each service regardless of network topology, reverse proxies, or TLS termination. `HealthCheckApi` is a minimal liveness endpoint and does not get a `public_url` field; it gains only `#[serde(deny_unknown_fields)]` for consistency.

## Background

The tracker configuration only specifies the **bind address** (the local IP:port where the service listens):

```toml
[[http_trackers]]
bind_address = "0.0.0.0:7070"
```

The application has no way to know the **public URL** clients use to reach each service. This matters when:

- The tracker runs behind a reverse proxy (Caddy, nginx) with TLS termination
- Multiple tracker instances share the same IP but serve different domains
- Metrics should be broken down by public URL, domain, or protocol

For example, the [Torrust Tracker Deployer](https://github.com/torrust/torrust-tracker-deployer) already defines per-tracker `domain` and `use_tls_proxy` fields in its environment configs ([example](https://github.com/torrust/torrust-tracker-deployer/blob/main/docs/ai-training/dataset/environment-configs/02-full-stack-lxd.json)), but these are deployer-internal and not propagated to the tracker itself.

### Use cases

1. **Metrics labels**: Prometheus metrics could include a `public_url` label to separate data per domain or protocol.
2. **Logging**: Log entries could record which public URL served a request.
3. **API discovery**: The health check endpoint could advertise service URLs.
4. **Notifications**: Service notifications could reference the correct public URL.

## Scope

### In Scope

- Add optional typed `public_url` fields: `Option<HttpUrl>` to `HttpTracker` and `HttpApi`, `Option<UdpUrl>` to `UdpTracker`; `HealthCheckApi` does not get a `public_url` field
- Use a **single URL string** (e.g. `"https://tracker1.example.com/announce"`) — not decomposed into domain/path components, since consumers can parse those as needed
- Validate URL protocol at deserialization time (HTTP tracker → `http://`/`https://`, UDP tracker → `udp://`, API → `http://`/`https://`)
- The URL protocol (`https://`) provides TLS status; the domain is extracted by consumers
- Document the field in default config examples
- No runtime behaviour change — the field is stored in config and available for use by consumers (metrics, logging, etc.)

### Out of Scope

- Adding runtime support for the URL in metrics/logging/API (separate issues)
- URL validation beyond basic format checks
- Changing the deployer's internal config format

### Follow-up: Metrics Labels

A follow-up issue would use the `public_url` field to add new labels to Prometheus metrics. The **domain** (parsed from the URL) is the most useful label, since:

- Protocol is already captured in existing metrics labels.
- The full URL would duplicate the information already available via the bind address socket label (each tracker instance has a unique bind address, so `url` and `bind_address` would always be 1:1).
- A `domain` label, on the other hand, enables aggregation across tracker instances sharing the same domain behind different ports or protocols.

No changes are needed in this issue — the field just needs to be present in the config for consumers to use.

## Design Decisions

**Single URL string vs decomposed fields**: The field is a single URL string. Consumers parse protocol, domain, and path as needed. This is the simplest user-facing form and avoids duplicating the deployer's `domain` + `use_tls_proxy` approach.

**Where the field lives**: `public_url` is a **flat field** on `HttpTracker`, `UdpTracker`, and `HttpApi` — **not inside the `Network` block** and **not on `HealthCheckApi`**. The `Network` block (established by #1640) groups **network topology** concerns (external IP, proxy awareness, socket behaviour). `public_url` is about **public exposure** (how users reach the service) — a different axis. `HealthCheckApi` is a minimal liveness endpoint; exposing a `public_url` there has no use-case in scope. A tracker instance can independently configure both `net.on_reverse_proxy` and `public_url`.

**URL validation implementation**: Use typed newtypes (`HttpUrl`, `UdpUrl`) defined in `v3_0_0/public_url.rs`. Each newtype wraps a `url::Url` (already a dependency), validates the scheme at construction, and implements `Serialize`/`Deserialize` directly — no `#[serde(deserialize_with = ...)]` attribute is needed on the struct field. The invariant is encoded in the type and never re-checked in consumers. See [ADR 20260721100000](../../adrs/20260721100000_use_newtypes_for_constrained_configuration_field_types.md) for the full rationale and the `HttpUrl`/`UdpUrl` granularity decision.

**`deny_unknown_fields`**: `HttpApi` and `HealthCheckApi` currently lack `#[serde(deny_unknown_fields)]` which all other v3 config structs have. Add it to both as part of this issue for consistency — we are already touching both structs.

**Protocol validation**: The URL protocol is validated at deserialization time:

- HTTP tracker: must use `http://` or `https://`
- UDP tracker: must use `udp://`
- HTTP API / Health Check API: must use `http://` or `https://`

This catches misconfigurations early (e.g., accidentally setting `public_url = "udp://..."` on an HTTP tracker).

## Implementation Plan

| ID  | Status | Task                                                                  | Notes                                                                                   |
| --- | ------ | --------------------------------------------------------------------- | --------------------------------------------------------------------------------------- |
| T0  | DONE   | Create `v3_0_0/public_url.rs` with `HttpUrl` and `UdpUrl` newtypes    | `url` crate; each newtype validates its scheme in its own `Deserialize` impl            |
| T1  | DONE   | Add `public_url: Option<HttpUrl>` to `HttpTracker` config             | Default `None`; scheme validated by `HttpUrl`                                           |
| T2  | DONE   | Add `public_url: Option<UdpUrl>` to `UdpTracker` config               | Default `None`; scheme validated by `UdpUrl`                                            |
| T3  | DONE   | Add `public_url: Option<HttpUrl>` to `HttpApi` config                 | Default `None`; also add `deny_unknown_fields`                                          |
| T4  | DONE   | Add `#[serde(deny_unknown_fields)]` to `HealthCheckApi`               | No `public_url` on this struct; consistency-only change                                 |
| T5  | DONE   | Document field in crate-level docs and doc comments                   | Default config migration is deferred to #1980                                           |
| T6  | DONE   | Run `linter all` and tests                                            |                                                                                         |
| T7  | DONE   | Update migration guide if this subissue affects the config public API | `packages/configuration/docs/migrate-v2-to-v3.md` |

## Progress Tracking

### Progress Log

- 2026-07-22 00:00 UTC - agent - Verified issue #1417 is CLOSED on GitHub and archived this spec to docs/issues/closed/.

- 2026-06-23 18:45 UTC - Copilot - Drafted from GitHub issue #1417 and discussions in issue #1640 spec review.
- 2026-07-14 00:00 UTC - josecelano - Resolved placement: `public_url` stays flat (not inside `Network`). Added protocol validation. Updated related-artifacts to v3 paths.
- 2026-07-21 12:00 UTC - agent - Started as next EPIC subissue (#4 of 11); #1640 schema slice merged (PR #2014) satisfying the dependency.
- 2026-07-21 16:00 UTC - agent - Implementation complete. All 7 tasks done. Pre-commit gate passes. Additional decisions recorded: used `HttpUrl`/`UdpUrl` typed newtypes instead of `Option<String>` (see ADR 20260721100000); added field-type convention notice to all v3 config modules; created `packages/configuration/AGENTS.md`; added `unvalidated` to project dictionary.

## Acceptance Criteria

- [x] AC1: `HttpTracker`, `UdpTracker`, and `HttpApi` gain `public_url: Option<HttpUrl>` / `Option<UdpUrl>` fields (typed newtypes, not raw `String`); `HealthCheckApi` does not
- [x] AC2: Protocol validation rejects mismatched protocols at deserialization time using the `url` crate (e.g., `udp://` on an HTTP tracker fails with a descriptive error)
- [x] AC3: Protocol validation also rejects structurally malformed URLs (parse error from `url` crate)
- [x] AC4: `HttpApi` and `HealthCheckApi` gain `#[serde(deny_unknown_fields)]` for consistency
- [x] AC5: No runtime behaviour change — field is present for consumer use, default is `None`
- [x] `linter all` exits with code `0`
- [x] Relevant tests pass
