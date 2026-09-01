---
semantic-links:
  skill-links:
    - write-markdown-docs
  related-artifacts:
    - docs/architecture/events.md
    - docs/issues/closed/2035-fix-duplicate-port-zero-tracker-instance-bootstrap/ISSUE.md
    - docs/issues/closed/2035-fix-duplicate-port-zero-tracker-instance-bootstrap/evidence.md
---

# Archived Implementation Attempt

## Status

The former implementation attempt is preserved on
`archive/2035-bootstrap-identity-attempt`. It is reference material only and
must not be merged or blindly cherry-picked.

## Evidence

The attempt established the bootstrap collision recorded in
[evidence.md](evidence.md): address-keyed containers overwrite one of two
configuration blocks that use the same `0.0.0.0:0` binding. It also established
that retaining bootstrap identity alone does not implement the intended
per-listener UDP metrics policy. UDP server events currently use a single
application-wide container, event bus, and aggregate repository.

Manual verification on the attempt showed HTTP behavior consistent with its
per-listener producer gate, while UDP server metrics still included traffic from
a metrics-disabled listener. The attempt further exposed that suppressing event
production for metrics can hide cookie-error facts from UDP banning.

## Pause Decision

The work was paused because bootstrap identity, runtime identity, and event
metrics policy must be delivered in a coherent order:

1. Land #2036 canonical runtime service and configuration-instance identity.
2. Land event-metrics normalization: always emit objective events, filter
   metrics in listeners by stable identity, and keep banning independent.
3. Reimplement #2035 from scratch on those foundations and verify duplicate
   port-zero listeners.

The archive remains useful for the reproduction, tests, and design questions;
it is not an accepted implementation. See
[the revised #2035 plan](ISSUE.md) and the
[event architecture guide](../../../architecture/events.md).
