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
