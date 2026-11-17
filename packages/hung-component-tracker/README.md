# Torrust Tracker Hung-Component Tracker

A test-only tracker executable that verifies the shutdown deadline abort at the executable boundary.

The `hung-component-tracker` binary starts the real tracker with the same library functions as the
shipped `torrust-tracker` executable, then adds one component, `hung_component`, that never completes
and ignores its cancellation token. On `SIGINT` or `SIGTERM` it runs the shared shutdown sequence
(`app::run_until_shutdown_signal`), so the supervisor must abort the hung component, and the health
check API that waits for it, at the process shutdown deadline and exit with code 1.

The crate is not published and is not a default workspace member, so it never enters the container
image or its test archives. CI runs its tests with the rest of the workspace.

## Tests

```sh
cargo test -p torrust-tracker-hung-component-tracker
```

The integration test in `tests/hung_component_abort.rs` runs the binary once per signal with a 4 s
deadline and checks the exit code, the exit time, and the logged aborted components. It is Unix-only.

## Manual run

```sh
cargo run -p torrust-tracker-hung-component-tracker -- --config-toml-path <tracker.toml>
```

Use a configuration with a short `[application.shutdown]` deadline and drain budgets that pass
validation, for example `deadline_in_secs = 4` and the three drain budgets at `1`, then send
`SIGTERM` or `SIGINT` to the process.
