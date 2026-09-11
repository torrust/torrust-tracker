---
doc-type: manual-verification-evidence
issue-spec: docs/issues/closed/2151-add-tracker-config-path-argument/ISSUE.md
last-updated-utc: 2026-09-11
---

# Manual Verification Evidence

## Environment and Prerequisites

- Date and time (UTC): 2026-09-08 17:15-17:28.
- Artifact under test: `target/release/torrust-tracker`, built from the current
  `2151-add-tracker-config-path-argument` branch.
- Operating system / environment: Linux; repository root as current working
  directory.
- Setup: Each process used a copy of
  `share/default/config/tracker.development.sqlite3.toml` with loopback,
  port-zero tracker/API bindings and an isolated SQLite file under
  `.tmp/issue-2151-manual/`. The manual runs invoked the release binary
  directly, rather than a test runner or disposable script.

## Verification Processes

### V1 - M1: CLI Path Only

- Goal: Confirm that a release tracker can start from an explicit configuration
  file without configuration-source environment variables.
- Initial state: Isolated valid configuration at
  `.tmp/issue-2151-manual/m1-clean/tracker.toml`; no
  `TORRUST_TRACKER_CONFIG_TOML` or `TORRUST_TRACKER_CONFIG_TOML_PATH`.
- Status: `DONE`

#### Steps Performed

1. Started the release binary directly:

   ```sh
   env -u TORRUST_TRACKER_CONFIG_TOML -u TORRUST_TRACKER_CONFIG_TOML_PATH \
     target/release/torrust-tracker -c "$PWD/.tmp/issue-2151-manual/m1-clean/tracker.toml" \
     > .tmp/issue-2151-manual/m1-clean/tracker.log 2>&1 &
   printf '%s\n' "$!" > .tmp/issue-2151-manual/m1-clean/tracker.pid
   ```

2. Called `http://127.0.0.1:1313/health_check`, sent SIGTERM to the PID in
   `tracker.pid`, and waited for the process to exit.

#### Observed Result

```text
M1 exit status: 0
HEALTH CHECK API: Started on: http://127.0.0.1:1313
HTTP/1.1 200 OK
Torrust tracker successfully shutdown.
```

#### Conclusion

The release binary read the explicit file, served the health endpoint, and
shut down cleanly. M1 passed.

### V2 - M2: CLI Source Precedence

- Goal: Confirm that the CLI file wins over complete-TOML and path environment
  base sources.
- Initial state: Three valid configurations specified health ports `43152`
  (CLI), `43153` (environment path), and `43154` (environment TOML content).
- Status: `DONE`

#### Steps Performed

1. Started the release binary with all three base sources:

   ```sh
   TORRUST_TRACKER_CONFIG_TOML="$(< .tmp/issue-2151-manual/m2/env-content.toml)" \
   TORRUST_TRACKER_CONFIG_TOML_PATH="$PWD/.tmp/issue-2151-manual/m2/env-path.toml" \
     target/release/torrust-tracker \
       --config-toml-path "$PWD/.tmp/issue-2151-manual/m2/cli.toml" \
       > .tmp/issue-2151-manual/m2/tracker.log 2>&1 &
   printf '%s\n' "$!" > .tmp/issue-2151-manual/m2/tracker.pid
   ```

2. Called `http://127.0.0.1:43152/health_check`, then sent SIGTERM and waited
   for the process.

#### Observed Result

```text
M2 exit status: 0
HEALTH CHECK API: Started on: http://127.0.0.1:43152
HTTP/1.1 200 OK
Torrust tracker successfully shutdown.
```

The `43153` and `43154` environment-source ports were not selected.

#### Conclusion

The CLI-selected file was the exclusive base source. M2 passed.

### V3 - M3: Per-Value Override

- Goal: Confirm that per-value overrides remain higher priority than a
  CLI-selected base file.
- Initial state: The CLI file set the health endpoint to `43155`; the override
  set it to `43156`.
- Status: `DONE`

#### Steps Performed

1. Started the release binary directly:

   ```sh
   TORRUST_TRACKER_CONFIG_OVERRIDE_HEALTH_CHECK_API__BIND_ADDRESS=127.0.0.1:43156 \
   env -u TORRUST_TRACKER_CONFIG_TOML -u TORRUST_TRACKER_CONFIG_TOML_PATH \
     target/release/torrust-tracker \
       --config-toml-path "$PWD/.tmp/issue-2151-manual/m3/tracker.toml" \
       > .tmp/issue-2151-manual/m3/tracker.log 2>&1 &
   printf '%s\n' "$!" > .tmp/issue-2151-manual/m3/tracker.pid
   ```

2. Called `http://127.0.0.1:43156/health_check`, then sent SIGTERM and waited
   for the process.

#### Observed Result

```text
M3 exit status: 0
HEALTH CHECK API: Started on: http://127.0.0.1:43156
HTTP/1.1 200 OK
Torrust tracker successfully shutdown.
```

The CLI file specified `43155`, which was not the active health endpoint.

#### Conclusion

The override applied over the CLI base file. M3 passed.

### V4 - M4: Invalid CLI Source

- Goal: Confirm user-visible failure behavior for invalid argument and source
  states, without starting a listener.
- Initial state: Isolated missing path, directory, mode-`000` valid TOML file,
  malformed TOML file, and a `tracker.toml` file only in the parent of the
  invocation directory.
- Status: `DONE`

#### Steps Performed

1. Ran the release binary directly with each of the following inputs, capturing
   stderr and the exit status in `.tmp/issue-2151-manual/m4/`:

   ```sh
   target/release/torrust-tracker --config-toml-path
   target/release/torrust-tracker --config-toml-path ''
   target/release/torrust-tracker --config-toml-path "$PWD/.tmp/issue-2151-manual/m4/missing.toml"
   target/release/torrust-tracker --config-toml-path "$PWD/.tmp/issue-2151-manual/m4"
   target/release/torrust-tracker --config-toml-path "$PWD/.tmp/issue-2151-manual/m4/unreadable.toml"
   target/release/torrust-tracker --config-toml-path "$PWD/.tmp/issue-2151-manual/m4/malformed.toml"
   (cd .tmp/issue-2151-manual/m4/parent/child && \
     "$OLDPWD/target/release/torrust-tracker" --config-toml-path tracker.toml)
   ```

2. Restored the unreadable file mode to `600` and checked that no scenario
   health ports `43152` through `43156` had a listening TCP socket.

#### Observed Result

```text
missing-value exit=2
error: a value is required for '--config-toml-path <CONFIG_TOML_PATH>' but none was supplied

empty-value exit=2
error: invalid value '' for '--config-toml-path <CONFIG_TOML_PATH>': configuration TOML path must not be empty

missing-file exit=1
Unable to load explicit configuration file `.../m4/missing.toml`: No such file or directory (os error 2)

directory exit=1
Unable to load explicit configuration file `.../m4`: path is not a regular file

unreadable exit=1
Unable to load explicit configuration file `.../m4/unreadable.toml`: Permission denied (os error 13)

malformed exit=1
Unable to process explicit configuration file `.../m4/malformed.toml`: Missing mandatory configuration option

parent-only-relative exit=1
Unable to load explicit configuration file `tracker.toml`: No such file or directory (os error 2)

ss -ltn | rg ':4315[2-6]\b'
# no output
```

#### Conclusion

The parser failures exited `2`; all invalid-source failures exited `1`, named
the supplied source, and did not start a scenario listener. The mode-`000`
regular-file behavior was enforced by this Linux environment. M4 passed.

### V5 - M5: Parallel Child Isolation (Unix)

- Goal: Start two release binaries concurrently with different explicit files,
  port-zero bindings, and isolated SQLite paths.
- Initial state: `first.toml` and `second.toml` specify separate databases
  under `.tmp/issue-2151-manual/m5/` and `health_check_api.bind_address` as
  `127.0.0.1:0`.
- Status: `DONE`

#### Steps Performed

1. Started the two release binaries directly with `first.toml` and
   `second.toml`, recording PIDs `476822` and `476823`.
2. Normalized terminal color escapes from the logs, extracted the
   `Started health check API` service bindings, and called both
   `/health_check` endpoints.
3. Sent SIGTERM to both PIDs and waited for both processes to exit.

#### Observed Result

```text
first_url=http://127.0.0.1:41693 second_url=http://127.0.0.1:38835
first_health=0 second_health=0 first_exit=0 second_exit=0

first health response: {"status":"Ok", ...}
second health response: {"status":"Ok", ...}

first:  HEALTH CHECK API: Started health check API ... service_binding=http://127.0.0.1:41693/
second: HEALTH CHECK API: Started health check API ... service_binding=http://127.0.0.1:38835/
first:  Torrust tracker successfully shutdown.
second: Torrust tracker successfully shutdown.
```

#### Conclusion

Both release processes used their own explicit source, dynamically assigned
health endpoint, and isolated storage. Both endpoint calls succeeded and both
processes shut down cleanly. M5 passed.

## Failures and Follow-up

- V5's initial endpoint-discovery command assumed no terminal color escape
  sequences between `HEALTH` and `CHECK`; therefore, it did not exercise the
  endpoints. The completed rerun normalized those escapes and used the stable
  service-binding message. This was a verification-command defect, not a
  product failure.
