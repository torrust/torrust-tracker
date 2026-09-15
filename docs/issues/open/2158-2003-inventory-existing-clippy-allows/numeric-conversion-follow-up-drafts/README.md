# Numeric Conversion Follow-up Drafts

These issue-local drafts are design inputs for the numeric conversion classification in #2158.
They are not standalone issue specifications and have no GitHub issue numbers. Retain them here
until #2158 has classified the full existing Clippy-allow baseline and the maintainer re-evaluates
whether each concern should become a separate issue, be resolved directly, or be organized under a
new numeric-conversion EPIC.

The drafts deliberately separate conversion concerns by invariant and ownership boundary rather
than creating one issue for every Clippy lint occurrence. Each linked inventory entry remains
**Temporary** only while its removal condition is unresolved. Entries with an already-proven,
well-documented local invariant may instead be reclassified as **Retain** or **Remove** during the
remaining inventory review.

| Draft | Candidate inventory entries | Re-evaluation question |
| ----- | --------------------------- | ---------------------- |
| `metric-aggregate-conversion-safety.md` | A080-A087, A113-A114, A143-A154, A170, A178-A222, A224-A227 | Should metrics expose checked, typed conversion APIs instead of each consumer casting `f64` to `u64`? |
| `wire-numeric-conversion-validation.md` | A156, A171 | Do wire-format values already have sufficient type/range invariants, or should parsing and response construction make those bounds explicit? |
| `domain-numeric-conversion-contracts.md` | A099, A123, A129 | Are local domain bounds already proven, or should conversions use checked APIs and focused tests? |

Benchmark/example conversions A112 and A120 are intentionally excluded. Their bounded inputs and
precision requirements are local and should be classified directly in #2158.
