---
semantic-links:
  skill-links:
    - create-adr
  related-artifacts:
    - packages/axum-server/src/custom_axum_server.rs
    - docs/issues/open/2776-2411-axum-server-0-9-timeouts/ISSUE.md
    - .github/skills/dev/planning/create-adr/SKILL.md
---

<!-- skill-link: create-adr -->

# Close idle connections without a 408 response

## Scope

Package-local. The behaviour belongs to the listener constructors in `packages/axum-server`
(`custom_axum_server`), which the HTTP tracker, REST API and health check API packages only call.
It is not shared configuration or an inter-package contract, so the ADR lives in
`packages/axum-server/docs/adrs/`. A future decision to make the timeouts configurable would be
repository-level and should reference this one.

## Description

A connection that is opened and never sends a request holds a Tokio task and a file descriptor for
as long as the client keeps it open. Issue #2776 made every Axum listener (plaintext and TLS, HTTP
tracker, REST API and health check API) close such connections with `axum-server` 0.9's
`Server::first_request_timeout`, 5 s by default.

When that timeout fires, `axum-server` returns from the connection task and drops the socket. The
client sees a plain close and no HTTP response. Some frameworks answer `408 Request Timeout` first
(Actix Web does, as the reproduction repository `josecelano/axum-server-timeout` shows). The
question is whether the tracker should do the same.

## Agreement

The tracker closes the connection silently and does not send `408 Request Timeout` for a
connection that has not sent a complete first request, an idle keep-alive connection, or a TLS
handshake that does not complete.

Reasons:

- A connection that sent nothing has no pending request to answer, so a bare close is valid HTTP
  (RFC 9110 allows a server to send `408` but does not require it).
- It is the cheapest response under abuse: no write, no response buffer, and no bytes sent to a
  client that already proved it will not send a request.
- It needs no custom code. A `408` would need a wrapper around the connection for plaintext, TLS
  and HTTP/2, like the `TimeoutAcceptor` removed in #2776, or a new upstream feature.
- The tracker's clients are BitTorrent clients that make short announce and scrape requests. A
  silent connection is almost certainly a scanner or an abuse attempt, not a client that needs an
  explanation.

A request that has arrived and whose handler is too slow still gets `408`, from the per-request
`TimeoutLayer` of the HTTP tracker and the REST API. That is a different timeout.

### Alternatives Considered

- **Send `408` then close.** Rejected: more server work in the situation that should be cheapest,
  custom code for each transport, and the client's next write still fails with a broken pipe, as in
  the Actix Web reproduction.

### Consequences

- Positive: no extra code to maintain; no response traffic for idle connections.
- Negative: clients and proxies see a reset or close instead of a `408`, and a reverse proxy may log
  an upstream close. This is a known limitation.
- Revisit this decision if real users report confusing logs behind a proxy. The preferred route is
  an optional `408` in `axum-server` upstream, not custom tracker code.

## Date

2026-11-11

## References

- Issue [#2776](https://github.com/torrust/torrust-tracker/issues/2776) and its spec,
  `docs/issues/open/2776-2411-axum-server-0-9-timeouts/ISSUE.md`
- `axum-server` 0.9.0, `Server::first_request_timeout`
- Reproduction repository: <https://github.com/josecelano/axum-server-timeout>
- RFC 9110, section 15.5.9 (`408 Request Timeout`)
