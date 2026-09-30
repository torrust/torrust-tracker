---
doc-type: log-capture
issue-spec: docs/issues/open/2387-review-tracker-lifecycle-logs/ISSUE.md
last-updated-utc: "2026-09-30 11:03"
---

# Baseline Lifecycle Logs

## Purpose

Verbatim startup and shutdown logs of the current tracker, captured before any change, as the
baseline for the lifecycle logging review. Later captures are compared against this one.

## Capture

- Date and time (UTC): 2026-09-30 10:48.
- Artifact: release build of `torrust-tracker` at the head of PR #2382 (issue #2370), which includes
  the SI-15 UDP drain logs; `rustc 1.101.0-nightly (c1070d693 2026-09-28)`; Linux x86_64.
- Configuration: `share/default/config/tracker.development.sqlite3.toml`, unchanged. It runs every
  service (two UDP trackers, two HTTP trackers, the REST API, and the health-check API) with
  `trace_filter = "info"` and usage statistics enabled.
- Steps:
  1. `./target/release/torrust-tracker --config-toml-path share/default/config/tracker.development.sqlite3.toml`
  2. Polled `curl -fsS http://127.0.0.1:1313/health_check` until it succeeded, then waited one second.
  3. Sent `SIGTERM` to the tracker PID and waited, bounded, for it to exit.
- Result: exit status `0`; 215 log lines. ANSI color codes were stripped; nothing else was edited.
  The configuration dump already masks the API access token (`***`).

The log has three parts:

| Lines | Part | Notes |
| --- | --- | --- |
| 1-165 | Startup | From logging initialization to the shutdown signal handlers being installed |
| 166-173 | Readiness request | The single health-check request used to confirm readiness; request logs, not lifecycle logs |
| 174-215 | Shutdown | From `SIGTERM` to the final success message |

This configuration emits no `WARN` lines. The eight startup `Ignoring ... event from an unknown or
metrics-disabled listener` warnings quoted in the issue specification appear only when usage
statistics are disabled, as in the issue #2370 benchmark configuration.

## Startup (Lines 1-165)

```text
2026-09-30T10:48:24.470017Z  INFO torrust_tracker_configuration::v3_0_0::logging: Logging initialized
2026-09-30T10:48:24.470062Z  INFO torrust_tracker_lib::bootstrap::app: Configuration:
{
  "metadata": {
    "app": "torrust-tracker",
    "purpose": "configuration",
    "schema_version": "3.0.0"
  },
  "logging": {
    "trace_filter": "info",
    "trace_style": "full"
  },
  "core": {
    "announce_policy": {
      "interval": 120,
      "interval_min": 120,
      "max_peers_per_announce": 74
    },
    "database": {
      "driver": "sqlite3",
      "path": "./storage/tracker/lib/database/sqlite3.db"
    },
    "inactive_peer_cleanup_interval": 120,
    "listed": false,
    "private": false,
    "private_mode": null,
    "tracker_policy": {
      "max_peer_timeout": 60,
      "persistent_torrent_completed_stat": true,
      "remove_peerless_torrents": true
    },
    "tracker_usage_statistics": true
  },
  "udp_trackers": [
    {
      "bind_address": "0.0.0.0:6868",
      "cookie_lifetime": {
        "secs": 120,
        "nanos": 0
      },
      "tracker_usage_statistics": true,
      "public_url": null,
      "network": {
        "external_ip": null,
        "on_reverse_proxy": false,
        "ipv6_v6only": false
      }
    },
    {
      "bind_address": "0.0.0.0:6969",
      "cookie_lifetime": {
        "secs": 120,
        "nanos": 0
      },
      "tracker_usage_statistics": true,
      "public_url": null,
      "network": {
        "external_ip": null,
        "on_reverse_proxy": false,
        "ipv6_v6only": false
      }
    }
  ],
  "http_trackers": [
    {
      "bind_address": "0.0.0.0:7070",
      "tls_config": null,
      "tracker_usage_statistics": true,
      "use_ip_from_query_string": false,
      "public_url": null,
      "network": {
        "external_ip": null,
        "on_reverse_proxy": false,
        "ipv6_v6only": false
      }
    },
    {
      "bind_address": "0.0.0.0:7171",
      "tls_config": null,
      "tracker_usage_statistics": true,
      "use_ip_from_query_string": false,
      "public_url": null,
      "network": {
        "external_ip": null,
        "on_reverse_proxy": false,
        "ipv6_v6only": false
      }
    }
  ],
  "udp_tracker_server": {
    "ip_bans_reset_interval_in_secs": 86400,
    "max_connection_id_errors_per_ip": 10,
    "connection_id_validation": "strict"
  },
  "http_api": {
    "bind_address": "0.0.0.0:1212",
    "tls_config": null,
    "access_tokens": {
      "admin": "***"
    },
    "public_url": null
  },
  "health_check_api": {
    "bind_address": "127.0.0.1:1313"
  }
}
2026-09-30T10:48:24.475180Z  INFO initialize: METRICS: type="counter" name="swarm_coordination_registry_torrents_added_total" unit=Some(Count) description=Some(MetricDescription("The total number of torrents added."))
2026-09-30T10:48:24.475198Z  INFO initialize: METRICS: type="counter" name="swarm_coordination_registry_torrents_removed_total" unit=Some(Count) description=Some(MetricDescription("The total number of torrents removed."))
2026-09-30T10:48:24.475203Z  INFO initialize: METRICS: type="gauge" name="swarm_coordination_registry_torrents_total" unit=Some(Count) description=Some(MetricDescription("The total number of torrents."))
2026-09-30T10:48:24.475207Z  INFO initialize: METRICS: type="counter" name="swarm_coordination_registry_torrents_downloads_total" unit=Some(Count) description=Some(MetricDescription("The total number of torrent downloads."))
2026-09-30T10:48:24.475210Z  INFO initialize: METRICS: type="gauge" name="swarm_coordination_registry_torrents_inactive_total" unit=Some(Count) description=Some(MetricDescription("The total number of inactive torrents."))
2026-09-30T10:48:24.475219Z  INFO initialize: METRICS: type="counter" name="swarm_coordination_registry_peers_added_total" unit=Some(Count) description=Some(MetricDescription("The total number of peers added."))
2026-09-30T10:48:24.475225Z  INFO initialize: METRICS: type="counter" name="swarm_coordination_registry_peers_removed_total" unit=Some(Count) description=Some(MetricDescription("The total number of peers removed."))
2026-09-30T10:48:24.475229Z  INFO initialize: METRICS: type="counter" name="swarm_coordination_registry_peers_updated_total" unit=Some(Count) description=Some(MetricDescription("The total number of peers updated."))
2026-09-30T10:48:24.475234Z  INFO initialize: METRICS: type="gauge" name="swarm_coordination_registry_peer_connections_total" unit=Some(Count) description=Some(MetricDescription("The total number of peer connections (one connection per torrent)."))
2026-09-30T10:48:24.475239Z  INFO initialize: METRICS: type="gauge" name="swarm_coordination_registry_unique_peers_total" unit=Some(Count) description=Some(MetricDescription("The total number of unique peers."))
2026-09-30T10:48:24.475244Z  INFO initialize: METRICS: type="gauge" name="swarm_coordination_registry_peers_inactive_total" unit=Some(Count) description=Some(MetricDescription("The total number of inactive peers."))
2026-09-30T10:48:24.475248Z  INFO initialize: METRICS: type="counter" name="swarm_coordination_registry_peers_completed_state_reverted_total" unit=Some(Count) description=Some(MetricDescription("The total number of peers whose completed state was reverted."))
2026-09-30T10:48:24.476097Z  INFO initialize: METRICS: type="counter" name="tracker_core_persistent_torrents_downloads_total" unit=Some(Count) description=Some(MetricDescription("Deprecated: use tracker_core_in_session_torrents_downloads_total or tracker_core_persisted_torrents_downloads_total. This counter is process-local unless persisted completed statistics are enabled."))
2026-09-30T10:48:24.476107Z  INFO initialize: METRICS: type="counter" name="tracker_core_in_session_torrents_downloads_total" unit=Some(Count) description=Some(MetricDescription("The number of torrent downloads completed since this tracker process started."))
2026-09-30T10:48:24.476112Z  INFO initialize: METRICS: type="counter" name="tracker_core_persisted_torrents_downloads_total" unit=Some(Count) description=Some(MetricDescription("The number of torrent downloads restored from and maintained in persistent storage."))
2026-09-30T10:48:24.481248Z  INFO initialize: METRICS: type="counter" name="http_tracker_core_requests_received_total" unit=Some(Count) description=Some(MetricDescription("Total number of HTTP requests received"))
2026-09-30T10:48:24.487748Z  INFO initialize: METRICS: type="counter" name="udp_tracker_core_requests_received_total" unit=Some(Count) description=Some(MetricDescription("Total number of UDP requests received"))
2026-09-30T10:48:24.493952Z  INFO initialize: METRICS: type="counter" name="udp_tracker_server_requests_aborted_total" unit=Some(Count) description=Some(MetricDescription("Total number of UDP requests aborted"))
2026-09-30T10:48:24.493960Z  INFO initialize: METRICS: type="counter" name="udp_tracker_server_requests_discarded_total" unit=Some(Count) description=Some(MetricDescription("Total number of UDP requests discarded before processing (e.g. client source port is 0)"))
2026-09-30T10:48:24.493965Z  INFO initialize: METRICS: type="counter" name="udp_tracker_server_requests_banned_total" unit=Some(Count) description=Some(MetricDescription("Total number of UDP requests banned"))
2026-09-30T10:48:24.493969Z  INFO initialize: METRICS: type="gauge" name="udp_tracker_server_ips_banned_total" unit=Some(Count) description=Some(MetricDescription("Total number of IPs banned from UDP requests"))
2026-09-30T10:48:24.493973Z  INFO initialize: METRICS: type="counter" name="udp_tracker_server_connection_id_errors_total" unit=Some(Count) description=Some(MetricDescription("Total number of requests with connection ID errors"))
2026-09-30T10:48:24.493977Z  INFO initialize: METRICS: type="counter" name="udp_tracker_server_requests_received_total" unit=Some(Count) description=Some(MetricDescription("Total number of UDP requests received"))
2026-09-30T10:48:24.493984Z  INFO initialize: METRICS: type="counter" name="udp_tracker_server_requests_accepted_total" unit=Some(Count) description=Some(MetricDescription("Total number of UDP requests accepted"))
2026-09-30T10:48:24.493988Z  INFO initialize: METRICS: type="counter" name="udp_tracker_server_responses_sent_total" unit=Some(Count) description=Some(MetricDescription("Total number of UDP responses sent"))
2026-09-30T10:48:24.493996Z  INFO initialize: METRICS: type="counter" name="udp_tracker_server_errors_total" unit=Some(Count) description=Some(MetricDescription("Total number of errors processing UDP requests"))
2026-09-30T10:48:24.494001Z  INFO initialize: METRICS: type="gauge" name="udp_tracker_server_performance_avg_processing_time_ns" unit=Some(Nanoseconds) description=Some(MetricDescription("Average time to process a UDP request in nanoseconds"))
2026-09-30T10:48:24.494005Z  INFO initialize: METRICS: type="counter" name="udp_tracker_server_performance_avg_processed_requests_total" unit=Some(Count) description=Some(MetricDescription("Total number of UDP requests processed for the average performance metrics"))
2026-09-30T10:48:24.494166Z  INFO SWARM_COORDINATION_REGISTRY: Starting swarm coordination registry event listener
2026-09-30T10:48:24.494168Z  INFO TRACKER_CORE: Starting tracker core in-memory statistics event listener
2026-09-30T10:48:24.494169Z  INFO complete_startup:start_job{connection_id_validation=Strict cancellation_token=CancellationToken { is_cancelled: false } service_role="udp_tracker" instance_index=0}: torrust_tracker_lib::bootstrap::jobs::udp_tracker: Starting UDP tracker instance bind_address=0.0.0.0:6868 tracker_usage_statistics=true
2026-09-30T10:48:24.494211Z  INFO UDP TRACKER: Starting UDP tracker server event listener (banning)
2026-09-30T10:48:24.494190Z  INFO UDP TRACKER: Starting UDP tracker server event listener
2026-09-30T10:48:24.494193Z  INFO HTTP TRACKER: Starting HTTP tracker core event listener
2026-09-30T10:48:24.494196Z  INFO UDP TRACKER: Starting UDP tracker core event listener
2026-09-30T10:48:24.494205Z  INFO UDP TRACKER: Starting UDP IP-ban cleanup job reset_interval_in_secs=86400
2026-09-30T10:48:24.494178Z  INFO TRACKER_CORE: Starting tracker core persistent completed statistics event listener
2026-09-30T10:48:24.494257Z  INFO complete_startup:start_job{connection_id_validation=Strict cancellation_token=CancellationToken { is_cancelled: false } service_role="udp_tracker" instance_index=0}:start_with_cancellation{cookie_lifetime=120s connection_id_validation=Strict service_role="udp_tracker" instance_index=0}: UDP TRACKER: Starting on: 0.0.0.0:6868
2026-09-30T10:48:24.494324Z  INFO complete_startup:start_job{connection_id_validation=Strict cancellation_token=CancellationToken { is_cancelled: false } service_role="udp_tracker" instance_index=0}:start_with_cancellation{cookie_lifetime=120s connection_id_validation=Strict service_role="udp_tracker" instance_index=0}: UDP TRACKER: Started on: udp://0.0.0.0:6868
2026-09-30T10:48:24.494334Z  INFO complete_startup:start_job{connection_id_validation=Strict cancellation_token=CancellationToken { is_cancelled: false } service_role="udp_tracker" instance_index=0}:start_with_cancellation{cookie_lifetime=120s connection_id_validation=Strict service_role="udp_tracker" instance_index=0}: UDP TRACKER: Started UDP tracker service_binding=udp://0.0.0.0:6868
2026-09-30T10:48:24.494355Z  INFO complete_startup:start_job{connection_id_validation=Strict cancellation_token=CancellationToken { is_cancelled: false } service_role="udp_tracker" instance_index=1}: torrust_tracker_lib::bootstrap::jobs::udp_tracker: Starting UDP tracker instance bind_address=0.0.0.0:6969 tracker_usage_statistics=true
2026-09-30T10:48:24.494373Z  INFO complete_startup:start_job{connection_id_validation=Strict cancellation_token=CancellationToken { is_cancelled: false } service_role="udp_tracker" instance_index=1}:start_with_cancellation{cookie_lifetime=120s connection_id_validation=Strict service_role="udp_tracker" instance_index=1}: UDP TRACKER: Starting on: 0.0.0.0:6969
2026-09-30T10:48:24.494382Z  INFO complete_startup:start_job{connection_id_validation=Strict cancellation_token=CancellationToken { is_cancelled: false } service_role="udp_tracker" instance_index=1}:start_with_cancellation{cookie_lifetime=120s connection_id_validation=Strict service_role="udp_tracker" instance_index=1}: UDP TRACKER: Started on: udp://0.0.0.0:6969
2026-09-30T10:48:24.494389Z  INFO complete_startup:start_job{connection_id_validation=Strict cancellation_token=CancellationToken { is_cancelled: false } service_role="udp_tracker" instance_index=1}:start_with_cancellation{cookie_lifetime=120s connection_id_validation=Strict service_role="udp_tracker" instance_index=1}: UDP TRACKER: Started UDP tracker service_binding=udp://0.0.0.0:6969
2026-09-30T10:48:24.494398Z  INFO complete_startup:start_job{version=V1 cancellation_token=CancellationToken { is_cancelled: false } service_role="http_tracker" instance_index=0}: torrust_tracker_lib::bootstrap::jobs::http_tracker: Starting HTTP tracker instance bind_address=0.0.0.0:7070 tracker_usage_statistics=true
2026-09-30T10:48:24.494482Z  INFO complete_startup:start_job{version=V1 cancellation_token=CancellationToken { is_cancelled: false } service_role="http_tracker" instance_index=0}:start_v1{cancellation_token=CancellationToken { is_cancelled: false } service_role="http_tracker" instance_index=0}: HTTP TRACKER: Starting on: http://0.0.0.0:7070
2026-09-30T10:48:24.494489Z  INFO complete_startup:start_job{version=V1 cancellation_token=CancellationToken { is_cancelled: false } service_role="http_tracker" instance_index=0}:start_v1{cancellation_token=CancellationToken { is_cancelled: false } service_role="http_tracker" instance_index=0}: HTTP TRACKER: Started on: http://0.0.0.0:7070
2026-09-30T10:48:24.494499Z  INFO complete_startup:start_job{version=V1 cancellation_token=CancellationToken { is_cancelled: false } service_role="http_tracker" instance_index=0}:start_v1{cancellation_token=CancellationToken { is_cancelled: false } service_role="http_tracker" instance_index=0}: torrust_tracker_axum_http_server::server: Started HTTP tracker service_binding=http://0.0.0.0:7070/
2026-09-30T10:48:24.494511Z  INFO complete_startup:start_job{version=V1 cancellation_token=CancellationToken { is_cancelled: false } service_role="http_tracker" instance_index=1}: torrust_tracker_lib::bootstrap::jobs::http_tracker: Starting HTTP tracker instance bind_address=0.0.0.0:7171 tracker_usage_statistics=true
2026-09-30T10:48:24.494552Z  INFO complete_startup:start_job{version=V1 cancellation_token=CancellationToken { is_cancelled: false } service_role="http_tracker" instance_index=1}:start_v1{cancellation_token=CancellationToken { is_cancelled: false } service_role="http_tracker" instance_index=1}: HTTP TRACKER: Starting on: http://0.0.0.0:7171
2026-09-30T10:48:24.494558Z  INFO complete_startup:start_job{version=V1 cancellation_token=CancellationToken { is_cancelled: false } service_role="http_tracker" instance_index=1}:start_v1{cancellation_token=CancellationToken { is_cancelled: false } service_role="http_tracker" instance_index=1}: HTTP TRACKER: Started on: http://0.0.0.0:7171
2026-09-30T10:48:24.494564Z  INFO complete_startup:start_job{version=V1 cancellation_token=CancellationToken { is_cancelled: false } service_role="http_tracker" instance_index=1}:start_v1{cancellation_token=CancellationToken { is_cancelled: false } service_role="http_tracker" instance_index=1}: torrust_tracker_axum_http_server::server: Started HTTP tracker service_binding=http://0.0.0.0:7171/
2026-09-30T10:48:24.494684Z  INFO complete_startup:start_job{version=V1 cancellation_token=CancellationToken { is_cancelled: false } service_role="tracker_rest_api" instance_index=0}:start_v1{cancellation_token=CancellationToken { is_cancelled: false } service_role="tracker_rest_api" instance_index=0}: API: Starting on: http://0.0.0.0:1212
2026-09-30T10:48:24.494691Z  INFO complete_startup:start_job{version=V1 cancellation_token=CancellationToken { is_cancelled: false } service_role="tracker_rest_api" instance_index=0}:start_v1{cancellation_token=CancellationToken { is_cancelled: false } service_role="tracker_rest_api" instance_index=0}: API: Started on: http://0.0.0.0:1212
2026-09-30T10:48:24.494698Z  INFO complete_startup:start_job{version=V1 cancellation_token=CancellationToken { is_cancelled: false } service_role="tracker_rest_api" instance_index=0}:start_v1{cancellation_token=CancellationToken { is_cancelled: false } service_role="tracker_rest_api" instance_index=0}: API: Started tracker API service_binding=http://0.0.0.0:1212/
2026-09-30T10:48:24.494716Z  INFO complete_startup:start_job{cancellation_token=CancellationToken { is_cancelled: false }}: HEALTH CHECK API: Starting on: http://127.0.0.1:1313
2026-09-30T10:48:24.494767Z  INFO complete_startup:start_job{cancellation_token=CancellationToken { is_cancelled: false }}: HEALTH CHECK API: Started health check API service_role="health_check_api" instance_index=0 service_binding=http://127.0.0.1:1313/
2026-09-30T10:48:24.494775Z  INFO complete_startup:start_job{cancellation_token=CancellationToken { is_cancelled: false }}: HEALTH CHECK API: Started on: http://127.0.0.1:1313
2026-09-30T10:48:24.494794Z  INFO torrust_tracker: Tracker shutdown signal handlers installed.
```

## Readiness Request (Lines 166-173)

```text
2026-09-30T10:48:24.585556Z  INFO request{method=GET uri=/health_check version=HTTP/1.1}: HEALTH CHECK API: request method=GET uri=/health_check request_id=24ecd080-9757-4eae-a9dd-fcdd7c6fbdd8
2026-09-30T10:48:24.592024Z  INFO request{method=GET uri=/health_check version=HTTP/1.1}: HTTP TRACKER: request server_socket_addr=0.0.0.0:7070 service_binding=http://0.0.0.0:7070/ method=GET uri=/health_check request_id=c707cb40-2aa8-4acd-b495-77445ab331f8
2026-09-30T10:48:24.592026Z  INFO request{method=GET uri=/health_check version=HTTP/1.1}: HTTP TRACKER: request server_socket_addr=0.0.0.0:7171 service_binding=http://0.0.0.0:7171/ method=GET uri=/health_check request_id=75bceea9-e048-4fdd-a3e3-73ffbe7c8ab8
2026-09-30T10:48:24.592050Z  INFO request{method=GET uri=/health_check version=HTTP/1.1}: HTTP TRACKER: response server_socket_addr=0.0.0.0:7070 service_binding=http://0.0.0.0:7070/ latency_ms=0 status_code=200 OK request_id=c707cb40-2aa8-4acd-b495-77445ab331f8
2026-09-30T10:48:24.592080Z  INFO request{method=GET uri=/health_check version=HTTP/1.1}: HTTP TRACKER: response server_socket_addr=0.0.0.0:7171 service_binding=http://0.0.0.0:7171/ latency_ms=0 status_code=200 OK request_id=75bceea9-e048-4fdd-a3e3-73ffbe7c8ab8
2026-09-30T10:48:24.592852Z  INFO request{method=GET uri=/api/health_check version=HTTP/1.1}: API: request server_socket_addr=0.0.0.0:1212 service_binding=http://0.0.0.0:1212/ method=GET uri=/api/health_check request_id=a7374a1d-8672-4721-9319-cdb43d7550e3
2026-09-30T10:48:24.592893Z  INFO request{method=GET uri=/api/health_check version=HTTP/1.1}: API: response latency_ms=0 status_code=200 OK server_socket_addr=0.0.0.0:1212 service_binding=http://0.0.0.0:1212/ request_id=a7374a1d-8672-4721-9319-cdb43d7550e3
2026-09-30T10:48:24.593003Z  INFO request{method=GET uri=/health_check version=HTTP/1.1}: HEALTH CHECK API: response latency_ms=7 status_code=200 OK request_id=24ecd080-9757-4eae-a9dd-fcdd7c6fbdd8
```

## Shutdown (Lines 174-215)

```text
2026-09-30T10:48:25.601147Z  INFO torrust_tracker: Torrust tracker shutting down (SIGTERM) ...
2026-09-30T10:48:25.601202Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Waiting for job to finish under the shared shutdown deadline timeout_seconds=10
2026-09-30T10:48:25.601211Z  INFO graceful_shutdown_on_cancellation{address=127.0.0.1:1313 drain_timeout=5s}: torrust_tracker_axum_server::signals: !! Shutting down health check API server on socket address: 127.0.0.1:1313 in 5s !!
2026-09-30T10:48:25.601232Z  INFO graceful_shutdown_on_cancellation{address=127.0.0.1:1313 drain_timeout=5s}: torrust_tracker_axum_server::signals: All connections closed, shutting down server in address 127.0.0.1:1313
2026-09-30T10:48:25.601238Z  INFO graceful_shutdown_on_cancellation{address=0.0.0.0:7171 drain_timeout=90s}: torrust_tracker_axum_server::signals: !! Shutting down HTTP server on socket address: 0.0.0.0:7171 in 90s !!
2026-09-30T10:48:25.601255Z  INFO graceful_shutdown_on_cancellation{address=0.0.0.0:7171 drain_timeout=90s}: torrust_tracker_axum_server::signals: All connections closed, shutting down server in address 0.0.0.0:7171
2026-09-30T10:48:25.601227Z  INFO graceful_shutdown_on_cancellation{address=0.0.0.0:1212 drain_timeout=90s}: torrust_tracker_axum_server::signals: !! Shutting down tracker API server on socket address: 0.0.0.0:1212 in 90s !!
2026-09-30T10:48:25.601279Z  INFO graceful_shutdown_on_cancellation{address=0.0.0.0:1212 drain_timeout=90s}: torrust_tracker_axum_server::signals: All connections closed, shutting down server in address 0.0.0.0:1212
2026-09-30T10:48:25.601262Z  INFO graceful_shutdown_on_cancellation{address=0.0.0.0:7070 drain_timeout=90s}: torrust_tracker_axum_server::signals: !! Shutting down HTTP server on socket address: 0.0.0.0:7070 in 90s !!
2026-09-30T10:48:25.601326Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job completed after cooperative cancellation job=http_instance_1_0.0.0.0:7171
2026-09-30T10:48:25.601287Z  INFO HEALTH CHECK API: Stopped server running on: http://127.0.0.1:1313
2026-09-30T10:48:25.601343Z  INFO TRACKER_CORE: Received cancellation request, shutting down tracker core event listener.
2026-09-30T10:48:25.601365Z  INFO torrust_tracker_lib::bootstrap::jobs::torrent_cleanup: Stopping torrent cleanup job ...
2026-09-30T10:48:25.601385Z  INFO UDP TRACKER: Received cancellation request, shutting down UDP tracker server event listener.
2026-09-30T10:48:25.601276Z  INFO run_udp_server_main{cookie_lifetime=120s connection_id_validation=Strict request_drain_deadline=5s}: UDP TRACKER: Draining UDP request processors service_binding="udp://0.0.0.0:6969" active=1 deadline=5s
2026-09-30T10:48:25.601325Z  INFO graceful_shutdown_on_cancellation{address=0.0.0.0:7070 drain_timeout=90s}: torrust_tracker_axum_server::signals: All connections closed, shutting down server in address 0.0.0.0:7070
2026-09-30T10:48:25.601303Z  INFO run_udp_server_main{cookie_lifetime=120s connection_id_validation=Strict request_drain_deadline=5s}: UDP TRACKER: Draining UDP request processors service_binding="udp://0.0.0.0:6868" active=1 deadline=5s
2026-09-30T10:48:25.601350Z  INFO UDP TRACKER: Stopping UDP IP-ban cleanup job ...
2026-09-30T10:48:25.601352Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job completed after cooperative cancellation job=health_check_api
2026-09-30T10:48:25.601486Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job completed after cooperative cancellation job=http_api
2026-09-30T10:48:25.601351Z  INFO TRACKER_CORE: Received cancellation request, shutting down tracker core persistent completed statistics event listener.
2026-09-30T10:48:25.601364Z  INFO run_job{max_peer_timeout=60 cancellation_token=CancellationToken { is_cancelled: false }}: torrust_tracker_swarm_coordination_registry::statistics::activity_metrics_updater: Stopping peers activity metrics update job ...
2026-09-30T10:48:25.601309Z  INFO SWARM_COORDINATION_REGISTRY: Received cancellation request, shutting down swarm coordination registry event listener.
2026-09-30T10:48:25.601370Z  INFO UDP TRACKER: Received cancellation request, shutting down UDP tracker core event listener.
2026-09-30T10:48:25.601373Z  INFO UDP TRACKER: Received cancellation request, shutting down UDP tracker server event listener.
2026-09-30T10:48:25.601320Z  INFO HTTP TRACKER: Received cancellation request, shutting down HTTP tracker core event listener.
2026-09-30T10:48:25.601427Z  INFO run_udp_server_main{cookie_lifetime=120s connection_id_validation=Strict request_drain_deadline=5s}: UDP TRACKER: UDP request processors drained service_binding="udp://0.0.0.0:6969" completed=1 failed=0 aborted=0 evicted=0 elapsed=5.2µs
2026-09-30T10:48:25.601462Z  INFO run_udp_server_main{cookie_lifetime=120s connection_id_validation=Strict request_drain_deadline=5s}: UDP TRACKER: UDP request processors drained service_binding="udp://0.0.0.0:6868" completed=1 failed=0 aborted=0 evicted=0 elapsed=1.844µs
2026-09-30T10:48:25.601500Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job completed after cooperative cancellation job=tracker_core_in_memory_event_listener
2026-09-30T10:48:25.601644Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job completed after cooperative cancellation job=torrent_cleanup
2026-09-30T10:48:25.601651Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job completed job=udp_ban_cleanup
2026-09-30T10:48:25.601659Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job completed after cooperative cancellation job=udp_server_stats_event_listener
2026-09-30T10:48:25.601663Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job completed after cooperative cancellation job=http_instance_0_0.0.0.0:7070
2026-09-30T10:48:25.601677Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job completed after cooperative cancellation job=tracker_core_persistent_completed_statistics_event_listener
2026-09-30T10:48:25.601683Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job completed after cooperative cancellation job=peers_inactivity_update
2026-09-30T10:48:25.601688Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job completed after cooperative cancellation job=swarm_coordination_registry_event_listener
2026-09-30T10:48:25.601693Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job completed after cooperative cancellation job=udp_core_event_listener
2026-09-30T10:48:25.601697Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job completed after cooperative cancellation job=udp_server_banning_event_listener
2026-09-30T10:48:25.601702Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job completed after cooperative cancellation job=http_core_event_listener
2026-09-30T10:48:25.601706Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job completed after cooperative cancellation job=udp_instance_1_0.0.0.0:6969
2026-09-30T10:48:25.601712Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job completed after cooperative cancellation job=udp_instance_0_0.0.0.0:6868
2026-09-30T10:48:25.601717Z  INFO torrust_tracker: Torrust tracker successfully shutdown.
```
