---
schema-version: 1
doc-type: issue
issue-type: task
status: planned
priority: p2
epic: 1488
github-issue: 2668
spec-path: docs/issues/open/2668-1488-si-20-configure-shutdown-policy/ISSUE.md
branch: "2668-configure-shutdown-policy-spec"
related-pr: 2669
last-updated-utc: "2026-10-27 09:04"
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

The tracker cannot validate the orchestrator's deadline from inside its process. The deployment
contract below states the external rule instead.

### Where each value goes

| Value | Consumer                                                                                                          |
| ----- | ----------------------------------------------------------------------------------------------------------------- |
| `P`   | `JobManager::wait_for_all` in `main` and in the startup-failure cleanup of `src/app.rs`.                          |
| `H`   | `graceful_shutdown_on_cancellation` of the HTTP tracker, REST API, and health-check API servers.                  |
| `U`   | The UDP receive loop's accepted-request drain (`start_receive_loop`).                                             |
| `L`   | The seven listener constructors, all given the same value (their `drain_timeout` parameter already exists).       |

How `P` reaches `main` and how `H` and `U` reach the server crates is decided at the design
review checkpoint (D5); the contract above does not depend on it. Package public APIs change
additively where possible: the current constants stay as named defaults so the test
environments and examples keep their behavior.

## Exit-Result Contract

`JobManager::wait_for_all` already returns named outcomes. A pure function in the root crate
maps them, and `main` returns the result as a `std::process::ExitCode` (D9):

| Outcome set                                                                  | Exit code |
| ---------------------------------------------------------------------------- | --------- |
| Every outcome is `Completed` or `Cancelled`                                  | `0`       |
| Any outcome is `Failed` (including a drain budget reached), `Panicked`, or `Aborted` (deadline abort) | `1`       |
| Startup cannot complete                                                      | `1`       |
| SIGKILL or another signal the process cannot handle                          | OS-defined (`128 + n` in a shell) |

Components keep returning outcomes to their owner and never call `std::process::exit`.
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
| T2  | TODO   | `[application.shutdown]` configuration section | `application.rs` (and `shutdown.rs`) with newtypes, defaults, `Validator` rule, error variant, module docs, unit tests (defaults, TOML and environment overrides, zero, over-max, chain violation, boundary, unknown key). **Then stop for the design review (D5).** |
| T3a | TODO   | Regression test, red                  | Pure outcome-to-exit-code function returning the current behavior (always success) and a parameterized test per `JobStatus`; the recorded red run fails for `Failed`, `Panicked`, and `Aborted`.               |
| T3b | TODO   | Fix: outcome-to-exit mapping          | Implement the mapping, `main` returns `ExitCode`, success log only on `0`, failing outcomes logged by name; record the green run.                                                                              |
| T4  | TODO   | Deliver `P` and `L`                   | Configured `P` in `main` and in the startup-failure cleanup; `L` into the seven listener constructors; tests that a configured value, not the default, reaches each consumer.                                  |
| T5  | TODO   | Deliver `H` and `U`                   | Additive options on the HTTP, REST API, health-check, and UDP server crates; defaults retained as named constants; paused-time tests per crate.                                                                |
| T6  | TODO   | Native-binary tests                   | In `tests/lifecycle/`, each file registered as a `[[test]]` target in `Cargo.toml` like `lifecycle-signals`: exit `0` on a clean stop, exit `1` with a held-open connection under small configured budgets, invalid configuration rejected before any service starts; SIGTERM and SIGINT. |
| T7  | TODO   | Fixtures and documentation            | `share/default/config/` fixtures, `docs/containers.md`, feature README, `application-jobs.md`, `tracker-instance-architecture.md`, glossary, `drain.rs` docs, configuration module docs and migration note, Q4 amendment, Q9 pointer, and the ADRs D7 approves. |
| T8  | TODO   | Manual verification evidence          | `manual-verification-evidence.md`: V2 to V8 below, plus the like-for-like recheck of V1 (B1 to B4), run after the last production change, naming the commit subject they ran on.                              |
| T9  | TODO   | Independent review and completion     | `review-task` review recorded in `agent-review-reports.md`; acceptance criteria re-reviewed; retrospective or a progress-log line saying why none.                                                           |

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
- [ ] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md`
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

## Open Decisions for the Maintainer

Status on 2026-10-27: the maintainer chose D1 = B, D2 = P2 (`[application.shutdown]`), and the name `application` (D10), and approved the specification with one requested change (the candidate-name list with pros and cons, added above). He raised no objection to the recommendations for D3 to D9; D5 stays open for the T2 design review.

| ID  | Decision                                          | Recommendation                                                                                                                                                                                            |
| --- | ------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| D1  | Defaults that satisfy the chain rule              | **B**: keep P = 25, U = 5, L = 5 and the 30 s orchestrator minimum; lower H to 15 s. Rejected: **A** keep 20 s and validate only `H < P` (a full HTTP drain starves the listeners); **C** raise P to 30 s (orchestrator minimum becomes 35 s, no longer fits Kubernetes' 30 s default); **D** shrink L (contradicts SI-22's recorded 5 s choice). |
| D2  | Where the shutdown settings live                  | **P2**: `[application.shutdown]`, all four values together, so the chain rule is checkable in one place; the event buses are created by the composition root, so the listener budget is application-level too. Rejected: P1 top-level `[shutdown]` (too specific); P3 per-service budgets (scatters the contract, no demonstrated need). See "Configuration Placement". |
| D3  | Health-check drain                                | Share `http_drain_in_secs` as Q4 says. This raises its current 5 s to 15 s; the drain only waits while a connection stays open, so a normal stop is unchanged.                                          |
| D4  | Key names and bounds                              | `_in_secs` keys, each 1 s to 3600 s, as value invariants. 3600 s is a conservative ceiling that also avoids the unchecked `Instant` addition; revisit if an operator needs more.                          |
| D5  | How `P`, `H`, and `U` reach their consumers       | Decide at the T2 design review, with the schema in hand: either `JobManager` carries the deadline, or `app::start` returns it. Server crates take a `Duration` through an additive option.                |
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
| M2 | Defaults, clean stop                                           | Start with no `[application]` section, `kill -TERM <pid>` and `kill -INT <pid>`.                                                                                              | Exit `0`, success log, all outcomes named and completed, listener ports released.                              | TODO   | section V2                                    |
| M3 | Configured budgets, drain budget reached                       | Config with `deadline_in_secs = 5`, `http_drain_in_secs = 2`, `event_listener_drain_in_secs = 1` under `[application.shutdown]`; hold a request open; SIGTERM, then SIGINT.  | Exit `1`, the HTTP component named as failed, no success log, total time about the drain budget plus listener stage. | TODO   | section V3                                    |
| M4 | Invalid configuration                                          | Start with a zero value, an over-max value, an unknown key, and `http_drain_in_secs + event_listener_drain_in_secs >= deadline_in_secs`.                                      | Each exits `1` before any listener binds, naming the keys and what to change.                                   | TODO   | section V4                                    |
| M5 | Failed and aborted outcomes at the executable boundary         | The like-for-like recheck of M1 on the fixed tree: re-apply the same temporary patch (not committed), run `slow-stop` and `fail-stop` with SIGTERM and SIGINT, then revert and prove the revert with `cmp`. | Exit `1`, the abort or failure logged by name, no "successfully shutdown" line; the unpatched control stays at exit `0`. | TODO   | section V5                                    |
| M6 | Docker with a sufficient grace period                          | Build the image, `docker run --stop-timeout 35` with the drain budget set through `TORRUST_TRACKER_CONFIG_OVERRIDE_APPLICATION__SHUTDOWN__HTTP_DRAIN_IN_SECS`, hold a request open, `docker stop`; inspect the exit code and logs. | The override reaches the tracker (logged configuration); exit `1` for the held request (drain budget reached) and `0` for a clean stop; SIGTERM reaches the tracker as PID 1. | TODO   | section V6                                    |
| M7 | Docker with the default 10 s                                   | Same image, held request, `docker stop` with the default time.                                                                                                                | Exit `137` (SIGKILL) while the tracker still drains, demonstrating the documented insufficiency.                | TODO   | section V7                                    |
| M8 | systemd and Podman                                             | `systemd-run --user` with `TimeoutStopSec=35` and `systemctl --user stop`; Podman if installed (it is not installed on the author's machine, so record "not run").            | Unit result matches the exit code; Podman recorded honestly as not run, or run.                                  | TODO   | section V8                                    |

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
  cover the same scenarios, and records the removal in the progress log; T9 checks it.

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
and the documentation together, and restores the 10 s deadline and the component constants. This
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

- Retrospective: `Not yet assessed`
- Create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md`
  only for material discoveries; otherwise add a progress-log line saying why none was needed.
- The independent `review-task` reviewer records its result in `agent-review-reports.md`.

## References

- Parent EPIC: #1488
- Related issues: #1586 (supervisor outcomes), #2410 (SI-22, listener drain), #2418 (batched persisted writes), #2132 (SIGTERM at main)
- Related ADRs: `docs/adrs/20261021184946_order_shutdown_by_event_flow_and_drain_event_listeners.md`, `docs/adrs/20260723184019_separate_configuration_value_invariants_from_consistency_validation.md`
- Decisions: [Q3 and Q4](../../../features/shutdown-process/questions.md#q3) (outcome and deadlines), [Q9](../../../features/shutdown-process/questions.md#q9) (container evidence)
