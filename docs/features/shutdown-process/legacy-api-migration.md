---
doc-type: feature-supporting-analysis
status: draft
last-updated-utc: "2026-11-10 15:30"
semantic-links:
  related-artifacts:
    - docs/features/shutdown-process/README.md
    - docs/issues/closed/2751-1488-si-18-deprecate-legacy-shutdown-api/ISSUE.md
    - docs/issues/closed/2801-1488-si-19-remove-legacy-shutdown-api/ISSUE.md
---

# Migrating from the Legacy Shutdown API

EPIC [#1488](../../issues/open/1488-overhaul-tracker-shutdown/ISSUE.md) replaces the `Halted` oneshot-channel
shutdown with a `tokio_util::sync::CancellationToken` that the caller injects and cancels. SI-18 (issue #2751)
deprecated the legacy API at the source, and SI-19 (issue #2801) removed it: the symbols in the first column
below no longer exist, and `torrust-server-lib` 0.6.0 removes its own. Use this guide to migrate code that still
calls them.

## What changed

- **Start with a token.** Create one `CancellationToken`, pass it to the `start_with_cancellation` variant, and keep
  the returned `CancellationRunning`.
- **Stop by cancelling.** Cancel the token instead of sending `Halted::Normal`. For the HTTP, REST API and
  health-check servers, join `task` and `shutdown_controller` of the `CancellationRunning`; the controller reports
  the drain outcome. The UDP server differs: its `CancellationRunning` holds only `local_addr` and one receive-loop
  `task`, which returns `Ok(())` after cancellation, so await that task; there is no drain controller.
- **Do not subscribe to signals in a library.** The executable subscribes to SIGINT and SIGTERM and cancels the
  token; `global_shutdown_signal` and `shutdown_signal*` were deprecated in `torrust-server-lib` 0.5.0 and are
  removed in 0.6.0. No server library subscribes to operating-system signals.

## Replacement table

| Removed (SI-19) | Replacement |
| --------------- | ----------- |
| `axum-http-server`: `HttpServer<Stopped>::start`, `start_with_health_check`, `RunningHttpServer` | `start_with_cancellation`, `start_with_cancellation_and_health_check` |
| `axum-http-server`: `Running`, `Running.halt_task`, `HttpServer<Running>::stop` | `CancellationRunning`; cancel the token and join `task` and `shutdown_controller` |
| `axum-rest-api-server`: `ApiServer<Stopped>::start`, `Launcher::start`, `RunningApiServer` | `ApiServer<Stopped>::start_with_cancellation` |
| `axum-rest-api-server`: `Running`, `Running.halt_task`, `Running::new`, `ApiServer<Running>::stop` | `CancellationRunning`; cancel the token and join `task` and `shutdown_controller` |
| `axum-health-check-api-server`: `server::start`, `RunningServer` | `start_with_cancellation`, `CancellationRunning` |
| `udp-server`: `Server<Stopped>::start`, `Launcher::run_with_graceful_shutdown`, `RunningUdpServer` | `Server<Stopped>::start_with_cancellation` |
| `udp-server`: `Running`, `Running.halt_task`, `Server<Running>::stop` | `CancellationRunning`; cancel the token and await its `task` |
| `udp-server`: `Spawner::spawn_launcher`, `LaunchRequest` (and `rx_halt`) | `Server<Stopped>::start_with_cancellation` |
| `axum-server`: `graceful_shutdown` | `graceful_shutdown_on_cancellation` |
| `torrust-tracker`: `HaltSignal`, `NestedServerTask` | a `CancellationToken` and the token-aware shutdown controller |
| `torrust-server-lib`: `Halted`, `shutdown_signal*`, `global_shutdown_signal` | `CancellationToken`, `cancellation_signal` (see the library's `CHANGELOG.md`) |

The five server package crates are unpublished and the root types were never in a published release, so for them
this guide is in-repo history; only `torrust-server-lib` had a support window (0.5.x), which ended when the
maintainer approved 0.6.0 on 2026-11-10 (recorded in the start gate of the [SI-19 spec](../../issues/closed/2801-1488-si-19-remove-legacy-shutdown-api/ISSUE.md), merged in torrust/torrust-tracker#2803).
