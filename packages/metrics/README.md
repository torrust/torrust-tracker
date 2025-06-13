# Torrust Tracker Metrics

A library with the metrics types used by the [Torrust Tracker](https://github.com/torrust/torrust-tracker) packages.

## Documentation

[Crate documentation](https://docs.rs/torrust-tracker-metrics).

## Testing

Run coverage report:

```console
cargo llvm-cov --package torrust-tracker-metrics 
```

Generate LCOV report with `llvm-cov` (for Visual Studio Code extension):

```console
mkdir -p ./.coverage
cargo llvm-cov --package torrust-tracker-metrics  --lcov --output-path=./.coverage/lcov.info
```

Generate HTML report with `llvm-cov`:

```console
mkdir -p ./.coverage
cargo llvm-cov --package torrust-tracker-metrics  --html --output-dir ./.coverage
```

## Acknowledgements

We copied some parts like units or function names and signatures from the crate [metrics](https://crates.io/crates/metrics) because we wanted to make it compatible as much as possible with it. In the future, we may consider using the `metrics` crate directly instead of maintaining our own version.

## License

The project is licensed under the terms of the [GNU AFFERO GENERAL PUBLIC LICENSE](./LICENSE).
