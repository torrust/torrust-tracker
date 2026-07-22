---
doc-type: issue
issue-type: enhancement
status: done
priority: p2
github-issue: 1640
spec-path: docs/issues/closed/1640-1978-per-http-tracker-on-reverse-proxy-setting.md
branch: "1640-move-network-to-per-instance-config"
related-pr: null
last-updated-utc: 2026-07-22 00:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/adrs/20260617093046_reject_wildcard_external_ip.md
    - issue #1417
    - packages/configuration/src/v3_0_0/http_tracker.rs
    - packages/configuration/src/v3_0_0/udp_tracker.rs
    - packages/configuration/src/v3_0_0/network.rs
    - packages/configuration/src/v3_0_0/core.rs
    - packages/tracker-core/src/announce_handler.rs
    - packages/tracker-core/src/lib.rs
    - packages/http-core/src/container.rs
    - packages/http-core/src/services/announce.rs
    - packages/http-core/src/services/scrape.rs
    - packages/http-core/benches/helpers/sync.rs
    - packages/http-protocol/src/v1/services/peer_ip_resolver.rs
    - packages/axum-http-server/src/v1/routes.rs
    - packages/axum-http-server/src/v1/handlers/announce.rs
    - packages/axum-http-server/src/v1/handlers/scrape.rs
    - packages/axum-http-server/src/server.rs
    - packages/axum-http-server/src/testing/environment.rs
    - packages/axum-http-server/examples/http_only_public_tracker.rs
    - packages/axum-rest-api-server/src/testing/environment.rs
    - packages/udp-core/src/services/announce.rs
    - packages/udp-server/src/server/launcher.rs
    - packages/udp-server/src/handlers/announce.rs
    - packages/udp-server/src/handlers/mod.rs
    - packages/test-helpers/src/configuration.rs
    - src/container.rs
    - src/bootstrap/jobs/http_tracker.rs
    - src/lib.rs
    - share/default/config/
    - docs/containers.md
---

# Issue #1640 - Move `on_reverse_proxy` to per-tracker config (and relocate `Network`)

> **EPIC position**: Subissue #3 of 11. Depends on #2 (`tsl` → `tls` typo fix). Must be implemented before #1417 (public_url) and #1490 (secrets) — both reference the `Network` block established here. Both #1640 and #1490 touch `Core`, so #1640 goes first.

## Goal

Give each tracker instance (`HttpTracker` and `UdpTracker`) its own `Network` config block containing `external_ip`, `on_reverse_proxy`, and `ipv6_v6only`. Remove the shared `[core.net]` section and make the domain-layer `AnnounceHandler` accept `external_ip` as a per-call parameter.

**End state**: Every tracker instance has its own networking config — socket behaviour, proxy awareness, and peer-IP replacement are all per-instance concerns. The shared `Core` only holds truly cross-cutting settings (database, policy, private mode).

### Schema Compatibility Boundary

This issue changes **only schema `v3.0.0`**. Schema `v2.0.0` remains unchanged in its
separate module for compatibility, but `v3_0_0` must exclusively use the per-instance
`network` fields. It must not deserialize, fall back to, or define precedence for the
removed `[core.net]` section or the removed flat `ipv6_v6only` fields.

The application-wide migration from v2 configuration types to v3 configuration types is
the responsibility of EPIC subissue #1980. Once that migration is complete, production
code will use only the v3 per-instance `network` values. No runtime compatibility bridge
between the v2 and v3 field layouts is required or permitted.

## Background

The issue was originally opened to allow per-HTTP-tracker `on_reverse_proxy` settings. During analysis we discovered a broader architectural problem: the entire `Network` struct (`external_ip`, `on_reverse_proxy`, `ipv6_v6only`) lived in `[core.net]` as a **global singleton** shared by all tracker instances. This caused three separate issues:

| Current field      | Currently in                                | Problem                                                  |
| ------------------ | ------------------------------------------- | -------------------------------------------------------- |
| `on_reverse_proxy` | `core.net` (global)                         | HTTP proxy config shouldn't be global — servers differ   |
| `external_ip`      | `core.net` (global)                         | Each tracker instance may have its own public IP         |
| `ipv6_v6only`      | `HttpTracker` / `UdpTracker` (per-instance) | Correct placement, but field is duplicated in both types |

**Final design**: `Network` becomes a per-instance struct placed inside `HttpTracker` and `UdpTracker`:

```toml
# BEFORE: Global shared config
[core.net]
external_ip = "203.0.113.5"
on_reverse_proxy = true

[[http_trackers]]
bind_address = "0.0.0.0:7070"
ipv6_v6only = false          # field directly in HttpTracker

[[udp_trackers]]
bind_address = "0.0.0.0:6969"
ipv6_v6only = true           # field directly in UdpTracker

# AFTER: Per-instance networking config
[[http_trackers]]
bind_address = "0.0.0.0:7070"

[http_trackers.network]
external_ip = "203.0.113.5"
on_reverse_proxy = true
ipv6_v6only = false

[[udp_trackers]]
bind_address = "0.0.0.0:6969"

[udp_trackers.network]
external_ip = "2001:db8::1"
on_reverse_proxy = false
ipv6_v6only = true
```

The JSON form makes the per-instance structure clearer:

```json
{
  "http_trackers": [
    {
      "bind_address": "0.0.0.0:7070",
      "network": {
        "external_ip": "203.0.113.5",
        "on_reverse_proxy": true,
        "ipv6_v6only": false
      }
    }
  ],
  "udp_trackers": [
    {
      "bind_address": "0.0.0.0:6969",
      "network": {
        "external_ip": "2001:db8::1",
        "on_reverse_proxy": false,
        "ipv6_v6only": true
      }
    }
  ]
}
```

### Why `external_ip` moves too

The `external_ip` is consumed by `AnnounceHandler::handle_announcement()` in `tracker-core`. It replaces loopback IPs with the tracker's public IP. If you have two tracker instances on different network interfaces with different public IPs, they need different `external_ip` values. The current global setting cannot express that.

Making `external_ip` per-instance requires passing it as a parameter to `handle_announcement()` instead of having the handler read it from `self.config` — this is architecturally correct: the handler shouldn't know about the server's network topology.

### Why `ipv6_v6only` moves into `Network`

`ipv6_v6only` controls how the OS socket handles IPv4-mapped IPv6 addresses. It is a **networking concern**, not a tracker-protocol concern. Grouping it with `external_ip` and `on_reverse_proxy` inside a per-instance `Network` block is more coherent than having it as a flat field in `HttpTracker`/`UdpTracker`.

## Final Architecture

```rust
// Per-instance network config — placed inside HttpTracker and UdpTracker
pub struct Network {
    pub external_ip: Option<ExternalIp>,
    pub on_reverse_proxy: bool,
    pub ipv6_v6only: bool,
}

// Server-layer config for each HTTP tracker
pub struct HttpTracker {
    pub bind_address: SocketAddr,
    pub tls_config: Option<TlsConfig>,
    pub tracker_usage_statistics: bool,
    pub network: Network,                // ← replaces individual fields
    // ipv6_v6only REMOVED — now inside network
}

// Server-layer config for each UDP tracker
pub struct UdpTracker {
    pub bind_address: SocketAddr,
    pub cookie_lifetime: Duration,
    pub tracker_usage_statistics: bool,
    pub max_connection_id_errors_per_ip: u32,
    pub network: Network,                // ← replaces individual fields
    // ipv6_v6only REMOVED — now inside network
}

// Core — no longer has a network field
pub struct Core {
    pub announce_policy: AnnouncePolicy,
    pub database: Database,
    pub inactive_peer_cleanup_interval: u64,
    pub listed: bool,
    // network: Network REMOVED
    pub private: bool,
    pub private_mode: Option<PrivateMode>,
    pub tracker_policy: TrackerPolicy,
    pub tracker_usage_statistics: bool,
}
```

### Design Note: `bind_address` stays flat (not inside `network`)

We considered moving `bind_address` into `Network` since it is a networking concern. We decided to keep it flat for two reasons:

1. **Primary key role**: `bind_address` is the HashMap key for tracker instance containers in `AppContainer` (`HashMap<SocketAddr, Arc<HttpTrackerCoreContainer>>`). Nesting it inside `network` would make lookup more cumbersome without benefit.
2. **TLS asymmetry**: `tls_config` (TLS certificate paths) cannot go into `Network`. Keeping `bind_address` and `tls_config` at the same level while `on_reverse_proxy`, `external_ip`, and `ipv6_v6only` group into `network` creates a cleaner boundary between _socket binding_ (flat) and _socket behaviour / network identity_ (grouped).

### Compatibility with Existing ADRs

| ADR                                               | Impact                                                                                                                                                                                                      | Status                                                                                             |
| ------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------- |
| `20260617093046` (reject wildcard `external_ip`)  | `ExternalIp` newtype unchanged. `external_ip` moves location (from `core.net` to `http_trackers[].network`). The `Network` struct with its `ExternalIp` field stays in `network.rs` as a shared definition. | ✅ Compatible. ADR says "no schema change" — needs updating since this issue changes the location. |
| `20260620000000` (add `ipv6_v6only` option)       | Field moves from flat `HttpTracker.ipv6_v6only` / `UdpTracker.ipv6_v6only` to `HttpTracker.network.ipv6_v6only` / `UdpTracker.network.ipv6_v6only`. Default (`false`) and behaviour unchanged.              | ✅ Compatible. ADR needs updating to reflect new field path.                                       |
| `20260527175600` (keep protocol/domain decoupled) | Not directly related — this issue touches configuration types and service-layer code, not protocol types.                                                                                                   | ✅ No impact.                                                                                      |

### User-Facing Migration Note

This is a **breaking configuration change**. Users upgrading to the new tracker version (4.0.0) must update their `tracker.toml`:

> **Note on versioning**: The tracker application and the configuration schema use independent version systems. The tracker app goes from 3.0.0 → 4.0.0, while the config schema goes from 2.0.0 → 3.0.0. This allows them to evolve independently — the configuration crate can also be used partially in other projects.

**Before:**

```toml
[core.net]
external_ip = "203.0.113.5"
on_reverse_proxy = true

[[http_trackers]]
bind_address = "0.0.0.0:7070"
ipv6_v6only = false
```

**After:**

```toml
[[http_trackers]]
bind_address = "0.0.0.0:7070"

[http_trackers.network]
external_ip = "203.0.113.5"
on_reverse_proxy = true
ipv6_v6only = false
```

The old `[core.net]` section is no longer valid. Each tracker instance has its own `Network` configuration. The TOML `network` block is optional and defaults to `external_ip = None`, `on_reverse_proxy = false`, and `ipv6_v6only = false` when omitted. The `external_ip` and `on_reverse_proxy` values must be moved into each configured `[[http_trackers]].network` (and/or `[[udp_trackers]].network`) block.

### Future Extensions (not implemented in this issue)

The per-instance `Network` block is a natural home for additional per-tracker networking fields in future issues. Relevant candidates from related work:

#### From the [Torrust Tracker Deployer](https://github.com/torrust/torrust-tracker-deployer)

The deployer's environment configs (e.g. [02-full-stack-lxd.json](https://github.com/torrust/torrust-tracker-deployer/blob/main/docs/ai-training/dataset/environment-configs/02-full-stack-lxd.json)) already include per-tracker metadata that the tracker configuration does not yet support:

```json
{
  "http_trackers": [
    {
      "bind_address": "0.0.0.0:7070",
      "domain": "tracker1.example.com",
      "use_tls_proxy": true
    },
    {
      "bind_address": "0.0.0.0:7071",
      "domain": "tracker2.example.com",
      "use_tls_proxy": true
    }
  ]
}
```

These fields (`domain`, `use_tls_proxy`) describe how each tracker instance is exposed to the public internet — a networking concern that fits naturally into per-instance config.

> **Note on TLS vs reverse proxy**: There are two independent TLS configurations:
>
> - `tls_config` on `HttpTracker` — the tracker terminates TLS **directly** (clients connect via HTTPS directly to the tracker). No proxy involved.
> - `use_tls_proxy` in the deployer — TLS is terminated at a **reverse proxy** (Caddy, nginx) before forwarding plain HTTP to the tracker.
>
> Both are orthogonal to `on_reverse_proxy` (trusting `X-Forwarded-For` headers). You can have:
>
> - Direct HTTPS tracker (`tls_config` set) with or without trusting proxy headers
> - Tracker behind a TLS proxy (`use_tls_proxy`) with `on_reverse_proxy = true` (common case)
> - Tracker behind a plain HTTP proxy (no TLS) with `on_reverse_proxy = true`
> - Tracker directly exposed via plain HTTP without any proxy
>
> This issue only addresses `on_reverse_proxy`; TLS configuration remains a separate concern.

### Related Issue: #1417 — Public Service URL (implemented in this EPIC)

Issue [#1417](https://github.com/torrust/torrust-tracker/issues/1417) adds an optional `public_url: Option<String>` field to each tracker instance (`HttpTracker`, `UdpTracker`) and API service (`HttpApi`, `HealthCheckApi`). This field is **implemented in this EPIC** (not a future extension) but lives as a **flat field** on each config struct — **not inside `Network`**.

**Why flat, not inside `Network`**: The `Network` block groups **network topology** concerns (how the tracker connects: external IP, proxy awareness, socket behaviour). `public_url` is about **public exposure** (how users reach the tracker). It's a different axis — one tracker instance might have both a `network.on_reverse_proxy` setting and a `public_url`, and they are independently configurable.

```toml
[[http_trackers]]
bind_address = "0.0.0.0:7070"
public_url = "https://tracker.torrust-demo.com/announce"

[http_trackers.network]
external_ip = "203.0.113.5"
on_reverse_proxy = true
ipv6_v6only = false
```

**Design decision (July 2026)**: The field is a full URL string (`"https://tracker1.example.com/announce"`). The URL protocol is validated: HTTP trackers must use `http://` or `https://`, UDP trackers must use `udp://`. This is simpler than decomposed fields (domain + path) and consumers can parse the URL as needed. The full URL also subsumes the deployer's `domain` + `use_tls_proxy` approach — the protocol tells us if TLS is used, and the domain is extracted from the URL.

### Full config types (this issue + #1417)

Below is how the full types would look after this issue's changes plus #1417 (`public_url`). Fields marked `†` are implemented in this issue; fields marked `‡` are implemented in #1417.

```rust
/// Per-instance network topology config.
/// Grouped because these fields together define how the tracker instance
/// connects to the network — the external identity, proxy awareness, and
/// socket behaviour.
pub struct Network {                              // † this issue
    pub external_ip: Option<ExternalIp>,          // † from core.net
    pub on_reverse_proxy: bool,                   // † from core.net
    pub ipv6_v6only: bool,                        // † from flat field
}

/// Server-layer config for each HTTP tracker.
pub struct HttpTracker {
    // Socket binding — how the OS binds the listener
    pub bind_address: SocketAddr,
    pub tls_config: Option<TlsConfig>,            // direct TLS (tracker terminates)

    // Instance metadata
    pub tracker_usage_statistics: bool,

    // Public exposure — how users reach this tracker
    pub public_url: Option<String>,               // ‡ #1417 — full URL (e.g. "https://tracker1.example.com/announce")

    // Network topology (grouped)
    pub network: Network,                          // † new
}

/// Server-layer config for each UDP tracker.
pub struct UdpTracker {
    pub bind_address: SocketAddr,
    pub cookie_lifetime: Duration,
    pub tracker_usage_statistics: bool,
    pub max_connection_id_errors_per_ip: u32,

    // Public exposure — how users reach this tracker
    pub public_url: Option<String>,               // ‡ #1417 — full URL (e.g. "udp://tracker1.example.com:6969")

    // Network topology (grouped)
    pub network: Network,                          // † new
}

/// Core — no longer has any networking config.
pub struct Core {
    pub announce_policy: AnnouncePolicy,
    pub database: Database,
    pub inactive_peer_cleanup_interval: u64,
    pub listed: bool,
    // network: Network REMOVED                     †
    pub private: bool,
    pub private_mode: Option<PrivateMode>,
    pub tracker_policy: TrackerPolicy,
    pub tracker_usage_statistics: bool,
}
```

**Rationale for keeping `public_url` flat (not inside `Network`)**:

The `Network` block groups **network topology** concerns — how the tracker instance connects to the network (external IP, proxy awareness, socket behaviour). `public_url` is about **public exposure** — how users reach the tracker. These are different axes:

- A tracker behind a reverse proxy might have `network.on_reverse_proxy = true` and `public_url = "https://tracker.example.com/announce"`
- A directly-exposed tracker might have `network.on_reverse_proxy = false` and `public_url = "http://tracker.example.com:7070/announce"`
- Both fields are independently configurable; nesting one inside the other would be misleading

The `AnnounceHandler` in `tracker-core` stops reading the global configuration's `external_ip` and instead accepts it as a parameter:

```rust
pub async fn handle_announcement(
    &self,
    info_hash: &InfoHash,
    peer: &mut peer::Peer,
    remote_client_ip: &IpAddr,
    peers_wanted: &PeersWanted,
    tracker_external_ip: Option<IpAddr>,  // NEW: passed in from caller
) -> Result<AnnounceData, AnnounceError> {
    ...
    peer.change_ip(&assign_ip_address_to_peer(remote_client_ip, tracker_external_ip));
    ...
}
```

## Scope

### In Scope (all phases)

- Add `network: Network` (with `external_ip`, `on_reverse_proxy`, `ipv6_v6only`) as an optional-in-TOML, per-instance field in both `HttpTracker` and `UdpTracker`
- Remove `Network` from `Core` (remove `core.net` entirely)
- Modify `AnnounceHandler::handle_announcement()` to accept `external_ip` per-call instead of reading from global config
- Update all callers of `handle_announcement()` (HTTP services, UDP services, tests) to pass per-instance `external_ip`
- Update all consumers of `ipv6_v6only` to read from `HttpTracker.network` / `UdpTracker.network` instead of flat struct fields
- Remove deprecated flat `ipv6_v6only` fields from `HttpTracker` and `UdpTracker`
- Update v3 configuration tests, docs, and doc comments
- Write ADR for the architecture decision

### Out of Scope

- TOML config migration tooling
- Migrating application consumers, test helpers, or default configuration files from schema v2 to v3 (subissue #1980)
- Supporting removed v2 fields in schema v3 or defining old-versus-new field precedence

## Approach B — Per-instance services (chosen)

For the `on_reverse_proxy` threading, we use **Approach B** (as analysed earlier): each `HttpTrackerCoreContainer` creates per-instance `AnnounceService` and `ScrapeService` storing their own `ReverseProxyMode`. This avoids extending Axum state tuples and keeps handler signatures stable. The full analysis is preserved below in the appendix.

## Implementation Strategy

### Phase 0 — ADR

Write the Architectural Decision Record documenting:

- Why `Network` moves from global `core.net` to per-instance configs
- Why `external_ip` becomes a parameter of `handle_announcement()`
- Why `ipv6_v6only` joins `Network`

### Phase 1 — Define the v3 per-instance `Network`

Add the new `network: Network` field to both tracker config structs. Remove `core.net` and the
flat `ipv6_v6only` fields from v3 at the same time. `Network` gains `ipv6_v6only`. The TOML
block is optional and deserializes to the safe defaults below when omitted.

Default for `Network`:

```rust
Network {
    external_ip: None,
    on_reverse_proxy: false,
    ipv6_v6only: false,
}
```

**Verification**: V3 configuration deserializes with an omitted `network` block and rejects the
removed v2 field layout. Schema v2 tests remain unchanged.

### Phase 2 — Modify `AnnounceHandler::handle_announcement()` to accept `external_ip`

Add `tracker_external_ip: Option<IpAddr>` parameter to `handle_announcement()`. V3 consumers
pass their instance's `network.external_ip`; no caller reads `core.net`.

**Verification**: All `handle_announcement()` call sites compile. No behaviour change.

### Phase 3 — Switch consumers to the new per-instance configs

This is the largest phase, split into sub-tasks (each committed and CI-verified independently):

#### 3a. `on_reverse_proxy`

- `test-helpers`: Set per-tracker `on_reverse_proxy` in `HttpTracker.network` instead of `core.net`
- `http-core/services/announce.rs` + `scrape.rs`: Read from per-instance `ReverseProxyMode` (Approach B)
- `HttpTrackerCoreServices` + `HttpTrackerCoreContainer`: Create per-instance services
- `src/container.rs`: Flow per-instance mode through `AppContainer`
- Unit/integration tests: Update all references to per-tracker

#### 3b. `ipv6_v6only`

- `HttpTracker` consumers (`server.rs`, `environment.rs`, `bootstrap/jobs/http_tracker.rs`, contract tests): Read from `http_tracker_config.network.ipv6_v6only`
- `UdpTracker` consumers (`launcher.rs`, contract tests): Read from `udp_tracker_config.network.ipv6_v6only`

#### 3c. `external_ip`

- `udp-server` tests: Pass per-tracker `external_ip` to `handle_announcement()` (now available from `udp_tracker_config.network.external_ip`)
- `http-core` tests: Pass per-tracker `external_ip` to `handle_announcement()` (now available from `http_tracker_config.network.external_ip`)
- `axum-http-server` contract tests: Same

### Phase 4 — Complete the v3 schema boundary

- Delete `core.net` from `Core` struct. Keep `network.rs` with both `Network` and `ExternalIp` — both `HttpTracker` and `UdpTracker` import `Network` from there (single definition, no duplication).
- Delete flat `ipv6_v6only` fields from `HttpTracker` and `UdpTracker`
- Delete `get_ext_ip()` from `Configuration` (no longer needed — each instance has its own `external_ip`)
- Update v3 doc comments and crate-level docs

### Phase 5 — Final verification

- `linter all`
- Full test suite
- Manual verification of mixed proxy/non-proxy scenarios
- Close the draft PR and open the final PR

## Implementation Plan

**Chosen approach**: **Approach B** (per-instance services with `reverse_proxy_mode` field) for `on_reverse_proxy` threading.

| ID  | Phase | Status   | Task                                                                                | Notes                                                                                             |
| --- | ----- | -------- | ----------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------- |
| T0  | 0     | DONE     | Write ADR                                                                           | `20260721000000_make_network_configuration_per_tracker_instance.md`                               |
| T1  | 1     | DONE     | Define v3 `network: Network` (with `ipv6_v6only`) in `HttpTracker` and `UdpTracker` | Removed v2 fields are rejected in v3; TOML block defaults safely when omitted                     |
| T2  | 2     | DEFERRED | Add `tracker_external_ip` param to `handle_announcement()`                          | Requires active runtime consumers to migrate to v3 in #1980                                       |
| T3a | 3a    | DEFERRED | Switch `on_reverse_proxy` consumers to per-instance                                 | Requires active runtime consumers to migrate to v3 in #1980                                       |
| T3b | 3b    | DEFERRED | Switch `ipv6_v6only` consumers to `network.ipv6_v6only`                             | Requires active runtime consumers to migrate to v3 in #1980                                       |
| T3c | 3c    | DEFERRED | Switch `external_ip` consumers                                                      | Requires active runtime consumers to migrate to v3 in #1980                                       |
| T4  | 4     | DONE     | Remove deprecated fields from v3                                                    | Removed `core.net`, flat `ipv6_v6only`, and `get_ext_ip()`                                        |
| T5  | 4     | DONE     | Update v3 documentation and doc comments                                            | V3 configuration module, ADR, and issue specification                                             |
| T7  | 5     | PARTIAL  | Run `linter all` and full test suite                                                | `linter all` and `cargo test -p torrust-tracker-configuration` pass; full suite deferred to #1980 |

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted in `docs/issues/open/`
- [ ] Spec reviewed and approved by user/maintainer
- [x] Phase 0: ADR created
- [x] Phase 1: v3 `network: Network` replaces `core.net` and flat `ipv6_v6only`
- [ ] Phase 2: `handle_announcement()` accepts `tracker_external_ip` param
- [ ] Phase 3a: `on_reverse_proxy` consumers switched to per-instance
- [ ] Phase 3b: `ipv6_v6only` consumers switched to `network.ipv6_v6only`
- [ ] Phase 3c: `external_ip` consumers switched to per-instance
- [x] Phase 4: V3 schema boundary complete (`core.net`, flat `ipv6_v6only`, `get_ext_ip()` removed)
- [ ] Phase 5: Final verification completed (`linter all`, full test suite)
- [ ] Manual verification scenarios executed and recorded (status + evidence)
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Committer verified spec progress is up to date before commit
- [x] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-07-22 00:00 UTC - agent - Verified issue #1640 is CLOSED on GitHub and archived this spec to docs/issues/closed/.

Append one line per meaningful update.

- 2026-06-23 00:00 UTC - Copilot - Spec drafted from issue #1640
- 2026-06-23 14:00 UTC - Copilot - Added design decision analysis (Approach A vs B) after maintainer review
- 2026-06-23 14:30 UTC - Copilot - Updated spec: remove global `[core.net].on_reverse_proxy`, move to per-tracker `HttpTracker.on_reverse_proxy: bool`. Added ADR task T1.
- 2026-06-23 16:00 UTC - Copilot - Rewrote spec with full architectural vision: per-instance `Network` for all three fields, phased implementation with baby steps + draft PR.
- 2026-06-23 17:45 UTC - Copilot - Added design note on `bind_address` staying flat, future extensions section (`domain`, `use_tls_proxy`, `public_url`) referencing deployer and issue #1417.
- 2026-06-23 18:30 UTC - Copilot - Completed deep review against ADRs 20260617093046, 20260620000000, 20260527175600 and issues #1417, #1671. Added compatibility table and migration note.
- 2026-07-14 00:00 UTC - josecelano - Resolved #1417 relationship: `public_url` is in this EPIC (not future), stays flat (not inside `Network`). Replaced "Future Extensions" section with "Related Issue: #1417" section. Updated config types to show `public_url` as `‡` field. Added versioning note (app 4.0.0, config schema 3.0.0).
- 2026-07-21 00:00 UTC - josecelano - Confirmed `network` as the per-instance field name, aligned with the `Network` type. Confirmed the TOML `[*.network]` block is optional and defaults to `external_ip = None`, `on_reverse_proxy = false`, and `ipv6_v6only = false`.
- 2026-07-21 00:00 UTC - josecelano - Confirmed the schema compatibility boundary: v3 accepts only per-instance `network` fields and has no fallback or precedence for removed v2 fields. Application migration to v3 remains subissue #1980.
- 2026-07-21 00:00 UTC - agent - Implemented the v3 schema slice: per-tracker `network` defaults, removed v3 global and flat fields, strict old-layout rejection tests, and ADR. Active runtime consumers remain on v2 and are deferred to #1980.
- 2026-07-21 12:00 UTC - agent - Marked DONE: PR #2014 merged; v3 schema slice is in `develop`. Runtime consumer tasks (T2–T3c: `handle_announcement` param, `on_reverse_proxy`/`ipv6_v6only`/`external_ip` consumer switch) are tracked under subissue #11 (#1980).

## Acceptance Criteria

- [x] AC1: `on_reverse_proxy` is removed from `[core.net]` and placed per-instance in `HttpTracker.network.on_reverse_proxy` (and `UdpTracker.network.on_reverse_proxy` for future UDP proxy use)
- [x] AC2: `external_ip` is removed from `[core.net]` and placed per-instance in `HttpTracker.network.external_ip` and `UdpTracker.network.external_ip`
- [x] AC3: `ipv6_v6only` is moved from flat `HttpTracker.ipv6_v6only` and `UdpTracker.ipv6_v6only` into `HttpTracker.network` / `UdpTracker.network`
- [x] AC4: `Core.net` (the `Network` struct) is removed from `Core`
- [ ] AC5: `AnnounceHandler::handle_announcement()` accepts `tracker_external_ip` per-call instead of reading from global config
- [ ] AC6: Two HTTP trackers with different `on_reverse_proxy` settings behave independently: - Tracker A (`on_reverse_proxy = true`) reads `X-Forwarded-For` headers - Tracker B (`on_reverse_proxy = false` or unset) reads connection info IP
- [ ] AC7: Example `http_only_public_tracker.rs` builds with the new `HttpTracker.network.on_reverse_proxy` field
- [x] AC8: V3 configuration documentation uses the new format; active application default configuration migration is deferred to #1980
- [x] AC9: Schema v3 rejects `[core.net]` and flat tracker `ipv6_v6only` fields; it does not define old-versus-new precedence
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented (status + evidence)
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
