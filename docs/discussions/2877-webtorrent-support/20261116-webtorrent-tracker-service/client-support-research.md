---
semantic-links:
  related-artifacts:
    - "issue #2877"
    - docs/discussions/2877-webtorrent-support/20261116-webtorrent-tracker-service/README.md
---

<!-- cspell:ignore Ageneau arvidn libdatachannel libjuice apfeltee kevincox gubatron yipperr shayzluf stalkerok jasminsehic Linuxiac Chocobo sledgehammer xavier ThaUnknown LewpyUK btorrent openwebtorrent -->

# WebTorrent Support in qBittorrent and libtorrent

Source material for the [WebTorrent tracker service discussion](README.md). Claude Code with Claude
Sonnet 5.5 researched it on 2026-11-16, and Claude Opus 5.5 checked the key claims against the
qBittorrent 5.3.0 release notes and qBittorrent issue 25058. It was edited for the linters and for
length. Each claim is marked as verified, partly verified, or unverified.

## Question

Why did qBittorrent add WebTorrent support, what use cases did its maintainers give, and what does it
mean for a tracker?

## Status of the Claims

| Claim                                                      | Status          | Evidence                                                                                                                                                                                                                         |
| ---------------------------------------------------------- | --------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| qBittorrent 5.3.0 uses libtorrent 2.1 by default           | Verified        | The release lists builds with libtorrent 2.1.2 and 1.2.20; press coverage says 2.1 is the default.                                                                                                                               |
| qBittorrent 5.3 exposes a WebTorrent setting               | Verified        | The 5.3.0 release notes list "Expose libtorrent setting for WebTorrent STUN server", qBittorrent PR 24837.                                                                                                                      |
| WebTorrent is enabled in the official qBittorrent 5.3 builds | Partly verified | In qBittorrent issue 25058, a maintainer says the rc1 build had it disabled and that he would enable it in the release. A user reports it fixed in 5.3. The final binary was not checked.                                     |
| WebTorrent needs libtorrent 2.1                            | Verified        | The libtorrent 2.1.0 release notes list WebTorrent, enabled by default. libtorrent 2.0 and 1.2 do not have it (arvidn/libtorrent issue 7283).                                                                                    |
| It is a build option, not a user setting                   | Verified        | libtorrent has the `webtorrent` build option. qBittorrent PR 24564 enables it in CI.                                                                                                                                             |
| NAT traversal is part of qBittorrent's reasoning           | Unverified      | The STUN server setting exists, but its pull request gives privacy as the reason.                                                                                                                                               |

## qBittorrent

- [Issue 4163](https://github.com/qbittorrent/qBittorrent/issues/4163) (2015): the original request.
  It says WebTorrent peers are cut off from the global swarm, because browsers cannot use TCP or uTP,
  and asks for WebRTC in libtorrent so browser peers join it. The maintainers closed it because the
  work belongs in libtorrent.
- [Issue 17974](https://github.com/qbittorrent/qBittorrent/issues/17974) (2022): a WebTorrent
  developer asked for the feature. A maintainer replied that qBittorrent would add it when libtorrent
  ships it in a release.
- [Issue 25058](https://github.com/qbittorrent/qBittorrent/issues/25058): the 5.3.0rc1 Windows build
  had WebTorrent disabled. The maintainer first thought it made no sense for a desktop client, then
  learned that non-web clients use it too, and said he would enable it in the release.
- [PR 24837](https://github.com/qbittorrent/qBittorrent/pull/24837): the WebTorrent STUN server
  setting. The libtorrent default is a Google STUN server; the stated motivation is privacy, so users
  can choose their own server or none.
- [PR 24613](https://github.com/qbittorrent/qBittorrent/pull/24613): a proposal to add three public
  `wss://` trackers to new torrents by default. Reviewers objected and it was closed as stale.
- [Issue 24899](https://github.com/qbittorrent/qBittorrent/issues/24899): a 5.3 beta user seeds about
  125 torrents to a browser client. Seeding stops after a few hours while UDP peers keep working. This
  is the only real-world use found, and it is an open bug report.
- [PR 25259](https://github.com/qbittorrent/qBittorrent/pull/25259) (open): fixes a 5.3 regression
  where the magnet link tracker filter dropped `ws://` and `wss://` trackers.
- Other links: [issue 15815](https://github.com/qbittorrent/qBittorrent/issues/15815) (duplicate of
  issue 4163), [PR 24564](https://github.com/qbittorrent/qBittorrent/pull/24564),
  [PR 24512](https://github.com/qbittorrent/qBittorrent/pull/24512),
  [release 5.3.0](https://github.com/qbittorrent/qBittorrent/releases/tag/release-5.3.0).

Not found: any qBittorrent maintainer arguing for WebTorrent on grounds of NAT traversal, PeerTube,
or streaming. The qBittorrent forum was not searched.

## libtorrent

- [Issue 223](https://github.com/arvidn/libtorrent/issues/223) (2015): the request. The motivation
  is bridging the classic swarm and the browser swarm. Others wanted streaming in the browser. Some
  contributors worried about swarm health, for example browser tabs that stop seeding when closed.
- [PR 4123](https://github.com/arvidn/libtorrent/pull/4123) by Paul-Louis Ageneau, merged
  2020-07-01: the implementation. Its stated goal is that libtorrent can run hybrid peers. It uses
  his libdatachannel library, which uses libjuice for ICE, and it needs a WebSocket tracker for
  signalling. The libtorrent maintainer accepted it on the conditions of C++17, a way to disable it,
  and thorough tests.
- libtorrent 2.1.0 release notes: WebTorrent is enabled by default, with fuzz targets. The notes
  warn that it significantly widens the attack surface and that it is best disabled if it is not
  needed.
- [Issue 7283](https://github.com/arvidn/libtorrent/issues/7283): WebTorrent is not in libtorrent
  2.0 or 1.2.

## What It Means for a Tracker

- WebTorrent peers need a `ws://` or `wss://` tracker to exchange WebRTC offers and answers. Without
  one, they cannot connect. A qBittorrent build without WebTorrent reports these trackers as an
  unsupported URL protocol.
- qBittorrent adds no WebSocket tracker by default, so a torrent reaches browser peers only if it
  already lists one.
- Trackers named in the sources: `wss://tracker.openwebtorrent.com` in the libtorrent test setup,
  and the reference tracker [webtorrent/bittorrent-tracker](https://github.com/webtorrent/bittorrent-tracker).
  The rejected qBittorrent PR 24613 also named `tracker.btorrent.xyz` and `tracker.webtorrent.dev`.
  aquatic's `ws` tracker and `wt-tracker` were not mentioned in the issues and pull requests searched.
