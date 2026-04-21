# Torrust Tracker — Packages

This directory contains all Cargo workspace packages. All domain logic, protocol
implementations, server infrastructure, and utility libraries live here.

For full project context see the [root AGENTS.md](../AGENTS.md).

## Architecture

Packages are organized in strict layers. Dependencies only flow downward — a package may only
depend on packages in the same layer or a lower one.

```text
┌────────────────────────────────────────────────────────────────┐
│  Servers (delivery layer)                                      │
│  axum-http-tracker-server  axum-rest-tracker-api-server        │
│  axum-health-check-api-server  udp-tracker-server              │
├────────────────────────────────────────────────────────────────┤
│  Core (domain layer)                                           │
│  http-tracker-core  udp-tracker-core  tracker-core             │
│  rest-tracker-api-core  swarm-coordination-registry            │
├────────────────────────────────────────────────────────────────┤
│  Protocols                                                     │
│  http-protocol  udp-protocol                                   │
├────────────────────────────────────────────────────────────────┤
│  Domain / Shared                                               │
│  torrent-repository  configuration  primitives                 │
│  events  metrics  clock  located-error  server-lib             │
├────────────────────────────────────────────────────────────────┤
│  Utilities / Test support                                      │
│  test-helpers                                                  │
└────────────────────────────────────────────────────────────────┘
```

**Key architectural rule**: Servers contain only network I/O logic. All business rules live in
`*-core` packages. Protocol parsing is isolated in `*-protocol` packages.

See [docs/packages.md](../docs/packages.md) for a full diagram.

## Package Catalog

### Servers (`axum-*`, `udp-tracker-server`)

Delivery layer — accept network connections, dispatch to core handlers, return responses.
These packages must not contain business logic.

| Package                        | Entry point  | Protocol    |
| ------------------------------ | ------------ | ----------- |
| `axum-http-tracker-server`     | `src/lib.rs` | HTTP BEP 3  |
| `axum-rest-tracker-api-server` | `src/lib.rs` | REST (JSON) |
| `axum-health-check-api-server` | `src/lib.rs` | HTTP        |
| `axum-server`                  | `src/lib.rs` | Axum base   |
| `udp-tracker-server`           | `src/lib.rs` | UDP BEP 15  |

### Core (`*-core`)

Domain layer — business rules, request validation, response building. No Axum or networking
imports. Each core package exposes a `container` module that wires up its dependencies via
dependency injection.

| Package                       | Purpose                                                                                                                                    |
| ----------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------ |
| `tracker-core`                | Central peer management: announce/scrape handlers, auth, whitelist, database abstraction (SQLite/MySQL drivers in `src/databases/driver/`) |
| `http-tracker-core`           | HTTP-specific validation and response formatting                                                                                           |
| `udp-tracker-core`            | UDP connection cookies, crypto, banning logic                                                                                              |
| `rest-tracker-api-core`       | REST API statistics and container wiring                                                                                                   |
| `swarm-coordination-registry` | Registry of torrents and their peer swarms                                                                                                 |

### Protocols (`*-protocol`)

Strict BEP implementations — parse and serialize wire formats only. No tracker logic.

| Package         | BEP    | Handles                                                        |
| --------------- | ------ | -------------------------------------------------------------- |
| `http-protocol` | BEP 3  | URL parameter parsing, bencoded responses, compact peer format |
| `udp-protocol`  | BEP 15 | Message framing, connection IDs, transaction IDs               |

### Domain / Shared

| Package              | Purpose                                                                                                                                                              |
| -------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `torrent-repository` | Torrent metadata storage; InfoHash management; peer coordination                                                                                                     |
| `configuration`      | Config file parsing (`share/default/config/`) and env var loading (`TORRUST_TRACKER_CONFIG_TOML`, `TORRUST_TRACKER_CONFIG_TOML_PATH`); versioned under `src/v2_0_0/` |
| `primitives`         | Core domain types: `InfoHash`, `PeerId`, `Peer`, `SwarmMetadata`, `ServiceBinding`                                                                                   |
| `events`             | Async event bus (broadcaster / receiver / shutdown) used across packages                                                                                             |
| `metrics`            | Prometheus-compatible metrics: counters, gauges, labels, samples                                                                                                     |
| `server-lib`         | Shared HTTP server utilities: logging, service registrar, signal handling                                                                                            |
| `clock`              | Mockable time source — use `clock::Working` in production, `clock::Stopped` in tests                                                                                 |
| `located-error`      | Error decorator that captures the source file/line of the original error                                                                                             |

### Client Tools

| Package                   | Purpose                                                  |
| ------------------------- | -------------------------------------------------------- |
| `tracker-client`          | Generic HTTP and UDP tracker clients (used by E2E tests) |
| `rest-tracker-api-client` | Typed REST API client library                            |

### Utilities / Test support

| Package                           | Purpose                                                    |
| --------------------------------- | ---------------------------------------------------------- |
| `test-helpers`                    | Mock servers, test data generators, shared test fixtures   |
| `torrent-repository-benchmarking` | Criterion benchmarks for alternative torrent storage impls |

## Naming Conventions

| Prefix / Suffix | Responsibility                            | May depend on                 |
| --------------- | ----------------------------------------- | ----------------------------- |
| `axum-*`        | HTTP server components using Axum         | `*-core`, Axum framework      |
| `*-server`      | Server implementations                    | Corresponding `*-core`        |
| `*-core`        | Domain logic and business rules           | `*-protocol`, domain packages |
| `*-protocol`    | BitTorrent protocol parsing/serialization | `primitives`                  |
| `udp-*`         | UDP-specific implementations              | `tracker-core`                |
| `http-*`        | HTTP-specific implementations             | `tracker-core`                |

## Adding or Modifying a Package

1. Create the directory under `packages/<new-package>/` with a `Cargo.toml` and `src/lib.rs`.
2. Add the package to the workspace `[members]` in the root `Cargo.toml`.
3. Follow the naming conventions above.
4. Each package must have:
   - A crate-level doc comment in `src/lib.rs` explaining its purpose and layer.
   - At minimum one unit test (doc-test acceptable for simple utility crates).
5. Run `cargo machete` after adding dependencies — unused deps must not be committed.
6. Run `linter all` before committing.

## Testing Packages

```sh
# All tests for a specific package
cargo test -p <package-name>

# Doc tests only
cargo test --doc -p <package-name>

# MySQL-specific tests in tracker-core (requires a running MySQL instance)
TORRUST_TRACKER_CORE_RUN_MYSQL_DRIVER_TEST=true cargo test -p torrust-tracker-core
```

Use `clock::Stopped` (from the `clock` package) in unit tests that need deterministic time.
Use `test-helpers` for mock tracker servers in integration tests.

## Key Dependency Notes

- `swarm-coordination-registry` is the authoritative store for peer swarms; `tracker-core`
  delegates peer lookups to it.
- `configuration` is the only package that reads from the filesystem or environment at startup;
  other packages receive config structs as arguments.
- `located-error` wraps any `std::error::Error` — use it at module boundaries to preserve
  error origin context without losing the original error type.
- `events` provides the only sanctioned inter-package async communication channel; avoid direct
  `tokio::sync` coupling between packages.
