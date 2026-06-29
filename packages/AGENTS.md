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
│  axum-http-server  axum-rest-api-server                        │
│  axum-health-check-api-server  udp-server                      │
├────────────────────────────────────────────────────────────────┤
│  Runtime Adapter                                               │
│  rest-api-runtime-adapter                                      │
├────────────────────────────────────────────────────────────────┤
│  Core (domain layer)                                           │
│  http-core  udp-core  tracker-core                             │
│  swarm-coordination-registry                                   │
├────────────────────────────────────────────────────────────────┤
│  Protocols                                                     │
│  http-protocol  udp-protocol                                   │
├────────────────────────────────────────────────────────────────┤
│  Domain / Shared                                               │
│  configuration  primitives  events                             │
│  (extracted: clock, located-error, metrics, net-primitives,    │
│   server-lib)                                                  │
├────────────────────────────────────────────────────────────────┤
│  Utilities / Test support                                      │
│  test-helpers                                                  │
└────────────────────────────────────────────────────────────────┘
```

**Key architectural rule**: Servers contain only network I/O logic. All business rules live in
`*-core` packages. Protocol parsing is isolated in `*-protocol` packages.

See [docs/packages.md](../docs/packages.md) for a full diagram.

## Package Catalog

### Servers (`axum-*`, `udp-server`)

Delivery layer — accept network connections, dispatch to core handlers, return responses.
These packages must not contain business logic.

| Package                        | Entry point  | Protocol    |
| ------------------------------ | ------------ | ----------- |
| `axum-http-server`             | `src/lib.rs` | HTTP BEP 3  |
| `axum-rest-api-server`         | `src/lib.rs` | REST (JSON) |
| `axum-health-check-api-server` | `src/lib.rs` | HTTP        |
| `axum-server`                  | `src/lib.rs` | Axum base   |
| `udp-server`                   | `src/lib.rs` | UDP BEP 15  |

### Core (`*-core`)

Domain layer — business rules, request validation, response building. No Axum or networking
imports. Each core package exposes a `container` module that wires up its dependencies via
dependency injection.

| Package                       | Purpose                                                                                                                                    |
| ----------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------ |
| `tracker-core`                | Central peer management: announce/scrape handlers, auth, whitelist, database abstraction (SQLite/MySQL drivers in `src/databases/driver/`) |
| `http-core`                   | HTTP-specific validation and response formatting                                                                                           |
| `udp-core`                    | UDP connection cookies, crypto, banning logic                                                                                              |
| `rest-api-runtime-adapter`    | REST API runtime adapter and container wiring (Runtime Adapter layer)                                                                      |
| `swarm-coordination-registry` | Registry of torrents and their peer swarms                                                                                                 |

### Protocols (`*-protocol`)

Strict BEP implementations — parse and serialize wire formats only. No tracker logic.

| Package         | BEP    | Handles                                                        |
| --------------- | ------ | -------------------------------------------------------------- |
| `http-protocol` | BEP 3  | URL parameter parsing, bencoded responses, compact peer format |
| `udp-protocol`  | BEP 15 | Message framing, connection IDs, transaction IDs               |

### Domain / Shared

| Package         | Purpose                                                                                                                                                              |
| --------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `configuration` | Config file parsing (`share/default/config/`) and env var loading (`TORRUST_TRACKER_CONFIG_TOML`, `TORRUST_TRACKER_CONFIG_TOML_PATH`); versioned under `src/v2_0_0/` |
| `primitives`    | Core domain types: `InfoHash`, `PeerId`, `Peer`, `SwarmMetadata`                                                                                                     |
| `events`        | Async event bus (broadcaster / receiver / shutdown) used across packages                                                                                             |

### Extracted (previously part of this workspace)

| Package          | Standalone Repository                                                               | Crate Name               | Description                                                      |
| ---------------- | ----------------------------------------------------------------------------------- | ------------------------ | ---------------------------------------------------------------- |
| `clock`          | [torrust/torrust-clock](https://github.com/torrust/torrust-clock)                   | `torrust-clock`          | Deterministic clock abstraction                                  |
| `located-error`  | [torrust/torrust-located-error](https://github.com/torrust/torrust-located-error)   | `torrust-located-error`  | Diagnostic errors with source locations                          |
| `metrics`        | [torrust/torrust-metrics](https://github.com/torrust/torrust-metrics)               | `torrust-metrics`        | Prometheus-compatible metrics: counters, gauges, labels, samples |
| `net-primitives` | [torrust/torrust-net-primitives](https://github.com/torrust/torrust-net-primitives) | `torrust-net-primitives` | Generic networking primitive types (ServiceBinding, Protocol)    |
| `server-lib`     | [torrust/torrust-server-lib](https://github.com/torrust/torrust-server-lib)         | `torrust-server-lib`     | Shared server library utilities                                  |

### Client Tools

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
7. **Layer boundary enforcement**: `deny.toml` at the workspace root configures `cargo deny check bans` to
   prevent cross-layer dependency violations. If you add a dependency on a server-layer or protocol crate
   to a package that isn't listed in that crate's `wrappers` list, the check will fail.
   See [`docs/packages.md`](../docs/packages.md) for the forbidden edge table and
   [`deny.toml`](../deny.toml) for the full configuration.

## Testing Packages

```sh
# All tests for a specific package
cargo test -p <package-name>

# Doc tests only
cargo test --doc -p <package-name>

# MySQL-specific tests in tracker-core (requires a running MySQL instance)
TORRUST_TRACKER_CORE_RUN_MYSQL_DRIVER_TEST=true cargo test -p torrust-tracker-core
```

Use `clock::Stopped` (from the [`torrust-clock`](https://crates.io/crates/torrust-clock) crate) in unit tests that need deterministic time.
Use `test-helpers` for mock tracker servers in integration tests.

## Key Dependency Notes

- `swarm-coordination-registry` is the authoritative store for peer swarms; `tracker-core`
  delegates peer lookups to it.
- `configuration` is the only package that reads from the filesystem or environment at startup;
  other packages receive config structs as arguments.
- `torrust-located-error` (extracted crate) wraps any `std::error::Error` — use it at module boundaries to preserve
  error origin context without losing the original error type.
- `events` provides the only sanctioned inter-package async communication channel; avoid direct
  `tokio::sync` coupling between packages.
