---
doc-type: manual-verification-evidence
status: not-started
spec-path: docs/issues/open/2234-1488-si-2-remove-global-shutdown-signal/ISSUE.md
last-updated-utc: 2026-09-16 09:10
semantic-links:
      related-artifacts:
            - docs/issues/open/2234-1488-si-2-remove-global-shutdown-signal/ISSUE.md
---

# Manual Verification Evidence - Additive Server Lifecycle API

> **Status**: Not started. This evidence applies only to the additive API
> introduction; it must not claim removal of legacy shutdown behavior.

## Environment

- Date:
- OS:
- Rust version (`rustc --version`):
- Tracker commit/branch:

## Test Results

### Additive Compatibility

- [ ] Existing `Halted` channel consumers compile and preserve their current
      stop behavior.
- [ ] The new lifecycle API accepts injected cancellation without requiring an
      OS-signal subscription.

### Deterministic Lifecycle Test

- [ ] A test requests cancellation without delivering an OS signal.
- [ ] The test awaits the component's top-level completion outcome.

### Release Evidence

- [ ] Record the `torrust-server-lib` release/version providing the additive
      API and the workspace dependency version that consumes it.
