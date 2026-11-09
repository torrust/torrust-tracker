---
doc-type: feature-supporting-analysis
status: draft
last-updated-utc: "2026-11-09 19:31"
semantic-links:
  related-artifacts:
    - docs/features/shutdown-process/README.md
    - docs/issues/open/2751-1488-si-18-deprecate-legacy-shutdown-api/ISSUE.md
---

# Migrating from the Legacy Shutdown API

EPIC [#1488](../../issues/open/1488-overhaul-tracker-shutdown/ISSUE.md) replaces the `Halted` oneshot-channel
shutdown with a `tokio_util::sync::CancellationToken` that the caller injects and cancels. SI-18 (issue #2751)
deprecates the legacy API at the source; nothing is removed and behavior is unchanged. SI-19 removes it.

## What changed

- **Start with a token.** Create one `CancellationToken`, pass it to the `start_with_cancellation` variant, and keep
  the returned `CancellationRunning`.
- **Stop by cancelling.** Cancel the token instead of sending `Halted::Normal`, then join `task` and
  `shutdown_controller` of the `CancellationRunning`; the controller reports the drain outcome.
- **Do not subscribe to signals in a library.** The executable subscribes to SIGINT and SIGTERM and cancels the
  token; `global_shutdown_signal` and `shutdown_signal*` are deprecated in `torrust-server-lib` 0.5.0.

## Replacement table

| Deprecated | Replacement |
| ---------- | ----------- |
| `axum-http-server`: `HttpServer<Stopped>::start`, `start_with_health_check` | `start_with_cancellation`, `start_with_cancellation_and_health_check` |
| `axum-http-server`: `Running`, `Running.halt_task`, `HttpServer<Running>::stop` | `CancellationRunning`; cancel the token and join its tasks |
| `axum-rest-api-server`: `ApiServer<Stopped>::start`, `Launcher::start` | `ApiServer<Stopped>::start_with_cancellation` |
| `axum-rest-api-server`: `Running`, `Running.halt_task`, `Running::new`, `ApiServer<Running>::stop` | `CancellationRunning`; cancel the token and join its tasks |
| `axum-health-check-api-server`: `server::start`, `RunningServer` | `start_with_cancellation`, `CancellationRunning` |
| `udp-server`: `Server<Stopped>::start`, `Launcher::run_with_graceful_shutdown` | `Server<Stopped>::start_with_cancellation` |
| `udp-server`: `Running`, `Running.halt_task`, `Server<Running>::stop` | `CancellationRunning`; cancel the token and join its tasks |
| `udp-server`: `Spawner::spawn_launcher`, `LaunchRequest` (and `rx_halt`) | `Server<Stopped>::start_with_cancellation` |
| `axum-server`: `graceful_shutdown` | `graceful_shutdown_on_cancellation` |
| `torrust-tracker`: `HaltSignal`, `NestedServerTask` | a `CancellationToken` and the token-aware shutdown controller |
| `torrust-server-lib`: `Halted`, `shutdown_signal*`, `global_shutdown_signal` | `CancellationToken`, `cancellation_signal` (see the library's `CHANGELOG.md`) |

Each deprecation message names its replacement and says the removal is planned for the next breaking release of
the package. The five server package crates are unpublished and the root types are not in the published 3.0.0, so
these notes are in-repo guidance; only the `torrust-server-lib` deprecation has a support window (0.5.x).
