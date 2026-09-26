# Issue #2281 Test-Design Review

Prose-first Arrange-Act-Assert reviews for each test-producing increment, as required by the
`write-unit-test` skill.

## T1a - Diagnostic Severity, Field Path, and Category Names

Tests added:

- `diagnostic::tests::it_should_serialize_each_category_as_its_stable_kebab_case_name`
- `diagnostic::tests::it_should_serialize_each_severity_as_its_stable_lowercase_name`

Tests changed (assertion extended from category alone to `(category, field_path)`):

- `profile::tests::it_should_reject_the_wrong_scalar_fixture` (`github-issue`)
- `profile::tests::it_should_reject_the_unknown_field_fixture` (`owner`)
- `profile::tests::it_should_reject_the_invalid_status_fixture` (`status`)
- `profile::tests::it_should_reject_an_unapproved_tagged_related_artifact`
  (`semantic-links.related-artifacts`; replaces a message-prefix assertion)
- `profile::tests::it_should_reject_an_invalid_skill_name` (`semantic-links.skill-links`; replaces a
  message-prefix assertion)
- `tests::it_should_reject_malformed_yaml` (no field path)
- `tests::it_should_reject_a_scalar_semantic_links_value` (`semantic-links`)
- `tests::it_should_reject_invalid_semantic_links_from_external_metadata`
  (`metadata.semantic-links`)

Prose-first comparison:

- **Serialized names.**
  - Arrange: each row pairs one enum value with its contract name, written as an independent
    literal rather than derived from the enum.
  - Act: serialize the value exactly as the command will.
  - Assert: the rendered name equals the contract name.

  The table rows state the causal input and expected result directly. The loop body stays at one
  abstraction level, and the assertion message names the failing variant. No prose was retained
  beyond the section labels.
- **Field paths.** Each rejection test already isolated one offending field in its fixture or
  inline Markdown. The field path is part of the same observable diagnostic identity as the
  category, so a single tuple assertion specifies one result rather than two unrelated behaviors.
  The expected path is a literal that matches the fixture's offending key; it is not derived from
  production code. Replacing the two message-prefix assertions with structured field paths removes
  coupling to human-readable message text.

Mutation evidence (stable Rust toolchain): changing the serde scalar-failure path back to
`Diagnostic::new` (no field path) made `it_should_reject_the_wrong_scalar_fixture` fail (1 failed,
46 passed); restoring the change made all 47 library tests pass.

Maintainer review follow-up: both serialization tests were first written as a `for` loop over a
case table. They are now `rstest` parameterized tests, following the `write-unit-test` skill's
Phase 3. Each named `#[case]` is a separate test, so a failure names the variant and one bad row
cannot hide the others. The Arrange comment was dropped because the cases are the Arrange.
`rstest` 0.27.0 is the current release and was already in the lockfile.

Neither the loop nor per-case literals caught a category added later without a test. The expected
category names therefore live in a test-local `contract_name` function with an exhaustive `match`
and no wildcard arm: a new category fails to compile until it has a contract name, and the case
list sits beside it. The names remain literals, independent of serde and production code.

## T1b - Reject Issue References Truncated by Unquoted YAML Comments

Test added:

- `profile::tests::it_should_reject_an_unquoted_issue_reference_that_yaml_truncates_to_issue`

Tests changed:

- `profile::tests::it_should_accept_all_provisional_related_artifact_forms` and the two accepted
  fixtures now quote their issue reference. They exercise the `issue #<n>` form for the first time.

Prose-first comparison:

- Arrange: a strict issue that is valid except for one `related-artifacts` entry, written as the
  unquoted `issue #2264` an author would type. The one causal difference is the missing quotes.
  The Arrange comment keeps the irreducible reason (YAML reads the space-prefixed `#2264` as a
  comment), because
  the inline Markdown alone does not reveal it.
- Act: the production strict-profile validation, called directly.
- Assert: one `(category, field_path)` identity. The expected values are literals, not derived
  from production code.

The test name states both the defect shape and the outcome. The full inline Markdown matches the
style of the neighboring reference-syntax tests, so no scenario fixture was introduced for a
single-field difference.

Red/green and mutation evidence: `manual-verification-evidence.md` section B1.
