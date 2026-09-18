---
spec-path: docs/issues/closed/2158-2003-inventory-existing-clippy-allows/ISSUE.md
last-updated-utc: 2026-09-18 14:40
semantic-links:
   related-artifacts:
      - docs/issues/closed/2158-2003-inventory-existing-clippy-allows/ISSUE.md
---

# Draft: Domain Numeric Conversion Contracts

## Status

Design input retained in #2158. This draft was promoted into approved follow-up issue #2246.

## Problem

Several domain conversions narrow numeric values across independently owned types:

- serializing milliseconds from `DurationSinceUnixEpoch` to `u64`;
- publishing swarm peer counts as `u32`; and
- converting a positive client-supplied peer limit from `i32` to `usize`.

Some call sites already describe a local invariant, but the invariant is not consistently encoded or
tested at the conversion boundary.

## Candidate Scope

- Inventory entry A099: timestamp serialization in `primitives`.
- Inventory entry A123: swarm metadata peer counts in torrent repository benchmarking.
- Inventory entry A129: positive peer-limit conversion in tracker core.

## Questions to Resolve

1. Does each source type have a documented maximum that proves the conversion is lossless on every
   supported platform?
2. Should each boundary use `TryFrom` or a domain constructor instead of `as`?
3. Which cases already have sufficient local evidence to retain directly in #2158 rather than become
   follow-up work?

## Stable Removal Condition

Each conversion is lossless by its source type and target type, or is handled through an explicit
checked conversion with deterministic failure behavior. Focused tests cover each boundary. The
related Clippy allow attributes are removed.

## Expected Validation

- Focused primitives, tracker-core, and benchmarking tests for boundary behavior.
- `linter all`.

## EPIC Assessment Input

If the final inventory shows this pattern crossing multiple domain packages or requiring shared
conversion abstractions, include it in a numeric-conversion EPIC. Otherwise, keep it as a small
package-scoped remediation issue or resolve it directly in #2158.
