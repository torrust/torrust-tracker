---
doc-type: issue
issue-type: enhancement
status: open
priority: p2
github-issue: 1640
spec-path: docs/issues/open/1640-1978-per-http-tracker-on-reverse-proxy-setting.md
branch: "1640-move-network-to-per-instance-config"
related-pr: null
last-updated-utc: 2026-06-23 18:30
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

> **EPIC position**: Subissue #3 of 9. Depends on #2 (tsl→tls typo fix). Must be implemented before #1417 (public_url) and #1490 (secrets) — both reference the `Network` block established here. Both #1640 and #1490 touch `Core`, so #1640 goes first.

## Goal

Give each tracker instance (`HttpTracker` and `UdpTracker`) its own `Network` config block containing `external_ip`, `on_reverse_proxy`, and `ipv6_v6only`. Remove the shared `[core.net]` section and make the domain-layer `AnnounceHandler` accept `external_ip` as a per-call parameter.

**End state**: Every tracker instance has its own networking config — socket behaviour, proxy awareness, and peer-IP replacement are all per-instance concerns. The shared `Core` only holds truly cross-cutting settings (database, policy, private mode).

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

[http_trackers.net]
external_ip = "203.0.113.5"
on_reverse_proxy = true
ipv6_v6only = false

[[udp_trackers]]
bind_address = "0.0.0.0:6969"

[udp_trackers.net]
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
      "net": {
        "external_ip": "203.0.113.5",
        "on_reverse_proxy": true,
        "ipv6_v6only": false
      }
    }
  ],
  "udp_trackers": [
    {
      "bind_address": "0.0.0.0:6969",
      "net": {
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
    pub tsl_config: Option<TslConfig>,
    pub tracker_usage_statistics: bool,
    pub net: Network,                    // ← replaces individual fields
    // ipv6_v6only REMOVED — now inside net
}

// Server-layer config for each UDP tracker
pub struct UdpTracker {
    pub bind_address: SocketAddr,
    pub cookie_lifetime: Duration,
    pub tracker_usage_statistics: bool,
    pub max_connection_id_errors_per_ip: u32,
    pub net: Network,                    // ← replaces individual fields
    // ipv6_v6only REMOVED — now inside net
}

// Core — no longer has a net field
pub struct Core {
    pub announce_policy: AnnouncePolicy,
    pub database: Database,
    pub inactive_peer_cleanup_interval: u64,
    pub listed: bool,
    // net: Network REMOVED
    pub private: bool,
    pub private_mode: Option<PrivateMode>,
    pub tracker_policy: TrackerPolicy,
    pub tracker_usage_statistics: bool,
}
```

### Design Note: `bind_address` stays flat (not inside `net`)

We considered moving `bind_address` into `Network` since it is a networking concern. We decided to keep it flat for two reasons:

1. **Primary key role**: `bind_address` is the HashMap key for tracker instance containers in `AppContainer` (`HashMap<SocketAddr, Arc<HttpTrackerCoreContainer>>`). Nesting it inside `net` would make lookup more cumbersome without benefit.
2. **TLS asymmetry**: `tsl_config` (TLS certificate paths) cannot go into `Network`. Keeping `bind_address` and `tsl_config` at the same level while `on_reverse_proxy`, `external_ip`, and `ipv6_v6only` group into `net` creates a cleaner boundary between _socket binding_ (flat) and _socket behaviour / network identity_ (grouped).

### Compatibility with Existing ADRs

| ADR                                               | Impact                                                                                                                                                                                                  | Status                                                                                             |
| ------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------- |
| `20260617093046` (reject wildcard `external_ip`)  | `ExternalIp` newtype unchanged. `external_ip` moves location (from `core.net` to `http_trackers[].net`). The `Network` struct with its `ExternalIp` field stays in `network.rs` as a shared definition. | ✅ Compatible. ADR says "no schema change" — needs updating since this issue changes the location. |
| `20260620000000` (add `ipv6_v6only` option)       | Field moves from flat `HttpTracker.ipv6_v6only` / `UdpTracker.ipv6_v6only` to `HttpTracker.net.ipv6_v6only` / `UdpTracker.net.ipv6_v6only`. Default (`false`) and behaviour unchanged.                  | ✅ Compatible. ADR needs updating to reflect new field path.                                       |
| `20260527175600` (keep protocol/domain decoupled) | Not directly related — this issue touches configuration types and service-layer code, not protocol types.                                                                                               | ✅ No impact.                                                                                      |

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

[http_trackers.net]
external_ip = "203.0.113.5"
on_reverse_proxy = true
ipv6_v6only = false
```

The old `[core.net]` section is no longer valid. Each tracker instance must have its own `net` block. The `external_ip` and `on_reverse_proxy` values must be moved into each `[[http_trackers]].net` (and/or `[[udp_trackers]].net`) block. Leaving them out means `false` / `None` defaults apply.

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
> - `tsl_config` on `HttpTracker` — the tracker terminates TLS **directly** (clients connect via HTTPS directly to the tracker). No proxy involved.
> - `use_tls_proxy` in the deployer — TLS is terminated at a **reverse proxy** (Caddy, nginx) before forwarding plain HTTP to the tracker.
>
> Both are orthogonal to `on_reverse_proxy` (trusting `X-Forwarded-For` headers). You can have:
>
> - Direct HTTPS tracker (`tsl_config` set) with or without trusting proxy headers
> - Tracker behind a TLS proxy (`use_tls_proxy`) with `on_reverse_proxy = true` (common case)
> - Tracker behind a plain HTTP proxy (no TLS) with `on_reverse_proxy = true`
> - Tracker directly exposed via plain HTTP without any proxy
>
> This issue only addresses `on_reverse_proxy`; TLS configuration remains a separate concern.

### Related Issue: #1417 — Public Service URL (implemented in this EPIC)

Issue [#1417](https://github.com/torrust/torrust-tracker/issues/1417) adds an optional `public_url: Option<String>` field to each tracker instance (`HttpTracker`, `UdpTracker`) and API service (`HttpApi`, `HealthCheckApi`). This field is **implemented in this EPIC** (not a future extension) but lives as a **flat field** on each config struct — **not inside `Network`**.

**Why flat, not inside `Network`**: The `Network` block groups **network topology** concerns (how the tracker connects: external IP, proxy awareness, socket behaviour). `public_url` is about **public exposure** (how users reach the tracker). It's a different axis — one tracker instance might have both a `net.on_reverse_proxy` setting and a `public_url`, and they are independently configurable.

```toml
[[http_trackers]]
bind_address = "0.0.0.0:7070"
public_url = "https://tracker.torrust-demo.com/announce"

[http_trackers.net]
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
    pub tsl_config: Option<TslConfig>,            // direct TLS (tracker terminates)

    // Instance metadata
    pub tracker_usage_statistics: bool,

    // Public exposure — how users reach this tracker
    pub public_url: Option<String>,               // ‡ #1417 — full URL (e.g. "https://tracker1.example.com/announce")

    // Network topology (grouped)
    pub net: Network,                              // † new
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
    pub net: Network,                              // † new
}

/// Core — no longer has any networking config.
pub struct Core {
    pub announce_policy: AnnouncePolicy,
    pub database: Database,
    pub inactive_peer_cleanup_interval: u64,
    pub listed: bool,
    // net: Network REMOVED                         †
    pub private: bool,
    pub private_mode: Option<PrivateMode>,
    pub tracker_policy: TrackerPolicy,
    pub tracker_usage_statistics: bool,
}
```

**Rationale for keeping `public_url` flat (not inside `Network`)**:

The `Network` block groups **network topology** concerns — how the tracker instance connects to the network (external IP, proxy awareness, socket behaviour). `public_url` is about **public exposure** — how users reach the tracker. These are different axes:

- A tracker behind a reverse proxy might have `net.on_reverse_proxy = true` and `public_url = "https://tracker.example.com/announce"`
- A directly-exposed tracker might have `net.on_reverse_proxy = false` and `public_url = "http://tracker.example.com:7070/announce"`
- Both fields are independently configurable; nesting one inside the other would be misleading

The `AnnounceHandler` in `tracker-core` stops reading from `self.config.net.external_ip` and instead accepts it as a parameter:

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

- Add `Network` (with `external_ip`, `on_reverse_proxy`, `ipv6_v6only`) as per-instance field in both `HttpTracker` and `UdpTracker`
- Remove `Network` from `Core` (remove `core.net` entirely)
- Modify `AnnounceHandler::handle_announcement()` to accept `external_ip` per-call instead of reading from global config
- Update all callers of `handle_announcement()` (HTTP services, UDP services, tests) to pass per-instance `external_ip`
- Update all consumers of `ipv6_v6only` to read from `HttpTracker.net` / `UdpTracker.net` instead of flat struct fields
- Remove deprecated flat `ipv6_v6only` fields from `HttpTracker` and `UdpTracker`
- Update test helpers, default config TOML files, integration tests, docs, and doc comments
- Write ADR for the architecture decision

### Out of Scope

- TOML config migration tooling

## Approach B — Per-instance services (chosen)

For the `on_reverse_proxy` threading, we use **Approach B** (as analysed earlier): each `HttpTrackerCoreContainer` creates per-instance `AnnounceService` and `ScrapeService` storing their own `ReverseProxyMode`. This avoids extending Axum state tuples and keeps handler signatures stable. The full analysis is preserved below in the appendix.

## Implementation Strategy: Baby Steps + Parallel Changes + Draft PR

**Key principles**:

1. **Baby steps**: Each commit is a small, verifiable change
2. **Parallel changes**: Introduce new code paths alongside old ones before deleting the old ones
3. **Draft PR**: Open early and keep updated after each commit, running CI checks continuously

### Phase 0 — ADR

Write the Architectural Decision Record documenting:

- Why `Network` moves from global `core.net` to per-instance configs
- Why `external_ip` becomes a parameter of `handle_announcement()`
- Why `ipv6_v6only` joins `Network`

### Phase 1 — Add `net: Network` to `HttpTracker` and `UdpTracker` (parallel add)

Add the new `net: Network` field to both tracker config structs. Keep the old fields (`core.net`, flat `ipv6_v6only`) for now. `Network` gains `ipv6_v6only`.

Default for `Network`:

```rust
Network {
    external_ip: None,
    on_reverse_proxy: false,
    ipv6_v6only: false,
}
```

**Verification**: Config deserialization still works with both old and new formats. All existing tests pass unchanged.

### Phase 2 — Modify `AnnounceHandler::handle_announcement()` to accept `external_ip`

Add `tracker_external_ip: Option<IpAddr>` parameter to `handle_announcement()`. The callers temporarily pass `self.config.net.external_ip.map(Into::into)` (still reading from the old global for now).

**Verification**: All `handle_announcement()` call sites compile. No behaviour change.

### Phase 3 — Switch consumers to the new per-instance configs

This is the largest phase, split into sub-tasks (each committed and CI-verified independently):

#### 3a. `on_reverse_proxy`

- `test-helpers`: Set per-tracker `on_reverse_proxy` in `HttpTracker` instead of `core.net`
- `http-core/services/announce.rs` + `scrape.rs`: Read from per-instance `ReverseProxyMode` (Approach B)
- `HttpTrackerCoreServices` + `HttpTrackerCoreContainer`: Create per-instance services
- `src/container.rs`: Flow per-instance mode through `AppContainer`
- Unit/integration tests: Update all references to per-tracker

#### 3b. `ipv6_v6only`

- `HttpTracker` consumers (`server.rs`, `environment.rs`, `bootstrap/jobs/http_tracker.rs`, contract tests): Read from `http_tracker_config.net.ipv6_v6only`
- `UdpTracker` consumers (`launcher.rs`, contract tests): Read from `udp_tracker_config.net.ipv6_v6only`

#### 3c. `external_ip`

- `udp-server` tests: Pass per-tracker `external_ip` to `handle_announcement()` (now available from `udp_tracker_config.net.external_ip`)
- `http-core` tests: Pass per-tracker `external_ip` to `handle_announcement()` (now available from `http_tracker_config.net.external_ip`)
- `axum-http-server` contract tests: Same

### Phase 4 — Remove deprecated fields

- Delete `core.net` from `Core` struct. Keep `network.rs` with both `Network` and `ExternalIp` — both `HttpTracker` and `UdpTracker` import `Network` from there (single definition, no duplication).
- Delete flat `ipv6_v6only` fields from `HttpTracker` and `UdpTracker`
- Delete `get_ext_ip()` from `Configuration` (no longer needed — each instance has its own `external_ip`)
- Update default TOML files to use the new format
- Update all doc comments and crate-level docs
- Update `docs/containers.md`

### Phase 5 — Final verification

- `linter all`
- Full test suite
- Manual verification of mixed proxy/non-proxy scenarios
- Close the draft PR and open the final PR

## Implementation Plan

**Chosen approach**: **Approach B** (per-instance services with `reverse_proxy_mode` field) for `on_reverse_proxy` threading.

| ID  | Phase | Status | Task                                                                      | Notes                                                                                                 |
| --- | ----- | ------ | ------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------- |
| T0  | 0     | TODO   | Write ADR                                                                 | Record: move `Network` to per-instance, parameterize `external_ip`, join `ipv6_v6only` into `Network` |
| T1  | 1     | TODO   | Add `net: Network` (with `ipv6_v6only`) to `HttpTracker` and `UdpTracker` | Parallel add — old fields kept. Default `ipv6_v6only = false`                                         |
| T2  | 2     | TODO   | Add `tracker_external_ip` param to `handle_announcement()`                | Callers pass old global value temporarily                                                             |
| T3a | 3a    | TODO   | Switch `on_reverse_proxy` consumers to per-instance                       | Approach B: per-instance services with `ReverseProxyMode`                                             |
| T3b | 3b    | TODO   | Switch `ipv6_v6only` consumers to `net.ipv6_v6only`                       | HTTP + UDP server launchers, tests                                                                    |
| T3c | 3c    | TODO   | Switch `external_ip` consumers                                            | All callers of `handle_announcement()` pass per-instance value                                        |
| T4  | 4     | TODO   | Remove deprecated fields                                                  | `core.net`, flat `ipv6_v6only`, `get_ext_ip()`                                                        |
| T5  | 4     | TODO   | Update default config TOML files                                          | 6 files in `share/default/config/`                                                                    |
| T6  | 4     | TODO   | Update docs and doc comments                                              | `mod.rs`, `lib.rs`, `containers.md`, `tracker-core/lib.rs`, `extractors/client_ip_sources.rs`         |
| T7  | 5     | TODO   | Run `linter all` and full test suite                                      |                                                                                                       |

## Progress Tracking

### Workflow Checkpoints

- [ ] Spec drafted in `docs/issues/open/`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] Phase 0: ADR created
- [ ] Phase 1: `net: Network` added to `HttpTracker` and `UdpTracker` (parallel with old fields)
- [ ] Phase 2: `handle_announcement()` accepts `tracker_external_ip` param
- [ ] Phase 3a: `on_reverse_proxy` consumers switched to per-instance
- [ ] Phase 3b: `ipv6_v6only` consumers switched to `net.ipv6_v6only`
- [ ] Phase 3c: `external_ip` consumers switched to per-instance
- [ ] Phase 4: Deprecated fields removed (`core.net`, flat `ipv6_v6only`)
- [ ] Phase 5: Final verification completed (`linter all`, full test suite)
- [ ] Manual verification scenarios executed and recorded (status + evidence)
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/closed/`

### Progress Log

Append one line per meaningful update.

- 2026-06-23 00:00 UTC - Copilot - Spec drafted from issue #1640
- 2026-06-23 14:00 UTC - Copilot - Added design decision analysis (Approach A vs B) after maintainer review
- 2026-06-23 14:30 UTC - Copilot - Updated spec: remove global `[core.net].on_reverse_proxy`, move to per-tracker `HttpTracker.on_reverse_proxy: bool`. Added ADR task T1.
- 2026-06-23 16:00 UTC - Copilot - Rewrote spec with full architectural vision: per-instance `Network` for all three fields, phased implementation with baby steps + draft PR.
- 2026-06-23 17:45 UTC - Copilot - Added design note on `bind_address` staying flat, future extensions section (`domain`, `use_tls_proxy`, `public_url`) referencing deployer and issue #1417.
- 2026-06-23 18:30 UTC - Copilot - Completed deep review against ADRs 20260617093046, 20260620000000, 20260527175600 and issues #1417, #1671. Added compatibility table and migration note.
- 2026-07-14 00:00 UTC - josecelano - Resolved #1417 relationship: `public_url` is in this EPIC (not future), stays flat (not inside `Network`). Replaced "Future Extensions" section with "Related Issue: #1417" section. Updated config types to show `public_url` as `‡` field. Added versioning note (app 4.0.0, config schema 3.0.0).

## Acceptance Criteria

- [ ] AC1: `on_reverse_proxy` is removed from `[core.net]` and placed per-instance in `HttpTracker.net.on_reverse_proxy` (and `UdpTracker.net.on_reverse_proxy` for future UDP proxy use)
- [ ] AC2: `external_ip` is removed from `[core.net]` and placed per-instance in `HttpTracker.net.external_ip` and `UdpTracker.net.external_ip`
- [ ] AC3: `ipv6_v6only` is moved from flat `HttpTracker.ipv6_v6only` and `UdpTracker.ipv6_v6only` into `HttpTracker.net` / `UdpTracker.net`
- [ ] AC4: `Core.net` (the `Network` struct) is removed from `Core`
- [ ] AC5: `AnnounceHandler::handle_announcement()` accepts `tracker_external_ip` per-call instead of reading from global config
- [ ] AC6: Two HTTP trackers with different `on_reverse_proxy` settings behave independently: - Tracker A (`on_reverse_proxy = true`) reads `X-Forwarded-For` headers - Tracker B (`on_reverse_proxy = false` or unset) reads connection info IP
- [ ] AC7: Example `http_only_public_tracker.rs` builds with the new `HttpTracker.net.on_reverse_proxy` field
- [ ] AC8: All default config files and docs use the new format
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented (status + evidence)
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
