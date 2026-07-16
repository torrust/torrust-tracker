# ADR: Accept only IP addresses (not DNS names) in the HTTP announce `ip` GET parameter

- **Date**: 2026-07-16
- **Status**: Accepted
- **Issue**: [#1985](https://github.com/torrust/torrust-tracker/issues/1985)
- **Spec**: `docs/issues/open/1985-rename-peer-addr-to-ip-in-http-announce-request/ISSUE.md`

## Context

[BEP 3](https://www.bittorrent.org/beps/bep_0003.html) defines the `ip` announce parameter as:

> An optional parameter giving the IP (or dns name) which this peer is at. Generally used
> for the origin if it's on the same machine as the tracker.

The current implementation parses the `ip` GET parameter by calling `IpAddr::from_str`. Any value
that is not a valid IP address (including DNS names) is silently dropped — the field is set to
`None` and the tracker falls back to using the connection IP.

A policy decision is needed: should the tracker support DNS names, resolve them, or explicitly
restrict the parameter to IP addresses only?

## Decision

**Accept only IP addresses in the HTTP announce `ip` GET parameter.**

Non-IP values (including DNS names) are silently ignored; the tracker falls back to the connection
IP. The restriction is documented in the module doc-comments.

## Considered Alternatives

| Approach                           | What                                                                                       | Pros                                                                             | Cons                                                                                                                                               |
| ---------------------------------- | ------------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------- |
| **A — IP only (this decision)**    | Accept only valid `IpAddr` values; silently ignore non-IP values; document the restriction | Simple, predictable, no latency, no DoS risk, consistent with all major trackers | Deviates from the literal BEP 3 spec text                                                                                                          |
| **B — Resolve DNS names**          | Accept DNS names and resolve them to IPs at announce time                                  | Closer to BEP 3 literal wording                                                  | Latency per announce, DoS amplification risk (attacker-controlled DNS lookups), complexity, no known client sends hostnames                        |
| **C — Accept and store hostnames** | Parse and store hostnames as strings alongside IPs                                         | Closest to BEP 3 literal wording                                                 | Incompatible with the `IpAddr`-based peer list model; no client or tracker implements this; no BEP defines how hostnames are returned in responses |

## Evidence from major trackers

- **opentracker**: accepts only IP addresses in `ip`. Has a separate compile-time feature flag
  (`WANT_IP_FROM_QUERY_STRING`) to optionally use the `ip` value for the peer's address; the type
  accepted is always an IP address.
- **chihaya**: accepts only IP addresses in `ip`.
- **No known tracker** supports DNS name resolution in the announce `ip` parameter.

## Consequences

- **Positive**: No latency impact on announce handling.
- **Positive**: No DNS-based DoS attack surface.
- **Positive**: Consistent with opentracker, chihaya, and all other known tracker implementations.
- **Positive**: The `IpAddr`-based peer list model is preserved without changes.
- **Negative**: Deviates from the literal BEP 3 spec text ("or dns name"). Mitigated by clear
  documentation and the fact that no known client sends a hostname in this field.

A future issue may choose to return an explicit parse error for non-IP values (e.g. DNS names)
instead of silently ignoring them. Clients MUST NOT send hostnames in the `ip` field when
communicating with Torrust Tracker.
