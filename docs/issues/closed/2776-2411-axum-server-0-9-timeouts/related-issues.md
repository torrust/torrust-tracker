---
doc-type: related-issues-disposition
issue-spec: docs/issues/closed/2776-2411-axum-server-0-9-timeouts/ISSUE.md
last-updated-utc: "2026-11-13 09:42"
semantic-links:
  related-artifacts:
    - docs/issues/closed/2776-2411-axum-server-0-9-timeouts/ISSUE.md
    - docs/issues/closed/2776-2411-axum-server-0-9-timeouts/manual-verification-evidence.md
    - docs/adrs/20261111095222_close_idle_connections_without_a_408_response.md
    - docs/issues/open/2411-spam-and-abuse-resistance/EPIC.md
---

# Related Issues: Disposition After #2776

## Purpose

Issue #2776 (`axum-server` 0.9 timeouts, implemented by PR #2813, merge commit `a83f3ef29`) was
the fix for four older GitHub issues: #612, #613, #603 and #324. This file records what each one
said and why it is closed or kept, so that a future reader does not need the GitHub API to
reconstruct the decision. It is the "T8 / AC7" outcome of the [#2776 spec](ISSUE.md): the
maintainer closes or re-scopes the issues on GitHub; this file is the in-repo record.

Evidence used below:

- Baseline on `develop` (`axum-server` 0.8.0), before the fix:
  [evidence/baseline-probe-results.txt](evidence/baseline-probe-results.txt). Plaintext HTTP tracker
  and REST API already closed a silent connection after 5 s. TLS after the handshake and the health
  check API never closed one (still open after 25 s).
- Final build, after the fix: [evidence/results-final-build.txt](evidence/results-final-build.txt) and V8, V9 in
  [manual-verification-evidence.md](manual-verification-evidence.md).
- Automatic tests: the silent-connection tests (plaintext and TLS in the HTTP tracker and REST API
  packages, plaintext in the health check API package, which has no TLS listener) and the
  listener-level suite `packages/axum-server/tests/idle_connections.rs`; red run in
  [evidence/red-run.txt](evidence/red-run.txt) and
  [evidence/red-run-service-tests.txt](evidence/red-run-service-tests.txt), green run in
  [evidence/green-run.txt](evidence/green-run.txt).
- ADR: [Close idle connections without a 408 response](../../../adrs/20261111095222_close_idle_connections_without_a_408_response.md).
- EPIC [#2411](../../open/2411-spam-and-abuse-resistance/EPIC.md): case A2 (connections without a
  client timeout, mitigated by #2776) and case A1 (no request-rate or connection limit, still open).

## Summary

| Issue | Title                                                | Disposition                                    |
| ----- | ---------------------------------------------------- | ---------------------------------------------- |
| #612  | Tracker API: Add a timeout to the TLS handshake, waiting for headers, and waiting for receiving headers | Fixed by #2813; close                          |
| #613  | HTTP Tracker: Add a timeout to the TLS handshake, waiting for headers, and waiting for receiving headers | Fixed by #2813; close                          |
| #603  | Review missing timeouts (EPIC)                       | All items done once #612 and #613 close; close |
| #324  | Denial of Service attack factor                      | Keep open, re-scoped to EPIC #2411 case A1     |

## #612 - Tracker API: Add a timeout to the TLS handshake, waiting for headers, and waiting for receiving headers

Original report (2024-01-16, maintainer; label Bug; child of #603): add timeouts to the REST API
for the TLS handshake, the wait for a first request after the connection opens, and the wait for
the rest of the headers once a client starts sending them. Comments tracked the upstream history:
the Hyper 1.0 migration removed the timeout from `axum-server` (`axum-server` issue #116, axum
issue 2741), a gist-based `TimeoutAcceptor` restored it for plaintext only (no `408`, not usable
with TLS), and Hyper 1.4.0's header-read fix did not remove the need for it. The discussion was
consolidated in #324.

Verdict: **fixed by #2813**. Evidence: the baseline probed the REST API in plaintext only (`:1212`,
closed after 5 s); the gap was that its TLS branch had no timeout wrapper (see the spec's
Background). The runtime TLS figures were measured on the HTTP tracker port: a TLS connection that
finishes the handshake and sends nothing was open after 25 s on `develop` and is closed after 5 s
on the final build, and an idle HTTP/2 connection after a response closes after about 10 s. The
idle HTTP/1 figure (5 s after a response) was measured in plaintext; over TLS it is covered by the
test `it_should_close_an_idle_http1_connection_after_a_response_over_tls` in
`packages/axum-server/tests/idle_connections.rs`. A stalled TLS handshake was already closed after 10 s before the fix
(upstream value). For the REST API's own TLS listener the evidence is the unit test
`it_should_close_a_tls_connection_that_completes_the_handshake_and_sends_nothing` in
`packages/axum-rest-api-server/src/server.rs` (red in `evidence/red-run-service-tests.txt`, green in
`evidence/green-run.txt`). The `408` that the issue's reproduction repository sends is deliberately
not reproduced; see the ADR.

Remains open: nothing for this issue. Rate limiting and connection caps live in EPIC #2411 case A1.

## #613 - HTTP Tracker: Add a timeout to the TLS handshake, waiting for headers, and waiting for receiving headers

Original report (2024-01-16, maintainer; label Bug; child of #603): identical text to #612, applied
to the HTTP tracker. Same comment history.

Verdict: **fixed by #2813**, same evidence as #612; the TLS runtime figures there are the HTTP tracker's own, and its TLS test is `packages/axum-http-server/src/server.rs`.

Remains open: nothing for this issue (see #612).

## #603 - Review missing timeouts

Original report (2024-01-12, maintainer; labels Bug, Enhancement, EPIC): a tracking issue after
the health check of the live demo hung without a timeout. It lists where timeouts may be missing:
the health check binary (#597, #604), the Tracker API (#612), the HTTP tracker (#613), the UDP
tracker (#609) and the Tracker Checker (#677, #678, #679, #682), and relates to #324. It is only
about timeouts, not rate limiting.

Verdict: **close once #612 and #613 are closed.** Checked on 2026-11-12: #597, #604, #609, #677,
issues #678, #679 and #682 are all closed; #612 and #613 are fixed by #2813. Every checklist item is
therefore complete. The tracker's request-rate and connection-limit gaps are not part of #603 and
are tracked by EPIC #2411.

Remains open: nothing for this issue.

## #324 - Denial of Service attack factor

Original report (2023-05-12, external contributor; labels Security, Needs Feedback, Needs
Research): a client that connects to the API or HTTP port with `telnet` and sends nothing is
never disconnected, which the reporter considered a critical security flaw. The thread also
raised a missing rate limit and lock contention under load. The reopening comment on #612 and #613
says the issue was reopened to consolidate those two.

Verdict: **keep open, re-scoped.** The idle-connection part is fixed by #2813 (baseline: the plaintext HTTP
tracker and REST API listeners were already protected; the real gaps were TLS and the health check API, both now
closed). What stays valid is the rate-limiting half of the report. In particular, a client that
stays connected and answers HTTP/2 keep-alive pings is not time-bounded by the timeouts (recorded
in the ADR and in AC4 of the #2776 spec), and no connection or request-rate limit exists on the
HTTP tracker, REST API or health check API.

Remains open, and where it lives: [EPIC #2411](../../open/2411-spam-and-abuse-resistance/EPIC.md)
case A1 ("No shared configurable request-rate policy"), whose evidence is detailed enough that no
new spec is drafted now. #324 is already a sub-issue of #2411. The rate-limiting design issue is
the EPIC's subissue 4 (not yet created, deferred until after v4.0.0). Lock contention (`RwLock`)
is not a timeout topic and has no spec here.

Known limits that remain after #2813: the server closes silently without a `408` (ADR); idle
HTTP/2 clients that answer pings are not time-bounded (A1).
