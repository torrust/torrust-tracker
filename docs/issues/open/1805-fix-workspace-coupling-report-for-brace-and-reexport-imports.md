---
doc-type: issue
issue-type: task
status: open
priority: p3
github-issue: 1805
spec-path: docs/issues/open/1805-fix-workspace-coupling-report-for-brace-and-reexport-imports.md
branch: "1805-fix-workspace-coupling-report-imports"
related-pr: null
last-updated-utc: 2026-05-20 00:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - contrib/dev-tools/analysis/workspace-coupling/src/main.rs
    - contrib/dev-tools/analysis/workspace-coupling/Cargo.toml
    - docs/issues/open/1669-overhaul-packages/workspace-coupling-report.md
---

<!-- skill-link: create-issue -->

# Issue #1805 - Overhaul workspace-coupling report tool: replace regex scanner with `syn` and adopt CLI output contract

## Goal

Replace the regex-based import scanner in the `workspace-coupling` analysis tool with a
`syn`-based Rust AST parser to correctly extract imported items from all `use` statement
forms, and bring the tool's CLI output into compliance with the global CLI output contract
(ADR `20260519000000_define_global_cli_output_contract`) by replacing plain-text `eprintln!`
calls with structured JSON NDJSON records on stderr.

## Background

The `workspace-coupling` tool (at
`contrib/dev-tools/analysis/workspace-coupling/src/main.rs`) uses a regex to extract imports:

```text
{module_name}::[A-Za-z_][A-Za-z0-9_]*(?:::[A-Za-z_][A-Za-z0-9_]*)?
```

This regex requires that the character after `::` is a letter or underscore (`[A-Za-z_]`). It
therefore misses at minimum two legitimate patterns:

1. **Brace-import groups**: `use torrust_tracker_contrib_bencode::{BMutAccess, ben_int, ben_map}`
   — after `::` there is `{`, which the regex does not match.
2. **Re-export statements**: `pub use bittorrent_peer_id::{PeerClient, PeerId}` — same issue.

When the regex matches nothing but the `has_any_reference` heuristic (a `\bMODULE\b` word
boundary check) detects the crate name, the tool emits:

> _Items not extracted — dependency used without a direct `use` path (macro, re-export, or glob
> import)._

This message is ambiguous and was confirmed to be a false negative in six cases where there are
clear, direct `use` statements:

| Package                            | Dep                               | Actual usage form                                  |
| ---------------------------------- | --------------------------------- | -------------------------------------------------- |
| `bittorrent-http-tracker-protocol` | `torrust-tracker-contrib-bencode` | `use crate::{BMutAccess, …}`                       |
| `bittorrent-http-tracker-protocol` | `torrust-tracker-located-error`   | `use crate::{Located, LocatedError}`               |
| `bittorrent-udp-tracker-core`      | `torrust-tracker-configuration`   | `use crate::{Core, UdpTracker}`                    |
| `bittorrent-udp-tracker-protocol`  | `bittorrent-peer-id`              | `pub use bittorrent_peer_id::{PeerClient, PeerId}` |
| `torrust-axum-server`              | `torrust-tracker-located-error`   | `use crate::{DynError, LocatedError}`              |
| `torrust-tracker-primitives`       | `bittorrent-peer-id`              | `pub use bittorrent_peer_id::{…}`                  |

Patching the regex for the known patterns (braces, re-exports) would fix the current failures
but leave the tool fragile against future Rust `use` idioms (nested paths, multi-line braces,
aliased imports). The chosen approach — replacing the regex scanner with `syn`-based AST
parsing — handles all valid `use` statement forms in one clean change.

Improving the scanner accuracy directly improves thin-dependency detection, which is the primary
purpose of the report.

### CLI output non-compliance

The `main` function currently writes plain text to stderr via `eprintln!`:

```rust
eprintln!("Running cargo metadata...");
eprintln!("cargo metadata failed:\n{}", ...);
eprintln!("Workspace root: {}", ...);
eprintln!("Output file: {}", ...);
eprintln!("Done.");
eprintln!("Report: {}", ...);
```

ADR `20260519000000_define_global_cli_output_contract` (section 1) requires that all stderr
records are JSON (NDJSON). Section 8 notes that `clippy::print_stderr` will be denied
workspace-wide once migration is complete — so these calls will break the build when that
lint is enabled.

The migration policy (section 10) states: _"Existing non-compliant commands are migrated
progressively when touched by new feature work."_ Since the rewrite already substantially
touches `main.rs`, applying the output contract here avoids a separate migration pass.

The tool classifies as **`no-stdout-result`**: it writes the Markdown report to a file, not
to stdout, so TTY refusal does not apply.

## Scope

### In Scope

- Replace the regex-based `scan_imports` function in
  `contrib/dev-tools/analysis/workspace-coupling/src/main.rs` with a `syn`-based AST visitor
  that walks every `.rs` file and collects all `use` paths referencing a given workspace
  dependency module.
- Add `syn` (with the `full` feature) to `contrib/dev-tools/analysis/workspace-coupling/Cargo.toml`.
- Handle all `use` statement forms: simple paths, brace groups, glob imports, and `pub use`
  re-exports.
- **Refactor for testability**: extract a pure function
  `parse_imports_from_source(source: &str, module_name: &str) -> BTreeSet<String>` so the
  import-extraction logic can be unit tested without filesystem I/O. `scan_imports` becomes a
  thin wrapper that reads files and calls it.
- **Unit tests**: add `#[cfg(test)]` tests in `src/` for `parse_imports_from_source` covering
  all four `use` forms (simple path, brace group, glob, `pub use` re-export) plus aliased
  imports. Written before the `syn` implementation (TDD).
- **Integration tests**: add a `tests/` directory with fixture `.rs` files and tests that
  invoke the binary (via `std::process::Command`) against a minimal fixture workspace,
  asserting correct report output. Written before the `syn` implementation (TDD).
- Add `tests/fixtures/` with a minimal fake workspace containing `.rs` files that exercise all
  `use` statement forms.
- Regenerate `docs/issues/open/1669-overhaul-packages/workspace-coupling-report.md` and verify
  the six previously missing entries now list the correct imported items.
- Replace all `eprintln!` progress and error messages in `main.rs` with JSON NDJSON records
  written to stderr, complying with ADR `20260519000000_define_global_cli_output_contract`.

### Out of Scope

- Glob imports (`use MODULE::*`) — items cannot be enumerated; recording `MODULE::*` as a single
  entry is acceptable.
- Switching the report generator to use `cargo metadata` for dependency resolution (separate
  concern, would overlap with the `cargo machete --with-metadata` work).
- Fixing the "No references found" (truly unused) entries — addressed by the
  `cargo machete --with-metadata` issue.
- Macro-generated imports or conditional compilation (`#[cfg(...)]`) — out of scope for a
  reporting-only tool.
- TTY refusal — not applicable; the tool writes its result to a file, not to stdout
  (`no-stdout-result` class under the ADR).
- Adding the tool to the ADR binary classification table — the tool lives under
  `contrib/dev-tools/` and is not a shipped binary; documenting it is deferred to the ADR
  migration issue.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

Tasks follow TDD order (tests written before implementation) and include manual run gates after
each step to confirm the tool still produces correct output at every inflection point.
ADR compliance comes first because it is non-functional and produces a clean, focused diff
before the scanner logic changes.

### Step 1 — ADR compliance: structured stderr output

| ID  | Status | Task                                                                                                  | Notes / Expected Output                                                          |
| --- | ------ | ----------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------- |
| T1  | TODO   | Replace all `eprintln!` calls in `main.rs` with JSON NDJSON records written to stderr                 | No bare `eprintln!` strings remain; each stderr line is a valid JSON object      |
| T2  | TODO   | **Manual gate**: run `cargo run -p workspace-coupling 2>.tmp/ws.stderr`, diff report against baseline | Report file byte-identical to before; every `.tmp/ws.stderr` line parses as JSON |

### Step 2 — Test infrastructure (TDD: tests before implementation)

Write tests first so they fail against the current regex implementation. The tests define the
expected behaviour of the `syn`-based scanner before a single line of it is written.

| ID  | Status | Task                                                                                                                                                        | Notes / Expected Output                                     |
| --- | ------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------- |
| T3  | TODO   | Refactor `scan_imports` into `parse_imports_from_source(source: &str, module: &str) -> BTreeSet<String>` (pure) + a thin `scan_imports` file-walker wrapper | Enables unit tests without filesystem I/O                   |
| T4  | TODO   | Add `tests/fixtures/` with minimal `.rs` files covering all `use` forms: simple, brace, glob, `pub use`, aliased                                            | Fixtures committed; used by both unit and integration tests |
| T5  | TODO   | Write unit tests for `parse_imports_from_source` using inline source strings — run `cargo test`, expect failures on brace/glob/pub-use cases                | Tests are red; define expected behavior                     |
| T6  | TODO   | Write integration tests in `tests/` that invoke the binary against the fixture workspace and assert report output — expect failures                         | Tests are red; cover end-to-end behavior                    |

### Step 3 — `syn` scanner implementation

| ID  | Status | Task                                                                                                    | Notes / Expected Output                                                                                           |
| --- | ------ | ------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------- |
| T7  | TODO   | Add `syn` (feature `full`) to `workspace-coupling/Cargo.toml`; remove the now-unused `regex` dependency | `cargo build -p workspace-coupling` succeeds; `cargo machete --with-metadata -p workspace-coupling` reports clean |
| T8  | TODO   | Rewrite `parse_imports_from_source` using `syn::visit`; record glob as `MODULE::*`                      | Unit and integration tests from Step 2 now pass (green)                                                           |
| T9  | TODO   | **Manual gate**: run tool against the real workspace, confirm six entries fixed                         | `grep "Items not extracted" <report>` returns zero for the six confirmed cases                                    |

### Step 4 — Report regeneration and final checks

| ID  | Status | Task                                                                              | Notes / Expected Output                                                          |
| --- | ------ | --------------------------------------------------------------------------------- | -------------------------------------------------------------------------------- |
| T10 | TODO   | Regenerate `docs/issues/open/1669-overhaul-packages/workspace-coupling-report.md` | Six previously "Items not extracted" entries now list the correct imported items |
| T11 | TODO   | Run `linter all`                                                                  | Exit code `0`                                                                    |

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted in `docs/issues/drafts/`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] GitHub issue created and issue number added to this spec
- [ ] Spec moved to `docs/issues/open/` with issue number prefix
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, `cargo test`)
- [ ] Manual verification scenarios executed and recorded (status + evidence)
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-05-20 00:00 UTC - josecelano - Spec drafted. Root cause identified: `scan_imports` regex
  does not handle `::{}` brace-imports or `pub use` re-exports. Six confirmed false-negative
  "Items not extracted" entries listed. Decision: replace regex with `syn`-based AST parsing
  after evaluating four approaches (regex patch, `syn`, rustc HIR, rust-analyzer — see
  Alternatives Considered). ADR compliance scope added: tool's `eprintln!` calls must become
  JSON NDJSON records (ADR section 1 + section 10 migration trigger). Testing scope added:
  unit tests for `parse_imports_from_source` and integration tests via `std::process::Command`
  against a fixture workspace.

## Acceptance Criteria

- [ ] AC1: The report no longer shows "Items not extracted" for the six confirmed cases; each
      entry lists the actual imported items.
- [ ] AC2: `pub use MODULE::Item` re-exports are captured and listed as `MODULE::Item`.
- [ ] AC3: Brace-import groups `use MODULE::{A, B}` are expanded to individual `MODULE::A`,
      `MODULE::B` entries.
- [ ] AC4: Glob imports appear as `MODULE::*` instead of triggering "Items not extracted".
- [ ] AC5: Unit tests for `parse_imports_from_source` covering all four `use` forms (simple,
      brace, glob, `pub use`) pass.
- [ ] AC6: All `eprintln!` progress and error messages emit a single JSON object per line
      on stderr (NDJSON); no plain-text strings remain.
- [ ] AC7: Integration tests in `tests/` invoke the binary against the fixture workspace and
      assert correct report output; all pass.
- [ ] AC8: `linter all` exits with code `0`.
- [ ] Manual verification scenarios are executed and documented (status + evidence).
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior.

## Verification Plan

### Automatic Checks

- `cargo test -p workspace-coupling`
- `cargo build --workspace` (verify `syn` dep does not break anything)
- `linter all`

> Note: `clippy::print_stderr` is not yet denied workspace-wide (pending ADR migration issue),
> but the implementation must not introduce new `eprintln!` bare-string calls regardless.

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                                                            | Command/Steps                                                                                           | Expected Result                                                       | Status | Evidence |
| --- | ------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------- | ------ | -------- |
| M1  | After Step 1: report unchanged, stderr is JSON                      | `cargo run -p workspace-coupling 2>.tmp/ws.stderr`; diff report against baseline; `jq . .tmp/ws.stderr` | Report identical to baseline; every stderr line parses as JSON        | TODO   |          |
| M2  | After Step 3: six confirmed entries now list actual items           | `cargo run -p workspace-coupling` then inspect report sections                                          | Sections for `torrust-tracker-contrib-bencode` etc. list actual items | TODO   |          |
| M3  | After Step 3: no spurious "Items not extracted" for confirmed cases | `grep "Items not extracted" docs/issues/open/1669-overhaul-packages/workspace-coupling-report.md`       | Zero matches for the six confirmed cases                              | TODO   |          |
| M4  | Integration test suite passes (unit + integration)                  | `cargo test -p workspace-coupling`                                                                      | All tests pass                                                        | TODO   |          |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | TODO                   |          |
| AC2   | TODO                   |          |
| AC3   | TODO                   |          |
| AC4   | TODO                   |          |
| AC5   | TODO                   |          |
| AC6   | TODO                   |          |
| AC7   | TODO                   |          |
| AC8   | TODO                   |          |

## Alternatives Considered

### Option 1 — Patch the existing regex (discarded)

Extend the current regex to also match `::{` brace groups and `pub use` prefixes.

**Why discarded**: fixing the regex for the two known failure modes leaves the scanner
fragile against future Rust `use` idioms (nested paths, aliased imports, multi-line brace
groups, conditionally compiled imports). Each new edge case requires another regex patch.
The incremental maintenance cost outweighs the low one-time effort of the proper fix.

### Option 2 — `syn` AST parsing (chosen)

Add the `syn` crate (feature `full`) and replace `scan_imports` with a `syn::visit`-based
AST walker.

**Why chosen**:

- Handles _all_ valid `use` syntax by construction — no per-pattern patches needed.
- Works on stable Rust with no nightly or unstable features.
- `syn` is a small, well-maintained, zero-runtime-overhead (compile-time only for proc-macros;
  here used as a library) crate with a stable API.
- A reporting-only dev tool is an appropriate context for it; it does not affect the
  workspace's main compilation.
- Glob imports (`use MODULE::*`) are representable as `UseGlob` in the AST — recordable as
  `MODULE::*` without special-casing.

**Trade-off**: adds one new dependency to the `workspace-coupling` crate; not a concern for a
dev-only tool not published to crates.io.

### Option 3 — rustc HIR / `rustc_private` (discarded)

Invoke the Rust compiler's High-level Intermediate Representation to resolve all imports with
full semantic knowledge (resolves re-exports transitively, understands macros, conditional
compilation, etc.).

**Why discarded**:

- Requires `#![feature(rustc_private)]` and a nightly toolchain.
- The `rustc_private` API is explicitly unstable and breaks between compiler versions.
- Invoking the compiler per crate makes the tool slow and requires a full build environment.
  The tool's goal is coupling _reporting_ (human-readable summary), not semantic analysis;
  full HIR accuracy is far beyond what is needed.

### Option 4 — rust-analyzer APIs (discarded)

Use `ra_ap_*` crates or the LSP interface of rust-analyzer to perform semantic queries.

**Why discarded**:

- `ra_ap_*` crates are unstable and version-pin to specific rust-analyzer releases.
- Starting a rust-analyzer instance adds significant latency and infrastructure complexity
  to a lightweight CLI tool.
- Same overkill argument as Option 3: the tool needs item-path listing, not full semantic
  resolution.

## Risks and Trade-offs

- `syn` parsing is syntactic, not semantic: it will not resolve re-exports transitively
  (i.e., if crate A re-exports from crate B, only the `pub use` statement in A's source is
  recorded, not the ultimate origin in B). This is acceptable for a coupling report — the
  goal is to enumerate what each package _declares_ it imports, not the full resolution chain.
- Macro-generated `use` statements are invisible to `syn` source-level parsing. This is an
  accepted limitation documented in the report's "How to read this report" section.

## References

- Related issues: #1669 (EPIC — Overhaul Packages)
- See also: #1804 (companion issue: `cargo machete --with-metadata` and unused dev dependency
  removal) — fixing the scanner's false negatives improves coupling report accuracy
  independently of that issue.
- Coupling report: `docs/issues/open/1669-overhaul-packages/workspace-coupling-report.md`
- Report tool: `contrib/dev-tools/analysis/workspace-coupling/src/main.rs`
- Global CLI output contract ADR: `docs/adrs/20260519000000_define_global_cli_output_contract.md`
- `syn` crate: <https://docs.rs/syn>
- `syn::visit` module: <https://docs.rs/syn/latest/syn/visit/index.html>
