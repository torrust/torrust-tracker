# Torrust Tracker Axum Health Check API Server

`torrust-tracker-axum-health-check-api-server` serves the health check endpoint of the Torrust Tracker. The tracker container runs it on a local port to probe the UDP tracker, HTTP tracker and REST API services, and uses it as the container health check. It does not serve any BitTorrent or management traffic.

## Where it fits

Server (`axum-*`) package. Workspace dependencies: `torrust-tracker-axum-server`, `torrust-tracker-configuration`, `torrust-tracker-primitives`. Used by: `torrust-tracker`. See the [package catalog](https://github.com/torrust/torrust-tracker/blob/develop/docs/packages.md#package-catalog) for the full picture.

## Main API

`server::start` and `server::start_with_cancellation` start the server, `handlers` holds the `/health_check` handler, and `resources` and `responses` define the JSON response types.

## Stability

Versions are independent per package (see the [ADR](https://github.com/torrust/torrust-tracker/blob/develop/docs/adrs/20260629000000_adopt_independent_package_versioning.md)). While the version is `0.x`, the public API can change in any minor release. The crate is developed for the Torrust Tracker first, so use it directly only if you accept that.

## Documentation

- [API documentation](https://docs.rs/torrust-tracker-axum-health-check-api-server)

## Endpoint

The Torrust tracker container starts a local HTTP server on port 1313 to check all services.

It's used for the container health check.

URL: <http://127.0.0.1:1313/health_check>

Example response:

```json
{
  "status": "Ok",
  "message": "",
  "details": [
    {
      "binding": "0.0.0.0:6969",
      "info": "checking the udp tracker health check at: 0.0.0.0:6969",
      "result": {
        "Ok": "Connected"
      }
    },
    {
      "binding": "0.0.0.0:1212",
      "info": "checking api health check at: http://0.0.0.0:1212/api/health_check",
      "result": {
        "Ok": "200 OK"
      }
    },
    {
      "binding": "0.0.0.0:7070",
      "info": "checking http tracker health check at: http://0.0.0.0:7070/health_check",
      "result": {
        "Ok": "200 OK"
      }
    }
  ]
}
```

## Readiness during shutdown

While the tracker is shutting down, `/health_check` answers `503 Service Unavailable` with
`{"status":"Error","message":"shutting down","details":[]}` and does not probe any service, so
readiness probes stop routing traffic to the instance. The server reads a
`torrust_tracker_primitives::Readiness` handle passed to `server::start_with_cancellation`; the
application marks it not ready before it cancels its components. See the
[shutdown process documentation](https://github.com/torrust/torrust-tracker/blob/develop/docs/features/shutdown-process/README.md#readiness-before-drain).

## License

The project is licensed under the terms of the [GNU AFFERO GENERAL PUBLIC LICENSE](./LICENSE) (`AGPL-3.0-only`).
