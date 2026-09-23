---
doc-type: refactor-plan
status: in_progress
related-issue: 2280
spec-path: docs/issues/open/2280-2264-generate-v1-schema-and-verify-drift/profile-refactor-plan.md
last-updated-utc: "2026-09-23 09:18"
semantic-links:
  skill-links:
    - create-refactor-plan
    - write-unit-test
  related-artifacts:
    - .github/skills/dev/planning/create-refactor-plan/SKILL.md
    - contrib/dev-tools/checks/frontmatter-validator/src/profile.rs
    - contrib/dev-tools/checks/frontmatter-validator/src/lib.rs
    - docs/issues/open/2280-2264-generate-v1-schema-and-verify-drift/ISSUE.md
    - docs/issues/open/2280-2264-generate-v1-schema-and-verify-drift/schema-command-refactor-plan.md
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

### Second Series Assessment (2026-09-23)

After items 1-4 the maintainer asked for a responsibility review of the remaining free functions
and an independent complexity audit. The audit passed every function (highest cyclomatic
complexity 10 in `strict_document_type`; no function exceeds 40 lines or nesting depth 2), so the
second series is driven by responsibility assignment and Rust idiom, not by complexity.

Responsibility mapping of the current module:

| Owner today          | Behavior                                                            | Assessment                                              |
| -------------------- | ------------------------------------------------------------------- | ------------------------------------------------------- |
| `Issue` / `Epic`     | Canonical data only                                                 | Should also own their typed invariant checks (item 5)   |
| `StrictProfileDefinition` | Allowed fields and allowed values                              | Should own the structural stage that applies them (item 6) |
| `StrictProfileKind`  | Closed dispatch                                                     | Should own the `doc-type` mapping and its definition (item 7) |
| Free predicates      | Reference syntax, path, integer, identifier, UTC-minute, YAML style | Pure, single-purpose; stay as functions                 |
| Free constructors    | `invalid_field_value`, `invalid_reference_syntax`, inline literals  | Inconsistent; unify (item 8)                            |
| `field_key`          | `&str` to `serde_yaml::Value` key conversion                        | Redundant: `Mapping::get` already indexes by `&str` (item 9) |

A `UtcMinute` value object with `TryFrom<&str>` was evaluated and deferred; see the rejected
alternatives.

## Items

### 1. [x] Move strict-profile tests beside their owning module [HIGH impact / LOW effort]

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

### 2. [x] Represent strict profile kind as a closed enum [MEDIUM impact / LOW effort]

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

### 3. [x] Make shared strict-profile validation order explicit [MEDIUM impact / MEDIUM effort]

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

### 4. [x] Separate UTC-minute format predicates from calendar validation [MEDIUM impact / MEDIUM effort]

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

### 5. [ ] Move typed invariant checks onto `Issue` and `Epic` [MEDIUM impact / LOW effort]

**Problem**: `validate_issue_invariants(&Issue, &str)` and `validate_epic_invariants(&Epic, &str)`
are free functions whose only input is the aggregate they inspect. The rules they encode (positive
identifiers, repository-relative `spec-path`, non-empty `branch`/`epic-owner`, UTC-minute
`last-updated-utc`) are invariants of those types, so the types should own them.

**Files**:

- `contrib/dev-tools/checks/frontmatter-validator/src/profile.rs`

**Change**: Replace both functions with private `fn validate_invariants(&self, yaml: &str)` methods
in `impl Issue` and `impl Epic`, and pass `Issue::validate_invariants` / `Epic::validate_invariants`
to the shared sequence. Keep the field-level helpers (`validate_optional_positive_integer`,
`validate_repository_relative_path`, `validate_non_empty_string`, `validate_utc_minute_string`) as
free functions: they are reusable pure checks parameterized by a field name. No new tests: the
existing invariant tests already fix the observable diagnostics; confirm they still pass and that
the schema bytes are unchanged.

---

### 6. [ ] Let `StrictProfileDefinition` own the structural stage [MEDIUM impact / LOW effort]

**Problem**: `StrictProfileDefinition` holds the allowed fields and allowed values, but the code
that applies them lives in `validate_strict_profile`, which reaches into `definition.fields` and
`definition.allowed_values`. Data and the behavior that interprets it are split, and the shared
`status` value list is duplicated between `ISSUE_PROFILE` and `EPIC_PROFILE`.

**Files**:

- `contrib/dev-tools/checks/frontmatter-validator/src/profile.rs`

**Change**: Add `fn validate_structure(&self, values: &Mapping) -> Result<(), Diagnostic>` to
`StrictProfileDefinition` that runs known-field, required-field, and allowed-value validation in the
current order, and have `validate_strict_profile` call it before reference syntax. Extract the
shared lifecycle list into one `const STATUS_VALUES: &[&str]`. Keep `validate_known_fields`,
`validate_required_fields`, and `validate_allowed_string` as free helpers or fold them into the
method only if that stays under the complexity thresholds. The item-3 precedence test already
guards the order; re-run its mutation (move `validate_known_fields` after reference syntax) to
confirm it still fails.

---

### 7. [ ] Let `StrictProfileKind` own recognition and definition lookup [MEDIUM impact / LOW effort]

**Problem**: `strict_document_type` reads `doc-type` twice (once as `Option<&str>` to decide
whether a non-integer `schema-version` is an error, once to map it to a kind) and hard-codes the
`"issue" | "epic"` literal set in two places. `validate` then matches the kind to pick a
definition and a validator by hand. The kind is the natural owner of the `doc-type` mapping and of
"which definition applies".

**Files**:

- `contrib/dev-tools/checks/frontmatter-validator/src/profile.rs`

**Change**: Add `StrictProfileKind::from_doc_type(&str) -> Option<Self>` and use it for both the
candidate check and the final mapping, so the literal set exists once. Add
`fn definition(&self) -> &'static StrictProfileDefinition` so `validate` obtains the definition
from the kind rather than from a parallel match. Keep the typed dispatch to `Issue`/`Epic` in
`validate` because the return type differs per arm; do not introduce a trait. Preserve every
diagnostic category and message. Extend the existing recognition tests only if a new branch
appears; otherwise rely on the accepted-fixture and permissive tests, and repeat the item-2 swap
mutation.

---

### 8. [ ] Unify diagnostic construction [LOW impact / LOW effort]

**Problem**: The module builds `Diagnostic` values three ways: inline struct literals (eight
sites), `const fn invalid_field_value`, and `const fn invalid_reference_syntax`. Readers cannot
tell whether the difference is meaningful, and adding a category means choosing a style again.

**Files**:

- `contrib/dev-tools/checks/frontmatter-validator/src/lib.rs`
- `contrib/dev-tools/checks/frontmatter-validator/src/profile.rs`

**Change**: Add `Diagnostic::new(category: DiagnosticCategory, message: impl Into<String>)` in
`lib.rs` beside the type, and use it everywhere in both modules. Remove the two profile-local
constructors unless they remain clearer at their call sites. No behavior change; the existing
category assertions cover it.

---

### 9. [ ] Index YAML mappings by `&str` directly [LOW impact / TRIVIAL effort]

**Problem**: `field_key(field)` allocates a `serde_yaml::Value::String` for every lookup, but
`serde_yaml::Mapping::get` and `contains_key` already accept `&str` through the sealed `Index`
trait (`serde_yaml` 0.9.34, `mapping.rs`). The helper adds an allocation and an indirection for
no semantic gain.

**Files**:

- `contrib/dev-tools/checks/frontmatter-validator/src/profile.rs`

**Change**: Replace `values.get(field_key("x"))` with `values.get("x")` and
`values.contains_key(field_key(f))` with `values.contains_key(*f)` (or `f` as the loop item type
dictates), then delete `field_key`. Pure mechanical change covered by the existing suite.

---

### 10. [ ] Declare shared schema patterns once [LOW impact / LOW effort]

**Problem**: The repository-relative path regex appears three times (`Issue::spec_path`,
`Epic::spec_path`, and inside the `RelatedArtifact` union pattern) and the UTC-minute regex twice.
A future contract change must edit every copy, and `schemars_derive` 1.2.1 already accepts an
expression for `regex(pattern = ...)`, so a literal is not required.

**Files**:

- `contrib/dev-tools/checks/frontmatter-validator/src/profile.rs`

**Change**: Introduce `const REPOSITORY_RELATIVE_PATH_PATTERN: &str` and
`const UTC_MINUTE_PATTERN: &str`, reference them from the `#[schemars(regex(pattern = ...))]`
attributes, and build the `RelatedArtifact` union pattern from the path constant with `concat!`
only if it stays readable; otherwise keep the union literal and add a test asserting it starts
with the path constant. The tracked artifact must remain byte-identical: run
`frontmatter-schema check` and the deterministic schema tests after the change.

---

## Order of Execution

| Order | Status | Item                                                     | Impact | Effort |
| ----- | ------ | -------------------------------------------------------- | ------ | ------ |
| 1     | [x]    | Move strict-profile tests beside their owning module     | High   | Low    |
| 2     | [x]    | Represent strict profile kind as a closed enum           | Medium | Low    |
| 3     | [x]    | Make shared strict-profile validation order explicit     | Medium | Medium |
| 4     | [x]    | Separate UTC-minute predicates from calendar validation  | Medium | Medium |
| 5     | [ ]    | Move typed invariant checks onto `Issue` and `Epic`      | Medium | Low    |
| 6     | [ ]    | Let `StrictProfileDefinition` own the structural stage   | Medium | Low    |
| 7     | [ ]    | Let `StrictProfileKind` own recognition and definition   | Medium | Low    |
| 8     | [ ]    | Unify diagnostic construction                            | Low    | Low    |
| 9     | [ ]    | Index YAML mappings by `&str` directly                   | Low    | Trivial |
| 10    | [ ]    | Declare shared schema patterns once                      | Low    | Low    |

Item 1 establishes the correct test ownership before production refactors. Item 2 makes the
dispatch boundary explicit, so item 3 can share the validation sequence without retaining raw
profile strings. Item 4 is independent but follows the structural work because it isolates one
invariant inside the final typed-validation phase.

Items 5-7 assign behavior to the types that already own the data it interprets; they are ordered
from the most self-contained (`Issue`/`Epic` methods) to the one that touches `validate`'s
dispatch (kind-owned definition lookup), so each step leaves the previous one's tests untouched.
Items 8-10 are idiom clean-ups with no ownership change; item 9 is placed before item 10 because
it removes code, whereas item 10 must prove schema bytes are unchanged.

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
- **Deferred: `UtcMinute` value object with `TryFrom<&str>`/`FromStr`.** A validated newtype is
  the idiomatic Rust shape for `is_valid_utc_minute_calendar`, and the repository already uses it
  (`SkillName`, `RelatedArtifact`, `FileName`). It is deferred because using it as the type of
  `last_updated_utc` would (a) move calendar failures from the `InvalidFieldValue` invariant stage
  into serde deserialization, changing the reported category to `WrongScalarType` and its
  precedence relative to reference-syntax errors, and (b) still leave the YAML double-quote rule
  outside the type, since that rule is about the source scalar style, not the value. Revisit if a
  later contract needs timestamp semantics (ordering, comparison, formatting) rather than
  validation alone, and then decide the precedence change explicitly.
- **Deferred: `RepositoryRelativePath` newtype for `spec-path`.** Same precedence concern as the
  timestamp: the current `InvalidFieldValue` diagnostic would become a deserialization error.

## Review Decision

The maintainer approved items 1-4 on 2026-09-23 and they were implemented as four signed commits.
Items 5-10 were added on 2026-09-23 after the responsibility and complexity review and await
maintainer approval before implementation. Implement items in order, record the required
prose-first test-design review, and commit each item separately.
