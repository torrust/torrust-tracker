# Torrust Axum Server

A wrapper for the Axum server for Torrust HTTP servers to add timeouts.

## Documentation

[Crate documentation](https://docs.rs/torrust-tracker-axum-server).

## Notes

This package is currently scoped under the `torrust-tracker-` prefix because `tsl.rs`
depends on two tracker-specific items:

- `TslConfig` from `torrust-tracker-configuration` — a small two-field struct (SSL
  certificate and key paths). It has no inherent tracker dependency and could be moved
  to a generic package.
- `LocatedError` / `DynError` from `torrust-tracker-located-error` — planned to be
  renamed to `torrust-located-error` (a generic package) under
  EPIC [#1669](https://github.com/torrust/torrust-tracker/issues/1669) SI-10.

Once `TslConfig` is extracted to a generic location and `torrust-tracker-located-error`
is renamed, this package could become a generic `torrust-axum-server` reusable across
the Torrust organisation. A near-identical module already exists in
[torrust-index](https://github.com/torrust/torrust-index/blob/develop/src/web/api/server/custom_axum.rs),
which confirms the generic utility of this pattern. This reorganization is tracked in
EPIC [#1669](https://github.com/torrust/torrust-tracker/issues/1669).

## License

The project is licensed under the terms of the [GNU AFFERO GENERAL PUBLIC LICENSE](./LICENSE).
