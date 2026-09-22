# Generated Schemas

## Frontmatter V1

`frontmatter-v1.schema.json` is the JSON Schema Draft 2020-12 projection of the
canonical strict v1 Rust model in
`contrib/dev-tools/checks/frontmatter-validator/src/profile.rs`. Do not edit the
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

The schema covers strict issue and EPIC field presence, JSON types, nullability,
enumerations, numeric bounds, string patterns, mapping shape, and approved
`x-` experimental fields. The Rust validator remains authoritative for Markdown
delimiter extraction, YAML parsing and scalar lexemes, exact double-quote
requirements, calendar-valid timestamps, strict-profile dispatch, and all
repository-aware checks such as path existence, lifecycle/location consistency,
skill discovery, and review-finding resolution.
