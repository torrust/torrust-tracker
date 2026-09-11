# Evidence Ledger: Initial Dependency-License Review

> **Status:** Technical evidence complete; approval and legal review pending.
>
> **Issue contract:** [ISSUE.md](ISSUE.md)
>
> **Decision record:** [initial-review.md](initial-review.md)

This ledger records reproducible technical evidence. It does not establish a
legal compatibility conclusion.

## E1: Locked Workspace Inventory

- **Question:** What dependency-license declarations occur in the complete
  resolved workspace graph?
- **Status:** PASS
- **Method:** At commit `c30fbff4`, ran:

  ```sh
  cargo metadata --locked --format-version=1
  ```

  The input `Cargo.lock` SHA-256 was
  `4fc7f17ed1d348a4500ef3772c661cda43799f5ef44fb51514386d7b408156d4`.

- **Observation:** The resolved graph contains 575 packages. Every package now
  has a declared Cargo license expression after `workspace-coupling` inherited
  the workspace `AGPL-3.0-only` declaration. The complete command output is
  retained as a compact deterministic inventory in
  [`locked-license-inventory.json`](locked-license-inventory.json), SHA-256
  `d61456809c69d4dbd6ada3771d39469f1491ac5cb99498068714a67418d43f3e`.
- **Conclusion:** The command and lockfile identify the complete review scope.
  A declared expression is metadata, not proof of the full license obligations.
- **Report Links:** [Scope and Method](initial-review.md#scope-and-method) and
  [Findings](initial-review.md#findings).

### Retained Inventory Projection

The compact artifact retains every resolved package name, version, and declared
license while excluding unrelated Cargo metadata. It was generated with:

```sh
cargo metadata --locked --format-version=1 | jq \
  '{format: "cargo-metadata-license-inventory-v1", command: "cargo metadata --locked --format-version=1", packages: ([.packages[] | {name, version, license: (.license // "NOASSERTION")}] | sort_by([.name, .version]))}' \
  > locked-license-inventory.json
```

The runtime-oriented artifact used:

```sh
cargo license --avoid-dev-deps --json | jq \
  '{format: "cargo-license-runtime-inventory-v1", command: "cargo license --avoid-dev-deps --json", packages: ([.[] | {name, version, license}] | sort_by([.name, .version]))}' \
  > runtime-license-inventory.json
```

## E2: Runtime-Oriented Inventory

- **Question:** Which declarations are visible in a production-oriented view?
- **Status:** PASS
- **Method:** Ran `cargo license --avoid-dev-deps --json` with
  `cargo-license` 0.7.0.
- **Observation:** The inventory contains 510 packages, including one
  `GPL-2.0` package (`bloom`), three `LGPL-3.0` packages, one
  `CDLA-Permissive-2.0` package, and common permissive expressions. The full
  package-license inventory is retained in
  [`runtime-license-inventory.json`](runtime-license-inventory.json), SHA-256
  `40b63b6192d529032ad0dec68f2463356459608f8dd0cac2cea898fd2cf1fd4c`.
- **Conclusion:** This inventory corroborates the material findings but is not
  the complete review scope because development dependencies are excluded.
- **Report Links:** [Scope and Method](initial-review.md#scope-and-method) and
  [Findings](initial-review.md#findings).

## E3: Tool Identity and Existing Enforcement

- **Question:** Which tools produced the inventory, and does existing policy
  enforcement evaluate license compatibility?
- **Status:** PASS
- **Method:** Recorded `cargo-license` 0.7.0 and `cargo-deny` 0.19.9 from the
  installed Cargo subcommands. Ran `cargo deny check licenses`.
- **Observation:** Cargo-deny rejected packages because the repository has no
  configured license allowlist. Its existing `bans` check remains independent
  of license policy.
- **Conclusion:** Neither tool provides a legal compatibility verdict; no
  automated license policy is configured or introduced by this review.
- **Report Links:** [Automation Decision](initial-review.md#automation-decision).

## E4: Material External-License Metadata

- **Question:** Do the flagged external packages' installed primary manifests
  match the inventory declarations, and do they include license material?
- **Status:** PASS
- **Method:** Inspected each `Cargo.toml` and license files under
  `$CARGO_HOME/registry/src/index.crates.io-*` (common Unix default:
  `~/.cargo`) for `bloom` 0.3.2, `webpki-root-certs` 1.0.9, `ring` 0.17.14,
  `aws-lc-sys` 0.44.0, `aws-lc-rs` 1.18.0, `encoding_rs` 0.8.35, and
  `unicode-ident` 1.0.24.
- **Observation:** Each manifest matches the declared expression in the
  inventory. Each package includes one or more license files. `bloom` declares
  `GPL-2.0`; its `src/lib.rs`, `src/bloom.rs`, and `src/valuevec.rs` each carry
  a notice permitting redistribution and modification under GPL version 2 “or
  (at your option) any later version.” The remaining packages have the
  non-routine or conjunctive expressions listed in the report.
- **Published-source records:**
  - [`bloom` 0.3.2](https://crates.io/crates/bloom/0.3.2): upstream
    [repository](https://github.com/nicklan/bloom-rs), `LICENSE` (GPL-2.0), and
    unresolved [upstream clarification issue #11](https://github.com/nicklan/bloom-rs/issues/11).
    The issue requests a GPL-2.0-or-later metadata correction; it was opened on
    2024-12-10 and had no maintainer response, linked change, or published
    correction when checked on 2026-08-28. The upstream repository's most
    recent commit predates the review by approximately a decade.
  - [`webpki-root-certs` 1.0.9](https://crates.io/crates/webpki-root-certs/1.0.9):
    upstream [revision](https://github.com/rustls/webpki-roots/commit/0a553dbc8b3f18ea05c4f881cffa3f2d005d0d30),
    `LICENSE` (CDLA-Permissive-2.0).
  - [`ring` 0.17.14](https://crates.io/crates/ring/0.17.14): upstream
    [repository](https://github.com/briansmith/ring), `LICENSE`,
    `LICENSE-BoringSSL`, `LICENSE-other-bits`, and bundled `once_cell` and fiat
    license texts. Its published VCS record is marked dirty, so the versioned
    crates.io source is the immutable package artifact.
  - [`aws-lc-sys` 0.44.0](https://crates.io/crates/aws-lc-sys/0.44.0): upstream
    [revision](https://github.com/aws/aws-lc-rs/commit/f464440d1fd3983ce9fb023e9eaf1698530919a2),
    `LICENSE`, `aws-lc/LICENSE`, and `aws-lc/third_party/fiat/LICENSE`.
  - [`aws-lc-rs` 1.18.0](https://crates.io/crates/aws-lc-rs/1.18.0): upstream
    [tag](https://github.com/aws/aws-lc-rs/tree/v1.18.0), `LICENSE`.
  - [`encoding_rs` 0.8.35](https://crates.io/crates/encoding_rs/0.8.35):
    upstream [tag](https://github.com/hsivonen/encoding_rs/tree/v0.8.35),
    `LICENSE-APACHE`, `LICENSE-MIT`, and `LICENSE-WHATWG`.
  - [`unicode-ident` 1.0.24](https://crates.io/crates/unicode-ident/1.0.24):
    upstream [tag](https://github.com/dtolnay/unicode-ident/tree/1.0.24),
    `LICENSE-APACHE`, `LICENSE-MIT`, and `LICENSE-UNICODE`.
- **Conclusion:** The declared metadata is independently reproducible from the
  installed package sources and immutable published artifacts. The `bloom`
  source notices and Cargo metadata conflict, and upstream has not clarified
  the intended declaration. The texts and distribution obligations still need
  maintainer classification, and `bloom` needs qualified legal review.
- **Report Links:** [Findings](initial-review.md#findings) and
  [Required Actions](initial-review.md#required-actions).

## E5: Runtime Reachability

- **Question:** Is `bloom` reachable from tracker runtime packages?
- **Status:** PASS
- **Method:** Ran:

  ```sh
  cargo tree --locked --workspace --target all --edges all -i bloom
  ```

  Inspected `packages/udp-core/Cargo.toml`.

- **Observation:** `torrust-tracker-udp-core` declares `bloom = "0.3.2"` as a
  normal dependency. The inverse tree reaches the tracker application and
  server packages.
- **Conclusion:** The GPL-2.0 finding is not limited to a test or build-only
  dependency and must not receive a compatibility approval without qualified
  legal review.
- **Report Links:** [Findings](initial-review.md#findings) and
  [Required Actions](initial-review.md#required-actions).

## E8: `bloom` Technical Remediation

- **Question:** Does the current locked graph still include the `bloom` finding
  identified in E4 and E5?
- **Status:** PASS
- **Method:** Reviewed merged [Issue #2114](https://github.com/torrust/torrust-tracker/issues/2114), its package-local
  [decision record](../../../../packages/udp-core/docs/adrs/20260829204258_use_exact_ip_counters_for_udp_banning.md),
  and the merged implementation in PR #2119. Ran:

  ```sh
  cargo metadata --locked --format-version=1 | jq '[.packages[] | select(.name == "bloom" or .name == "bit-vec")] | length'
  cargo tree --locked --workspace --target all --edges all -i bloom
  ```

- **Observation:** Issue 2114 removed the runtime `bloom` dependency and its
  transitive `bit-vec` dependency after a focused Criterion comparison found
  the exact-map ban-counter path faster for the measured operations. The
  metadata query returned `0`; the inverse tree reported no matching `bloom`
  package.
- **Conclusion:** The `bloom` finding is technically remediated for the current
  locked graph. E4 and E5 remain historical evidence for the initial snapshot;
  this does not decide any obligation associated with releases that included
  `bloom`.
- **Report Links:** [Findings](initial-review.md#findings) and
  [Required Actions](initial-review.md#required-actions).

## E6: Internal Metadata Completion

- **Question:** Can the missing `workspace-coupling` license declaration be
  made explicit using the repository convention?
- **Status:** PASS
- **Method:** Added `license.workspace = true` to
  `contrib/dev-tools/analysis/workspace-coupling/Cargo.toml`, then ran
  `cargo check -p workspace-coupling` and repeated E1's missing-license query.
- **Observation:** The package compiles and resolves to `AGPL-3.0-only`; no
  resolved package is now missing a license declaration.
- **Conclusion:** The inventory metadata gap is closed. This does not resolve
  third-party license compatibility questions.
- **Report Links:** [Findings](initial-review.md#findings).

## E7: Additional LGPL-3.0 Packages

- **Question:** What evidence and disposition are required for each LGPL-3.0
  declaration not covered by the workspace-client grouping?
- **Status:** PASS
- **Method:** Queried the retained locked inventory, ran `cargo tree --locked
--workspace --target all --edges all -i openmetrics-parser`, and inspected
  package manifests and bundled license files. Verified the published package
  sources below.
- **Observation:** `openmetrics-parser` 0.4.4 is runtime-reachable through
  `torrust-metrics` and declares `LGPL-3.0`. Its immutable
  [published source](https://crates.io/api/v1/crates/openmetrics-parser/0.4.4/download)
  has Cargo checksum
  `e40a68c62e09c5dfec2f6472af3bd5e8ddf506fcf14c78ece23794ffbb874eca`,
  upstream [repository](https://github.com/sinkingpoint/openmetrics-parser),
  and `LICENSE` headed as LGPL version 3. `bencode2json` 0.1.0 is present in
  the complete locked graph and declares `LGPL-3.0`. Its immutable
  [published source](https://crates.io/api/v1/crates/bencode2json/0.1.0/download)
  has Cargo checksum
  `928290081480add37a5b8ce7777f1ad566a9ab3f44c4c485e4be0d259fe00e88`,
  upstream [repository](https://github.com/torrust/bencode2json), `LICENSE`,
  and `docs/licenses/LICENSE-MIT_0`; its manifest declares only `LGPL-3.0`.
- **Conclusion:** Both declarations and bundled text are reproducibly verified.
  Their distribution and obligations remain pending maintainer classification
  or qualified legal review; bundled files do not establish a selected license
  path or compatibility conclusion.
- **Report Links:** [Findings](initial-review.md#findings) and
  [Required Actions](initial-review.md#required-actions).
