---
doc-type: refactor-plan
status: in_progress
related-issue: 2280
spec-path: docs/issues/open/2280-2264-generate-v1-schema-and-verify-drift/refactor-plan.md
last-updated-utc: "2026-09-22 17:55"
semantic-links:
  skill-links:
    - create-refactor-plan
    - write-unit-test
    - handle-errors-in-code
  related-artifacts:
    - .github/skills/dev/planning/create-refactor-plan/SKILL.md
    - contrib/dev-tools/checks/frontmatter-validator/src/bin/frontmatter-schema.rs
    - contrib/dev-tools/checks/frontmatter-validator/src/lib.rs
    - docs/adrs/20260519000000_define_global_cli_output_contract.md
    - issue #2280
---

<!-- skill-link: create-refactor-plan -->

# Refactor Plan — Frontmatter Schema Command

## Goal

Give the `frontmatter-schema` command added by #2280 complete unit coverage and make it easier to
test, read, and extend by separating pure decisions (argument parsing, canonical encoding, drift
comparison, exit-code mapping) from filesystem and process I/O. The generated schema contract and
the tracked artifact bytes do not change.

Related artifact: `docs/issues/open/2280-2264-generate-v1-schema-and-verify-drift/ISSUE.md`

This plan is review-only. It does not authorize implementation and does not expand into the
general command surface owned by #2281.

## Current Assessment

`frontmatter-schema.rs` mixes five concerns in one module: argument parsing, default artifact
resolution, canonical JSON encoding, filesystem read/write, and process reporting. The three
existing tests protect deterministic encoding, the explicit `--artifact` path, and one drifted
artifact. The following observable behavior has no automated protection:

- successful `generate` (file written, parent directories created, exact bytes);
- successful `check` against a current artifact;
- every invalid invocation form and the default artifact location;
- filesystem failures (missing artifact, parent that cannot be created);
- the exit code returned for each failure class.

Two latent defects were also found while auditing:

- `temporary_artifact_path()` derives its name from `process::id()`, which is shared by every test
  in the binary. Any second test writing to it would race under parallel execution, and the file
  leaks when an assertion panics before `remove_file`.
- The command exits `1` for usage errors. The CLI output ADR cited by the issue reserves exit code
  `2` for usage errors and `1` for runtime failures.

## Items

### 1. [x] Replace the PID-based temporary path with `tempfile::TempDir` [HIGH impact / TRIVIAL effort]

**Problem**: `temporary_artifact_path()` in the binary's `tests` module is not unique per test and
does not clean up on panic. Every later test item would inherit that race and leak.

**Files**:

- `contrib/dev-tools/checks/frontmatter-validator/Cargo.toml`
- `contrib/dev-tools/checks/frontmatter-validator/src/bin/frontmatter-schema.rs`

**Change**: Add `tempfile` as a dev-dependency at the version already pinned in the workspace root
(`3.27.0`, also used by `package-coverage-check`). Replace the helper with a `TempDir` owned by
each test; the directory is unique and removed on drop, including on panic. Delete the manual
`remove_file` call from the drift test.

---

### 2. [x] Cover successful generation and clean check [HIGH impact / LOW effort]

**Problem**: `write_schema` and the success branch of `check_schema` are the command's primary
purpose but are only exercised by manual evidence.

**Files**:

- `contrib/dev-tools/checks/frontmatter-validator/src/bin/frontmatter-schema.rs`

**Change**: Add two tests over a `TempDir`:

1. `write_schema` to a nested path such as `<tmp>/nested/dir/schema.json` asserts the file exists
   and its bytes equal `schema_json()`, proving parent-directory creation and byte identity.
2. `check_schema` on that freshly written file returns `Ok(())`.

Keep the expected bytes visible in the test body; derive them from the same encoding function the
production Act uses so an unrelated formatting change cannot make the test fail with stale data.

---

### 3. [x] Cover argument parsing edge cases and guard the default artifact location [HIGH impact / LOW effort]

**Problem**: `artifact_path` is already pure but only its happy path is tested. The default
resolution walks `CARGO_MANIFEST_DIR.ancestors().nth(4)`; if the crate is moved, `generate` would
silently create a new `docs/schemas/` in the wrong directory instead of failing.

**Files**:

- `contrib/dev-tools/checks/frontmatter-validator/src/bin/frontmatter-schema.rs`

**Change**: Add tests asserting:

1. no arguments resolve to a path ending in `docs/schemas/frontmatter-v1.schema.json` **and** that
   path `is_file()` in the checkout, which pins the ancestor depth;
2. `["--artifact"]`, `["--other", "x"]`, and `["--artifact", "a", "b"]` each return the usage
   message.

Dispatch of an unknown action is covered in item 6 once `run` accepts explicit arguments.

---

### 4. [x] Cover filesystem failure paths with a real temporary directory [MEDIUM impact / LOW effort]

**Problem**: The `could not read` and `could not create` branches are untested. A filesystem
trait was considered and rejected: every relevant failure is reproducible deterministically on
every platform without one.

**Files**:

- `contrib/dev-tools/checks/frontmatter-validator/src/bin/frontmatter-schema.rs`

**Change**: Add tests asserting:

1. `check_schema` on `<tmp>/missing.json` returns an error containing `could not read` and the
   path;
2. `write_schema` to `<tmp>/blocker/schema.json`, where `<tmp>/blocker` is a regular file, returns
   an error containing `could not create`, because `create_dir_all` cannot replace a file.

Permission-based failures are not tested: they are platform-dependent and ignored when tests run as
root in containers.

---

### 5. [x] Distinguish usage errors (exit 2) from runtime failures (exit 1) [HIGH impact / LOW effort]

**Problem**: `main` maps every `Err` to `ExitCode::FAILURE`. The repository CLI output contract
(`docs/adrs/20260519000000_define_global_cli_output_contract.md`) assigns `2` to usage errors and
`1` to runtime failures. Shell callers and future pre-commit wiring in #2281 cannot tell a typo
from a drifted artifact.

**Files**:

- `contrib/dev-tools/checks/frontmatter-validator/Cargo.toml`
- `contrib/dev-tools/checks/frontmatter-validator/src/bin/frontmatter-schema.rs`

**Change**: Introduce a minimal `thiserror` error enum (the workspace already pins `thiserror = "2"`)
with two variants, `Usage` and `Runtime(String)`, and an `exit_code()` method. `main` writes the
`Display` text to stderr and returns that code. Unit-test the mapping directly on the enum. Item 8
refines the runtime variant into specific variants; do not do that here.

The ADR also requires NDJSON on stderr. That is a repository-wide rollout the ADR itself defers
("start simple"), so it stays out of scope; the plain-text stderr line is unchanged.

---

### 6. [x] Split `run` into a pure `Command::parse` and an executing `Command::execute` [MEDIUM impact / MEDIUM effort]

**Problem**: `run` reads `env::args()` itself, so the action/artifact decision cannot be tested
without a child process, and the unknown-action branch is unreachable from tests.

**Files**:

- `contrib/dev-tools/checks/frontmatter-validator/src/bin/frontmatter-schema.rs`

**Change**: Define

```rust
enum Command {
    Generate { artifact: PathBuf },
    Check { artifact: PathBuf },
}
```

with `Command::parse(arguments: impl Iterator<Item = String>) -> Result<Command, Error>` (pure, no
I/O) and `Command::execute(&self) -> Result<(), Error>`. `main` becomes
`Command::parse(env::args().skip(1)).and_then(|command| command.execute())`. Parsing tests then
cover `generate`, `check`, an unknown action, and a missing action without touching the filesystem.
This replaces the free `artifact_path` function; its tests from item 3 move to `Command::parse`.

Do not add a CLI framework; the surface is two actions and one optional flag.

---

### 7. [x] Move the canonical artifact encoding into the library [MEDIUM impact / LOW effort]

**Problem**: `schema_json()` (pretty JSON plus trailing newline) is the definition of "the tracked
artifact bytes", yet it lives in the binary. The library test for `v1_schema()` can only inspect a
`serde_json::Value`, and the #2281 command layer could not reuse the encoding without duplicating
it.

**Files**:

- `contrib/dev-tools/checks/frontmatter-validator/src/lib.rs` (or `src/profile.rs`)
- `contrib/dev-tools/checks/frontmatter-validator/src/bin/frontmatter-schema.rs`

**Change**: Add `pub fn v1_schema_json() -> String` beside `v1_schema()`, documented as the exact
byte encoding of the tracked artifact. Serialization of a `schemars::Schema` cannot fail in
practice, so the function is infallible; keep `expect` with a one-line justification rather than
threading a `Result` that no caller can act on. Move the determinism test to the library. The
binary calls the library function and owns only I/O.

---

### 8. [x] Encapsulate artifact I/O in a `SchemaArtifact` value object with a specific error enum [MEDIUM impact / MEDIUM effort]

**Problem**: After items 5–7 the binary still passes `&Path` through free functions and formats
error strings at the point of failure, so tests match on substrings and the process boundary
cannot reason about what failed.

**Files**:

- `contrib/dev-tools/checks/frontmatter-validator/src/bin/frontmatter-schema.rs`

**Change**: Introduce

```rust
struct SchemaArtifact { path: PathBuf }

impl SchemaArtifact {
    fn write(&self) -> Result<(), Error>;
    fn verify_current(&self) -> Result<(), Error>;
}
```

and replace `Runtime(String)` with specific variants carrying the path and `source: io::Error`:
`CreateDirectory`, `Write`, `Read`, `Drift`. Each `#[error(...)]` keeps the current what/where/how
to fix wording so the user-facing text is unchanged. Tests assert on variants with `matches!`
instead of substrings. `Command::execute` builds the artifact and calls one method.

This is the idiomatic Rust form of object-oriented design: private state, methods that preserve the
invariant, and typed outcomes. No traits, inheritance, or interfaces are introduced.

---

### 9. [x] Make the drift hint match the documented command [LOW impact / TRIVIAL effort]

**Problem**: The drift error always suggests `generate --artifact <absolute path>`, even when the
default artifact was checked. `docs/schemas/README.md` documents the default command without
`--artifact`, so the hint and the docs disagree in the common case.

**Files**:

- `contrib/dev-tools/checks/frontmatter-validator/src/bin/frontmatter-schema.rs`

**Change**: Carry whether the artifact was explicit in `Command`/`SchemaArtifact` and render the
hint as `-- generate` for the default and `-- generate --artifact <path>` otherwise. Add one test
per form.

---

### 10. [x] Do not interpolate caller-controlled paths into a shell command [HIGH impact / LOW effort]

**Problem**: `SchemaArtifact::regenerate_command` interpolates an explicit artifact path directly
into a shell command displayed on stderr. Paths with whitespace do not round-trip when pasted;
shell special characters can alter a pasted command's meaning. The command must not construct
executable shell text from caller-controlled data.

**Files**:

- `contrib/dev-tools/checks/frontmatter-validator/src/bin/frontmatter-schema.rs`
- `docs/schemas/README.md`

**Change**: Replace `Error::Drift`'s stored `regenerate_command: String` with a structured
regeneration instruction. For the tracked artifact, display the exact documented default command.
For an explicit artifact, name the path separately and direct the user to run the documented
generator with `--artifact <path>`; do not render a shell command that incorporates the
path. Add behavior tests using paths with whitespace and shell special characters.

---

### 11. [x] Replace the tracked artifact atomically [HIGH impact / MEDIUM effort]

**Problem**: `SchemaArtifact::write` calls `fs::write` directly on the destination. It truncates
the existing generated artifact before the full replacement bytes are durable. An interrupted
generation or storage failure can leave a corrupt tracked schema.

**Files**:

- `contrib/dev-tools/checks/frontmatter-validator/Cargo.toml`
- `contrib/dev-tools/checks/frontmatter-validator/src/bin/frontmatter-schema.rs`

**Change**: Write canonical bytes to a temporary file in the destination directory, flush it, then
replace the target with a same-directory rename using a platform-aware operation. Define and test
the symlink policy first: direct write follows a symlink, while rename normally replaces it.
Preserve the selected policy deliberately. Test successful replacement preserves exact canonical
bytes and leaves no temporary artifact. Record any platform limitation that prevents a fully
atomic overwrite.

**Decision**: Stage the artifact with `tempfile::NamedTempFile` in the destination directory,
call `sync_all` on the staged file, and replace with `persist`. The operation atomically replaces
the destination on supported same-filesystem platforms and intentionally replaces a destination
symlink rather than following it. `tempfile` does not synchronize the containing directory, so the
command does not promise crash-durable directory metadata on every platform.

---

### 12. [x] Model artifact origin instead of carrying an `explicit` boolean [MEDIUM impact / LOW effort]

**Problem**: `SchemaArtifact` represents selection origin with `path: PathBuf` plus `explicit:
bool`, while `Error::Drift` independently stores `artifact: PathBuf` plus a derived command
string. Both pairs permit inconsistent states, such as a tracked path with `explicit: true` or a
drift path paired with a regeneration command for another path.

**Files**:

- `contrib/dev-tools/checks/frontmatter-validator/src/bin/frontmatter-schema.rs`

**Change**: Replace the boolean with `ArtifactOrigin::Tracked` / `ArtifactOrigin::Explicit`. Let
`SchemaArtifact` own its path and origin, and let drift retain the artifact or a narrow immutable
regeneration instruction instead of a preformatted string. Render messages at the error-display
boundary. Add unit tests for both origins and their valid instruction forms.

---

### 13. [ ] Make process reporting unit-testable without spawning the binary [MEDIUM impact / LOW effort]

**Problem**: `main` is the only owner of stderr reporting, but the test suite asserts only error
values and exit codes independently. A future edit could emit the wrong prefix, write a success
diagnostic, or ignore a writer failure without a focused test observing it.

**Files**:

- `contrib/dev-tools/checks/frontmatter-validator/src/bin/frontmatter-schema.rs`

**Change**: Extract a narrow process adapter that accepts parsed arguments and `&mut impl Write`,
returns an `ExitCode`, and contains the single stderr formatting decision. Keep `main` as the line
that locks real stderr and delegates. Test success emits no stderr, usage emits one exact line and
code `2`, runtime failure emits one exact line and code `1`, and a failing writer is handled
deliberately. Do not add a generic framework or change the ADR-owned NDJSON rollout.

---

## Order of Execution

| Order | Status | Item                                                              | Impact | Effort  |
| ----- | ------ | ----------------------------------------------------------------- | ------ | ------- |
| 1     | [x]    | Replace the PID-based temporary path with `tempfile::TempDir`     | High   | Trivial |
| 2     | [x]    | Cover successful generation and clean check                       | High   | Low     |
| 3     | [x]    | Cover argument parsing edge cases and guard the default location  | High   | Low     |
| 4     | [x]    | Cover filesystem failure paths with a real temporary directory    | Medium | Low     |
| 5     | [x]    | Distinguish usage errors (exit 2) from runtime failures (exit 1)  | High   | Low     |
| 6     | [x]    | Split `run` into pure `Command::parse` and `Command::execute`     | Medium | Medium  |
| 7     | [x]    | Move the canonical artifact encoding into the library             | Medium | Low     |
| 8     | [x]    | Encapsulate artifact I/O in `SchemaArtifact` with a typed error   | Medium | Medium  |
| 9     | [x]    | Make the drift hint match the documented command                  | Low    | Trivial |
| 10    | [x]    | Keep caller-controlled paths out of shell command text            | High   | Low     |
| 11    | [x]    | Replace the tracked artifact atomically                            | High   | Medium  |
| 12    | [x]    | Model artifact origin instead of carrying an `explicit` boolean   | Medium | Low     |
| 13    | [ ]    | Make process reporting unit-testable without spawning the binary  | Medium | Low     |

Item 4 is placed before item 5 despite its lower impact because it completes the test baseline
that item 5 changes behavior against. Item 7 is placed after item 6 because it changes the same
call sites that item 6 restructures; landing it first would be rewritten immediately. Item 10 must
precede item 12 because the origin model should encode the structured instruction selected for
safe diagnostics. Item 11 is independent and may land before item 12. Item 13 should land last so
it tests the final diagnostic shape.

## Test Design Rules

- Record a prose-first Arrange/Act/Assert review in `test-design-review.md` before each item.
- Every test owns its `TempDir`; never touch `docs/schemas/frontmatter-v1.schema.json`.
- One failure cause per test, so a red test names its cause without reading the code.
- After each item: `cargo test --package frontmatter-validator`,
  `cargo run --offline --package frontmatter-validator --bin frontmatter-schema -- check`,
  `cargo +nightly fmt --all -- --check`, `cargo clippy --package frontmatter-validator -- -D warnings`,
  `cargo machete --with-metadata`, `linter all`.
- Items 5–9 change production code; run the mutation check from the `write-unit-test` skill for
  each new test (reintroduce the old behavior, confirm the test fails, restore).
- Items 10–13 change production code; use the same mutation-check workflow for each behavior
  whose regression test should distinguish the previous implementation.

## Non-Goals and Rejected Alternatives

- No change to the generated schema or the tracked artifact bytes.
- No file discovery, `--staged`, NDJSON stderr, pre-commit integration, or CLI framework (#2281).
- No filesystem trait or test double: every failure path is reproducible with a real `TempDir`.
- No `anyhow`; the repository error-handling skill prefers explicit enums.

## Review Decision

The maintainer approved the plan on 2026-09-22 and chose to implement every item inside the #2280
branch, one signed commit per item, before opening the pull request. Items 1–9 are complete; each
commit carries its prose-first design review in `test-design-review.md`.

A deeper post-implementation review identified items 10–13. They are pending maintainer review;
do not implement them until approved.
