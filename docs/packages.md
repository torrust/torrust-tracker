---
semantic-links:
  skill-links:
    - write-markdown-docs
  related-artifacts:
    - docs/index.md
    - docs/adrs/20260429000000_keep_database_as_aggregate_supertrait.md
    - packages/
---

# Torrust Tracker Package Architecture

- [Package Conventions](#package-conventions)
- [Package Catalog](#package-catalog)
- [Architectural Philosophy](#architectural-philosophy)
- [Design Decisions](#design-decisions)
- [Protocol Implementation Details](#protocol-implementation-details)

```output
packages/
├── axum-health-check-api-server
├── axum-http-server
├── axum-rest-api-server
├── axum-server
├── configuration
├── e2e-tools
├── events
├── http-protocol
├── http-tracker-core
├── persistence-benchmark
├── primitives
├── rest-api-client
├── rest-api-core
├── server-lib
├── swarm-coordination-registry
├── test-helpers
├── torrent-repository-benchmarking
├── tracker-client
├── tracker-core
├── udp-protocol
├── udp-tracker-core
└── udp-server
```

```output
console/
└── tracker-client      # Client for interacting with trackers
```

```output
contrib/
└── dev-tools           # Developer tooling (git hooks, container scripts, etc.)
```

## Package Conventions

| Prefix       | Responsibility                         | Dependencies             |
| ------------ | -------------------------------------- | ------------------------ |
| `axum-*`     | HTTP server components using Axum      | Axum framework           |
| `*-server`   | Server implementations                 | Corresponding \*-core    |
| `*-core`     | Domain logic & business rules          | Protocol implementations |
| `*-protocol` | BitTorrent protocol implementations    | BitTorrent protocol      |
| `udp-*`      | UDP Protocol-specific implementations  | Tracker core             |
| `http-*`     | HTTP Protocol-specific implementations | Tracker core             |

Key Architectural Principles:

1. **Separation of Concerns**: Servers contain only network I/O logic.
2. **Protocol Compliance**: `*-protocol` packages strictly implement BEP specifications.
3. **Extensibility**: Core logic is framework-agnostic for easy protocol additions.

## Design Decisions

- Persistence trait boundaries and the aggregate supertrait choice:
  [docs/adrs/20260429000000_keep_database_as_aggregate_supertrait.md](adrs/20260429000000_keep_database_as_aggregate_supertrait.md)

## Package Catalog

| Package                           | Description                          | Key Responsibilities                       |
| --------------------------------- | ------------------------------------ | ------------------------------------------ |
| **axum-\***                       |                                      |                                            |
| `axum-server`                     | Base Axum HTTP server infrastructure | HTTP server lifecycle management           |
| `axum-http-server`                | BitTorrent HTTP tracker (BEP 3/23)   | Handle announce/scrape requests            |
| `axum-rest-api-server`            | Management REST API                  | Tracker configuration & monitoring         |
| `axum-health-check-api-server`    | Health monitoring endpoint           | System health reporting                    |
| **Core Components**               |                                      |                                            |
| `http-tracker-core`               | HTTP-specific implementation         | Request validation, Response formatting    |
| `udp-tracker-core`                | UDP-specific implementation          | Connectionless request handling            |
| `tracker-core`                    | Central tracker logic                | Peer management                            |
| **Protocols**                     |                                      |                                            |
| `http-protocol`                   | HTTP tracker protocol (BEP 3/23)     | Announce/scrape request parsing            |
| `udp-protocol`                    | UDP tracker protocol (BEP 15)        | UDP message framing/parsing                |
| **Domain**                        |                                      |                                            |
| `swarm-coordination-registry`     | Peer swarm registry                  | Torrent/peer coordination                  |
| `configuration`                   | Runtime configuration                | Config file parsing, Environment variables |
| `primitives`                      | Domain-specific types                | PeerId, Peer, SwarmMetadata                |
| `events`                          | Async event bus                      | Inter-package communication                |
| **Utilities**                     |                                      |                                            |
| `server-lib`                      | Shared HTTP server utilities         | Logging, signal handling                   |
| `test-helpers`                    | Testing utilities                    | Mock servers, Test data generation         |
| **Client Tools**                  |                                      |                                            |
| `tracker-client` (`packages/`)    | Tracker client library               | Generic tracker client library             |
| `rest-api-client`                 | API client library                   | REST API integration                       |
| **Benchmarking**                  |                                      |                                            |
| `torrent-repository-benchmarking` | Torrent storage benchmarks           | Criterion benchmarks                       |
| `persistence-benchmark`           | Persistence layer benchmarks         | SQLite/MySQL/PostgreSQL benchmarks         |

### Extracted Packages

Packages that have been extracted to their own standalone repositories.

| Package          | Standalone Repository                                                               | Crate Name               | Description                                                      |
| ---------------- | ----------------------------------------------------------------------------------- | ------------------------ | ---------------------------------------------------------------- |
| `clock`          | [torrust/torrust-clock](https://github.com/torrust/torrust-clock)                   | `torrust-clock`          | Deterministic clock abstraction                                  |
| `located-error`  | [torrust/torrust-located-error](https://github.com/torrust/torrust-located-error)   | `torrust-located-error`  | Diagnostic errors with source locations                          |
| `metrics`        | [torrust/torrust-metrics](https://github.com/torrust/torrust-metrics)               | `torrust-metrics`        | Prometheus-compatible metrics: counters, gauges, labels, samples |
| `net-primitives` | [torrust/torrust-net-primitives](https://github.com/torrust/torrust-net-primitives) | `torrust-net-primitives` | Generic networking primitive types (ServiceBinding, Protocol)    |

## Protocol Implementation Details

### HTTP Tracker (BEP 3/23)

- `http-protocol` implements:
  - URL parameter parsing
  - Response bencoding
  - Error code mapping
  - Compact peer formatting

### UDP Tracker (BEP 15)

- `udp-protocol` handles:
  - Connection ID management
  - Message framing (32-bit big-endian)
  - Transaction ID tracking
  - Error response codes

## Architectural Philosophy

1. **Testability**: Core packages have minimal dependencies for easy unit testing
2. **Observability**: Health checks and metrics built into server packages
3. **Modularity**: Protocol implementations decoupled from transport layers
4. **Extensibility**: New protocols can be added without modifying core logic

![Torrust Tracker Architecture Diagram](./media/packages/torrust-tracker-layers-with-packages.png)

> Diagram shows clean separation between network I/O (servers), protocol handling, and core tracker logic
