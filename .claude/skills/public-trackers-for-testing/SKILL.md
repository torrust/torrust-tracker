---
name: public-trackers-for-testing
description: Public trackers to test tracker clients against. Use when checking announce or scrape against a live tracker, comparing local and public behavior, or debugging a timeout.
metadata:
  author: torrust
  version: "1.0"
---

# Public Trackers for Testing

## Skill Links

This skill depends on these artifacts. If any of them change, review this skill.

- `console/tracker-client/src/console/clients/unified/udp.rs`
- `console/tracker-client/src/console/clients/unified/http.rs`
- `.claude/skills/run-tracker-locally/SKILL.md`

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

> `tracker_client` writes JSON to stdout and refuses to run when stdout is a terminal (exit 2, a `tty_refusal` record on stderr), so the commands below pipe the output to `jq`. Pipe or redirect it the same way in your own commands.

```bash
cargo run -q -p torrust-tracker-client --bin tracker_client udp scrape \
  udp://udp1.torrust-tracker-demo.com:6969/scrape \
  "$INFO_HASH" \
  --format text | jq .
```

### UDP announce (preferred public demo)

```bash
cargo run -q -p torrust-tracker-client --bin tracker_client udp announce \
  udp://udp1.torrust-tracker-demo.com:6969/announce \
  "$INFO_HASH" \
  --format json | jq .
```

### HTTP announce (preferred public demo)

```bash
cargo run -q -p torrust-tracker-client --bin tracker_client http announce \
  https://http1.torrust-tracker-demo.com:443 \
  "$INFO_HASH" | jq .
```

## Timeout Triage

If a public target times out:

1. Retry once against the same target.
2. Retry against the other public demo.
3. If both fail, run locally and verify behavior deterministically.

Do not assume client regression from a single public timeout.

## Local Fallback

Use this workflow when public trackers are unavailable or overloaded:

- `.claude/skills/run-tracker-locally/SKILL.md`

Then re-run the same client command against `127.0.0.1`.

## Notes

- Public demo load varies over time.
- Trackers may contain existing swarm state, so results can differ from clean local runs.
- Prefer local checks for acceptance criteria that require deterministic values.
