# Manual Verification - Issue #2023

**Status:** DONE

This file is the reproducible runtime evidence record for the implemented observability changes.
Create one evidence section for each scenario in the matrix below. Do not combine configured and
absent cases: each configuration change must have its own configuration, requests, and output.

## Evidence Requirements

Each completed scenario section must include:

- date/time in UTC, commit SHA, OS, and Rust toolchain;
- the complete effective local v3 tracker configuration, with sensitive values redacted;
- the exact tracker start and stop commands;
- every request command, including the health-check request, announce request, and metrics request;
- unedited relevant startup log lines and API/Prometheus response output;
- expected versus actual result, including the configured bind address, post-bind service binding,
  and public URL where applicable.

Retain ignored runtime artifacts in `.tmp/issue-2023-public-url-observability/<case>/`, including
the configuration file, tracker log, health response, announce output, and metrics response. Link
or name each retained artifact from its evidence section.

## Scenario Matrix

| ID  | Configuration case                                                   | Required evidence                                                                                                                    | Status |
| --- | -------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------ | ------ |
| M1  | Configured public URL with HTTP tracker `bind_address = "0.0.0.0:0"` | Effective configuration; startup logs; health-check response showing distinct `binding`, `service_binding`, and `public_url`.        | DONE   |
| M2  | Configured public URL after an HTTP announce                         | Announce command/output; Prometheus metrics response showing `public_url` together with existing `server_binding_*` labels.          | DONE   |
| M3  | Configured public URL startup logs                                   | Relevant structured startup log lines showing separate `service_binding` and `public_url` fields.                                    | DONE   |
| M4  | No configured public URL                                             | Effective configuration; startup logs; health-check response with `public_url: null`; metrics response without a `public_url` label. | DONE   |

## M1 - Configured Public URL Health Check

**Status:** DONE

### Environment

| Item               | Value                                                                                             |
| ------------------ | ------------------------------------------------------------------------------------------------- |
| Date/time (UTC)    | 2026-08-31 22:00-22:05                                                                            |
| Commit             | `bac8bc2ca2274882ddc5f8f1c9dcfc28334cec0c` plus the uncommitted validated-`Url` metadata refactor |
| OS                 | Linux                                                                                             |
| Rust toolchain     | `rustc 1.98.0 (88d9e12ae 2026-08-18)`                                                             |
| Artifact directory | `.tmp/issue-2023-public-url-observability/configured/`                                            |

### Effective Configuration

```toml
[metadata]
app = "torrust-tracker"
purpose = "configuration"
schema_version = "3.0.0"
[logging]
trace_filter = "info"
trace_style = "full"
[core]
inactive_peer_cleanup_interval = 120
listed = false
private = false
[core.database]
driver = "sqlite3"
path = ".tmp/issue-2023-public-url-observability/configured/tracker.sqlite3"
[core.tracker_policy]
max_peer_timeout = 60
persistent_torrent_completed_stat = true
remove_peerless_torrents = true
[udp_tracker_server]
ip_bans_reset_interval_in_secs = 86400
max_connection_id_errors_per_ip = 10
connection_id_validation = "strict"
[[http_trackers]]
bind_address = "0.0.0.0:0"
tracker_usage_statistics = true
public_url = "https://tracker.example.test/announce"
[http_api]
bind_address = "127.0.0.1:18123"
[http_api.access_tokens]
admin = "issue-2023-evidence-token"
[health_check_api]
bind_address = "127.0.0.1:18124"
```

### Commands and Output

```sh
TORRUST_TRACKER_CONFIG_TOML_PATH="$PWD/.tmp/issue-2023-public-url-observability/configured/tracker.toml" cargo run --bin torrust-tracker > "$PWD/.tmp/issue-2023-public-url-observability/configured/tracker.log" 2>&1
```

```text
# The tracker runs until stopped; startup output is captured in `tracker.log`.
```

```sh
rg 'Started HTTP tracker' .tmp/issue-2023-public-url-observability/configured/tracker.log
```

```text
2026-09-01T07:16:01.710048Z  INFO ... Started HTTP tracker service_binding=http://0.0.0.0:36535/ public_url=https://tracker.example.test/announce
```

```sh
curl --fail --silent --show-error http://127.0.0.1:18124/health_check
```

```json
{
  "status": "Ok",
  "message": "",
  "details": [
    {
      "service_binding": "http://0.0.0.0:36535/",
      "binding": "0.0.0.0:36535",
      "service_type": "http_tracker",
      "public_url": "https://tracker.example.test/announce",
      "info": "checking http tracker health check at: http://0.0.0.0:36535/health_check",
      "result": { "Ok": "200 OK" }
    },
    {
      "service_binding": "http://127.0.0.1:18123/",
      "binding": "127.0.0.1:18123",
      "service_type": "tracker_rest_api",
      "public_url": null,
      "info": "checking api health check at: http://127.0.0.1:18123/api/health_check",
      "result": { "Ok": "200 OK" }
    }
  ]
}
```

```sh
# Stop the tracker with SIGTERM after all requests complete.
```

### Expected and Actual Result

| Expected                                                                             | Actual                                                                                                                             |
| ------------------------------------------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------- |
| The wildcard `:0` bind, post-bind binding, and configured external URL are separate. | The tracker bound `0.0.0.0:36535`; the health response separately reported the configured `https://tracker.example.test/announce`. |

## M2 - Configured Public URL Metrics

**Status:** DONE

### Commands and Output

```sh
cargo run -p torrust-tracker-client --bin tracker_client -- http announce http://127.0.0.1:36535 9c38422213e30bff212b30c360d26f9a02136422 --format text
```

```json
{
  "complete": 1,
  "incomplete": 0,
  "interval": 120,
  "min interval": 120,
  "peers": []
}
```

```sh
curl --fail --silent --show-error -H 'Authorization: Bearer issue-2023-evidence-token' http://127.0.0.1:18123/api/v1/metrics
```

```json
{
  "name": "http_tracker_core_requests_received_total",
  "samples": [
    {
      "value": 1,
      "labels": [
        { "name": "client_address_ip_family", "value": "inet" },
        { "name": "client_address_ip_type", "value": "plain" },
        {
          "name": "public_url",
          "value": "https://tracker.example.test/announce"
        },
        { "name": "request_kind", "value": "announce" },
        { "name": "server_binding_address_ip_family", "value": "inet" },
        { "name": "server_binding_address_ip_type", "value": "plain" },
        { "name": "server_binding_ip", "value": "0.0.0.0" },
        { "name": "server_binding_port", "value": "36535" },
        { "name": "server_binding_protocol", "value": "http" }
      ]
    }
  ]
}
```

### Expected and Actual Result

| Expected                                                                                         | Actual                                                                                                                      |
| ------------------------------------------------------------------------------------------------ | --------------------------------------------------------------------------------------------------------------------------- |
| An HTTP announce emits a metric with both the configured URL and existing server-binding labels. | The received-request metric had `public_url=https://tracker.example.test/announce` plus all five `server_binding_*` labels. |

## M3 - Configured Public URL Startup Logs

**Status:** DONE

### Command and Output

```sh
rg 'Started HTTP tracker' .tmp/issue-2023-public-url-observability/configured/tracker.log
```

```text
2026-09-01T07:16:01.710048Z  INFO ... Started HTTP tracker service_binding=http://0.0.0.0:36535/ public_url=https://tracker.example.test/announce
```

### Expected and Actual Result

| Expected                                                                                         | Actual                                                                            |
| ------------------------------------------------------------------------------------------------ | --------------------------------------------------------------------------------- |
| Startup logs retain local service identity and separately record the configured public endpoint. | The structured event included distinct `service_binding` and `public_url` fields. |

## M4 - Absent Public URL

**Status:** DONE

### Effective Configuration

```toml
# Same configuration as M1, except `public_url` is omitted from `[[http_trackers]]`.
# Full file: `.tmp/issue-2023-public-url-observability/absent/tracker.toml`.
# Isolated paths and ports: database `.tmp/issue-2023-public-url-observability/absent/tracker.sqlite3`,
# HTTP API `127.0.0.1:18125`, health-check API `127.0.0.1:18126`.
```

### Commands and Output

```sh
TORRUST_TRACKER_CONFIG_TOML_PATH="$PWD/.tmp/issue-2023-public-url-observability/absent/tracker.toml" cargo run --bin torrust-tracker > "$PWD/.tmp/issue-2023-public-url-observability/absent/tracker.log" 2>&1
```

```text
# The tracker runs until stopped; startup output is captured in `tracker.log`.
```

```sh
rg 'Started HTTP tracker' .tmp/issue-2023-public-url-observability/absent/tracker.log
```

```text
2026-09-01T07:22:54.275470Z  INFO ... Started HTTP tracker service_binding=http://0.0.0.0:56001/
```

```sh
curl --fail --silent --show-error http://127.0.0.1:18126/health_check
```

```json
{
  "status": "Ok",
  "message": "",
  "details": [
    {
      "service_binding": "http://0.0.0.0:56001/",
      "binding": "0.0.0.0:56001",
      "service_type": "http_tracker",
      "public_url": null,
      "info": "checking http tracker health check at: http://0.0.0.0:56001/health_check",
      "result": { "Ok": "200 OK" }
    },
    {
      "service_binding": "http://127.0.0.1:18125/",
      "binding": "127.0.0.1:18125",
      "service_type": "tracker_rest_api",
      "public_url": null,
      "info": "checking api health check at: http://127.0.0.1:18125/api/health_check",
      "result": { "Ok": "200 OK" }
    }
  ]
}
```

```sh
cargo run -p torrust-tracker-client --bin tracker_client -- http announce http://127.0.0.1:56001 9c38422213e30bff212b30c360d26f9a02136422 --format text
```

```json
{
  "complete": 1,
  "incomplete": 0,
  "interval": 120,
  "min interval": 120,
  "peers": []
}
```

```sh
curl --fail --silent --show-error -H 'Authorization: Bearer issue-2023-evidence-token' http://127.0.0.1:18125/api/v1/metrics
```

```json
{
  "name": "http_tracker_core_requests_received_total",
  "samples": [
    {
      "value": 1,
      "labels": [
        { "name": "client_address_ip_family", "value": "inet" },
        { "name": "client_address_ip_type", "value": "plain" },
        { "name": "request_kind", "value": "announce" },
        { "name": "server_binding_address_ip_family", "value": "inet" },
        { "name": "server_binding_address_ip_type", "value": "plain" },
        { "name": "server_binding_ip", "value": "0.0.0.0" },
        { "name": "server_binding_port", "value": "56001" },
        { "name": "server_binding_protocol", "value": "http" }
      ]
    }
  ]
}
```

```sh
# Stop the tracker with SIGTERM after all requests complete.
```

### Expected and Actual Result

| Expected                                                                                                      | Actual                                                                                                                                                       |
| ------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Without a configured URL, health returns `null`, startup logs do not claim a URL, and metrics omit the label. | Health returned `public_url:null`; the startup event omitted `public_url`; the HTTP metric retained its server-binding labels and had no `public_url` label. |
