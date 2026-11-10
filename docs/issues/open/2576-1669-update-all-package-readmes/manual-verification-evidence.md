---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2576-1669-update-all-package-readmes/ISSUE.md
last-updated-utc: 2026-11-10 17:30
---

# Manual Verification Evidence

## Environment and Prerequisites

- Date (UTC): 2026-11-10
- Artifact under test: the commits of the #2576 implementation branch (README template,
  `documentation` removal, one `docs(<package>)` commit per crate).
- Environment: Linux, stable Rust toolchain, local checkout.

## Baseline (T2)

After #2482 closed (2026-11-10) `cargo metadata --no-deps --format-version 1` shows the same
tiers as the 2026-10-20 check: 22 publishable packages (tier U 18 plus tier P 4), 3 tier N packages
with `publish = false`, and the developer tools. Before the change: every member inherited
`documentation = https://docs.rs/crate/torrust-tracker/`, 7 crates declared 6 or 7 keywords, 2
crates set categories and 3 crates declared `LGPL-3.0`.

## V1 - Metadata check (AC2, AC3)

Command, from the repository root (the spec's command, with `license` added):

```sh
cargo metadata --no-deps --format-version 1 | jq -r '.packages[] | select(.publish == null)
  | [.name, (.keywords | length), (.categories | length), (.documentation // ""), .license] | @tsv'
```

Result: 22 rows; every row has 4 or 5 keywords, 1 or 2 categories, an empty `documentation`,
and a description. The three LGPL crates show `LGPL-3.0-only`.
`grep -n '^documentation' Cargo.toml packages/*/Cargo.toml console/*/Cargo.toml` printed nothing.
Every category slug of those rows was found in the list from
`https://crates.io/api/v1/category_slugs` (104 slugs).

## V2 - Packaging (M2)

`cargo package -p <crate> --list --allow-dirty` for all 22 tier U and P crates lists `README.md`
(22 of 22). `grep -n '](\.\./' packages/*/README.md console/*/README.md` finds no relative link
that leaves the package; links to files outside the package are absolute
`https://github.com/torrust/torrust-tracker/...` URLs. `linter all` (including the lychee local
link check) passed in every commit hook.

## V3 - Re-audit (M3, AC5)

See the "Re-audit 2026-11-10" section of
[`readme-audit.md`](../1669-overhaul-packages/readme-audit.md): no tier U or P package is rated
stub or minimal.

## V4 - Builds and doctests (AC8)

`cargo build --workspace` passed after the `documentation` removal and `cargo test --doc
--workspace` passed (the README of no crate has a Rust code block, so no README doctest hook was
added; every code fence in a README is tagged `json`, `sh`, `text` or `console`).

## Not verifiable before the first publish

Each tier U crate's `https://docs.rs/<crate>` link matches the crate name but resolves only after
its first publish; the publish tracking issue (#2583) checks it.
