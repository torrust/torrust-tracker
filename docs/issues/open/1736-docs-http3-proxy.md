---
doc-type: issue
issue-type: task
status: in-progress
priority: p1
github-issue: 1736
spec-path: docs/issues/open/1736-docs-http3-proxy.md
branch: 1736-docs-http3-proxy-follow-up
related-pr: null
last-updated-utc: 2026-05-12 15:47
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/templates/ISSUE.md
---

<!-- skill-link: create-issue -->

# Issue #1736 - docs(http): document HTTP/3 support via reverse proxy

## Goal

Document how tracker HTTP endpoints can expose HTTP/3 to clients via a reverse proxy (e.g., Caddy), and create a follow-up task to test and evaluate direct/native HTTP/3 support in the tracker once upstream Rust HTTP ecosystem support stabilizes.

## Background

Operators deploying the tracker may assume that native HTTP/3 support in the tracker itself is required to offer HTTP/3 to clients. In practice, an edge reverse proxy (e.g., Caddy with QUIC/UDP 443 enabled) can provide HTTP/3 at the edge while the backend tracker remains on HTTP/1.1 or HTTP/2.

Additionally, the Rust HTTP ecosystem (Hyper, Axum, Tokio) is still maturing HTTP/3 support. The project should document the current proxy-based deployment pattern and create a clear reminder to evaluate native HTTP/3 once upstream dependencies stabilize.

## Scope

### In Scope

- Document in [docs/containers.md](../../containers.md) how to provide HTTP/3 at the proxy edge for tracker HTTP endpoints.
- Explain protocol boundaries: client → proxy (HTTP/3 optional) vs. proxy → backend (HTTP/1.1/HTTP/2).
- Include an example Caddy configuration snippet showing UDP 443 (QUIC) enablement.
- Add operational guidance on monitoring and the optional/reversible nature of HTTP/3 at the edge.
- Create a follow-up issue spec and GitHub issue to track native HTTP/3 support readiness.

### Out of Scope

- Implementing native HTTP/3 in the tracker HTTP server (future work, blocked on upstream support).
- Modifying tracker HTTP server code in this task.
- Performance benchmarks (will be in the follow-up task).

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                     | Notes / Expected Output                                                |
| --- | ------ | -------------------------------------------------------- | ---------------------------------------------------------------------- |
| T1  | TODO   | Review current [docs/containers.md](../../containers.md) | Identify sections for HTTP/3 documentation additions.                  |
| T2  | TODO   | Draft HTTP/3 proxy section in containers docs            | Explain protocol boundaries; describe reverse proxy pattern.           |
| T3  | TODO   | Add Caddy example configuration                          | Include UDP 443 (QUIC) snippet; link to Caddy HTTP/3 documentation.    |
| T4  | TODO   | Add operational guidance                                 | CPU/load monitoring notes; reversible deployment notes.                |
| T5  | DONE   | Create follow-up issue spec for native HTTP/3 readiness  | Spec at `docs/issues/open/1765-native-http3-readiness.md`.             |
| T6  | DONE   | Cross-link follow-up issue in this spec and vice versa   | Follow-up is issue #1765; linked in References below.                  |
| T7  | TODO   | Add manual HTTP/3 verification steps to the docs         | Document `curl --http3-only` commands for both proxy and native cases. |
| T8  | TODO   | Run linter and review documentation                      | Ensure markdown, spelling, and formatting pass all checks.             |

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted in `docs/issues/drafts/`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [x] Follow-up issue created and linked
- [ ] Implementation completed (docs updated)
- [ ] Reviewer validated acceptance criteria
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-05-12 00:00 UTC - Agent - Spec drafted in `docs/issues/drafts/1736-docs-http3-proxy.md`
- 2026-05-12 15:35 UTC - Agent - Spec reviewed and approved; GitHub issue #1736 confirmed; follow-up issue #1765 created; spec moved to `docs/issues/open/1736-docs-http3-proxy.md`
- 2026-05-12 15:47 UTC - Agent - Verified HTTP/3 works on the demo deployment (Caddy proxy); added manual verification section with tested `curl --http3-only` commands

## Acceptance Criteria

- [ ] AC1: [docs/containers.md](../../containers.md) contains a new section explaining HTTP/3 support via reverse proxy.
- [ ] AC2: Docs clearly explain the protocol boundary between edge (HTTP/3 optional) and backend (HTTP/1.1/HTTP/2).
- [ ] AC3: Example Caddy configuration with UDP 443 (QUIC) is included.
- [ ] AC4: Operational guidance covers monitoring, reversibility, and optional deployment of HTTP/3.
- [ ] AC5: A follow-up issue (and spec) exists to test native HTTP/3 support once upstream dependencies support it.
- [ ] AC6: The follow-up issue includes a minimal test/benchmark checklist.
- [ ] AC7: `linter all` exits with code `0`.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                                        |
| ----- | ---------------------- | ----------------------------------------------- |
| AC1   | TODO                   | docs/containers.md                              |
| AC2   | TODO                   | docs/containers.md                              |
| AC3   | TODO                   | docs/containers.md                              |
| AC4   | TODO                   | docs/containers.md                              |
| AC5   | DONE                   | docs/issues/open/1765-native-http3-readiness.md |
| AC6   | DONE                   | docs/issues/open/1765-native-http3-readiness.md |
| AC7   | TODO                   | linter output                                   |

## Manual HTTP/3 Verification

These commands verify HTTP/3 is working for the tracker HTTP endpoints. They apply to both the
proxy-based case (today) and the future native case.

### Prerequisites

The system `curl` on Ubuntu/Debian does not include HTTP/3 support. Install the snap build:

```bash
sudo snap install curl --channel=latest/stable
# snap curl lives at /snap/bin/curl
```

Confirm HTTP/3 support is present:

```bash
/snap/bin/curl --version | grep -E 'ngtcp2|nghttp3'
# Expected: ngtcp2/x.x.x nghttp3/x.x.x in the version line
```

### Step 1 — Confirm the server advertises HTTP/3

The first request over HTTP/1.1 or HTTP/2 should include an `alt-svc` header advertising `h3`:

```bash
curl -sI https://http1.torrust-tracker-demo.com/announce | grep -i alt-svc
# Expected: alt-svc: h3=":443"; ma=2592000
```

### Step 2 — Force an HTTP/3-only HEAD request

```bash
/snap/bin/curl --http3-only -sI https://http1.torrust-tracker-demo.com/announce
# Expected first line: HTTP/3 200
```

### Step 3 — Verbose output to confirm QUIC negotiation

```bash
/snap/bin/curl --http3-only -v https://http1.torrust-tracker-demo.com/announce 2>&1 \
  | grep -E 'QUIC|HTTP/3|h3|Connected|protocol'
```

### Step 4 — Full announce request over HTTP/3

Replace `<info_hash>` and `<peer_id>` with valid values:

```bash
/snap/bin/curl --http3-only -s \
  "https://http1.torrust-tracker-demo.com/announce?info_hash=%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00&peer_id=-TR3000-abcdefghijkl&port=6881&uploaded=0&downloaded=0&left=0&event=started"
```

### Verified results (proxy case — Caddy demo deployment)

Tested on 2026-05-12 against `https://http1.torrust-tracker-demo.com`:

```text
# Step 1
alt-svc: h3=":443"; ma=2592000

# Step 2
HTTP/3 200
date: Tue, 12 May 2026 15:46:55 GMT
content-type: text/plain; charset=utf-8
via: 1.1 Caddy
```

The `via: 1.1 Caddy` header confirms the request was handled by the Caddy reverse proxy.
HTTP/3 is terminated at Caddy; the backend tracker still receives HTTP/1.1 or HTTP/2.

## Risks and Trade-offs

- **Risk**: Caddy configuration examples may become outdated if Caddy's HTTP/3 setup changes.
  - _Mitigation_: Link to official Caddy HTTP/3 documentation; pin example to current stable release.
- **Risk**: Without clear protocol boundary explanation, operators may attempt to upgrade the tracker backend prematurely.
  - _Mitigation_: Use clear diagrams or ASCII art; explicitly state "proxy handles HTTP/3 negotiation."

## References

- GitHub issue: https://github.com/torrust/torrust-tracker/issues/1736
- Follow-up issue: #1765 — https://github.com/torrust/torrust-tracker/issues/1765
- Related GitHub issue (demo): https://github.com/torrust/torrust-tracker-demo/issues/31
- Upstream tracker: https://github.com/hyperium/hyper/pull/3925 (Hyper HTTP/3 support)
- Caddy HTTP/3 docs: https://caddyserver.com/docs/protocol/http3
- Related website docs issue: https://github.com/torrust/torrust-website/issues/198
