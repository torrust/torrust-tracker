---
name: public-trackers-for-testing
description: Public tracker targets for manual testing and debugging of tracker clients. Use when validating announce/scrape behavior against live services, comparing local vs public behavior, or diagnosing network timeouts. Triggers on "public tracker", "test against demo tracker", "debug tracker timeout", or "which tracker should I use".
metadata:
  author: torrust
  version: "1.0"
---

# Public Trackers for Testing

## Skill Links

This skill depends on these artifacts. If any of them change, review this skill.

- `console/tracker-client/src/console/clients/udp/app.rs`
- `console/tracker-client/src/console/clients/http/app.rs`
- `.github/skills/dev/environment-setup/run-tracker-locally/SKILL.md`

Use the marker `skill-link: public-trackers-for-testing` in affected artifacts.

## Purpose

Use this skill to choose reliable public tracker endpoints for manual verification and debugging.

It provides:

- preferred endpoint order
- copy-paste test commands
- timeout triage and fallback workflow

## Preferred Target Order

When testing against public services, use this order:

1. Tracker demo (newer, usually lower load)
2. Index+Tracker demo (older, can be busy)
3. Local tracker fallback for deterministic checks

## Public Endpoints

### Tracker Demo (preferred)

Repository: <https://github.com/torrust/torrust-tracker-demo>

- HTTP: `https://http1.torrust-tracker-demo.com:443/announce`
- HTTP: `https://http1.torrust-tracker-demo.com:443`
- UDP: `udp://udp1.torrust-tracker-demo.com:6969/announce`

### Index+Tracker Demo (secondary)

Repository: <https://github.com/torrust/torrust-demo>

- HTTP: `https://tracker.torrust-demo.com/announce`
- HTTP: `https://tracker.torrust-demo.com`
- UDP: `udp://tracker.torrust-demo.com:6969/announce`

## Quick Commands

Use a test info hash:

```bash
INFO_HASH=000620bbc6c52d5a96d98f6c0f1dfa523a40df82
```

### UDP scrape (preferred public demo)

```bash
cargo run -q -p torrust-tracker-client --bin udp_tracker_client scrape \
  udp://udp1.torrust-tracker-demo.com:6969/scrape \
  "$INFO_HASH" \
  --format pretty
```

### UDP announce (preferred public demo)

```bash
cargo run -q -p torrust-tracker-client --bin udp_tracker_client announce \
  udp://udp1.torrust-tracker-demo.com:6969/announce \
  "$INFO_HASH" \
  --format compact
```

### HTTP announce (preferred public demo)

```bash
cargo run -q -p torrust-tracker-client --bin http_tracker_client announce \
  https://http1.torrust-tracker-demo.com:443 \
  "$INFO_HASH"
```

## Timeout Triage

If a public target times out:

1. Retry once against the same target.
2. Retry against the other public demo.
3. If both fail, run locally and verify behavior deterministically.

Do not assume client regression from a single public timeout.

## Local Fallback

Use this workflow when public trackers are unavailable or overloaded:

- `.github/skills/dev/environment-setup/run-tracker-locally/SKILL.md`

Then re-run the same client command against `127.0.0.1`.

## Notes

- Public demo load varies over time.
- Trackers may contain existing swarm state, so results can differ from clean local runs.
- Prefer local checks for acceptance criteria that require deterministic values.
