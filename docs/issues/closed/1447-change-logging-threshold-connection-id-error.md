---
doc-type: issue
issue-type: task
status: done
priority: p3
github-issue: 1447
spec-path: docs/issues/closed/1447-change-logging-threshold-connection-id-error.md
branch: "1447-change-logging-threshold-connection-id-error"
related-pr: null
last-updated-utc: 2026-07-15
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/udp-server/src/handlers/error.rs
---


# Issue #1447 - Change the logging threshold for connection ID error to `WARNING`

## Goal

Change the log level for UDP connection ID errors from `ERROR` to `WARNING` to reduce
log noise in production deployments (especially the Torrust Tracker demo).

## Background

The UDP tracker receives a high volume of requests with invalid connection IDs from
misconfigured or abusive peers. These produce errors like:

- `cookie value is expired`
- `cookie value is from future`

These are currently logged at `ERROR` level, which floods the logs and makes it hard to
identify other types of errors.

The tracker already bans IPs that make too many such requests (tracked via the
`udp_tracker_server_connection_id_errors_total` metric and the ban service), so the
logging can safely be downgraded. A `WARNING` level is still appropriate because there
is no other monitoring/analytics tool to detect unusual patterns — the log remains the
primary observability channel for connection ID issues.

This is not an application error — it is expected behaviour from bad client traffic.

## Scope

### In Scope

- Change the `tracing::error!` call in `log_error()` in `packages/udp-server/src/handlers/error.rs` to `tracing::warn!`
- Verify that the change does not break any tests that assert on log level or output
- Run `linter all` and the full test suite

### Out of Scope

- Adding a configuration option for the log level (not configurable for now)
- Changing log levels for other error types
- Changing the banning behaviour (stays at `ERROR`-level events)
- Adding separate monitoring/analytics tooling

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                    | Notes / Expected Output                                                                                                                         |
| --- | ------ | --------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | TODO   | Change log level in `handlers/error.rs` | Make `log_error()` inspect the error type: use `tracing::warn!` for `ConnectionCookie` errors, keep `tracing::error!` for all other error types |
| T2  | TODO   | Run verification                        | `linter all`, `cargo test --workspace`, pre-commit checks                                                                                       |

## Technical Details

The `ServerError` type in `packages/udp-server/src/error.rs` has several variants.
Connection cookie errors flow through two paths:

- `Error::AnnounceFailed { source: UdpAnnounceError::ConnectionCookieError { .. } }`
- `Error::ScrapeFailed { source: UdpScrapeError::ConnectionCookieError { .. } }`

The current `log_error()` function in `packages/udp-server/src/handlers/error.rs` is called for **all** UDP error types, not just connection cookie errors:

```rust
fn log_error(
    error: &Error,
    client_socket_addr: SocketAddr,
    server_socket_addr: SocketAddr,
    opt_transaction_id: Option<TransactionId>,
    request_id: Uuid,
) {
    match opt_transaction_id {
        Some(transaction_id) => {
            let transaction_id = transaction_id.0.to_string();
            tracing::error!(target: UDP_TRACKER_LOG_TARGET, error = %error, %client_socket_addr, %server_socket_addr, %request_id, %transaction_id, "response error");
        }
        None => {
            tracing::error!(target: UDP_TRACKER_LOG_TARGET, error = %error, %client_socket_addr, %server_socket_addr, %request_id, "response error");
        }
    }
}
```

The implementation should inspect the error variant and use `tracing::warn!` for
`ConnectionCookie` errors while keeping `tracing::error!` for other error types
(invalid requests, announce/scrape errors, internal errors, etc.).

The `Error` type derives `Clone` and can be pattern-matched. Matching on
`matches!(error, Error::AnnounceFailed { source: UdpAnnounceError::ConnectionCookieError { .. } })`
or similar approach.

Note: The `ErrorKind::ConnectionCookie` variant is specifically handled by the banning
event handler (`packages/udp-server/src/banning/event/handler.rs`) to track IP bans
separately — this behaviour is unaffected by the log level change.

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted in `docs/issues/open/`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue exists and issue number matches spec
- [x] Implementation completed (PR #1975 merged)
- [x] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [x] Manual verification scenarios executed and recorded (status + evidence)
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [x] Reviewer validated acceptance criteria and updated checkboxes
- [x] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-07-13 12:00 UTC - Copilot - Spec draft created
- 2026-07-13 18:33 UTC - PR #1975 merged - Implementation completed
- 2026-07-15 UTC - Spec archived to `docs/issues/closed/`
