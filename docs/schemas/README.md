# Generated Schemas

## Frontmatter V1

`frontmatter-v1.schema.json` is the JSON Schema Draft 2020-12 projection of the
canonical strict v1 Rust model in
`contrib/dev-tools/checks/frontmatter-validator/src/profile.rs`, using the frozen
value-syntax patterns declared beside their predicates in
`contrib/dev-tools/checks/frontmatter-validator/src/syntax.rs`. Do not edit the
generated JSON directly.

Regenerate the artifact from the repository root with:

```sh
cargo run --offline --package frontmatter-validator --bin frontmatter-schema -- generate
```

Verify that the tracked artifact has no drift without modifying it with:

```sh
cargo run --offline --package frontmatter-validator --bin frontmatter-schema -- check
```

For a disposable copy, pass `--artifact <path>` after the action. For example:

```sh
cargo run --offline --package frontmatter-validator --bin frontmatter-schema -- check --artifact .tmp/frontmatter-v1.schema.json
```

The command exits `0` on success, `1` when it cannot read or write the artifact
or the artifact has drifted, and `2` for invalid arguments, following
`docs/adrs/20260519000000_define_global_cli_output_contract.md`.

The schema covers strict issue and EPIC field presence, JSON types, nullability,
enumerations, numeric bounds, string patterns, mapping shape, and approved
`x-` experimental fields. Each profile's `required` array is generated from the
same field list the validator enforces, so nullable fields such as `epic` and
`related-pr` are required to be present and may be `null`.

The Rust validator remains authoritative for Markdown
delimiter extraction, YAML parsing and scalar lexemes, exact double-quote
requirements, calendar-valid timestamps, strict-profile dispatch, and all
repository-aware checks such as path existence, lifecycle/location consistency,
skill discovery, and review-finding resolution. It is also stricter than the
schema in one place the schema cannot express without a parallel model:
`semantic-links.skill-links` and `semantic-links.related-artifacts` may be
omitted but may not be an explicit `null`, while the schema types them as
`array` or `null`.

## Agent Reply V1

`agent-reply-v1.schema.json` is the JSON Schema Draft 2020-12 projection of the reply model in
`contrib/dev-tools/checks/agent-reply-validator/src/reply.rs`: the content of one reply file that
an AI agent writes in the `agent-replies/` folder of an issue specification (decision D18 of
issue #2904). Do not edit the generated JSON directly.

Regenerate the artifact with:

```sh
cargo run --package agent-reply-validator --bin agent-reply-schema
```

The test `contrib/dev-tools/checks/agent-reply-validator/tests/schema.rs` fails when the tracked
artifact differs from the model, so `cargo test`, the pre-push hook and CI catch a stale schema.

Check reply files with:

```sh
cargo run --package agent-reply-validator --bin agent-reply-validator -- --all
cargo run --package agent-reply-validator --bin agent-reply-validator -- <reply-file>...
```

`--all` checks the staged content of every file in an `agent-replies/` folder or named like a
reply file; the pre-commit hook and the Docs Lint workflow run it. With paths, it checks those
files on disk. Besides the schema, the validator checks the file's place
(`docs/issues/<drafts|open|closed>/<specification>/agent-replies/`), its name
(`<agent>-<subject>-r<round>.json`), and that the name agrees with the `agent`, `subject` and
`round` fields. It also differs from the schema where the schema cannot follow it. It is
stricter on a real calendar date and time in `created-utc`, fractional numbers, repeated keys,
and a subject number beyond 32 bits. For blank text it uses Rust's Unicode white space, not the
ECMA-262 set of `\S`: a text of only U+0085 passes the schema and fails the validator, and a text
of only U+FEFF fails the schema and passes the validator. The module documentation of
`src/schema.rs` lists these.

### The hooks of the task reviewer

`agent-reply-guard`, the second binary of the crate, is the program of the two hooks of the
`task-reviewer` agent (decision D17 of issue #2904): a stop hook, and a hook before the tool that
hands the agent's report back, because Claude Code discards a stop hook's refusal when the agent
ends its turn with that tool. The agent declares both in its frontmatter:

```yaml
hooks:
  Stop:
    - hooks:
        - type: command
          command: cargo run --quiet --manifest-path "$CLAUDE_PROJECT_DIR/Cargo.toml" --package agent-reply-validator --bin agent-reply-guard
  PreToolUse:
    - matcher: SubagentHandback
      hooks:
        - type: command
          command: cargo run --quiet --manifest-path "$CLAUDE_PROJECT_DIR/Cargo.toml" --package agent-reply-validator --bin agent-reply-guard
```

Claude Code runs it when the agent tries to finish and when it calls the hand-back tool, with the
hook input as JSON on standard input. The first input names the agent's own transcript; the second
names only the session's, and the program finds the agent's from it (see `src/transcript.rs`).
The program reads the reply file that the agent's latest request names from the agent's transcript,
checks it with the validator above, and answers with its exit code: `0` lets the agent finish
(standard output is empty, or one JSON object whose `systemMessage` says why the reply was not
enforced or that the limit was reached), `2` keeps the agent working with the problems on standard
error, and `1` is a failure of the program itself, which never holds the agent. It refuses at most
three times for one request, that is for one run of the agent and one reply file, counting both hooks together, and keeps its
count in `.agents/issues/<issue-number>/reply-guard/`, a folder that Git ignores and the linters
exclude. The rules are in `src/guard.rs`, the format of the transcript it relies on in
`src/transcript.rs`, and the tests run with the crate's others: `cargo test -p agent-reply-validator`.
