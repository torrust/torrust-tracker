---
doc-type: issue
issue-type: enhancement
status: draft
priority: p3
github-issue: 1417
spec-path: docs/issues/drafts/1417-add-public-service-url-to-configuration.md
branch: "1417-add-public-service-url"
related-pr: null
last-updated-utc: 2026-06-23 18:45
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/open/1640-move-network-to-per-instance-config.md
    - github torrust/torrust-tracker-deployer
    - github torrust/torrust-tracker-deployer docs/ai-training/dataset/environment-configs/02-full-stack-lxd.json
    - packages/configuration/src/v2_0_0/http_tracker.rs
    - packages/configuration/src/v2_0_0/udp_tracker.rs
---

<!-- skill-link: create-issue -->

# Issue #1417 - Include public service URL in configuration

## Goal

Add an optional `public_url` field to each tracker instance (`HttpTracker`, `UdpTracker`) and API service (`HttpApi`, `HealthCheckApi`) so the application knows the public-facing URL for each service regardless of network topology, reverse proxies, or TLS termination.

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

- Add optional `public_url: Option<String>` field to `HttpTracker`, `UdpTracker`, `HttpApi`, and `HealthCheckApi`
- Use a **single URL string** (e.g. `"https://tracker1.example.com/announce"`) — not decomposed into domain/path components, since consumers can parse those as needed
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

**Where the field lives**: This field is about **public exposure** (how users reach the service), not **network topology** (how the service connects). It could stay flat on the config struct or join a `Network` block depending on future architecture decisions. For now, keep it flat — issue #1640 establishes the `Network` block, and this field can be placed accordingly.

## Implementation Plan

| ID  | Status | Task                                                        | Notes          |
| --- | ------ | ----------------------------------------------------------- | -------------- |
| T1  | TODO   | Add `public_url: Option<String>` to `HttpTracker` config    | Default `None` |
| T2  | TODO   | Add `public_url: Option<String>` to `UdpTracker` config     | Default `None` |
| T3  | TODO   | Add `public_url: Option<String>` to `HttpApi` config        | Default `None` |
| T4  | TODO   | Add `public_url: Option<String>` to `HealthCheckApi` config | Default `None` |
| T5  | TODO   | Document field in default config examples and crate docs    |                |
| T6  | TODO   | Run `linter all` and tests                                  |                |

## Progress Tracking

### Progress Log

- 2026-06-23 18:45 UTC - Copilot - Drafted from GitHub issue #1417 and discussions in issue #1640 spec review.

## Acceptance Criteria

- [ ] AC1: All config structs gain `public_url: Option<String>` field
- [ ] AC2: Default config examples include the new field (commented or shown)
- [ ] AC3: No runtime behaviour change — field is present for consumer use
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
