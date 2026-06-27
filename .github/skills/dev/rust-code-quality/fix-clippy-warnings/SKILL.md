---
name: fix-clippy-warnings
description: Guide for fixing Rust Clippy warnings in the torrust-tracker project. Covers proper application of clippy suggestions, when to add allowances, and how to document exceptions. Use when asked to fix clippy warnings, improve code quality, or resolve linter issues. Triggers on "fix clippy", "clippy warnings", "rust code quality", or "linting issues".
metadata:
  author: torrust
  version: "1.0"
---

# Fix Clippy Warnings

This skill guides you through the proper handling of Rust Clippy warnings in the Torrust Tracker project.

## Clippy Philosophy

**Always prefer fixing clippy warnings with the suggested approach** rather than adding `#[allow(...)]` attributes. Clippy warnings are designed to improve code quality, readability, and maintainability.

## When to Apply Clippy Suggestions

### ✅ Apply Suggested Fixes

When clippy suggests a specific code change that improves quality:

- Use `as_chunks::<N>()` instead of `chunks_exact(N)` (as we did in SI-4)
- Use `#[allow(clippy::explicit_iter_loop)]` instead of `iter()` when it's more concise
- Apply any other suggestion that improves code quality

### ⚠️ When to Add Allowances

Only add `#[allow(...)]` when:

1. The suggestion is **not applicable** to the specific use case
2. The suggestion would **break existing functionality** or API
3. The suggestion is **temporarily ignored** during a refactoring phase
4. The suggestion is **not yet supported** in the current Rust version

## How to Document Exceptions

When adding `#[allow(...)]` attributes, always include a clear comment explaining why:

```rust
// This is a temporary workaround during refactoring of the announce response parser
// TODO: Remove this allowance when the parser is fully refactored
#[allow(clippy::unnecessary_wraps)]
fn parse_announce_response(data: &[u8]) -> Result<Response, ParseError> {
    // implementation
}
```

## Common Clippy Patterns

### Pattern 1: `chunks_exact` → `as_chunks`

**Before:**

```rust
for chunk in bytes.chunks_exact(6) {
    // process 6-byte chunks
}
```

**After:**

```rust
let (chunks, remainder) = bytes.as_chunks::<6>();
if !remainder.is_empty() {
    return Err(ParseError::InvalidChunkSize);
}
for chunk in chunks.iter() {
    // process 6-byte chunks
}
```

### Pattern 2: Explicit Iterator Loop

**Before:**

```rust
for item in items.iter() {
    // process item
}
```

**After:**

```rust
for item in &items {
    // process item
}
```

## Clippy Workflow

1. **Identify the warning**: Run `linter clippy` to see specific clippy errors
2. **Apply suggestion**: Try the suggested fix first
3. **Verify functionality**: Ensure the change doesn't break existing behavior
4. **Document exceptions**: Add clear comments for any allowances
5. **Run full linters**: Confirm `linter all` passes

## Related Skills

- [`run-linters`](../git-workflow/run-linters/SKILL.md) - Run all code quality checks
- [`commit-changes`](../git-workflow/commit-changes/SKILL.md) - Commit changes with proper conventions
