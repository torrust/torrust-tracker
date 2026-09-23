---
schema-version: 1
doc-type: issue
issue-type: bug
status: open
priority: p0
epic: null
github-issue: 2314
spec-path: docs/issues/open/2314-preserve-udp-scrape-response-order/ISSUE.md
branch: "2314-preserve-udp-scrape-response-order-spec"
related-pr: null
last-updated-utc: "2026-09-23 10:55"
semantic-links:
  skill-links:
    - create-issue
    - fix-bug
    - write-unit-test
  related-artifacts:
    - .github/skills/dev/debugging/fix-bug/SKILL.md
    - .github/skills/dev/planning/create-issue/SKILL.md
    - .github/skills/dev/testing/write-unit-test/SKILL.md
    - docs/issues/open/2314-preserve-udp-scrape-response-order/manual-verification-evidence.md
    - docs/issues/open/2314-preserve-udp-scrape-response-order/scrape-benchmark-evidence.md
    - packages/primitives/src/scrape.rs
    - packages/tracker-core/src/scrape_handler.rs
    - packages/udp-core/src/services/scrape.rs
    - packages/udp-server/src/handlers/scrape.rs
---

<!-- skill-link: create-issue -->

# Issue #2314 - Preserve UDP Scrape Response Order

## Goal

Ensure each UDP scrape response statistic is returned at the same index as its corresponding
requested info hash, with exactly one entry per requested info hash.

## Background

BEP 15 scrape responses contain an ordered sequence of `torrent_stats` entries with no info-hash
key. A client associates the first response entry with the first requested info hash, the second
entry with the second requested info hash, and so on. The response is therefore only meaningful if
the tracker preserves the request order and returns one entry per requested hash.

### BEP 15 Scrape Wire Format

All values use network byte order (big endian). BEP 15 defines the scrape request and response as
follows:

```text
scrape request:

Offset          Size            Name            Value
0               64-bit integer  connection_id
8               32-bit integer  action          2 // scrape
12              32-bit integer  transaction_id
16 + 20 * n     20-byte string   info_hash
16 + 20 * N

scrape response:

Offset          Size            Name            Value
0               32-bit integer  action          2 // scrape
4               32-bit integer  transaction_id
8 + 12 * n      32-bit integer  seeders
12 + 12 * n     32-bit integer  completed
16 + 12 * n     32-bit integer  leechers
8 + 12 * N
```

The response has no `info_hash` field after its header. Its $n$th 12-byte statistics block must
therefore describe the $n$th requested `info_hash`; a tracker cannot reorder, drop, or merge
entries without making the response ambiguous.

The current code loses that order:

1. `tracker-core::ScrapeHandler::handle_scrape` iterates the request's `info_hashes` in order but
   inserts each result into `ScrapeData.files`
   ([`scrape_handler.rs`](../../../../packages/tracker-core/src/scrape_handler.rs)).
2. `ScrapeData.files` is a `HashMap<InfoHash, SwarmMetadata>`
   ([`primitives/src/scrape.rs`](../../../../packages/primitives/src/scrape.rs)).
3. `udp-server::handlers::scrape::build_response` iterates `scrape_data.files` to construct the
   positional `ScrapeResponse.torrent_stats` vector
   ([`udp-server/src/handlers/scrape.rs`](../../../../packages/udp-server/src/handlers/scrape.rs)).

`HashMap` iteration order is not insertion order, and Rust's default `RandomState` hasher is seeded
per map instance, so the order also varies between otherwise identical requests. With two or more
requested torrents having different statistics, the response can associate a statistic with the
wrong info hash.

### Reproduction summary

Reproduced on 2026-09-23 against unmodified `develop` (`60a4a160`) with the default development
configuration. Two torrents were seeded with distinguishable statistics (A: 1 seeder; B: 2 seeders,
1 leecher) and the same UDP scrape request was repeated ten times per order:

| Request order | Correct responses | Swapped responses |
| ------------- | ----------------- | ----------------- |
| `[A, B]`      | 7 / 10            | 3 / 10            |
| `[B, A]`      | 3 / 10            | 7 / 10            |

A duplicate-hash request `[A, A, B]` returned only 2 entries, because the map collapses duplicate
keys. Full commands and output: [`manual-verification-evidence.md`](manual-verification-evidence.md)
section V1.

The HTTP tracker is not affected: its scrape response is a bencoded dictionary keyed by info hash
(`http-protocol` uses its own `BTreeMap`-backed `ScrapeData`), so order carries no meaning there.

The released `main` branch (tag `v3.0.0`) contains the same `for file in &scrape_data.files` loop in
`src/servers/udp/handlers.rs`, so the defect is present in the current release, not only in
`develop`. No backport or hotfix is planned: the fix ships with the next major release (4.0.0),
to which all users are expected to migrate.

This was discovered while reviewing test maintainability under issue #2283 (part of the
"Overhaul: Packages Testing" EPIC #1347). It is a product correctness bug, not coverage work, and is
intentionally outside that issue's scope; it is therefore not filed as a subissue of #1347.

## Scope

### In Scope

- Preserve requested info-hash order across the scrape-data contract and UDP response encoding.
- Return exactly one `torrent_stats` entry per requested info hash, including duplicates. This is
  the same causal seam (the `HashMap` representation) and the same protocol contract; fixing order
  without fixing entry count would leave positional decoding broken for duplicate requests.
  Confirmed by the maintainer: position is the only matching mechanism in UDP, so every requested
  item must be answered.
- Add deterministic maintained regression tests for both facets (order and entry count).
- Verify the corrected behavior with the same local UDP scenario recorded in V1 and record all
  evidence.

### Out of Scope

- The pending `handlers/scrape.rs` test readability refactor from issue #2283.
- Order-insensitive response assertions.
- Changing cookie validation, scrape authorization, or counter-saturation behavior.
- HTTP scrape response changes.
- Broad refactors unrelated to preserving the scrape response contract.

## Architectural Decisions

- Related ADRs:
  [`20260527175600_keep_protocol_and_domain_types_decoupled.md`](../../../adrs/20260527175600_keep_protocol_and_domain_types_decoupled.md)
  (the domain `ScrapeData` must stay protocol-agnostic; any ordered representation belongs to the
  domain or the UDP adapter, not to `udp-protocol`).
- ADRs to create: `None anticipated`; create one only if preserving order requires a reusable
  cross-protocol result abstraction with material architectural consequences.
- Do not sort by info hash. Sorting creates a stable order but does not preserve the caller's
  requested order.

Two candidate fixes were identified during analysis. The maintainer chose Option 1 during spec
review (see Decision below); B7 only re-confirms it against the red tests.

### Ownership framing

Both `ScrapeData` types identify torrents correctly by info hash: the domain type in `primitives`
(`HashMap`) and the HTTP protocol type in `http-protocol` (`BTreeMap`). Neither loses information.
The defect appears only where a positional wire format is produced from keyed data, which today is
the UDP response builder. That makes position a delivery-layer concern of the UDP adapter, not a
property of the domain result.

### Option 1 - UDP response builder iterates the request

`build_response` in `udp-server::handlers::scrape` iterates `request.info_hashes` and reads each
hash's metadata from `scrape_data.files`, falling back to zeroed metadata if a hash is absent.
`ScrapeData` gains a doc comment stating that it is keyed and unordered and that positional
protocols must iterate the request.

Pros:

- Matches ownership: positional identity exists only in the UDP wire format, so the UDP adapter
  owns it. Consistent with the protocol/domain decoupling ADR.
- Smallest production change: one function in one file, plus tests.
- Fixes both facets by construction: one entry per requested hash, duplicates included.
- No change to the `ScrapeData` API, `zeroed()`, equality semantics, or any HTTP consumer; zero
  risk to the HTTP path.
- No new dependency.
- Robust to future `tracker-core` changes (deduplication, reordering, parallel lookups): the
  response stays aligned with the request regardless of how the map was filled.

Cons:

- The order guarantee lives in adapter code, not in the type; a future positional consumer must
  repeat the pattern (mitigated by the doc comment and the regression tests).
- Needs a defined behavior for a requested hash missing from the map. It cannot happen today
  because `tracker-core` inserts every requested hash, but the builder must choose; zeroed
  metadata matches how unknown and unauthorized torrents are already reported.
- Duplicate hashes are still authorized and looked up once per occurrence in `tracker-core`
  (pre-existing, harmless).
- One extra `HashMap` lookup per requested hash (at most about 74 per request); negligible.

### Option 2 - Ordered `ScrapeData` in `primitives`

Replace `HashMap<InfoHash, SwarmMetadata>` with an insertion-ordered representation and update
every producer and consumer.

Pros:

- Request order becomes an explicit property of the domain result; any consumer gets it for free.
- The UDP builder needs no second lookup.

Cons:

- Changes a public primitive type used by five crates (`primitives`, `tracker-core`, `http-core`,
  `axum-http-server`, `udp-server`) and their tests; `zeroed()`, `add_file`, and `PartialEq`
  expectations must be revisited.
- Encodes a UDP-only concern (position) into a protocol-agnostic domain type, against the spirit
  of the decoupling ADR.
- Does not fix duplicates on its own. An insertion-ordered map type still collapses duplicate keys, so
  UDP would still need to iterate the request (Option 1 again). A `Vec<(InfoHash, SwarmMetadata)>`
  keeps duplicates but loses keyed lookup, and the HTTP conversion into `BTreeMap` collapses them
  anyway.
- Either adds a dependency (an insertion-ordered map crate) or trades O(1) lookup for a linear scan.
- Larger diff, review, and regression surface for the same user-visible outcome.

### Decision

**Option 1 is selected** (maintainer decision, 2026-09-23, agreeing with the analysis
recommendation). Rationale, in order of weight:

1. **It fixes both observed symptoms; Option 2 alone does not.** An insertion-ordered map still
   collapses `[A, A, B]` to two entries, so the UDP builder would have to iterate the request
   anyway, which is Option 1. Only a `Vec` of pairs keeps duplicates, and that breaks keyed lookup
   for the HTTP path.
2. **It puts the fix where the contract lives.** Position carries meaning only in the BEP 15 wire
   format. Both `ScrapeData` types are correctly keyed by info hash and lose no information;
   matching a statistics block to a torrent by position is a responsibility of the UDP response
   builder, not of the domain result. Teaching the protocol-agnostic domain type about position
   would leak a UDP concern into shared code, against the decoupling ADR.
3. **Smallest blast radius.** One function in one file plus tests, versus a public primitive type
   used by five crates and their tests, with no HTTP behavior to re-verify.
4. **Robust to future core changes.** However `tracker-core` fills the map (deduplication,
   parallel lookups, caching), the response stays aligned with the request.

Accepted cost: the ordering guarantee lives in adapter code rather than in the type. It is guarded
by the two regression tests and a doc comment on `ScrapeData` stating that it is keyed and
unordered and that positional protocols must iterate the request. Revisit Option 2 only if a second
positional consumer of `ScrapeData` appears.

### Performance Considerations

The tracker is built in layers and copies scrape data upward from `tracker-core` to the UDP
delivery layer. That design buys maintainability, modularity, and observability at some
performance cost, which is acceptable as long as it stays small. This fix must not make the
scrape path measurably slower.

Expected cost of Option 1, from analysis:

- The old builder iterates the `HashMap` once: O(N) over sparse buckets, N = requested hashes
  (at most about 74 per BEP 15).
- The new builder iterates `request.info_hashes` and performs N keyed lookups: O(N) with one
  SipHash of a 20-byte key per lookup, roughly 20-30 ns each, so about 2 µs worst case per
  request. No additional allocation is introduced.
- The rest of the scrape path already performs, per request, cookie validation, N asynchronous
  whitelist authorizations, N repository lookups, one `HashMap` allocation, an event send, and a
  UDP syscall. The added lookups are expected to be far below the documented ±5-10% run-to-run
  variance of the end-to-end load test.
- Include a free micro-improvement in the same change: allocate `torrent_stats` with
  `Vec::with_capacity(request.info_hashes.len())` instead of growing it from empty.

Because an end-to-end load test cannot resolve a change of a few microseconds per request, the
verification uses two instruments:

1. **End-to-end UDP load test** (`aquatic_udp_load_test`, documented in
   [`docs/benchmarking.md`](../../../benchmarking.md)) with a scrape-heavy request mix. This is the
   user-visible check: it proves there is no gross regression in scrape responses per second.
2. **Criterion microbenchmark** for the UDP scrape handler (`handle_scrape` with 74 requested
   hashes), added to `packages/udp-server/benches/` following the existing `udp-core` `connect_once`
   bench. This is the precise instrument: it quantifies the actual per-request delta of the changed
   function and leaves the scrape path with maintained benchmark coverage, which it currently lacks.

The pre-existing cost of copying scrape data across layers is out of scope here. The baseline
recorded by P1 gives the number needed to open a separate performance issue if desired.

## Design and Ownership Review

The defect crosses an asynchronous service boundary but introduces no new lifecycle, readiness, or
child-process ownership. Before implementation, confirm:

- the owner of the positional contract. The order requirement exists only where the wire format is
  positional, which today is UDP (`udp-server::handlers::scrape`); `tracker-core` produces the data
  and `udp-core::ScrapeService` passes it through unchanged;
- whether any current `ScrapeData` consumer other than UDP requires request order. Known consumers:
  `udp-server::handlers::scrape::build_response` (positional), `axum-http-server` scrape handler
  (converts to a keyed `BTreeMap`), `http-core::ScrapeService` (zeroed on failed authentication),
  tests in `tracker-core` and `http-core`;
- duplicate semantics: BEP 15 sends one 20-byte hash per entry and defines the response as one
  12-byte stats block per requested hash. Current behavior collapses duplicates (V1). The builder
  must produce one entry per requested hash; and
- that the adaptation change stays confined to the UDP response builder and does not expose
  storage details.

Complete the B7 re-confirmation checkpoint after the red regression tests identify the causal seam
and before changing production code.

## Bug-Fix Process

Follow `.github/skills/dev/debugging/fix-bug/SKILL.md`.

1. **Analysis (done).** Source-level hypothesis confirmed: `HashMap` iteration in
   `build_response` over a map built from an ordered request. See Background.
2. **Reproduction (done).** Recorded in `manual-verification-evidence.md` V1 against a real local
   UDP tracker build: 10 of 20 identical requests returned swapped entries; duplicates collapse.
3. **Select the regression-test boundary.** See Regression Test Strategy.
4. **Prove red.** Add the regression tests before the production fix, run them against the broken
   implementation, and record the failing command and output in the evidence file.
5. **Fix the causal layer.** Apply Option 1 (re-confirmed in B7). Do not make the regression test
   accept either order.
6. **Verify green and recheck like-for-like.** Run the regression tests, focused tests for every
   changed crate, and repeat V1 unchanged as V2.

## Regression Test Strategy

**Boundary.** A collaboration test in `packages/udp-server/src/handlers/scrape.rs` that calls the
production `handle_scrape` (UDP handler → `udp-core::ScrapeService` → `tracker-core::ScrapeHandler`
→ `build_response`). This is the smallest boundary that observes the defect as users do: a direct
`build_response` test cannot show where request order is lost, and a `tracker-core`-only test cannot
observe positional encoding. The existing fixtures in that module
(`initialize_core_tracker_services_for_public_tracker`, `add_a_seeder`, `build_scrape_request`)
already support this boundary. No `primitives` test is needed: `ScrapeData` stays unchanged apart
from a doc comment.

Package-level tests are the goal; do not add a socket-level integration test under `tests/`
unless the package boundary proves unable to guard the bug, and record that reason if it happens.
The purpose is regression protection only; broader `udp-server` coverage belongs to issue #2283
under EPIC #1347.

**Determinism.** Rust's `std::collections::HashMap` uses a per-instance random seed, so a two-hash
request has roughly a 50% chance of coming back in request order (V1 showed 10/20). A red test
must not depend on that coin flip. Use these two tests:

1. **Order test.** Request `N` distinct info hashes (`N ≥ 8`), each seeded with a distinct seeder
   count (`hash_i` has `i` seeders) so every entry is identifiable, and assert
   `torrent_stats` equals the complete expected vector in request order. Against the broken
   implementation the test passes by chance only if `HashMap` iteration happens to match insertion
   order for all `N` keys: `1/N!` (`1/40320` for `N = 8`). Record the chosen `N` and this bound in
   the evidence; this is the strongest red evidence obtainable without changing the production
   hasher.
2. **Entry-count test.** Request `[A, A]` and assert two entries. This fails deterministically
   against the `HashMap` implementation (one entry) and passes deterministically after the fix.

Arrange must make visible: the ordered list of requested hashes, the distinguishable metadata per
hash, and the complete expected `torrent_stats` vector in request order. Keep the production Act
(`handle_scrape`) and typed assertions visible; fixtures own only incidental mechanics (cookie,
socket addresses, service wiring). Apply the prose-first Arrange-Act-Assert review from the
`write-unit-test` skill before commit.

Record red output, green output, and the V2 recheck in `manual-verification-evidence.md`.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| --- | ------ | ---- | ----------------------- |
| B1 | DONE | Analyze the causal path and inventory consumers | Confirmed: `tracker-core` writes request results into `ScrapeData.files`; `udp-server` iterates that `HashMap` to create the positional response. HTTP consumes the same domain data only through keyed maps. |
| B2 | DONE | Reproduce against a real local tracker artifact | V1 establishes distinguishable torrent statistics, observes swapped results in 10 of 20 UDP scrapes, and observes duplicate-hash collapse. |
| B3 | TODO | Read the current UDP scrape tests and load `write-unit-test` | Inventory the test module's existing helpers and decide which incidental mechanics a fixture may own. Record the causal initial-state difference and prose-first Arrange-Act-Assert design in task evidence before adding tests. |
| B4 | TODO | Add the deterministic duplicate-entry red regression | In `udp-server` handler collaboration tests, request `[A, A]` and assert two typed statistics entries. Run the focused test against the unfixed code; it must fail because `HashMap` collapses the duplicate. Record the exact command and failure output in `manual-verification-evidence.md`. |
| B5 | TODO | Add the high-confidence order red regression | Request at least eight distinct hashes in a non-sorted known order, establish a distinct statistic for each, and assert the complete typed response vector in request order. Run it against the unfixed code and record the command, output, chosen `N`, and $1/N!$ accidental-pass bound. Do not continue if it passes; increase `N` or redesign the test boundary. |
| B6 | TODO | Review the red tests' design | Verify each test uses production `handle_scrape`, exposes the single causal difference, keeps the expected vector visible, and does not accept reordering. Record the completed prose-first review. |
| P0 | TODO | Add the scrape microbenchmark | Add `packages/udp-server/benches/udp_tracker_server_benchmark.rs` (Criterion, `harness = false`) with a `scrape_once` group that calls the production `handle_scrape` with 74 distinct requested hashes present in the repository, modeled on `packages/udp-core/benches/udp_tracker_core_benchmark.rs`. Commit it before the fix so P1 and P2 run the identical bench. |
| P1 | TODO | Record the pre-fix performance baseline | On the unfixed code, run the microbenchmark (P0) and the scrape-heavy end-to-end load test per the Performance Verification procedure. Record machine characteristics, exact commands, configs, per-run results, and medians in issue-local `scrape-benchmark-evidence.md`. |
| B7 | TODO | Re-confirm the repair boundary after the red tests | Option 1 is already selected (see Architectural Decisions, Decision). Confirm the red tests point at `build_response` as the causal seam and that no `ScrapeData` consumer discovered during B3-B6 needs request order; record the confirmation. Escalate to the maintainer only if the red tests contradict the decision. |
| B8 | TODO | Implement the smallest causal fix | In `udp-server::handlers::scrape::build_response`, iterate `request.info_hashes` and look up each hash in `scrape_data.files` before encoding it; never iterate the map for a positional response. Allocate `torrent_stats` with `Vec::with_capacity(request.info_hashes.len())`. Use zeroed metadata for a hash absent from the map (cannot happen today; matches how unknown and unauthorized torrents are reported) and cover it with a test. Add the keyed-and-unordered doc comment to `ScrapeData`. |
| B9 | TODO | Run focused regression tests green | Re-run B4 and B5 after the fix. Both tests must pass without weakened assertions; record the commands and output in `manual-verification-evidence.md`. |
| B10 | TODO | Run focused affected-crate tests | Run the `torrust-tracker-udp-server` package tests and, because of the `ScrapeData` doc comment, `torrust-tracker-primitives` (`cargo test --doc` included). |
| P2 | TODO | Repeat the performance measurement after the fix | Same machine, same procedure, same configs and run count as P1, on the fixed code. Record results next to the baseline in `scrape-benchmark-evidence.md` and compute the median delta for both instruments. If the end-to-end median drops more than 5% or the microbenchmark shows a delta inconsistent with the analysis above, stop and investigate before B11. |
| B11 | TODO | Recheck the original user-visible artifact | Repeat V1 unchanged against a fresh local tracker as V2: ten `[A, B]` scrapes, ten `[B, A]` scrapes, and `[A, A, B]`. Record commands, output, runtime details, and relevant logs. Every response must align with request order and the duplicate request must contain three entries. |
| B12 | TODO | Complete quality gates and acceptance review | Run `cargo +nightly fmt --all -- --check` and `linter all`; review every acceptance criterion against the red/green and V1/V2 evidence. |
| B13 | TODO | Record completion outcome | Add an implementation retrospective only if a material design discovery occurred; otherwise record why one was unnecessary in the progress log. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| B1-B2 | Issue spec and reproduction evidence | Commit in the spec-first PR. |
| B3-B6 | Red regression tests and their test-design review | Commit after both focused red validations and completed prose-first review. |
| P0 | Scrape microbenchmark | Commit before the fix; `cargo bench -p torrust-tracker-udp-server --no-run` must build. |
| P1 | Pre-fix benchmark evidence | Commit `scrape-benchmark-evidence.md` with the baseline before B8. |
| B7-B10 | Selected repair and green focused verification | Commit after documented design decision and focused green validation. |
| P2 | Post-fix benchmark evidence | Commit the comparison update to `scrape-benchmark-evidence.md`. |
| B11-B13 | Artifact recheck, quality gates, and completion evidence | Commit only if tracked issue artifacts changed. |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/open/2314-preserve-udp-scrape-response-order/ISSUE.md`
- [x] Local UDP artifact reproduction recorded in `manual-verification-evidence.md` (V1)
- [ ] Spec reviewed and approved by user/maintainer
- [ ] GitHub issue created and issue number added to this spec
- [ ] Spec-only PR merged into `develop` before implementation
- [ ] Source-level hypothesis reviewed with the selected implementation agent
- [ ] Red regression tests recorded
- [ ] Scrape microbenchmark added and pre-fix baseline recorded (P0, P1)
- [x] Fix option decided by maintainer (Option 1; see Architectural Decisions)
- [ ] Fix option re-confirmed against the red tests (B7)
- [ ] Production fix completed
- [ ] Green regression and like-for-like artifact recheck (V2) recorded
- [ ] Post-fix benchmark recorded and compared with the baseline (P2)
- [ ] Automatic verification completed (`linter all`, relevant tests, pre-push checks)
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-09-23 - GitHub Copilot - Drafted from source analysis during issue #2283 scrape test
  review. No runtime reproduction performed yet.
- 2026-09-23 08:30 UTC - GitHub Copilot - Copied draft into this repository; confirmed hypothesis
  against source; reproduced against local `develop` (`60a4a160`): 10/20 swapped responses,
  duplicates collapsed. Added duplicate-entry facet to scope, fix options table, and deterministic
  red-test strategy. Set `epic: null` (bug is not testing work). Evidence:
  `manual-verification-evidence.md` V1.
- 2026-09-23 09:10 UTC - GitHub Copilot - Maintainer review: priority raised to p0; duplicates
  confirmed in scope; no backport to `v3.0.0` (fix ships in 4.0.0); package-level tests preferred
  over socket-level integration tests; fix-option analysis expanded with a recommendation for
  Option 1, final choice pending maintainer decision. Confirmed `main` (`v3.0.0`) carries the same
  loop.
- 2026-09-23 09:40 UTC - GitHub Copilot - Maintainer selected Option 1 (UDP response builder
  iterates the request). Rationale recorded under Architectural Decisions, Decision. B7 reduced to a
  re-confirmation step; B8 and B10 narrowed to the Option 1 change set.
- 2026-09-23 10:05 UTC - GitHub Copilot - Maintainer raised performance concern about layered
  copying plus per-request lookups. Added Performance Considerations (analysis: expected delta of
  about 2 µs worst case, below load-test variance), tasks P0-P2 (scrape microbenchmark, pre-fix
  baseline, post-fix comparison on the same machine), Performance Verification procedure, AC7, and
  the `scrape-benchmark-evidence.md` artifact. Tooling inventory: `aquatic_udp_load_test` supports a
  scrape-heavy mix; no scrape microbenchmark exists yet in the workspace.
- 2026-09-23 10:30 UTC - GitHub Copilot - Created issue-local `scrape-benchmark-evidence.md`
  skeleton (environment, code under test, per-instrument P1/P2 tables, comparison rule, anomalies).
  Maintainer decision: keep it issue-specific; no generic benchmark template unless the measurement
  is automated later.
- 2026-09-23 10:55 UTC - GitHub Copilot - Created GitHub issue #2314, moved this folder to the
  open issue location, and prepared the spec-only branch.

## Acceptance Criteria

- [x] AC1: A local UDP tracker artifact demonstrates the pre-fix ordering mismatch
  (`manual-verification-evidence.md` V1).
- [ ] AC2: Maintained regression tests fail before the fix: the order test for `N ≥ 8` distinct
  hashes with distinct statistics, and the entry-count test for a duplicated hash.
- [ ] AC3: The production path returns `torrent_stats[i]` for `info_hashes[i]` for every `i`,
  including duplicated hashes, without sorting info hashes or accepting unordered results.
- [ ] AC4: The regression tests pass after the fix.
- [ ] AC5: The V1 scenario is rerun unchanged after the fix (V2) and every response returns
  statistics in request order; `[A, A, B]` returns three entries.
- [ ] AC6: HTTP scrape behavior is unchanged (`http-core` and `axum-http-server` tests pass).
- [ ] AC7: Scrape performance is measured before and after the fix on the same machine with the
  same procedure (P1, P2). The end-to-end median scrape throughput after the fix is within 5% of
  the baseline, and the microbenchmark delta is consistent with the analysis in Performance
  Considerations; any larger regression is investigated and its resolution recorded before merge.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior

## Verification Plan

### Automatic Checks

- Focused red/green regression tests in `torrust-tracker-udp-server` (`handlers::scrape`).
- Tests for each changed crate: `torrust-tracker-udp-server` and `torrust-tracker-primitives`
  (doc comment only; include `cargo test --doc`).
- `cargo +nightly fmt --all -- --check` (nightly Rust toolchain).
- `linter all`.

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1 | Reproduce pre-fix ordering | Start the local tracker (`./target/debug/torrust-tracker`), seed A with 1 seeder and B with 2 seeders + 1 leecher via `tracker_client udp announce`, then run `tracker_client udp scrape 127.0.0.1:6969 $A $B` ten times and `... $B $A` ten times; also `... $A $A $B`. | Some responses return the entries at the wrong index; the duplicate request returns 2 entries. | DONE | `manual-verification-evidence.md` V1 |
| M2 | Recheck fixed ordering | Repeat M1 unchanged after the fix. | All 20 responses return statistics in request order; `[A, A, B]` returns 3 entries. | TODO | `manual-verification-evidence.md` V2 |

### Performance Verification

Run the same procedure twice on the same machine: P1 on the unfixed code (after P0 is committed),
P2 on the fixed code. Record everything in issue-local
[`scrape-benchmark-evidence.md`](scrape-benchmark-evidence.md), which already contains the
environment table, per-instrument result tables, and the comparison rule; fill it with actual
results only. The file is issue-specific (`doc-type: benchmark-report`, same as issue #1505); no
generic benchmark template is created because the content depends on what is benchmarked.
Promoting it to a template is worthwhile only if this measurement is repeated by an automatic or
scheduled workflow, which is out of scope here.

Machine characteristics to record once, at the top of the evidence file: CPU model and core count
(`lscpu`), RAM (`free -g`), kernel (`uname -srm`), Rust toolchain (`rustc --version`), CPU
frequency governor at run time (the `scaling_governor` file under
`/sys/devices/system/cpu/cpu0/`, or the `lscpu` scaling percentage),
and whether the machine was otherwise idle. Set the governor to `performance` for the runs when
possible; if not, record the actual governor and the observed scaling percentage.

Instrument 1, microbenchmark (precise):

```console
cargo bench -p torrust-tracker-udp-server -- scrape_once
```

Run it three times; record Criterion's reported mean and confidence interval for each run.

Instrument 2, end-to-end load test (user-visible):

1. Build the tracker in release mode and start it with
   `share/default/config/tracker.udp.benchmarking.toml` (logging at `error`, UDP on `0.0.0.0:3000`).
2. Build `aquatic_udp_load_test` from source per `docs/benchmarking.md` and generate its config.
3. Use a scrape-heavy mix and the BEP 15 practical maximum of hashes per scrape; keep enough
   connects for cookies and enough announces so that scraped torrents exist in the repository.
   Suggested starting values, to be recorded verbatim with the results:

   ```toml
   duration = 30
   summarize_last = 20

   [requests]
   scrape_max_torrents = 74
   weight_connect = 10
   weight_announce = 10
   weight_scrape = 80
   ```

4. Run five iterations; record `Scrape responses/second` and total responses per second for
   each, then the median. The median is the comparison value; P2 reuses the identical config file.

Comparison rule: P2 end-to-end median within 5% of P1 passes AC7. Because documented run-to-run
variance is ±5-10%, a drop between 5% and 10% is not automatically a regression, but it must be
investigated (rerun, check governor and background load, consult the microbenchmark delta) and the
conclusion recorded before merge.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | DONE                   | `manual-verification-evidence.md` V1 |
| AC2   | TODO                   | Red test output in `manual-verification-evidence.md` |
| AC3   | TODO                   | Fix commit |
| AC4   | TODO                   | Green test output in `manual-verification-evidence.md` |
| AC5   | TODO                   | `manual-verification-evidence.md` V2 |
| AC6   | TODO                   | Focused HTTP crate test output |
| AC7   | TODO                   | `scrape-benchmark-evidence.md` P1/P2 comparison |

## Risks and Trade-offs

- **Probabilistic red for the order test:** `HashMap` order is random per instance, so no two-hash
  test is reliably red. Mitigation: use `N ≥ 8` hashes (chance pass `1/N!`) and pair it with the
  deterministic entry-count test; record both red outputs.
- **Duplicate-hash semantic drift:** an ordered fix must return one entry per requested hash.
  Option 1 does this by construction. Mitigation: the entry-count test guards it.
- **Cross-protocol impact:** not applicable with Option 1; `ScrapeData` and every HTTP consumer
  stay unchanged. AC6 confirms it.
- **Performance regression on the scrape path:** the fix adds N keyed lookups per request.
  Analysis predicts about 2 µs worst case, below load-test noise, but the layered design already
  carries copying overhead and the project has not benchmarked recently. Mitigation: P0-P2 measure
  before and after on the same machine with both a microbenchmark and an end-to-end load test;
  AC7 gates the merge on the result.
- **Benchmark noise misread as regression (or masking one):** ±5-10% variance on a non-dedicated
  machine. Mitigation: fixed governor, idle machine, five runs, median comparison, identical
  configs, and the microbenchmark as the tie-breaker.

## Implementation Completion Review

- Retrospective: `Not yet assessed`.
- If needed, create `implementation-retrospective.md` from
  `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this directory; otherwise add a concise
  progress-log entry explaining why none was needed.

## References

- Discovered during issue #2283 `handlers/scrape.rs` test review (EPIC #1347).
- [BEP 15: UDP Tracker Protocol](https://www.bittorrent.org/beps/bep_0015.html) — scrape
  response format.
- `.github/skills/dev/debugging/fix-bug/SKILL.md`.
- [`docs/benchmarking.md`](../../../benchmarking.md) and
  [`docs/issues/closed/1505-optimize-peer-ip-list-from-swarm/aquatic-benchmarking-guide.md`](../../closed/1505-optimize-peer-ip-list-from-swarm/aquatic-benchmarking-guide.md)
  (end-to-end UDP load-test procedure).
- `packages/udp-core/benches/udp_tracker_core_benchmark.rs` (Criterion bench to model P0 on).
- `packages/primitives/src/scrape.rs`.
- `packages/tracker-core/src/scrape_handler.rs`.
- `packages/udp-core/src/services/scrape.rs`.
- `packages/udp-server/src/handlers/scrape.rs`.
- `packages/http-protocol/src/v1/responses/scrape/data.rs` (unaffected keyed HTTP representation).
