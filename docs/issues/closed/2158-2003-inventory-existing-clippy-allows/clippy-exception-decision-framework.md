# Clippy Exception Decision Framework

This framework governs outcomes for historical Clippy allowance remediation initiated by #2158,
including #2244, #2245, #2246, and #2261. It supplements #2157's native-reason and
temporary-removal-information requirements; it does not replace them.

## Decision Order

For every diagnostic, prefer the first applicable outcome:

1. Remove the allowance and apply a behavior-preserving fix.
2. Change the public API when that produces a clearer, safer design. The upcoming 4.0.0 major
   release permits a breaking change; preserving an existing API is not itself a reason to retain
   an allowance.
3. Retain a source-specific exception when it expresses a permanent, evidence-backed design or
   tool limitation.
4. Defer the exception temporarily only when a concrete refactor or external change is already
   identified and the native reason names its stable issue reference or removal condition.

## Reasonable Permanent Exceptions

An indefinite source-specific exception is reasonable only when its native `reason` records the
local evidence for one of these cases:

- A protocol, correctness, safety, or interoperability invariant makes Clippy's suggested change
  less clear or incorrect.
- A deliberate public API or domain-model choice is preferable after considering a breaking API
  change.
- A Clippy false positive, macro expansion, generated code, or supported-toolchain limitation
  cannot be corrected locally.
- A required external interface constrains the implementation, and the exception documents that
  interface and its relevant invariant.

The source-level reason must name the specific invariant, interface, or tool limitation. Broad
crate-level suppression and statements that only say the warning is intentional are insufficient.

## Temporary Exceptions

A temporary exception is reasonable only when a planned change makes removal feasible but cannot
be completed in the current work. Its native reason must include a stable follow-up issue or a
specific removal condition, as #2157 requires.

For example, `too_many_arguments` may be deferred while an identified refactor introduces a
coherent parameter type. A vague intention to refactor later is insufficient. The follow-up work
must remove the exception or explicitly reclassify it under the permanent criteria above; it must
not remain temporary merely because the refactor was forgotten.

## Required Evidence

Each remediation issue records, for every owned inventory entry:

- the diagnostic and affected source;
- the selected outcome and the applicable criterion above;
- any protocol, API, bounds, or toolchain evidence supporting a retained exception; and
- the focused validation that confirms the outcome.

Update this framework only through maintainer review when a new reusable exception category is
needed.
