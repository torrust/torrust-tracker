# Torrust Tracker Persistence Benchmark

Developer tool for benchmarking the Torrust Tracker persistence layer directly against database drivers.

This binary is intended for local development and is excluded from the production container image.

## Usage

```sh
# Benchmark SQLite
cargo run -p torrust-tracker-persistence-benchmark --bin persistence_benchmark_runner -- \
  --driver sqlite3

# Benchmark MySQL
cargo run -p torrust-tracker-persistence-benchmark --bin persistence_benchmark_runner -- \
  --driver mysql \
  --db-version 8.4
```
