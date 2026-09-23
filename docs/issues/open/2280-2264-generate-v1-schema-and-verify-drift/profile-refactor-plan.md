---
doc-type: refactor-plan
status: in_progress
related-issue: 2280
spec-path: docs/issues/open/2280-2264-generate-v1-schema-and-verify-drift/profile-refactor-plan.md
last-updated-utc: "2026-09-23 07:36"
semantic-links:
  skill-links:
    - create-refactor-plan
    - write-unit-test
  related-artifacts:
    - .github/skills/dev/planning/create-refactor-plan/SKILL.md
    - contrib/dev-tools/checks/frontmatter-validator/src/profile.rs
    - contrib/dev-tools/checks/frontmatter-validator/src/lib.rs
    - docs/issues/open/2280-2264-generate-v1-schema-and-verify-drift/ISSUE.md
    - docs/issues/open/2280-2264-generate-v1-schema-and-verify-drift/refactor-plan.md
    - issue #2280
---

<!-- skill-link: create-refactor-plan -->

# Refactor Plan - Frontmatter Profile Validation

## Goal

Improve the test ownership, readability, and maintainability of strict frontmatter profile
validation in `profile.rs` without changing the v1 schema contract, generated artifact bytes, or
the public diagnostics consumed by later command work.

Related issue: `docs/issues/open/2280-2264-generate-v1-schema-and-verify-drift/ISSUE.md`

This plan is distinct from the completed schema-command plan. That plan refactored process,
filesystem, and artifact behavior in `src/bin/frontmatter-schema.rs`; this plan owns only the
canonical strict-profile model and validation decisions in `src/profile.rs`.

## Current Assessment

`profile.rs` owns four decisions: identifying a strict v1 profile, validating its structural
rules, validating v1-specific invariants, and projecting the canonical model as JSON Schema.
`lib.rs` currently contains the 41 crate tests, including the profile-validation tests, even though
the production decisions and private helpers they exercise live in `profile.rs`. This hides the
module's test boundary and makes `lib.rs` mix extraction, schema, and profile concerns.

The strict-profile path has two smaller design debts:

- `strict_document_type` returns a raw `&str` that `validate` matches again, so the closed
  `issue`/`epic` domain is encoded as strings across two decisions.
- `validate_issue` and `validate_epic` repeat the same validation sequence, differing only in a
  declarative contract (fields and allowed values) and a profile-specific final invariant check.

The timestamp validator is correct for existing tested cases, but its format, calendar, and YAML
style predicates are folded into one branch. This makes boundary coverage hard to see and makes a
future format change unnecessarily risky.

## Items

### 1. [ ] Move strict-profile tests beside their owning module [HIGH impact / LOW effort]

**Problem**: Profile-validation tests live in `lib.rs`, while `profile.rs` owns `validate`, schema
projection, profile recognition, invariant checks, and reference syntax. The test module instead
mixes three owners: extraction, schema encoding, and strict-profile validation. A reader cannot
discover the profile contract or its coverage from `profile.rs` alone.

**Files**:

- `contrib/dev-tools/checks/frontmatter-validator/src/lib.rs`
- `contrib/dev-tools/checks/frontmatter-validator/src/profile.rs`

**Change**: Move only strict-profile and schema-projection tests to a `#[cfg(test)]` module in
`profile.rs`. Keep delimiter extraction and universal-envelope tests in `lib.rs`, where their
decisions remain owned. Preserve behavior-driven names, causal Arrange/Act/Assert comments, and
fixture inputs. Add a short test-boundary inventory comment in each test module that names the
decisions it owns. Do not change production behavior in this item.

---

### 2. [ ] Represent strict profile kind as a closed enum [MEDIUM impact / LOW effort]

**Problem**: `strict_document_type` identifies only `issue` and `epic` but returns `Option<&str>`;
`validate` matches those string literals again. Adding a new strict profile could update one branch
without the other, and raw strings conceal the closed domain from the type system.

**Files**:

- `contrib/dev-tools/checks/frontmatter-validator/src/profile.rs`

**Change**: Introduce a private `StrictProfileKind` enum with `Issue` and `Epic` variants.
`strict_document_type` returns `Result<Option<StrictProfileKind>, Diagnostic>` and `validate`
dispatches on its exhaustive match. Preserve permissive behavior for legacy and unknown document
classes, diagnostic categories, and display messages. Add focused tests for each kind and the
permissive path; mutate one enum-to-validator mapping before commit to prove the tests detect an
incorrect dispatch.

---

### 3. [ ] Make shared strict-profile validation order explicit [MEDIUM impact / MEDIUM effort]

**Problem**: `validate_issue` and `validate_epic` each perform known-field validation, required
field validation, allowed-value validation, reference-syntax validation, deserialization, and
invariant validation. The duplicated sequence can drift, changing diagnostic precedence between
profiles. The profile-specific differences are contracts, but the workflow is shared.

**Files**:

- `contrib/dev-tools/checks/frontmatter-validator/src/profile.rs`

**Change**: Extract a small private profile definition or shared validation helper that accepts the
allowed fields, allowed string values, and final typed invariant validator. Keep deserialization
and profile-specific invariants strongly typed; do not add traits, dynamic dispatch, or a generic
validation framework. Test that equivalent issue and EPIC malformed inputs retain the intended
diagnostic category and precedence. Preserve JSON Schema output byte-for-byte and record a
red-first mutation that skips one common validation stage.

---

### 4. [ ] Separate UTC-minute format predicates from calendar validation [MEDIUM impact / MEDIUM effort]

**Problem**: `validate_utc_minute_string` combines byte layout, ASCII digits, calendar validity,
and double-quoted YAML source-style validation in one compound condition. Existing tests exercise
representative errors, but the independent boundary rules are not visible or directly testable.

**Files**:

- `contrib/dev-tools/checks/frontmatter-validator/src/profile.rs`

**Change**: Extract pure private predicates for fixed UTC-minute layout and calendar validity, then
retain `validate_utc_minute_string` as the one diagnostic-producing composition point. Add a
table-driven unit test covering leap days, invalid month/day combinations, hour/minute bounds, and
short or non-ASCII input without panics. Keep the YAML double-quote source-style rule separate and
test it through the current profile-validation boundary. Do not introduce a time library or change
the accepted `YYYY-MM-DD HH:MM` contract.

---

## Order of Execution

| Order | Status | Item                                                     | Impact | Effort |
| ----- | ------ | -------------------------------------------------------- | ------ | ------ |
| 1     | [ ]    | Move strict-profile tests beside their owning module     | High   | Low    |
| 2     | [ ]    | Represent strict profile kind as a closed enum           | Medium | Low    |
| 3     | [ ]    | Make shared strict-profile validation order explicit     | Medium | Medium |
| 4     | [ ]    | Separate UTC-minute predicates from calendar validation  | Medium | Medium |

Item 1 establishes the correct test ownership before production refactors. Item 2 makes the
dispatch boundary explicit, so item 3 can share the validation sequence without retaining raw
profile strings. Item 4 is independent but follows the structural work because it isolates one
invariant inside the final typed-validation phase.

## Test Design Rules

- Before each item, record an Arrange/Act/Assert review beside this plan or in the #2280 issue
  folder before changing production code.
- Test at the lowest owning boundary: extraction tests stay in `lib.rs`; strict-profile decisions
  move to `profile.rs`.
- Retain semantic fixtures where they express a document contract; use focused inline YAML only
  when it makes one causal invalid state clearer.
- Preserve category and user-facing message assertions where they are part of the validator's
  public contract; do not test private helper structure when an observable profile result is
  clearer.
- For every new regression behavior, temporarily reintroduce the prior or defective behavior,
  confirm the focused test fails, restore the implementation, and record the result.
- After each item, run `cargo test --package frontmatter-validator`,
  `cargo run --offline --package frontmatter-validator --bin frontmatter-schema -- check`,
  `cargo +nightly fmt --all -- --check`,
  `cargo clippy --package frontmatter-validator -- -D warnings`, `cargo machete --with-metadata`,
  and `linter all`.

## Non-Goals and Rejected Alternatives

- No change to v1 field names, values, profile rules, schema projection, or generated schema bytes.
- No change to universal frontmatter extraction, document-ownership policy, file discovery, or
  command behavior owned by #2281.
- No external date/time dependency: the existing format is deliberately small and can be validated
  by pure local predicates.
- No generic validator framework, trait hierarchy, or dynamic dispatch for two strict profiles.
- No extension of the completed #2280 schema-command plan; preserve it as an accurate historical
  record of the binary-only refactor series.

## Review Decision

The maintainer approved this plan on 2026-09-23. Implement items in order, record the required
prose-first test-design review, and commit each item separately.
