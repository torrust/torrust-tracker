---
doc-type: issue
issue-type: feature
status: open
priority: p2
github-issue: 1987
spec-path: docs/issues/open/1987-add-config-option-to-use-ip-from-announce-query-string/ISSUE.md
branch: "1987-add-config-option-to-use-ip-from-announce-query-string"
related-pr: null
depends-on:
  - docs/issues/open/1985-rename-peer-addr-to-ip-in-http-announce-request/ISSUE.md
blocks: null
last-updated-utc: 2026-07-15 00:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/http-protocol/src/v1/requests/announce.rs
    - packages/http-core/src/services/announce.rs
    - packages/configuration/src/v2_0_0/
    - docs/issues/open/1978-configuration-overhaul-epic/EPIC.md
    - docs/issues/open/1640-1978-per-http-tracker-on-reverse-proxy-setting.md
    - evidence-opentracker-no-dns-support.md
    - evidence-chihaya-no-dns-support.md
---

# Issue #1987 - Add per-HTTP-tracker config option to use peer IP from `ip` GET parameter (sub-issue of #1978)

## Goal

Add an optional per-HTTP-tracker configuration setting that allows the tracker to use the IP address provided in the `ip` GET parameter of the announce request instead of always deriving the peer IP from the TCP connection. This feature is analogous to opentracker's `WANT_IP_FROM_QUERY_STRING` compile-time option.

## Background

### Current behaviour

The Torrust Tracker HTTP announce handler always derives the peer IP from the TCP connection (or from the `X-Forwarded-For` header when running behind a reverse proxy). The `ip` GET parameter — defined as optional in [BEP 3](https://www.bittorrent.org/beps/bep_0003.html) — is parsed but then **silently ignored**.

BEP 3 states:

> An optional parameter giving the IP (or dns name) which this peer is at. Generally used for the origin if it's on the same machine as the tracker.

The BEP's "generally used for the origin" note explains the primary use case: a peer that is on the same host as the tracker announces itself and wants the tracker to register a specific routable IP (rather than `127.0.0.1` from the loopback connection).

### Feature request

A user request was filed (see [torrust/torrust-tracker #163 comment](https://github.com/torrust/torrust-tracker/issues/163#issuecomment-1836642956)) asking for the ability to use the IP from the query string. This mirrors opentracker's `WANT_IP_FROM_QUERY_STRING` feature, which is enabled via a compile-time flag.

### Why it belongs to the configuration overhaul epic (#1978)

This feature requires adding a new per-HTTP-tracker configuration field. The configuration overhaul (schema v3.0.0) is the right time to introduce new per-tracker settings cleanly, rather than adding them to the existing `v2.0.0` schema that is already being overhauled. The related per-tracker `on_reverse_proxy` setting (#1640) is being introduced in the same epic.

### Prerequisites

This issue depends on the `ip` GET parameter rename (from `peer_addr` to `ip`) being completed first. The rename issue must be resolved before this feature is implemented.

### Interaction with `on_reverse_proxy`

When both `use_ip_from_query_string` and `on_reverse_proxy` are enabled, the query string `ip` takes precedence over the `X-Forwarded-For` header. This is because the operator explicitly opted into trusting the query string value. When `use_ip_from_query_string` is disabled (default), the existing `on_reverse_proxy` logic applies unchanged. The two settings are not mutually exclusive; the query string IP wins when both are active and a valid IP is provided.

### Security consideration

Enabling this feature allows a remote client to claim any IP address in its announce request. The tracker would accept that address and include it in the peer list. This is a potential source of IP spoofing in the peer list. The feature must therefore be **opt-in**, disabled by default, and clearly documented as a trust-based setting — suitable only for private/controlled deployments, or as a workaround for peers behind symmetric NAT that cannot be reached via their connection IP.

## Scope

### In Scope

- Add a new optional boolean configuration field to the per-HTTP-tracker configuration (name TBD during schema design, e.g. `use_ip_from_query_string`), disabled by default.
- When the option is enabled, and the `ip` GET parameter contains a valid IP address, use that IP as the peer's address instead of the connection IP.
- Document the security implications of enabling this option in the configuration schema and in the module documentation.
- Add contract tests covering both the enabled and disabled behaviour.

### Out of Scope

- DNS name resolution in the `ip` parameter (decided against in a separate ADR — see the rename issue).
- Changing the default behaviour (the tracker still uses the connection IP by default).
- Any changes to the UDP tracker protocol.
- Any changes to the scrape endpoint.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                          | Notes / Expected Output                                                                                                                                                                                                                                                                                                |
| --- | ------ | ------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | TODO   | Design the configuration field name and schema placement      | Align with #1978 schema v3.0.0 design; propose name (e.g. `use_ip_from_query_string`)                                                                                                                                                                                                                                  |
| T2  | TODO   | Add the field to the per-HTTP-tracker configuration struct    | Target the v3.0.0 schema under `packages/configuration/` as part of the #1978 overhaul                                                                                                                                                                                                                                 |
| T3  | TODO   | Thread the config value through to the announce service       | `packages/http-core/src/services/announce.rs` `peer_from_request`                                                                                                                                                                                                                                                      |
| T4  | TODO   | Implement the conditional IP selection in `peer_from_request` | Use `announce_request.ip` if `use_ip_from_query_string` is `true` and the field is `Some`; otherwise use the connection IP. When both `use_ip_from_query_string` and `on_reverse_proxy` are enabled, the query string IP takes precedence. Requires prerequisite issue (rename `peer_addr` → `ip`) to be merged first. |
| T5  | TODO   | Add contract tests for enabled and disabled behaviour         | New tests in `packages/axum-http-server/tests/`                                                                                                                                                                                                                                                                        |
| T6  | TODO   | Update configuration documentation                            | `packages/configuration/` docs and `share/default/` config file                                                                                                                                                                                                                                                        |
| T7  | TODO   | Run `cargo test --workspace` — no regressions                 | All tests pass                                                                                                                                                                                                                                                                                                         |
| T8  | TODO   | Run `linter all`                                              | Must exit `0`                                                                                                                                                                                                                                                                                                          |

## Progress Tracking

### Workflow Checkpoints

- [ ] Spec drafted in `docs/issues/drafts/`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] GitHub issue created and issue number added to this spec
- [ ] Prerequisites completed (rename `peer_addr` → `ip` issue resolved)
- [ ] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded (status + evidence)
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-07-15 00:00 UTC - Copilot/User - Spec drafted as a sub-issue of #1978; feature deferred to the configuration overhaul epic.

## Acceptance Criteria

- [ ] AC1: When `use_ip_from_query_string` is `false` (default), the tracker always uses the connection IP regardless of the `ip` GET parameter.
- [ ] AC2: When `use_ip_from_query_string` is `true` and a valid IP is provided in the `ip` GET parameter, the tracker uses that IP as the peer's address.
- [ ] AC3: When `use_ip_from_query_string` is `true` but the `ip` GET parameter is absent or contains a non-IP value, the tracker falls back to the connection IP.
- [ ] AC4: The default configuration file (`share/default/`) has `use_ip_from_query_string` set to `false` (or omitted, defaulting to `false`).
- [ ] AC5: The configuration schema documentation clearly states the security implications of enabling this option.
- [ ] AC6: Contract tests cover both enabled and disabled cases.
- [ ] AC7: `linter all` exits with code `0`.
- [ ] AC8: Relevant tests pass with no regressions.
- [ ] Manual verification scenarios are executed and documented (status + evidence).
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behaviour.
- [ ] Documentation is updated when behaviour/workflow changes.

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test --workspace`
- Pre-push checks (when applicable)

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                                            | Command/Steps                                                                                                            | Expected Result                                          | Status | Evidence |
| --- | --------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------ | -------------------------------------------------------- | ------ | -------- |
| M1  | Default config: `ip` GET param is ignored           | Start tracker with default config; announce with `ip=1.2.3.4` from a different source IP; check the peer list            | Peer is registered with the connection IP, not `1.2.3.4` | TODO   |          |
| M2  | Opt-in config: `ip` GET param is used               | Enable `use_ip_from_query_string`; announce with `ip=1.2.3.4`; check the peer list                                       | Peer is registered with `1.2.3.4`                        | TODO   |          |
| M3  | Opt-in config: no `ip` param — fallback             | Enable `use_ip_from_query_string`; announce without `ip` param                                                           | Peer is registered with the connection IP                | TODO   |          |
| M4  | Opt-in + reverse proxy: `ip` param takes precedence | Enable both `use_ip_from_query_string` and `on_reverse_proxy`; announce with `ip=1.2.3.4` and `X-Forwarded-For: 5.6.7.8` | Peer is registered with `1.2.3.4` (query string wins)    | TODO   |          |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | TODO                   |          |
| AC2   | TODO                   |          |
| AC3   | TODO                   |          |
| AC4   | TODO                   |          |
| AC5   | TODO                   |          |
| AC6   | TODO                   |          |
| AC7   | TODO                   |          |
| AC8   | TODO                   |          |

## Risks and Trade-offs

- **IP spoofing**: When enabled, a client can register any IP address in the peer list. This is inherent to the feature and must be clearly documented. The opt-in default mitigates the risk for deployments that do not need this.
- **Interaction with reverse proxy mode**: Resolved — when both `use_ip_from_query_string` and `on_reverse_proxy` are enabled, the query string `ip` takes precedence. See "Interaction with `on_reverse_proxy`" above for rationale.
- **IPv4/IPv6**: The `ip` parameter accepts both IPv4 and IPv6 addresses (via `IpAddr::from_str`). If the tracker is bound to an IPv6-only socket and a client sends an IPv4 `ip`, the address is accepted as-is — the tracker does not validate address family compatibility with the listener binding.

## References

- BEP 3 — The BitTorrent Protocol Specification: <https://www.bittorrent.org/beps/bep_0003.html>
- Feature request: <https://github.com/torrust/torrust-tracker/issues/163#issuecomment-1836642956>
- Parent epic: [#1978 — Configuration Overhaul](../1978-configuration-overhaul-epic/EPIC.md)
- Prerequisite issue: rename `peer_addr` → `ip` (to be linked once created)
- Related issue: [#1640 — Per-HTTP-tracker `on_reverse_proxy` setting](../1640-1978-per-http-tracker-on-reverse-proxy-setting.md)
- opentracker `WANT_IP_FROM_QUERY_STRING`: <https://erdgeist.org/arts/software/opentracker/>
- Research evidence — opentracker DNS name support: [evidence-opentracker-no-dns-support.md](evidence-opentracker-no-dns-support.md)
- Research evidence — chihaya DNS name support: [evidence-chihaya-no-dns-support.md](evidence-chihaya-no-dns-support.md)
