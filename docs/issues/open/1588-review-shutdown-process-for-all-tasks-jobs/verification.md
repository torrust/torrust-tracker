# Verification Evidence — Issue #1588: Shutdown Task Inventory

> **Status**: Final implementation-time inventory validation passed.

## Environment

- Date: 2026-09-08
- OS: Linux
- Tracker branch: `1588-review-shutdown-process`
- Baseline: `ccc1a7c4` (`JoinSet` supervision merged by PR #2164)

## Inventory and Ownership Evidence

The revalidated [task inventory](../../../features/shutdown-process/task-inventory.md)
is the complete current ownership record. It distinguishes direct `JoinSet`
components, the intentionally narrow legacy periodic-job registry,
component-owned children, framework-owned work, and still-detached controller
or request work.

`JobManager::spawn` directly owns named top-level component futures and records
completed, cancelled, failed, panicked, and deadline-aborted outcomes under one
shared deadline. It does not own component children. The only pre-spawned
handles retained in `legacy_jobs` are `torrent_cleanup`,
`peers_inactivity_update`, and `udp_ban_cleanup`; all share the deadline and
are joined after either normal completion or escalation.

The inventory includes all seven event-listener categories. In particular,
`tracker_core_persistent_completed_statistics_event_listener` is conditional on
enabled persistent completed statistics and valid persistence; it was missing
from the preliminary map.

## Confirmed Gaps and Roadmap Coverage

| Implementation-time observation                                                                                                                 | Roadmap owner          | Status of this review                  |
| ----------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------- | -------------------------------------- |
| `main.rs` handles Unix SIGTERM at the executable boundary.                                                                                      | SI-1                   | Complete; not an open gap.             |
| Direct top-level supervisor ownership and shared deadline are in `JoinSet`; nested handles remain component-owned.                              | #1586                  | Complete supervisor boundary.          |
| Torrent cleanup listens for Ctrl-C directly.                                                                                                    | SI-4                   | Pending.                               |
| Activity metrics updater listens for Ctrl-C directly.                                                                                           | SI-5                   | Pending.                               |
| Server libraries retain `Halted` and `global_shutdown_signal` behavior.                                                                         | SI-2, then SI-18/SI-19 | Pending migration/deprecation/removal. |
| HTTP and REST Axum drain controller handles are discarded; their 90/95-second drain budget conflicts with the process-wide ten-second deadline. | SI-10, SI-11, SI-12    | Pending.                               |
| Health-check owns and joins its current controller, but still uses the legacy bridge and does not become unready before draining.               | SI-13, SI-21           | Pending.                               |
| UDP receive-loop shutdown aborts the loop; active request processors retain only abort handles and no joined outcomes.                          | SI-14, SI-15           | Pending.                               |
| Standalone HTTP and UDP examples remain Ctrl-C-based.                                                                                           | SI-16, SI-17           | Pending.                               |
| `main.rs` does not map supervisor outcomes to an exit code or a configured deadline policy.                                                     | SI-20                  | Pending.                               |

No evidence identified a missing independently releasable migration slice.
The active #1488 roadmap therefore needs no change.

## Manual SIGTERM Verification

- [x] Built the tracker binary with `cargo build --bin torrust-tracker`.
- [x] Launched `./target/debug/torrust-tracker` directly, redirecting its log to
      git-ignored `.tmp/1588-sigterm.log`.
- [x] Waited for `Tracker shutdown signal handlers installed.`.
- [x] Verified the signal target before signalling: PID `3139887`, command
      `./target/debug/torrust-tracker`.
- [x] Sent `SIGTERM` to that binary PID, rather than to a shell or `cargo run`
      wrapper.

Observed facts:

- The process logged `Torrust tracker shutting down (SIGTERM) ...` and
  `Waiting for job to finish under the shared shutdown deadline`.
- Direct listeners and server/API components reported cooperative cancellation;
  `udp_ban_cleanup` completed normally.
- Listener ports `6969`, `7070`, and `1212` had no `ss` result after shutdown,
  and no `target/debug/torrust-tracker` process remained.
- `torrent_cleanup` and `peers_inactivity_update` were logged as aborted after
  the ten-second deadline, as expected while SI-4 and SI-5 remain pending.
- The direct binary exited with status `0` and logged `Torrust tracker
successfully shutdown.` This does not complete SI-20, which owns the final
  outcome-to-exit-code policy.

## Validation

| Command                                                                      | Result                                                                                                              |
| ---------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------- |
| `cargo build --bin torrust-tracker`                                          | Pass; emitted pre-existing Cargo manifest naming/readme warnings.                                                   |
| Direct-binary SIGTERM scenario above                                         | Pass; coordinator logs, port closure, and process exit recorded.                                                    |
| `linter all`                                                                 | Pass.                                                                                                               |
| `git diff --check`                                                           | Pass.                                                                                                               |
| `TORRUST_GIT_HOOKS_LOG_DIR=.tmp ./contrib/dev-tools/git/hooks/pre-commit.sh` | Pass; dictionary formatting, cargo machete, layer-boundary bans, linters, hadolint, and documentation tests passed. |
