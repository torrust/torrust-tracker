# Profile Test Design Review

## Refactor Plan Item 1 - Module-Owned Profile Tests

### Arrange

The existing test suite separates into extraction-owned tests in `lib.rs` and schema/profiling
tests whose production decisions live in `profile.rs`.

### Act

Move the schema-projection and strict-profile tests into `profile.rs` without changing fixture
inputs, Acts, or assertions.

### Assert

The package still runs 41 library tests. Test paths show schema and strict-profile behavior under
`profile::tests`, while delimiter, YAML, mapping-root, and universal-envelope behavior remains
under `lib.rs` tests.

### Review

This reorganization changes ownership visibility only. Existing tests already state the causal
document fixtures and observable diagnostic categories, so adding parallel coverage would create
duplication rather than confidence. The focused package run proves each test still compiles at its
new private-module boundary.

## Refactor Plan Item 2 - Closed Strict Profile Kind

### Arrange

The accepted issue and EPIC fixtures exercise each strict validator, while legacy records exercise
the permissive path. A schema-version-one document with an unrecognized string `doc-type` is the
remaining boundary between strict recognition and permissive classification.

### Act

Classify the three document shapes through `validate` after representing recognized strict kinds
with a private closed enum.

### Assert

Issue and EPIC documents produce their corresponding `Profile` variants. The unknown document
type remains `Profile::Permissive`, preserving the documented compatibility behavior. Temporarily
swapping the enum-to-validator mappings must make the accepted issue/EPIC classification tests
fail, proving they guard dispatch rather than recognition alone.

### Review

The enum is intentionally private because it exists only between recognition and validation.
Public profile variants, generated schema bytes, and diagnostics remain the behavior boundary.
The red mutation swapped the two enum-to-validator mappings; both accepted-profile classification
tests failed with the expected unknown-field diagnostics before the correct mappings were restored.

## Refactor Plan Item 3 - Shared Strict Validation Sequence

### Arrange

An issue and an EPIC can each contain both an unknown field and an invalid semantic-link value.
The existing validation contract establishes unknown-field detection before reference syntax.

### Act

Validate the two malformed documents through the strict-profile boundary after routing their
declarative field and allowed-value contracts through one private shared sequence.

### Assert

Both documents report `UnknownField`, preserving the same diagnostic precedence despite their
different profile contracts. A temporary mutation that skips the shared known-field stage must
make this test fail before the stage is restored.

### Review

The helper remains private and takes only the differing declarative contract plus a typed final
invariant validator. Deserialization remains generic but strongly typed at each call site; no
trait or dynamic-dispatch framework is introduced.
Removing the stage directly was rejected at compile time by the strict dead-code policy. Moving
the stage after reference syntax produced `InvalidReferenceSyntax` rather than `UnknownField` for
both documents, so the precedence test failed before the original sequence was restored.

## Refactor Plan Item 4 - UTC-Minute Layout and Calendar Predicates

### Arrange

UTC-minute strings have independent byte-layout and calendar rules. Boundary cases include valid
and invalid leap days, invalid month/day combinations, clock bounds, short input, and non-ASCII
input. Existing profile tests retain coverage for YAML double-quote source style.

### Act

Evaluate the pure layout predicate for every table row and evaluate the calendar predicate only
when the value has the required fixed layout.

### Assert

Each row yields the expected layout and calendar result without panicking. Temporarily accepting
every calendar-shaped value must fail the table rows for invalid dates and clock bounds before the
real predicate is restored.

### Review

The diagnostic-producing validator remains the single composition point for layout, calendar, and
YAML source-style rules. No time dependency or public behavior change is introduced.
Replacing the calendar conjunction with an always-permissive disjunction made the table fail at
the non-leap-year `2025-02-29 23:59` row before the real predicate was restored.

## Refactor Plan Item 5 - Aggregate-Owned Invariants

### Arrange

The existing strict-profile tests exercise issue identifiers, specification paths, timestamps, and
EPIC ownership. Those checks inspect only the fully deserialized aggregate and the original YAML.

### Act

Run the existing strict-profile validation boundary after relocating each aggregate's invariant
sequence to its private `validate_invariants` method.

### Assert

The same fixtures and inline invalid states retain their profile results and diagnostic categories.
No new test is needed because this is ownership-only relocation with unchanged observable inputs,
order, and assertions; schema drift verification proves the model projection remains unchanged.

## Refactor Plan Item 6 - Definition-Owned Structural Validation

### Arrange

The existing cross-profile precedence test supplies both an unrecognized field and an invalid
skill link. It requires known-field validation to precede reference syntax for both profile
definitions.

### Act

Validate those documents after moving the field, required-field, and allowed-value sequence into
`StrictProfileDefinition::validate_structure`.

### Assert

Both profiles still report `UnknownField`. Temporarily calling reference validation before the
definition method must change the result to `InvalidReferenceSyntax`, proving the existing test
guards the behavior after ownership moves.
The mutation produced `InvalidReferenceSyntax` for the cross-profile test before the original
order was restored.

## Refactor Plan Item 7 - Kind-Owned Recognition and Definition Lookup

### Arrange

Accepted issue and EPIC fixtures exercise each closed kind and its profile definition. A
schema-version-one `note` record exercises the permissive unknown-kind boundary.

### Act

Classify those documents after moving literal-to-kind recognition and definition lookup onto
`StrictProfileKind`.

### Assert

Issue and EPIC fixtures still produce their matching `Profile` variants, while `note` remains
permissive. Swapping typed dispatch mappings must fail the two accepted-fixture tests, proving
that centralizing lookup does not decouple a kind from its validator.
The swapped mapping made both fixture tests fail with the opposite `doc-type` deserialization
error before the matching typed dispatch was restored.

## Refactor Plan Item 8 - Unified Diagnostic Construction

### Arrange

Existing extraction and strict-profile tests assert every public diagnostic category.

### Act

Construct every diagnostic through `Diagnostic::new` while keeping each category and message
expression unchanged.

### Assert

The existing extraction and strict-profile tests retain their diagnostic category results. This is
a behavior-preserving construction cleanup, so no extra test is needed.
