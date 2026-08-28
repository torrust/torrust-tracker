---
assessment-date-utc: 2026-08-28 10:35
scope: complete-resolved-cargo-graph
input-lockfile: Cargo.lock
input-revision: d745ec694b05d3b41c6455868239d159179741d5
status: preliminary-not-a-legal-opinion
---

# Preliminary dependency-license assessment

## Purpose and Limitations

This is initial technical triage, not the formal twice-yearly review and not
legal advice. It identifies license metadata requiring maintainer attention
before the full evidence-grounded review establishes conclusions or policy.

The assessment uses current local Cargo metadata and registry package manifests.
It does not determine legal compatibility, audit all distributed source files,
or establish that declared SPDX expressions are complete.

## Evidence Collected

| Evidence                                                                | Result                                                                                                                                                                                                     |
| ----------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `cargo license --avoid-dev-deps`                                        | Completed with `cargo-license` 0.7.0. It reports 23 AGPL-3.0 project packages, 324 Apache-2.0-or-MIT packages, one GPL-2.0 package (`bloom`), three LGPL-3.0 packages, and other expressions listed below. |
| `cargo metadata --locked --format-version=1`                            | Found 575 resolved packages, 37 distinct declared license expressions, and one package without a `license` field.                                                                                          |
| `cargo deny check licenses`                                             | Failed because `[licenses]` has no configured allowlist. This confirms the check is not configured; its rejections are not compatibility findings.                                                         |
| `cargo tree --locked --workspace --target all --edges all -i <package>` | Used to trace the non-routine package findings to workspace dependents.                                                                                                                                    |

The `cargo license` result excludes development dependencies by design. The
`cargo metadata` inventory covers the complete resolved graph and remains the
scope evidence for this assessment. Both commands used the repository revision
`d745ec694b05d3b41c6455868239d159179741d5`; the full formal review must also
record a checksum for the exact `Cargo.lock` input.

## Preliminary Findings

### Requires urgent maintainer review

| Package       | Declared license | Reachability                                                                                               | Why it needs review                                                                                                                                                  |
| ------------- | ---------------- | ---------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `bloom` 0.3.2 | `GPL-2.0`        | Direct normal dependency of `torrust-tracker-udp-core`; therefore reachable from tracker runtime packages. | A GPL-2.0-only dependency inside an AGPL-3.0-only project is a material licensing question. Do not assume compatibility or incompatibility without qualified review. |

Evidence: the local registry manifest declares `license = "GPL-2.0"`; the
dependency is declared directly in `packages/udp-core/Cargo.toml` and the
workspace dependency tree reaches the tracker application.

### Requires documented review

| Package or group                                                                              | Declared license                                                                  | Reachability                                                                                                | Review need                                                                                                        |
| --------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------ |
| `torrust-tracker-client`, `torrust-tracker-client-lib`, and `torrust-tracker-rest-api-client` | `LGPL-3.0`                                                                        | Workspace console/client packages.                                                                          | Confirm intended distribution and licensing relationship of LGPL client artifacts with the AGPL tracker workspace. |
| `webpki-root-certs` 1.0.9                                                                     | `CDLA-Permissive-2.0`                                                             | Transitive dependency of `reqwest` through `rustls-platform-verifier`; used by tracker and client packages. | Non-routine permissive license requiring evidence and policy classification.                                       |
| `ring` 0.17.14                                                                                | `Apache-2.0 AND ISC`                                                              | TLS dependency.                                                                                             | Conjunctive license expression; confirm retained notices and obligations.                                          |
| `aws-lc-sys` 0.44.0 and `aws-lc-rs` 1.18.0                                                    | Multiple conjunctive expressions including Apache-2.0, ISC, MIT, and BSD-3-Clause | TLS dependency path.                                                                                        | Complex multi-license metadata; verify upstream notices and obligations.                                           |
| `encoding_rs` 0.8.35 and `unicode-ident` 1.0.24                                               | Expressions combining permissive licenses with BSD-3-Clause or Unicode-3.0        | Transitive dependencies.                                                                                    | Record the selected license path and any notice requirements.                                                      |

### Metadata gap

| Package                    | Finding                   | Reachability                                                                            | Required action                                                                                    |
| -------------------------- | ------------------------- | --------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------- |
| `workspace-coupling` 0.1.0 | No Cargo `license` field. | Internal developer-analysis tool under `contrib/dev-tools/analysis/workspace-coupling`. | Add or document its intended license before the formal review can account for the whole workspace. |

## Preliminary Conclusion

There is no basis to say that the workspace is “mostly fine” from this
assessment alone. Most of the 575 resolved packages declare common permissive
or dual-permissive SPDX expressions, but the direct `GPL-2.0` dependency is a
significant unresolved item. The LGPL workspace artifacts, custom or complex
expressions, and missing internal metadata must also be reviewed.

No immediate claim of a confirmed license violation is made. The formal review
must collect stronger evidence, determine each dependency's distribution and
linkage context, obtain the required maintainer approval, and escalate the
`bloom` finding for qualified legal review before a final conclusion.

Installing `cargo-license` improved the reproducibility of the preliminary
inventory, but it did not change this conclusion. Its output is an inventory of
declared license expressions, not a compatibility verdict.

## Next Actions

1. Preserve reproducible inventory output with tool versions and the input
   `Cargo.lock` revision.
2. Obtain qualified legal guidance for the `bloom` GPL-2.0 finding before
   approving its continued use or planning its replacement.
3. Add a license declaration or other approved licensing record for
   `workspace-coupling`.
4. Research and record primary-source license texts and notices for every
   non-routine or ambiguous expression.
5. Define the recurring review template, approval process, and criteria for
   future automation.

## References

- Main specification: [`ISSUE.md`](ISSUE.md)
- `bloom` manifest: `$CARGO_HOME/registry/src/index.crates.io-*/bloom-0.3.2/Cargo.toml` (common Unix default: `~/.cargo`)
- Direct dependency declaration: `packages/udp-core/Cargo.toml`
- Missing metadata declaration: `contrib/dev-tools/analysis/workspace-coupling/Cargo.toml`
