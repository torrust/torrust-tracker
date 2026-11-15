---
doc-type: mutation-evidence
issue: 2824
package: torrust-tracker-primitives
status: completed
measured-utc: 2026-11-15T07:51:51Z
---

# Primitives Mutation Evidence

This is the bounded mutation-testing assessment (T4) for Issue #2824. The package has only 147 mutants, so every source
file was run. It is not a package score, a coverage target or a CI gate. The per-file mutation samples of S2-S4 are in
each file's decision record; this run checks the whole package once, after the last test-producing commit.

## Configuration

```text
RUSTFLAGS="--cap-lints=warn" cargo mutants \
  -p torrust-tracker-primitives --all-features \
  --output .tmp/mutants-2824 --timeout-multiplier 5 --jobs 2
```

`cargo-mutants 27.0.0`, stable Rust 1.99.1, on branch `2824-1347-add-tests-primitives-final` at the commit
`test(primitives): [#2824] record the no-change files and compare instance id JSON by value`. Started
2026-11-15T07:51:51Z, finished 07:52:56Z (64 s). `--cap-lints=warn` follows the #1348 lesson: the workspace denies
`unused`, which would turn replaced bodies into unviable mutants. The baseline without mutations passed.

## Results

147 mutants: **94 caught, 3 missed, 2 timeouts, 48 unviable.**

| File | Caught | Missed | Timeout | Unviable |
| --- | ---: | ---: | ---: | ---: |
| `announce.rs` | 2 | 0 | 0 | 2 |
| `announce_interval.rs` | 12 | 0 | 0 | 5 |
| `configuration_instance_id.rs` | 1 | 1 | 0 | 1 |
| `driver.rs` | 5 | 0 | 0 | 1 |
| `mode.rs` | 1 | 0 | 0 | 0 |
| `pagination.rs` | 3 | 0 | 0 | 1 |
| `peer.rs` | 54 | 0 | 0 | 33 |
| `policy.rs` | 4 | 0 | 0 | 0 |
| `readiness.rs` | 1 | 0 | 2 | 0 |
| `runtime_service_metadata.rs` | 1 | 0 | 0 | 4 |
| `scrape.rs` | 3 | 1 | 0 | 0 |
| `service_role.rs` | 3 | 0 | 0 | 0 |
| `swarm_metadata.rs` | 4 | 1 | 0 | 1 |

`number_of_bytes.rs`, `peer_id.rs` and `lib.rs` produced no mutants.

### Missed

1. `swarm_metadata.rs:35`: replace `SwarmMetadata::zeroed` with `Default::default()`. **Equivalent**: the body is
   `Self::default()`. Selected action: none.
2. `scrape.rs:22`: replace `ScrapeData::empty` with `Default::default()`. **Equivalent**: the derived `Default` of
   `ScrapeData` is an empty `HashMap`, the same value `empty` builds. Selected action: none.
3. `configuration_instance_id.rs:37`: replace `ConfigurationInstanceId::instance_index` with `1`. **A real, cheap
   gap**: the only test that calls `instance_index()` builds the id with index `1`, the constant the mutant returns;
   the other tests compare whole ids and never call the accessor. Fixed after the maintainer approved it (2026-11-15):
   `it_should_expose_its_role_and_zero_based_index` now uses index `2` (commit
   `test(primitives): [#2824] test the instance index with a value no mutant returns`). A re-run on the file at
   2026-11-15T08:33Z (`--file packages/primitives/src/configuration_instance_id.rs`, output `.tmp/mutants-2824-cid`) gave
   **2 caught, 1 unviable**, so the mutant is now killed.

### Timeouts

`readiness.rs:32` (`is_ready` returns `true`) and `readiness.rs:39` (`mark_not_ready` does nothing): the concurrent
test's readers loop until they see "not ready", so both mutants hang it until the timeout (20 s). They are detected,
since the run fails, but by a hang, not a failed assertion. `it_should_not_be_ready_after_it_is_marked_not_ready` also
fails for both mutants; the timeout comes first only because the whole test binary is stopped. Selected action: none.
The test has no time bound of its own, so a regression would surface in CI as a hung job rather than a failure;
recorded here, not changed (the maintainer kept the test as it is in the S4 no-change record, Q3).

### Unviable

All 48 failed to build; none is a lint failure (see the configuration). The first compiler error of each per-mutant log
gives these kinds, which add up to 48:

- **No `Default` for the returned type** (30, `E0277`): `AnnounceIntervalInSecs` (6), the serializer's `Ok` type (6),
  `PeerId` (4), `PeerRole` (3), `AnnounceEvent` (2), `ServiceRole` (2), `SocketAddr` (2), and one each for
  `ConfigurationInstanceId`, `Driver`, `IpAddr`, `RuntimeServiceMetadata` and `Url`.
- **Non-`const` call in a `const fn`** (17, `E0015`): `Default::default()` and similar replacements inside the
  `const fn` accessors and constructors.
- **Conditionally-`const` call in a `const fn`** (1, `E0658`).

## Conclusion

Every behavior the decision records chose to protect kills its mutants. Of the three survivors of the package run, two
are equivalent and one was a cheap test gap, now closed (the file re-run kills it). With that fix the package has no
non-equivalent survivor.
