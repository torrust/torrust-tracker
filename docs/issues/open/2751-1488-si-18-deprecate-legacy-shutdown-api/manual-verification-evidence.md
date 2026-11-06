---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2751-1488-si-18-deprecate-legacy-shutdown-api/ISSUE.md
last-updated-utc: "2026-11-06 14:00"
---

# Manual Verification Evidence: Deprecate Legacy Shutdown API

## V1: Eligibility gate

Checked 2026-11-06 against `torrust/develop` at `da8bd15d1`.

- Application components: `src/bootstrap/jobs/{http_tracker,tracker_apis,health_check_api,udp_tracker}.rs`
  call `start_with_cancellation`.
- Standalone environments and examples: SI-16 (#2412), SI-17 (#2448).
- REST API and health-check API test environments: SI-23 (#2449), SI-24 (#2450).
- Task inventory: `docs/features/shutdown-process/task-inventory.md`, finding 4.
- Q5: `docs/features/shutdown-process/questions.md` (Resolved) and
  `docs/containers.md#stopping-the-tracker`.
- External crate: `torrust-server-lib` 0.5.0 published on 2026-11-06 (merge commit 0ad8ca0 of
  torrust-server-lib PR 4); `Halted`, `shutdown_signal`, `shutdown_signal_with_message` and
  `global_shutdown_signal` deprecated since 0.5.0, removal planned for 0.6.0.
