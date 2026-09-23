---
doc-type: refactor-plan
status: in_progress
related-issue: 2280
spec-path: docs/issues/open/2280-2264-generate-v1-schema-and-verify-drift/crate-layout-refactor-plan.md
last-updated-utc: "2026-09-23 10:42"
semantic-links:
  skill-links:
    - create-refactor-plan
    - write-unit-test
  related-artifacts:
    - .github/skills/dev/planning/create-refactor-plan/SKILL.md
    - contrib/dev-tools/checks/frontmatter-validator/src/lib.rs
    - contrib/dev-tools/checks/frontmatter-validator/src/profile.rs
    - contrib/dev-tools/checks/frontmatter-validator/src/bin/frontmatter-schema.rs
    - docs/issues/open/2280-2264-generate-v1-schema-and-verify-drift/ISSUE.md
    - docs/issues/open/2280-2264-generate-v1-schema-and-verify-drift/profile-refactor-plan.md
    - docs/issues/open/2280-2264-generate-v1-schema-and-verify-drift/schema-command-refactor-plan.md
    - docs/schemas/README.md
    - issue #2280
    - issue #2281
---

<!-- skill-link: create-refactor-plan -->

# Refactor Plan - Frontmatter Validator Crate Layout

## Goal

Make the module boundaries of the `frontmatter-validator` crate match its real dependency edges
so that each module owns exactly one concern, the seams between modules are named queries rather
than shared raw data, and the crate's own test inputs cannot be removed by a documentation
lifecycle step. The v1 schema contract, the tracked artifact bytes, the diagnostic categories, and
their precedence do not change.

Related issue: `docs/issues/open/2280-2264-generate-v1-schema-and-verify-drift/ISSUE.md`

Predecessors: the completed `schema-command-refactor-plan.md` (process, filesystem, and artifact
behavior in `src/bin/frontmatter-schema.rs`) and the completed `profile-refactor-plan.md`
(responsibility assignment inside `src/profile.rs`). Both plans deliberately stayed inside one
file. This plan is separate because every item below crosses a file boundary: it changes what
`lib.rs` exposes to `profile.rs`, adds modules, or moves files the crate depends on.

## Current Assessment

The crate has three files and one binary dependency edge:

| File                          | Lines | Declared concern                                  | Consumers                       |
| ----------------------------- | ----- | ------------------------------------------------- | ------------------------------- |
| `src/lib.rs`                  | 409   | Markdown extraction, universal envelope           | `profile.rs`, tests             |
| `src/profile.rs`              | 1122  | Strict v1 models, recognition, validation, schema | `lib.rs` re-exports, the binary |
| `src/bin/frontmatter-schema.rs` | 643 | Schema artifact generation and drift command      | none                            |

The binary depends on the library only through `v1_schema_json`. The independent complexity audit
recorded in `profile-refactor-plan.md` passed every function, so none of the items below is driven
by function size or branching. They are driven by four observations about what crosses the seams.

### Observation 1: raw YAML source crosses from extraction into profile validation

`Frontmatter.yaml` (`lib.rs` line 17) is `pub(crate)` for one consumer: `profile.rs` lines
491-530, where `validate_utc_minute_string` receives the whole YAML source and
`has_double_quoted_timestamp` plus `strip_yaml_comment` scan it for the `last-updated-utc:` line
to decide whether the scalar was double-quoted. That is lexical parsing of YAML source text, which
`docs/schemas/README.md` itself assigns to the extraction layer ("YAML parsing and scalar lexemes,
exact double-quote requirements"). The profile module should ask a question about a field; it
should not receive the source and parse it. The dependency is also hidden: `Issue::validate_invariants(&self, yaml: &str)`
and `Epic::validate_invariants(&self, yaml: &str)` thread an opaque string through three calls
before it is interpreted.

### Observation 2: the diagnostic vocabulary is documented as an extraction type

`DiagnosticCategory` (`lib.rs` line 62) is documented as "A category for frontmatter extraction or
universal-envelope failures" and `Diagnostic` (line 87) as "A deterministic failure found while
extracting frontmatter". Six of the ten categories (`UnknownField`, `MissingRequiredField`,
`WrongScalarType`, `InvalidAllowedValue`, `InvalidFieldValue`, `InvalidReferenceSyntax`) are
strict-profile failures produced only by `profile.rs`. The types are the crate-wide failure
vocabulary, consumed by both modules today and by the #2281 command adapter next. The
documentation is wrong, and the placement inside the extraction module implies an ownership that
does not exist.

### Observation 3: pure value syntax is interleaved with mapping-level policy

`profile.rs` contains two layers with no dependency between them:

- Pure predicates over `&str` with no `serde_yaml`, `Mapping`, or `Diagnostic` dependency:
  `is_skill_name`, `is_related_artifact`, `is_repository_relative_path`, `is_issue_reference`,
  `is_review_finding`, `is_positive_integer`, `is_lowercase_identifier` (lines 414-456),
  `has_utc_minute_layout`, `is_valid_utc_minute_calendar`, `is_leap_year` (lines 502-552), and
  the regex constants `REPOSITORY_RELATIVE_PATH_PATTERN`, `UTC_MINUTE_PATTERN` (lines 72-73) plus
  the `SkillName` and `RelatedArtifact` `pattern` attributes (lines 180, 186).
- Policy over a YAML `Mapping` that produces `Diagnostic` values: recognition, structural checks,
  deserialization, and invariant checks.

The first layer is the frozen v1 reference and value syntax. Each regex constant must agree with
one Rust predicate, and `docs/schemas/README.md` records that the Rust side is authoritative. Today
the two halves of each pair are far apart: `REPOSITORY_RELATIVE_PATH_PATTERN` is declared at line
72 and `is_repository_relative_path` at line 422. A reader checking that the schema and the
validator agree has to hold both in mind across 350 lines. Separating the syntax layer gives the
pairs one home, gives the UTC table test a module that tests exactly what it exercises, and gives
issue #2281 a dependency-free module to reuse for path and reference checks.

### Observation 4: the crate's test inputs live in an archived issue folder

Six tests use `include_str!` on
`docs/issues/closed/2265-2264-inventory-markdown-frontmatter-contracts/frontmatter-fixtures/`
(`profile.rs` lines 770-831 and 1032). The `cleanup-completed-issues` skill documents that closed
issue specs may be permanently deleted on request. Deleting that folder would break compilation of
a permanent repository tool. The fixture manifest also states that whole-tree validation must
exclude the fixture directory; that exclusion is easier to express and justify when the fixtures
are visibly owned by the tool rather than by a closed issue.

### Observation 5: reference syntax is validated on two paths

`validate_reference_syntax` (`profile.rs` line 387) walks the universal `SemanticLinks` strings
and emits `InvalidReferenceSyntax`. Afterwards `deserialize_strict` (line 604) parses the same
values into `SkillName` and `RelatedArtifact` through `TryFrom<String>` (lines 189-211). Because the
first pass always runs first, the `TryFrom` error branches are unreachable through `validate`; they
exist for the schema projection and the typed model. The precedence reason is real:
`InvalidReferenceSyntax` must be reported before `WrongScalarType`. The duplication is in call
paths, not logic (both delegate to the same predicates), so this is the least urgent observation
and the only one whose resolution changes observable message text. It is recorded as an
approval-gated item so the decision is visible either way.

### Observation 6: `profile.rs` is not ordered for reading

`StrictProfileKind::definition` (line 35) references `ISSUE_PROFILE`, declared at line 660.
The public entry point `validate` (line 286) sits between private model types and private helpers.
The field lists and profile definitions (lines 614-672) sit after the scalar validators they do not
use. After items 2-4 remove the syntax layer and diagnostics, the remaining file can be ordered
public surface first, then dispatch, then definitions, then policy helpers.

### Target layout

```text
contrib/dev-tools/checks/frontmatter-validator/
  fixtures/                       accepted/ and rejected/ Markdown fixtures plus manifest
  src/
    lib.rs                        extraction, universal envelope, Frontmatter queries, re-exports
    diagnostic.rs                 Diagnostic, DiagnosticCategory
    syntax.rs                     v1 value syntax: regex constants paired with their predicates
    profile.rs                    strict models, kinds, definitions, validate, mapping policy
    bin/frontmatter-schema.rs     unchanged
```

`lib.rs` keeps `pub use` re-exports so every public path (`frontmatter_validator::Diagnostic`,
`frontmatter_validator::v1_schema_json`, and so on) is unchanged. The binary is not touched.

## Items

### 1. [x] Move the Markdown fixtures into the crate [HIGH impact / LOW effort]

**Problem**: Six `include_str!` paths in `profile.rs` (lines 770-831 and 1032) reach five
directories up into `docs/issues/closed/2265-.../frontmatter-fixtures/`. The repository's
documented issue-cleanup workflow permits permanent deletion of closed specs, so a documentation
maintenance step can break `cargo test` for a permanent tool. The fixtures are the crate's
compatibility baseline, not issue evidence.

**Files**:

- `docs/issues/closed/2265-2264-inventory-markdown-frontmatter-contracts/frontmatter-fixtures/`
  (moved)
- `contrib/dev-tools/checks/frontmatter-validator/fixtures/` (new location)
- `contrib/dev-tools/checks/frontmatter-validator/src/profile.rs`
- `docs/issues/closed/2265-2264-inventory-markdown-frontmatter-contracts/ISSUE.md`

**Change**: `git mv` the whole `frontmatter-fixtures/` directory, including `manifest.md`, to
`contrib/dev-tools/checks/frontmatter-validator/fixtures/`. Update the six `include_str!` paths to
`../fixtures/accepted/issue.md` and siblings. Keep the manifest's statement that whole-tree
validation must exclude the directory. Add one relocation note to the #2265 progress log rather
than rewriting its historical task table. Run `linter all` to confirm markdownlint, cspell, and
lychee accept the new location without ignore-file changes; if an ignore entry is needed, add it
to the linter-specific ignore file and record why.

---

### 2. [ ] Home the diagnostic vocabulary in its own module [MEDIUM impact / LOW effort]

**Problem**: `Diagnostic` and `DiagnosticCategory` (`lib.rs` lines 58-100) are documented as
extraction failures but six of ten categories are produced only by `profile.rs`. The #2281 command
adapter will consume them as the crate's output contract. Their placement inside the extraction
module misstates ownership and the doc comments are inaccurate today.

**Files**:

- `contrib/dev-tools/checks/frontmatter-validator/src/lib.rs`
- `contrib/dev-tools/checks/frontmatter-validator/src/diagnostic.rs` (new)
- `contrib/dev-tools/checks/frontmatter-validator/src/profile.rs`

**Change**: Move `Diagnostic`, `DiagnosticCategory`, and `Diagnostic::new` to `diagnostic.rs`.
Make `Diagnostic::new` `pub(crate)`. Re-export both types from `lib.rs` so public paths are
unchanged. Correct the doc comments: the category enum describes "a deterministic frontmatter
extraction or validation failure" and the struct "a deterministic failure found while extracting or
validating frontmatter". Update the `lib.rs` module doc so it no longer implies it owns
diagnostics. No behavior change; the existing 44 library tests and 24 binary tests must pass
unchanged.

---

### 3. [ ] Replace the raw YAML seam with a `Frontmatter` query [MEDIUM impact / LOW effort]

**Problem**: `Frontmatter.yaml` is exposed `pub(crate)` (`lib.rs` line 17) so that
`profile.rs` can lexically scan YAML source for the `last-updated-utc:` line
(`has_double_quoted_timestamp` and `strip_yaml_comment`, lines 516-530). Scalar-style detection is
an extraction-layer concern; the profile layer should not parse YAML text. The dependency is also
invisible at the call sites because `validate_invariants(&self, yaml: &str)` passes an opaque
string.

**Files**:

- `contrib/dev-tools/checks/frontmatter-validator/src/lib.rs`
- `contrib/dev-tools/checks/frontmatter-validator/src/profile.rs`

**Change**: Add `Frontmatter::has_double_quoted_scalar(&self, field: &str) -> bool` in `lib.rs`,
implemented by moving the two lexical helpers there and generalizing the hard-coded
`last-updated-utc:` prefix to the `field` parameter. Make `yaml` private. Change
`Issue::validate_invariants` and `Epic::validate_invariants` to take `&Frontmatter` and call
`frontmatter.has_double_quoted_scalar("last-updated-utc")`; `validate_utc_minute_string` receives
the value and the boolean. Add a direct `lib.rs` test for the query covering a double-quoted scalar,
an unquoted scalar, and a quoted scalar followed by a tab-separated comment, so the ownership is
visible from the extraction module's own tests. Mutation proof: invert the query's return and
confirm `it_should_reject_an_unquoted_strict_timestamp` and the new direct test fail.

---

### 4. [ ] Extract the v1 value syntax into `syntax.rs` [MEDIUM impact / MEDIUM effort]

**Problem**: `profile.rs` interleaves dependency-free string predicates (lines 414-456 and
502-552) and their schema regex constants (lines 72-73, 180, 186) with mapping-level policy that
produces diagnostics. Each regex must agree with one predicate and the two halves are hundreds of
lines apart. The UTC table test is the only test that exercises predicates directly, and it lives in
a module whose stated ownership is strict-profile decisions.

**Files**:

- `contrib/dev-tools/checks/frontmatter-validator/src/syntax.rs` (new)
- `contrib/dev-tools/checks/frontmatter-validator/src/profile.rs`
- `contrib/dev-tools/checks/frontmatter-validator/src/lib.rs`

**Change**: Create `syntax.rs` containing, as `pub(crate)` items, the four regex constants
(`REPOSITORY_RELATIVE_PATH_PATTERN`, `UTC_MINUTE_PATTERN`, and new `SKILL_NAME_PATTERN`,
`RELATED_ARTIFACT_PATTERN` lifted from the `schemars(extend("pattern" = ...))` attributes) and the
ten predicates. Declare each constant immediately above the predicate it must agree with, with a
one-line comment naming the pairing. Move
`it_should_distinguish_utc_minute_layout_from_calendar_boundaries` into `syntax.rs` tests. Keep
`SkillName`, `RelatedArtifact`, and their `TryFrom` impls in `profile.rs`; their attributes reference
the constants. Keep `validate_*` functions that produce `Diagnostic` in `profile.rs`. Add
`mod syntax;` to `lib.rs` without re-exporting anything. Run
`cargo run --offline --package frontmatter-validator --bin frontmatter-schema -- check` to prove the
artifact is byte-identical, and extend the existing schema test to assert the `SkillName` and
`RelatedArtifact` patterns equal the new constants, mirroring the item-10 assertions in the profile
plan.

---

### 5. [ ] Validate reference syntax on one path [MEDIUM impact / MEDIUM effort]

**Problem**: `validate_reference_syntax` (`profile.rs` line 387) checks universal `SemanticLinks`
strings, then `deserialize_strict` re-parses the same values through `SkillName` and
`RelatedArtifact` `TryFrom<String>`, whose error branches are therefore unreachable via `validate`.
The `Option<&SemanticLinks>` parameter is threaded through `validate`, `validate_issue`,
`validate_epic`, and `validate_strict_profile` only for this pass. The
`it_should_report_a_missing_envelope_instead_of_panicking` test constructs an inconsistent
`Frontmatter` (`values` has the key, `semantic_links` is `None`) to cover a state that only exists
because of the two paths.

**Files**:

- `contrib/dev-tools/checks/frontmatter-validator/src/profile.rs`

**Change**: After structural validation, deserialize `values["semantic-links"]` into
`StrictSemanticLinks` and map its error to `InvalidReferenceSyntax`, then run `deserialize_strict`
for the full profile. Delete `validate_reference_syntax` and the `semantic_links` parameters.
Delete the inconsistent-state test, because the state becomes unrepresentable. Diagnostic
categories and their precedence are unchanged; message text changes because serde prefixes the
field path. This item was called out for separate approval because it alters message wording that
a future adapter could surface; the maintainer approved it on 2026-09-23. Mutation proof: swap the
mapped category to `WrongScalarType` and confirm the four reference-syntax rejection tests fail.

---

### 6. [ ] Order `profile.rs` for top-down reading [LOW impact / LOW effort]

**Problem**: After items 2-4 the file still declares the public entry point `validate` between
private models and private helpers, and `StrictProfileKind::definition` references
`ISSUE_PROFILE` declared roughly 600 lines later.

**Files**:

- `contrib/dev-tools/checks/frontmatter-validator/src/profile.rs`

**Change**: Reorder declarations only: public `Profile`, `FrontmatterV1`, `v1_schema`,
`v1_schema_json`, and `validate` first; then `StrictProfileKind` and `strict_document_type`; then
`StrictProfileDefinition`, the field lists, and the two profile constants; then the `Issue` and
`Epic` models with their enums and invariants; then `StrictSemanticLinks`, `SkillName`,
`RelatedArtifact`; then mapping-policy helpers. No signature, visibility, or behavior change.
`cargo +nightly fmt --all -- --check` and the full test suite are the only required proof.

---

## Order of Execution

| Order | Status | Item                                                | Impact | Effort |
| ----- | ------ | --------------------------------------------------- | ------ | ------ |
| 1     | [x]    | Move the Markdown fixtures into the crate           | High   | Low    |
| 2     | [ ]    | Home the diagnostic vocabulary in its own module    | Medium | Low    |
| 3     | [ ]    | Replace the raw YAML seam with a `Frontmatter` query | Medium | Low    |
| 4     | [ ]    | Extract the v1 value syntax into `syntax.rs`        | Medium | Medium |
| 5     | [ ]    | Validate reference syntax on one path               | Medium | Medium |
| 6     | [ ]    | Order `profile.rs` for top-down reading             | Low    | Low    |

Each item is one signed commit that also flips its checkbox in the heading and the table. Every
item runs the full gate before commit: `cargo test --package frontmatter-validator`, the offline
`frontmatter-schema check`, `cargo +nightly fmt --all -- --check`,
`cargo clippy --package frontmatter-validator -- -D warnings`, `cargo machete --with-metadata`,
and `linter all`. Items that add a test record the prose-first Arrange-Act-Assert design and the
mutation result in `crate-layout-test-design-review.md` beside this plan.

## Non-Goals and Rejected Alternatives

- **Rejected: `profile/` folder module with `models.rs`, `validators/`, and `schema.rs`.** After
  items 2 and 4 the remaining `profile.rs` is one concern (mapping-level strict-profile policy over
  the canonical models) plus its colocated tests. Splitting further would force `pub(super)`
  visibility between files that share one decision and would scatter the validation sequence that
  item 3 of the profile plan made explicit.
- **Rejected: separate `schema.rs`.** `FrontmatterV1`, `v1_schema`, and `v1_schema_json` are
  thirty lines that project the models declared beside them. Moving them adds a module whose only
  job is to import from `profile.rs`; the `lib.rs` re-export already isolates what the binary sees.
- **Rejected: moving binary tests to `tests/`.** They exercise private `Command`, `SchemaArtifact`,
  and `Error` behavior; inline placement is what makes that possible without widening visibility.
- **Rejected: a `regex` dependency to prove pattern/predicate agreement.** Item 4 makes the pairs
  adjacent and pins the generated patterns; a runtime regex check would add a dependency to a
  validator whose Rust predicates are already authoritative.
- **Deferred: `UtcMinute` and `RepositoryRelativePath` value objects.** Still deferred for the
  precedence reasons recorded in `profile-refactor-plan.md`; item 3 here keeps the timestamp checks
  as predicates.
- **Out of scope: `src/bin/frontmatter-schema.rs`.** Its one dependency edge (`v1_schema_json`)
  and private test surface are already correct.

## Review Decision

Drafted on 2026-09-23 after the maintainer asked for a cross-file organization review following
the completed profile plan. Items 1-4 and 6 are behavior-preserving and keep the generated schema
byte-identical. Item 5 changes diagnostic message text and was called out for separate approval.
The maintainer approved all six items on 2026-09-23. Implement items in order, record the required
prose-first test-design review, and commit each item separately.
