# Feature Proposal: JSON Input for Tracker Client Requests

## Status

Deferred (not planned for immediate implementation).

## Summary

This document describes an alternative to many CLI flags for announce requests.
Instead of passing request parameters only as command-line flags, the client could
accept a full JSON object.

The proposal applies to both protocols under the unified client:

- `tracker_client http`
- `tracker_client udp`

## Motivation

Current CLI flags are clear and practical for manual use. However, a JSON-based
input mode can be more convenient for larger payloads, reusable test fixtures,
and future automation.

## Proposed Interfaces

### 1) JSON file input

```bash
tracker_client http announce \
  http://127.0.0.1:7070 \
  --request-file ./announce.json
```

```bash
tracker_client udp announce \
  127.0.0.1:6969 \
  --request-file ./announce.json
```

### 2) Inline JSON input

```bash
tracker_client http announce \
  http://127.0.0.1:7070 \
  --request-json '{"info_hash":"443c7602b4fde83d1154d6d9da48808418b181b6","event":"completed"}'
```

### 3) Standard input (stdin)

```bash
echo '{"info_hash":"443c7602b4fde83d1154d6d9da48808418b181b6","event":"completed"}' \
  | tracker_client http announce http://127.0.0.1:7070 --request-stdin
```

```bash
cat announce.json | tracker_client udp announce 127.0.0.1:6969 --request-stdin
```

## Input Shape (Draft)

```json
{
  "info_hash": "443c7602b4fde83d1154d6d9da48808418b181b6",
  "event": "completed",
  "uploaded": 1234,
  "downloaded": 5678,
  "left": 0,
  "port": 6881,
  "ip": "10.0.0.1",
  "peer_id": "-RC00000000000000001",
  "compact": 1,
  "key": 42,
  "peers_wanted": 50,
  "ip_address": "10.0.0.1"
}
```

Notes:

- HTTP uses `ip` and `compact`.
- UDP uses `ip_address`, `key`, and `peers_wanted`.
- A shared schema can allow optional protocol-specific fields.

## Compatibility Warning: Byte-String Fields

Some protocol fields are byte strings, not guaranteed UTF-8 text.
The most important example is `peer_id` (20 bytes on the wire).

In practice, many peer IDs are ASCII-like and fit naturally in CLI args or JSON
strings. However, full protocol compatibility should allow arbitrary byte values.

If strict compatibility becomes a requirement, both CLI and JSON modes should
support an explicit binary-safe representation.

Possible approaches:

- Keep text form as default for ergonomics.
- Add an explicit encoded form for binary-safe input (for example
  `peer_id_hex` or `peer_id_base64`).
- For CLI, add corresponding flags such as `--peer-id-hex` and
  `--peer-id-base64`.
- For stdin mode, allow raw bytes only when the transport format is binary-safe
  and unambiguous (otherwise prefer explicit encoding).

Example JSON (binary-safe):

```json
{
  "info_hash": "443c7602b4fde83d1154d6d9da48808418b181b6",
  "peer_id_base64": "LVJDMDAwMDAwMDAwMDAwMDAwMDE="
}
```

## Precedence Rule (If Implemented)

If JSON input and flags are provided together, flags should override JSON values.

## Pros

- Better ergonomics for complex requests.
- Easier to store/version fixtures.
- Better fit for automation and generated input.
- Easier composition through stdin pipelines.

## Cons

- Lower discoverability than `--help` flags alone.
- More validation and error-reporting complexity.
- Inline JSON quoting is cumbersome in shells.
- Adds maintenance cost without current automation demand.

## Decision: Why Deferred Now

Not implementing now for the following reasons:

- Request parameters are not expected to change very often.
- There is no current automation pipeline that strongly benefits from JSON input.
- Existing flag-based UX already satisfies manual day-to-day usage.

## Revisit Triggers

Re-open this proposal if one or more are true:

- CI or external tools begin generating tracker-client requests.
- Repeated manual tests require many parameter permutations.
- More request fields are added and CLI flag UX becomes cumbersome.

## Open Questions

- Should stdin mode read from `--request-file -` instead of a dedicated `--request-stdin`?
- Should unknown JSON fields fail fast or be ignored?
- Should protocol-specific fields be split into separate JSON schemas?
