<!-- markdownlint-disable MD041 -->
<!--
Template for the README.md of a workspace package (crates.io and docs.rs show it).
Copy it to `<package directory>/README.md`, replace every `<placeholder>`, delete the optional
sections that do not apply and delete these comments. See the quality bar in
docs/issues/open/2576-1669-update-all-package-readmes/ISSUE.md.

Rules:
- Only the package directory is published, so link files outside it with absolute
  https://github.com/torrust/torrust-tracker/... URLs.
- A Rust code block is allowed only when the crate root includes this README in a
  `#[cfg(doctest)]` item (`include_str!("../README.md")`), so `cargo test --doc` compiles it.
  Tag every other fence (`sh`, `toml`, `text`) so rustdoc does not run it.
- Keep the description short: do not copy `docs/packages.md`, link to it.
-->

# <Title derived from the crate name, for example "Torrust Tracker UDP Core">

`<crate-name>` is <one paragraph: what the crate does, what it does not do, and who should depend
on it. A crate that exists only to build the tracker says so and points to the tracker.>

## Where it fits

<Layer (`*-protocol`, `*-core`, server, `axum-*`, client, domain), its main workspace dependencies
and dependents in one or two sentences.> See the
[package catalog](https://github.com/torrust/torrust-tracker/blob/develop/docs/packages.md#package-catalog)
for the full picture.

## Main API

<The few public types, traits or modules a user starts from, or a link to docs.rs when the list
would only repeat it.>

## Usage

<Optional. A short example for a crate meant for reuse outside the tracker. A Rust example needs
the `#[cfg(doctest)]` hook described above.>

## Stability

<Accurate statement for the version, for example: "Versions are independent (see the
[ADR](https://github.com/torrust/torrust-tracker/blob/develop/docs/adrs/20260629000000_adopt_independent_package_versioning.md)).
While the version is `0.x`, the public API can change in any minor release. The crate is
developed for the Torrust Tracker first.">

## Documentation

- [API documentation](https://docs.rs/<crate-name>)
- <Links to package docs that ship with it: ADRs, benchmarking notes.>

## License

<Matches the `license` field of `Cargo.toml` and links a text of that license.>

- AGPL crates: "The project is licensed under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
  (`AGPL-3.0-only`)", linking the `LICENSE` file that sits in the package directory.
- LGPL crates: `LGPL-3.0-only`, linking <https://www.gnu.org/licenses/lgpl-3.0.html>.
