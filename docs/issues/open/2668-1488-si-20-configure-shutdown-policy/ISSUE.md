---
schema-version: 1
doc-type: issue
issue-type: task
status: in-progress
priority: p2
epic: 1488
github-issue: 2668
spec-path: docs/issues/open/2668-1488-si-20-configure-shutdown-policy/ISSUE.md
branch: "2668-configure-shutdown-policy"
related-pr: 2678
last-updated-utc: "2026-10-28 12:12"
semantic-links:
  skill-links:
    - create-issue
    - write-unit-test
    - handle-errors-in-code
  related-artifacts:
    - packages/configuration/src/v3_0_0/mod.rs
    - packages/configuration/src/v3_0_0/types.rs
    - packages/configuration/src/v3_0_0/udp_tracker_server.rs
    - packages/configuration/src/validator.rs
    - packages/events/src/drain.rs
    - packages/axum-server/src/signals.rs
    - packages/axum-http-server/src/server.rs
    - packages/axum-rest-api-server/src/server.rs
    - packages/axum-health-check-api-server/src/server.rs
    - packages/udp-server/src/server/launcher.rs
    - src/main.rs
    - src/app.rs
    - src/bootstrap/jobs/manager.rs
    - tests/lifecycle/signals.rs
    - share/default/config/tracker.container.sqlite3.toml
    - share/default/config/tracker.development.sqlite3.toml
    - docs/containers.md
    - docs/architecture/application-jobs.md
    - docs/architecture/tracker-instance-architecture.md
    - docs/architecture/glossary.md
    - packages/configuration/AGENTS.md
    - docs/adrs/20260721000000_make_network_configuration_per_tracker_instance.md
    - docs/adrs/20260727180000_shared_services_across_tracker_instances.md
    - docs/features/shutdown-process/README.md
    - docs/features/shutdown-process/questions.md
    - docs/adrs/20261021184946_order_shutdown_by_event_flow_and_drain_event_listeners.md
    - docs/adrs/20260723184019_separate_configuration_value_invariants_from_consistency_validation.md
    - docs/issues/closed/1586-evaluate-job-manager-join-set/ISSUE.md
    - docs/issues/closed/2410-1488-si-22-process-queued-events-before-listeners-stop/EPIC.md
    - docs/issues/drafts/1488-si-8-configurable-grace-periods/ISSUE.md
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
---

<!-- skill-link: create-issue -->

# Issue #2668 - Configure Shutdown Policy and Deployment Contract (SI-20)

Parent: [EPIC #1488 - Overhaul: Tracker Shutdown](../1488-overhaul-tracker-shutdown/ISSUE.md)

> **EPIC position**: Roadmap sequence 20 (SI-20). Final policy and configuration task. It applies
> the approved Q3 and Q4 outcomes to the lifecycle that sequences 0 to 17 and sub-EPIC SI-22
> (#2410) built. It has no dependency on SI-18, SI-19, or SI-21.

## Goal

Make the shutdown policy real: expose the shutdown budgets as validated tracker configuration,
deliver them to the supervisor and to every component that drains, make the process exit code
report the shutdown outcome, and document the deployment deadlines an operator must set.

Today a shutdown that times out, fails, or is aborted still exits with code `0` and logs
"Torrust tracker successfully shutdown." (read from source, not run). This task makes that
outcome visible to Docker, systemd, Kubernetes, CI, and scripts.

This task configures an existing lifecycle. It does not change cancellation propagation, stop
order, component ownership, or join and abort behavior.

## Background

### Refresh note (2026-10-27)

The previous draft (2026-10-02) predates sub-EPIC SI-22 (#2410), SI-23, and SI-24. This refresh
re-derived every fact from `develop` at the merge of PR #2664. What changed from the draft:

| Draft statement                                                                       | Now                                                                                                                                                         |
| ------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------- |
| "Both 90 s HTTP drain constants"                                                      | The HTTP tracker and REST API drains are 90 s; the health-check API drain is already 5 s, set with the comment "SI-20 owns configurable budgets".          |
| Three budgets (process, HTTP drain, UDP request)                                      | A fourth: the event-listener drain admission budget (`DEFAULT_DRAIN_TIMEOUT`, 5 s, seven listeners). SI-22 injects it through constructors; SI-20 owns it. |
| "A component budget below the process deadline" is enough                             | Not enough. Listeners stop only after their producers finish, so the worst case is a producer drain plus a listener drain (see "Budget model").            |
| Approved defaults 25 s / 20 s / 5 s validate                                          | With a 5 s listener budget, 20 s + 5 s = 25 s leaves no slack. The default HTTP drain needs to change or the process deadline must grow (decision D1).      |
| One hard-coded 10 s deadline, in `main`                                               | Two production sites: `src/main.rs` and the startup-failure cleanup in `src/app.rs`. The developer-only profiler (`src/console/profiling.rs`) has a third.  |
| Cancellation tree of one root token                                                   | Per-component tokens and stop-before edges; the ADR of 2026-10-21 supersedes the cancellation-tree ADR.                                                     |
| Evidence in `verification.md`                                                         | `manual-verification-evidence.md`, created from the template when evidence is recorded; the skeleton `verification.md` is removed.                          |
| Top-level `[shutdown]` section                                                        | `[application.shutdown]` in a generic application-level section (see "Configuration Placement").                                                            |

### Current state (read from source, not run)

All rows are read from the source at the merge of PR #2664 and have not been run. The first
implementation task records a runtime baseline.

| Fact                                                                                                         | Source                                                                                                                  |
| ------------------------------------------------------------------------------------------------------------ | ----------------------------------------------------------------------------------------------------------------------- |
| `main` waits `Duration::from_secs(10)`, discards the returned `Vec<JobOutcome>`, and logs a success message. | `src/main.rs` (`jobs.wait_for_all(Duration::from_secs(10))`)                                                            |
| A startup failure after some components started cancels them and waits a hard-coded 10 s.                    | `src/app.rs` `start_jobs`                                                                                               |
| Startup failure exits with code 1 through `std::process::exit(1)` in `main`.                                 | `src/main.rs`                                                                                                           |
| `JobManager::wait_for_all(grace_period)` computes `Instant::now() + grace_period` without a checked add.     | `src/bootstrap/jobs/manager.rs`                                                                                         |
| HTTP tracker drain budget is 90 s (`HTTP_GRACEFUL_DRAIN_TIMEOUT`).                                           | `packages/axum-http-server/src/server.rs`                                                                               |
| REST API drain budget is 90 s (`API_GRACEFUL_DRAIN_TIMEOUT`).                                                | `packages/axum-rest-api-server/src/server.rs`                                                                           |
| Health-check API drain budget is 5 s (`HEALTH_CHECK_API_GRACEFUL_DRAIN_TIMEOUT`).                            | `packages/axum-health-check-api-server/src/server.rs`                                                                   |
| UDP accepted-request drain budget is 5 s (`REQUEST_DRAIN_DEADLINE`), private to the launcher.                | `packages/udp-server/src/server/launcher.rs`                                                                            |
| Seven event listeners get `DEFAULT_DRAIN_TIMEOUT` (5 s), an admission budget checked between events.        | `packages/events/src/drain.rs`; `src/bootstrap/jobs/{torrent_repository,tracker_core,http_tracker_core,udp_tracker_core,udp_tracker_server}.rs` |
| A drain that reaches its budget makes the HTTP, REST API, and health-check components return `Err(ComponentError)`, which the supervisor reports as `JobStatus::Failed`. | `src/bootstrap/jobs/{http_tracker,tracker_apis,health_check_api}.rs`                                                    |
| The drain loop polls every second and force-closes at its budget.                                            | `packages/axum-server/src/signals.rs` `graceful_shutdown_on_cancellation`                                               |
| Stop-before edges: HTTP and UDP trackers and torrent cleanup stop before the listeners of the buses they publish on; the REST API, health-check API, activity metrics and ban cleanup have no edges. | `src/app.rs`; ADR `20261021184946` section 4                                                                            |
| The legacy `graceful_shutdown` keeps 90 s and 95 s constants. SI-19 removes it.                              | `packages/axum-server/src/signals.rs`                                                                                   |
| The container entry script ends with `exec /bin/su-exec torrust "$@"`, so the tracker replaces the shell.    | `share/container/entry_script_sh`                                                                                       |
| `docs/containers.md` does not mention a stop or grace period at all.                                         | `docs/containers.md`                                                                                                    |

### The defect in plain terms

This task also corrects one defect, so a reviewer can follow it without reading the code:

1. The operator sends SIGTERM or SIGINT. `src/main.rs` asks the supervisor to stop every component and waits at most 10 s for them.
2. Some component may fail, or may still be running at the deadline and be aborted. The supervisor records that as a named outcome (`Failed`, `Panicked`, or `Aborted`) and logs it as a warning.
3. `main` discards the returned outcomes, logs "Torrust tracker successfully shutdown.", and returns, so the process exits with code `0`.

Q3 of `docs/features/shutdown-process/questions.md` requires exit code `1` in step 2, and the shutdown-order ADR repeats it. The impact: Docker, systemd, Kubernetes, CI, and scripts read exit code `0` as a clean stop although a component failed or was cut off, and the success line in the log says the same. The wrong outcome was observed on the real binary with one component made to fail or outlast the deadline by a temporary patch (V1 of `manual-verification-evidence.md`, classified Reproduced with that patch). Four network attempts on the unmodified binary did not produce a failed or aborted shutdown, so the specification does not claim a reproduction without the patch.

### Budget model

The ADR orders shutdown by event flow. A listener is asked to stop only after every producer
that publishes on its bus has completed, and its drain budget is an admission budget: an event
already taken from the queue is not cut short. So the worst case of a normal shutdown is two
sequential stages, not one:

```text
t = 0                       t = max(H, U)                   t = max(H, U) + L (+ one in-flight event)
|-- stage 1: producers -----|-- stage 2: listeners drain ---|
   HTTP trackers   (<= H)      swarm, tracker-core (2),
   UDP trackers    (<= U)      http-core, udp-core,
   REST API        (<= H)      udp-server (2) listeners  (<= L)
   health check    (<= H)
```

Q4 states `T_component < T_process`. That holds per component but not for the chain, so this
task validates the chain: `max(H, U) + L < P`, where `P` is the process deadline, `H` the
HTTP-family drain, `U` the UDP request drain, and `L` the listener drain. The strict inequality
leaves at least one second for the in-flight event of a listener and for task joins.

With the Q4 numbers and the listener budget SI-22 chose (P = 25, H = 20, U = 5, L = 5), the chain
is `20 + 5 = 25`, which is not below 25: an HTTP drain that uses its whole budget leaves the
listeners no time, and the supervisor aborts them at the deadline. That is the loss SI-22 set
out to prevent, now reachable through the deadline instead of the stop order. See decision D1.

## Configuration Placement

The previous draft added a top-level `[shutdown]` section. That is too specific: the next
process-wide setting would need another root section. This refresh looked for where
process-wide settings belong.

### What the documents already say

| Source                                                                                                    | Statement                                                                                                                                                                                                                  |
| --------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [Tracker instance architecture](../../../architecture/tracker-instance-architecture.md), "Shared Services" | "when [shutdown policy] is introduced, process-wide policy belongs in an application-level v3 configuration section, while component-specific budgets remain owned by their lifecycle contracts."                          |
| Same document, "Configuration Placement Rule"                                                             | A setting goes on a listener only when each listener can apply it independently without changing shared state; shared or once-per-process concerns go on `core` or another shared-service section.                          |
| [Shutdown-order ADR](../../../adrs/20261021184946_order_shutdown_by_event_flow_and_drain_event_listeners.md) and [glossary](../../../architecture/glossary.md) | The composition root (`src/app.rs`, `src/bootstrap/`) owns wiring, the event buses, and the shared deadline; `JobManager` is the application supervisor.                                                                  |
| `packages/configuration/AGENTS.md`                                                                        | Typed newtypes for constrained fields, `deny_unknown_fields`, defaults through associated functions. Existing sections are named after what they configure (`core`, `udp_tracker_server`, `health_check_api`, `logging`). |

No section exists today for the root crate itself. `[logging]` is the only process-wide section,
and `[core]` mixes tracker policy with application job settings
(`inactive_peer_cleanup_interval`).

### Candidate application-level values found in source (read from source, not run)

| Value                                                                 | Where it is today                                                                                                  | Verdict                                                                                                     |
| --------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------ | ----------------------------------------------------------------------------------------------------------- |
| Process shutdown deadline; two hard-coded 10 s sites                  | `src/main.rs`, `src/app.rs`                                                                                        | **This issue**: `[application.shutdown]`.                                                                   |
| HTTP, REST API, health-check drain; UDP request drain; listener drain | Constants in four server crates and `DEFAULT_DRAIN_TIMEOUT` in `packages/events/src/drain.rs`                      | **This issue**: `[application.shutdown]` (decision D2).                                                     |
| Startup limits                                                        | None. Binding fails fast, the initial persistence load has no timeout, and no startup deadline exists.             | **Nothing to configure.** Reserve the shape (`[application.startup]` fits later); add no empty table.      |
| Event-bus capacity                                                    | `CHANNEL_CAPACITY = 65536` in `packages/events/src/broadcaster.rs`, four buses created by the composition root     | Future candidate (`[application.events]`). It sets memory and the backlog a listener drain must persist; #2418 is the related remedy. Not here. |
| Activity-metrics update interval                                      | `ACTIVITY_METRICS_UPDATE_INTERVAL_SECS = 15` with a `// todo: make this configurable`                              | Future candidate for an application-job schedule. Not here.                                                 |
| Periodic job schedules already configurable                           | `core.inactive_peer_cleanup_interval`, `udp_tracker_server.ip_bans_reset_interval_in_secs`                         | Stay where they are; moving them is a breaking change outside this issue.                                   |
| Logging                                                               | `[logging]`                                                                                                         | A sibling today. Consolidating it under the application section is a separate decision (D10).               |
| UDP active-request ring capacity                                      | `ACTIVE_REQUESTS_CAPACITY = 50` in `packages/udp-server/src/server/request_buffer.rs`                              | A UDP server concern, not application level.                                                                |
| Handled signals, exit codes                                           | Fixed by Q3, Q5, and the ADR                                                                                       | A contract, not a setting.                                                                                  |

### Options

| Option                                                                 | Shape                                                                                                | Assessment                                                                                                                                                                                                                      |
| ---------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| P1: top-level `[shutdown]`                                             | The previous draft.                                                                                  | Rejected: too specific, and each future process-wide concern adds a root section.                                                                                                                                               |
| **P2: `[application]` root section with an `[application.shutdown]` sub-table** | Recommended. One root section for the application's own policy; `shutdown` now, `startup`, `events`, or job schedules when a concrete value exists. | Matches the architecture document's "application-level section", keeps all four budgets in one place so the chain rule is checkable, and reads like `AppContainer`, `app.rs`, and `application-jobs.md`.                         |
| P3: split by owner                                                     | Process deadline in `[application.shutdown]`; each drain budget on its own service section (`[[http_trackers]]`, `[http_api]`, `[health_check_api]`, `[udp_tracker_server]`) and the listener budget in `[core]`. | Follows the second half of the architecture sentence literally, but scatters one contract over five sections, multiplies per-instance values for `[[http_trackers]]`, and spreads the chain rule across sections. No demonstrated need for per-service budgets. |

Candidate names for the root section, with the case for and against each. The maintainer asked
for the full list, including `main`.

| Name             | For                                                                                                                                                      | Against                                                                                                                                                                                         |
| ---------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **`application`** | The term the architecture guide uses ("an application-level v3 configuration section"); matches `AppContainer`, `app.rs`, `application-jobs.md`; names a concept, not a file. | Longer to type; "application" is also the generic word for the whole program, so it could be read as "everything" (the section holds only what is not owned by a service or the domain).        |
| `app`            | Same meaning as `application`; shorter; matches `app.rs`, `bootstrap::app`, `AppContainer`.                                                               | Abbreviation; the existing sections spell names out (`health_check_api`, `udp_tracker_server`); a lone abbreviation would be inconsistent.                                                      |
| `main`           | Short; the root crate's entry point is `src/main.rs`, where signals map to a shutdown request and where the exit code is decided; everything this issue configures is reached from `main`. | Names a file, not a concept: the composition root is `app.rs` and `bootstrap/`, not `main.rs`, and the event buses, which later settings may configure, are built there. "main" also reads as "primary" (the main tracker, the main branch), and `[main]` would suggest the other sections are secondary. |
| `process`        | Exact for signals, deadlines, and exit codes; operators think in processes when they set `stop_grace_period`.                                              | Too narrow for non-process settings such as event-bus capacity or job schedules.                                                                                                                  |
| `lifecycle`      | Describes `startup` and `shutdown` precisely, and `[lifecycle.shutdown]` reads naturally.                                                                   | Excludes everything that is not a lifecycle phase (event-bus capacity, activity-metrics interval), so a second root section would be needed again.                                               |
| `supervisor`     | Names `JobManager`, the component that applies the shared deadline.                                                                                      | Internal type name leaked into operator configuration; the budgets are consumed by servers and listeners, not by the supervisor alone.                                                          |
| `bootstrap`      | Matches `src/bootstrap/`, which composes the application.                                                                                                 | Means startup only; shutdown is not bootstrap, so `[bootstrap.shutdown]` would be contradictory.                                                                                                 |
| `runtime`        | Used by the documents ("runtime architecture", `RuntimeServiceMetadata`).                                                                                 | Collides with the Tokio runtime in operator minds and in code reviews; unclear whether it means the async runtime.                                                                              |
| `tracker`        | The product name.                                                                                                                                         | Collides with `[[http_trackers]]`, `[[udp_trackers]]`, and `tracker_policy`; every section is "tracker" configuration.                                                                           |
| `orchestrator`   | The word the maintainer used for the root crate's role.                                                                                                   | This specification and the deployment documents already use "orchestrator" for Docker, Kubernetes, and systemd (`T_orchestrator`), so `[orchestrator.shutdown]` would be ambiguous in the same page. |

Recommendation: `application`. The maintainer decides (D2, D10).

With P2 the architecture sentence is read as follows: the process-wide shutdown policy, including
the numbers the component lifecycle contracts consume, is operator policy and lives in
`[application.shutdown]`; each component keeps owning how it applies the number it receives. T7
updates that sentence.

## Proposed Policy and Configuration Contract

### Defaults

| Budget                                                   | Key                            | Q4 default               | Proposed default           |
| -------------------------------------------------------- | ------------------------------ | ------------------------ | -------------------------- |
| Process shutdown deadline (`P`), one concurrent deadline | `deadline_in_secs`             | 25 s                     | 25 s                       |
| HTTP tracker, REST API, health-check API drain (`H`)     | `http_drain_in_secs`           | 20 s                     | **15 s** (D1)              |
| UDP accepted-request completion (`U`)                    | `udp_request_drain_in_secs`    | 5 s                      | 5 s                        |
| Event-listener drain admission (`L`), all seven listeners | `event_listener_drain_in_secs` | none (SI-22 default 5 s) | 5 s                        |
| Orchestrator grace period (documentation only)           | not configurable               | 30 s min                 | 30 s min, 35 s recommended |

### Schema

```toml
[application.shutdown]
deadline_in_secs = 25
http_drain_in_secs = 15
udp_request_drain_in_secs = 5
event_listener_drain_in_secs = 5
```

- The `[application]` section, its `shutdown` sub-table, and each key are optional; missing values
  take the defaults above, so every existing configuration file keeps working. The schema
  version stays `3.0.0` because the addition is optional and `develop` carries `3.0.0-develop`
  (D8).
- Key names follow the repository's `_in_secs` convention
  (`ip_bans_reset_interval_in_secs`), not the draft's `_secs`. Inside `shutdown`, the deadline
  is just `deadline_in_secs`.
- Environment overrides use the existing nested form, for example
  `TORRUST_TRACKER_CONFIG_OVERRIDE_APPLICATION__SHUTDOWN__DEADLINE_IN_SECS=30` (read from
  source, not run; M6 runs it). Containers usually set values this way, so the deployment
  documentation shows it.
- Rust shape: `Configuration.application: Application` with `#[serde(default)]`, and
  `Application { shutdown: Shutdown }`, in `packages/configuration/src/v3_0_0/application.rs` (the
  `Shutdown` type may live in its own `application/shutdown.rs`). Both structs carry
  `deny_unknown_fields`; defaults come from associated functions, per the package rules.

### Validation

Following the
[ADR on value invariants and consistency validation](../../../adrs/20260723184019_separate_configuration_value_invariants_from_consistency_validation.md):

| Layer                     | Rule                                                                                  | Mechanism                                                          |
| ------------------------- | ------------------------------------------------------------------------------------- | ------------------------------------------------------------------ |
| Value invariant           | Every value is at least 1 s and at most 3600 s.                                       | Constrained newtypes built on `AtLeastU64`, rejected at load time. |
| Configuration consistency | `max(H, U) + L < P`.                                                                  | `Validator` and a new `SemanticValidationError` variant.           |

The upper bound is not cosmetic: `wait_for_all` adds the deadline to `Instant::now()`
unchecked, so an unbounded `u64` of seconds can panic at shutdown (the same latent defect SI-22
found and fixed for listener drains; read from source, not run). The bound keeps every
configured value representable (D4).

Errors name the offending keys and values and say what to change, for example:

```text
application.shutdown.http_drain_in_secs (20) plus application.shutdown.event_listener_drain_in_secs (5)
must be less than application.shutdown.deadline_in_secs (25): event listeners are stopped only
after the HTTP servers finish draining. Lower the drain budgets or raise the deadline.
```

When the UDP request drain is the larger stage, the message names `udp_request_drain_in_secs` and
its value in place of the HTTP key.

The tracker cannot validate the orchestrator's deadline from inside its process. The deployment
contract below states the external rule instead.

### Where each value goes

| Value | Consumer                                                                                                          |
| ----- | ----------------------------------------------------------------------------------------------------------------- |
| `P`   | `JobManager::wait_for_all` in `main` and in the startup-failure cleanup of `src/app.rs`.                          |
| `H`   | `graceful_shutdown_on_cancellation` of the HTTP tracker, REST API, and health-check API servers.                  |
| `U`   | The UDP receive loop's accepted-request drain (`start_receive_loop`).                                             |
| `L`   | The seven listener constructors, all given the same value (their `drain_timeout` parameter already exists).       |

The maintainer decided D5 at the T2 design review: **inject top to bottom**. The value is
configuration, so it is passed from the composition root to whoever applies it, and the signatures
change where needed (version 3.0.0 of the configuration package is not published, and the server
crates are `0.1.0`). A callee that needs only a value takes that value, a `Duration`, not the
`Shutdown` section, which keeps the coupling low. A public constant for the default exists only
where injection would add a dependency that the callee should not have. The design is
re-evaluated for cohesion and coupling after T7, before the pull request (see the Progress Log).

In the composition root every value comes from one holder, `AppContainer.application_config`
(`Arc<Application>`), including the seven listener starters (the maintainer decided this during the
pull-request review, review of PR #2678): a starter takes the `AppContainer` it already has, and
takes the `Configuration` only when it reads other settings from it (`torrent_repository`, and the
two `tracker_core` listeners read `core`). The job starters of the HTTP tracker, the REST API, the
health check and the UDP tracker receive a `Duration` parameter and read no configuration, as D5
asks; their tests that already build an `AppContainer` take the value from it.

## Exit-Result Contract

`JobManager::wait_for_all` already returns named outcomes. A pure function in the root crate
maps them, and `main` returns the result as a `std::process::ExitCode` (D9):

| Outcome set                                                                  | Exit code |
| ---------------------------------------------------------------------------- | --------- |
| Every outcome is `Completed` or `Cancelled`                                  | `0`       |
| Any outcome is `Failed` (including an HTTP-family drain budget reached), `Panicked`, or `Aborted` (deadline abort) | `1`       |
| Startup cannot complete                                                      | `1`       |
| SIGKILL or another signal the process cannot handle                          | OS-defined (`128 + n` in a shell) |

Components keep returning outcomes to their owner and never call `std::process::exit`. Only the HTTP-family drains (HTTP tracker, REST API, health check) return an error when their budget is reached. The UDP request drain and the event listener drain log a warning when their budget expires and still end as `Completed` or `Cancelled`, so exit `0` does not prove that no UDP request or queued event was discarded (review finding `review-finding:pr-2678-f1`).
"Torrust tracker successfully shutdown." is logged only on exit code `0`; otherwise `main` logs
the failing outcomes by name and an error line.

## Deployment Contract

`T_orchestrator >= P + 5 s`. For the defaults that is 30 s minimum; recommend 35 s or more.

| Runtime               | Setting                                                    | Minimum | Recommended |
| --------------------- | ---------------------------------------------------------- | ------- | ----------- |
| Docker / Podman       | `docker run --stop-timeout` or `docker stop --time`        | 30 s    | 35 s        |
| Docker Compose        | `stop_grace_period`                                        | 30s     | 35s         |
| Kubernetes            | `terminationGracePeriodSeconds`                            | 30      | 35          |
| systemd               | `TimeoutStopSec`                                           | 30      | 35          |

Docker and Podman default to 10 s, which is shorter than the default process deadline: they
send SIGKILL (exit status 137) while the tracker still drains. Documentation must say that plainly.
Raising `P` requires raising the orchestrator value by the same amount. A non-zero exit after a
requested stop makes a systemd unit `failed`; do not list it in `SuccessExitStatus`.

## Scope

### In Scope

- An `[application]` section with an `[application.shutdown]` sub-table (see "Configuration
  Placement"), defaults, value-invariant newtypes, and the `max(H, U) + L < P` consistency rule,
  in `packages/configuration`, with module documentation.
- Deliver `P`, `H`, `U`, and `L` as described in "Where each value goes", without per-job process
  deadlines.
- Replace the hard-coded 10 s in `src/main.rs` and in the startup-failure cleanup of `src/app.rs`.
- The pure outcome-to-exit-code function, `main` returning `ExitCode`, and the
  conditional success log.
- Default configuration fixtures: add the `[application.shutdown]` block to the container and
  development fixtures under `share/default/config/`.
- Deployment documentation: `docs/containers.md` (Docker, Podman, Compose, Kubernetes, systemd,
  the insufficient 10 s default, and the environment-override form),
  `docs/features/shutdown-process/README.md`, and `docs/architecture/application-jobs.md` (it
  still says "ten-second").
- Update the statements that the new configuration makes stale: the shutdown sentence and the
  placement rule in `docs/architecture/tracker-instance-architecture.md`, the "Shared deadline" and
  "Drain timeout" entries of `docs/architecture/glossary.md` ("10 seconds today", "90-second"),
  the `DEFAULT_DRAIN_TIMEOUT` documentation in `packages/events/src/drain.rs` (it says the
  default is "below the 10-second shared shutdown deadline"), the section list in the
  `packages/configuration` module documentation, and a note in
  `packages/configuration/docs/migrate-v2-to-v3.md` that the section is new and optional.
- Amend Q4 in `docs/features/shutdown-process/questions.md` with the chain rule and the new
  defaults, and point Q9's evidence location to `manual-verification-evidence.md` (D7).
- Deterministic unit tests, native-binary tests with real signals, and the manual evidence in
  the Verification Plan.
- Review the repository's own Compose files for a tracker service stopped with the 10 s default.

### Out of Scope

- Cancellation propagation, stop order, lifecycle ownership, and join or abort behavior.
- Removing legacy APIs, the legacy `graceful_shutdown` 90 s constants, and OS-signal helpers
  (SI-19), or deprecating them (SI-18).
- Readiness during shutdown (SI-21). It must use part of `P` and add no timer of its own.
- Batching persisted download writes (#2418), the remedy for a listener that cannot persist a
  full buffer within `L`.
- Per-listener drain budgets, per-instance drain budgets, and automatic discovery of the
  orchestrator's deadline.
- Other application-level values (event-bus capacity, the activity-metrics interval), a
  `startup` sub-table, and moving `[logging]` or the `[core]` job intervals under
  `[application]` (D10). They are recorded as candidates in "Configuration Placement".
- The developer-only profiler binary (`src/console/profiling.rs`, its own 10 s) and the
  integration-test fixture constant in `tests/common/workspace.rs`. Both keep their literal
  values; they are not operator-facing.
- Changing the `JobManager` stop-order mechanism or the `EventFlows` registry.

## Architectural Decisions

- Related ADRs: [Order shutdown by event flow and drain event listeners](../../../adrs/20261021184946_order_shutdown_by_event_flow_and_drain_event_listeners.md)
  (it leaves the budgets and exit-result policy to this task);
  [Separate configuration value invariants from consistency validation](../../../adrs/20260723184019_separate_configuration_value_invariants_from_consistency_validation.md);
  [Make network configuration per tracker instance](../../../adrs/20260721000000_make_network_configuration_per_tracker_instance.md)
  and [Shared services across tracker instances](../../../adrs/20260727180000_shared_services_across_tracker_instances.md)
  (the placement rule); the superseded
  [cancellation-tree ADR](../../../adrs/20260902074438_adopt_supervised_cancellation_tree_for_shutdown.md)
  for the Q3 and Q4 policy it recorded.
- ADRs to create, if the maintainer approves D2, D7, and D10: (1) a short root-collection ADR
  recording the application-level `[application]` section and the rule for what belongs in it;
  (2) a short root-collection ADR recording the budget model (chained stages, the
  `max(H, U) + L < P` rule, and the amended defaults). The placement ADR is the lasting rule for
  later contributors, so it is not folded into the Q4 amendment.

## Design and Ownership Review

This task touches shutdown timing and process exit, so these concerns apply.

- **Responsibilities**: `packages/configuration` owns the values, their invariants, and the
  consistency rule. The root crate owns delivery to the supervisor and the outcome-to-exit
  mapping. Each server crate owns only how it applies a drain `Duration` it is given; none reads
  configuration directly.
- **Ownership and lifetimes**: values are `Copy` durations read once at startup and passed by
  value. No new task, token, or channel is introduced. A component still joins or aborts its
  children before it reports.
- **Absolute deadline coverage**: `P` is one `Instant` computed once in `wait_for_all`. Every
  test or script wait is bounded by a timeout shorter than the test's own budget; a native test
  that spawns the binary kills and reaps it on every path (the existing fixture does).
- **Failure and drop paths**: a failed drain and a deadline abort map to exit `1`; startup
  failure keeps `1`; a dropped native-test fixture force-kills the child.
- **Checkpoint**: after the first implementation step that changes the configuration schema
  (T2), stop for a design review with the maintainer before wiring it into the supervisor (D5).

## Bug-Fix Process

The work is mainly a configuration task (`issue-type: task`), but one part of it is substantively a
bug: exit code `0` after a failed or aborted shutdown. That part follows
[fix-bug](../../../../.github/skills/dev/debugging/fix-bug/SKILL.md):

| Step                                | Status                                                                                                                                                                                                                       |
| ----------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Analysis and hypothesis             | Done. `main` discards the supervisor outcomes and always logs success (`src/main.rs`, read from source). Hypothesis: a failed or aborted component still gives exit `0`.                                                      |
| Real-artifact reproduction          | Done before review: **Reproduced, with a temporary fault-injection patch** (V1 in [manual-verification-evidence.md](manual-verification-evidence.md)). Exit `0` and the success message for an aborted component and for a failed one, with SIGTERM and SIGINT. Four network attempts on the unmodified binary did not fail a component, and the evidence says so. |
| Regression-test boundary            | A unit test of a pure outcome-to-exit-code function, one case per `JobStatus` (T3a). Native-binary tests in `tests/lifecycle/` cover the executable boundary (T6).                                                            |
| Red test before the fix             | T3a adds the function with the current behavior (always success) and the test, runs it red, and records the output in the evidence file.                                                                                      |
| Fix                                 | T3b implements the mapping and makes `main` return it.                                                                                                                                                                       |
| Green test and like-for-like recheck | T3b records the green run. T8 reruns V1's runs B1 to B4 after the last production change, with the same temporary patch re-applied (not committed), expecting exit `1`, and names the commit subject it ran on.                |

## Regression Test Strategy

- **Selected boundary**: a unit test of the pure function that maps `Vec<JobOutcome>` to the exit code
  (parameterized, one case per `JobStatus`, plus an empty outcome list and a mix of outcomes). It is
  the smallest deterministic maintained test at the causal seam: `main` discards the outcomes
  today, and the test needs no OS signal and no network.
- **Why not only a higher boundary**: a native-binary test cannot make a component fail or outlast
  the deadline deterministically (V1, A1 to A4), so it covers the executable boundary (a clean stop
  and a reached drain budget) and not every outcome. The unit test covers every outcome.
- **Red evidence**: T3a records the failing run against the always-success function. If that proves
  impractical, the mutate-then-restore method of the `write-unit-test` skill applies: reintroduce
  the always-success mapping, observe the test fail, restore, and prove the restore with `cmp`.
- **Every other new test** is mutation-proven the same way: show it fails when the guarded behavior
  is removed, then restore the code and prove the restore with `cmp`.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                  | Notes / Expected Output                                                                                                                                                                                      |
| --- | ------ | ------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| T1  | DONE   | Pre-review reproduction                | Done before review and recorded as V1 in `manual-verification-evidence.md`: Reproduced with a temporary fault-injection patch; four network attempts on the unmodified binary did not fail a component. Rerun only if production code under `src/` changes on `develop` before T3a starts. |
| T2  | DONE   | `[application.shutdown]` configuration section | `application.rs` and `shutdown.rs` with the validated `ShutdownBudgetInSecs` newtype (1 to 3600 s), defaults 25 / 15 / 5 / 5, the `Validator` chain rule with its error variant, module docs, and 25 new unit tests, each mutation-proven (see "T2" in `manual-verification-evidence.md`). **Stopped for the design review (D5) before any wiring into the supervisor.** |
| T3a | DONE   | Regression test, red                  | Pure outcome-to-exit-code function returning the current behavior (always success) and a parameterized test per `JobStatus`; the recorded red run fails for `Failed`, `Panicked`, and `Aborted`.               |
| T3b | DONE   | Fix: outcome-to-exit mapping          | Implement the mapping, `main` returns `ExitCode`, success log only on `0`, failing outcomes logged by name; record the green run.                                                                              |
| T4  | DONE   | Deliver `P` and `L`                   | Configured `P` in `main` and in the startup-failure cleanup; `L` into the seven listener constructors (read from the application container at each site; see "Where each value goes"); tests that a configured value, not the default, reaches each consumer.                                  |
| T5  | DONE   | Deliver `H` and `U`                   | Injected a `Duration` from the composition root into the HTTP, REST API, health-check, and UDP server crates; the standalone test environments receive the configuration default and the examples use those environments, so no second set of defaults remains (D5). One bounded real-time test per HTTP-based crate (a connection held open under a 300 ms drain timeout must end the drain as timed out within 5 s; paused time was not used because the crates do not enable tokio `test-util`), the UDP shutdown scenarios now start through the public token-aware start, and the drain helper bounds an oversized timeout. Evidence section "T5". |
| T6  | DONE   | Native-binary tests                   | `tests/lifecycle/shutdown_policy.rs`, registered as the `lifecycle-shutdown-policy` target: exit `0` on a clean stop with the default 25 s deadline (SIGTERM and SIGINT); the configured 7 s deadline in the supervisor log; exit `1` with a held `partial-header` connection under a 2 s HTTP drain (SIGTERM and SIGINT) with a 10 s deadline instead of the 5 s of M3, because a 5 s deadline equal to the connection's header timeout cannot tell a configured drain from the deadline; exit `1` and the configured deadline when a service fails to start; a zero deadline and a chain-rule violation rejected before any service starts; the largest valid HTTP drain accepted. Evidence section "T6". |
| T7  | DONE   | Fixtures and documentation            | The five container and development templates carry the `[application.shutdown]` block (a test pins them to the defaults); `docs/containers.md` has a "Stopping the Tracker" section; the E2E Compose files set `stop_grace_period: 35s`; the architecture documents, glossary, feature README, `drain.rs` documentation and migration guide no longer state the 10 s and 90 s values; Q4 is amended and Q9 points to the evidence; two root ADRs (placement rule, budget model) are registered. Evidence section "T7". |
| T8  | DONE   | Manual verification evidence          | `manual-verification-evidence.md` sections V2 to V8 (M2 to M8) and the like-for-like recheck of V1 (B1 to B4, in V5), run after the last production change (`refactor(configuration): read each shutdown budget through a named accessor`). Podman was not run. |
| T9  | DONE   | Independent review and completion     | `review-task` review recorded in `agent-review-reports.md` (first entry `REVIEW FAILED` for documentation findings, re-review `REVIEW PASSED`); retrospective written; disposable script removed. The acceptance criteria are re-reviewed against evidence but not ticked: that is the maintainer's. |

## Commit Points

| Task | Coherent change set                                                                         | Commit policy                                        |
| ---- | ------------------------------------------------------------------------------------------- | ---------------------------------------------------- |
| T1   | Evidence only: `manual-verification-evidence.md` and `evidence/` (script and configuration)  | Committed with this specification.                   |
| T2   | New `application.rs` and `shutdown.rs`, `Configuration` field, default, validator rule, tests, module docs | Commit after focused validation; stop for review.    |
| T3a  | The function, its parameterized test, and the recorded red run                              | Commit after the red run is recorded.                |
| T3b  | The mapping, `main` change, and the recorded green run                                      | Commit after focused validation and required review. |
| T4   | `P` and `L` delivery with tests                                                             | Commit after focused validation and required review. |
| T5   | One commit per server crate (HTTP, REST API, health check, UDP) with its tests              | Commit after focused validation and required review. |
| T6   | Native-binary tests                                                                         | Commit after focused validation and required review. |
| T7   | Fixtures; docs; Q4 amendment and the approved ADRs as separate commits                      | Commit after `linter all`.                           |
| T8   | Evidence file completed                                                                     | Commit after the last production change.             |
| T9   | Review report, acceptance review, completion entry                                          | Commit after the independent review.                 |

For test-producing tasks use the `write-unit-test` skill: behavior-driven names, Arrange-Act-Assert,
every wait bounded, and the prose-first Arrange-Act-Assert comparison recorded in the evidence.
Sign every commit with GPG. Any new error enum follows the `handle-errors-in-code` skill.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted (moved to `docs/issues/open/2668-1488-si-20-configure-shutdown-policy/ISSUE.md`)
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec (#2668)
- [x] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation
- [x] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [x] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [x] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [x] Independent reviewer reports recorded in issue-local `agent-review-reports.md`
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-10-02 15:42 UTC - Previous draft last updated (before SI-22, SI-23, and SI-24).
- 2026-10-27 07:26 UTC - GitHub Copilot - Refreshed the draft against `develop` at the merge of PR #2664: added the listener drain budget and the chained budget model, corrected the health-check drain (5 s, not 90 s), found the second hard-coded 10 s in the startup-failure path, replaced `verification.md` with the evidence template, and proposed the schema and open decisions D1 to D9 for maintainer review. Every source claim is "read from source, not run".
- 2026-10-27 07:38 UTC - GitHub Copilot - The maintainer did not approve the top-level `[shutdown]` section: too specific, and he asked for a generic application-level section with sub-sections. Read the architecture documents, ADRs, the configuration EPIC, and the source for other application-level values; added "Configuration Placement" (documents, candidate inventory, options P1 to P3), changed the schema to `[application.shutdown]`, added D10, and extended the documentation scope (architecture guide, glossary, `drain.rs` docs).
- 2026-10-27 07:44 UTC - GitHub Copilot - The maintainer approved the specification with D1 = B, D2 = P2, and the name `application`, and asked for the list of candidate names with pros and cons (added to "Configuration Placement", including `main`). The GitHub issue is created next.
- 2026-10-27 07:46 UTC - GitHub Copilot - Created GitHub issue #2668 (label `task`), linked it as a sub-issue of #1488 (checked: `parent_issue_url` is #1488), moved the spec from `docs/issues/drafts/1488-si-20-configure-shutdown-policy/` to this folder on branch `2668-configure-shutdown-policy-spec`, removed the skeleton `verification.md`, and updated the links to the old draft path (EPIC roadmap row 20, SI-8, and three closed specs).
- 2026-10-27 08:35 UTC - GitHub Copilot - Opened the spec-only PR #2669 (`Related to #2668`, no closing keyword); its Docs Lint check passed. The maintainer asked for `related-pr` to record it, so this entry and the frontmatter field are the second commit of the PR.
- 2026-10-27 09:02 UTC - GitHub Copilot - `review-finding:pr-2669-f1` (Copilot review, finding R2): reproduced the false-success exit code on the real binary before review and recorded it as V1 in `manual-verification-evidence.md` (Reproduced with a temporary fault-injection patch; four network attempts on the unmodified binary did not fail a component). Replaced the "Not applicable" bug sections with the workflow link, regression boundary and recheck plan, split T3 into T3a (red) and T3b (fix), marked T1 done, rewrote M1 and M5, and recorded the disposable script. The type stays `task`; whether to relabel it `bug` is a maintainer decision.
- 2026-10-27 09:04 UTC - GitHub Copilot - `review-finding:pr-2669-f2` (Copilot review, finding R3): replaced the nonexistent `integration` test target in the Automatic Checks with the registered `lifecycle-signals` target, and made T6 register its new files as `[[test]]` targets. Rechecked every other command in the specification: `cargo test -p torrust-tracker-configuration --no-run` builds and `contrib/dev-tools/git/hooks/pre-push.sh` exists.
- 2026-10-27 09:06 UTC - GitHub Copilot - `review-finding:pr-2669-f3` (Copilot review, finding R1): M3 left the UDP drain at its 5 s default, so `max(2, 5) + 1 = 6 >= 5` and startup would reject it; M3 now sets `udp_request_drain_in_secs = 1` (`max(2, 1) + 1 = 3 < 5`). Rechecked every worked example against `max(H, U) + L < P`: M2 (defaults, 20 < 25), M4 (now six explicit configurations with the arithmetic, one accepted boundary), M6 (HTTP drain 3, `max(3, 5) + 5 = 10 < 25`), M7 (defaults, 20 < 25), T6 (the M3 budgets), and the `DEADLINE_IN_SECS=30` override example (20 < 30) all hold. Found while doing it, from the V1 attempts: a held request ends within about 5 s on its own, so M6 and M7 could not show what they claimed with the budgets they had; M6 now uses a 3 s drain, and M7 uses the temporary fault-injection patch.
- 2026-10-27 11:27 UTC - GitHub Copilot - PR #2669 (the spec-only PR) was merged by the maintainer at 10:57 UTC; implementation started on branch `2668-configure-shutdown-policy` from `torrust/develop`. T1 needs no rerun: `src/`, `packages/`, `tests/`, and the manifests are unchanged on `develop` since the V1 base. T2 done in `packages/configuration`: `[application.shutdown]` with `deadline_in_secs` 25, `http_drain_in_secs` 15, `udp_request_drain_in_secs` 5, `event_listener_drain_in_secs` 5; one shared `ShutdownBudgetInSecs` newtype (1 to 3600 s) instead of newtypes built on `AtLeastU64`, because `AtLeastU64` has no upper bound and one invariant serves all four keys; the chain rule `max(H, U) + L < P` as a `SemanticValidationError` variant reached through `Configuration::validate`; the four tests that pin the default TOML were updated. 25 new tests at T2 (129 to 154 in the package; 27 in the final tree, because T4 added the `as_duration` test and the D5 follow-up the accessor test), each mutation-proven; two surviving mutations found gaps and were closed with new tests (evidence section "T2"). Nothing is wired into the supervisor or the servers yet. Stopping here for the design review of D5 (how `P`, `H`, `U`, and `L` reach their consumers) with the maintainer.
- 2026-10-27 12:28 UTC - GitHub Copilot - The maintainer decided D5: inject the values top to bottom instead of keeping separate constants in the server crates, change signatures where needed because configuration 3.0.0 is not published, pass a plain `Duration` where the callee needs only a value, and use a public default constant only where injection would add an unwanted dependency. Then implement, and re-evaluate cohesion and coupling at the end. The recorded spec text ("Where each value goes", T5, D5) was updated to match. The design proposed earlier (`AppContainer` holds the settings, additive builders with kept constants) is superseded by this decision.
- 2026-10-27 12:31 UTC - GitHub Copilot - T3a done: `shutdown_exit_code` added in `src/bootstrap/jobs/manager.rs` returning the current behavior (always exit `0`), with five unit tests. Recorded the red run in `manual-verification-evidence.md` (section "T3a"): the `Failed`, `Panicked`, and `Aborted` tests fail with `left: 0, right: 1`; the two graceful tests pass. T3b follows in the next commit.
- 2026-10-27 12:37 UTC - GitHub Copilot - T3b done: `shutdown_exit_code` maps the outcomes (`0` only when every component is `Completed` or `Cancelled`, otherwise `1`), `incomplete_components` names the components, and `main` returns `ExitCode`: the success message appears only on exit `0`, an incomplete shutdown logs an error naming the components, and a startup failure returns `ExitCode::FAILURE` instead of calling `process::exit`. The three red tests are green (39 manager tests), six mutations were proven one at a time, and the workspace suite passes (3,540 tests). The real-binary recheck of V1 is T8's; `report_shutdown` in `main` has no unit test and is covered by T6 and T8. D9 stays open until T8 checks that returning from `main` does not hang on a blocking task.
- 2026-10-27 12:48 UTC - GitHub Copilot - T4 done, following D5 (inject top to bottom): `ShutdownBudgetInSecs::as_duration`; `AppContainer.application_config`, from which `main` reads the deadline, and `start_jobs` reads it from `config` for the cleanup after a failed startup, so no hard-coded 10 s remains in `src/main.rs` or `src/app.rs`; `event_listener_drain_timeout(config)` called by the five bootstrap files for the seven listeners (the banning listener gained a `config` parameter). Four new tests, all mutation-proven (evidence section "T4"). One mutation survived (T2b: a site discarded the helper's result); the source-level guard test was strengthened to count the helper call per listener and re-proven. The two deadline call sites have no unit test (no observable seam); T6 and T8 assert the `timeout_seconds` log line of the real binary. The workspace suite passes (3,544 tests).
- 2026-10-27 13:07 UTC - GitHub Copilot - T5 done, following D5: the HTTP tracker, REST API, health-check and UDP server crates take their drain from the caller (`Launcher.drain_timeout` for HTTP and REST, a `drain_timeout` argument for the health check, a `request_drain_deadline` argument for UDP), so the three 90 s and 5 s constants are gone and `src/app.rs` passes `http_drain_in_secs` and `udp_request_drain_in_secs`. Five signed commits: the drain helper now shortens a timeout above one year (it panicked on overflow), then one commit per crate. Four new tests, each mutation-proven (evidence section "T5"); the UDP shutdown scenarios now start through the public token-aware start. The health-check drain default rises from 5 s to 15 s because it follows `http_drain_in_secs`. No version bump (packages at 0.1.0, as in the earlier steps). The workspace suite passes (3,548 tests). The real-binary check of H and U is T6 and T8.
- 2026-10-27 14:47 UTC - GitHub Copilot - T6 done: the `lifecycle-shutdown-policy` target (9 tests) starts the real binary with configured budgets and signals its exact PID: exit `0` and the default 25 s deadline on a clean stop (SIGTERM, SIGINT), the configured deadline in the supervisor log, exit `1` when a held connection outlasts a 2 s HTTP drain (SIGTERM, SIGINT), exit `1` and the configured deadline when a service fails to start, rejection of a zero deadline and of a chain-rule violation before any service starts, and acceptance of the largest valid drain. Five mutations at the executable boundary, each caught (evidence section "T6"). The held-connection scenario uses a 10 s deadline instead of M3's 5 s, so the test tells a configured drain from the deadline. The shared fixture's `shutdown()` ignored the exit status; it now requires `0`. A review of my own tests found unbounded acceptance probes, bounded in a follow-up commit. The disposable script stays until T8 has used it (M5, M6, M7); T9 removes it. The workspace suite passes (3,575 tests).
- 2026-10-27 15:03 UTC - GitHub Copilot - T7 done: the container and development templates state the default `[application.shutdown]` block and a test pins them to the defaults (mutation-proven twice); `docs/containers.md` gained "Stopping the Tracker" with the deployment contract; the three E2E Compose files set `stop_grace_period: 35s` (validated with `docker compose config`, the stacks run in CI); the architecture documents, glossary, feature README, `drain.rs` documentation and the v2 to v3 migration guide were updated; Q4 is amended and Q9 points to the evidence file (its link names the open path, which the archive step repairs); two root ADRs were added and registered (placement rule, budget model). While writing the evidence I found that the T5 documentation commit had deleted the heading `## Failures and Follow-up`; it is restored here. The workspace suite passes (3,576 tests). Next: re-evaluate the design for cohesion and coupling (D5), then T8.
- 2026-10-27 20:15 UTC - GitHub Copilot - D5 follow-up done (cohesion and coupling review, evidence section "D5 follow-up"): the server crates receive plain durations and no consumer needs the configuration type; one finding led to a change, `Shutdown::deadline`, `http_drain`, `udp_request_drain` and `event_listener_drain` replace the key-and-conversion spelled at 31 call sites (one mutation-proven test; two executable-boundary mutations repeated on the new path). Four findings were left as they are, with reasons. The workspace suite passes (3,577 tests). Next: T8, the manual verification V2 to V8 and the V1 recheck, which must run after the last production change.
- 2026-10-27 20:40 UTC - GitHub Copilot - T8 done: M2 to M8 ran on the finished change (last production commit `refactor(configuration): read each shutdown budget through a named accessor`) and are recorded as V2 to V8. Defaults: exit `0`, `timeout_seconds=25`, three drains of 15 s. Budgets 5/2/1/1 with a held connection: exit `1` in 2.0 s on SIGTERM and SIGINT, HTTP component named, no success line (the stop took the HTTP drain alone, not the 3 s the chain bound allows). Five invalid configurations rejected before any listener bound, each naming the key; the boundary `19 + 5` accepted. V1 recheck on the fixed tree: `slow-stop` and `fail-stop` with SIGTERM and SIGINT all exit `1` and name the component, where V1 recorded `0`; the temporary patch needed `std::time::Duration` because the hard-coded 10 s is gone, was reverted and proven identical to `HEAD`. Docker release image: `--stop-timeout 35` and a 3 s override give exit `1` in 3.2 s with a held connection and `0` clean; the patched image gives `137` at 10.1 s with the default and `1` at 25.1 s with 35 s. systemd: clean `Result=success`, held connection `status=1/FAILURE`. Podman not run. AC7 source search found `process::exit` only in separate CLI binaries. Next: T9 (independent review, script removal, acceptance review, retrospective decision).
- 2026-10-27 20:52 UTC - GitHub Copilot - T9 started: the disposable script `evidence/hold-and-stop.sh` and `evidence/baseline.toml` were removed after V2 to V8 had used them (the Rust target `lifecycle-shutdown-policy` covers the held-connection exit codes; the fault-injection, Docker and systemd runs are one-off evidence). Next: the independent `review-task` review.
- 2026-10-27 21:05 UTC - GitHub Copilot - The independent `review-task` review returned `REVIEW FAILED` for documentation and evidence findings only (recorded in `agent-review-reports.md`). Addressed: the retrospective was written (the reviewer judged one required) and the completion-review lines updated; V9, a real-binary run with a UDP tracker and distinct budgets 20/6/3/2, was added to the evidence (the UDP deadline `request_drain_deadline=3s` and the HTTP-family `drain_timeout=6s` appear in the logs; the listener drain is not logged and has no executable-boundary check); the stale evidence text, the `src/AGENTS.md` statement, the README stamp, the M3 expectation (2.0 s), the test counts (27 in the configuration package) and the comment lost in `src/container.rs` were fixed; the Rollback text now says that a revert restores the old drain defaults. Left for the maintainer: AC5 is not guarded by a maintained test for `U` (swapping the keys in `src/app.rs` passes every test) and `L` (source-level guard only), the wrapper-style native tests (the Act sits in a helper), and `JobManager::wait_for_all` adding to the clock unchecked (a follow-up candidate, recorded in the retrospective). No acceptance criterion is ticked. A re-review entry follows.
- 2026-10-27 21:09 UTC - GitHub Copilot - T9 done: the independent re-review returned `REVIEW PASSED` (second entry of `agent-review-reports.md`); B1, M1, m1 to m5, m8 and AC11 are resolved, and m6, m7, m9 and the AC5 test gap for `U` and `L` are recorded for the maintainer. All implementation tasks T1 to T9 are done and no acceptance criterion is ticked. Final state before the pull request: 29 signed commits ahead of `torrust/develop` (counting the one that carries this entry), `linter all` exit 0, the workspace suite 3,577 passed at the last code change. Next: the pre-push checks, then ask the maintainer before pushing and before opening the pull request.
- 2026-10-28 09:56 UTC - GitHub Copilot - The implementation PR #2678 was opened on 2026-10-27 and its CI finished green (25 passed, 3 skipped: two publish jobs and Docker E2E). Afterwards another pull request merged into `develop` and conflicted with the shared native-tracker fixture (`tests/common/native_tracker/command.rs`, the `TORRUST_TRACKER_BINARY` override). The branch was rebased onto the new `develop` (a required rebase, because of the conflict): the only conflict was the import line of that file's test module, resolved by keeping both sets of tests and imports; all 29 commits were re-signed; a local backup branch holds the pre-rebase history. This entry cites commit subjects only, so the rebase invalidates no reference in repository files. On the rebased tree `linter all` and the formatter check exit 0, the workspace suite passes (3,652 tests, which includes the tests that `develop` gained), and the doc tests pass. The manual evidence (V1 to V9) ran before the rebase on code that the rebase did not change apart from the fixture merge; it is not repeated. `related-pr` is set to 2678.
- 2026-10-28 11:10 UTC - GitHub Copilot - Review of PR #2678 (Copilot, one inline finding, `review-finding:pr-2678-f1`, audit in `docs/pr-reviews/pr-2678-review/PR-REVIEW.md`): only the HTTP-family drains end as a failed component; the UDP request drain and the listener drain only log a warning and still exit `0`. The budget ADR and `docs/containers.md` were corrected in the commit `docs(shutdown): qualify which drain timeouts end a stop with exit code 1`, and the exit-result section of this specification (the same overstatement) here. CI on the head before the fix: all green except one job still running at the time of the review.
- 2026-10-28 12:12 UTC - GitHub Copilot - Two maintainer decisions on PR #2678. (1) The stamp of `docs/features/shutdown-process/README.md` was behind the last commit that touched it; it is bumped, and every changed file's `last-updated-utc` was re-checked against the author time of its last commit. (2) The listeners read the listener drain budget from `app_container.application_config.shutdown.event_listener_drain()` like the other budgets (single source, D5): the helper `event_listener_drain_timeout(config)` is removed, the HTTP core, UDP core and UDP server statistics starters and their `src/app.rs` wrappers drop the `config` parameter, the UDP banning starter has its `develop` signature again, and the torrent repository and tracker core starters keep `config` for core settings but take the drain from the container too. The job starters take a `Duration` and read no configuration; their tests that build an `AppContainer` use it. Evidence section "Listener drain read from the container"; the workspace suite passes (3,651 tests, one helper test removed), with two mutation proofs of the source-level guard.

## Open Decisions for the Maintainer

Status on 2026-10-27: the maintainer chose D1 = B, D2 = P2 (`[application.shutdown]`), and the name `application` (D10), and approved the specification with one requested change (the candidate-name list with pros and cons, added above). He raised no objection to the recommendations for D3 to D9; D5 stays open for the T2 design review.

| ID  | Decision                                          | Recommendation                                                                                                                                                                                            |
| --- | ------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| D1  | Defaults that satisfy the chain rule              | **B**: keep P = 25, U = 5, L = 5 and the 30 s orchestrator minimum; lower H to 15 s. Rejected: **A** keep 20 s and validate only `H < P` (a full HTTP drain starves the listeners); **C** raise P to 30 s (orchestrator minimum becomes 35 s, no longer fits Kubernetes' 30 s default); **D** shrink L (contradicts SI-22's recorded 5 s choice). |
| D2  | Where the shutdown settings live                  | **P2**: `[application.shutdown]`, all four values together, so the chain rule is checkable in one place; the event buses are created by the composition root, so the listener budget is application-level too. Rejected: P1 top-level `[shutdown]` (too specific); P3 per-service budgets (scatters the contract, no demonstrated need). See "Configuration Placement". |
| D3  | Health-check drain                                | Share `http_drain_in_secs` as Q4 says. This raises its current 5 s to 15 s; the drain only waits while a connection stays open, so a normal stop is unchanged.                                          |
| D4  | Key names and bounds                              | `_in_secs` keys, each 1 s to 3600 s, as value invariants. 3600 s is a conservative ceiling that also avoids the unchecked `Instant` addition; revisit if an operator needs more.                          |
| D5  | How `P`, `H`, `U`, and `L` reach their consumers  | **Decided by the maintainer on 2026-10-27: inject top to bottom**, changing signatures where needed, passing a `Duration` where the callee needs only a value, and adding a public default constant only where injection would add an unwanted dependency. Rejected: a `JobManager` that owns the deadline, additive builders with separate crate defaults, and a new struct returned by `app::start`. Re-evaluate cohesion and coupling after T7 (done 2026-10-27: five findings, one change, the named accessors; see the evidence section "D5 follow-up"). |
| D6  | Scope of "no component calls `process::exit`"     | Verify by source search recorded in the evidence. A `disallowed-methods` Clippy rule is a possible later guard but needs allows in the binaries and console crates, so not in this task.                  |
| D7  | Record the decisions                              | Amend Q4 (Actions Taken addendum) and add two short root ADRs: the `[application]` placement rule (lasting convention for later contributors) and the budget model (lasting validation contract).         |
| D8  | Schema version                                    | Keep `3.0.0`; the new section is optional and `develop` is `3.0.0-develop`. Per-package version bumps follow the independent-versioning ADR at implementation time.                                       |
| D9  | `main` returning `ExitCode`                       | Use `ExitCode` instead of `std::process::exit`, so the runtime drops normally; T8 checks that returning from `main` does not hang on a blocking task. Fall back to a logged `exit` after the check if it does. |
| D10 | Root section name, and what moves in later        | `[application]`, the word the architecture documents use (alternatives and why `[orchestrator]` is rejected are in "Configuration Placement"). Create no `startup` table and move nothing now: `[logging]` and the `[core]` job intervals stay, and a later EPIC #1978 issue may consolidate them. |

## Acceptance Criteria

- [ ] AC1: `[application.shutdown]` provides the documented defaults, each section and key optional, with the rationale for every value recorded next to it.
- [ ] AC2: A zero value or a value above the maximum is rejected at load time with an error that names the key.
- [ ] AC3: A configuration with `max(H, U) + L >= P` is rejected at startup before any service starts, with an actionable error naming the keys and values.
- [ ] AC4: The configured `P` reaches `wait_for_all` in `main` and in the startup-failure cleanup; no production site keeps a hard-coded 10 s.
- [ ] AC5: The configured `H` reaches the HTTP tracker, REST API, and health-check drains; `U` reaches the UDP request drain; `L` reaches all seven listeners; each is covered by a test that fails with the default.
- [ ] AC6: `main` exits `0` only when every outcome is `Completed` or `Cancelled`, and `1` for any `Failed`, `Panicked`, or `Aborted` outcome or a startup failure; the success log appears only with exit `0`.
- [ ] AC7: No component task calls `std::process::exit`, shown by a recorded source search.
- [ ] AC8: With the real binary and its exact PID, SIGTERM and SIGINT give exit `0` on a clean stop and exit `1` when a held-open connection outlasts the configured drain; an invalid configuration exits `1` before services start.
- [ ] AC9: The container, Compose, Kubernetes, and systemd settings are documented with the 30 s minimum and 35 s recommendation, and the 10 s Docker/Podman default is shown to be insufficient (exit 137) and documented as such.
- [ ] AC10: Default fixtures carry the `[application.shutdown]` block; Q4, the feature README, `application-jobs.md`, the architecture guide, the glossary, and the `drain.rs` documentation match the implemented values; the placement and budget-model decisions are recorded as agreed in D7.
- [ ] AC11: The rollback path in this specification is accurate for the merged change.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test -p torrust-tracker-configuration` on the stable Rust toolchain
- `cargo test --doc --workspace` and `cargo test --tests --benches --examples --workspace --all-targets --all-features` on the stable Rust toolchain
- `cargo test --test lifecycle-signals` on the stable Rust toolchain: the existing native-binary lifecycle target (`tests/lifecycle/signals.rs`, registered in the root `Cargo.toml`). T6 adds its files next to it and registers each as its own `[[test]]` target, because Cargo does not discover files in subdirectories of `tests/`; each new target is then run with `cargo test --test <target-name>`.
- `./contrib/dev-tools/git/hooks/pre-push.sh` before pushing
- Performance: not applicable. The values are read once at startup and used only on the shutdown path; no request-path code changes, so no benchmark is required. The spec records this and the implementation review confirms it.

### Manual Verification Scenarios

Every scenario starts the real binary (or image), signals the actual tracker PID (not a `cargo run`
parent), captures bounded exit status and shutdown output, and repeats with SIGINT where noted.
Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario                                                       | Human-oriented command/steps                                                                                                                                                  | Expected Result                                                                                                  | Status | Evidence                                      |
| -- | -------------------------------------------------------------- | ----------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------- | ------ | --------------------------------------------- |
| M1 | Pre-review reproduction of exit `0` after a failed or aborted shutdown | Unmodified binary: SIGTERM with held connections (A1 to A4). Then the temporary fault-injection patch with `SI20_REPRO_FAULT=slow-stop` and `fail-stop`, SIGTERM and SIGINT (B1 to B4), using `evidence/hold-and-stop.sh`. | Unmodified: exit `0`, no component failed. Patched: exit `0` and the success log although a component was aborted or failed (the defect). | DONE   | `manual-verification-evidence.md` section V1  |
| M2 | Defaults, clean stop                                           | Start with no `[application]` section (defaults 25 / 15 / 5 / 5: `max(15, 5) + 5 = 20 < 25`), `kill -TERM <pid>` and `kill -INT <pid>`.                                      | Exit `0`, success log, all outcomes named and completed, listener ports released.                              | DONE   | section V2                                    |
| M3 | Configured budgets, drain budget reached                       | Config under `[application.shutdown]` with `deadline_in_secs = 5`, `http_drain_in_secs = 2`, `udp_request_drain_in_secs = 1`, `event_listener_drain_in_secs = 1` (`max(2, 1) + 1 = 3 < 5`, so startup accepts it). Hold a `partial-header` connection on the HTTP tracker with `evidence/hold-and-stop.sh` (the 5 s header timeout would close it only after the 2 s budget); SIGTERM, then SIGINT. | Exit `1`, the HTTP component named as failed because its drain was reached, no success log, exit after about 3 s. | DONE   | section V3 (2.0 s: the listeners had nothing queued, so the stop took the HTTP drain alone)                                   |
| M4 | Invalid configuration                                          | Start six configurations, each otherwise default: `deadline_in_secs = 0`; `http_drain_in_secs = 3601`; an unknown key `drain_in_secs = 5`; `http_drain_in_secs = 20` (`20 + 5 = 25 >= 25`); `udp_request_drain_in_secs = 21` (`max(15, 21) + 5 = 26 >= 25`); and, as the accepted boundary, `http_drain_in_secs = 19` (`19 + 5 = 24 < 25`). | The first five exit `1` before any listener binds, naming the key, the values, and what to change; the sixth starts and stops cleanly. | DONE   | section V4                                    |
| M5 | Failed and aborted outcomes at the executable boundary         | The like-for-like recheck of M1 on the fixed tree: re-apply the same temporary patch (not committed), run `slow-stop` and `fail-stop` with SIGTERM and SIGINT, then revert and prove the revert with `cmp`. | Exit `1`, the abort or failure logged by name, no "successfully shutdown" line; the unpatched control stays at exit `0`. | DONE   | section V5                                    |
| M6 | Docker with a sufficient grace period                          | Build the image. `docker run --stop-timeout 35` with `TORRUST_TRACKER_CONFIG_OVERRIDE_APPLICATION__SHUTDOWN__HTTP_DRAIN_IN_SECS=3` (other values default: `max(3, 5) + 5 = 10 < 25`), a `partial-header` connection held on the published HTTP port, then `docker stop`; inspect the exit code and logs. Repeat with no held connection. | The override reaches the tracker (logged configuration); exit `1` with the held connection (the 3 s drain budget is reached before the 5 s header timeout) and `0` for a clean stop; SIGTERM reaches the tracker as PID 1. | DONE   | section V6                                    |
| M7 | Docker with the default 10 s                                   | A network client cannot make the default tracker outlast 10 s (V1, A1 to A4: the drain ends within about 5 s), so use the temporary fault-injection patch of V1 (`SI20_REPRO_FAULT=slow-stop`, passed with `docker run -e`) in an image built from a patched copy of the tree, never committed. Run `docker stop` with the default time, then with `--time 35` (defaults: `20 < 25`). | Default time: exit `137` (SIGKILL) at about 10 s, demonstrating the documented insufficiency. With `--time 35`: the supervisor aborts the component at the 25 s deadline and the exit code is `1`. If the patched image cannot be built, record "not run" with the reason. | DONE   | section V7                                    |
| M8 | systemd and Podman                                             | `systemd-run --user` with `TimeoutStopSec=35` and `systemctl --user stop`; Podman if installed (it is not installed on the author's machine, so record "not run").            | Unit result matches the exit code; Podman recorded honestly as not run, or run.                                  | DONE   | section V8 (Podman not run)                   |

Notes:

- Manual verification is mandatory even when automated tests pass.
- Record each command result with the toolchain or runtime that produced it, for example the
  stable Rust toolchain or Docker 28.3.3.
- Create `manual-verification-evidence.md` from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`;
  keep larger logs under an `evidence/` subfolder.
- Run the evidence after the last production change, including changes made for review
  findings, and name the commit subject it ran on. Do not run mutation experiments or benchmarks
  in the same worktree while another build or test is running.

### Disposable Verification Scripts

One script exists, created for V1 and reused by M3, M5, and M6 where it fits:

- **Path**: `evidence/hold-and-stop.sh`, with its configuration `evidence/baseline.toml`.
- **What it verifies**: it starts the real tracker binary, holds one connection open (idle, partial
  header, or an unfinished request body), sends SIGTERM or SIGINT to the binary PID, and prints the
  exit status and the seconds to the exit. It makes no assertion; a person reads the output.
- **Why a script and not a maintained Rust test**: the evidence needs a real binary, real signals,
  and a person's reading of the logs for one-off scenarios, several of which need a temporary
  patch that must never be committed. The durable behavior (clean stop and drain budget reached)
  is promoted to Rust tests in `tests/lifecycle/` by T6.
- **Language**: bash, not Python, so the Python rule does not apply.
- **Removal owner**: the author of T6 removes the script and its configuration when the Rust tests
  cover the same scenarios, and records the removal in the progress log; T9 checks it. T6 did not
  remove them: the Rust tests cover the held-connection exit codes, but M5 (fault injection with a
  patch that is never committed) and M6 and M7 (Docker) still use the script, so the T9 author
  removes it after T8 and records the removal. **Done in T9**: the script and `baseline.toml` were
  removed after V2 to V8 had used them (progress log, 2026-10-27).

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | TODO                   |          |
| AC2   | TODO                   |          |
| AC3   | TODO                   |          |
| AC4   | TODO                   |          |
| AC5   | TODO                   |          |
| AC6   | TODO                   |          |
| AC7   | TODO                   |          |
| AC8   | TODO                   |          |
| AC9   | TODO                   |          |
| AC10  | TODO                   |          |
| AC11  | TODO                   |          |

## Rollback

Revert the implementation pull request: it removes the `[application]` section, the exit mapping,
and the documentation together, and restores the 10 s deadline and the component constants (90 s for
the HTTP tracker and the REST API, 5 s for the health check and the UDP request drain). Reverting
therefore also restores the old drain defaults, so an operator who relied on the new 15 s HTTP drain
or on the 15 s health-check drain sees them change back. This
is safe until an operator relies on the published configuration. After a release that ships the
section, do not remove it: keep the keys and their meaning in a compatibility patch rather than
silently changing them. The exit-code change is observable by process managers, so reverting it
after release also changes how systemd and Docker see a stop; state that in the release notes.

## Risks and Trade-offs

- **Default change**: lowering the HTTP drain from 90 s to 15 s (and the health-check drain from 5 s
  to 15 s) is a behavior change for operators with very slow clients. Announce it in the release
  notes; the keys let them raise it, and the validation tells them what else to raise.
- **Exit code `1` after a requested stop**: systemd marks the unit `failed` and `restart: on-failure`
  policies may restart. This is the Q3 decision; document it, do not hide it.
- **Returning from `main` waits for the runtime to drop**: a blocking task could delay the exit
  past the deadline. T8 checks this (D9) and the fallback is a logged `exit` after the outcomes.
- **Chain rule is config-level**: it does not know which services are enabled, so it can be
  stricter than needed for a deployment without HTTP. Simplicity and a fixed contract win over
  case analysis.
- **Listener budget is admission only**: a slow persistent listener can still be cut at `P`.
  That is the SI-22 trade-off; #2418 is the remedy.
- **No Podman here**: the Podman evidence may be recorded as not run.

## Implementation Completion Review

- Retrospective: `implementation-retrospective.md` (created 2026-10-27 after the independent
  reviewer judged one required: the D5 design change, the fixture that ignored the exit status, the
  second `Duration` overflow, and the open AC5 gap for the UDP and listener budgets).
- The independent `review-task` reviewer records its result in `agent-review-reports.md`; the first
  entry is `REVIEW FAILED` for documentation findings, all addressed in later commits, and a re-review
  entry follows.

## References

- Parent EPIC: #1488
- Related issues: #1586 (supervisor outcomes), #2410 (SI-22, listener drain), #2418 (batched persisted writes), #2132 (SIGTERM at main)
- Related ADRs: `docs/adrs/20261021184946_order_shutdown_by_event_flow_and_drain_event_listeners.md`, `docs/adrs/20260723184019_separate_configuration_value_invariants_from_consistency_validation.md`
- Decisions: [Q3 and Q4](../../../features/shutdown-process/questions.md#q3) (outcome and deadlines), [Q9](../../../features/shutdown-process/questions.md#q9) (container evidence)
