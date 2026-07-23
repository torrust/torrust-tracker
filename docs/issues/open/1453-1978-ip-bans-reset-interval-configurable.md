---
doc-type: issue
issue-type: enhancement
status: open
priority: p2
github-issue: 1453
spec-path: docs/issues/open/1453-1978-ip-bans-reset-interval-configurable.md
branch: "1453-ip-bans-reset-interval"
related-pr: null
last-updated-utc: 2026-07-23 17:02
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/configuration/src/v3_0_0/
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

The value must be at least `3600` seconds (one hour). The v3 configuration module must declare
the minimum as the canonical constant (for example,
`UdpTrackerServer::MINIMUM_IP_BANS_RESET_INTERVAL_IN_SECS`) and use it for validation and the
explicit error message. This prevents the documented policy, validation, and error text from
drifting apart. A zero value does not disable cleanup; disabling cleanup is out of scope.

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
- Start the cleanup job only when at least one UDP tracker is configured, and manage it through
  `JobManager` cancellation
- Keep the bootstrap job's current hardcoded 24-hour interval temporarily; #1980 replaces it
  with `udp_tracker_server.ip_bans_reset_interval_in_secs` when it migrates application consumers
  to v3
- Update v3 configuration documentation and tests; defer runtime consumption and tracked default
  configuration files to #1980, which performs the v2-to-v3 migration

### Out of Scope

- Changing the `BanService` implementation itself
- Adding similar config sections for other server types (HTTP, API)
- Per-instance ban configuration

## Implementation Plan

| ID  | Status | Task                                                                       | Notes                                                                                        |
| --- | ------ | -------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------- |
| T1  | TODO   | Add `UdpTrackerServer` config struct with `ip_bans_reset_interval_in_secs` | In `packages/configuration/src/v3_0_0/`; declare the canonical minimum and default constants |
| T2  | TODO   | Add `udp_tracker_server` field to root v3 `Configuration` struct           | Optional with default; do not migrate v2 consumers in this issue                             |
| T3  | TODO   | Reject intervals below the minimum                                         | Error must state the minimum using the canonical constant; add boundary tests                |
| T4  | TODO   | Move ban cleanup task from per-server launcher to bootstrap                | Register one job only when UDP trackers are configured; use `JobManager` cancellation        |
| T5  | TODO   | Preserve the current hardcoded 24-hour bootstrap interval                  | Remove the launcher constant and duplicate task spawn; defer v3 config consumption to #1980  |
| T6  | TODO   | Update v3 docs and tests                                                   | Production default config files and global v3 imports remain #1980                           |
| T7  | TODO   | Run `linter all` and relevant tests                                        |                                                                                              |

## Progress Tracking

### Workflow Checkpoints

- [ ] Spec drafted in `docs/issues/drafts/`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] GitHub issue created and issue number added to this spec
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests)
- [ ] Manual verification scenarios executed and recorded
- [ ] Acceptance criteria reviewed after implementation
- [ ] Issue closed and spec moved to `docs/issues/open/`

### Progress Log

- 2026-07-13 21:00 UTC - josecelano - Initial spec drafted
- 2026-07-23 17:02 UTC - josecelano - Approved the v3-only schema boundary: active
  application consumers and default configuration files remain deferred to #1980. The cleanup
  job starts only when UDP trackers are configured and is cancelled through `JobManager`.
  Added a minimum interval policy of 3600 seconds; validation errors must use the configuration
  type's canonical minimum constant so policy and diagnostics cannot diverge.
- 2026-07-23 17:02 UTC - josecelano - Confirmed staged delivery: #1453 creates and validates the
  v3 setting while fixing duplicate cleanup with the existing hardcoded 24-hour interval. #1980
  will make the setting effective during the application-wide v3 consumer migration. Recorded
  torrust-demo#28 as operational evidence for the 24-hour default.

## Acceptance Criteria

- [ ] AC1: New `[udp_tracker_server]` config section with `ip_bans_reset_interval_in_secs` exists
- [ ] AC2: Default value is `86400` (24 hours)
- [ ] AC2a: Values below `3600` seconds are rejected with an error that states the canonical minimum
- [ ] AC3: Ban cleanup task is spawned exactly once at app bootstrap
- [ ] AC4: No duplicate cleanup tasks when multiple UDP servers are configured
- [ ] AC5: The cleanup job is not started when no UDP trackers are configured and is cancelled by `JobManager`
- [ ] AC6: The bootstrap cleanup job retains the current hardcoded 24-hour interval pending #1980
- [ ] AC7: The v3 configuration documentation and tests cover the section; runtime consumption and v2 consumer/default-config migration remain deferred to #1980
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test --workspace`

### Manual Verification Scenarios

| ID  | Scenario                   | Command/Steps                                                      | Expected Result                                                              | Status | Evidence |
| --- | -------------------------- | ------------------------------------------------------------------ | ---------------------------------------------------------------------------- | ------ | -------- |
| M1  | Verify v3 config parsing   | Load v3 configuration with custom `ip_bans_reset_interval_in_secs` | Configuration retains the configured value; runtime use is deferred to #1980 | TODO   |          |
| M2  | Verify single cleanup task | Run tracker with 2+ UDP servers, check logs for cleanup task count | Only one cleanup task spawned                                                | TODO   |          |
| M3  | Verify default value       | Load v3 config without the new option                              | Configuration defaults to 86400 seconds                                      | TODO   |          |
| M4  | Reject too-short interval  | Load v3 config with a value below 3600 seconds                     | Explicit error states 3600-second minimum                                    | TODO   |          |

### Acceptance Verification

| AC ID | Status | Evidence |
| ----- | ------ | -------- |
| AC1   | TODO   |          |
| AC2   | TODO   |          |
| AC3   | TODO   |          |
| AC4   | TODO   |          |
| AC5   | TODO   |          |
| AC6   | TODO   |          |
| AC7   | TODO   |          |

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
