---
doc-type: issue
issue-type: bug
status: done
priority: p2
github-issue: 1985
spec-path: docs/issues/closed/1985-rename-peer-addr-to-ip-in-http-announce-request/ISSUE.md
branch: "1985-rename-peer-addr-to-ip-in-http-announce-request"
related-pr: null
depends-on: null
blocks:
  - docs/issues/open/1987-add-config-option-to-use-ip-from-announce-query-string/ISSUE.md
last-updated-utc: 2026-07-22 00:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/http-protocol/src/v1/requests/announce.rs
    - packages/axum-http-server/src/lib.rs
    - packages/axum-http-server/src/v1/extractors/announce_request.rs
    - packages/http-core/src/services/announce.rs
    - packages/tracker-core/src/torrent/mod.rs
    - docs/adrs/
---

# Issue #1985 - Rename `peer_addr` GET param to `ip` in HTTP announce request (BEP 3)

## Goal

Rename the HTTP announce GET parameter from the non-standard `peer_addr` to the BEP 3-specified `ip`, aligning the wire protocol with the specification. Rename the corresponding Rust field and constant to match, so the wire name and the code name are consistent. Additionally, make an explicit architectural decision about DNS name support in the `ip` parameter.

## Background

[BEP 3 — The BitTorrent Protocol Specification](https://www.bittorrent.org/beps/bep_0003.html) defines the `ip` parameter as:

> An optional parameter giving the IP (or dns name) which this peer is at. Generally used for the origin if it's on the same machine as the tracker.

The Torrust Tracker HTTP announce handler currently uses `peer_addr` as the GET parameter name, which is a non-standard name not defined in any BEP. The correct BEP 3 wire name is `ip`.

### Current state

- The wire GET parameter name is `peer_addr` (constant `PEER_ADDR = "peer_addr"` in `packages/http-protocol/src/v1/requests/announce.rs`).
- The Rust struct field is also named `peer_addr`.
- The existing module documentation in `packages/axum-http-server/src/lib.rs` contains a factually incorrect `NOTICE` (lines 65–70) claiming `peer_addr` comes from the UDP tracker protocol (BEP 15). This is wrong: `ip` is defined in BEP 3 (HTTP) and has been there from the start. The BEP 15 angle is irrelevant to this parameter.
- The field type is `Option<IpAddr>`. DNS names provided by a client are silently dropped by `IpAddr::from_str` in `extract_peer_addr`, with no error returned to the client.
- The parameter is always ignored at the announce service level: `peer_from_request` in `packages/http-core/src/services/announce.rs` builds the peer using the connection-derived IP, never from `announce_request.peer_addr`. Whether to honour the `ip` param in future is addressed separately (see "The 'honour the `ip` param' question" below and Issue 3).

### The DNS name question

BEP 3 specifies the `ip` parameter as accepting "IP (or dns name)". In practice:

- No major tracker implementation supports DNS names in this field (opentracker, chihaya, and others accept IPs only).
- The tracker's peer list stores `IpAddr` values, not hostnames. Supporting DNS would require either resolving names at announce time (latency, DoS vector) or storing hostnames (incompatible with the peer list model).
- The current behaviour (silently drop non-IP values) is confusing and undocumented.

An explicit decision is needed. The decision is captured in the ADR drafted as part of this issue: [`docs/adrs/YYYYMMDD_accept_only_ip_addresses_in_http_announce_ip_param.md`](../../adrs/).

### The "honour the `ip` param" question

This issue deliberately does **not** address whether the tracker should honour the `ip` GET parameter value instead of always using the connection IP. That is a separate feature request tracked as a sub-issue of the configuration overhaul epic (#1978). See related issues below.

## Scope

### In Scope

- Rename the wire GET parameter from `peer_addr` to `ip` throughout the HTTP protocol layer:
  - Rename the constant `PEER_ADDR` → `IP` and its value `"peer_addr"` → `"ip"` in `packages/http-protocol/src/v1/requests/announce.rs`. Also fix the hardcoded `"peer_addr"` literal in the `Display` impl (line 307) to use the renamed `IP` constant.
  - Rename the struct field `peer_addr` → `ip` on `Announce` in the same file. Also fix the doc comment on the `Announce` struct (line 83) which incorrectly claims `peer_addr` is "as per BEP 3" — BEP 3 uses `ip`.
  - Rename the builder method `with_peer_addr` → `with_ip` and update `AnnounceBuilder::with_default_values` accordingly.
  - Update `extract_peer_addr` → `extract_ip` and update all call sites.
- Fix the factually incorrect `NOTICE` in `packages/axum-http-server/src/lib.rs` (lines 65–70): replace the claim that `peer_addr` comes from BEP 15 with an accurate description referencing BEP 3 `ip`.
- Update the parameter table in `packages/axum-http-server/src/lib.rs` from `peer_addr` to `ip`.
- Update sample URLs in documentation and doc-comments that contain `peer_addr=` to use `ip=`.
- Update any tests, fixtures, and the tracker client that construct or parse announce URLs with `peer_addr=`.
- Draft and commit the ADR for the decision to accept only IP addresses (not DNS names) in the `ip` parameter.

### Out of Scope

- Honouring the `ip` parameter value instead of the connection IP (separate issue, sub-issue of #1978).
- Returning a parse error to the client when a DNS name is provided instead of an IP (could be a follow-up; for now silently ignoring remains acceptable once the ADR is in place).
- Any changes to the UDP tracker protocol.
- Any changes to the scrape endpoint.

## ADR: Accept only IP addresses in the HTTP announce `ip` parameter

The following decision record will be committed to `docs/adrs/` as part of this issue.

---

### Title

Accept only IP addresses (not DNS names) in the HTTP announce `ip` GET parameter

### Description

BEP 3 defines the `ip` announce parameter as accepting "IP (or dns name)". The current implementation silently drops any value that cannot be parsed as an `IpAddr`. A decision is needed on whether to support DNS names, resolve them, or explicitly restrict the parameter to IP addresses only.

### Context

The `ip` GET parameter is optional and currently always ignored by the tracker at the service level. Its value is parsed and stored on the `Announce` struct but never forwarded to `peer_from_request`. Even so, a clear policy is needed for what values the tracker accepts in this field.

Three approaches were considered:

| Approach                           | What                                                                                                                       | Pros                                                                             | Cons                                                                                                                                               |
| ---------------------------------- | -------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------- |
| **A — IP only (explicit)**         | Accept only valid `IpAddr` values; return a parse error or silently ignore non-IP values; document the restriction clearly | Simple, predictable, no latency, no DoS risk, consistent with all major trackers | Deviates from the literal BEP 3 spec text                                                                                                          |
| **B — Resolve DNS names**          | Accept DNS names and resolve them to IPs at announce time                                                                  | Closer to BEP 3 literal wording                                                  | Latency per announce, DoS amplification risk (attacker-controlled DNS lookups), complexity, and no known client actually sends hostnames           |
| **C — Accept and store hostnames** | Parse and store hostnames as strings alongside IPs                                                                         | Closest to BEP 3 literal wording                                                 | Incompatible with the `IpAddr`-based peer list model; no client or tracker implements this; no BEP defines how hostnames are returned in responses |

### Evidence from major trackers

- **opentracker**: accepts only IP addresses in `ip`. Has a separate compile-time feature flag (`WANT_IP_FROM_QUERY_STRING`) to optionally use the `ip` value for the peer's address; the type accepted is always an IP.
- **chihaya**: accepts only IP addresses in `ip`.
- **No known tracker** supports DNS name resolution in the announce `ip` parameter.

### Agreement

**Approach A** — accept only IP addresses in the HTTP announce `ip` parameter. Non-IP values (including DNS names) are silently ignored; the tracker falls back to the connection IP. The restriction is documented clearly in the module doc-comment.

This deviates from the literal BEP 3 wording ("or dns name") but matches the de-facto standard across all known tracker implementations. Clients MUST NOT send hostnames in this field when communicating with Torrust Tracker. A future issue may choose to return an explicit parse error for non-IP values instead of silently ignoring them.

### Consequences

- **Positive**: No latency impact on announce handling.
- **Positive**: No DNS-based DoS attack surface.
- **Positive**: Consistent with opentracker, chihaya, and all other known tracker implementations.
- **Positive**: The `IpAddr`-based peer list model is preserved without changes.
- **Negative**: Deviates from the literal BEP 3 spec text ("or dns name"). Mitigated by clear documentation and the fact that no known client sends a hostname.

---

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                                                   | Notes / Expected Output                                                                                                                                                                                                                                                                 |
| --- | ------ | -------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | DONE   | Rename `PEER_ADDR` constant and `"peer_addr"` wire string to `IP` / `"ip"`             | `packages/http-protocol/src/v1/requests/announce.rs`. Also fix the hardcoded `"peer_addr"` literal in the `Display` impl (line 307) to use the renamed `IP` constant instead of a string literal.                                                                                       |
| T2  | DONE   | Rename struct field `peer_addr` → `ip` on `Announce`                                   | Same file; update all construction and match sites. Also fix the doc comment on the `Announce` struct (line 83) which incorrectly claims `peer_addr` is "as per BEP 3" — BEP 3 uses `ip`.                                                                                               |
| T3  | DONE   | Rename `with_peer_addr` → `with_ip` on `AnnounceBuilder`; update `with_default_values` | Same file                                                                                                                                                                                                                                                                               |
| T4  | DONE   | Rename `extract_peer_addr` → `extract_ip`; update call sites                           | Same file                                                                                                                                                                                                                                                                               |
| T5  | DONE   | Update the `NOTICE` and parameter table in `packages/axum-http-server/src/lib.rs`      | Replace incorrect BEP 15 reference with correct BEP 3 `ip` description                                                                                                                                                                                                                  |
| T6  | DONE   | Update sample URLs in doc-comments from `peer_addr=` to `ip=`                          | `packages/axum-http-server/src/lib.rs`, `extractors/announce_request.rs`, `packages/tracker-core/src/torrent/mod.rs`                                                                                                                                                                    |
| T7  | DONE   | Update test fixtures and inline URL strings that use `peer_addr=`                      | `packages/axum-http-server/tests/server/v1/contract/for_all_config_modes/receiving_an_announce_request.rs`, `packages/axum-http-server/tests/server/v1/contract/configured_as_private.rs`, `packages/axum-http-server/src/v1/extractors/announce_request.rs` (inline test query string) |
| T8  | DONE   | Rename `--peer-addr` CLI flag to `--ip` in tracker-client binaries                     | `console/tracker-client/src/console/clients/http/app.rs`, `console/tracker-client/src/console/clients/unified/http.rs`. Also rename `peer_addr` CLI arg struct field and `AnnounceOptions` field to `ip`.                                                                               |
| T9  | DONE   | Update JSON key in tracker-client docs from `peer_addr` to `ip`                        | `console/tracker-client/docs/features/json-request-input/README.md`                                                                                                                                                                                                                     |
| T10 | DONE   | Commit the ADR to `docs/adrs/`                                                         | File: `docs/adrs/20260716000000_accept_only_ip_addresses_in_http_announce_ip_param.md`                                                                                                                                                                                                  |
| T11 | DONE   | Run `cargo test --workspace` — no regressions                                          | All tests pass                                                                                                                                                                                                                                                                          |
| T12 | DONE   | Run `linter all`                                                                       | Must exit `0`                                                                                                                                                                                                                                                                           |
| T13 | DONE   | Rename test function `should_not_fail_when_the_peer_address_param_is_invalid`          | Rename to `should_not_fail_when_the_ip_param_is_invalid` in `packages/axum-http-server/tests/server/v1/contract/for_all_config_modes/receiving_an_announce_request.rs`                                                                                                                  |

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted in `docs/issues/drafts/`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [ ] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [x] Manual verification scenarios executed and recorded (status + evidence)
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [x] Committer verified spec progress is up to date before commit
- [x] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-07-22 00:00 UTC - agent - Verified issue #1985 is CLOSED on GitHub and archived this spec to docs/issues/closed/.

- 2026-07-15 00:00 UTC - Copilot/User - Spec drafted; ADR embedded as a section pending extraction to `docs/adrs/` during implementation.
- 2026-07-16 00:00 UTC - Copilot/User - Spec updated with user feedback (CLI flag renamed to `--ip`; JSON doc key renamed to `ip`; ADR date set to 2026-07-16). Implementation completed. All pre-commit checks pass.
- 2026-07-16 16:16 UTC - Copilot/User - Manual verification M1/M2/M3 executed against local tracker build. All scenarios pass. Evidence recorded in `manual-verification.md`.

## Acceptance Criteria

- [x] AC1: An HTTP announce request using `ip=<address>` is correctly parsed — the `ip` field on the `Announce` struct is populated.
- [x] AC2: An HTTP announce request using the old `peer_addr=<address>` parameter no longer populates the field (the old name is not recognised).
- [x] AC3: The Rust struct field, builder method, extractor function, and constant all use the name `ip` (no remaining `peer_addr` references for the wire parameter). The `Display` impl uses the `IP` constant rather than a hardcoded string literal.
- [x] AC4: The `NOTICE` in `packages/axum-http-server/src/lib.rs` accurately describes the `ip` parameter with a correct BEP 3 reference (no BEP 15 mention for this parameter).
- [x] AC5: All sample URLs in documentation use `ip=` instead of `peer_addr=`.
- [x] AC6: The ADR `docs/adrs/20260716000000_accept_only_ip_addresses_in_http_announce_ip_param.md` is committed.
- [x] AC7: `linter all` exits with code `0`.
- [x] AC8: Relevant tests pass with no regressions.
- [x] Manual verification scenarios are executed and documented (status + evidence).
- [x] Acceptance criteria are re-reviewed after implementation and reflect actual behaviour.
- [x] Documentation is updated when behaviour/workflow changes.

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test --workspace`
- Pre-push checks (when applicable)

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                                                             | Command/Steps                                                                                                        | Expected Result                                                 | Status | Evidence                                                |
| --- | -------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------- | ------ | ------------------------------------------------------- |
| M1  | Announce with `ip=<address>` — field is parsed                       | `curl -s "http://localhost:7070/announce?info_hash=...&peer_id=...&port=6881&ip=2.137.87.41"` and check tracker logs | Tracker logs show `ip` was parsed                               | DONE   | See [manual-verification.md](manual-verification.md#m1) |
| M2  | Announce with old `peer_addr=<address>` — field is ignored           | Replace `ip=` with `peer_addr=` in M1 URL                                                                            | Tracker ignores the parameter (no parse error, field is `None`) | DONE   | See [manual-verification.md](manual-verification.md#m2) |
| M3  | Announce with `ip=hostname.example.com` — non-IP is silently ignored | Use a DNS name as the `ip` value                                                                                     | Field is `None`; no error returned                              | DONE   | See [manual-verification.md](manual-verification.md#m3) |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                                                                                                                 |
| ----- | ---------------------- | ------------------------------------------------------------------------------------------------------------------------ |
| AC1   | DONE                   | Verified by `it_should_extract_the_announce_request_from_the_url_query_params` in `announce_request.rs` test using `ip=` |
| AC2   | DONE                   | `PEER_ADDR` constant removed; `extract_peer_addr` → `extract_ip` reads `IP = "ip"` constant                              |
| AC3   | DONE                   | `grep peer_addr` across protocol/server/client sources returns no wire-param references                                  |
| AC4   | DONE                   | `packages/axum-http-server/src/lib.rs` NOTICE updated to reference BEP 3                                                 |
| AC5   | DONE                   | All sample URLs updated in lib.rs, extractor, torrent/mod.rs, tracker-client docs                                        |
| AC6   | DONE                   | `docs/adrs/20260716000000_accept_only_ip_addresses_in_http_announce_ip_param.md` created                                 |
| AC7   | DONE                   | `linter all` exits `0`                                                                                                   |
| AC8   | DONE                   | All pre-commit checks pass; 0 test failures                                                                              |

## Risks and Trade-offs

- **Breaking wire change**: Clients currently sending `peer_addr=` will have the field silently ignored after this rename. Since BEP 3 specifies `ip=` and no spec-compliant client should be sending `peer_addr=`, this is acceptable. Our own test helpers and tracker client use `peer_addr=` and are updated in scope. However, any downstream users who copied the `peer_addr=` pattern from the tracker's own documentation (which currently shows `peer_addr=` in sample URLs) will experience a silent break. Consider adding a deprecation period where both `peer_addr` and `ip` are accepted, with `peer_addr` emitting a warning, before removing it in a follow-up issue.
- **ADR timing**: The ADR decision (IP-only) reflects current tracker behaviour. No behaviour change is introduced by this issue; the ADR simply makes the policy explicit.

## References

- BEP 3 — The BitTorrent Protocol Specification: <https://www.bittorrent.org/beps/bep_0003.html>
- Related issue (honour `ip` param — sub-issue of #1978): to be created
- Related epic: [#1978 — Configuration Overhaul](../1978-configuration-overhaul-epic/EPIC.md)
- opentracker `WANT_IP_FROM_QUERY_STRING`: <https://erdgeist.org/arts/software/opentracker/>
