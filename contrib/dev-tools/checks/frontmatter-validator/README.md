# frontmatter-validator

Non-published crate for the repository's v1 Markdown frontmatter contract. It holds the canonical
Rust model and two binaries:

- `frontmatter-validator`: a read-only check, described below.
- `frontmatter-schema`: generates and checks `docs/schemas/frontmatter-v1.schema.json`. See
  [`docs/schemas/README.md`](../../../../docs/schemas/README.md).

The contract and its decisions are specified in
[issue #2266](../../../../docs/issues/closed/2266-2264-implement-rust-frontmatter-model-and-validator/ISSUE.md)
and [issue #2281](../../../../docs/issues/open/2281-2264-frontmatter-validator-command/ISSUE.md).

## Running the Validator

Exactly one mode is required:

```sh
# Explicit files, and tracked Markdown under directories
cargo run --package frontmatter-validator --bin frontmatter-validator -- docs/issues/open

# The staged content of staged Markdown files (used by the pre-commit hook)
cargo run --package frontmatter-validator --bin frontmatter-validator -- --staged

# Every tracked Markdown file (manual)
cargo run --package frontmatter-validator --bin frontmatter-validator -- --all
```

Output follows the `no-stdout-result` class of the
[CLI output contract ADR](../../../../docs/adrs/20260519000000_define_global_cli_output_contract.md):

- stdout stays empty;
- stderr carries one JSON record per line, of kind `diagnostic`, `usage_error`, `runtime_error`,
  or `help`;
- the exit code is `0` with no errors (warnings allowed), `1` for any error or runtime failure,
  and `2` for invalid invocation.

`--all` currently exits `1`, because legacy draft and open specs are migrated one at a time as
they are edited.

Severity depends on where a document lives:

- Malformed frontmatter and an invalid `semantic-links` envelope are errors everywhere.
- Draft and open issue specs are strict.
- Closed specs get advisory warnings for profile and repository findings.
- Strict v1 records outside `docs/issues/` report structural errors but no lifecycle checks.
- `docs/templates/` and this crate's `fixtures/` are never validated.

## Migrating a Legacy Spec to V1

A staged edit to a draft or open `ISSUE.md` or `EPIC.md` without v1 frontmatter fails with
`legacy-shape`. Migrate it in the same commit:

1. Copy the frontmatter shape from
   [`docs/templates/ISSUE.md`](../../../../docs/templates/ISSUE.md) or
   [`docs/templates/EPIC.md`](../../../../docs/templates/EPIC.md). Add `schema-version: 1`, and
   fill every field the template lists. Use `null` for unknown nullable numbers such as `epic`,
   `github-issue`, or `related-pr`.
2. Use a lifecycle `status` that fits the folder:
   - `draft` in `drafts/`;
   - `planned`, `in-progress`, `blocked`, or `in-review` in `open/`;
   - `done` in `closed/`.

   Replace other values, such as `open`, with the one that matches the issue's real state.
3. Remove fields the profile does not define, such as `issue-type` or `priority` on an EPIC.
   Prefix a field with `x-` only to keep it as an experimental extension; that produces a
   warning.
4. Write `last-updated-utc` as a double-quoted `"YYYY-MM-DD HH:MM"` UTC value.
5. In `semantic-links.related-artifacts`:
   - quote issue references as `"issue #<n>"`, because an unquoted `#` after a space starts a
     YAML comment;
   - drop trailing slashes from directory paths.

   Only `skill-links` and `related-artifacts` are allowed under `semantic-links`.
6. Repair stale references. Every repository path in `related-artifacts` must name a tracked file
   or directory, and every `skill-links` name must match a `.github/skills/**/<name>/SKILL.md`.
   Specs moved to `docs/issues/closed/` are the usual cause.
7. Set `spec-path` to the file's own repository path.

Re-run the validator on the file until it exits `0`.

## Temporary Placement

This crate is approved early work under EPIC #2003, which owns the long-term automation
architecture. The validator follows the `clippy-allow-reasons` precedent: a workspace crate that
`contrib/dev-tools/git/hooks/pre-commit.sh` runs through `cargo run --package`.

The code is split so that relocation stays cheap:

- **Library policy (`profile`, `repository`):** pure functions over a repository path, document
  text, and a `RepositoryFiles` snapshot. It moves unchanged into whichever package #2003
  selects.
- **Binary (`src/bin/frontmatter-validator/`):** argument parsing, discovery, git access, and
  NDJSON rendering. This is the adapter a future shared runner would replace.
- **Tests:** unit tests cover the policy. `tests/cli.rs` covers the command boundary in
  disposable git repositories.

The crate is excluded from the container test archives because those tests need `git`.
