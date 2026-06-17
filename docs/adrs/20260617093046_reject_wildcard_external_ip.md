---
semantic-links:
  skill-links:
    - create-adr
  related-artifacts:
    - issue #1507
    - packages/tracker-core/src/announce_handler.rs
    - packages/configuration/src/v2_0_0/network.rs
    - packages/configuration/src/v2_0_0/core.rs
    - packages/configuration/src/validator.rs
---

# Reject wildcard IPs as invalid `external_ip` values

## Description

Reject wildcard/unspecified addresses (`0.0.0.0`, `::`) in the `core.net.external_ip`
configuration option at startup, and change the default value from `Some(0.0.0.0)` to `None`.

## Context

The `external_ip` config option tells the tracker what external/public IP to assign to
loopback-address clients (peers that announce from `127.0.0.1` or `::1`). The tracker assumes
those peers are on the same machine (or LAN behind the same NAT) and replaces their loopback
address with the tracker's external IP so remote peers can contact them.

### The problem

The default value for `external_ip` is `Some(Ipv4Addr::UNSPECIFIED)` — `0.0.0.0`. This address
is the wildcard / "not bound to any specific interface" address (RFC 1122). It is never a
valid external IP.

When a peer announces from a loopback address and `external_ip` is `0.0.0.0`:

1. `assign_ip_address_to_peer` sees the client IP is loopback
2. It replaces it with the configured `external_ip` → `0.0.0.0`
3. Other peers receive `0.0.0.0` as the announcing peer's address — useless

### Why this needs a decision not just a code fix

There are two possible approaches:

| Approach | What                                                                         | Pros                                       | Cons                                                        |
| -------- | ---------------------------------------------------------------------------- | ------------------------------------------ | ----------------------------------------------------------- |
| **A**    | Silently treat `0.0.0.0`/`::` as "not configured" (fall back to original IP) | Backward compatible                        | Still silently accepts invalid config; operator never knows |
| **B**    | Reject `0.0.0.0`/`::` at config validation, default to `None`                | Fail fast, clear error, explicit semantics | Breaking change for anyone relying on the old default       |

Without a new major version, Approach A would be the pragmatic choice. Since a new major
version is approaching, Approach B is clearly better — fail fast is the correct engineering
response to invalid configuration.

### Production impact

| Deployment scenario                    | external_ip         | Before fix                           | After fix                                    |
| -------------------------------------- | ------------------- | ------------------------------------ | -------------------------------------------- |
| Dev (client + tracker on same machine) | default (`0.0.0.0`) | Peers get `0.0.0.0` ✗                | Default is `None` → peers keep `127.0.0.1` ✓ |
| Production (separate machines)         | `None`              | Not possible (default was `0.0.0.0`) | Peers keep their real IP ✓                   |
| Production (LAN clients)               | `203.0.113.5`       | Works fine ✓                         | Works fine ✓                                 |
| Production (unconfigured)              | default             | Silent bug → `0.0.0.0` peers ✗       | `None` → peers keep real IP ✓                |

## Agreement

Reject wildcard addresses as invalid `external_ip` values and change the default to `None`.

### Consequences

- **Positive**: Fail fast — operators who explicitly set `external_ip = "0.0.0.0"` get a clear
  parse-time error from the `ExternalIp` newtype, not silent runtime bugs.
- **Positive**: The `None` default is semantically correct — `external_ip` is truly
  optional and only needed when LAN/loopback clients share the tracker's public IP.
- **Positive**: In the common case (production deployments without LAN clients),
  no configuration is needed and behavior is correct.
- **Positive**: No startup error for unset `external_ip` — leaving it unset is valid
  and means "no loopback replacement".
- **Negative**: Breaking change — operators who explicitly set `external_ip = "0.0.0.0"`
  will get a parse-time error and must either set a valid IP or remove the value.
  This is acceptable because a new major version is upcoming.

### What changes

1. **Default value**: `Network::default_external_ip()` returns `None` instead of `Some(0.0.0.0)`
2. **New `ExternalIp` newtype**: Replaces `Option<IpAddr>` with `Option<ExternalIp>` in
   the config field. The newtype rejects unspecified addresses (`0.0.0.0`, `::`) at
   construction/parse time via `TryFrom<IpAddr>`, `FromStr`, and custom `Deserialize`.
3. **Config validation simplified**: No `UnspecifiedExternalIp` variant needed — the
   constraint is enforced at the type level, which is consistent with the philosophy
   that the `Validator` trait is for cross-field invariants.
4. **Function `assign_ip_address_to_peer`**: Added defense-in-depth guard: even if an
   unspecified address somehow reaches the function, it falls back to the original IP.

### What does NOT change

- **Config schema version**: remains `2.0.0`. No fields are added, removed, or renamed; no types
  change. The change is behavioral (rejecting previously-accepted invalid values) but the config
  file format is backward-compatible.
- **Config file structure**: TOML sections and field names stay identical.
- **Default config files**: remain unchanged (they don't specify `external_ip` explicitly, so
  the serde default will now be `None` instead of `0.0.0.0`).

### Breaking change classification

This is a **behavioral breaking change** (operators who explicitly set `external_ip = "0.0.0.0"`
will get a startup validation error), not a **config schema breaking change**. The config file
format stays compatible across the tracker's major version bump.

## Date

2026-06-17

## References

- [Issue #1507](https://github.com/torrust/torrust-tracker/issues/1507) — Original bug report
- [Issue spec](../../docs/issues/open/1507-review-localhost-peer-ip.md) — Implementation specification
