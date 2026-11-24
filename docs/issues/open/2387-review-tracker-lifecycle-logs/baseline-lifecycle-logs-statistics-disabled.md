---
doc-type: log-capture
issue-spec: docs/issues/open/2387-review-tracker-lifecycle-logs/EPIC.md
last-updated-utc: "2026-11-23 16:05"
---

# Baseline Lifecycle Logs with Usage Statistics Disabled

## Purpose

Verbatim startup and shutdown logs of the current tracker at `info` with usage statistics disabled in every tracker service, captured before any change of EPIC #2387. See [baseline-lifecycle-logs.md](baseline-lifecycle-logs.md) for the capture with statistics enabled.

## Capture

- Date and time (UTC): 2026-11-23 16:05.
- Artifact: release build of `torrust-tracker` at `develop` commit `ff3600f10`, after EPIC #1488 (tracker shutdown overhaul) closed; `rustc 1.100.0 (0ef1bc386 2026-11-09)`; Linux x86_64.
- Configuration: `share/default/config/tracker.development.sqlite3.toml` with one change: `tracker_usage_statistics = false` in the two `[[udp_trackers]]` and the two `[[http_trackers]]` tables.
- Steps:
  1. `./target/release/torrust-tracker --config-toml-path <configuration>`
  2. Polled `curl -fsS http://127.0.0.1:1313/health_check` until it succeeded, then waited one second.
  3. Sent `SIGTERM` to the tracker PID and waited, bounded, for it to exit.
- Result: exit status `0`; 262 log lines. ANSI color codes were stripped; nothing else was edited. The configuration dump already masks the API access token (`***`).

The log has three parts:

| Lines | Part | Notes |
| --- | --- | --- |
| 1-173 | Startup | From logging initialization to the shutdown signal handlers being installed |
| 174-189 | Readiness request | The single health check request used to confirm readiness; request logs, not lifecycle logs |
| 190-262 | Shutdown | From `SIGTERM` to the final success message |

This configuration emits 8 `WARN` lines (from line 175) and no `ERROR` line. They are the statistics listeners ignoring the events of the UDP check that the health check request makes against each of the two UDP trackers: four events for each tracker. Decision D8 in [decisions.md](decisions.md) and sub-issue SI-1 cover them.

## Log

```text
2026-11-23T16:05:59.591019Z  INFO torrust_tracker_configuration::v3_0_0::logging: Logging initialized
2026-11-23T16:05:59.591043Z  INFO torrust_tracker_lib::bootstrap::app: Configuration:
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
      "tracker_usage_statistics": false,
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
      "tracker_usage_statistics": false,
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
      "tracker_usage_statistics": false,
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
      "tracker_usage_statistics": false,
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
2026-11-23T16:05:59.596230Z  INFO initialize: METRICS: type="counter" name="swarm_coordination_registry_torrents_added_total" unit=Some(Count) description=Some(MetricDescription("The total number of torrents added."))
2026-11-23T16:05:59.596250Z  INFO initialize: METRICS: type="counter" name="swarm_coordination_registry_torrents_removed_total" unit=Some(Count) description=Some(MetricDescription("The total number of torrents removed."))
2026-11-23T16:05:59.596254Z  INFO initialize: METRICS: type="gauge" name="swarm_coordination_registry_torrents_total" unit=Some(Count) description=Some(MetricDescription("The total number of torrents."))
2026-11-23T16:05:59.596256Z  INFO initialize: METRICS: type="counter" name="swarm_coordination_registry_torrents_downloads_total" unit=Some(Count) description=Some(MetricDescription("The total number of torrent downloads."))
2026-11-23T16:05:59.596262Z  INFO initialize: METRICS: type="gauge" name="swarm_coordination_registry_torrents_inactive_total" unit=Some(Count) description=Some(MetricDescription("The total number of inactive torrents."))
2026-11-23T16:05:59.596265Z  INFO initialize: METRICS: type="counter" name="swarm_coordination_registry_peers_added_total" unit=Some(Count) description=Some(MetricDescription("The total number of peers added."))
2026-11-23T16:05:59.596268Z  INFO initialize: METRICS: type="counter" name="swarm_coordination_registry_peers_removed_total" unit=Some(Count) description=Some(MetricDescription("The total number of peers removed."))
2026-11-23T16:05:59.596270Z  INFO initialize: METRICS: type="counter" name="swarm_coordination_registry_peers_updated_total" unit=Some(Count) description=Some(MetricDescription("The total number of peers updated."))
2026-11-23T16:05:59.596272Z  INFO initialize: METRICS: type="gauge" name="swarm_coordination_registry_peer_connections_total" unit=Some(Count) description=Some(MetricDescription("The total number of peer connections (one connection per torrent)."))
2026-11-23T16:05:59.596274Z  INFO initialize: METRICS: type="gauge" name="swarm_coordination_registry_unique_peers_total" unit=Some(Count) description=Some(MetricDescription("The total number of unique peers."))
2026-11-23T16:05:59.596277Z  INFO initialize: METRICS: type="gauge" name="swarm_coordination_registry_peers_inactive_total" unit=Some(Count) description=Some(MetricDescription("The total number of inactive peers."))
2026-11-23T16:05:59.596279Z  INFO initialize: METRICS: type="counter" name="swarm_coordination_registry_peers_completed_state_reverted_total" unit=Some(Count) description=Some(MetricDescription("The total number of peers whose completed state was reverted."))
2026-11-23T16:05:59.597053Z  INFO initialize: METRICS: type="counter" name="tracker_core_persistent_torrents_downloads_total" unit=Some(Count) description=Some(MetricDescription("Deprecated: use tracker_core_in_session_torrents_downloads_total or tracker_core_persisted_torrents_downloads_total. This counter is process-local unless persisted completed statistics are enabled."))
2026-11-23T16:05:59.597061Z  INFO initialize: METRICS: type="counter" name="tracker_core_in_session_torrents_downloads_total" unit=Some(Count) description=Some(MetricDescription("The number of torrent downloads completed since this tracker process started."))
2026-11-23T16:05:59.597064Z  INFO initialize: METRICS: type="counter" name="tracker_core_persisted_torrents_downloads_total" unit=Some(Count) description=Some(MetricDescription("The number of torrent downloads restored from and maintained in persistent storage."))
2026-11-23T16:05:59.601922Z  INFO initialize: METRICS: type="counter" name="http_tracker_core_requests_received_total" unit=Some(Count) description=Some(MetricDescription("Total number of HTTP requests received"))
2026-11-23T16:05:59.607281Z  INFO initialize: METRICS: type="counter" name="udp_tracker_core_requests_received_total" unit=Some(Count) description=Some(MetricDescription("Total number of UDP requests received"))
2026-11-23T16:05:59.613021Z  INFO initialize: METRICS: type="counter" name="udp_tracker_server_requests_aborted_total" unit=Some(Count) description=Some(MetricDescription("Total number of UDP requests aborted"))
2026-11-23T16:05:59.613026Z  INFO initialize: METRICS: type="counter" name="udp_tracker_server_requests_discarded_total" unit=Some(Count) description=Some(MetricDescription("Total number of UDP requests discarded before processing (e.g. client source port is 0)"))
2026-11-23T16:05:59.613028Z  INFO initialize: METRICS: type="counter" name="udp_tracker_server_requests_banned_total" unit=Some(Count) description=Some(MetricDescription("Total number of UDP requests banned"))
2026-11-23T16:05:59.613030Z  INFO initialize: METRICS: type="gauge" name="udp_tracker_server_ips_banned_total" unit=Some(Count) description=Some(MetricDescription("Number of IPs whose connection ID errors exceed the limit (banned unless connection ID validation is disabled)"))
2026-11-23T16:05:59.613033Z  INFO initialize: METRICS: type="counter" name="udp_tracker_server_connection_id_errors_total" unit=Some(Count) description=Some(MetricDescription("Total number of requests with connection ID errors"))
2026-11-23T16:05:59.613035Z  INFO initialize: METRICS: type="counter" name="udp_tracker_server_requests_received_total" unit=Some(Count) description=Some(MetricDescription("Total number of UDP requests received"))
2026-11-23T16:05:59.613037Z  INFO initialize: METRICS: type="counter" name="udp_tracker_server_requests_accepted_total" unit=Some(Count) description=Some(MetricDescription("Total number of UDP requests accepted"))
2026-11-23T16:05:59.613039Z  INFO initialize: METRICS: type="counter" name="udp_tracker_server_responses_sent_total" unit=Some(Count) description=Some(MetricDescription("Total number of UDP responses sent"))
2026-11-23T16:05:59.613041Z  INFO initialize: METRICS: type="counter" name="udp_tracker_server_errors_total" unit=Some(Count) description=Some(MetricDescription("Total number of errors processing UDP requests"))
2026-11-23T16:05:59.613046Z  INFO initialize: METRICS: type="gauge" name="udp_tracker_server_performance_avg_processing_time_ns" unit=Some(Nanoseconds) description=Some(MetricDescription("Average time to process a UDP request in nanoseconds"))
2026-11-23T16:05:59.613049Z  INFO initialize: METRICS: type="counter" name="udp_tracker_server_performance_avg_processed_requests_total" unit=Some(Count) description=Some(MetricDescription("Total number of UDP requests processed for the average performance metrics"))
2026-11-23T16:05:59.613174Z  INFO SWARM_COORDINATION_REGISTRY: Starting swarm coordination registry event listener
2026-11-23T16:05:59.613179Z  INFO TRACKER_CORE: Starting tracker core persistent completed statistics event listener
2026-11-23T16:05:59.613175Z  INFO TRACKER_CORE: Starting tracker core in-memory statistics event listener
2026-11-23T16:05:59.613188Z  INFO HTTP TRACKER: Starting HTTP tracker core event listener
2026-11-23T16:05:59.613191Z  INFO UDP TRACKER: Starting UDP tracker core event listener
2026-11-23T16:05:59.613192Z  INFO UDP TRACKER: Starting UDP tracker server event listener (banning)
2026-11-23T16:05:59.613195Z  INFO UDP TRACKER: Starting UDP tracker server event listener
2026-11-23T16:05:59.613191Z  INFO complete_startup:start_job{connection_id_validation=Strict cancellation_token=CancellationToken { is_cancelled: false } request_drain_deadline=5s service_role="udp_tracker" instance_index=0}: torrust_tracker_lib::bootstrap::jobs::udp_tracker: Starting UDP tracker instance bind_address=0.0.0.0:6868 tracker_usage_statistics=false
2026-11-23T16:05:59.613196Z  INFO UDP TRACKER: Starting UDP IP-ban cleanup job reset_interval_in_secs=86400
2026-11-23T16:05:59.613240Z  INFO complete_startup:start_job{connection_id_validation=Strict cancellation_token=CancellationToken { is_cancelled: false } request_drain_deadline=5s service_role="udp_tracker" instance_index=0}:start_with_cancellation{cookie_lifetime=120s connection_id_validation=Strict request_drain_deadline=5s service_role="udp_tracker" instance_index=0}: UDP TRACKER: Starting on: 0.0.0.0:6868
2026-11-23T16:05:59.613244Z  INFO complete_startup:start_job{connection_id_validation=Strict cancellation_token=CancellationToken { is_cancelled: false } request_drain_deadline=5s service_role="udp_tracker" instance_index=0}:start_with_cancellation{cookie_lifetime=120s connection_id_validation=Strict request_drain_deadline=5s service_role="udp_tracker" instance_index=0}: UDP TRACKER: Started on: udp://0.0.0.0:6868
2026-11-23T16:05:59.613251Z  INFO complete_startup:start_job{connection_id_validation=Strict cancellation_token=CancellationToken { is_cancelled: false } request_drain_deadline=5s service_role="udp_tracker" instance_index=0}:start_with_cancellation{cookie_lifetime=120s connection_id_validation=Strict request_drain_deadline=5s service_role="udp_tracker" instance_index=0}: UDP TRACKER: Started UDP tracker service_binding=udp://0.0.0.0:6868
2026-11-23T16:05:59.613271Z  INFO complete_startup:start_job{connection_id_validation=Strict cancellation_token=CancellationToken { is_cancelled: false } request_drain_deadline=5s service_role="udp_tracker" instance_index=1}: torrust_tracker_lib::bootstrap::jobs::udp_tracker: Starting UDP tracker instance bind_address=0.0.0.0:6969 tracker_usage_statistics=false
2026-11-23T16:05:59.613285Z  INFO complete_startup:start_job{connection_id_validation=Strict cancellation_token=CancellationToken { is_cancelled: false } request_drain_deadline=5s service_role="udp_tracker" instance_index=1}:start_with_cancellation{cookie_lifetime=120s connection_id_validation=Strict request_drain_deadline=5s service_role="udp_tracker" instance_index=1}: UDP TRACKER: Starting on: 0.0.0.0:6969
2026-11-23T16:05:59.613287Z  INFO complete_startup:start_job{connection_id_validation=Strict cancellation_token=CancellationToken { is_cancelled: false } request_drain_deadline=5s service_role="udp_tracker" instance_index=1}:start_with_cancellation{cookie_lifetime=120s connection_id_validation=Strict request_drain_deadline=5s service_role="udp_tracker" instance_index=1}: UDP TRACKER: Started on: udp://0.0.0.0:6969
2026-11-23T16:05:59.613294Z  INFO complete_startup:start_job{connection_id_validation=Strict cancellation_token=CancellationToken { is_cancelled: false } request_drain_deadline=5s service_role="udp_tracker" instance_index=1}:start_with_cancellation{cookie_lifetime=120s connection_id_validation=Strict request_drain_deadline=5s service_role="udp_tracker" instance_index=1}: UDP TRACKER: Started UDP tracker service_binding=udp://0.0.0.0:6969
2026-11-23T16:05:59.613302Z  INFO complete_startup:start_job{version=V1 cancellation_token=CancellationToken { is_cancelled: false } drain_timeout=15s service_role="http_tracker" instance_index=0}: torrust_tracker_lib::bootstrap::jobs::http_tracker: Starting HTTP tracker instance bind_address=0.0.0.0:7070 tracker_usage_statistics=false
2026-11-23T16:05:59.613391Z  INFO complete_startup:start_job{version=V1 cancellation_token=CancellationToken { is_cancelled: false } drain_timeout=15s service_role="http_tracker" instance_index=0}:start_v1{cancellation_token=CancellationToken { is_cancelled: false } drain_timeout=15s service_role="http_tracker" instance_index=0}: HTTP TRACKER: Starting on: http://0.0.0.0:7070
2026-11-23T16:05:59.613394Z  INFO complete_startup:start_job{version=V1 cancellation_token=CancellationToken { is_cancelled: false } drain_timeout=15s service_role="http_tracker" instance_index=0}:start_v1{cancellation_token=CancellationToken { is_cancelled: false } drain_timeout=15s service_role="http_tracker" instance_index=0}: HTTP TRACKER: Started on: http://0.0.0.0:7070
2026-11-23T16:05:59.613400Z  INFO complete_startup:start_job{version=V1 cancellation_token=CancellationToken { is_cancelled: false } drain_timeout=15s service_role="http_tracker" instance_index=0}:start_v1{cancellation_token=CancellationToken { is_cancelled: false } drain_timeout=15s service_role="http_tracker" instance_index=0}: torrust_tracker_axum_http_server::server: Started HTTP tracker service_binding=http://0.0.0.0:7070/
2026-11-23T16:05:59.613414Z  INFO complete_startup:start_job{version=V1 cancellation_token=CancellationToken { is_cancelled: false } drain_timeout=15s service_role="http_tracker" instance_index=1}: torrust_tracker_lib::bootstrap::jobs::http_tracker: Starting HTTP tracker instance bind_address=0.0.0.0:7171 tracker_usage_statistics=false
2026-11-23T16:05:59.613460Z  INFO complete_startup:start_job{version=V1 cancellation_token=CancellationToken { is_cancelled: false } drain_timeout=15s service_role="http_tracker" instance_index=1}:start_v1{cancellation_token=CancellationToken { is_cancelled: false } drain_timeout=15s service_role="http_tracker" instance_index=1}: HTTP TRACKER: Starting on: http://0.0.0.0:7171
2026-11-23T16:05:59.613462Z  INFO complete_startup:start_job{version=V1 cancellation_token=CancellationToken { is_cancelled: false } drain_timeout=15s service_role="http_tracker" instance_index=1}:start_v1{cancellation_token=CancellationToken { is_cancelled: false } drain_timeout=15s service_role="http_tracker" instance_index=1}: HTTP TRACKER: Started on: http://0.0.0.0:7171
2026-11-23T16:05:59.613467Z  INFO complete_startup:start_job{version=V1 cancellation_token=CancellationToken { is_cancelled: false } drain_timeout=15s service_role="http_tracker" instance_index=1}:start_v1{cancellation_token=CancellationToken { is_cancelled: false } drain_timeout=15s service_role="http_tracker" instance_index=1}: torrust_tracker_axum_http_server::server: Started HTTP tracker service_binding=http://0.0.0.0:7171/
2026-11-23T16:05:59.613562Z  INFO complete_startup:start_job{version=V1 cancellation_token=CancellationToken { is_cancelled: false } drain_timeout=15s service_role="tracker_rest_api" instance_index=0}:start_v1{cancellation_token=CancellationToken { is_cancelled: false } service_role="tracker_rest_api" instance_index=0}: API: Starting on: http://0.0.0.0:1212
2026-11-23T16:05:59.613567Z  INFO complete_startup:start_job{version=V1 cancellation_token=CancellationToken { is_cancelled: false } drain_timeout=15s service_role="tracker_rest_api" instance_index=0}:start_v1{cancellation_token=CancellationToken { is_cancelled: false } service_role="tracker_rest_api" instance_index=0}: API: Started on: http://0.0.0.0:1212
2026-11-23T16:05:59.613572Z  INFO complete_startup:start_job{version=V1 cancellation_token=CancellationToken { is_cancelled: false } drain_timeout=15s service_role="tracker_rest_api" instance_index=0}:start_v1{cancellation_token=CancellationToken { is_cancelled: false } service_role="tracker_rest_api" instance_index=0}: API: Started tracker API service_binding=http://0.0.0.0:1212/
2026-11-23T16:05:59.613583Z  INFO complete_startup:start_job{readiness=Readiness { ready: true } cancellation_token=CancellationToken { is_cancelled: false } drain_timeout=15s}: HEALTH CHECK API: Starting on: http://127.0.0.1:1313
2026-11-23T16:05:59.613621Z  INFO complete_startup:start_job{readiness=Readiness { ready: true } cancellation_token=CancellationToken { is_cancelled: false } drain_timeout=15s}: HEALTH CHECK API: Started health check API service_role="health_check_api" instance_index=0 service_binding=http://127.0.0.1:1313/
2026-11-23T16:05:59.613625Z  INFO complete_startup:start_job{readiness=Readiness { ready: true } cancellation_token=CancellationToken { is_cancelled: false } drain_timeout=15s}: HEALTH CHECK API: Started on: http://127.0.0.1:1313
2026-11-23T16:05:59.613642Z  INFO torrust_tracker: Tracker shutdown signal handlers installed.
2026-11-23T16:06:00.100599Z  INFO request{method=GET uri=/health_check version=HTTP/1.1}: HEALTH CHECK API: request method=GET uri=/health_check request_id=a02c1ca1-f2cd-4d21-91ce-9b65afb7f9f4
2026-11-23T16:06:00.100786Z  WARN UDP TRACKER: Ignoring UDP server event from an unknown or metrics-disabled listener configuration_instance_id=ConfigurationInstanceId { service_role: UdpTracker, instance_index: 0 }
2026-11-23T16:06:00.100826Z  WARN UDP TRACKER: Ignoring UDP server event from an unknown or metrics-disabled listener configuration_instance_id=ConfigurationInstanceId { service_role: UdpTracker, instance_index: 0 }
2026-11-23T16:06:00.100849Z  WARN UDP TRACKER: Ignoring UDP server event from an unknown or metrics-disabled listener configuration_instance_id=ConfigurationInstanceId { service_role: UdpTracker, instance_index: 0 }
2026-11-23T16:06:00.100854Z  WARN UDP TRACKER: Ignoring UDP tracker event from an unknown or metrics-disabled listener configuration_instance_id=ConfigurationInstanceId { service_role: UdpTracker, instance_index: 0 }
2026-11-23T16:06:00.102921Z  WARN UDP TRACKER: Ignoring UDP server event from an unknown or metrics-disabled listener configuration_instance_id=ConfigurationInstanceId { service_role: UdpTracker, instance_index: 1 }
2026-11-23T16:06:00.102948Z  WARN UDP TRACKER: Ignoring UDP server event from an unknown or metrics-disabled listener configuration_instance_id=ConfigurationInstanceId { service_role: UdpTracker, instance_index: 1 }
2026-11-23T16:06:00.102963Z  WARN UDP TRACKER: Ignoring UDP server event from an unknown or metrics-disabled listener configuration_instance_id=ConfigurationInstanceId { service_role: UdpTracker, instance_index: 1 }
2026-11-23T16:06:00.102971Z  WARN UDP TRACKER: Ignoring UDP tracker event from an unknown or metrics-disabled listener configuration_instance_id=ConfigurationInstanceId { service_role: UdpTracker, instance_index: 1 }
2026-11-23T16:06:00.104670Z  INFO request{method=GET uri=/api/health_check version=HTTP/1.1}: API: request server_socket_addr=0.0.0.0:1212 service_binding=http://0.0.0.0:1212/ method=GET uri=/api/health_check request_id=e0cc0e88-a814-4571-9541-0b4a63a1ac2d
2026-11-23T16:06:00.104702Z  INFO request{method=GET uri=/api/health_check version=HTTP/1.1}: API: response latency_ms=0 status_code=200 OK server_socket_addr=0.0.0.0:1212 service_binding=http://0.0.0.0:1212/ request_id=e0cc0e88-a814-4571-9541-0b4a63a1ac2d
2026-11-23T16:06:00.106277Z  INFO request{method=GET uri=/health_check version=HTTP/1.1}: HTTP TRACKER: request server_socket_addr=0.0.0.0:7070 service_binding=http://0.0.0.0:7070/ method=GET uri=/health_check request_id=2e75708d-8bbe-4ff6-b2d0-a28e54f47a85
2026-11-23T16:06:00.106277Z  INFO request{method=GET uri=/health_check version=HTTP/1.1}: HTTP TRACKER: request server_socket_addr=0.0.0.0:7171 service_binding=http://0.0.0.0:7171/ method=GET uri=/health_check request_id=24b73b33-e146-4b06-a34c-01e8ed378725
2026-11-23T16:06:00.106310Z  INFO request{method=GET uri=/health_check version=HTTP/1.1}: HTTP TRACKER: response server_socket_addr=0.0.0.0:7171 service_binding=http://0.0.0.0:7171/ latency_ms=0 status_code=200 OK request_id=24b73b33-e146-4b06-a34c-01e8ed378725
2026-11-23T16:06:00.106312Z  INFO request{method=GET uri=/health_check version=HTTP/1.1}: HTTP TRACKER: response server_socket_addr=0.0.0.0:7070 service_binding=http://0.0.0.0:7070/ latency_ms=0 status_code=200 OK request_id=2e75708d-8bbe-4ff6-b2d0-a28e54f47a85
2026-11-23T16:06:00.106382Z  INFO request{method=GET uri=/health_check version=HTTP/1.1}: HEALTH CHECK API: response latency_ms=5 status_code=200 OK request_id=a02c1ca1-f2cd-4d21-91ce-9b65afb7f9f4
2026-11-23T16:06:01.109390Z  INFO torrust_tracker: Torrust tracker shutting down (SIGTERM) ...
2026-11-23T16:06:01.109427Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Requested the job to stop: it waits for no other job job=udp_ban_cleanup
2026-11-23T16:06:01.109430Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Requested the job to stop: it waits for no other job job=udp_instance_0_0.0.0.0:6868
2026-11-23T16:06:01.109432Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Requested the job to stop: it waits for no other job job=udp_instance_1_0.0.0.0:6969
2026-11-23T16:06:01.109431Z  INFO UDP TRACKER: Stopping UDP IP-ban cleanup job ...
2026-11-23T16:06:01.109434Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Requested the job to stop: it waits for no other job job=http_instance_0_0.0.0.0:7070
2026-11-23T16:06:01.109437Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Requested the job to stop: it waits for no other job job=http_instance_1_0.0.0.0:7171
2026-11-23T16:06:01.109439Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Requested the job to stop: it waits for no other job job=torrent_cleanup
2026-11-23T16:06:01.109441Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Requested the job to stop: it waits for no other job job=peers_inactivity_update
2026-11-23T16:06:01.109442Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Requested the job to stop: it waits for no other job job=http_api
2026-11-23T16:06:01.109445Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job will be asked to stop after the jobs it waits for have completed job=health_check_api waiting_for=swarm_coordination_registry_event_listener, tracker_core_in_memory_event_listener, tracker_core_persistent_completed_statistics_event_listener, http_core_event_listener, udp_core_event_listener, udp_server_stats_event_listener, udp_server_banning_event_listener, udp_ban_cleanup, udp_instance_0_0.0.0.0:6868, udp_instance_1_0.0.0.0:6969, http_instance_0_0.0.0.0:7070, http_instance_1_0.0.0.0:7171, torrent_cleanup, peers_inactivity_update, http_api
2026-11-23T16:06:01.109443Z  INFO run_udp_server_main{cookie_lifetime=120s connection_id_validation=Strict request_drain_deadline=5s}: UDP TRACKER: Draining UDP request processors service_binding="udp://0.0.0.0:6868" active=1 deadline=5s
2026-11-23T16:06:01.109448Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job will be asked to stop after the jobs it waits for have completed job=swarm_coordination_registry_event_listener waiting_for=udp_instance_0_0.0.0.0:6868, udp_instance_1_0.0.0.0:6969, http_instance_0_0.0.0.0:7070, http_instance_1_0.0.0.0:7171, torrent_cleanup
2026-11-23T16:06:01.109451Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job will be asked to stop after the jobs it waits for have completed job=tracker_core_in_memory_event_listener waiting_for=udp_instance_0_0.0.0.0:6868, udp_instance_1_0.0.0.0:6969, http_instance_0_0.0.0.0:7070, http_instance_1_0.0.0.0:7171, torrent_cleanup
2026-11-23T16:06:01.109453Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job will be asked to stop after the jobs it waits for have completed job=tracker_core_persistent_completed_statistics_event_listener waiting_for=udp_instance_0_0.0.0.0:6868, udp_instance_1_0.0.0.0:6969, http_instance_0_0.0.0.0:7070, http_instance_1_0.0.0.0:7171, torrent_cleanup
2026-11-23T16:06:01.109452Z  INFO run_udp_server_main{cookie_lifetime=120s connection_id_validation=Strict request_drain_deadline=5s}: UDP TRACKER: UDP request processors drained service_binding="udp://0.0.0.0:6868" completed=1 failed=0 aborted=0 evicted=0 elapsed=2.505µs
2026-11-23T16:06:01.109457Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job will be asked to stop after the jobs it waits for have completed job=http_core_event_listener waiting_for=http_instance_0_0.0.0.0:7070, http_instance_1_0.0.0.0:7171
2026-11-23T16:06:01.109459Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job will be asked to stop after the jobs it waits for have completed job=udp_core_event_listener waiting_for=udp_instance_0_0.0.0.0:6868, udp_instance_1_0.0.0.0:6969
2026-11-23T16:06:01.109461Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job will be asked to stop after the jobs it waits for have completed job=udp_server_stats_event_listener waiting_for=udp_instance_0_0.0.0.0:6868, udp_instance_1_0.0.0.0:6969
2026-11-23T16:06:01.109463Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job will be asked to stop after the jobs it waits for have completed job=udp_server_banning_event_listener waiting_for=udp_instance_0_0.0.0.0:6868, udp_instance_1_0.0.0.0:6969
2026-11-23T16:06:01.109458Z  INFO run_udp_server_main{cookie_lifetime=120s connection_id_validation=Strict request_drain_deadline=5s}: UDP TRACKER: Draining UDP request processors service_binding="udp://0.0.0.0:6969" active=1 deadline=5s
2026-11-23T16:06:01.109472Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Waiting for job to finish under the shared shutdown deadline timeout_seconds=25
2026-11-23T16:06:01.109479Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job completed after cooperative cancellation job=udp_ban_cleanup
2026-11-23T16:06:01.109476Z  INFO graceful_shutdown_on_cancellation{address=0.0.0.0:7070 drain_timeout=15s}: torrust_tracker_axum_server::signals: !! Shutting down HTTP server on socket address: 0.0.0.0:7070 in 15s !!
2026-11-23T16:06:01.109483Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job completed after cooperative cancellation job=udp_instance_0_0.0.0.0:6868
2026-11-23T16:06:01.109486Z  INFO graceful_shutdown_on_cancellation{address=0.0.0.0:7070 drain_timeout=15s}: torrust_tracker_axum_server::signals: All connections closed, shutting down server in address 0.0.0.0:7070
2026-11-23T16:06:01.109479Z  INFO graceful_shutdown_on_cancellation{address=0.0.0.0:7171 drain_timeout=15s}: torrust_tracker_axum_server::signals: !! Shutting down HTTP server on socket address: 0.0.0.0:7171 in 15s !!
2026-11-23T16:06:01.109492Z  INFO graceful_shutdown_on_cancellation{address=0.0.0.0:7171 drain_timeout=15s}: torrust_tracker_axum_server::signals: All connections closed, shutting down server in address 0.0.0.0:7171
2026-11-23T16:06:01.109477Z  INFO run_udp_server_main{cookie_lifetime=120s connection_id_validation=Strict request_drain_deadline=5s}: UDP TRACKER: UDP request processors drained service_binding="udp://0.0.0.0:6969" completed=1 failed=0 aborted=0 evicted=0 elapsed=3.457µs
2026-11-23T16:06:01.109497Z  INFO torrust_tracker_lib::bootstrap::jobs::torrent_cleanup: Stopping torrent cleanup job ...
2026-11-23T16:06:01.109517Z  INFO run_job{max_peer_timeout=60 cancellation_token=CancellationToken { is_cancelled: false }}: torrust_tracker_swarm_coordination_registry::statistics::activity_metrics_updater: Stopping peers activity metrics update job ...
2026-11-23T16:06:01.109521Z  INFO graceful_shutdown_on_cancellation{address=0.0.0.0:1212 drain_timeout=15s}: torrust_tracker_axum_server::signals: !! Shutting down tracker API server on socket address: 0.0.0.0:1212 in 15s !!
2026-11-23T16:06:01.109525Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job completed after cooperative cancellation job=torrent_cleanup
2026-11-23T16:06:01.109527Z  INFO graceful_shutdown_on_cancellation{address=0.0.0.0:1212 drain_timeout=15s}: torrust_tracker_axum_server::signals: All connections closed, shutting down server in address 0.0.0.0:1212
2026-11-23T16:06:01.109531Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job completed after cooperative cancellation job=peers_inactivity_update
2026-11-23T16:06:01.109564Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job completed after cooperative cancellation job=udp_instance_1_0.0.0.0:6969
2026-11-23T16:06:01.109574Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Requested the job to stop: every job it waited for has completed job=udp_core_event_listener last_completed_job=udp_instance_1_0.0.0.0:6969
2026-11-23T16:06:01.109576Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Requested the job to stop: every job it waited for has completed job=udp_server_stats_event_listener last_completed_job=udp_instance_1_0.0.0.0:6969
2026-11-23T16:06:01.109578Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Requested the job to stop: every job it waited for has completed job=udp_server_banning_event_listener last_completed_job=udp_instance_1_0.0.0.0:6969
2026-11-23T16:06:01.109582Z  INFO UDP TRACKER: Received cancellation request, draining UDP tracker server event listener.
2026-11-23T16:06:01.109582Z  INFO UDP TRACKER: Received cancellation request, draining UDP tracker core event listener.
2026-11-23T16:06:01.109586Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job completed after cooperative cancellation job=http_instance_0_0.0.0.0:7070
2026-11-23T16:06:01.109588Z  INFO UDP TRACKER: Received cancellation request, draining UDP tracker server event listener (banning).
2026-11-23T16:06:01.109589Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job completed after cooperative cancellation job=http_instance_1_0.0.0.0:7171
2026-11-23T16:06:01.109587Z  INFO UDP TRACKER: Event listener drained its queue after cancellation listener="udp_tracker_core_statistics" outcome="emptied" processed=0 lost_to_lag=0
2026-11-23T16:06:01.109591Z  INFO UDP TRACKER: Event listener drained its queue after cancellation listener="udp_tracker_server_banning" outcome="emptied" processed=0 lost_to_lag=0
2026-11-23T16:06:01.109602Z  INFO TRACKER_CORE: Received cancellation request, draining tracker core in-memory statistics event listener.
2026-11-23T16:06:01.109593Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Requested the job to stop: every job it waited for has completed job=swarm_coordination_registry_event_listener last_completed_job=http_instance_1_0.0.0.0:7171
2026-11-23T16:06:01.109609Z  INFO TRACKER_CORE: Event listener drained its queue after cancellation listener="tracker_core_in_memory_statistics" outcome="emptied" processed=0 lost_to_lag=0
2026-11-23T16:06:01.109602Z  INFO SWARM_COORDINATION_REGISTRY: Received cancellation request, draining swarm coordination registry event listener.
2026-11-23T16:06:01.109618Z  INFO SWARM_COORDINATION_REGISTRY: Event listener drained its queue after cancellation listener="swarm_coordination_registry_statistics" outcome="emptied" processed=0 lost_to_lag=0
2026-11-23T16:06:01.109611Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Requested the job to stop: every job it waited for has completed job=tracker_core_in_memory_event_listener last_completed_job=http_instance_1_0.0.0.0:7171
2026-11-23T16:06:01.109586Z  INFO UDP TRACKER: Event listener drained its queue after cancellation listener="udp_tracker_server_statistics" outcome="emptied" processed=0 lost_to_lag=0
2026-11-23T16:06:01.109621Z  INFO HTTP TRACKER: Received cancellation request, draining HTTP tracker core event listener.
2026-11-23T16:06:01.109625Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Requested the job to stop: every job it waited for has completed job=tracker_core_persistent_completed_statistics_event_listener last_completed_job=http_instance_1_0.0.0.0:7171
2026-11-23T16:06:01.109634Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Requested the job to stop: every job it waited for has completed job=http_core_event_listener last_completed_job=http_instance_1_0.0.0.0:7171
2026-11-23T16:06:01.109608Z  INFO TRACKER_CORE: Received cancellation request, draining tracker core persistent completed statistics event listener.
2026-11-23T16:06:01.109638Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job completed after cooperative cancellation job=udp_core_event_listener
2026-11-23T16:06:01.109639Z  INFO TRACKER_CORE: Event listener drained its queue after cancellation listener="tracker_core_persistent_completed_statistics" outcome="emptied" processed=0 lost_to_lag=0
2026-11-23T16:06:01.109630Z  INFO HTTP TRACKER: Event listener drained its queue after cancellation listener="http_tracker_core_statistics" outcome="emptied" processed=0 lost_to_lag=0
2026-11-23T16:06:01.109642Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job completed after cooperative cancellation job=udp_server_banning_event_listener
2026-11-23T16:06:01.109649Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job completed after cooperative cancellation job=tracker_core_in_memory_event_listener
2026-11-23T16:06:01.109652Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job completed after cooperative cancellation job=http_api
2026-11-23T16:06:01.109655Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job completed after cooperative cancellation job=swarm_coordination_registry_event_listener
2026-11-23T16:06:01.109658Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job completed after cooperative cancellation job=udp_server_stats_event_listener
2026-11-23T16:06:01.109660Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job completed after cooperative cancellation job=tracker_core_persistent_completed_statistics_event_listener
2026-11-23T16:06:01.109662Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job completed after cooperative cancellation job=http_core_event_listener
2026-11-23T16:06:01.109665Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Requested the job to stop: every job it waited for has completed job=health_check_api last_completed_job=http_core_event_listener
2026-11-23T16:06:01.109902Z  INFO graceful_shutdown_on_cancellation{address=127.0.0.1:1313 drain_timeout=15s}: torrust_tracker_axum_server::signals: !! Shutting down health check API server on socket address: 127.0.0.1:1313 in 15s !!
2026-11-23T16:06:01.109907Z  INFO graceful_shutdown_on_cancellation{address=127.0.0.1:1313 drain_timeout=15s}: torrust_tracker_axum_server::signals: All connections closed, shutting down server in address 127.0.0.1:1313
2026-11-23T16:06:01.109941Z  INFO HEALTH CHECK API: Stopped server running on: http://127.0.0.1:1313
2026-11-23T16:06:01.109951Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job completed after cooperative cancellation job=health_check_api
2026-11-23T16:06:01.109959Z  INFO torrust_tracker: Torrust tracker successfully shutdown.
```
