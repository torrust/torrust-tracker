---
doc-type: manual-verification-evidence
issue-spec: docs/issues/closed/2576-1669-update-all-package-readmes/ISSUE.md
last-updated-utc: 2026-11-11 08:59
---

# Manual Verification Evidence

## Purpose

Real verification of the #2576 change, recorded from commands actually run on 2026-11-10.

## Environment and Prerequisites

- Date (UTC): 2026-11-10
- Artifact under test: the `i2576-update-package-readmes` branch of PR #2812 (commits named by
  their Conventional Commit subjects, for example `chore(workspace): remove the documentation
  field so crates.io links each crate's own docs.rs page`).
- Environment: Linux, stable Rust toolchain, local checkout and `curl` for URL checks.
- Setup: none beyond `cargo` and `jq`.

## Baseline (T2)

After #2482 closed (2026-11-10), `cargo metadata --no-deps --format-version 1` shows the tiers of
the 2026-10-20 check: 22 publishable packages (18 never published plus 4 published), 3
`publish = false` tracker packages and the developer tools. Before the change every tracker package
inherited `documentation = https://docs.rs/crate/torrust-tracker/`, 7 crates declared 6 or 7
keywords, 2 crates set categories and 3 crates declared `LGPL-3.0`.

## Verification Processes

### V1 - Metadata check (M-metadata; AC2, AC3)

- Goal: every tier U and P crate has a description, at most 5 keywords, valid categories, no
  `documentation`, and the right license identifier.
- Initial state: branch head after the review fixes.
- Status: `DONE`

#### Steps Performed

1. `cargo metadata --no-deps --format-version 1 | jq -r '.packages[] | select(.publish == null) | [.name, (.keywords | length), (.categories | length), (.documentation // ""), .license, (.description // "MISSING")] | @tsv'`
2. `grep -n '^documentation' Cargo.toml packages/*/Cargo.toml console/*/Cargo.toml`
3. Compared every category with `curl https://crates.io/api/v1/category_slugs` (104 slugs).

#### Observed Result

Columns: crate (prefix `torrust-tracker-` removed), keyword count, category count, `documentation`
(empty), license, description. Step 2 printed nothing and every category was in the slug list.

```text
client | 5 | 2 |  | LGPL-3.0-only | A collection of console clients to make requests to BitTorrent trackers.
client-lib | 5 | 2 |  | LGPL-3.0-only | A library with the generic tracker clients.
http-protocol | 5 | 2 |  | AGPL-3.0-only | Types and functions for the BitTorrent HTTP tracker protocol (BEP 3 and BEP 23).
udp-protocol | 5 | 2 |  | AGPL-3.0-only | Types and functions for the BitTorrent UDP tracker protocol (BEP 15).
axum-health-check-api-server | 5 | 1 |  | AGPL-3.0-only | Health check API server of the Torrust Tracker, used by the container health check.
axum-server | 5 | 2 |  | AGPL-3.0-only | A wrapper for the Axum server for Torrust HTTP servers to add timeouts.
configuration | 4 | 1 |  | AGPL-3.0-only | A library to provide configuration to the Torrust Tracker.
primitives | 5 | 2 |  | AGPL-3.0-only | A library with the primitive types shared by the Torrust tracker packages.
axum-http-server | 5 | 2 |  | AGPL-3.0-only | BitTorrent HTTP tracker server (BEP 3 and BEP 23) built on Axum for the Torrust Tracker.
core | 5 | 1 |  | AGPL-3.0-only | A library with the core functionality needed to implement a BitTorrent tracker.
events | 4 | 2 |  | AGPL-3.0-only | Asynchronous event bus used by the Torrust Tracker packages.
swarm-coordination-registry | 5 | 2 |  | AGPL-3.0-only | In-memory registry of the torrent swarms and peers tracked by the Torrust Tracker.
test-helpers | 4 | 1 |  | AGPL-3.0-only | A library providing helpers for testing the Torrust tracker.
http-core | 5 | 1 |  | AGPL-3.0-only | A library with the core functionality needed to implement a BitTorrent HTTP tracker.
axum-rest-api-server | 5 | 1 |  | AGPL-3.0-only | Axum transport layer of the Torrust Tracker management REST API.
rest-api-application | 4 | 1 |  | AGPL-3.0-only | Application and use-case layer of the Torrust Tracker management REST API.
rest-api-protocol | 5 | 1 |  | AGPL-3.0-only | Versioned contract types of the Torrust Tracker management REST API.
rest-api-client | 4 | 1 |  | LGPL-3.0-only | A library to interact with the Torrust Tracker REST API.
rest-api-runtime-adapter | 5 | 1 |  | AGPL-3.0-only | Tracker-specific runtime adapter for the REST API application layer.
udp-core | 5 | 1 |  | AGPL-3.0-only | A library with the core functionality needed to implement a BitTorrent UDP tracker.
udp-server | 5 | 2 |  | AGPL-3.0-only | BitTorrent UDP tracker server (BEP 15) for the Torrust Tracker.
torrust-tracker | 5 | 2 |  | AGPL-3.0-only | A feature rich BitTorrent tracker.
```

#### Conclusion

Met: 22 rows, every one with a description, 4 or 5 keywords, 1 or 2 valid categories and an
empty `documentation`; the three LGPL crates use `LGPL-3.0-only`. The keywords equal the table
approved by the maintainer and recorded in the spec (the rule lets the approved table list a
keyword that repeats a word of the crate name).

### V2 - Packaged files and links (M2; AC1)

- Goal: each tier U and P crate packages its README, and no README links a file outside the
  package with a relative path.
- Status: `DONE`

#### Steps Performed

1. `cargo package -p <crate> --list --allow-dirty | grep -x README.md` for the 22 crates.
2. `grep -n '](\.\./' packages/*/README.md console/*/README.md`
3. `linter all` (lychee local links and fragments), run by every commit hook.

#### Observed Result

```text
step 1: README.md listed for 22 of 22 crates
step 2: no output
step 3: exit 0 in every commit hook
```

#### Conclusion

Met. Links to files outside a package are absolute `https://github.com/torrust/torrust-tracker/...` URLs, checked in V3.

### V3 - Read-through and external links (M1; AC1, AC3)

- Goal: read each changed README as a new user and follow its links.
- Status: `DONE`

#### Steps Performed

1. Read each changed README as rendered Markdown source in the checkout (title, purpose, where it
   fits, main API, stability, documentation, license), comparing the module names in "Main API"
   with `pub mod` and `pub use` lines of the crate's `src/lib.rs` on this branch, which includes
   the #2482 results.
2. Requested every absolute URL in the changed READMEs with `curl -sL -o /dev/null -w '%{http_code}'`.
3. Requested `https://docs.rs/<crate>` for the 22 crates.

#### Observed Result

```text
step 2: HTTP 200 for the package catalog, the ADR, the shutdown process doc, the coverage evidence,
the MIT-0 text, the repository LICENSE, https://www.gnu.org/licenses/lgpl-3.0.html,
https://www.fsf.org/, the cargo-llvm-cov and Aquatic pages and the Coverage Gutters extension page
(the localhost URLs in the example payloads are not links to follow).
step 3: 200 for torrust-tracker, torrust-tracker-configuration, torrust-tracker-primitives and
torrust-tracker-test-helpers; 404 for the 18 never published crates (pending, see below)
```

#### Conclusion

Met, with one pending item: each tier U crate's own docs.rs link matches its crate name and
returns 404 until its first publish. The publish tracking issue #2583 checks that it resolves
after each publish. The console client README has no docs.rs link (out of scope by decision).

### V4 - Re-audit (M3; AC5)

- Goal: re-rate every package with the audit's scale.
- Status: `DONE`

#### Steps Performed

1. Re-read each README against the quality bar and recorded the result in the "Re-audit
   2026-11-10" section of [`readme-audit.md`](../../open/1669-overhaul-packages/readme-audit.md).

#### Observed Result

No tier U or P package is rated stub or minimal (the one minimal row is the tier N
`torrent-repository-benchmarking`).

#### Conclusion

Met.

### V5 - Build and doctests (AC8)

- Goal: the workspace builds without `documentation` and README doctests pass.
- Status: `DONE`

#### Steps Performed

1. `cargo build --workspace`
2. `cargo test --doc --workspace`
3. `grep -c '```rust' packages/*/README.md console/*/README.md`

#### Observed Result

```text
steps 1 and 2: pass; step 3: 0 in every file
```

#### Conclusion

Met: no README has a Rust fence, so no README doctest hook is needed or present.

## Per-AC evidence

- AC1: V2, V3. AC2: V1. AC3: V1, V3. AC4: V3 step 1 (module names checked against the post-#2482
  sources). AC5: V4. AC6: PR #2812 merged 2026-11-11 08:23 UTC and no crate was published. AC7: the template file and
  its row in `docs/templates/README.md` and `docs/index.md`. AC8: V5.

## Failures and Follow-up

Review finding F2 on PR #2812 found the first version of this file missing M1 and per-AC
evidence; V3 and the list above are the correction. No other scenario failed.
