# `src/` — Binary and Library Entry Points

This directory contains only the top-level wiring of the application: the binary entry points,
the bootstrap sequence, and the dependency-injection container. All domain logic lives in
`packages/`; this directory merely assembles and launches it.

## File Map

| Path                        | Purpose                                                                                                                                                                                            |
| --------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `main.rs`                   | Main binary entry point. Parses `-c` / `--config-toml-path`, starts the app, then waits for shutdown and cancels jobs.                                                                             |
| `lib.rs`                    | Library crate root and crate-level documentation. Re-exports the public API used by integration tests and other binaries.                                                                          |
| `app.rs`                    | Environment-compatible `start()`, parameterized startup, and `complete_startup()` — orchestrate the full startup sequence.                                                                         |
| `container.rs`              | `AppContainer` — dependency-injection struct that holds `Arc`-wrapped instances of every per-layer container.                                                                                      |
| `bootstrap/app.rs`          | `setup()` — receives optional explicit path, loads config, validates it, initializes services, builds `AppContainer`.                                                                              |
| `bootstrap/config.rs`       | `initialize_configuration()` — loads config from an optional explicit path or existing environment/default sources.                                                                                |
| `bootstrap/jobs/`           | One module per service: each module exposes a starter function called from `app::start_jobs`.                                                                                                      |
| `bootstrap/jobs/manager.rs` | `JobManager` — directly owns named component futures in a `JoinSet`, retains legacy periodic handles through a compatibility registry, owns the `CancellationToken`, and drives graceful shutdown. |
| `bin/e2e_tests_runner.rs`   | Binary that runs E2E tests by delegating to `src/console/ci/`.                                                                                                                                     |
| `bin/http_health_check.rs`  | Minimal HTTP health-check binary used inside containers (avoids curl/wget dependency).                                                                                                             |
| `bin/profiling.rs`          | Binary for Valgrind / kcachegrind profiling sessions.                                                                                                                                              |
| `console/`                  | Internal console apps (`ci/e2e`, `profiling`) used by the extra binaries above.                                                                                                                    |

## Bootstrap Flow

```text
main()
   └─ app::start_with_explicit_config_toml_path()
      ├─ bootstrap::app::setup(explicit_config_toml_path)
      │    ├─ bootstrap::config::initialize_configuration()   ← explicit path / TOML / env vars
      │    ├─ configuration.validate()                        ← returns typed startup errors
       │    ├─ initialize_global_services()                    ← logging, crypto seed
       │    └─ AppContainer::initialize(&configuration)        ← builds all containers
       │
       └─ app::start(&config, &app_container)
            ├─ load_data_from_database()                       ← peer keys, whitelist, metrics
            └─ start_jobs()
                 ├─ start_swarm_coordination_registry_event_listener
                 ├─ start_tracker_core_event_listener
                 ├─ start_http_core_event_listener
                 ├─ start_udp_core_event_listener
                 ├─ start_udp_server_stats_event_listener
                 ├─ start_udp_server_banning_event_listener
                 ├─ start_the_udp_instances        ← one job per configured UDP bind address
                 ├─ start_the_http_instances       ← one job per configured HTTP bind address
                 ├─ start_torrent_cleanup
                 ├─ start_peers_inactivity_update
                 ├─ start_the_http_api
                 └─ start_health_check_api         ← always started
```

Shutdown (`main`): receives `Ctrl-C` → calls `jobs.cancel()` (fires the `CancellationToken`) →
waits up to 10 seconds for all direct component tasks to complete.

## `AppContainer`

`AppContainer` (`container.rs`) is a plain struct — not a framework, not a trait object tree.
It holds one `Arc<…Container>` per architectural layer:

| Field                                                                                            | Layer / Package                                          |
| ------------------------------------------------------------------------------------------------ | -------------------------------------------------------- |
| `registar`                                                                                       | `server-lib` — tracks active server socket registrations |
| `swarm_coordination_registry_container`                                                          | `swarm-coordination-registry`                            |
| `tracker_core_container`                                                                         | `tracker-core`                                           |
| `http_tracker_core_services` / `http_tracker_instance_containers`                                | `http-core`                                              |
| `udp_tracker_core_services` / `udp_tracker_server_container` / `udp_tracker_instance_containers` | `udp-core` / `udp-server`                                |

`AppContainer::initialize` is the only place where domain containers are constructed.
Every `bootstrap/jobs/` starter receives an `&Arc<AppContainer>` and pulls out exactly what it
needs — no globals, no lazy statics for domain objects.

## `JobManager`

`JobManager` (`bootstrap/jobs/manager.rs`) directly owns named `ComponentResult` futures in a
`JoinSet` and shares a `CancellationToken` with them:

- `spawn(name, future)` — directly registers a component runner; do not pass an already-spawned
  `JoinHandle` through a wrapper.
- `cancel()` — fires the token; all jobs that own a clone of it will observe cancellation.
- `wait_for_all(timeout)` — applies one concurrent deadline. It aborts and joins every remaining
  direct component, preventing detached startup jobs.

The torrent-cleanup, activity-metrics, and UDP ban-cleanup jobs retain their pre-existing
starter and cancellation semantics: the first two listen for Ctrl-C and the latter keeps its
existing manager token. Because `JoinSet` cannot adopt a pre-spawned `JoinHandle` without a
forbidden wrapper task, `register_legacy(name, handle)` retains their handles in a narrow
compatibility registry. They share the same process-wide deadline as direct components and are
then aborted and joined; they do not claim `JoinSet` ownership. This transitional exception is
expected to be removed by the SI-4/SI-5 periodic-job migrations. New components must use `spawn`.

Direct components own nested server handles. If the manager aborts an outer component at the
shared deadline, its drop-safe server-task owner signals normal halt, aborts the nested task, and
prevents it from detaching. This is distinct from retained legacy handles: those are manager-owned
but intentionally remain outside the `JoinSet` until their starter APIs are migrated.

## Adding a New Service

When wiring a new server or background task, follow this checklist in order:

1. **Package** — add the new crate under `packages/` with the appropriate layer prefix.
2. **Container field** — add an `Arc<NewServiceContainer>` field to `AppContainer` and
   initialize it inside `AppContainer::initialize`.
3. **Job launcher** — create `src/bootstrap/jobs/new_service.rs` and register it in
   `src/bootstrap/jobs/mod.rs`.
4. **Wire into `app::start_jobs`** — pass the new component runner directly to
   `job_manager.spawn`.
5. **Graceful shutdown** — ensure the new service listens for the `CancellationToken` passed
   from `JobManager`.
6. **Config guard** — if the service is optional, gate direct registration behind the
   appropriate config field.

## Key Rules for This Directory

- **No domain logic here.** This directory is pure wiring. Business rules belong in `packages/`.
- **No globals for domain objects.** All state flows through `AppContainer`.
- **Startup errors are typed.** `bootstrap::app::setup()`, `app::complete_startup()`, and `app::start()` return
  source-preserving `thiserror` errors for expected configuration, composition, persistence-load,
  and initial service-start failures. Entrypoints report their friendly, actionable display message
  and exit unsuccessfully. If an initial service fails after jobs started, `start()` cancels and joins
  those jobs before returning the error. `check_seed()` remains an assertion because it protects an
  internal cryptographic invariant; failures after a task has started are runtime supervision, not
  startup results.
- **Health check always starts.** The health-check API job is unconditional — do not gate it
  behind a config flag.
- **`lib.rs` is the integration-test surface.** Integration tests import
  `torrust_tracker_lib::…`. Keep the public API in `lib.rs` stable; avoid leaking internal
  bootstrap details.
