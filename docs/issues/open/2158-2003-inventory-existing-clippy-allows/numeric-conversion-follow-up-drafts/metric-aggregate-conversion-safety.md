# Draft: Metric Aggregate Conversion Safety

## Status

Design input retained in #2158; do not create a GitHub issue from this document until the maintainer
re-evaluates the complete numeric-conversion inventory.

## Problem

Metric collection aggregate APIs return `f64` for gauges and averages. HTTP core, UDP core, and UDP
server accessors currently cast these values to `u64`, suppressing `cast_sign_loss` and
`cast_possible_truncation`. The casts silently truncate fractions and do not make the behavior for
negative, non-finite, or out-of-range values explicit.

## Candidate Scope

- Inventory entries A080-A087: HTTP-core TCP request aggregates.
- Inventory entries A113-A114: swarm-registry inactive-count gauges.
- Inventory entries A143-A154: UDP-core request aggregates.
- Inventory entry A170 and entries A178-A222: UDP-server count, counter, and processing-time aggregates.
- Inventory entries A224-A227: UDP-server repository test aggregates.

## Questions to Resolve

1. Are each of these metrics semantically non-negative integer counters, or are some gauges whose
   fractional values are meaningful?
2. Should the `torrust-metrics` aggregate API distinguish typed counter output from gauge/average
   output, or should tracker-owned helpers validate and convert at the domain boundary?
3. What result is required for negative, `NaN`, infinite, and values above `u64::MAX`?
4. Which conversions should round, truncate, reject, or remain floating-point values?

## Stable Removal Condition

Every affected conversion has a typed or checked boundary that explicitly defines the behavior for
negative, non-finite, fractional, and out-of-range values. Focused tests cover that behavior. The
corresponding direct `f64` to `u64` casts and Clippy allow attributes are removed.

## Expected Validation

- Focused tests for conversion behavior and metric semantics.
- Relevant HTTP core, UDP core, and UDP server package tests.
- `linter all`.

## EPIC Assessment Input

This draft spans three tracker packages and may require a contract decision in the extracted
`torrust-metrics` crate. If the completed inventory reveals other cross-package numeric conversion
families with the same need for typed boundaries, organize this work under a new numeric-conversion
EPIC rather than opening it as an isolated issue.
