# Mutation Testing Plan for the `metrics` Package

## Overview

Mutation testing systematically introduces small code changes ("mutants") and verifies that
the test suite detects each one. A mutant that is **not caught** ("survived") reveals either a
gap in the tests or dead/redundant production code.

This plan applies [`cargo-mutants`](https://mutants.rs/) to `torrust-tracker-metrics` and
defines a workflow for triaging, fixing, and tracking survived mutants.

## Tool

```sh
# Install (already available in this repo)
cargo install cargo-mutants

# Verify version
cargo mutants --version   # 27.0.0 at time of writing
```

## Baseline

Run **before** writing any new tests so that every subsequent run can be compared against it.

```sh
# Full run — all 276 mutants, single job (safe baseline)
cargo mutants --package torrust-tracker-metrics

# Faster run — 8 parallel workers (requires enough CPU cores)
cargo mutants --package torrust-tracker-metrics --jobs 8

# List every mutant without running tests (dry-run)
cargo mutants --list --package torrust-tracker-metrics
```

Mutant counts per file (baseline from `cargo mutants --list`, commit `b8a131de`):

| File                                   | Mutants |
| -------------------------------------- | ------: |
| `metric/mod.rs`                        |      37 |
| `metric_collection/prometheus.rs`      |      35 |
| `sample.rs`                            |      26 |
| `label/set.rs`                         |      26 |
| `sample_collection.rs`                 |      19 |
| `metric_collection/mod.rs`             |      19 |
| `gauge.rs`                             |      18 |
| `metric_collection/kind_collection.rs` |      16 |
| `counter.rs`                           |      14 |
| `metric_collection/aggregate/sum.rs`   |      12 |
| `metric_collection/aggregate/avg.rs`   |      12 |
| `metric/name.rs`                       |      11 |
| `label/name.rs`                        |       9 |
| `metric/aggregate/avg.rs`              |       6 |
| `metric_collection/serde.rs`           |       4 |
| `label/value.rs`                       |       4 |
| `prometheus.rs`                        |       2 |
| `metric/description.rs`                |       2 |
| `metric/aggregate/sum.rs`              |       2 |
| `label/pair.rs`                        |       2 |
| **Total**                              | **276** |

## Priority Order

Tackle files in descending mutant count, focusing on files where the domain logic is
most critical for correctness. Three tiers:

### Tier 1 — highest value (domain logic, error paths, protocol parsing)

| File                                   | Mutants | Rationale                                            |
| -------------------------------------- | ------: | ---------------------------------------------------- |
| `metric_collection/prometheus.rs`      |      35 | Deserialization; error branches still partially grey |
| `metric_collection/mod.rs`             |      19 | Core merge/collision logic                           |
| `metric_collection/aggregate/sum.rs`   |      12 | Aggregation arithmetic                               |
| `metric_collection/aggregate/avg.rs`   |      12 | Aggregation arithmetic                               |
| `metric_collection/kind_collection.rs` |      16 | Duplicate-name detection                             |

### Tier 2 — value types and primitive operations

| File                   | Mutants | Rationale                      |
| ---------------------- | ------: | ------------------------------ |
| `counter.rs`           |      14 | Arithmetic mutations (±, ×)    |
| `gauge.rs`             |      18 | Arithmetic mutations           |
| `sample.rs`            |      26 | Core data wrapper              |
| `sample_collection.rs` |      19 | Storage and iteration          |
| `label/set.rs`         |      26 | Label matching used everywhere |

### Tier 3 — supporting types (lower risk)

| File                         | Mutants |
| ---------------------------- | ------: |
| `metric/mod.rs`              |      37 |
| `metric/name.rs`             |      11 |
| `label/name.rs`              |       9 |
| `metric_collection/serde.rs` |       4 |
| everything else              |      12 |

## Running Mutation Tests

### Scoped to a single file

```sh
cargo mutants --package torrust-tracker-metrics \
  --file packages/metrics/src/metric_collection/prometheus.rs
```

### Scoped to a single function

```sh
cargo mutants --package torrust-tracker-metrics \
  --file packages/metrics/src/metric_collection/prometheus.rs \
  --re "counter_value_from_prom"
```

### With a timeout per mutant (avoid hangs)

```sh
cargo mutants --package torrust-tracker-metrics --timeout 30
```

### Output

`cargo mutants` writes results to `mutants.out/`:

```text
mutants.out/
  outcome.json        # machine-readable results
  missed.txt          # survived mutants
  caught.txt          # caught mutants
  unviable.txt        # mutants that didn't compile
  timeout.txt         # mutants that timed out
```

Inspect survivors:

```sh
cat mutants.out/missed.txt
```

## Triage Workflow

For each survived mutant, apply one of:

| Outcome                         | Action                                                                                                                                                              |
| ------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Write a test**                | The mutant reveals a real gap. Add a targeted unit test that catches it.                                                                                            |
| **Mark `#[mutants::skip]`**     | The mutant is logically equivalent (e.g., `0 == 0` both ways) or tests the surviving variant indirectly through a higher-level test in another crate. Document why. |
| **Unreachable production code** | The mutant reveals dead code. Consider removing the branch or restructuring.                                                                                        |

### Adding `#[mutants::skip]`

Use sparingly. Always include a comment explaining the skip:

```rust
// The alternative return value is observationally equivalent from the public API
// because callers only check `is_some()`, not the concrete value.
#[mutants::skip]
fn helper_returning_option() -> Option<Foo> { … }
```

Add `mutants` to `[dev-dependencies]` if not already present:

```toml
# packages/metrics/Cargo.toml
[dev-dependencies]
mutants = "0.0.3"   # provides the #[mutants::skip] attribute
```

## Progress

Update this table after completing each task. Columns:

- **Mutants** — total mutants from `cargo mutants --list` for that file
- **Caught** — killed by the test suite after the task
- **Survived** — still alive after the task (target: 0)
- **Skipped** — annotated `#[mutants::skip]` (with documented reason)
- **Status** — `[ ]` not started · `[~]` in progress · `[x]` done

| Status | Task      | File(s)                              | Mutants | Caught | Survived | Skipped |
| :----: | --------- | ------------------------------------ | ------: | -----: | -------: | ------: |
| `[x]`  | 1         | `metric_collection/prometheus.rs`    |      35 |     24 |        0 |       0 |
| `[x]`  | 2         | `metric_collection/mod.rs`           |      19 |      2 |        0 |       0 |
| `[x]`  | 3         | `counter.rs` + `gauge.rs`            |      32 |     20 |        0 |       0 |
| `[x]`  | 4         | `sample_collection.rs` + `sample.rs` |      45 |     12 |        0 |       0 |
| `[ ]`  | 5         | `label/set.rs`                       |      26 |      — |        — |       — |
| `[ ]`  | 6         | all remaining files                  |     119 |      — |        — |       — |
| **—**  | **Total** |                                      | **276** |  **—** |    **—** |   **—** |

> Replace `—` with actual numbers as each task is completed. The goal is **Survived = 0**
> across the board (or every non-zero entry in Skipped has a documented reason in the
> relevant source file).

---

## Tasks

Work through tiers in order. For each file:

1. **Run** `cargo mutants --package torrust-tracker-metrics --file <path>`.
2. **Inspect** `mutants.out/missed.txt`.
3. **Triage** each survivor (test gap / equivalent / dead code).
4. **Act** (write test, add skip, or remove dead code).
5. **Re-run** to confirm the survivor is caught.
6. **Commit** test additions with `test(metrics): kill <N> surviving mutants in <file>`.

### Task 1 — `metric_collection/prometheus.rs` (35 mutants)

Key survivors to expect based on current grey lines:

- `counter_value_from_prom`: the `Unknown(_)` arm and the catch-all `other` arm both return
  `Err(...)` — a mutation replacing one error variant with another may survive if no test
  asserts the exact variant.
- `gauge_value_from_prom`: same issue.
- `parse_prometheus_timestamp`: the nanosecond overflow carry (`nanos - 1_000_000_000`) — a
  mutation changing `-` to `+` should be caught by `it_should_handle_nanosecond_boundary_overflow`,
  but verify.
- `build_metric_collection`: the `?` propagation — a mutation that replaces `Ok(())` with the
  body of the function. The `it_should_classify_duplicate_metric_names_as_collection_errors` test
  covers this but confirm.

### Task 2 — `metric_collection/mod.rs` (19 mutants)

Key candidates:

- `check_cross_type_collision` → replace with `Ok(())`: caught only if a test asserts that a
  counter and gauge with the same name produce an error.
- `merge` → replace with `Ok(())`: caught only if a test checks the state _after_ merging.
- `collect_names` → replace with empty set: caught only if `check_cross_type_collision` is
  called and the test checks the error.

### Task 3 — `counter.rs` / `gauge.rs` arithmetic (14 + 18 mutants)

Examples:

- `Counter::increment` `+=` → `-=`: caught by any test that increments then reads the value.
- `Gauge::decrement` `-=` → `+=`: same.
- `From<i32> for Counter` → `Default::default()`: caught only if a test uses a non-zero i32.

### Task 4 — `sample_collection.rs` + `sample.rs` (19 + 26 mutants)

Examples:

- `SampleCollection::new` → early-return `Ok(empty)`: caught only if tests verify contents
  after construction.
- `Sample::new` field assignments: caught by accessor tests.

### Task 5 — `label/set.rs` (26 mutants)

Label matching is load-bearing for every metric lookup. Pay attention to:

- `LabelSet::matches` boolean logic mutations (`&&` → `||`, etc.).
- `try_from` error-path mutations.

### Task 6 — Remaining Tier 2 / Tier 3 files

Apply the same triage workflow to all remaining files.

## Acceptance Criteria

- **Zero unaddressed survivors**: Every survived mutant is either covered by a new test or
  annotated with `#[mutants::skip]` with a documented reason.
- **All existing tests still pass**: `cargo test -p torrust-tracker-metrics` exits `0`.
- **`linter all` passes**: No new clippy or formatting warnings introduced.
- **Coverage does not regress**: `cargo llvm-cov --package torrust-tracker-metrics --summary-only`
  shows no decrease from the post-coverage-plan baseline.

## Configuration (optional)

`cargo-mutants` can be configured in `Cargo.toml` or `.cargo/mutants.toml`:

```toml
# Cargo.toml (workspace root)
[workspace.metadata.cargo-mutants]
# Skip files that are intentionally not mutation-tested
exclude_globs = [
    # Generated code or trivial impls
    "packages/metrics/src/lib.rs",
]
# Default timeout per mutant in seconds
timeout_multiplier = 2.0
```

## References

- [`cargo-mutants` documentation](https://mutants.rs/)
- [`mutants` crate (`#[mutants::skip]`)](https://docs.rs/mutants/latest/mutants/)
- [Mutation Testing — general theory](https://en.wikipedia.org/wiki/Mutation_testing)
- llvm-cov baseline: `docs/issues/1582-add-prometheus-deserialization-metrics/increase-unit-test-coverage.md`
