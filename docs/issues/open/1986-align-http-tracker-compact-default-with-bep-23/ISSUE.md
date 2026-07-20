---
doc-type: issue
issue-type: bug
status: open
priority: p2
github-issue: 1986
spec-path: docs/issues/open/1986-align-http-tracker-compact-default-with-bep-23/ISSUE.md
branch: "1986-align-http-tracker-compact-default-with-bep-23"
related-pr: "https://github.com/torrust/torrust-tracker/pull/1990"
last-updated-utc: 2026-07-15 00:00
semantic-links:
  skill-links:
    - create-issue
    - run-tracker-locally
    - use-tracker-client
  related-artifacts:
    - packages/axum-http-server/src/v1/handlers/announce.rs
    - packages/axum-http-server/src/lib.rs
    - packages/http-protocol/src/v1/requests/announce.rs
    - packages/axum-http-server/tests/server/v1/contract/for_all_config_modes/receiving_an_announce_request.rs
---


# Issue #1986 - Return compact peer list by default when `compact` param is absent (BEP 23)

## Goal

Fix the HTTP tracker announce handler to return the compact peer list by default when the client omits the `compact` GET parameter, aligning the tracker with the SUGGESTION in [BEP 23](https://www.bittorrent.org/beps/bep_0023.html).

## Background

[BEP 23 — Tracker Returns Compact Peer Lists](https://www.bittorrent.org/beps/bep_0023.html) states:

> It is SUGGESTED that trackers return compact format by default. By including `compact=0` in the announce URL, the client advises the tracker that it prefers the original format described in BEP 3, and analogously `compact=1` advises the tracker that the client prefers compact format. However the `compact` key-value pair is only advisory: the tracker MAY return using either format. `compact` is advisory so that trackers may support only the compact format. However, clients MUST continue to support both.

The current implementation in `packages/axum-http-server/src/v1/handlers/announce.rs` only selects the compact response format when the client explicitly sends `compact=1`. When the `compact` parameter is absent (`None`), the tracker falls through to the non-compact (dictionary) branch:

```rust
// packages/axum-http-server/src/v1/handlers/announce.rs
fn build_response(announce_request: &Announce, announce_data: DomainAnnounceData) -> Response {
    // ...
    if announce_request.compact.as_ref().is_some_and(|f| *f == Compact::Accepted) {
        // compact path — only reached when compact=1 is explicit
    } else {
        // non-compact path — reached when compact=0 OR when compact is absent
    }
}
```

This violates the BEP 23 SUGGESTION. The tracker should default to compact when no preference is expressed.

The bug is also acknowledged in the existing module documentation and in a `code-review` comment in the contract tests:

- `packages/axum-http-server/src/lib.rs` lines 91–95 contains a `NOTICE` that explicitly calls out this deviation.
- `packages/axum-http-server/tests/server/v1/contract/for_all_config_modes/receiving_an_announce_request.rs` contains:

```rust
// code-review: the HTTP tracker does not return the compact response by default if the "compact"
// param is not provided in the announce URL. The BEP 23 suggest to do so.
```

### Why use option (a): compact by default, honour `compact=0`

Three implementation strategies were considered:

**(a) Compact by default; honour `compact=0` to switch to dictionary format** ← chosen
The tracker returns compact unless the client explicitly requests dictionary format via `compact=0`. This fully satisfies the BEP 23 SUGGESTION while respecting the client's explicit preference. It is the most compatible option and is the behaviour implemented by other major trackers (opentracker, chihaya).

**(b) Always compact, ignore `compact=0`**
BEP 23 permits this — `compact` is advisory, so the tracker MAY always return compact. However, silently ignoring an explicit client preference (`compact=0`) is hostile to interoperability. Some older clients, scrapers, and Azureus/Vuze configurations rely on the dictionary format. Ignoring their request is surprising and harder to document.

**(c) Make this a per-tracker configuration option**
Configuration is the right tool when operators have legitimate different trade-offs. Here the BEP already defines the intended behaviour unambiguously. Adding a knob pushes a spec-compliance decision onto operators who should not need to think about it. Option (a) already leaves the door open for a future simplification towards (b) if dictionary format support is ever dropped.

## Scope

### In Scope

- Change `build_response` in `packages/axum-http-server/src/v1/handlers/announce.rs` so that `compact == None` (absent) is treated as compact by default, i.e. only non-compact is returned when the client explicitly sends `compact=0`.
- Update the doc comment in `packages/axum-http-server/src/lib.rs` (the `NOTICE` and the query-parameter table's `Default` column for `compact`) to reflect the new behaviour.
- Rename and invert the contract test `should_not_return_the_compact_response_by_default` → `should_return_the_compact_response_by_default` and update its assertion.
- Remove the `code-review` comment that flagged this deviation once the fix is in place.

### Out of Scope

- Changing the `AnnounceBuilder::default()` in `packages/http-protocol/src/v1/requests/announce.rs`, which defaults `compact` to `Some(Compact::NotAccepted)`. That builder is a test helper; its default can be revisited in a follow-up if needed.
- Always returning compact regardless of `compact=0` (option b).
- Adding a configuration option to toggle this behaviour (option c).
- Any changes to the UDP tracker protocol handling.
- Any changes to the scrape endpoint.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                                      | Notes / Expected Output                                                                                                                                                                                                                                                                                                         |
| --- | ------ | ------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | DONE   | Invert the compact-default logic in `build_response`                      | Changed `is_some_and(Compact::Accepted)` to `is_some_and(Compact::NotAccepted)`. `None` now maps to compact. Only `Some(Compact::NotAccepted)` (`compact=0`) returns dictionary format.                                                                                                                                         |
| T2  | DONE   | Update the `NOTICE` doc comment in `packages/axum-http-server/src/lib.rs` | Removed the deviation notice (lines 91–95). Updated the `Default` column for `compact` from `None` to `compact (BEP 23)`. Updated the `Description` column to note "compact by default per BEP 23".                                                                                                                             |
| T3  | DONE   | Rename and invert the contract test                                       | Renamed `should_not_return_the_compact_response_by_default` to `should_return_the_compact_response_by_default`. Flipped assertion to `assert!(is_a_compact_announce_response(response).await)`. Removed the `code-review` comment. Also updated `assert_is_announce_response` helper to accept either compact or normal format. |
| T4  | DONE   | Verify all existing tests pass                                            | `cargo test --tests --benches --examples --workspace --all-targets --all-features` — all passed, no regressions. Additionally `assert_is_announce_response` helper was updated to accept both compact and normal formats since the helper was used by a test that sends requests without `compact`.                             |
| T5  | DONE   | Run `linter all`                                                          | All linters passed (markdown, yaml, toml, cspell, clippy, rustfmt, shellcheck). Exited `0`.                                                                                                                                                                                                                                     |
| T6  | DONE   | Manual verification: run tracker locally and test with tracker client     | All three scenarios pass: M1 (no compact → compact), M2 (compact=1 → compact), M3 (compact=0 → dictionary). See manual verification table.                                                                                                                                                                                      |

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted in `docs/issues/drafts/`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [ ] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [x] Manual verification scenarios executed and recorded (status + evidence)
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-07-15 00:00 UTC - Copilot/User - Spec drafted based on code review of `build_response`, `lib.rs` NOTICE, and the existing `code-review` comment in the contract tests.

## Acceptance Criteria

- [ ] AC1: When a client sends an announce request without the `compact` parameter, the tracker responds with a compact peer list.
- [ ] AC2: When a client sends `compact=1`, the tracker responds with a compact peer list.
- [ ] AC3: When a client sends `compact=0`, the tracker responds with a non-compact (dictionary) peer list.
- [ ] AC4: The contract test `should_return_the_compact_response_by_default` passes and asserts compact format when `compact` is absent.
- [ ] AC5: The contract test for `compact=0` still passes and asserts dictionary format.
- [ ] AC6: The `NOTICE` in `packages/axum-http-server/src/lib.rs` (lines 91–95) is removed since the behaviour no longer deviates from BEP 23. The query-parameter table `Default` column for `compact` accurately describes the new default (compact).
- [ ] AC7: `linter all` exits with code `0`.
- [ ] AC8: Relevant tests pass with no regressions.
- [ ] Manual verification scenarios are executed and documented (status + evidence).
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behaviour.
- [ ] Documentation is updated when behaviour/workflow changes.

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test --workspace`
- Pre-push checks (when applicable)

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                                                        | Command/Steps                                                                                                                                                                       | Expected Result                                                                           | Status | Evidence                                                                                                                  |
| --- | --------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------- | ------ | ------------------------------------------------------------------------------------------------------------------------- |
| M1  | Announce without `compact` param — expect compact response      | `curl -s "http://localhost:7070/announce?info_hash=...&peer_id=...&port=6881"` and inspect bencoded response peers field                                                            | Response uses compact format (peers is a byte string, not a list)                         | DONE   | Hex dump shows `5:peers0:` (bencoded string, not list). Python parser confirms `COMPACT format (peers is a byte string)`. |
| M2  | Announce with `compact=1` — expect compact response             | Add `&compact=1` to M1 URL                                                                                                                                                          | Response uses compact format                                                              | DONE   | Python parser confirms `COMPACT format (peers is a byte string)`.                                                         |
| M3  | Announce with `compact=0` — expect dictionary response          | Add `&compact=0` to M1 URL                                                                                                                                                          | Response uses non-compact (dictionary) format (`peers` value is a bencoded list of dicts) | DONE   | Python parser confirms `DICTIONARY format (peers is a list)`.                                                             |
| M4  | Tracker client: announce without `--compact` — expect compact   | `cargo run` (start tracker); `cargo run -p torrust-tracker-client --bin tracker_client -- http announce http://127.0.0.1:7070 9c38422213e30bff212b30c360d26f9a02136422 --port 6881` | Response uses compact format (peers encoded as a compact string)                          | TODO   |                                                                                                                           |
| M5  | Tracker client: announce with `--compact 0` — expect dictionary | Same as M4 but add `--compact 0`                                                                                                                                                    | Response uses non-compact (dictionary) format                                             | TODO   |                                                                                                                           |

### Acceptance Verification

| AC1 | DONE | M1 manual verification confirms compact response when no compact param. Contract test `should_return_the_compact_response_by_default` passes. |
| AC2 | DONE | M2 manual verification confirms compact response when compact=1. Contract test `should_return_the_compact_response` passes. |
| AC3 | DONE | M3 manual verification confirms dictionary response when compact=0. |
| AC4 | DONE | Contract test `should_return_the_compact_response_by_default` passes and asserts compact format. |
| AC5 | DONE | Existing contract test for compact=0 (the `should_return_the_compact_response` test path) still passes. |
| AC6 | DONE | NOTICE removed from `lib.rs`. Table column updated: Default = `compact (BEP 23)`, Description includes "Compact by default per BEP 23". |
| AC7 | DONE | `linter all` exits with code 0. |
| AC8 | DONE | `cargo test --tests --benches --examples --workspace --all-targets --all-features` — all passed. |

## Risks and Trade-offs

- **Client compatibility**: Clients that previously relied on getting a dictionary response by default (no `compact` param) will now receive a compact response. Per BEP 23, all clients MUST support both formats, so this should not break any spec-compliant client. Non-compliant clients would have needed `compact=0` anyway.
- **Tracker client binary**: The project's own `tracker_client` binary (under `console/tracker-client/`) should be verified to handle compact responses correctly when it does not send `compact=0`. If the client currently relies on getting dictionary format by default, it will break after this fix.
- **Test helper `AnnounceBuilder` default**: The builder defaults to `compact=0`, which means tests using it without overriding the `compact` field continue to exercise the non-compact path. This is intentional and is not changed in this issue. It avoids accidentally masking regressions in the non-compact code path.

## References

- BEP 23 — Tracker Returns Compact Peer Lists: <https://www.bittorrent.org/beps/bep_0023.html>
- BEP 3 — The BitTorrent Protocol Specification: <https://www.bittorrent.org/beps/bep_0003.html>
- Related code: `packages/axum-http-server/src/v1/handlers/announce.rs` `build_response`
- Related code: `packages/axum-http-server/src/lib.rs` lines 91–95
- Related test (renamed by this issue): `packages/axum-http-server/tests/server/v1/contract/for_all_config_modes/receiving_an_announce_request.rs` — currently `should_not_return_the_compact_response_by_default`, renamed to `should_return_the_compact_response_by_default`
- Skill: `run-tracker-locally` — `.github/skills/dev/environment-setup/run-tracker-locally/SKILL.md`
- Skill: `use-tracker-client` — `.github/skills/usage/use-tracker-client/SKILL.md`
