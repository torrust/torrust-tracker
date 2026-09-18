---
spec-path: docs/issues/closed/2158-2003-inventory-existing-clippy-allows/ISSUE.md
last-updated-utc: 2026-09-18 14:40
semantic-links:
   related-artifacts:
      - docs/issues/closed/2158-2003-inventory-existing-clippy-allows/ISSUE.md
---

# Draft: Wire Numeric Conversion Validation

## Status

Design input retained in #2158. This draft was promoted into approved follow-up issue #2245.

## Problem

The UDP protocol crate has a crate-level `cast_possible_truncation` allowance inherited from its
vendored origin. UDP announce response construction also narrows interval and peer-count values to
wire-width signed integers. The current code does not make every protocol-width bound evident at the
conversion boundary.

## Candidate Scope

- Inventory entry A156: crate-level `cast_possible_truncation` in `udp-protocol`.
- Inventory entry A171: UDP announce response construction in `udp-server`.

## Questions to Resolve

1. Which conversions are fixed by BEP field widths, and which are tracker configuration or aggregate
   values that require runtime validation?
2. Do existing protocol newtypes validate their input ranges, including the values supplied by server
   response construction?
3. Can the crate-level allowance be replaced with narrow, documented item-level exceptions while
   preserving the vendored implementation's behavior?

## Stable Removal Condition

Every narrowing conversion has a protocol-specific checked conversion or an input type that proves
the wire-width bound. Tests cover valid extremes and rejected out-of-range values. The broad crate
allowance is removed or narrowed, and the UDP-server response conversion no longer needs an allow.

## Expected Validation

- UDP protocol serialization/deserialization tests covering boundary values.
- UDP server response tests covering accepted and rejected values.
- `linter all`.

## EPIC Assessment Input

If the inventory identifies similar protocol-width conversions across HTTP and UDP packages, this
work belongs under a numeric-conversion EPIC that can establish a consistent wire-boundary policy.
