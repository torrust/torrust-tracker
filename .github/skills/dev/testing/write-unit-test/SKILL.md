.github/skills/dev/testing/write-unit-test/SKILL.md---
name: write-unit-test
description: Guide for writing unit tests following project conventions including behavior-driven naming (it*should*\*), AAA pattern, MockClock for deterministic time testing, and parameterized tests with rstest. Use when adding tests for domain entities, value objects, utilities, or tracker logic. Triggers on "write unit test", "add test", "test coverage", "unit testing", or "add unit tests".
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
cargo test -p tracker-core

# Run specific test by name
cargo test it_should_return_error_when_info_hash_is_invalid

# Run tests in a module
cargo test info_hash::tests

# Run with output
cargo test -- --nocapture
```

## Phase 2: Deterministic Time with MockClock

The `clock` workspace package provides a `MockClock` for deterministic time testing.
Never use `std::time::SystemTime::now()` or `chrono::Utc::now()` directly in production code
that needs testing.

### Inject the Clock Dependency

```rust
use torrust_tracker_clock::clock::Clock;
use std::sync::Arc;

pub struct PeerList {
    clock: Arc<dyn Clock>,
}

impl PeerList {
    pub fn new(clock: Arc<dyn Clock>) -> Self {
        Self { clock }
    }

    pub fn is_peer_expired(&self, last_seen: i64, ttl: u32) -> bool {
        let now = self.clock.now();
        now - last_seen > i64::from(ttl)
    }
}
```

### Use MockClock in Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use torrust_tracker_clock::clock::stopped::Stopped as MockClock;
    use std::sync::Arc;

    #[test]
    fn it_should_mark_peer_as_expired_when_ttl_has_elapsed() {
        // Arrange
        let fixed_time = 1_700_000_100i64; // specific Unix timestamp
        let clock = Arc::new(MockClock::new(fixed_time));
        let list = PeerList::new(clock);
        let last_seen = 1_700_000_000i64;
        let ttl = 60u32;

        // Act
        let expired = list.is_peer_expired(last_seen, ttl);

        // Assert
        assert!(expired);
    }
}
```

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

## Quick Checklist

- [ ] Test name uses `it_should_` prefix
- [ ] Test follows AAA pattern with comments (`// Arrange`, `// Act`, `// Assert`)
- [ ] No `std::time::SystemTime::now()` in production code — inject `Clock` instead
- [ ] No shared mutable state between tests
- [ ] `cargo test -p <package>` passes
