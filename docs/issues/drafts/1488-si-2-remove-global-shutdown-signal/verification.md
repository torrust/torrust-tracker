# Verification Evidence — Additive Server Lifecycle API

> **Status**: Not started. This evidence applies only to the additive API
> introduction; it must not claim removal of legacy shutdown behavior.

## Environment

- Date:
- OS:
- Rust version (`rustc --version`):
- Tracker commit/branch:

## Test Results

### Additive compatibility

- [ ] Existing `Halted` channel consumers compile and preserve their current
      stop behavior.
- [ ] The new lifecycle API accepts injected cancellation without requiring an
      OS-signal subscription.

### Deterministic lifecycle test

- [ ] A test requests cancellation without delivering an OS signal.
- [ ] The test awaits the component's top-level completion outcome.

### Release evidence

- [ ] Record the `torrust-server-lib` release/version providing the additive
      API and the workspace dependency version that consumes it.
