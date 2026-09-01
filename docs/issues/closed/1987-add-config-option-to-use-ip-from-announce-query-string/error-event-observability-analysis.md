# Error Event Observability Analysis

**Decision:** Option B was selected on 2026-08-19. The #1987-specific rejection
event and metric were removed. The strict validation behavior remains.

The deferred cross-service work is recorded in the draft EPIC
[`generalize-error-events.md`](../../drafts/generalize-error-events.md). No
GitHub issue or additional ADR will be created until that draft is refined.

## Context

Issue #1987 introduced a bounded metric for rejected non-empty HTTP announce
`ip` parameters. The initial implementation emits
`Event::TcpAnnouncePeerIpRejected` so the statistics listener can increment the
metric.

This adds a rejected-request outcome to the HTTP-core event enum. The existing
[Events Are Objective Facts ADR](../../../adrs/20260727000000_events_are_objective_facts.md)
requires event variants to describe objective facts rather than consumer-specific
policy decisions. The proposed event must therefore be evaluated as a potential
public event-stream contract, not merely as a metrics implementation detail.

## Problem

The tracker needs to decide whether rejected requests should be exposed as events
and, if so, establish a coherent contract for all services. Introducing only one
rejection event for one HTTP announce validation rule could mislead consumers into
thinking that the event stream exposes every rejected request.

The metric is operationally useful: it could show whether stricter handling of
the optional BEP 3 `ip` parameter rejects clients in practice. However, it is a
convenience for operators, not a prerequisite for the core correctness of strict
validation. Existing request logs can be used to investigate problematic client
usage while a broader observability design is deferred.

## Questions Requiring a Decision

### 1. Which rejected requests emit events?

Possible scopes include:

- only selected protocol-validation rejections;
- every announce rejection after request parsing;
- every HTTP request rejection, including announce and scrape;
- all rejected requests across HTTP, UDP, REST, and future services.

A partial scope must be explicit. Otherwise consumers cannot distinguish an
unobserved rejection from a service failure or a missing event implementation.

### 2. Do parser failures emit events?

Some errors occur before `AnnounceService::handle_announce`, while a request is
being parsed or extracted. A complete rejected-request event contract must decide
whether those failures emit events and how request context is represented when
no valid request DTO exists.

### 3. Are authentication and authorization denials included?

Authentication-key failures, private-mode authentication failures, whitelist
denials, malformed requests, and tracker-core announce failures have different
context and privacy properties. Omitting them from a supposedly general rejection
contract would create an inconsistent interface; including them expands the work
substantially.

### 4. What is the stable reason API?

The service return type `HttpAnnounceError` is not a suitable event payload. It
contains internal error composition and wrapped implementation details that may
change independently of an event contract.

If rejection events are exposed, they should use dedicated, bounded,
consumer-safe reason types. The design must decide whether those enums are:

- exhaustive and changed only in a major version; or
- explicitly non-exhaustive/extensible, with consumer guidance for unknown
  future values.

### 5. What privacy constraints apply?

Event payloads must not include raw client-controlled query values by default.
Raw values may contain addresses, hostnames, identifiers, or arbitrary text. A
stable event contract should carry only the minimum request context and bounded
reason classifications required by consumers.

### 6. Who are event-stream consumers?

The event stream currently decouples internal metrics and future consumers from
request handling. Before exposing rejected outcomes, the project must state
whether the stream is:

- an internal implementation mechanism;
- a supported API for in-process or external consumers; or
- both, with versioning and compatibility guarantees.

## Options

### Option A: Keep the #1987 rejection event and metric

Treat `TcpAnnouncePeerIpRejected` as a narrow, supported event contract.

**Advantages:** preserves the immediate operational metric and event-based
decoupling.

**Disadvantages:** establishes a one-off error-observability precedent without
answering the questions above. Consumers may incorrectly infer comprehensive
rejection coverage.

### Option B: Remove the #1987 rejection event and metric

Keep strict `ip` validation and bencoded failure responses. Defer rejected
request event/metric design to a dedicated cross-service effort.

**Advantages:** keeps #1987 focused on its protocol and configuration behavior;
avoids an accidental public event API; preserves the existing event architecture
without directly coupling announce handling to metrics.

**Disadvantages:** operators do not receive a dedicated aggregate rejection
counter initially. They must use existing request logs and normal diagnostics to
assess client compatibility.

### Option C: Design a general rejected-request event contract now

Create an ADR and implement a coherent event family across relevant HTTP and UDP
request paths.

**Advantages:** provides a deliberate, homogeneous observable interface.

**Disadvantages:** significantly expands scope, requires decisions for all
questions above, and should not be implemented only for HTTP announce `ip`
validation.

## Decision

**Option B** was selected: `TcpAnnouncePeerIpRejected`, its bounded reason type,
and its metric were removed. Strict protocol validation remains intact.

The future work is documented as a local draft EPIC rather than an open GitHub
issue. It must define the public rejected-request event contract before adding
similar metrics or events. The future contract should cover its explicitly
chosen service/method boundaries consistently, define reason stability and
privacy rules, and retain the objective-fact principles in the Events ADR.

## Relationship to the Events ADR

The existing ADR remains applicable: events must be objective facts and not
consumer-specific policy decisions. This analysis identifies an additional
unresolved boundary: even an objective rejection outcome needs a deliberate,
complete, stable contract before it is added to a shared event enum.
