# Manual Verification Evidence - Issue #1415

This file preserves reproducible manual-verification evidence before and after the implementation
of issue #1415. The baseline was captured from commit `31841042` on branch
`1415-use-service-binding` before source changes for this issue.

## Environment

| Item                  | Value                                                   |
| --------------------- | ------------------------------------------------------- |
| Date                  | 2026-07-22 12:48-12:50 UTC                              |
| Tracker command       | `cargo run` from the repository root                    |
| Configuration         | `share/default/config/tracker.development.sqlite3.toml` |
| Health-check endpoint | `http://127.0.0.1:1313/health_check`                    |
| REST API endpoint     | `http://127.0.0.1:1212`                                 |
| HTTP tracker endpoint | `http://127.0.0.1:7070`                                 |
| REST API token        | Development-config `admin` token                        |

## Baseline - Before Implementation

### M1: Health Check

**Command**:

```console
curl --fail --silent --show-error http://127.0.0.1:1313/health_check | jq .
```

**Output**:

```json
{
  "status": "Ok",
  "message": "",
  "details": [
    {
      "service_binding": "udp://0.0.0.0:6868",
      "binding": "0.0.0.0:6868",
      "service_type": "udp_tracker",
      "info": "checking the udp tracker health check at: 0.0.0.0:6868",
      "result": { "Ok": "Connected" }
    },
    {
      "service_binding": "udp://0.0.0.0:6969",
      "binding": "0.0.0.0:6969",
      "service_type": "udp_tracker",
      "info": "checking the udp tracker health check at: 0.0.0.0:6969",
      "result": { "Ok": "Connected" }
    },
    {
      "service_binding": "http://0.0.0.0:7171/",
      "binding": "0.0.0.0:7171",
      "service_type": "http_tracker",
      "info": "checking http tracker health check at: http://0.0.0.0:7171/health_check",
      "result": { "Ok": "200 OK" }
    },
    {
      "service_binding": "http://0.0.0.0:1212/",
      "binding": "0.0.0.0:1212",
      "service_type": "tracker_rest_api",
      "info": "checking api health check at: http://0.0.0.0:1212/api/health_check",
      "result": { "Ok": "200 OK" }
    },
    {
      "service_binding": "http://0.0.0.0:7070/",
      "binding": "0.0.0.0:7070",
      "service_type": "http_tracker",
      "info": "checking http tracker health check at: http://0.0.0.0:7070/health_check",
      "result": { "Ok": "200 OK" }
    }
  ]
}
```

**Baseline result**: PASS. The endpoint already exposes a protocol-aware
`service_binding` for every registered service.

**Post-implementation expected output**: The same contract remains available. Each registered
HTTP and UDP service includes a `service_binding` whose scheme matches its protocol and whose
address matches `binding` (HTTP values include the URL serializer's trailing slash).

### M2: HTTP Announce and Metrics

**Announce command**:

```console
cargo run -p torrust-tracker-client --bin tracker_client -- http announce http://127.0.0.1:7070 9c38422213e30bff212b30c360d26f9a02136422
```

**Announce output**:

```json
{
  "complete": 1,
  "incomplete": 0,
  "interval": 120,
  "min interval": 120,
  "peers": []
}
```

**Metrics command**:

```console
curl --fail --silent --show-error 'http://127.0.0.1:1212/api/v1/metrics?token=MyAccessToken&format=prometheus' | grep -iE 'announce|binding|http_tracker'
```

**Relevant output**:

```text
# HELP http_tracker_core_requests_received_total Total number of HTTP requests received
# TYPE http_tracker_core_requests_received_total counter
http_tracker_core_requests_received_total{client_address_ip_family="inet",client_address_ip_type="plain",request_kind="announce",server_binding_address_ip_family="inet",server_binding_address_ip_type="plain",server_binding_ip="0.0.0.0",server_binding_port="7070",server_binding_protocol="http"} 1
```

**Baseline result**: PASS. A successful HTTP announce produces an HTTP metric with the split
`server_binding_*` labels.

**Post-implementation expected output**: The metric name and current label set remain available;
the announce sample contains `server_binding_ip="0.0.0.0"`,
`server_binding_port="7070"`, and `server_binding_protocol="http"`.

## Runtime-Log Contract

The baseline tracker logs show protocol-aware startup output, for example:

```text
HTTP TRACKER: Started on: http://0.0.0.0:7070
API: Started on: http://0.0.0.0:1212
```

However, HTTP tracker request logs still record only a socket address:

```text
HTTP TRACKER: request server_socket_addr=0.0.0.0:7070 method=GET uri=/announce?...
API: response latency_ms=0 status_code=200 OK server_socket_addr=0.0.0.0:1212
```

### Post-Implementation Expected Output

Issue #1415 retains `server_socket_addr` and adds `service_binding` to service request and
response logs. `server_socket_addr` remains a valid socket-address value; `service_binding`
adds the protocol-aware service identity already used by the health-check API. It is serialized
with `ServiceBinding`'s display representation:

```text
HTTP TRACKER: request server_socket_addr=0.0.0.0:7070 service_binding=http://0.0.0.0:7070/ method=GET uri=/announce?...
API: response latency_ms=0 status_code=200 OK server_socket_addr=0.0.0.0:1212 service_binding=http://0.0.0.0:1212/
```

The exact unrelated fields and their ordering may differ according to the tracing formatter, but
the following are required:

- request and response logs that identify the serving HTTP tracker or REST API use
  `service_binding=<protocol>://<post-bind-address>/`;
- the `ServiceBinding` scheme is `http` for both HTTP tracker and REST API services;
- the existing `server_socket_addr=<post-bind-address>` remains present for compatibility;
- `server_socket_addr` and `service_binding` describe the same post-bind socket address, with
  `service_binding` adding the service protocol;
- a wildcard bind address remains wildcard, and a configured port `0` is replaced with the
  OS-assigned port in both fields.

This contract does not claim that the displayed wildcard URL is directly reachable. It identifies
the local bound service only; any future operator-declared `public_url` is out of scope for #1415.

**Post-implementation verification**: run the tracker, send an HTTP announce, make a REST API
request, and inspect the corresponding request/response logs for the expected fields above.

## Post-Implementation Evidence

Not yet executed. After implementation, repeat M1 and M2 and perform the runtime-log verification
defined above. Replace this section with the actual commands, outputs, date, commit, and result.
