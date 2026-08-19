---
semantic-links:
  related-artifacts:
    - docs/adrs/index.md
    - docs/issues/closed/1136-1978-configurable-udp-connection-id-validation-policy.md
    - packages/udp-core/src/event.rs
    - packages/udp-server/src/event.rs
    - packages/http-core/src/event.rs
    - packages/swarm-coordination-registry/src/event.rs
    - docs/issues/drafts/generalize-error-events.md
    - docs/issues/open/1987-add-config-option-to-use-ip-from-announce-query-string/error-event-observability-analysis.md
---

# Events Are Objective Facts

## Description

The tracker uses a pub/sub event system across multiple packages. Each event bus
has its own `event.rs` module that defines an `Event` enum. Multiple listeners
(ban handler, statistics, metrics, …) subscribe to these events and react
independently.

During the implementation of the configurable UDP connection ID validation policy
(issue [#1136][1136]), a design mistake was made:

A new `UdpCookieErrorObserved` event variant was created specifically so that the
ban handler would **not** react to it when the validation policy was `Disabled`.
The reasoning was: "if we emit a different event, the existing ban listener won't
see it as a ban-worthy error."

This is the wrong pattern.

## Agreement

**Event variants must be objective facts** about what happened in the system.
They must not be designed around what a particular consumer should or should not
do in response to them.

### The wrong pattern

Creating a new event variant (e.g. `UdpCookieErrorObserved`) that is structurally
identical to an existing one (`UdpError { ConnectionCookie }`) but named
differently so a specific listener silently ignores it.

```rust
// WRONG — variant exists purely to prevent the ban handler from reacting
Event::UdpCookieErrorObserved { context, kind, error }
```

Problems:

- Couples the event schema to the internal behaviour of one consumer.
- Hides a policy decision (ban enforcement on/off) inside the event layer.
- Any new consumer that subscribes to `UdpError` but not `UdpCookieErrorObserved`
  will silently miss the observation entirely.
- Forces every future listener to duplicate the routing logic.

### The right pattern

Emit the same objective event regardless of the active policy. Gate enforcement
at the **enforcement point**, not at the event definition.

```rust
// RIGHT — objective fact: a cookie error occurred
Event::UdpError {
    context: ConnectionContext::new(client_socket_addr, server_service_binding),
    kind: Some(UdpRequestKind::Announce { .. }),
    error: ErrorKind::ConnectionCookie(cookie_error.to_string()),
}
```

The ban handler receives the event and increments the counter (observability
data). The main server loop — the **enforcement point** — decides whether to act:

```rust
// Enforcement is gated on the active policy, not on the event type
let ban_enforcement_active = connection_id_validation == ConnectionIdValidationPolicy::Strict;

if ban_enforcement_active && ban_service.is_banned(&req.from.ip()) {
    // block request
}
```

This keeps three concerns cleanly separated:

| Concern | Owner                       | Behaviour when policy = Disabled |
| ------- | --------------------------- | -------------------------------- |
| Observe | event emitter (handler)     | always emits `UdpError`          |
| Count   | ban listener                | always increments counter        |
| Enforce | main loop `is_banned` check | **skipped** — no enforcement     |

### Naming heuristic

A well-named event variant:

- Uses past tense, from the system's perspective (`UdpError`, `UdpRequestBanned`).
- Does **not** embed a policy or mode (`UdpCookieErrorInLenientMode` — bad).
- Does **not** mirror a consumer's internal decision (`UdpCookieErrorObserved`
  as a synonym for "ignore this error" — bad).

**Red flag**: if you find yourself adding a new variant whose name includes a
policy name, mode name, or whose sole purpose is to make a listener ignore it —
stop and move the policy to the consumer or the enforcement point instead.

**Structural red flag**: if a proposed new variant has the same fields as an
existing one, ask "why not reuse the existing event and change the consumer?"
Almost always the answer is: change the consumer.

### Alternatives Considered

**Keep `UdpCookieErrorObserved` and teach each listener to ignore it.**

Rejected because it scales poorly: every new consumer must know which variants
to skip, the event enum becomes a leaky log of consumer decisions, and the
intent is hidden from new contributors.

**Skip event emission entirely in `Disabled` mode.**

Rejected because it breaks observability — the connection ID error counter would
no longer reflect real traffic when validation is disabled, defeating the purpose
of the metric.

### Consequences

#### Positive

- Event consumers remain fully decoupled from policy decisions.
- Observability is preserved regardless of the active policy.
- Adding a new consumer requires no knowledge of existing consumers' reactions.
- The design principle is explicit and co-located with all event definitions via
  the ADR link in each `event.rs` module.

#### Negative

- Enforcement logic is spread between the event emitter (which still emits the
  event) and the enforcement point (which decides to act or not). This split must
  be documented — which it now is, in the module-level doc of each `event.rs`.

[1136]: https://github.com/torrust/torrust-tracker/issues/1136
