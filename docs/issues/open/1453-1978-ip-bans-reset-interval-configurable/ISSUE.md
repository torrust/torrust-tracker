---
doc-type: issue
issue-type: enhancement
status: in_review
priority: p2
github-issue: 1453
spec-path: docs/issues/open/1453-1978-ip-bans-reset-interval-configurable/ISSUE.md
branch: "1453-ip-bans-reset-interval"
related-pr: null
last-updated-utc: 2026-07-24 15:59
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/configuration/src/v3_0_0/
    - packages/configuration/src/v3_0_0/types.rs
    - docs/adrs/20260723184019_separate_configuration_value_invariants_from_consistency_validation.md
    - docs/application-jobs.md
    - docs/issues/open/1453-1978-ip-bans-reset-interval-configurable/evidence/
    - packages/udp-core/src/services/banning.rs
    - packages/udp-server/src/server/launcher.rs
    - src/bootstrap/jobs/
---

# Issue #1453 - Allow setting the IP bans reset interval via configuration and remove duplicate execution of cronjob to clean bans

> **EPIC position**: Subissue #6 of 12. Independent — new `[udp_tracker_server]` section with no overlap. Can run in parallel with #1415, #1490, #889.

## Goal

Add a new `[udp_tracker_server]` configuration section with an `ip_bans_reset_interval_in_secs` option, and fix the duplicate spawning of the ban cleanup task (one per UDP server instead of once globally). The new v3 setting becomes effective when #1980 migrates application consumers to v3.

## Background

The tracker has a `BanService` (in `packages/udp-core/src/services/banning.rs`) that bans client IPs sending many requests with the wrong connection ID. There are two problems:

### Task 1: Hardcoded interval

The ban cleanup interval is hardcoded. There is no configuration section for settings that apply to the UDP tracker server as a whole (as opposed to per-instance settings like `bind_address` or `cookie_lifetime`).

Proposed new config section:

```toml
[udp_tracker_server]
ip_bans_reset_interval_in_secs = 3600
```

Default value: `86400` (24 hours).

The 24-hour default is based on production observations. The tracker demo experiment in
[torrust-demo#28](https://github.com/torrust/torrust-demo/issues/28) first increased the ban
duration from two minutes to one hour after sustained invalid-connection-ID traffic. The duration
was subsequently increased to 24 hours because many clients continued sending requests without a
valid connection ID. Future changes to the default or minimum should be supported by comparable
operational evidence.

The value must be at least `3600` seconds (one hour). It is a single-value domain invariant, so
the v3 configuration module must encode it in the typed `IpBansResetIntervalInSecs` newtype,
backed by the reusable `AtLeastU64<MINIMUM_IP_BANS_RESET_INTERVAL_IN_SECS>` lower-bound type,
and reject invalid values while constructing or deserializing it. This prevents the documented
policy and validation from drifting apart. A zero value does not disable cleanup; disabling
cleanup is out of scope. See ADR `20260723184019` for the validation-layer boundary.

### Task 2: Duplicate cleanup task

Every time the tracker starts a new UDP server, it spawns a new task to reset the bans:

```rust
tokio::spawn(async move {
    let mut cleaner_interval = interval(Duration::from_secs(IP_BANS_RESET_INTERVAL_IN_SECS));
    cleaner_interval.tick().await;
    loop {
        cleaner_interval.tick().await;
        ban_cleaner.write().await.reset_bans();
    }
});
```

Since all UDP servers are launched simultaneously at startup, the bans are being reset N times (once per UDP server) instead of once. This is a bug — the cleanup should be spawned once at the main app bootstrapping level.

## Scope

### In Scope

- Add `[udp_tracker_server]` config section with `ip_bans_reset_interval_in_secs: u64` field
- Default value: `86400` (24 hours)
- Reject values below the canonical minimum of `3600` seconds with an explicit validation error
- Move ban cleanup task spawning from per-UDP-server launcher to main app bootstrap
- Ensure only one cleanup task runs regardless of the number of UDP servers
- Start the UDP service group only when at least one UDP tracker is configured and the tracker is
  not private; manage its cleanup job through `JobManager` cancellation
- Temporarily use `UdpTrackerServer::DEFAULT_IP_BANS_RESET_INTERVAL_IN_SECS` in the bootstrap
  job; #1980 replaces it with `udp_tracker_server.ip_bans_reset_interval_in_secs` when it
  migrates application consumers to v3
- Update v3 configuration documentation and tests; defer runtime consumption and tracked default
  configuration files to #1980, which performs the v2-to-v3 migration

### Out of Scope

- Changing the `BanService` implementation itself
- Adding similar config sections for other server types (HTTP, API)
- Per-instance ban configuration

## Implementation Plan

| ID  | Status | Task                                                                       | Notes                                                                                                                                            |
| --- | ------ | -------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------ |
| T1  | DONE   | Add `UdpTrackerServer` config struct with `ip_bans_reset_interval_in_secs` | `udp_tracker_server.rs` defines canonical minimum/default constants                                                                              |
| T2  | DONE   | Add `udp_tracker_server` field to root v3 `Configuration` struct           | Defaults through `UdpTrackerServer::default`; v2 consumers unchanged                                                                             |
| T3  | DONE   | Reject intervals below the minimum                                         | `IpBansResetIntervalInSecs` newtype uses the canonical minimum; boundary tests added                                                             |
| T4  | DONE   | Move ban cleanup task from per-server launcher to bootstrap                | One configuration-gated UDP service group owns the cancellation-managed cleanup job                                                              |
| T5  | DONE   | Preserve the current 24-hour bootstrap interval                            | Uses `UdpTrackerServer::DEFAULT_IP_BANS_RESET_INTERVAL_IN_SECS`; #1980 enables config reading                                                    |
| T6  | DONE   | Update v3 docs and tests                                                   | V3 module docs, configuration serialization, and focused job-condition tests updated                                                             |
| T7  | DONE   | Run `linter all` and relevant tests                                        | `linter all`, focused tests, and formatting passed; the optional workspace-wide cognitive-complexity check is blocked by unrelated existing code |

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted in `docs/issues/drafts/`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, formatting, and focused tests)
- [x] Manual verification scenarios executed and recorded
- [x] Acceptance criteria reviewed after implementation
- [ ] Issue closed and spec moved to `docs/issues/open/`

### Progress Log

- 2026-07-13 21:00 UTC - josecelano - Initial spec drafted
- 2026-07-23 17:02 UTC - josecelano - Approved the v3-only schema boundary: active
  application consumers and default configuration files remain deferred to #1980. The cleanup
  job starts only when UDP trackers are configured and is cancelled through `JobManager`.
  Added a minimum interval policy of 3600 seconds; the newtype validation must use the
  configuration type's canonical minimum constant so policy and diagnostics cannot diverge.
- 2026-07-23 17:02 UTC - josecelano - Confirmed staged delivery: #1453 creates and validates the
  v3 setting while fixing duplicate cleanup with the existing hardcoded 24-hour interval. #1980
  will make the setting effective during the application-wide v3 consumer migration. Recorded
  torrust-demo#28 as operational evidence for the 24-hour default.
- 2026-07-23 17:02 UTC - agent - Implemented the approved staged delivery. Added the validated
  v3 `UdpTrackerServer` configuration section; moved IP-ban cleanup from each UDP launcher into
  one cancellation-managed bootstrap job; and retained the v3 type's canonical 24-hour default
  constant until #1980 enables configured runtime consumption. Focused tests passed; ready for
  maintainer review.
- 2026-07-23 18:40 UTC - josecelano - Replaced the single-field use of semantic validation with
  the reusable `AtLeastU64` value type and the domain newtype `IpBansResetIntervalInSecs`. Added
  ADR `20260723184019` to distinguish value invariants, cross-field consistency validation, and
  runtime/environment validation. The `validator` module has a code-review marker for a future
  coordinated rename of its ambiguous public API.
- 2026-07-23 18:49 UTC - agent - Verified the implementation with `cargo fmt --check`, focused
  configuration/application/UDP-server tests, and `linter all`. The optional workspace-wide
  cognitive-complexity check remains blocked by pre-existing violations in
  `swarm-coordination-registry`, outside this issue's scope.
- 2026-07-24 00:00 UTC - josecelano - Documented the current job ownership and lifecycle model
  in `docs/application-jobs.md`. #1453 is the concrete example of an application-owned cleanup
  job for a service shared across UDP instances; the final supervision design remains #1488.
- 2026-07-24 15:59 UTC - agent - Recorded M2 manual runtime evidence in
  [`evidence/2026-07-24-manual-runtime-verification.md`](evidence/2026-07-24-manual-runtime-verification.md).
  Two UDP listeners started locally and produced one cleanup-job start log entry.

## Acceptance Criteria

- [x] AC1: New `[udp_tracker_server]` config section with `ip_bans_reset_interval_in_secs` exists
- [x] AC2: Default value is `86400` (24 hours)
- [x] AC2a: Values below `3600` seconds are rejected with an error that states the canonical minimum
- [x] AC3: Ban cleanup task is spawned exactly once at app bootstrap
- [x] AC4: No duplicate cleanup tasks when multiple UDP servers are configured
- [x] AC5: UDP jobs, including cleanup, are not started when no UDP listeners are configured or the tracker is private; cleanup is cancelled by `JobManager`
- [x] AC6: The bootstrap cleanup job uses `UdpTrackerServer::DEFAULT_IP_BANS_RESET_INTERVAL_IN_SECS` pending #1980
- [x] AC7: The v3 configuration documentation and tests cover the section; runtime consumption and v2 consumer/default-config migration remain deferred to #1980
- [x] `linter all` exits with code `0`
- [x] Relevant focused tests pass

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test --workspace`

### Manual Verification Scenarios

| ID  | Scenario                   | Command/Steps                                                      | Expected Result                                                              | Status | Evidence                                                                                          |
| --- | -------------------------- | ------------------------------------------------------------------ | ---------------------------------------------------------------------------- | ------ | ------------------------------------------------------------------------------------------------- |
| M1  | Verify v3 config parsing   | Load v3 configuration with custom `ip_bans_reset_interval_in_secs` | Configuration retains the configured value; runtime use is deferred to #1980 | TODO   | Deferred: runtime does not consume v3 until #1980                                                 |
| M2  | Verify single cleanup task | Run tracker with 2+ UDP servers, check logs for cleanup task count | Only one cleanup task spawned                                                | DONE   | [`2026-07-24-manual-runtime-verification.md`](evidence/2026-07-24-manual-runtime-verification.md) |
| M3  | Verify default value       | Load v3 config without the new option                              | Configuration defaults to 86400 seconds                                      | DONE   | `cargo test -p torrust-tracker-configuration`                                                     |
| M4  | Reject too-short interval  | Load v3 config with a value below 3600 seconds                     | Explicit error states 3600-second minimum                                    | DONE   | `cargo test -p torrust-tracker-configuration`                                                     |

### Acceptance Verification

| AC ID | Status | Evidence                                                                                                                                  |
| ----- | ------ | ----------------------------------------------------------------------------------------------------------------------------------------- |
| AC1   | DONE   | `v3_0_0::udp_tracker_server::UdpTrackerServer`                                                                                            |
| AC2   | DONE   | Default-configuration serialization and unit test                                                                                         |
| AC2a  | DONE   | `IpBansResetIntervalInSecs` boundary tests assert the explicit 3600-second error                                                          |
| AC3   | DONE   | One bootstrap registration; [M2 runtime evidence](evidence/2026-07-24-manual-runtime-verification.md)                                     |
| AC4   | DONE   | Two UDP listeners produced one cleanup job; [M2 runtime evidence](evidence/2026-07-24-manual-runtime-verification.md)                     |
| AC5   | DONE   | UDP service-group condition tests cover no configured listeners and private mode; cleanup uses the shared `JobManager` cancellation token |
| AC6   | DONE   | Bootstrap job reads `UdpTrackerServer::DEFAULT_IP_BANS_RESET_INTERVAL_IN_SECS`                                                            |
| AC7   | DONE   | Docs, ADR, and focused tests updated; #1980 owns runtime configuration consumption                                                        |

## Risks and Trade-offs

- **New config section**: Adding `[udp_tracker_server]` is a breaking change for config file format. Mitigation: the field is optional with a sensible default.
- **Bootstrap refactoring**: Moving the cleanup task requires understanding the app bootstrap flow. Mitigation: keep the change minimal — just move the spawn call.
- **Configuration migration boundary**: Global aliases and tracked default configurations still use v2. Mitigation: restrict this issue to self-contained v3 schema work and defer consumer migration to #1980.
- **Duration policy**: A shorter interval can allow invalid clients to resume sooner. Mitigation: retain the evidence-based 24-hour runtime interval and reconsider the v3 default only with operational data.

## References

- Related issues: #1444, #1452
- Related: `packages/udp-core/src/services/banning.rs`
- Related: `packages/udp-server/src/server/launcher.rs`
- Operational evidence: [torrust-demo#28](https://github.com/torrust/torrust-demo/issues/28) — experiment increasing the ban duration from two minutes to one hour; follow-up investigation [torrust-demo#29](https://github.com/torrust/torrust-demo/issues/29)
