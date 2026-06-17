---
doc-type: issue
issue-type: bug
status: planned
priority: p1
github-issue: 1507
spec-path: docs/issues/open/1507-review-localhost-peer-ip.md
branch: "1507-review-localhost-peer-ip"
related-pr: null
last-updated-utc: 2026-06-17 18:05
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/adrs/20260617093046_reject_wildcard_external_ip.md
    - packages/tracker-core/src/announce_handler.rs
    - packages/configuration/src/v2_0_0/network.rs
    - packages/configuration/src/v2_0_0/core.rs
    - share/default/config/
---

<!-- skill-link: create-issue -->

# Issue #1507 - Review IP assigned to localhost peers

## Goal

Fix the peer IP assignment bug where the unspecified address `0.0.0.0` is returned for localhost peers, and prevent silent misconfiguration by rejecting wildcard addresses as invalid for the `external_ip` config option.

## Background

When running the tracker locally and announcing with a loopback IP (`127.0.0.1`), the `assign_ip_address_to_peer` function replaces the client's loopback address with the configured `external_ip`. However, the default value for `external_ip` is `Some(Ipv4Addr::UNSPECIFIED)` (`0.0.0.0`), which is the wildcard/unspecified address. This means peers in announce responses get `0.0.0.0` as their IP — useless for contacting them.

The current algorithm:

```mermaid
flowchart TD
    A[Client announces] --> B{Client IP is loopback?}
    B -->|No| C[Use client's actual IP]
    B -->|Yes| D{external_ip configured?}
    D -->|Yes, Some&#40;ip&#41;| E[Use external_ip]
    D -->|None| F[Use loopback IP]
```

The gap is that `Some(0.0.0.0)` is treated the same as a properly configured public IP, producing broken peer addresses.

### Root cause chain

1. `external_ip` defaults to `Some(Ipv4Addr::UNSPECIFIED)` → `0.0.0.0`
2. `assign_ip_address_to_peer` sees the client is loopback (`127.0.0.1`) and replaces it with the tracker's `external_ip`
3. Result: peers get `0.0.0.0` instead of their actual `127.0.0.1` address

### Why this needs a breaking change

Wildcard addresses (`0.0.0.0`, `::`) are **never valid external IPs**. The current code silently accepts them, which:

- Breaks loopback/LAN peers silently when `external_ip` is left at the default
- Masks operator misconfiguration (explicitly setting `0.0.0.0`)
- Only manifests at runtime when someone tries to connect to a LAN peer

Since a new major version is coming, this is the right time to:

1. Change the default to `None` (no external IP = no loopback replacement)
2. Add validation to reject wildcard addresses with a clear startup error

> **Note on config schema version**: The config file format (`schema_version = "2.0.0"`) stays unchanged. No fields are added, removed, renamed, or changed in type. This is a **behavioral** breaking change — operators who explicitly set `external_ip = "0.0.0.0"` will get a startup validation error — not a config **schema** breaking change. The config version is bumped only for structural changes (field additions/removals/renames, type changes, TOML restructuring).

### Code review: how `external_ip` is used

A thorough codebase investigation confirmed that `external_ip` has a **single purpose**: it is only used as input to `assign_ip_address_to_peer()` in the announce handler.

| Usage                      | File                                                                                                                        | Purpose                                                                     |
| -------------------------- | --------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------- |
| **Config field + default** | [`packages/configuration/src/v2_0_0/network.rs`](../../packages/configuration/src/v2_0_0/network.rs)                        | Struct field definition & `default_external_ip()` returning `Some(0.0.0.0)` |
| **Config getter**          | [`packages/configuration/src/v2_0_0/mod.rs`](../../packages/configuration/src/v2_0_0/mod.rs#L301)                           | `get_ext_ip()` helper                                                       |
| **Single call site**       | [`packages/tracker-core/src/announce_handler.rs`](../../packages/tracker-core/src/announce_handler.rs#L166)                 | `assign_ip_address_to_peer(remote_client_ip, self.config.net.external_ip)`  |
| **Function definition**    | [`packages/tracker-core/src/announce_handler.rs`](../../packages/tracker-core/src/announce_handler.rs#L265)                 | Loopback → external IP replacement logic                                    |
| **Test helper**            | [`packages/test-helpers/src/configuration.rs`](../../packages/test-helpers/src/configuration.rs#L145)                       | `ephemeral_with_external_ip()`                                              |
| **Unit tests**             | [`packages/tracker-core/src/announce_handler.rs`](../../packages/tracker-core/src/announce_handler.rs#L355)                 | 8 tests covering loopback/IPv4/IPv6 combinations                            |
| **Integration tests**      | [`packages/axum-http-server/tests/server/v1/contract.rs`](../../packages/axum-http-server/tests/server/v1/contract.rs#L902) | HTTP tracker: IPv4 + IPv6 loopback scenarios                                |
| **Integration tests**      | [`packages/udp-server/src/handlers/announce.rs`](../../packages/udp-server/src/handlers/announce.rs#L491)                   | UDP server: peer IP replaced with external IP                               |

No other code path reads `external_ip`. It is not used for server binding, health checks, API responses, scrape responses, or any other runtime behavior. This means changing the default and adding validation is **safe** — there is zero risk of side effects beyond the announce-handler code path.

The fix:

```mermaid
flowchart TD
    A[Client announces] --> B{Client IP is loopback?}
    B -->|No| C[Use client's actual IP]
    B -->|Yes| D{external_ip configured?}
    D -->|None| F[Use loopback IP]
    D -->|Yes, valid IP| E[Use external_ip]
```

## Scope

### In Scope

- Add config validation to reject `0.0.0.0` / `::` as invalid `external_ip` values (ADR required)
- Change the default value of `external_ip` from `Some(0.0.0.0)` to `None`
- Update `assign_ip_address_to_peer` documentation (logic already handles `None` correctly)
- Add/update unit tests for the new behavior
- Update the ADR index and add a new ADR documenting this decision
- Update doc example in `src/lib.rs` that shows `external_ip = "0.0.0.0"`

### Out of Scope

- Adding a separate config option for "LAN peer public IP"
- Changing the general model of loopback IP replacement (it is correct for properly-configured deployments)
- Updating integration tests (existing ones use explicit external IPs only and should not be affected)

## Testing Requirements

Every code path affected by this change must be covered by tests. Prefer **unit tests** at the appropriate level. If a scenario cannot be tested in isolation with a unit test, use integration tests or end-to-end tests as a fallback, and document why the unit test was not feasible.

| Scenario                                                | Existing tests                                  | Action                                                          |
| ------------------------------------------------------- | ----------------------------------------------- | --------------------------------------------------------------- |
| Loopback peer with `external_ip = None`                 | Unit tests exist for `None` (keeps `127.0.0.1`) | Verify they still pass after default change                     |
| Loopback peer with `external_ip = Some(valid_ip)`       | Unit tests + integration tests exist            | No change needed                                                |
| Loopback peer with `external_ip = Some(0.0.0.0)` (IPv4) | **No tests** — this is the buggy case           | Add unit test: `assign_ip_address_to_peer` with `Some(0.0.0.0)` |
| Loopback peer with `external_ip = Some(::)` (IPv6)      | **No tests** — this is the buggy case           | Add unit test: `assign_ip_address_to_peer` with `Some(::)`      |
| Non-loopback peer with any `external_ip`                | Unit tests exist                                | No change needed                                                |
| Config validation rejects `0.0.0.0`                     | **No tests** — new feature                      | Add unit test for config validation                             |
| Config validation rejects `::`                          | **No tests** — new feature                      | Add unit test for config validation                             |
| Config validation accepts valid IP                      | **No tests** — new feature                      | Add unit test for config validation                             |

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                     | Notes / Expected Output                                                 |
| --- | ------ | -------------------------------------------------------- | ----------------------------------------------------------------------- |
| T1  | DONE   | Draft ADR for rejecting wildcard external_ip             | `docs/adrs/20260617093046_reject_wildcard_external_ip.md`               |
| T2  | TODO   | Change default `external_ip` to `None`                   | `default_external_ip()` returns `None`                                  |
| T3  | TODO   | Add config validation to reject unspecified addresses    | Reject `0.0.0.0` and `::` with a clear startup error                    |
| T4  | TODO   | Update `assign_ip_address_to_peer` docs                  | Document that unspecified is rejected at config level                   |
| T5  | TODO   | Add unit tests for config validation                     | Validate `0.0.0.0` rejection, `::` rejection, valid IP acceptance       |
| T6  | TODO   | Add unit test for `assign_ip_address_to_peer` edge cases | Test with `Some(0.0.0.0)` and `Some(::)` — should treat as unconfigured |
| T7  | TODO   | Verify existing unit tests still pass                    | Confirm tests for `None`, valid `Some(ip)`, and non-loopback cases      |
| T8  | TODO   | Update doc example in `src/lib.rs`                       | Line 193: remove `external_ip = "0.0.0.0"` from the example             |
| T9  | TODO   | Run linter and tests                                     | `linter all`, `cargo test --workspace`                                  |

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted in `docs/issues/open/` (this document)
- [x] ADR drafted and added to ADR index
- [ ] Spec and ADR reviewed and approved by user/maintainer
- [ ] Spec committed to branch
- [ ] ADR committed to branch
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, pre-push checks)
- [ ] Manual verification scenarios executed and recorded (status + evidence)
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-06-17 17:55 UTC - GitHub Copilot - Initial spec drafted
- 2026-06-17 18:05 UTC - GitHub Copilot - Expanded scope: config validation, breaking change, ADR
- 2026-06-17 18:30 UTC - GitHub Copilot - Added testing requirements table, forward ref to `src/lib.rs` doc example

## Acceptance Criteria

- [ ] AC1: The default `external_ip` is `None` (no config, no replacement)
- [ ] AC2: Config validation rejects `0.0.0.0` and `::` as `external_ip` values with a clear error
- [ ] AC3: Peers announced from a loopback IP get the configured `external_ip` when it is a valid public IP
- [ ] AC4: Peers announced from a loopback IP keep `127.0.0.1` when `external_ip` is `None`
- [ ] AC5: Peers announced from a non-loopback IP always get their real IP regardless of `external_ip`
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass (including new config validation tests)
- [ ] Manual verification scenarios are executed and documented (status + evidence)
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] ADR is linked from this spec and added to the ADR index

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test -p torrust-tracker-core` (unit tests for `assign_ip_address_to_peer`)
- `cargo test -p torrust-tracker-configuration` (config validation tests)
- `cargo test --workspace` (full suite)

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                                   | Command/Steps                                                                                                                                                                    | Expected Result                                                   | Status | Evidence |
| --- | ------------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------- | ------ | -------- |
| M1  | Run tracker locally and announce over HTTP | 1. `cargo run` (starts tracker with default config)<br>2. `cargo run --bin tracker_client -- http announce http://127.0.0.1:7070 443c7602b4fde83d1154d6d9da48808418b181b6 \| jq` | Peer IP is `127.0.0.1`, not `0.0.0.0`                             | TODO   |          |
| M2  | Run tracker locally and announce over UDP  | 1. `cargo run`<br>2. `cargo run --bin tracker_client -- udp announce udp://127.0.0.1:6969 443c7602b4fde83d1154d6d9da48808418b181b6 \| jq`                                        | Peer IP is `127.0.0.1`, not `0.0.0.0`                             | TODO   |          |
| M3  | Invalid config rejected                    | 1. Create a config with `external_ip = "0.0.0.0"`<br>2. Start tracker with that config                                                                                           | Tracker fails to start with clear error about invalid external_ip | TODO   |          |
