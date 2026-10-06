---
semantic-links:
  skill-links:
    - write-markdown-docs
  related-artifacts:
    - docs/issues/open/2245-2243-review-numeric-protocol-wire-conversions/ISSUE.md
    - docs/adrs/20260723184019_separate_configuration_value_invariants_from_consistency_validation.md
---

# Implementation Retrospective — Review Numeric Protocol Wire Conversions (#2245)

## Purpose

Record evidence-based process improvements discovered while implementing #2245. This is a blameless
review of the implementation approach; it does not replace acceptance-criteria verification.

## Outcome

Both allowances are removed. A156 hid one cast, the scrape parser's cursor offset, now a checked
conversion mapped to the parser's existing `invalid data` error. A171 was a real defect: a
configured `interval` above `i32::MAX` was sent over UDP as a negative value. A
`saturating_wire_i32` helper now clamps the interval and the peer counts to `i32::MAX`. Unit tests,
a mutation check, and manual scenario M1 verify it. Rejecting such an interval at configuration
load is deferred to a follow-up issue.

## What Went Well

1. Removing the crate-level allowance temporarily and letting Clippy list what it hid turned an
   unknown scope ("A156 covers the whole crate") into one concrete cast before any decision.
2. Reproducing the suspected interval defect against a local tracker before proposing options made
   the decision evidence-based. The same configuration later served as manual scenario M1.

## What Changed During Implementation

Two maintainer-approved decisions were revised after implementation started:

1. **A156**: the approved `try_from(..).expect(..)` failed pedantic Clippy (`missing_panics_doc`).
   The `expect` made the public `Request::parse_bytes` look like it could panic. The fix became a
   mapped parse error.
2. **A171 interval**: the approved configuration-load check in `Core::validate` conflicted with an
   accepted ADR, which requires a single-value bound to be a typed newtype rejected during
   deserialization, and explicitly rejects one-field rules in `Validator`. Configuration-load
   rejection was moved to a follow-up issue.

Both revisions went back to the maintainer before any code was committed.

## Root Cause

Both options were presented without checking the constraints that apply to the code they would
touch:

1. The visibility of the enclosing function, which decides whether a new panic path triggers
   `missing_panics_doc` under the workspace's pedantic Clippy profile.
2. The ADRs governing the target module. The validator source links its ADR in its first line, but
   it was read only when implementation began.

## Improvements for Future Work

1. Before offering a conversion option that adds `expect` or `unwrap`, check whether the enclosing
   function is public. If it is, say in the option that it needs a `# Panics` section or must
   become an error.
2. Before offering an option that places logic in a module, read the ADRs that module links or that
   cover its area (`docs/adrs/`), and state any constraint in the option.

## Avoiding Overcorrection

Do not add a repository-wide rule or checklist step for these two checks. Both are cheap reads that
belong in the option-preparation habit described above, and in each case the revision cost only one
extra question. Retaining the ADR and pedantic Clippy as the authorities is correct.

## Evidence

- Issue specification: `ISSUE.md` in this folder (Decisions and Progress Log entries of 2026-10-06
  11:32 and 11:41 UTC)
- Manual verification: `manual-verification-evidence.md` in this folder
- Commits: `refactor(udp-protocol): [#2245] remove the crate-level truncation allowance` and
  `fix(udp-server): [#2245] clamp announce wire fields instead of wrapping negative`
