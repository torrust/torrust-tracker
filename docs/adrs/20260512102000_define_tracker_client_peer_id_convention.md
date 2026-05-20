---
semantic-links:
  skill-links:
    - create-adr
  related-artifacts:
    - .github/skills/dev/planning/create-adr/SKILL.md
    - packages/peer-id/
    - packages/tracker-client/
    - console/tracker-client/
---

# Define Tracker-Client Peer ID Convention

## Description

Tracker-client defaults currently use a qBittorrent peer ID prefix (`-qB`), which
misrepresents Torrust tracker-client traffic.

Issue [#1564](https://github.com/torrust/torrust-tracker/issues/1564) requires
adopting a Torrust-specific convention while keeping protocol fixtures explicit
and package boundaries decoupled.

## Agreement

We adopt the following tracker-client peer ID convention:

- Prefix: `RC` (Rust Client)
- Version field: `3000` for the current `v3.0.0` line
- Full layout: `-<CC><VVVV>-<12-digit-suffix>` (Azureus-style)

Defaults are split by context:

- Production defaults use `-RC3000-` plus a randomized 12-digit suffix.
- The production default is generated once per process and reused.
- Tests and fixtures use deterministic values such as
  `-RC3000-000000000001`.

Version source policy:

- Version bytes are hard-coded per release for now.
- The value is updated explicitly when the client versioning policy changes.

Package coupling policy:

- Protocol and server package fixtures do not import tracker-client constants.
- They may define local deterministic constants that follow the same convention.

## Date

2026-05-12

## References

- <https://github.com/torrust/torrust-tracker/issues/1564>
- <https://www.bittorrent.org/beps/bep_0020.html>
- <https://wiki.theory.org/BitTorrentSpecification#peer_id>
- [Issue Spec](../issues/open/1564-tracker-client-change-default-peer-id.md)
