# Torrust Tracker E2E Tools

E2E test runners and developer profiling tools for the Torrust Tracker.

These binaries are intended for CI E2E testing and local development only.
They are excluded from the production container image.

## Binaries

- `e2e_tests_runner` — runs the Torrust Tracker E2E test suite against a running container image
- `qbittorrent_e2e_runner` — runs the qBittorrent E2E test suite against a running container image
- `profiling` — developer profiling tool for tracker performance analysis

## Usage

```sh
# Run E2E tests against a local tracker image
cargo run -p torrust-tracker-e2e-tools --bin e2e_tests_runner -- \
  --config-toml-path "./share/default/config/tracker.e2e.container.sqlite3.toml" \
  --tracker-image "torrust-tracker:local"

# Run qBittorrent E2E tests (SQLite3)
cargo run -p torrust-tracker-e2e-tools --bin qbittorrent_e2e_runner -- \
  --tracker-image "torrust-tracker:local" \
  --db-driver sqlite3
```
