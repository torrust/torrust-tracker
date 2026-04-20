# `src/` — Binary and Library Entry Points

This directory contains only the top-level wiring of the application: the binary entry points,
the bootstrap sequence, and the dependency-injection container. All domain logic lives in
`packages/`; this directory merely assembles and launches it.

## File Map

| Path                        | Purpose                                                                                                                   |
| --------------------------- | ------------------------------------------------------------------------------------------------------------------------- |
| `main.rs`                   | Binary entry point. Calls `app::run()`, waits for Ctrl-C, then cancels jobs and waits for graceful shutdown.              |
| `lib.rs`                    | Library crate root and crate-level documentation. Re-exports the public API used by integration tests and other binaries. |
| `app.rs`                    | `run()` and `start()` — orchestrates the full startup sequence (setup → load data from DB → start jobs).                  |
| `container.rs`              | `AppContainer` — dependency-injection struct that holds `Arc`-wrapped instances of every per-layer container.             |
| `bootstrap/app.rs`          | `setup()` — loads config, validates it, initializes logging and global services, builds `AppContainer`.                   |
| `bootstrap/config.rs`       | `initialize_configuration()` — reads config from the environment / file.                                                  |
| `bootstrap/jobs/`           | One module per service: each module exposes a starter function called from `app::start_jobs`.                             |
| `bootstrap/jobs/manager.rs` | `JobManager` — collects `JoinHandle`s, owns the `CancellationToken`, and drives graceful shutdown.                        |
| `bin/e2e_tests_runner.rs`   | Binary that runs E2E tests by delegating to `src/console/ci/`.                                                            |
| `bin/http_health_check.rs`  | Minimal HTTP health-check binary used inside containers (avoids curl/wget dependency).                                    |
| `bin/profiling.rs`          | Binary for Valgrind / kcachegrind profiling sessions.                                                                     |
| `console/`                  | Internal console apps (`ci/e2e`, `profiling`) used by the extra binaries above.                                           |

## Bootstrap Flow

```text
main()
  └─ app::run()
       ├─ bootstrap::app::setup()
       │    ├─ bootstrap::config::initialize_configuration()   ← reads TOML / env vars
       │    ├─ configuration.validate()                        ← panics on invalid config
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
waits up to 10 seconds for all `JoinHandle`s to complete.

## `AppContainer`

`AppContainer` (`container.rs`) is a plain struct — not a framework, not a trait object tree.
It holds one `Arc<…Container>` per architectural layer:

| Field                                                                                            | Layer / Package                                          |
| ------------------------------------------------------------------------------------------------ | -------------------------------------------------------- |
| `registar`                                                                                       | `server-lib` — tracks active server socket registrations |
| `swarm_coordination_registry_container`                                                          | `swarm-coordination-registry`                            |
| `tracker_core_container`                                                                         | `tracker-core`                                           |
| `http_tracker_core_services` / `http_tracker_instance_containers`                                | `http-tracker-core`                                      |
| `udp_tracker_core_services` / `udp_tracker_server_container` / `udp_tracker_instance_containers` | `udp-tracker-core` / `udp-tracker-server`                |

`AppContainer::initialize` is the only place where domain containers are constructed.
Every `bootstrap/jobs/` starter receives an `&Arc<AppContainer>` and pulls out exactly what it
needs — no globals, no lazy statics for domain objects.

## `JobManager`

`JobManager` (`bootstrap/jobs/manager.rs`) is a thin wrapper around a `Vec<Job>` (each `Job`
holds a name + `JoinHandle<()>`) and a shared `CancellationToken`:

- `push(name, handle)` — registers a job.
- `push_opt(name, handle)` — convenience for jobs that may be disabled.
- `cancel()` — fires the token; all jobs that own a clone of it will observe cancellation.
- `wait_for_all(timeout)` — joins all handles with a timeout, logging warnings for any that
  exceed it.

## Adding a New Service

When wiring a new server or background task, follow this checklist in order:

1. **Package** — add the new crate under `packages/` with the appropriate layer prefix.
2. **Container field** — add an `Arc<NewServiceContainer>` field to `AppContainer` and
   initialize it inside `AppContainer::initialize`.
3. **Job launcher** — create `src/bootstrap/jobs/new_service.rs` and register it in
   `src/bootstrap/jobs/mod.rs`.
4. **Wire into `app::start_jobs`** — call the new starter function and push its handle to
   `job_manager`.
5. **Graceful shutdown** — ensure the new service listens for the `CancellationToken` passed
   from `JobManager`.
6. **Config guard** — if the service is optional, gate the starter behind the appropriate
   config field and use `push_opt`.

## Key Rules for This Directory

- **No domain logic here.** This directory is pure wiring. Business rules belong in `packages/`.
- **No globals for domain objects.** All state flows through `AppContainer`.
- **Startup errors panic.** `bootstrap::app::setup()` panics on invalid config or a bad crypto
  seed — this is intentional (fail fast before binding ports).
- **Health check always starts.** The health-check API job is unconditional — do not gate it
  behind a config flag.
- **`lib.rs` is the integration-test surface.** Integration tests import
  `torrust_tracker_lib::…`. Keep the public API in `lib.rs` stable; avoid leaking internal
  bootstrap details.
