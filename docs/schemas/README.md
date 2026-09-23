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
