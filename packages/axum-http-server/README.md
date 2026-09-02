# Torrust Axum HTTP Tracker

The Torrust Bittorrent HTTP tracker.

## Documentation

[Crate documentation](https://docs.rs/torrust-tracker-axum-http-server).

## Testing and Coverage

This crate belongs to the Torrust Tracker Cargo workspace. Run its tests from the repository root:

```text
cargo test -p torrust-tracker-axum-http-server
```

Install [`cargo-llvm-cov`](https://github.com/taiki-e/cargo-llvm-cov) to measure coverage. From
the repository root, run:

```text
cargo llvm-cov -p torrust-tracker-axum-http-server --all-features --summary-only
```

When working from this package directory, use its manifest explicitly:

```text
cargo llvm-cov --manifest-path Cargo.toml --all-features --summary-only
```

Use `--json` instead of `--summary-only` when you need per-file, function, and region detail.
Aggregate percentages are only a navigation aid: prioritize behavior risk and per-file gaps when
choosing tests. For Issue #1348's current measurement method and coverage interpretation, see
[`coverage-evidence.md`](../../docs/issues/open/1348-1347-add-tests-axum-http-server/coverage-evidence.md).

## License

The project is licensed under the terms of the [GNU AFFERO GENERAL PUBLIC LICENSE](./LICENSE).
