---
semantic-links:
  skill-links:
    - write-markdown-docs
  related-artifacts:
    - docs/index.md
    - docs/packages.md
    - docs/application-jobs.md
    - docs/architecture/events.md
    - docs/architecture/tracker-instance-architecture.md
    - docs/adrs/20260727180000_shared_services_across_tracker_instances.md
    - docs/skills/semantic-skill-link-convention.md
---

# Runtime Architecture

This directory contains evolving guides to the tracker application's runtime
composition and behavior. These guides explain the architecture implemented by
the current codebase; they do not replace the accepted decisions in
[`docs/adrs/`](../adrs/README.md).

## Guides

| Document                                                             | Purpose                                                                                                                               |
| -------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------- |
| [tracker-instance-architecture.md](tracker-instance-architecture.md) | Process topology, shared services, listener-instance boundaries, configuration placement, and when to use separate tracker processes. |
| [events.md](events.md)                                               | Event topology, event consumers, aggregate statistics, and per-listener metrics policy.                                               |

## Related Documentation

| Document                                                                                                                                 | Purpose                                                                 |
| ---------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------- |
| [../packages.md](../packages.md)                                                                                                         | Workspace package catalog, dependency layers, and boundary enforcement. |
| [../application-jobs.md](../application-jobs.md)                                                                                         | Current job ownership, lifecycle, and shutdown behavior.                |
| [../adrs/20260727180000_shared_services_across_tracker_instances.md](../adrs/20260727180000_shared_services_across_tracker_instances.md) | Accepted decision to share selected services across listener instances. |
| [../adrs/20260727000000_events_are_objective_facts.md](../adrs/20260727000000_events_are_objective_facts.md)                             | Accepted event-design rule.                                             |

## Documentation Boundary

Use this directory to explain how the running application is composed and why
its components interact as they do. Record a new, consequential architectural
choice in an ADR, then update these guides to explain the resulting runtime
model. Keep package dependency rules in `packages.md` and background-task
ownership in `application-jobs.md` rather than duplicating them here.
