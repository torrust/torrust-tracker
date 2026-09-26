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

## T2 - Command, Explicit-Path Mode, and the D9 Record Catalog

Tests added in `src/bin/frontmatter-validator.rs` (14 test cases):

- `it_should_exit_zero_without_records_when_every_document_is_valid` (strict v1 issue, no
  frontmatter)
- `it_should_report_one_diagnostic_record_and_exit_one_for_an_invalid_document`
- `it_should_emit_a_null_field_path_when_the_failure_concerns_no_single_field`
- `it_should_order_records_by_path_regardless_of_argument_order`
- `it_should_validate_top_level_semantic_links_only_for_repository_owned_file_names` (`SKILL.md`,
  `*.agent.md`, ordinary file)
- `it_should_report_a_single_usage_error_record_and_exit_two` (no arguments, `--version`,
  nonexistent path)
- `it_should_render_help_as_a_single_help_record_and_exit_zero`
- `it_should_report_a_runtime_error_record_and_exit_one_when_a_file_cannot_be_read`
- `it_should_exit_one_when_stderr_cannot_be_written`

Prose-first comparison:

- **Arrange.** Each document is written into a `TempDir`, which owns and removes it. The causal
  content is the only variable:
  - an accepted or rejected crate fixture, included as text rather than by path, so the D6
    fixture exclusion added in T3 cannot change these tests;
  - one inline malformed document;
  - identical content under different file names for ownership dispatch;
  - a non-UTF-8 byte pair for the read failure.

  Usage-error and help tests need no Arrange.
- **Act.** `validator(&[...])` calls the production `run` with the program name prepended and an
  in-memory stderr. That is one named action at the argument level, and it hides only the NDJSON
  line parsing.
- **Assert.** Each test pins the observable contract it owns:
  - the exit code;
  - the record's ordered key list, which is the D9 field order;
  - the values that identify the record.

  Human-readable `message` texts are not pinned, because D9 declares them illustrative. The key
  list and the identifying values are two assertions about one record's shape. Merging them into
  one tuple would lose the readable key-order failure.

Mutation evidence (stable Rust toolchain; each mutation restored from a `.tmp` backup, verified
byte-identical with `cmp`):

| Mutation | Tests failed |
| -------- | ------------ |
| Ownership dispatch always returns `Repository` | 2 (`agent_skill`, `agent_profile` cases) |
| Remove `paths.sort()` | 1 (`it_should_order_records_by_path_regardless_of_argument_order`) |
| Omit `field_path` when `None` | 1 (`it_should_emit_a_null_field_path_when_the_failure_concerns_no_single_field`) |
| `--help` exits `2` | 1 (`it_should_render_help_as_a_single_help_record_and_exit_zero`) |

## T3 - Discovery, `--staged`, `--all`, and Exclusions

Boundary decision: git-dependent behavior is tested through the built binary in
`tests/cli.rs`, following the `clippy-allow-reasons/tests/cli.rs` precedent. Each test gets a
disposable `git init` repository in a `TempDir`, which removes it on drop. Every git invocation,
including the binary's, clears inherited `GIT_*` variables and sets `GIT_CEILING_DIRECTORIES` to
the system temp directory. A run inside a git hook therefore cannot read or change the real
repository. Tests that need no repository stay as unit tests in `main.rs`: mode selection, help,
stderr failure, and ownership dispatch.

T2 tests moved: the explicit-file tests from `main.rs` now run in `tests/cli.rs`. R4 makes an
explicit path outside a repository a usage error, and D9 renders repository-relative paths. Their
contracts are unchanged, apart from the path now being repository-relative.

Tests added (20 integration and 11 unit test cases in total after the move):

- mode selection usage errors: no mode, `--staged --all`, paths with `--staged`, `--version`;
- `it_should_dispatch_ownership_by_file_name`, including a directory named `SKILL.md`;
- repository-relative paths from a subdirectory working directory;
- usage errors for nonexistent and outside-repository paths;
- a runtime error with a `null` path outside any repository, and with the path for unreadable
  content;
- an explicit untracked file is validated;
- directory expansion to tracked Markdown only;
- `--all` covers tracked Markdown that still exists;
- `--staged` uses index content in both directions, validates only staged files, and ignores a
  staged deletion;
- exclusions apply in every mode.

Prose-first comparison:

- **Arrange** names repository state with one-level actions: `write`, `stage`, `commit`, `delete`.
  The causal difference is visible in each test body:
  - which file is tracked, staged, deleted, or untracked;
  - which prefix it lives under;
  - which version is staged versus in the working copy.

  Every unselected file uses the same invalid content as the selected one. A wrong selection
  therefore shows up as an extra path, not as a silent pass.
- **Act** is one `validate(&[...])` call with the command-line arguments a user types.
- **Assert** compares the ordered list of reported paths, or the exit code and record identity,
  against literals. The `Outcome` helper also asserts that stdout is empty on every run, which
  covers the D9 no-stdout rule without a separate test per mode.

Mutation evidence (stable Rust toolchain; each restored from a `.tmp` backup and verified
byte-identical with `cmp`):

| Mutation | Tests failed |
| -------- | ------------ |
| `--staged` reads the working tree instead of the index | 2 (both staged-content cases) |
| Empty exclusion list | 4 (all exclusion cases) |
| `--all` keeps tracked files deleted from the working tree | 1 |
| No `--diff-filter=ACMR` on staged discovery | 1 (staged deletion) |
| Paths outside the repository accepted | 1 (outside-repository usage error) |

The first attempt at the index mutation did not compile, because the unused index reader tripped
`-D warnings`. It was redone with the reader still referenced, and it failed as expected.
