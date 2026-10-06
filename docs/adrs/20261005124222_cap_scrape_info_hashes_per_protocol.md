---
semantic-links:
  skill-links:
    - create-adr
  related-artifacts:
    - packages/udp-protocol/src/request.rs
    - packages/http-protocol/src/v1/requests/scrape.rs
    - packages/udp-server/src/handlers/mod.rs
    - packages/tracker-core/src/scrape_handler.rs
    - docs/issues/closed/2417-2411-verify-http-scrape-info-hash-limit/ISSUE.md
---

# Cap Scrape Info Hashes per Protocol, Each for Its Own Reason

- **Date**: 2026-10-05
- **Issue**: [#2417](https://github.com/torrust/torrust-tracker/issues/2417)
- **Spec**: [ISSUE.md](../issues/closed/2417-2411-verify-http-scrape-info-hash-limit/ISSUE.md)

## Scope

Root ADR: the decision spans `udp-protocol`, `http-protocol`, `udp-server`,
`axum-http-server`, and `tracker-core`.

## Context

`torrust_tracker_core::MAX_SCRAPE_TORRENTS` (`74`) said it applied to both the
UDP and the HTTP tracker "at the domain level". Only the UDP parser used it. The
HTTP tracker returned every requested info hash: a local probe returned 1000
entries for 1000 hashes. The original Warp HTTP tracker rejected scrapes above
the limit; the Axum rewrite dropped that check without a recorded decision, and
no test noticed.

The two protocols have different reasons for a limit:

- **UDP: transport constraint.** The server reads each datagram into a
  `MAX_PACKET_SIZE` (1496-byte) buffer. A scrape request has a 16-byte header
  (connection ID, action, transaction ID) followed by 20-byte info hashes, so
  at most `(1496 - 16) / 20 = 74` hashes fit. This is the "up to about 74
  torrents" of [BEP 15](https://www.bittorrent.org/beps/bep_0015.html).
- **HTTP: no transport constraint.** The response is a TCP stream, and the
  request URL has no protocol-level length limit relevant at this scale. Any
  HTTP cap is a policy choice whose only reason is to bound the work one
  request can cause.

## Decision

1. **Each protocol parser caps the number of scrape info hashes, and keeps the
   first N in request order, silently ignoring the rest** (truncate, like UDP).
   No log or metric records the truncation.
2. **UDP: `torrust_tracker_udp_protocol::MAX_SCRAPE_INFO_HASHES`**, computed in
   code from `MAX_PACKET_SIZE`, the scrape request header, and the info-hash
   size (74 today). The reason lives in the expression.
3. **HTTP: `torrust_tracker_http_protocol::v1::requests::scrape::MAX_SCRAPE_INFO_HASHES = 100`**,
   a standalone abuse-mitigation policy value. It is neither derived from the
   UDP limit nor from the persistence batch size; that 100 hashes fit one
   persisted-downloads query
   ([ADR 20261002173716](20261002173716_load_persisted_scrape_downloads_with_a_batched_uncached_lookup.md))
   is a consequence, not a coupling.
4. **The HTTP cap counts `info_hash` parameters as sent.** Duplicates count,
   like UDP's raw 20-byte slots, and parsing stops at the cap, so values past it
   are not decoded or validated.
5. **`tracker-core` has no cap.** `ScrapeHandler` returns an entry for every
   info hash it receives; limits belong to the protocol that has the reason for
   them. `torrust_tracker_core::MAX_SCRAPE_TORRENTS` is removed.
6. **Tests pin each limit with literal counts** (74/75, 100/101), not with the
   constants, so changing a limit fails the tests and leads back here.

A per-request cap is **not** abuse protection on its own: a client can send many
capped requests in parallel and cause the same total work. It bounds the cost
and response size of one request. Total-load protection is rate limiting,
tracked in the spam and abuse EPIC
([#2411](../issues/open/2411-spam-and-abuse-resistance/EPIC.md)).

## Considered Alternatives

| Option | Behavior | Why not chosen |
| --- | --- | --- |
| **A - Truncate (chosen)** | Keep the first N, ignore the rest | Silent, but matches UDP and keeps clients that send too many hashes working |
| **B - Reject** | Bencoded failure above N | Breaks clients that send more than N today; the previous Warp behavior was not restored because it was never a recorded decision |
| **C - Unlimited HTTP** | Fix only the documentation | Leaves the per-request cost unbounded |
| **Shared constant** | One value for both protocols | Changing one protocol's limit would silently change the other, and the UDP reason does not apply to HTTP |
| **74 for HTTP** | Same value on both protocols | The UDP origin is not a reason for HTTP |
| **Literal 74 for UDP** | Keep a hand-written value | The value is a consequence of the packet size; a literal hides that |

## Consequences

- **Positive**: The HTTP documentation matches the behavior, and each limit's
  reason is stated once, next to the value that carries it.
- **Positive**: HTTP per-request scrape cost and response size are bounded.
- **Negative**: HTTP clients sending more than 100 hashes receive fewer entries
  without being told. They must split large scrapes.
- **Negative**: Removing `torrust_tracker_core::MAX_SCRAPE_TORRENTS` is a public
  API change, accepted for `3.0.0-develop`.
- **Note**: UDP truncates twice. The receive buffer already drops hashes past
  the 74th for a datagram arriving on a socket; the parser cap also protects
  callers that parse payloads directly. A `handle_packet` unit test covers the
  parser wiring, because a socket-level test cannot distinguish the two.

## Affected Code

- [`packages/udp-protocol/src/request.rs`](../../packages/udp-protocol/src/request.rs): UDP constant and parser cap
- [`packages/http-protocol/src/v1/requests/scrape.rs`](../../packages/http-protocol/src/v1/requests/scrape.rs): HTTP constant and parser cap
- [`packages/udp-server/src/handlers/mod.rs`](../../packages/udp-server/src/handlers/mod.rs): passes the UDP cap to the parser
- [`packages/tracker-core/src/scrape_handler.rs`](../../packages/tracker-core/src/scrape_handler.rs): uncapped domain handler
