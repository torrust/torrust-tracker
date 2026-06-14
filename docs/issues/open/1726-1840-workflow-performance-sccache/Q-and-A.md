# sccache Research — Questions & Answers

> File to record questions from the reviewer and responses from the implementer
> regarding the sccache research (issue #1726).

---

<!-- Template for each entry:
## Q: [Date] Question title

**Question**: ...

**Answer**: ...
-->

## Q: [2026-06-12] Would increasing the GHA cache limit improve workflow performance?

**Question**: The org-level cache settings show torrust-tracker at 8.65 GB out of 10 GB.
Your fork has cache settings (retention, size eviction limit) but the upstream repo doesn't
seem to expose those settings. Would increasing the cache limit help?

**Answer**: Increasing the cache limit would **not** fix the two main bottlenecks we found:

1. **Cache transfer speed** (30-70 MB/s): A 9 GB `target/` takes 130-300 s to restore —
   longer than recompiling. More space doesn't help throughput.
2. **Token scope**: sccache's GHA backend uses `ACTIONS_RUNTIME_TOKEN` which is job-scoped.
   Task 3a proved cross-run sccache restores DO work (93.38 % hit rate) because new jobs
   receive a new token that can read existing cache entries. However, the cache API has
   rate limits and the 10 GB pool is shared, so token renewal alone is not a bottleneck.

Where it **might** help is reducing eviction of `Swatinem/rust-cache` entries (600-730 MB each,
105 active caches), which compete with sccache entries for the same 10 GB pool. If eviction
becomes frequent, increasing to 15-20 GB could help.

**Recommendation**: Wait and observe. Let the new sccache workflows run for a week, then check
if cache eviction is causing misses. Only increase if needed — above 10 GB incurs cost.

The upstream org cache settings are at:
`https://github.com/organizations/torrust/settings/actions/caches`
— they exist, just at a different URL than the repo-level settings page.

---

## Q: [2026-06-12] Upstream repo cache is already over the 10 GB limit

**Question**: The upstream `torrust/torrust-tracker` cache page shows "13.03 GB of 10 GB Used"
— over the limit, with active eviction. The org-level page showed 8.65 GB.

**Answer**: The repo is **already over the 10 GB limit** and eviction is actively happening.
This confirms the cache pool problem we identified in the compile-hotspot-analysis.

The discrepancy between org-level (8.65 GB) and repo-level (13.03 GB) may mean the org-level
summary is stale or aggregates differently.

**Implications**:

- `Swatinem/rust-cache` entries (600-730 MB each) are already being evicted before they can be
  reused. This explains why `Swatinem/rust-cache` shows limited benefit — entries don't survive
  long enough.
- Adding sccache on top of the same 10 GB pool will increase eviction pressure.
- sccache entries (~450 MB for a full build) will also get evicted, reducing cross-run hit rates.

**Options**:

1. **Increase the limit** (e.g., 20 GB) — avoids eviction, gives headroom for both Swatinem
   and sccache caches. Costs money above 10 GB.
2. **Remove `Swatinem/rust-cache`** from jobs where sccache is added — frees ~600-730 MB per job.
   This might solve the problem without spending money.

---

## Q: [2026-06-12] Should we keep `Swatinem/rust-cache` alongside sccache?

**Question**: After adding sccache to CI workflows, should we keep using `Swatinem/rust-cache`
or remove it?

**Answer**: **Remove `Swatinem/rust-cache` from jobs where sccache is added.**

Comparison:

| Feature                    | Swatinem/rust-cache           | sccache GHA backend             |
| -------------------------- | ----------------------------- | ------------------------------- |
| Granularity                | Entire `target/` (~9 GB blob) | Individual `rlib` units         |
| Restore cost               | 130-300 s (blob download)     | Zero (build starts immediately) |
| Cross-run hit rate         | ~0 % (restore > recompile)    | **93.38 %**                     |
| Cache size per job         | 600-730 MB                    | ~450 MB                         |
| Cross-job sharing          | No (per-job key)              | Yes (GHA backend)               |
| Value on Cargo.lock change | Total miss                    | Partial hits (unchanged deps)   |

With the repo already at 13.03 GB / 10 GB, keeping both guarantees eviction of both.
sccache is strictly superior for this workspace — the experiment data proves it.

**Action**: Remove `Swatinem/rust-cache@v2` steps from all jobs that now have sccache.
