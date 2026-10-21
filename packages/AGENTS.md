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
│  axum-health-check-api-server  axum-server  udp-server         │
├────────────────────────────────────────────────────────────────┤
│  Runtime Adapter                                               │
│  rest-api-runtime-adapter                                      │
├────────────────────────────────────────────────────────────────┤
│  Application                                                   │
│  rest-api-application                                          │
├────────────────────────────────────────────────────────────────┤
│  Core (domain layer)                                           │
│  http-core  udp-core  tracker-core                             │
│  swarm-coordination-registry                                   │
├────────────────────────────────────────────────────────────────┤
│  Clients                                                       │
│  tracker-client  rest-api-client                               │
├────────────────────────────────────────────────────────────────┤
│  Protocols                                                     │
│  http-protocol  udp-protocol  rest-api-protocol                │
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

The benchmarks and developer tools (`e2e-tools`, `persistence-benchmark`,
`torrent-repository-benchmarking`) are not part of these layers; they are never dependencies of
another package.

**Key architectural rule**: Servers contain only network I/O logic. All business rules live in
`*-core` packages. Protocol parsing is isolated in `*-protocol` packages.

See [docs/packages.md](../docs/packages.md) for a full diagram.

## Packages by Layer

The canonical list of packages, with crate names, layers and publish status, is the
[Package Catalog in the root `AGENTS.md`](../AGENTS.md#-package-catalog).
[docs/packages.md](../docs/packages.md#package-catalog) groups the packages by architectural role.
The notes below are the per-layer rules for working in this directory.

### Servers (`axum-*`, `udp-server`)

Delivery layer — accept network connections, dispatch to core handlers, return responses.
These packages must not contain business logic. Each has its entry point in `src/lib.rs`.

### REST API layers (`rest-api-*`)

Contract-first layers: `rest-api-protocol` (wire contract), `rest-api-application` (ports and use
cases) and `rest-api-runtime-adapter` (the only layer that depends on tracker internals). See the
[REST API architecture](../docs/packages.md#rest-api-contract-first-architecture).

### Core (`*-core`)

Domain layer — business rules, request validation, response building. No Axum or networking
imports. Each core package exposes a `container` module that wires up its dependencies via
dependency injection.

- `tracker-core`: central peer management: announce/scrape handlers, auth, whitelist, and the
  database abstraction (SQLite, MySQL and PostgreSQL drivers in `src/databases/driver/`).
- `http-core`: HTTP-specific validation and response formatting.
- `udp-core`: UDP connection cookies, crypto, banning logic.
- `swarm-coordination-registry`: registry of torrents and their peer swarms.

### Protocols (`*-protocol`)

Parse and serialize wire formats only. No tracker logic. `http-protocol` (BEP 3/23) and
`udp-protocol` (BEP 15) are strict BEP implementations; `rest-api-protocol` is the REST API
contract.

### Domain / Shared

- `configuration`: config file parsing (`share/default/config/`) and env var loading
  (`TORRUST_TRACKER_CONFIG_TOML`, `TORRUST_TRACKER_CONFIG_TOML_PATH`); versioned under
  `src/v2_0_0/`.
- `primitives`: core domain types such as `PeerId`, `Peer` and `SwarmMetadata`.
- `events`: async event bus (broadcaster / receiver / shutdown) used across packages.

## Naming Conventions

| Prefix / Suffix | Responsibility                      | May depend on                                                                    |
| --------------- | ----------------------------------- | -------------------------------------------------------------------------------- |
| `axum-*`        | HTTP server components using Axum   | `*-core`, Axum framework                                                         |
| `*-server`      | Server implementations              | Corresponding `*-core`                                                           |
| `*-core`        | Domain logic and business rules     | `*-protocol`, domain packages                                                    |
| `*-protocol`    | Wire protocol parsing/serialization | `primitives`                                                                     |
| `rest-api-*`    | REST API layers (contract-first)    | See [docs/packages.md](../docs/packages.md#rest-api-contract-first-architecture) |
| `udp-*`         | UDP-specific implementations        | `tracker-core`                                                                   |
| `http-*`        | HTTP-specific implementations       | `tracker-core`                                                                   |

## Adding or Modifying a Package

1. Create the directory under `packages/<new-package>/` with a `Cargo.toml` and `src/lib.rs`.
2. Add the package to the workspace `[members]` in the root `Cargo.toml`.
3. Add it to the [Package Catalog in the root `AGENTS.md`](../AGENTS.md#-package-catalog) and to the
   [`docs/packages.md` catalog](../docs/packages.md#package-catalog).
4. Follow the naming conventions above.
5. Each package must have:
   - A crate-level doc comment in `src/lib.rs` explaining its purpose and layer.
   - At minimum one unit test (doc-test acceptable for simple utility crates).
6. Run `cargo machete` after adding dependencies — unused deps must not be committed.
7. Run `linter all` before committing.
8. **Layer boundary enforcement**: `deny.toml` at the workspace root configures `cargo deny check bans` to
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
