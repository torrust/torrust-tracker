# Verification Evidence — Legacy Shutdown API Deprecation

> **Status**: Not started — do not begin before the eligibility gate is met.

## Environment

- Date:
- OS:
- Rust version (`rustc --version`):
- Tracker commit/branch:
- `torrust-server-lib` release/version:

## Eligibility Evidence

- [ ] Link HTTP tracker, REST API, health-check API, and UDP migration evidence.
- [ ] Link standalone HTTP and UDP environment/example migration evidence.
- [ ] Link final #1588 task inventory showing no supported in-workspace legacy
      shutdown consumer.
- [ ] Link external release notes with the deprecation support window.
- [ ] Link Q5 correction and final process-wrapper/removal-plan rationale.

## Compatibility Tests

### Test 1: Legacy APIs remain available

- [ ] Compile representative legacy `Halted` and signal-helper consumers.
- [ ] Record expected deprecation warnings.
- [ ] Verify legacy behavior remains unchanged.

**Evidence:**

```text
(paste compiler and focused test output)
```

### Test 2: Migrated paths have no legacy dependency

- [ ] Compile migrated paths with deprecation warnings treated as errors.
- [ ] Verify no production migrated path uses the deprecated shutdown API.

**Evidence:**

```text
(paste compiler and focused test output)
```

## Documentation Review

- [ ] Deprecated API messages identify the replacement and planned removal release.
- [ ] Release notes state the support window and removal prerequisites.
- [ ] Generated Rust documentation renders deprecation guidance accurately.

**Evidence:**

```text
(paste documentation-review notes)
```

## Summary

| Check                | Result  | Evidence link or note |
| -------------------- | ------- | --------------------- |
| Eligibility gate     | Pending |                       |
| Legacy compatibility | Pending |                       |
| Migrated paths clean | Pending |                       |
| Deprecation guidance | Pending |                       |
