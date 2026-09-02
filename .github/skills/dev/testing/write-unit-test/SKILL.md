---
name: write-unit-test
description: Guide for writing unit tests following project conventions including behavior-driven naming (it*should*\*), AAA pattern, MockClock for deterministic time testing, and parameterized tests with rstest. Use when adding tests for domain entities, value objects, utilities, or tracker logic. Triggers on "write unit test", "add test", "test coverage", "unit testing", or "add unit tests".
semantic-links:
  related-artifacts:
    - docs/testing/README.md
    - docs/testing/refactoring-patterns/README.md
metadata:
  author: torrust
  version: "1.0"
---

# Writing Unit Tests

## Core Principles

Unit tests in this project are written against the **Test Desiderata** — the 12 properties that
make tests valuable, defined by Kent Beck. Not every property applies equally to every test, but
treat them as the standard to reason about and optimize for.

| Property                  | What it means                                                                       |
| ------------------------- | ----------------------------------------------------------------------------------- |
| **Isolated**              | Tests return the same result regardless of run order. No shared mutable state.      |
| **Composable**            | Different dimensions of variability can be tested separately and results combined.  |
| **Deterministic**         | Same inputs always produce the same result. No randomness, no wall-clock time.      |
| **Fast**                  | Tests run in milliseconds. Unit tests must never block on I/O or sleep.             |
| **Writable**              | Writing the test should cost much less than writing the code it covers.             |
| **Readable**              | A reader can understand what behaviour is being tested and why, without context.    |
| **Behavioral**            | Tests are sensitive to changes in observable behaviour, not internal structure.     |
| **Structure-insensitive** | Refactoring the implementation should not break tests that test the same behaviour. |
| **Automated**             | Tests run without human intervention (`cargo test`).                                |
| **Specific**              | When a test fails, the cause is immediately obvious from the failure message.       |
| **Predictive**            | Passing tests give genuine confidence the code is ready for production.             |
| **Inspiring**             | Passing the full suite inspires confidence to ship.                                 |

Some properties support each other (automation makes tests faster). Some trade off against each
other (more predictive tests tend to be slower). Use composability to resolve apparent conflicts.

Reference: <https://testdesiderata.com/> and Kent Beck's original papers on
[Test Desiderata](https://medium.com/@kentbeck_7670/test-desiderata-94150638a4b3) and
[Programmer Test Principles](https://medium.com/@kentbeck_7670/programmer-test-principles-d01c064d7934).

## Coverage and Test-Gap Policy

The repository prefers high maintainable automated coverage.

Practical priority order:

1. Unit tests first (fast, deterministic, low maintenance)
2. Integration tests where unit tests are insufficient
3. End-to-end tests for cross-process/system validation

When behaviour is left untested, document why explicitly in one or more of:

- code comments near the boundary/constraint,
- issue spec notes,
- PR description.

Acceptable reasons to defer or avoid direct unit tests include:

- behaviour depends on out-of-process services not controlled by the test,
- deterministic unit tests would be disproportionately brittle,
- validation is better covered by integration/E2E tests with clear evidence.

If a feature is hard to test, treat that as design feedback first and improve testability when
practical.

### Lifecycle Fixture Design Review

When a test fixture manages a child process, asynchronous I/O, network
readiness, or failure cleanup, define its lifecycle design before implementing
the main scenario:

1. Identify the narrow test-facing interface and the owner of each resource.
2. Keep passive infrastructure, such as output draining, separate from
   domain-specific interpretation such as readiness rules.
3. State resource lifetime through normal shutdown and panic/drop cleanup;
   retain diagnostic output after reader tasks complete where possible.
4. Use one absolute deadline that bounds all awaited readiness work, including
   connection attempts, response decoding, child-exit checks, and retry delays.
5. After the first passing vertical slice, review whether the responsibilities
   remain coherent. Record material lessons in the issue completion review;
   do not create generic abstractions without a demonstrated need.

### Project-specific conventions

- **Behavior-driven naming** — test names document what the code does
- **AAA Pattern** — Arrange → Act → Assert (clear structure)
- **Deterministic** — use `MockClock` instead of real time (see Phase 2)
- **Isolated** — no shared mutable state between tests
- **Fast** — unit tests run in milliseconds

## Phase 1: Basic Unit Test

### Naming Convention

**Format**: `it_should_{expected_behavior}_when_{condition}`

- Always use the `it_should_` prefix
- Never use the `test_` prefix
- Use `when_` or `given_` for conditions
- Be specific and descriptive

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_return_error_when_info_hash_is_invalid() {
        // Arrange
        let invalid_hash = "not-a-valid-hash";

        // Act
        let result = InfoHash::from_str(invalid_hash);

        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn it_should_parse_valid_info_hash() {
        // Arrange
        let valid_hex = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

        // Act
        let result = InfoHash::from_str(valid_hex);

        // Assert
        assert!(result.is_ok());
    }
}
```

### Running Tests

```bash
# Run all tests in a package
cargo test -p bittorrent-tracker-core

# Run specific test by name
cargo test it_should_return_error_when_info_hash_is_invalid

# Run tests in a module
cargo test info_hash::tests

# Run with output
cargo test -- --nocapture
```

## Phase 2: Deterministic Time with `clock::Stopped`

The `clock` workspace package provides `clock::Stopped` for deterministic time testing.
Never call `std::time::SystemTime::now()` or `chrono::Utc::now()` directly in production code
that needs testing. Instead, use the type-level clock abstraction.

### Use the Type-Level Clock Alias

Copy the following boilerplate into each crate that needs a clock. The `CurrentClock` alias
automatically selects `Working` in production and `Stopped` in tests:

```rust
/// Working version, for production.
#[cfg(not(test))]
pub(crate) type CurrentClock = torrust_clock::clock::Working;

/// Stopped version, for testing.
#[cfg(test)]
pub(crate) type CurrentClock = torrust_clock::clock::Stopped;
```

In production code, obtain the current time via the `Time` trait:

```rust
use torrust_clock::clock::Time as _;

pub fn is_peer_expired(last_seen: std::time::Duration, ttl: u32) -> bool {
    let now = CurrentClock::now(); // returns DurationSinceUnixEpoch (= std::time::Duration)
    now.saturating_sub(last_seen) > std::time::Duration::from_secs(u64::from(ttl))
}
```

### Control Time in Tests

Use `clock::Stopped::local_set` to pin the clock to a specific instant. The stopped clock is
thread-local, so tests are isolated from each other by default.

```rust
#[cfg(test)]
mod tests {
    use std::time::Duration;

    use torrust_clock::clock::{stopped::Stopped as _, Time as _};
    use torrust_clock::clock::Stopped;

    use super::*;

    #[test]
    fn it_should_mark_peer_as_expired_when_ttl_has_elapsed() {
        // Arrange — pin the clock to a known instant
        let fixed_time = Duration::from_secs(1_700_000_100);
        Stopped::local_set(&fixed_time);

        let last_seen = Duration::from_secs(1_700_000_000);
        let ttl = 60u32;

        // Act
        let expired = is_peer_expired(last_seen, ttl);

        // Assert
        assert!(expired);

        // Clean up — reset to zero so other tests start from a clean state
        Stopped::local_reset();
    }
}
```

> **Key points**
>
> - `Stopped::now()` defaults to `Duration::ZERO` at the start of each test thread.
> - `Stopped::local_set(&duration)` sets the current time for the calling thread only.
> - `Stopped::local_reset()` resets back to `Duration::ZERO`.
> - `Stopped::local_add(&duration)` advances the clock by the given amount.
> - Import the `Stopped` trait (`use …::stopped::Stopped as _`) to bring its methods into scope.

## Phase 3: Parameterized Tests with rstest

Use `rstest` for multiple input/output combinations to avoid repetition.

```toml
[dev-dependencies]
rstest = { workspace = true }
```

```rust
use rstest::rstest;

#[rstest]
#[case("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", true)]
#[case("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA", true)]
#[case("not-a-hash", false)]
#[case("", false)]
fn it_should_validate_info_hash(#[case] input: &str, #[case] is_valid: bool) {
    let result = InfoHash::from_str(input);
    assert_eq!(result.is_ok(), is_valid, "input: {input}");
}
```

## Phase 4: Test Helpers

The `test-helpers` workspace package provides shared test utilities.

```toml
[dev-dependencies]
torrust-tracker-test-helpers = { workspace = true }
```

Check the package for available mock servers, fixture generators, and utility types.

## Reusable Refactoring Patterns

Before adding a bespoke helper or refactoring generated test code, consult the
[test refactoring-pattern catalog](../../../../../docs/testing/refactoring-patterns/README.md).
It contains repository-native examples with their problem, selected pattern, and constraints.
Prefer an existing catalog pattern when it fits. Add a focused entry when a reviewed refactor
establishes a reusable pattern for future tests.

## Quick Checklist

- [ ] Test name uses `it_should_` prefix
- [ ] Test follows AAA pattern with comments (`// Arrange`, `// Act`, `// Assert`)
- [ ] No `std::time::SystemTime::now()` in production code — use the `CurrentClock` type alias instead
- [ ] No shared mutable state between tests
- [ ] Behaviour coverage is maximized with maintainable tests
- [ ] Any intentional test gaps are explicitly documented with rationale
- [ ] `cargo test -p <package>` passes
