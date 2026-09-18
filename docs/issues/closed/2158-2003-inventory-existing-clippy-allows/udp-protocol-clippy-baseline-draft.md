---
spec-path: docs/issues/closed/2158-2003-inventory-existing-clippy-allows/ISSUE.md
last-updated-utc: 2026-09-18 14:40
semantic-links:
	related-artifacts:
		- docs/issues/closed/2158-2003-inventory-existing-clippy-allows/ISSUE.md
---

# Draft: UDP Protocol Clippy Baseline Remediation

## Status

Design input retained in #2158. This draft was promoted into approved follow-up issue #2261.

## Problem

`udp-protocol` originated from vendored `aquatic_udp_protocol` code and retains a broad crate-level
set of Clippy allows. They cover style, documentation, API ergonomics, numeric conversion, imports,
and macro expansion. A crate-level baseline hides which source items still require an exception.

## Candidate Scope

- Inventory entries A156-A168 in `packages/udp-protocol/src/lib.rs`.

## Questions to Resolve

1. Which warnings are caused by `FromBytes` macro expansion or other third-party generated code?
2. Which vendored code can adopt focused modern Clippy suggestions without changing BEP wire behavior?
3. Which public protocol APIs require their current ownership, return, or documentation shape?
4. Can all retained suppressions move to a narrow item or module scope with a specific rationale?

## Stable Removal Condition

Every current crate-level Clippy allow is removed, replaced by a focused source-level allowance with
a native `reason` and concrete evidence, or eliminated through a behavior-preserving remediation.

## Expected Validation

- UDP protocol unit and documentation tests.
- Protocol encoding/decoding boundary tests for changed conversions.
- `linter all`.

## EPIC Assessment Input

If this review uncovers broad vendored-code modernization needs beyond Clippy remediation, group it
with other protocol-maintenance work. Otherwise keep it a focused package-scoped follow-up.
