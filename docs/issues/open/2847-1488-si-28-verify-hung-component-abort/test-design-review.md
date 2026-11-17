# Test Design Review

Prose-first Arrange-Act-Assert review of the T2/T3 integration test
`packages/hung-component-tracker/tests/hung_component_abort.rs`, as required by the spec's Commit
Points. One increment: the shared assertion function and its two signal cases.

## Hung Component Aborted At The Deadline (SIGTERM, SIGINT)

### Arrange (prose)

A tracker that is the shipped one except for one extra component, `hung_component`, which never
completes and ignores its cancellation token. The process deadline `P` is 4 s, with drain budgets of
1 s that pass validation. The tracker is running and its signal handlers are installed.

### Act (prose)

Deliver the signal (`SIGTERM` or `SIGINT`) to the exact child PID and wait, bounded, for it to exit.

### Assert (prose)

The tracker handled the signal; it exits with code 1; not before `P`, and within `P` plus a 3 s
margin; the supervisor logs both `hung_component` and `health_check_api` as aborted after the
deadline; the final error line names exactly those two in `incomplete_components`.

### Review

- **One causal initial-state difference.** The only difference from a clean shutdown (covered by
  `tests/lifecycle/shutdown_policy.rs`, exit 0) is `hung_component`, added by the binary, not by the
  test. The Arrange comment names it. The budgets are a named constant with the validation arithmetic
  in its doc comment; they are context, not a second cause.
- **Fixture owns only incidental mechanics.** `tests/common/mod.rs` writes the configuration (health
  check API on port 0, database in a `TempDir`), clears inherited `TORRUST_TRACKER_CONFIG*`
  variables, spawns the binary from `CARGO_BIN_EXE_hung-component-tracker`, drains output on two
  threads, waits for the readiness marker with a 30 s bound, delivers the signal, waits for the exit
  with a 30 s bound, and kills and reaps the child on timeout and in `Drop` (a failed assertion
  included). It makes no assertion about the shutdown result.
- **Act and expected result stay visible.** The Act is one call, `signal_and_wait_for_exit(signal,
  EXIT_TIMEOUT)`. Every expected value (code 1, `DEADLINE`, `EXIT_MARGIN`, the two component names)
  is written in the test file, independent of the production code; the log messages are matched as
  constants, and `job=<name>` as a whole word so `hung_component` cannot match a longer name.
- **Helper boundaries.** `assert_aborted_after_the_deadline` and `incomplete_components` are named
  assertions at the test's level of abstraction; the latter only parses the final line, the
  comparison stays in the test. The shared `assert_the_supervisor_aborts_the_hung_component_at_the_deadline`
  keeps the two signal cases identical except for the signal.
- **Timing.** The elapsed time is measured from just before `kill` to the observed exit, with a
  10 ms poll. The lower bound `>= P` holds because the supervisor starts its deadline after it
  receives the signal. The 3 s margin absorbs a loaded runner; the abort log lines, not the margin,
  prove the health check API was aborted rather than drained.
- **Mutation check.** With `hung_component` returning `Completed` instead of never completing, both
  cases fail on the exit-code assertion (exit status 0). Restored afterwards.
- **Retained prose.** Only the Arrange comment naming the causal difference and the one-line reason
  `health_check_api` is aborted remain; the rest of the prose is expressed by names and constants.
