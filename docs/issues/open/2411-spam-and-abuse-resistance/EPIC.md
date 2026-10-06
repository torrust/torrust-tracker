---
schema-version: 1
doc-type: epic
status: planned
epic: null
github-issue: 2411
spec-path: docs/issues/open/2411-spam-and-abuse-resistance/EPIC.md
epic-owner: josecelano
last-updated-utc: "2026-10-06 16:00"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
---

<!-- skill-link: create-issue -->

# EPIC #2411 - Spam and Abuse Resistance

## Goal

Keep one inventory of the ways clients can overload the tracker or its APIs,
then evaluate a shared rate-limiting design and any complementary controls.

## Why This Is Needed

The tracker lacks a shared configurable request-rate policy across its services.
Existing protections, including UDP connection-ID validation, banning, and
bounded active-request handling, must be inventoried rather than assumed absent.
Per-request limits bound individual work but do not bound aggregate work from
parallel requests. This EPIC collects the cases before selecting controls.

## Scope

### In Scope

- An inventory of abuse cases, updated whenever a new one is found.
- Linking existing issues as sub-issues.
- Later: a rate-limiting design (likely an ADR) and its implementation.

### Out of Scope

- Security vulnerabilities that must be reported privately (see
  `SECURITY.md`). Do not add undisclosed vulnerabilities to this public
  inventory.
- Implementing controls in this spec-only PR; implementation belongs to later
  child issues, after the inventory and design are reviewed.

## Abuse Case Inventory

Add a row for every new case. "Source" says where it was found.

| ID | Case | Affected | Effect | Status | Source |
| --- | --- | --- | --- | --- | --- |
| A1 | No shared configurable request-rate policy | UDP, HTTP, REST API, health check | Aggregate work can exceed capacity despite existing local protections | Confirmed: no service has a request-rate policy; HTTP, REST API and health check have no concurrency or connection limit; UDP has only connection-ID banning and an approximate active-request ring. Hypothesis: the capacity at which aggregate work degrades service; confirm by measuring throughput and latency under parallel load on an isolated local tracker | Known limitation |
| A2 | Connections without a client timeout | HTTP, REST API, health check | Idle connections hold resources | Confirmed: the HTTP tracker and REST API bound header reads and handler work to 5 s, and their plaintext listeners close a connection idle for 5 s, but TLS listeners omit that idle-connection acceptor; the health check API sets none of these timeouts. Hypothesis: whether idle TLS and health check connections are ever closed; confirm by holding idle connections against an isolated local tracker | #324 |
| A3 | HTTP scrape exceeds the documented 74 info-hash limit | HTTP tracker | Local requests returned 75 and 1000 entries; overload impact not measured | Decided in #2417: HTTP keeps the first 100 as a per-request policy cap ([ADR](../../../adrs/20261005124222_cap_scrape_info_hashes_per_protocol.md)); total load still needs rate limiting | [HTTP limit evidence](../../closed/2417-2411-verify-http-scrape-info-hash-limit/manual-verification-evidence.md) |
| A4 | Announce for new info hashes adds torrents to memory when policy permits | UDP, HTTP tracker | Memory is retained while peers remain active; expiry and optional peerless cleanup affect retention | Confirmed: with the default `listed = false` every info hash is authorized and its first announce adds a swarm, with no cap on the number of swarms; a swarm stays until its peers expire and peerless cleanup removes it, and stays indefinitely when `remove_peerless_torrents = false` or cleanup is disabled. Hypothesis: memory per swarm and its growth rate; confirm by measuring resident memory against distinct announced info hashes | SI-22 session, 2026-10-02 |
| A5 | First announce of an unknown torrent reads the database when persistence is enabled | UDP, HTTP tracker | Database load from random info hashes | Confirmed: an announce for a swarm absent from memory reads the database once when `persistent_torrent_completed_stat` is enabled (default `false`); the swarm it then adds (A4) prevents re-reads until eviction. Hypothesis: database load at a given rate of new info hashes; confirm by counting queries and latency on an isolated local tracker | `AnnounceHandler::load_downloads_metric_if_needed` |
| A6 | Scrape reads the database for authorized info hashes absent from memory when persistence is enabled | UDP, HTTP tracker | Database load via scrape; no memory growth, since a scrape adds no swarm | Confirmed: after the fix, a scrape with persistence enabled loads authorized info hashes absent from memory in one uncached batch (at most 100 per query) and never adds a swarm, so it reads the database on every such request but does not grow memory. Hypothesis: database load from repeated or large HTTP scrapes (A3); confirm by counting queries and latency per scrape rate and size | #2406 |
| A7 | Announces with many distinct peer addresses grow a swarm's peer list | UDP, HTTP tracker | Memory is retained per peer until `max_peer_timeout` expires it | Confirmed: a swarm keys peers by address with no per-swarm or per-client cap; only the announce response is capped. Hypothesis: memory per peer and its growth rate; confirm by measuring resident memory against announced peers on an isolated local tracker | Inventory review, 2026-10-05 |
| A8 | Invalid UDP connection IDs from many source addresses grow the ban counter | UDP tracker | One counter per source IP is retained until the ban reset | Confirmed: the exact counter map is unbounded until the reset, as its ADR records. Hypothesis: memory growth rate; confirm by measuring resident memory against distinct invalid sources | [UDP banning ADR](../../../../packages/udp-core/docs/adrs/20260829204258_use_exact_ip_counters_for_udp_banning.md) |

Notes:

- A3: each protocol parser enforces its own scrape limit
  (`MAX_SCRAPE_INFO_HASHES`: 74 in `udp-protocol`, 100 in `http-protocol`).
- A5: sequential announces avoid a database read while the torrent remains in
  memory. Eviction permits another read, and concurrent first announces may
  both observe a miss. A6's caching policy remains an implementation decision
  in #2406; do not assume it already exists.
- Case status means an inventory hypothesis unless reproduced. Confirm current
  behavior and existing controls before choosing a mitigation or publishing
  newly discovered security-sensitive details.
- Review basis: a "Confirmed" status was read from the code at `050726d46`, not reproduced at runtime; each "Hypothesis" names the measurement that would confirm it. A3's status records the #2417 decision, whose evidence includes runtime checks (V1 to V3). The `path:line` citations resolve at `7970cdf0a`. [Existing Controls](#existing-controls) lists the protections each case relies on.
- A1: no rate limiter, semaphore or connection limit exists in the HTTP tracker, REST API or health check API servers; the UDP ring is not an exact bound (`packages/udp-server/src/server/request_buffer.rs:20`); `interval_min` is advertised to clients (`packages/http-protocol/src/v1/responses/announce/encoding.rs:66`) but not enforced.
- A2: the idle-connection acceptor (`packages/axum-server/src/custom_axum_server.rs:89`) is applied only to plaintext listeners (`packages/axum-http-server/src/server.rs:145`, `packages/axum-rest-api-server/src/server.rs:314`); the REST API records why TLS omits it (`packages/axum-rest-api-server/src/server.rs:464`); the health check API starts a plain server (`packages/axum-health-check-api-server/src/server.rs:154`).
- A3: UDP keeps the first 74 hashes (`packages/udp-protocol/src/request.rs:34`) and its 1496-byte receive buffer (`packages/udp-server/src/server/receiver.rs:15`) cannot carry more; HTTP keeps the first 100 `info_hash` parameters (`packages/http-protocol/src/v1/requests/scrape.rs:25`, `packages/http-protocol/src/v1/requests/scrape.rs:78`) and ignores the rest without decoding them, as its documentation states (`packages/axum-http-server/src/lib.rs:235`).
- A4: authorization passes every info hash unless the tracker is listed (`packages/tracker-core/src/whitelist/authorization.rs:51`); the first announce inserts the swarm (`packages/swarm-coordination-registry/src/swarm/registry.rs:61`); a peerless swarm is removed only when `remove_peerless_torrents` is set (`packages/swarm-coordination-registry/src/swarm/coordinator.rs:205`); cleanup is not scheduled when `inactive_peer_cleanup_interval` is `0` (`src/app.rs:535`).
- A5: the memory check and the read are `packages/tracker-core/src/announce_handler.rs:199` and `packages/tracker-core/src/announce_handler.rs:203`; nothing serializes them, which is consistent with the concurrent double read noted above; eviction is peerless cleanup (`packages/swarm-coordination-registry/src/swarm/registry.rs:294`).
- A6: the batch is `packages/tracker-core/src/scrape_handler.rs:186`, split per query by `packages/tracker-core/src/databases/driver/mod.rs:15`; a scrape adds no swarm (`packages/tracker-core/src/scrape_handler.rs:72`, tested by `packages/tracker-core/tests/integration.rs:86`). The fix added no cache and rejected caching a peerless swarm on scrape, as the [batched uncached lookup ADR](../../../adrs/20261002173716_load_persisted_scrape_downloads_with_a_batched_uncached_lookup.md) records.
- A7: peers are inserted by address (`packages/swarm-coordination-registry/src/swarm/coordinator.rs:162`); `max_peers_per_announce` caps only the response (`packages/tracker-core/src/announce_handler.rs:218`).
- A8: one entry per source IP (`packages/udp-core/src/services/banning.rs:37`), cleared only by the reset job (`src/bootstrap/jobs/udp_tracker_server.rs:87`).

### Existing Controls

Each control bounds one cost locally; none bounds aggregate work across requests or clients (A1).

| Control | Where | What it bounds | Configuration (default) | Cases |
| --- | --- | --- | --- | --- |
| UDP connection-ID validation | `packages/udp-core/src/connection_cookie.rs:159`, `packages/udp-core/src/connection_cookie.rs:66` | Announce and scrape need a connection ID whose issue time falls within the cookie lifetime; the cookie binds a fingerprint of the source address and the issue time by arithmetic mixing, not a MAC, so a cookie minted for one fingerprint can coincidentally pass for another; the key is generated at startup and not rotated while the process runs (`packages/udp-core/src/crypto/ephemeral_instance_keys.rs:24`) | `udp_tracker_server.connection_id_validation`, `strict` (`packages/configuration/src/v3_0_0/udp_tracker_server.rs:27`); `udp_trackers[].cookie_lifetime`, 120 s (`packages/configuration/src/v3_0_0/udp_tracker.rs:60`) | A1, A8 |
| UDP IP banning | `packages/udp-server/src/server/launcher.rs:487` | Drops, without a response, datagrams from an IP with more connection-ID errors than the limit; only connection-ID errors count (`packages/udp-server/src/banning/event/handler.rs:27`); shared by all UDP listeners; enforced only under `strict` | `udp_tracker_server.max_connection_id_errors_per_ip`, 10 (`packages/configuration/src/v3_0_0/udp_tracker_server.rs:159`); `udp_tracker_server.ip_bans_reset_interval_in_secs`, 86400, minimum 3600 (`packages/configuration/src/v3_0_0/udp_tracker_server.rs:95`) | A1, A8 |
| UDP active-request ring | `packages/udp-server/src/server/request_buffer.rs:9` | When every tracked processor is unfinished, aborts the oldest after one yield to admit a new datagram (`packages/udp-server/src/server/launcher.rs:434`); not an exact concurrency bound, as the [ring ADR](../../../../packages/udp-server/docs/adrs/20260929181216_bound_udp_request_concurrency_with_task_per_request_ring.md) records | Not configurable, 50 | A1 |
| UDP source-port-zero discard | `packages/udp-server/src/server/launcher.rs:467` | Discards, before processing, datagrams that cannot receive a response | None | A1 |
| UDP datagram size | `packages/udp-protocol/src/common.rs:19` | One 1496-byte receive buffer per datagram (`packages/udp-server/src/server/receiver.rs:15`) | Not configurable | A3 |
| UDP scrape count | `packages/udp-protocol/src/request.rs:34` | First 74 info hashes per scrape, the rest ignored (`packages/udp-protocol/src/request.rs:145`), with the cap passed by the server (`packages/udp-server/src/handlers/mod.rs:87`) | Not configurable, 74: `MAX_SCRAPE_INFO_HASHES` in `udp-protocol`, computed from the datagram size | A3 |
| HTTP scrape count | `packages/http-protocol/src/v1/requests/scrape.rs:25` | First 100 `info_hash` parameters per request, the rest ignored without decoding (`packages/http-protocol/src/v1/requests/scrape.rs:78`) | Not configurable, 100: `MAX_SCRAPE_INFO_HASHES` in `http-protocol` | A3 |
| HTTP idle-connection timeout | `packages/axum-server/src/custom_axum_server.rs:89` | Closes a plaintext HTTP tracker or REST API connection that completes no request within 5 s of opening or of the last response | Not configurable, 5 s (`packages/axum-server/src/custom_axum_server.rs:42`) | A2 |
| HTTP header-read and HTTP/2 keep-alive timeouts | `packages/axum-server/src/custom_axum_server.rs:68` | HTTP/1 request headers within 5 s; HTTP/2 keep-alive pings (`packages/axum-server/src/custom_axum_server.rs:72`); HTTP tracker and REST API, plaintext and TLS | Not configurable, 5 s | A2 |
| HTTP per-request timeout | `packages/axum-http-server/src/v1/routes.rs:161` | Handler work over 5 s answers `408` on the HTTP tracker and the REST API (`packages/axum-rest-api-server/src/routes.rs:145`) | Not configurable, 5 s (`packages/axum-http-server/src/v1/routes.rs:28`) | A1, A2 |
| Announce response size | `packages/tracker-core/src/announce_handler.rs:218` | Peers returned per announce, not peers stored | `core.announce_policy.max_peers_per_announce`, 74 (`packages/primitives/src/announce.rs:81`) | A7 |
| Peer expiry and peerless cleanup | `packages/tracker-core/src/torrent/manager.rs:104` | Removes peers idle longer than `max_peer_timeout`, then peerless swarms when enabled (`packages/tracker-core/src/torrent/manager.rs:108`) | `core.tracker_policy.max_peer_timeout`, 900 s (`packages/primitives/src/policy.rs:59`); `core.inactive_peer_cleanup_interval`, 600 s, `0` disables (`packages/configuration/src/v3_0_0/core.rs:80`); `core.tracker_policy.remove_peerless_torrents`, `true` (`packages/primitives/src/policy.rs:67`) | A4, A5, A7 |
| Listed and private modes | `packages/tracker-core/src/whitelist/authorization.rs:51` | Only whitelisted info hashes (listed) or keyed HTTP clients (private, `packages/http-core/src/services/announce.rs:306`; UDP listeners do not start in private mode, `src/app.rs:329`) reach the swarm registry | `core.listed`, `false` (`packages/configuration/src/v3_0_0/core.rs:84`); `core.private`, `false` | A4, A5, A7 |
| Persisted-downloads query size | `packages/tracker-core/src/databases/driver/mod.rs:15` | At most 100 info hashes per query; larger scrapes run several | Not configurable, 100 | A6 |
| REST API token check | `packages/axum-rest-api-server/src/v1/middlewares/auth.rs:161` | Constant-time comparison; no attempt limit | `http_api.access_tokens`; bound to `127.0.0.1:1212` by default (`packages/configuration/src/v3_0_0/tracker_api.rs:62`) | A1 |
| Health check API exposure | `packages/configuration/src/v3_0_0/health_check_api.rs:33` | The loopback default is the only control; each request starts one probe per registered service (`packages/axum-health-check-api-server/src/handlers.rs:21`) | `health_check_api.bind_address`, `127.0.0.1:1313` | A1, A2 |

## Subissues

| Order | Issue | Local Spec | Status | Notes |
| --- | --- | --- | --- | --- |
| 1 | #324 - Denial of Service attack factor | None (pre-dates specs) | TODO | Existing open issue; linked as a sub-issue |
| 2 | #2417 - Verify whether HTTP scrape enforces the 74 info-hash limit | [ISSUE.md](../../closed/2417-2411-verify-http-scrape-info-hash-limit/ISSUE.md) | DONE | A3 |
| 3 | #[To be assigned] - Rate-limiting design | Not drafted | TODO | After the inventory is reviewed |

## Delivery Strategy

### Phase 1 - Inventory

- Outcome: the inventory lists known cases; existing issues are linked.
- Exit criteria: the maintainer reviews the inventory.

### Phase 2 - Design

- Outcome: a rate-limiting design (per IP, per service, per request type)
  evaluated against the inventory, recorded in a root ADR. Rate limiting is
  not assumed sufficient for idle connections, request sizes, retained state,
  distributed traffic, or shared-IP clients; assess complementary controls.
- Exit criteria: the ADR is accepted.

### Phase 3 - Implementation

- Outcome: sub-issues implement the design and close the inventory cases.

## Progress Tracking

### Workflow Checkpoints

- [x] Epic spec drafted in `docs/issues/drafts/`
- [x] Epic spec reviewed and approved by user/maintainer
- [x] GitHub epic issue created and issue number added to this spec
- [x] #324 linked as a sub-issue
- [x] Subissues created and linked in this spec (#2417; later children are added when approved)

### Progress Log

- 2026-10-02 10:49 UTC - GitHub Copilot - Drafted the EPIC with the first
  inventory (A1-A6), at the maintainer's request during the #1488 SI-22
  session.
- 2026-10-02 18:45 UTC - GitHub Copilot - Marked the creation and linking
  checkpoints done: #2411 exists with #324 and #2417 as GitHub sub-issues
  (PR #2421 review).
- 2026-10-05 13:13 UTC - da2ce7 - Reviewed the inventory against the code at `050726d46`: confirmed A1-A6 from source with `path:line` evidence, separated each remaining hypothesis and its measurement, added A7 and A8, recorded the existing controls, and marked AC1 in progress pending maintainer review.

## Acceptance Criteria

- [ ] AC1: The inventory is reviewed, distinguishes hypotheses from evidence,
  and records existing controls.
- [ ] AC2: #324 and approved public child issues are linked; related issues
  owned elsewhere, including #2406, remain cross-references.
- [ ] AC3: The design maps every case to a mitigation or an explicit deferral.
- [ ] AC4: Each implemented child includes automatic checks, local manual
  evidence, and post-implementation acceptance and completion review.

### Acceptance Verification

| AC ID | Status | Evidence |
| --- | --- | --- |
| AC1 | IN_PROGRESS | Inventory reviewed against the code at `050726d46`; maintainer review pending |
| AC2 | TODO | GitHub parent-child links |
| AC3 | TODO | Accepted ADR and case-to-control mapping |
| AC4 | TODO | Child verification and completion records |

## Architectural Decisions

No existing ADR is selected yet. Phase 2 creates a root ADR because the policy
crosses protocol and API boundaries. Do not prescribe one algorithm before
measuring the cases and reviewing client identity behind proxies and NAT.

## Verification and Completion

Each child runs `linter all`, relevant tests, and required pre-push checks.
Manual checks use an isolated local tracker, never unsolicited public load,
and record commands, toolchain, limits, outcomes, and logs in issue-local
`manual-verification-evidence.md`. Review acceptance criteria after each child.
At EPIC closure, record material discoveries in an implementation retrospective
or state why none is needed. Keep future discoveries in this inventory until
closure, then assign a follow-up owner rather than preventing closure forever.

## Risks and Trade-offs

- Rate limiting can block legitimate heavy users, for example behind a NAT;
  the design must make limits configurable.
- A public inventory tells attackers what is weak; it lists only cases that
  are already public or generic, never private vulnerabilities.

## References

- Related issues: #324, #1510
- Security policy: `SECURITY.md`
