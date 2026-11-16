---
semantic-links:
  related-artifacts:
    - "issue #2877"
    - "issue #15"
    - "issue #2491"
    - docs/discussions/2491-i2p-peer-support/20261008-i2p-peer-support-design/README.md
    - docs/discussions/2877-webtorrent-support/20261116-webtorrent-tracker-service/client-support-research.md
    - docs/adrs/20261007082938_bound_protocol_agnostic_values_by_the_tightest_delivery_protocol.md
    - docs/discussions/AGENTS.md
    - docs/templates/DISCUSSION.md
---

<!-- cspell:ignore arvidn webrtc -->

# WebTorrent Tracker Service

| Field          | Value                                                                                                                                                                                        |
| -------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Status         | Open for rounds                                                                                                                                                                              |
| Started        | 2026-11-16                                                                                                                                                                                   |
| Opened by      | Jose Celano (`josecelano`)                                                                                                                                                                   |
| Decision owner | Not assigned yet: the owner of the WebTorrent EPIC, once it is created                                                                                                                       |
| Informs        | Issue #2877 - qbittorrent 5.3 lt 2.x already support webrtc/webtorrent with NAT transfer                                                                                                     |
| Scope          | Whether and how the tracker adds a WebTorrent (WebSocket signalling) service, after the v4.0.0 release. Implementation details are left to the feature definition and the EPIC specification |
| AI assistance  | Claude Code with Claude Opus 5.5 drafted the context; Claude Sonnet 5.5 researched the qBittorrent and libtorrent sources                                                                    |

## Context

### Why This Discussion

On 2026-11-16, `i990049` opened issue #2877, asking whether the tracker can support WebTorrent now
that qBittorrent 5.3, built on libtorrent 2.x, supports WebRTC and WebTorrent. Jose Celano replied
on the issue, and an analysis followed in a second comment.

WebTorrent support was already in the project's plans:

- The README lists WebTorrent as a planned protocol.
- The GitHub discussion
  [torrust/torrust-tracker discussions/138](https://github.com/torrust/torrust-tracker/discussions/138)
  (moved from issue #15) explains the protocol and its costs, with input from the author of
  aquatic's WebTorrent tracker.
- Q12 of the [I2P peer support design discussion](../../2491-i2p-peer-support/20261008-i2p-peer-support-design/README.md#q12---webtorrent-as-a-second-design-input)
  checks the shared core design against WebTorrent as well as I2P.
- The ADR
  [bound protocol-agnostic values by the tightest delivery protocol](../../../adrs/20261007082938_bound_protocol_agnostic_values_by_the_tightest_delivery_protocol.md)
  names WebTorrent as a possible later delivery protocol.

The maintainers are focused on releasing v4.0.0, so nothing is decided before that release. After
it, a new service follows the project's process: the GitHub discussion, this design discussion, a
feature definition in `docs/features/`, an EPIC specification for the new service, and then the
implementation. This discussion records what was learned while answering issue #2877, so that it is
not lost before the roadmap is redefined.

Nothing here changes a specification or a rule. A conclusion takes effect only when the decision
owner records it in the Outcome and in the canonical document it links.

### What Changed: Hybrid Desktop Clients

Until now, the main problem with WebTorrent was that browser peers formed a swarm of their own.
Browsers cannot open TCP or uTP connections, so they cannot reach regular BitTorrent clients, and a
torrent had two swarms that did not share data.

libtorrent 2.1 adds WebRTC as a peer transport next to TCP and uTP, and enables it by default. A
client built on it is a hybrid peer. For a torrent that lists both kinds of trackers, it:

- announces to the `udp://` and `http://` trackers and exchanges data with desktop peers over TCP
  or uTP, as before;
- also announces to the `ws://` and `wss://` trackers over a WebSocket, keeps that connection open,
  and exchanges data with browser peers over WebRTC data channels.

So a desktop client can now seed to browser users and download from them. qBittorrent 5.3 uses
libtorrent 2.1 by default; its libtorrent 1.2 builds have no WebTorrent. The sources and what could
not be verified are in
[client-support-research.md](client-support-research.md).

### Why the Tracker Cannot Bridge the Swarms

A browser cannot connect to an `IP:port`, so a peer list from a UDP or HTTP announce is useless to
it. A browser reaches a peer like this:

1. It sends WebRTC offers to the tracker over its WebSocket.
2. The tracker forwards each offer to a peer of the swarm that keeps a WebSocket open.
3. That peer answers through the tracker.
4. The two peers connect directly, using ICE and STUN to get through NAT.

A client that only announced over UDP has no open connection to receive an offer on. The bridge
between the two swarms has to be in the client, which libtorrent 2.1 now provides. The tracker's part
is the WebSocket signalling service. Without it, hybrid clients cannot use WebTorrent with a Torrust
tracker.

### Facts About the Clients

From [client-support-research.md](client-support-research.md):

- libtorrent added WebTorrent in
  [arvidn/libtorrent PR 4123](https://github.com/arvidn/libtorrent/pull/4123) (merged 2020-07-01),
  with the stated aim of hybrid peers. The 2.1.0 release notes warn that the feature widens the
  attack surface and should be disabled if it is not needed.
- qBittorrent's maintainers did not write their own rationale. They waited for libtorrent to ship
  the feature in a release. The original request,
  [qBittorrent issue 4163](https://github.com/qbittorrent/qBittorrent/issues/4163) (2015), says
  browser peers are cut off from the global swarm.
- qBittorrent 5.3 adds an advanced setting for the WebTorrent STUN server. The reason given for it
  is privacy, not NAT traversal.
- Partly verified: whether the official qBittorrent 5.3 binaries ship with WebTorrent enabled. A
  maintainer said the release would enable it and a user reports that 5.3 fixed it; the final binary
  was not checked.
- qBittorrent does not add WebSocket trackers to torrents by default. A torrent reaches browser
  peers only if it already lists a `ws://` or `wss://` tracker.

### Known Costs

- Every WebTorrent peer keeps a WebSocket connection open, and the tracker relays offers and
  answers between peers. This is much heavier than the UDP tracker, where one announce is one small
  datagram and no connection is kept. The load concern was raised in discussions/138.
- There is no formal specification of the WebTorrent tracker protocol. The references are the
  reference tracker
  [webtorrent/bittorrent-tracker](https://github.com/webtorrent/bittorrent-tracker) and aquatic's
  [`ws_protocol` and `ws` crates](https://github.com/greatest-ape/aquatic).

## Topics

### Q1 - Should WebTorrent be a priority after v4.0.0?

Hybrid desktop clients make WebTorrent support more useful than it was: it lets a tracker serve
mixed swarms of desktop and browser peers. Other candidates for the roadmap after v4.0.0 include a
REST API version 2, re-enabling the live demo, I2P peer support, and updatable torrents. Where
should WebTorrent go among them? Added by Jose Celano.

#### Positions

### Q2 - Are swarm statistics shared between classic and WebTorrent peers?

A hybrid client announces the same torrent to the UDP or HTTP service and to the WebTorrent
service, so the same client appears in two peer lists. Should the seeder, leecher, and download
counts of an info hash cover both, or be kept per addressing method? The
[Q5 entry of the I2P design discussion](../../2491-i2p-peer-support/20261008-i2p-peer-support-design/README.md#q5---jose-celano-josecelano-2026-10-19-pr-2562)
proposes one peer group per addressing method and asks the same question for I2P and WebTorrent
clients. Added by Jose Celano.

#### Positions

### Q3 - How is the cost of long-lived connections bounded?

The service keeps a WebSocket connection open for every WebTorrent peer and relays offers and
answers. What limits are needed on connections, offers per announce, and message rates, and how are
they configured? Added by Jose Celano.

#### Positions

### Q4 - How does the service fit the tracker's architecture?

Is WebTorrent a new delivery service next to the UDP and HTTP trackers, with its own configuration
section, sharing the tracker core? What does the core need to offer a service that relays messages
to peers, not only returns peer lists? Added by Jose Celano.

#### Positions

### Q5 - What is the security posture?

libtorrent warns that WebTorrent widens the attack surface. What does the tracker need to defend
against: unauthenticated WebSocket clients, offer flooding, oversized messages, and peers that never
answer? Does the service support the tracker's private mode and keys? Added by Jose Celano.

#### Positions

## Outcome

Pending: no decision recorded.
