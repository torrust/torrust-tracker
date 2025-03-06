# Torrust Axum HTTP Tracker

The Torrust Tracker Health Check API.

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

## Documentation

[Crate documentation](https://docs.rs/torrust-axum-health-check-api-server).

## License

The project is licensed under the terms of the [GNU AFFERO GENERAL PUBLIC LICENSE](./LICENSE).
