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

## T4 - Location-Dependent Severity and Warnings

Placement (approved R1): the policy lives in the library's `repository` module as pure functions
over a repository-relative path and the document text. It owns:

- `Location::of`;
- `ownership`, moved from the binary together with its test;
- `validate_document`, which returns every diagnostic for one document (R2).

The binary now only discovers, reads, renders, and sets the exit code.

Tests added:

- `repository::tests` (unit):
  - location classification, including `docs/issues/README.md` and a similar `opened/` prefix;
  - ownership dispatch;
  - `legacy-shape` errors for draft and open primary specs, with or without frontmatter, and for
    files that declare `doc-type` `issue` or `epic`;
  - no v1 requirement for closed specs, other locations, supporting files, or strict v1 records;
  - strict-profile severity by location;
  - syntax errors stay errors in closed specs;
  - one `experimental-field` warning per `x-` field, after the structural error, in source order;
  - externally governed files are exempt.
- `tests/cli.rs`: `it_should_exit_one_only_when_a_record_is_an_error`. A closed-spec warning
  exits `0`, while an open-spec error and an open legacy spec exit `1`. The exit-code rule is
  owned by the binary, so it is tested at the command boundary.

Prose-first comparison:

- The path is the causal Arrange value: each `#[case]` pairs a repository-relative path with a
  named content constant. Constants such as `LEGACY_ISSUE`, `EVIDENCE`, and `PLAIN` name the
  document shape, so each case reads as "this kind of document at this location".
- The Act is the production `validate_document` or `Location::of` call.
- The Assert compares an ordered list of `(category, severity)` pairs. The experimental-field
  test compares `(category, severity, field_path)` triples. Expected values are literals.

Mutation evidence (stable Rust toolchain; restored from a `.tmp` backup, verified with `cmp`):

| Mutation | Tests failed |
| -------- | ------------ |
| Closed specs keep error severity | 1 |
| Primary-spec rule disabled | 3 |
| `legacy-shape` applies in every location | 2 |
| `x-` findings reported as errors | 1 |
| External files not exempted from `legacy-shape` | 1, after strengthening the test |

The first run of the last mutation survived: the external-file test's content had no
`doc-type: issue`, so removing the exemption changed nothing. The test now gives the Agent Skill
an external `doc-type: issue`, which is the case the exemption exists for. Two other first
attempts, the primary-spec and every-location mutations, did not compile under `-D warnings`.
They were redone as mutants that compile.

## T5 - Repository-Aware Checks

Design: D7 lookups use a pure library value, `RepositoryFiles`, built from repository-relative
file paths. The binary fills it with index entries for `--staged`, and otherwise with tracked files
present in the working tree. The library derives skill names from `.github/skills/**/<name>/SKILL.md`
entries. Every D7 rule is therefore unit-tested without git or a trait object. Only the binary's
choice of snapshot is tested at the command boundary. D7 applies only inside the issue lifecycle
folders: drafts and open get all four checks as errors, closed specs get only the status and
`spec-path` checks as warnings, and other locations get none.

Tests added:

- `repository::tests` (unit):
  - a consistent open spec passes;
  - status versus lifecycle folder, 7 cases across drafts, open, and closed;
  - `spec-path` mismatch, as an error when open and a warning when closed;
  - related-artifact resolution: a tracked file, a directory with tracked files, a missing file,
    a sibling sharing a name prefix, and syntax-only issue and review-finding references;
  - skill resolution: a nested skill, a top-level skill, a `SKILL.md` outside `.github/skills/`,
    and a skill folder without `SKILL.md`;
  - stale references ignored in closed and other locations;
  - warnings ordered before repository findings.
- `tests/cli.rs`: `it_should_resolve_related_artifacts_against_the_index_only_with_staged`. A file
  in the index but deleted from the working tree resolves in `--staged` mode and is missing in
  path mode.

Prose-first comparison:

- A `strict_issue(spec_path, status, links)` builder names the only three inputs D7 reads. Each
  test states its causal value visibly: the status, the moved `spec-path`, one related artifact, or
  one tracked file.
- `repository(&[...])` lists the repository snapshot literally, in the test body.
- Expected findings are literal `(category, severity)` lists. Optional expectations are written as
  `Option`/`bool` cases rather than branches in the test body.

The first draft of the command-boundary test used an untracked working-tree file. That cannot
distinguish the modes, because D7 requires tracked files in both. It was replaced by a tracked
file deleted from disk, the state that actually separates index from working tree.

Mutation evidence (stable Rust toolchain; each mutation verified as applied, restored from a
`.tmp` backup, and checked with `cmp`):

| Mutation | Tests failed |
| -------- | ------------ |
| Open specs accept `done` | 1 |
| Directory match without the trailing `/` | 1 (shared-prefix sibling) |
| Closed specs resolve references | 1 |
| Skills recognized outside `.github/skills/` | 1 |
| `--staged` resolves against the working tree | 1 (command boundary) |
| `spec-path` never checked | 2 |

Whole-repository run (`--all`, read-only, 0.04 s, empty stdout):

- 50 `legacy-shape` errors, matching D10/D11's 42 issues plus 8 EPICs;
- 14 structural errors in documents outside the v1 profiles;
- 3 closed-spec warnings;
- no D7 findings in the five open v1 specs.

The structural errors are T6 input.
