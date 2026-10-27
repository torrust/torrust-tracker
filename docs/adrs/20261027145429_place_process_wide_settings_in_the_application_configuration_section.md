---
semantic-links:
  skill-links:
    - create-adr
    - write-markdown-docs
  related-artifacts:
    - docs/issues/open/2668-1488-si-20-configure-shutdown-policy/ISSUE.md
    - docs/architecture/tracker-instance-architecture.md
    - docs/adrs/20260723184019_separate_configuration_value_invariants_from_consistency_validation.md
    - packages/configuration/src/v3_0_0/application.rs
    - packages/configuration/src/v3_0_0/shutdown.rs
    - packages/configuration/AGENTS.md
---

<!-- skill-link: create-adr -->

# Place Process-Wide Settings in the `[application]` Configuration Section

## Scope

Repository-level decision, so it belongs in `docs/adrs/`. It sets a convention for the v3 configuration
schema in `packages/configuration`, which every service package and the application bootstrap
(`src/app.rs`, `src/main.rs`) consume, so no single package owns it.

## Description

Until issue #2668 the configuration had no place for a setting that belongs to the running process
as a whole. `[logging]` is process-wide, `[core]` holds tracker policy and also one application job
interval (`inactive_peer_cleanup_interval`), and the other sections are named after one service
(`http_api`, `health_check_api`, `udp_tracker_server`) or a listener list (`http_trackers`,
`udp_trackers`).

The shutdown policy needed such a place: a process deadline, the drain budgets of four different
services, and the drain budget of the event listeners, which the composition root creates and no
service owns. The [tracker instance architecture](../architecture/tracker-instance-architecture.md)
already said that "process-wide policy belongs in an application-level v3 configuration section" and
that a setting goes on a listener only when each listener can apply it independently. A top-level
`[shutdown]` section would have answered the first need and left the next process-wide setting
(event-bus capacity and the activity-metrics interval are candidates) without a home.

## Agreement

Process-wide settings live in an optional top-level `[application]` section. Each topic is a
sub-table, `[application.<topic>]`, and the first one is `[application.shutdown]`.

A setting belongs in `[application]` when all of these hold:

1. It governs the process that the composition root builds, not one service, one listener, or the
   shared tracker policy.
2. No single service package owns the concept: the composition root, the supervisor, or the event
   buses it creates apply it.
3. An operator may reasonably change it without changing which services run.

A setting that one service applies independently stays on that service's section, as the
[configuration placement rule](../architecture/tracker-instance-architecture.md#configuration-placement-rule)
says. Tracker policy and shared-service settings stay in `[core]`.

The schema rules of `packages/configuration` apply unchanged to the new section: every key is
optional with a documented default, unknown keys are rejected, a constrained value is a newtype whose
invariants hold after parsing, and a rule that relates several values is checked by a `Validator`
(see [the ADR on value invariants and consistency validation](20260723184019_separate_configuration_value_invariants_from_consistency_validation.md)).
The section keeps schema version `3.0.0`, because it is optional and `3.0.0` is not yet published.

Nothing moves into `[application]` as part of this decision. `[logging]`, the `[core]` job intervals,
and the other existing sections stay where they are; moving one is a breaking change that needs its
own decision. No empty `startup` table is created.

### Alternatives Considered

- **A top-level `[shutdown]` section.** Rejected: too specific. The next process-wide setting would
  need another root section.
- **Per-service budgets** (a drain key on each of `http_trackers`, `http_api`, `health_check_api`,
  `udp_trackers`). Rejected: it scatters one contract over five places, makes the relation between
  the budgets impossible to validate in one place, and the event-listener drain has no service to
  carry it. No demonstrated need exists for per-instance budgets.
- **`[orchestrator]`, `[main]`, `[runtime]`, `[process]`.** Rejected: the orchestrator is the
  environment around the tracker, which the tracker does not configure; `main` names a function, not
  a concern; `runtime` suggests the Tokio runtime; `process` suggests the operating-system process
  attributes. `[application]` is the word the architecture documents already use for the layer that
  owns the composition root and the supervisor.

## Consequences

- A future process-wide setting has a documented home and a test of whether it belongs there.
- The schema gains one more root section, optional, so existing configuration files stay valid.
- `[logging]` stays a sibling of `[application]` although it is process-wide. That inconsistency is
  accepted until a decision moves it.
- Operators who set one key through the environment use the form
  `TORRUST_TRACKER_CONFIG_OVERRIDE_APPLICATION__SHUTDOWN__DEADLINE_IN_SECS`.

## Date

2026-10-27

## References

- Issue #2668 (SI-20): configure the shutdown policy and deployment contract; its specification holds
  the candidate list and the comparison of section names.
- [Tracker instance architecture](../architecture/tracker-instance-architecture.md), "Shared Services"
  and "Configuration Placement Rule".
- [Budget the shutdown in two chained stages inside one process deadline](20261027145446_budget_shutdown_in_two_chained_stages_inside_one_process_deadline.md),
  the decision that the first `[application]` sub-table implements.
- EPIC #1488: overhaul tracker shutdown.
