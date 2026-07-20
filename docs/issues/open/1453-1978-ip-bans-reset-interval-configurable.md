---
doc-type: issue
issue-type: enhancement
status: open
priority: p2
github-issue: 1453
spec-path: docs/issues/open/1453-1978-ip-bans-reset-interval-configurable.md
branch: "1453-ip-bans-reset-interval"
related-pr: null
last-updated-utc: 2026-07-13 21:00
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

> **EPIC position**: Subissue #6 of 9. Independent — new `[udp_tracker_server]` section with no overlap. Can run in parallel with #1415, #1490, #889.

## Goal

Add a new `[udp_tracker_server]` configuration section with an `ip_bans_reset_interval_in_secs` option, and fix the duplicate spawning of the ban cleanup task (one per UDP server instead of once globally).

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
- Move ban cleanup task spawning from per-UDP-server launcher to main app bootstrap
- Ensure only one cleanup task runs regardless of the number of UDP servers
- Update default config examples

### Out of Scope

- Changing the `BanService` implementation itself
- Adding similar config sections for other server types (HTTP, API)
- Per-instance ban configuration

## Implementation Plan

| ID  | Status | Task                                                                       | Notes                                                |
| --- | ------ | -------------------------------------------------------------------------- | ---------------------------------------------------- |
| T1  | TODO   | Add `UdpTrackerServer` config struct with `ip_bans_reset_interval_in_secs` | In `packages/configuration/src/v3_0_0/`              |
| T2  | TODO   | Add `udp_tracker_server` field to root `Configuration` struct              | Optional with default                                |
| T3  | TODO   | Move ban cleanup task from per-server launcher to bootstrap                | Spawn once in `src/bootstrap/` or `src/container.rs` |
| T4  | TODO   | Read interval from config instead of hardcoded constant                    |                                                      |
| T5  | TODO   | Update default config files with new section                               |                                                      |
| T6  | TODO   | Run `linter all` and tests                                                 |                                                      |

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

## Acceptance Criteria

- [ ] AC1: New `[udp_tracker_server]` config section with `ip_bans_reset_interval_in_secs` exists
- [ ] AC2: Default value is `86400` (24 hours)
- [ ] AC3: Ban cleanup task is spawned exactly once at app bootstrap
- [ ] AC4: No duplicate cleanup tasks when multiple UDP servers are configured
- [ ] AC5: Default config files include the new section
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test --workspace`

### Manual Verification Scenarios

| ID  | Scenario                   | Command/Steps                                                      | Expected Result                  | Status | Evidence |
| --- | -------------------------- | ------------------------------------------------------------------ | -------------------------------- | ------ | -------- |
| M1  | Verify config parsing      | Run tracker with custom `ip_bans_reset_interval_in_secs` in config | Tracker starts, interval is read | TODO   |          |
| M2  | Verify single cleanup task | Run tracker with 2+ UDP servers, check logs for cleanup task count | Only one cleanup task spawned    | TODO   |          |
| M3  | Verify default value       | Run tracker without the new config option                          | Uses default 86400 interval      | TODO   |          |

### Acceptance Verification

| AC ID | Status | Evidence |
| ----- | ------ | -------- |
| AC1   | TODO   |          |
| AC2   | TODO   |          |
| AC3   | TODO   |          |
| AC4   | TODO   |          |
| AC5   | TODO   |          |

## Risks and Trade-offs

- **New config section**: Adding `[udp_tracker_server]` is a breaking change for config file format. Mitigation: the field is optional with a sensible default.
- **Bootstrap refactoring**: Moving the cleanup task requires understanding the app bootstrap flow. Mitigation: keep the change minimal — just move the spawn call.

## References

- Related issues: #1444, #1452
- Related: `packages/udp-core/src/services/banning.rs`
- Related: `packages/udp-server/src/server/launcher.rs`
