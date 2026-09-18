---
spec-path: docs/issues/closed/2158-2003-inventory-existing-clippy-allows/ISSUE.md
last-updated-utc: 2026-09-18 14:40
semantic-links:
	related-artifacts:
		- docs/issues/closed/2158-2003-inventory-existing-clippy-allows/ISSUE.md
---

# Numeric Conversion Follow-up Drafts

These issue-local drafts are the design inputs that produced approved numeric-conversion EPIC #2243
and child issues #2244, #2245, and #2246. They are retained here as #2158 classification evidence,
not as pending issue specifications.

The drafts deliberately separate conversion concerns by invariant and ownership boundary rather
than creating one issue for every Clippy lint occurrence. Their linked temporary entries now point
to the approved GitHub issues that own the remaining review and remediation work.

| Draft | Approved issue | Inventory entries |
| ----- | -------------- | ----------------- |
| `metric-aggregate-conversion-safety.md` | #2244 | A080-A087, A113-A114, A143-A154, A170, A178-A222, A224-A227 |
| `wire-numeric-conversion-validation.md` | #2245 | A156, A171 |
| `domain-numeric-conversion-contracts.md` | #2246 | A099, A123, A129 |

Benchmark/example conversions A112 and A120 are intentionally excluded. Their bounded inputs and
precision requirements are local and should be classified directly in #2158.
