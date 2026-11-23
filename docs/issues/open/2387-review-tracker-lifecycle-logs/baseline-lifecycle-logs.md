---
doc-type: log-capture
issue-spec: docs/issues/open/2387-review-tracker-lifecycle-logs/EPIC.md
last-updated-utc: "2026-11-23 16:05"
---

# Baseline Lifecycle Logs

## Purpose

Verbatim startup and shutdown logs of the current tracker at `info`, captured before any change of EPIC #2387, as the baseline for its sub-issues. Later captures are compared against this one.

This capture replaces the one of 2026-09-30, which predated the last sub-issues of EPIC #1488 (the old capture is in the Git history of this file). Two more captures of the same commit are beside it: [baseline-lifecycle-logs-debug.md](baseline-lifecycle-logs-debug.md) and [baseline-lifecycle-logs-statistics-disabled.md](baseline-lifecycle-logs-statistics-disabled.md).

## Capture

- Date and time (UTC): 2026-11-23 16:05.
- Artifact: release build of `torrust-tracker` at `develop` commit `ff3600f10`, after EPIC #1488 (tracker shutdown overhaul) closed; `rustc 1.100.0 (0ef1bc386 2026-11-09)`; Linux x86_64.
- Configuration: `share/default/config/tracker.development.sqlite3.toml`, unchanged. It runs every service (two UDP trackers, two HTTP trackers, the REST API and the health check API) with `trace_filter = "info"` and usage statistics enabled.
- Steps:
  1. `./target/release/torrust-tracker --config-toml-path <configuration>`
  2. Polled `curl -fsS http://127.0.0.1:1313/health_check` until it succeeded, then waited one second.
  3. Sent `SIGTERM` to the tracker PID and waited, bounded, for it to exit.
- Result: exit status `0`; 254 log lines. ANSI color codes were stripped; nothing else was edited. The configuration dump already masks the API access token (`***`).

The log has three parts:

| Lines | Part | Notes |
| --- | --- | --- |
| 1-173 | Startup | From logging initialization to the shutdown signal handlers being installed |
| 174-181 | Readiness request | The single health check request used to confirm readiness; request logs, not lifecycle logs |
| 182-254 | Shutdown | From `SIGTERM` to the final success message |

This configuration emits no `WARN` and no `ERROR` line.

## Log

```text
2026-11-23T16:05:55.518876Z  INFO torrust_tracker_configuration::v3_0_0::logging: Logging initialized
2026-11-23T16:05:55.518909Z  INFO torrust_tracker_lib::bootstrap::app: Configuration:
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
  "application": {
    "shutdown": {
      "deadline_in_secs": 25,
      "http_drain_in_secs": 15,
      "udp_request_drain_in_secs": 5,
      "event_listener_drain_in_secs": 5
    }
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
2026-11-23T16:05:55.524526Z  INFO initialize: METRICS: type="counter" name="swarm_coordination_registry_torrents_added_total" unit=Some(Count) description=Some(MetricDescription("The total number of torrents added."))
2026-11-23T16:05:55.524544Z  INFO initialize: METRICS: type="counter" name="swarm_coordination_registry_torrents_removed_total" unit=Some(Count) description=Some(MetricDescription("The total number of torrents removed."))
2026-11-23T16:05:55.524547Z  INFO initialize: METRICS: type="gauge" name="swarm_coordination_registry_torrents_total" unit=Some(Count) description=Some(MetricDescription("The total number of torrents."))
2026-11-23T16:05:55.524550Z  INFO initialize: METRICS: type="counter" name="swarm_coordination_registry_torrents_downloads_total" unit=Some(Count) description=Some(MetricDescription("The total number of torrent downloads."))
2026-11-23T16:05:55.524555Z  INFO initialize: METRICS: type="gauge" name="swarm_coordination_registry_torrents_inactive_total" unit=Some(Count) description=Some(MetricDescription("The total number of inactive torrents."))
2026-11-23T16:05:55.524558Z  INFO initialize: METRICS: type="counter" name="swarm_coordination_registry_peers_added_total" unit=Some(Count) description=Some(MetricDescription("The total number of peers added."))
2026-11-23T16:05:55.524560Z  INFO initialize: METRICS: type="counter" name="swarm_coordination_registry_peers_removed_total" unit=Some(Count) description=Some(MetricDescription("The total number of peers removed."))
2026-11-23T16:05:55.524563Z  INFO initialize: METRICS: type="counter" name="swarm_coordination_registry_peers_updated_total" unit=Some(Count) description=Some(MetricDescription("The total number of peers updated."))
2026-11-23T16:05:55.524565Z  INFO initialize: METRICS: type="gauge" name="swarm_coordination_registry_peer_connections_total" unit=Some(Count) description=Some(MetricDescription("The total number of peer connections (one connection per torrent)."))
2026-11-23T16:05:55.524568Z  INFO initialize: METRICS: type="gauge" name="swarm_coordination_registry_unique_peers_total" unit=Some(Count) description=Some(MetricDescription("The total number of unique peers."))
2026-11-23T16:05:55.524571Z  INFO initialize: METRICS: type="gauge" name="swarm_coordination_registry_peers_inactive_total" unit=Some(Count) description=Some(MetricDescription("The total number of inactive peers."))
2026-11-23T16:05:55.524573Z  INFO initialize: METRICS: type="counter" name="swarm_coordination_registry_peers_completed_state_reverted_total" unit=Some(Count) description=Some(MetricDescription("The total number of peers whose completed state was reverted."))
2026-11-23T16:05:55.525542Z  INFO initialize: METRICS: type="counter" name="tracker_core_persistent_torrents_downloads_total" unit=Some(Count) description=Some(MetricDescription("Deprecated: use tracker_core_in_session_torrents_downloads_total or tracker_core_persisted_torrents_downloads_total. This counter is process-local unless persisted completed statistics are enabled."))
2026-11-23T16:05:55.525547Z  INFO initialize: METRICS: type="counter" name="tracker_core_in_session_torrents_downloads_total" unit=Some(Count) description=Some(MetricDescription("The number of torrent downloads completed since this tracker process started."))
2026-11-23T16:05:55.525550Z  INFO initialize: METRICS: type="counter" name="tracker_core_persisted_torrents_downloads_total" unit=Some(Count) description=Some(MetricDescription("The number of torrent downloads restored from and maintained in persistent storage."))
2026-11-23T16:05:55.531055Z  INFO initialize: METRICS: type="counter" name="http_tracker_core_requests_received_total" unit=Some(Count) description=Some(MetricDescription("Total number of HTTP requests received"))
2026-11-23T16:05:55.536351Z  INFO initialize: METRICS: type="counter" name="udp_tracker_core_requests_received_total" unit=Some(Count) description=Some(MetricDescription("Total number of UDP requests received"))
2026-11-23T16:05:55.542190Z  INFO initialize: METRICS: type="counter" name="udp_tracker_server_requests_aborted_total" unit=Some(Count) description=Some(MetricDescription("Total number of UDP requests aborted"))
2026-11-23T16:05:55.542200Z  INFO initialize: METRICS: type="counter" name="udp_tracker_server_requests_discarded_total" unit=Some(Count) description=Some(MetricDescription("Total number of UDP requests discarded before processing (e.g. client source port is 0)"))
2026-11-23T16:05:55.542203Z  INFO initialize: METRICS: type="counter" name="udp_tracker_server_requests_banned_total" unit=Some(Count) description=Some(MetricDescription("Total number of UDP requests banned"))
2026-11-23T16:05:55.542205Z  INFO initialize: METRICS: type="gauge" name="udp_tracker_server_ips_banned_total" unit=Some(Count) description=Some(MetricDescription("Number of IPs whose connection ID errors exceed the limit (banned unless connection ID validation is disabled)"))
2026-11-23T16:05:55.542207Z  INFO initialize: METRICS: type="counter" name="udp_tracker_server_connection_id_errors_total" unit=Some(Count) description=Some(MetricDescription("Total number of requests with connection ID errors"))
2026-11-23T16:05:55.542210Z  INFO initialize: METRICS: type="counter" name="udp_tracker_server_requests_received_total" unit=Some(Count) description=Some(MetricDescription("Total number of UDP requests received"))
2026-11-23T16:05:55.542212Z  INFO initialize: METRICS: type="counter" name="udp_tracker_server_requests_accepted_total" unit=Some(Count) description=Some(MetricDescription("Total number of UDP requests accepted"))
2026-11-23T16:05:55.542214Z  INFO initialize: METRICS: type="counter" name="udp_tracker_server_responses_sent_total" unit=Some(Count) description=Some(MetricDescription("Total number of UDP responses sent"))
2026-11-23T16:05:55.542216Z  INFO initialize: METRICS: type="counter" name="udp_tracker_server_errors_total" unit=Some(Count) description=Some(MetricDescription("Total number of errors processing UDP requests"))
2026-11-23T16:05:55.542220Z  INFO initialize: METRICS: type="gauge" name="udp_tracker_server_performance_avg_processing_time_ns" unit=Some(Nanoseconds) description=Some(MetricDescription("Average time to process a UDP request in nanoseconds"))
2026-11-23T16:05:55.542222Z  INFO initialize: METRICS: type="counter" name="udp_tracker_server_performance_avg_processed_requests_total" unit=Some(Count) description=Some(MetricDescription("Total number of UDP requests processed for the average performance metrics"))
2026-11-23T16:05:55.542362Z  INFO SWARM_COORDINATION_REGISTRY: Starting swarm coordination registry event listener
2026-11-23T16:05:55.542367Z  INFO TRACKER_CORE: Starting tracker core in-memory statistics event listener
2026-11-23T16:05:55.542372Z  INFO HTTP TRACKER: Starting HTTP tracker core event listener
2026-11-23T16:05:55.542372Z  INFO TRACKER_CORE: Starting tracker core persistent completed statistics event listener
2026-11-23T16:05:55.542379Z  INFO UDP TRACKER: Starting UDP tracker server event listener (banning)
2026-11-23T16:05:55.542381Z  INFO UDP TRACKER: Starting UDP tracker server event listener
2026-11-23T16:05:55.542384Z  INFO UDP TRACKER: Starting UDP tracker core event listener
2026-11-23T16:05:55.542387Z  INFO UDP TRACKER: Starting UDP IP-ban cleanup job reset_interval_in_secs=86400
2026-11-23T16:05:55.542391Z  INFO complete_startup:start_job{connection_id_validation=Strict cancellation_token=CancellationToken { is_cancelled: false } request_drain_deadline=5s service_role="udp_tracker" instance_index=0}: torrust_tracker_lib::bootstrap::jobs::udp_tracker: Starting UDP tracker instance bind_address=0.0.0.0:6868 tracker_usage_statistics=true
2026-11-23T16:05:55.542441Z  INFO complete_startup:start_job{connection_id_validation=Strict cancellation_token=CancellationToken { is_cancelled: false } request_drain_deadline=5s service_role="udp_tracker" instance_index=0}:start_with_cancellation{cookie_lifetime=120s connection_id_validation=Strict request_drain_deadline=5s service_role="udp_tracker" instance_index=0}: UDP TRACKER: Starting on: 0.0.0.0:6868
2026-11-23T16:05:55.542444Z  INFO complete_startup:start_job{connection_id_validation=Strict cancellation_token=CancellationToken { is_cancelled: false } request_drain_deadline=5s service_role="udp_tracker" instance_index=0}:start_with_cancellation{cookie_lifetime=120s connection_id_validation=Strict request_drain_deadline=5s service_role="udp_tracker" instance_index=0}: UDP TRACKER: Started on: udp://0.0.0.0:6868
2026-11-23T16:05:55.542451Z  INFO complete_startup:start_job{connection_id_validation=Strict cancellation_token=CancellationToken { is_cancelled: false } request_drain_deadline=5s service_role="udp_tracker" instance_index=0}:start_with_cancellation{cookie_lifetime=120s connection_id_validation=Strict request_drain_deadline=5s service_role="udp_tracker" instance_index=0}: UDP TRACKER: Started UDP tracker service_binding=udp://0.0.0.0:6868
2026-11-23T16:05:55.542472Z  INFO complete_startup:start_job{connection_id_validation=Strict cancellation_token=CancellationToken { is_cancelled: false } request_drain_deadline=5s service_role="udp_tracker" instance_index=1}: torrust_tracker_lib::bootstrap::jobs::udp_tracker: Starting UDP tracker instance bind_address=0.0.0.0:6969 tracker_usage_statistics=true
2026-11-23T16:05:55.542487Z  INFO complete_startup:start_job{connection_id_validation=Strict cancellation_token=CancellationToken { is_cancelled: false } request_drain_deadline=5s service_role="udp_tracker" instance_index=1}:start_with_cancellation{cookie_lifetime=120s connection_id_validation=Strict request_drain_deadline=5s service_role="udp_tracker" instance_index=1}: UDP TRACKER: Starting on: 0.0.0.0:6969
2026-11-23T16:05:55.542489Z  INFO complete_startup:start_job{connection_id_validation=Strict cancellation_token=CancellationToken { is_cancelled: false } request_drain_deadline=5s service_role="udp_tracker" instance_index=1}:start_with_cancellation{cookie_lifetime=120s connection_id_validation=Strict request_drain_deadline=5s service_role="udp_tracker" instance_index=1}: UDP TRACKER: Started on: udp://0.0.0.0:6969
2026-11-23T16:05:55.542496Z  INFO complete_startup:start_job{connection_id_validation=Strict cancellation_token=CancellationToken { is_cancelled: false } request_drain_deadline=5s service_role="udp_tracker" instance_index=1}:start_with_cancellation{cookie_lifetime=120s connection_id_validation=Strict request_drain_deadline=5s service_role="udp_tracker" instance_index=1}: UDP TRACKER: Started UDP tracker service_binding=udp://0.0.0.0:6969
2026-11-23T16:05:55.542505Z  INFO complete_startup:start_job{version=V1 cancellation_token=CancellationToken { is_cancelled: false } drain_timeout=15s service_role="http_tracker" instance_index=0}: torrust_tracker_lib::bootstrap::jobs::http_tracker: Starting HTTP tracker instance bind_address=0.0.0.0:7070 tracker_usage_statistics=true
2026-11-23T16:05:55.542593Z  INFO complete_startup:start_job{version=V1 cancellation_token=CancellationToken { is_cancelled: false } drain_timeout=15s service_role="http_tracker" instance_index=0}:start_v1{cancellation_token=CancellationToken { is_cancelled: false } drain_timeout=15s service_role="http_tracker" instance_index=0}: HTTP TRACKER: Starting on: http://0.0.0.0:7070
2026-11-23T16:05:55.542596Z  INFO complete_startup:start_job{version=V1 cancellation_token=CancellationToken { is_cancelled: false } drain_timeout=15s service_role="http_tracker" instance_index=0}:start_v1{cancellation_token=CancellationToken { is_cancelled: false } drain_timeout=15s service_role="http_tracker" instance_index=0}: HTTP TRACKER: Started on: http://0.0.0.0:7070
2026-11-23T16:05:55.542603Z  INFO complete_startup:start_job{version=V1 cancellation_token=CancellationToken { is_cancelled: false } drain_timeout=15s service_role="http_tracker" instance_index=0}:start_v1{cancellation_token=CancellationToken { is_cancelled: false } drain_timeout=15s service_role="http_tracker" instance_index=0}: torrust_tracker_axum_http_server::server: Started HTTP tracker service_binding=http://0.0.0.0:7070/
2026-11-23T16:05:55.542615Z  INFO complete_startup:start_job{version=V1 cancellation_token=CancellationToken { is_cancelled: false } drain_timeout=15s service_role="http_tracker" instance_index=1}: torrust_tracker_lib::bootstrap::jobs::http_tracker: Starting HTTP tracker instance bind_address=0.0.0.0:7171 tracker_usage_statistics=true
2026-11-23T16:05:55.542652Z  INFO complete_startup:start_job{version=V1 cancellation_token=CancellationToken { is_cancelled: false } drain_timeout=15s service_role="http_tracker" instance_index=1}:start_v1{cancellation_token=CancellationToken { is_cancelled: false } drain_timeout=15s service_role="http_tracker" instance_index=1}: HTTP TRACKER: Starting on: http://0.0.0.0:7171
2026-11-23T16:05:55.542654Z  INFO complete_startup:start_job{version=V1 cancellation_token=CancellationToken { is_cancelled: false } drain_timeout=15s service_role="http_tracker" instance_index=1}:start_v1{cancellation_token=CancellationToken { is_cancelled: false } drain_timeout=15s service_role="http_tracker" instance_index=1}: HTTP TRACKER: Started on: http://0.0.0.0:7171
2026-11-23T16:05:55.542659Z  INFO complete_startup:start_job{version=V1 cancellation_token=CancellationToken { is_cancelled: false } drain_timeout=15s service_role="http_tracker" instance_index=1}:start_v1{cancellation_token=CancellationToken { is_cancelled: false } drain_timeout=15s service_role="http_tracker" instance_index=1}: torrust_tracker_axum_http_server::server: Started HTTP tracker service_binding=http://0.0.0.0:7171/
2026-11-23T16:05:55.542753Z  INFO complete_startup:start_job{version=V1 cancellation_token=CancellationToken { is_cancelled: false } drain_timeout=15s service_role="tracker_rest_api" instance_index=0}:start_v1{cancellation_token=CancellationToken { is_cancelled: false } service_role="tracker_rest_api" instance_index=0}: API: Starting on: http://0.0.0.0:1212
2026-11-23T16:05:55.542758Z  INFO complete_startup:start_job{version=V1 cancellation_token=CancellationToken { is_cancelled: false } drain_timeout=15s service_role="tracker_rest_api" instance_index=0}:start_v1{cancellation_token=CancellationToken { is_cancelled: false } service_role="tracker_rest_api" instance_index=0}: API: Started on: http://0.0.0.0:1212
2026-11-23T16:05:55.542762Z  INFO complete_startup:start_job{version=V1 cancellation_token=CancellationToken { is_cancelled: false } drain_timeout=15s service_role="tracker_rest_api" instance_index=0}:start_v1{cancellation_token=CancellationToken { is_cancelled: false } service_role="tracker_rest_api" instance_index=0}: API: Started tracker API service_binding=http://0.0.0.0:1212/
2026-11-23T16:05:55.542773Z  INFO complete_startup:start_job{readiness=Readiness { ready: true } cancellation_token=CancellationToken { is_cancelled: false } drain_timeout=15s}: HEALTH CHECK API: Starting on: http://127.0.0.1:1313
2026-11-23T16:05:55.542813Z  INFO complete_startup:start_job{readiness=Readiness { ready: true } cancellation_token=CancellationToken { is_cancelled: false } drain_timeout=15s}: HEALTH CHECK API: Started health check API service_role="health_check_api" instance_index=0 service_binding=http://127.0.0.1:1313/
2026-11-23T16:05:55.542817Z  INFO complete_startup:start_job{readiness=Readiness { ready: true } cancellation_token=CancellationToken { is_cancelled: false } drain_timeout=15s}: HEALTH CHECK API: Started on: http://127.0.0.1:1313
2026-11-23T16:05:55.542831Z  INFO torrust_tracker: Tracker shutdown signal handlers installed.
2026-11-23T16:05:56.034087Z  INFO request{method=GET uri=/health_check version=HTTP/1.1}: HEALTH CHECK API: request method=GET uri=/health_check request_id=14911f52-a1eb-4ddd-b874-3059030232cc
2026-11-23T16:05:56.038131Z  INFO request{method=GET uri=/api/health_check version=HTTP/1.1}: API: request server_socket_addr=0.0.0.0:1212 service_binding=http://0.0.0.0:1212/ method=GET uri=/api/health_check request_id=7583d1de-3e65-467a-8e63-89541eda4b0d
2026-11-23T16:05:56.038170Z  INFO request{method=GET uri=/api/health_check version=HTTP/1.1}: API: response latency_ms=0 status_code=200 OK server_socket_addr=0.0.0.0:1212 service_binding=http://0.0.0.0:1212/ request_id=7583d1de-3e65-467a-8e63-89541eda4b0d
2026-11-23T16:05:56.039666Z  INFO request{method=GET uri=/health_check version=HTTP/1.1}: HTTP TRACKER: request server_socket_addr=0.0.0.0:7171 service_binding=http://0.0.0.0:7171/ method=GET uri=/health_check request_id=c80dbaef-60ac-48d8-a81d-5e0a7d84aa0f
2026-11-23T16:05:56.039666Z  INFO request{method=GET uri=/health_check version=HTTP/1.1}: HTTP TRACKER: request server_socket_addr=0.0.0.0:7070 service_binding=http://0.0.0.0:7070/ method=GET uri=/health_check request_id=da411584-622d-4002-9e6a-dab2b4671cd6
2026-11-23T16:05:56.039683Z  INFO request{method=GET uri=/health_check version=HTTP/1.1}: HTTP TRACKER: response server_socket_addr=0.0.0.0:7171 service_binding=http://0.0.0.0:7171/ latency_ms=0 status_code=200 OK request_id=c80dbaef-60ac-48d8-a81d-5e0a7d84aa0f
2026-11-23T16:05:56.039687Z  INFO request{method=GET uri=/health_check version=HTTP/1.1}: HTTP TRACKER: response server_socket_addr=0.0.0.0:7070 service_binding=http://0.0.0.0:7070/ latency_ms=0 status_code=200 OK request_id=da411584-622d-4002-9e6a-dab2b4671cd6
2026-11-23T16:05:56.039761Z  INFO request{method=GET uri=/health_check version=HTTP/1.1}: HEALTH CHECK API: response latency_ms=5 status_code=200 OK request_id=14911f52-a1eb-4ddd-b874-3059030232cc
2026-11-23T16:05:57.042549Z  INFO torrust_tracker: Torrust tracker shutting down (SIGTERM) ...
2026-11-23T16:05:57.042582Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Requested the job to stop: it waits for no other job job=udp_ban_cleanup
2026-11-23T16:05:57.042585Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Requested the job to stop: it waits for no other job job=udp_instance_0_0.0.0.0:6868
2026-11-23T16:05:57.042587Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Requested the job to stop: it waits for no other job job=udp_instance_1_0.0.0.0:6969
2026-11-23T16:05:57.042588Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Requested the job to stop: it waits for no other job job=http_instance_0_0.0.0.0:7070
2026-11-23T16:05:57.042587Z  INFO UDP TRACKER: Stopping UDP IP-ban cleanup job ...
2026-11-23T16:05:57.042590Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Requested the job to stop: it waits for no other job job=http_instance_1_0.0.0.0:7171
2026-11-23T16:05:57.042598Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Requested the job to stop: it waits for no other job job=torrent_cleanup
2026-11-23T16:05:57.042599Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Requested the job to stop: it waits for no other job job=peers_inactivity_update
2026-11-23T16:05:57.042601Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Requested the job to stop: it waits for no other job job=http_api
2026-11-23T16:05:57.042602Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job will be asked to stop after the jobs it waits for have completed job=health_check_api waiting_for=swarm_coordination_registry_event_listener, tracker_core_in_memory_event_listener, tracker_core_persistent_completed_statistics_event_listener, http_core_event_listener, udp_core_event_listener, udp_server_stats_event_listener, udp_server_banning_event_listener, udp_ban_cleanup, udp_instance_0_0.0.0.0:6868, udp_instance_1_0.0.0.0:6969, http_instance_0_0.0.0.0:7070, http_instance_1_0.0.0.0:7171, torrent_cleanup, peers_inactivity_update, http_api
2026-11-23T16:05:57.042611Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job will be asked to stop after the jobs it waits for have completed job=swarm_coordination_registry_event_listener waiting_for=udp_instance_0_0.0.0.0:6868, udp_instance_1_0.0.0.0:6969, http_instance_0_0.0.0.0:7070, http_instance_1_0.0.0.0:7171, torrent_cleanup
2026-11-23T16:05:57.042612Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job will be asked to stop after the jobs it waits for have completed job=tracker_core_in_memory_event_listener waiting_for=udp_instance_0_0.0.0.0:6868, udp_instance_1_0.0.0.0:6969, http_instance_0_0.0.0.0:7070, http_instance_1_0.0.0.0:7171, torrent_cleanup
2026-11-23T16:05:57.042600Z  INFO run_udp_server_main{cookie_lifetime=120s connection_id_validation=Strict request_drain_deadline=5s}: UDP TRACKER: Draining UDP request processors service_binding="udp://0.0.0.0:6868" active=1 deadline=5s
2026-11-23T16:05:57.042614Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job will be asked to stop after the jobs it waits for have completed job=tracker_core_persistent_completed_statistics_event_listener waiting_for=udp_instance_0_0.0.0.0:6868, udp_instance_1_0.0.0.0:6969, http_instance_0_0.0.0.0:7070, http_instance_1_0.0.0.0:7171, torrent_cleanup
2026-11-23T16:05:57.042616Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job will be asked to stop after the jobs it waits for have completed job=http_core_event_listener waiting_for=http_instance_0_0.0.0.0:7070, http_instance_1_0.0.0.0:7171
2026-11-23T16:05:57.042618Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job will be asked to stop after the jobs it waits for have completed job=udp_core_event_listener waiting_for=udp_instance_0_0.0.0.0:6868, udp_instance_1_0.0.0.0:6969
2026-11-23T16:05:57.042619Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job will be asked to stop after the jobs it waits for have completed job=udp_server_stats_event_listener waiting_for=udp_instance_0_0.0.0.0:6868, udp_instance_1_0.0.0.0:6969
2026-11-23T16:05:57.042619Z  INFO run_udp_server_main{cookie_lifetime=120s connection_id_validation=Strict request_drain_deadline=5s}: UDP TRACKER: UDP request processors drained service_binding="udp://0.0.0.0:6868" completed=1 failed=0 aborted=0 evicted=0 elapsed=3.346µs
2026-11-23T16:05:57.042621Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job will be asked to stop after the jobs it waits for have completed job=udp_server_banning_event_listener waiting_for=udp_instance_0_0.0.0.0:6868, udp_instance_1_0.0.0.0:6969
2026-11-23T16:05:57.042613Z  INFO run_udp_server_main{cookie_lifetime=120s connection_id_validation=Strict request_drain_deadline=5s}: UDP TRACKER: Draining UDP request processors service_binding="udp://0.0.0.0:6969" active=1 deadline=5s
2026-11-23T16:05:57.042643Z  INFO torrust_tracker_lib::bootstrap::jobs::torrent_cleanup: Stopping torrent cleanup job ...
2026-11-23T16:05:57.042642Z  INFO graceful_shutdown_on_cancellation{address=0.0.0.0:7171 drain_timeout=15s}: torrust_tracker_axum_server::signals: !! Shutting down HTTP server on socket address: 0.0.0.0:7171 in 15s !!
2026-11-23T16:05:57.042649Z  INFO run_udp_server_main{cookie_lifetime=120s connection_id_validation=Strict request_drain_deadline=5s}: UDP TRACKER: UDP request processors drained service_binding="udp://0.0.0.0:6969" completed=1 failed=0 aborted=0 evicted=0 elapsed=2.535µs
2026-11-23T16:05:57.042651Z  INFO graceful_shutdown_on_cancellation{address=0.0.0.0:1212 drain_timeout=15s}: torrust_tracker_axum_server::signals: !! Shutting down tracker API server on socket address: 0.0.0.0:1212 in 15s !!
2026-11-23T16:05:57.042649Z  INFO run_job{max_peer_timeout=60 cancellation_token=CancellationToken { is_cancelled: false }}: torrust_tracker_swarm_coordination_registry::statistics::activity_metrics_updater: Stopping peers activity metrics update job ...
2026-11-23T16:05:57.042655Z  INFO graceful_shutdown_on_cancellation{address=0.0.0.0:1212 drain_timeout=15s}: torrust_tracker_axum_server::signals: All connections closed, shutting down server in address 0.0.0.0:1212
2026-11-23T16:05:57.042629Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Waiting for job to finish under the shared shutdown deadline timeout_seconds=25
2026-11-23T16:05:57.042653Z  INFO graceful_shutdown_on_cancellation{address=0.0.0.0:7171 drain_timeout=15s}: torrust_tracker_axum_server::signals: All connections closed, shutting down server in address 0.0.0.0:7171
2026-11-23T16:05:57.042666Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job completed after cooperative cancellation job=udp_ban_cleanup
2026-11-23T16:05:57.042633Z  INFO graceful_shutdown_on_cancellation{address=0.0.0.0:7070 drain_timeout=15s}: torrust_tracker_axum_server::signals: !! Shutting down HTTP server on socket address: 0.0.0.0:7070 in 15s !!
2026-11-23T16:05:57.042671Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job completed after cooperative cancellation job=udp_instance_0_0.0.0.0:6868
2026-11-23T16:05:57.042681Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job completed after cooperative cancellation job=torrent_cleanup
2026-11-23T16:05:57.042675Z  INFO graceful_shutdown_on_cancellation{address=0.0.0.0:7070 drain_timeout=15s}: torrust_tracker_axum_server::signals: All connections closed, shutting down server in address 0.0.0.0:7070
2026-11-23T16:05:57.042687Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job completed after cooperative cancellation job=peers_inactivity_update
2026-11-23T16:05:57.042690Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job completed after cooperative cancellation job=udp_instance_1_0.0.0.0:6969
2026-11-23T16:05:57.042697Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Requested the job to stop: every job it waited for has completed job=udp_core_event_listener last_completed_job=udp_instance_1_0.0.0.0:6969
2026-11-23T16:05:57.042699Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Requested the job to stop: every job it waited for has completed job=udp_server_stats_event_listener last_completed_job=udp_instance_1_0.0.0.0:6969
2026-11-23T16:05:57.042701Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Requested the job to stop: every job it waited for has completed job=udp_server_banning_event_listener last_completed_job=udp_instance_1_0.0.0.0:6969
2026-11-23T16:05:57.042704Z  INFO UDP TRACKER: Received cancellation request, draining UDP tracker core event listener.
2026-11-23T16:05:57.042708Z  INFO UDP TRACKER: Event listener drained its queue after cancellation listener="udp_tracker_core_statistics" outcome="emptied" processed=0 lost_to_lag=0
2026-11-23T16:05:57.042710Z  INFO UDP TRACKER: Received cancellation request, draining UDP tracker server event listener.
2026-11-23T16:05:57.042713Z  INFO UDP TRACKER: Event listener drained its queue after cancellation listener="udp_tracker_server_statistics" outcome="emptied" processed=0 lost_to_lag=0
2026-11-23T16:05:57.042725Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job completed after cooperative cancellation job=udp_core_event_listener
2026-11-23T16:05:57.042728Z  INFO UDP TRACKER: Received cancellation request, draining UDP tracker server event listener (banning).
2026-11-23T16:05:57.042730Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job completed after cooperative cancellation job=udp_server_stats_event_listener
2026-11-23T16:05:57.042736Z  INFO UDP TRACKER: Event listener drained its queue after cancellation listener="udp_tracker_server_banning" outcome="emptied" processed=0 lost_to_lag=0
2026-11-23T16:05:57.042750Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job completed after cooperative cancellation job=udp_server_banning_event_listener
2026-11-23T16:05:57.042773Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job completed after cooperative cancellation job=http_instance_0_0.0.0.0:7070
2026-11-23T16:05:57.042807Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job completed after cooperative cancellation job=http_api
2026-11-23T16:05:57.042810Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job completed after cooperative cancellation job=http_instance_1_0.0.0.0:7171
2026-11-23T16:05:57.042815Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Requested the job to stop: every job it waited for has completed job=swarm_coordination_registry_event_listener last_completed_job=http_instance_1_0.0.0.0:7171
2026-11-23T16:05:57.042817Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Requested the job to stop: every job it waited for has completed job=tracker_core_in_memory_event_listener last_completed_job=http_instance_1_0.0.0.0:7171
2026-11-23T16:05:57.042820Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Requested the job to stop: every job it waited for has completed job=tracker_core_persistent_completed_statistics_event_listener last_completed_job=http_instance_1_0.0.0.0:7171
2026-11-23T16:05:57.042822Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Requested the job to stop: every job it waited for has completed job=http_core_event_listener last_completed_job=http_instance_1_0.0.0.0:7171
2026-11-23T16:05:57.042821Z  INFO SWARM_COORDINATION_REGISTRY: Received cancellation request, draining swarm coordination registry event listener.
2026-11-23T16:05:57.042826Z  INFO TRACKER_CORE: Received cancellation request, draining tracker core in-memory statistics event listener.
2026-11-23T16:05:57.042828Z  INFO SWARM_COORDINATION_REGISTRY: Event listener drained its queue after cancellation listener="swarm_coordination_registry_statistics" outcome="emptied" processed=0 lost_to_lag=0
2026-11-23T16:05:57.042830Z  INFO TRACKER_CORE: Event listener drained its queue after cancellation listener="tracker_core_in_memory_statistics" outcome="emptied" processed=0 lost_to_lag=0
2026-11-23T16:05:57.042834Z  INFO TRACKER_CORE: Received cancellation request, draining tracker core persistent completed statistics event listener.
2026-11-23T16:05:57.042837Z  INFO HTTP TRACKER: Received cancellation request, draining HTTP tracker core event listener.
2026-11-23T16:05:57.042838Z  INFO TRACKER_CORE: Event listener drained its queue after cancellation listener="tracker_core_persistent_completed_statistics" outcome="emptied" processed=0 lost_to_lag=0
2026-11-23T16:05:57.042840Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job completed after cooperative cancellation job=swarm_coordination_registry_event_listener
2026-11-23T16:05:57.042840Z  INFO HTTP TRACKER: Event listener drained its queue after cancellation listener="http_tracker_core_statistics" outcome="emptied" processed=0 lost_to_lag=0
2026-11-23T16:05:57.042844Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job completed after cooperative cancellation job=tracker_core_in_memory_event_listener
2026-11-23T16:05:57.042848Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job completed after cooperative cancellation job=tracker_core_persistent_completed_statistics_event_listener
2026-11-23T16:05:57.042851Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job completed after cooperative cancellation job=http_core_event_listener
2026-11-23T16:05:57.042855Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Requested the job to stop: every job it waited for has completed job=health_check_api last_completed_job=http_core_event_listener
2026-11-23T16:05:57.042860Z  INFO graceful_shutdown_on_cancellation{address=127.0.0.1:1313 drain_timeout=15s}: torrust_tracker_axum_server::signals: !! Shutting down health check API server on socket address: 127.0.0.1:1313 in 15s !!
2026-11-23T16:05:57.042864Z  INFO graceful_shutdown_on_cancellation{address=127.0.0.1:1313 drain_timeout=15s}: torrust_tracker_axum_server::signals: All connections closed, shutting down server in address 127.0.0.1:1313
2026-11-23T16:05:57.042894Z  INFO HEALTH CHECK API: Stopped server running on: http://127.0.0.1:1313
2026-11-23T16:05:57.042903Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job completed after cooperative cancellation job=health_check_api
2026-11-23T16:05:57.042912Z  INFO torrust_tracker: Torrust tracker successfully shutdown.
```
