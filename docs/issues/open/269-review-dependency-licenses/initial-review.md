---
review-date-utc: 2026-08-28
status: blocked-pending-legal-and-maintainer-approval
scope: complete-resolved-cargo-graph
input-lockfile: Cargo.lock
input-lockfile-sha256: 4fc7f17ed1d348a4500ef3772c661cda43799f5ef44fb51514386d7b408156d4
input-revision: c30fbff4
next-scheduled-review-utc: 2027-02-28
---

# Initial Dependency-License Review

## Decision Status

Technical inventory is complete, but the formal review is **in progress**. It
must not be treated as approval of every dependency: `bloom` 0.3.2 is a direct
runtime dependency declaring `GPL-2.0` and requires qualified legal review.
Traceable authoritative-source evidence for the non-routine declarations must
also be retained. All active maintainers must then explicitly approve the
protocol and recorded outcome. This report is technical analysis, not legal
advice.

## Scope and Method

The review covers all 575 packages resolved by the locked workspace graph,
including normal, build, development, target-specific, optional, and
transitive dependencies. It uses commit `c30fbff4` and the lockfile checksum
in this document's frontmatter.

The complete declared-license inventory was produced with:

```sh
cargo metadata --locked --format-version=1
```

The supporting production-oriented inventory was produced with:

```sh
cargo license --avoid-dev-deps --json
```

The tools were `cargo-license` 0.7.0 and `cargo-deny` 0.19.9. Evidence E1-E6
in [evidence.md](evidence.md) records the results, source checks, and their
limits. Cargo metadata and cargo-license report declarations; they do not
decide compatibility, obligations, or the selected path of a multi-license
expression.

## Review Protocol

Use this report structure for each twice-yearly review.

1. Record the reviewed commit, lockfile checksum, commands, tool versions, and
   complete resolved-graph inventory.
2. Verify missing, custom, conjunctive, copyleft, or otherwise non-routine
   declarations against installed package manifests and license or notice files.
3. Record each finding as approved, pending maintainer classification, or
   requiring qualified legal review. Never infer legal compatibility from an
   SPDX expression alone.
4. Open or link remediation issues for findings that cannot be approved, and
   document the precise exception rationale where one is proposed.
5. Identify active maintainers from the repository's current governance and
   request an explicit approval or objection from each on the GitHub issue or
   review thread. The review is approved only when every active maintainer has
   explicitly approved the protocol and outcome; a missing response is pending,
   not approval.
6. Schedule the next review six months later. Perform an interim review when a
   `Cargo.lock` change adds a dependency, changes a declared expression, or
   introduces a missing, copyleft, custom, or non-routine license declaration.

## Findings

| Finding                                                                                                                              | Evidence   | Status   | Required disposition                                                                                         |
| ------------------------------------------------------------------------------------------------------------------------------------ | ---------- | -------- | ------------------------------------------------------------------------------------------------------------ |
| `bloom` 0.3.2 declares `GPL-2.0` and is a direct normal dependency of `torrust-tracker-udp-core`, reaching tracker runtime packages. | E4, E5     | BLOCKED  | Obtain qualified legal review before approving continued use, an exception, or replacement.                  |
| `torrust-tracker-client` 0.1.0 declares `LGPL-3.0`.                                                                                  | E1, E2     | PENDING  | Classify distribution and licensing obligations; obtain maintainer approval or legal escalation.             |
| `torrust-tracker-client-lib` 0.1.0 declares `LGPL-3.0`.                                                                              | E1, E2     | PENDING  | Classify distribution and licensing obligations; obtain maintainer approval or legal escalation.             |
| `torrust-tracker-rest-api-client` 0.1.0 declares `LGPL-3.0`.                                                                         | E1, E2     | PENDING  | Classify distribution and licensing obligations; obtain maintainer approval or legal escalation.             |
| `openmetrics-parser` 0.4.4 declares `LGPL-3.0` and is runtime-reachable through `torrust-metrics`.                                   | E1, E2, E7 | PENDING  | Classify distribution and licensing obligations; obtain maintainer approval or legal escalation.             |
| `bencode2json` 0.1.0 declares `LGPL-3.0` in the complete locked graph.                                                               | E1, E7     | PENDING  | Classify its locked-graph role and distribution obligations; obtain maintainer approval or legal escalation. |
| `webpki-root-certs` 1.0.9 declares `CDLA-Permissive-2.0`.                                                                            | E2, E4     | PENDING  | Classify the license and any applicable notices.                                                             |
| `ring`, `aws-lc-sys`, `aws-lc-rs`, `encoding_rs`, and `unicode-ident` have conjunctive or mixed declarations.                        | E1, E4     | PENDING  | Record selected paths, retained notices, and maintainer rationale.                                           |
| `workspace-coupling` lacked a declared license.                                                                                      | E6         | RESOLVED | It now inherits the workspace `AGPL-3.0-only` declaration.                                                   |

Common permissive and dual-permissive declarations were included in the full
inventory but receive no blanket compatibility conclusion from this report.

## Maintainer Decision Checklist

For every `PENDING` or `BLOCKED` finding, a maintainer should record the
following facts in the finding's issue or review thread before asking for an
approval decision. This is a practical evidence checklist, not legal advice.

1. Identify the package, version, license expression, and evidence record that
   supports the declaration and license-text source.
2. Identify how the workspace uses it: which package declares it, whether it is
   a normal, build, or development dependency, and whether it reaches a binary,
   library, container image, client artifact, or developer-only tool that the
   project distributes.
3. Identify what the project distributes that includes or depends on it, and
   whether the distributed artifact contains the package's source, binaries,
   notices, or only uses it while building or testing.
4. Read the linked package license and notice files. Record factual obligations
   that are stated plainly, such as retaining a notice or license text. Do not
   infer unclear obligations or compatibility from the SPDX expression.
5. Choose and record one disposition:
   - **Escalate for qualified legal review** when the dependency is copyleft,
     the expression or distribution model is unclear, an exception is proposed,
     or the maintainer cannot confidently state the relevant facts. `bloom`
     requires this disposition.
   - **Create remediation work** to remove, replace, reconfigure, or stop
     distributing a dependency when its continued use cannot be approved.
   - **Propose an approved rationale** only after the required legal or policy
     review is complete; link the decision and any required notice-handling
     work.
6. Ask every active maintainer to approve or object to the recorded disposition.
   An approval confirms the documented project decision; it is not an individual
   legal opinion. A missing response leaves the finding pending.

For this first review, maintainers should begin with `bloom`, then process the
five LGPL findings, `webpki-root-certs`, and the mixed-expression group in the
order recorded above. No finding becomes approved merely because it appears in
the inventory or because its license text has been located.

## Prepared Factual Briefing

The following factual classification is complete. It is the technical context
needed for the decision checklist; it is not a compatibility conclusion.

| Finding                                 | Verified use and distribution context                                                                                                                                                                           | Decision still required                                                                                 |
| --------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------- |
| `bloom` 0.3.2                           | A normal dependency of `torrust-tracker-udp-core`, used by the UDP banning service's counting Bloom filter. Its dependency path reaches the tracker application and server packages.                            | Qualified legal review of continued runtime use, an exception, or replacement.                          |
| `torrust-tracker-client` 0.1.0          | A workspace console-client package, explicitly `LGPL-3.0` and eligible for publication through `publish.workspace = true`. It normally depends on `bencode2json` and `torrust-tracker-client-lib`.              | Maintainer classification of the separately distributed client artifact and legal escalation if needed. |
| `torrust-tracker-client-lib` 0.1.0      | A workspace client library, explicitly `LGPL-3.0` and eligible for publication through `publish.workspace = true`. It is used by the console client and tracker workspace packages.                             | Maintainer classification of library distribution and legal escalation if needed.                       |
| `torrust-tracker-rest-api-client` 0.1.0 | A workspace REST client library, explicitly `LGPL-3.0` and eligible for publication through `publish.workspace = true`. It is a normal dependency of tracker workspace packages.                                | Maintainer classification of library distribution and legal escalation if needed.                       |
| `openmetrics-parser` 0.4.4              | A normal dependency of `torrust-metrics`; its inverse dependency path reaches the tracker application and server packages.                                                                                      | Maintainer classification of runtime distribution and legal escalation if needed.                       |
| `bencode2json` 0.1.0                    | A normal dependency of the publishable `torrust-tracker-client` console-client package. It is present in the complete locked graph; the current evidence does not show it as a main tracker runtime dependency. | Maintainer classification of console-client distribution and legal escalation if needed.                |
| `webpki-root-certs` 1.0.9               | Reached through `reqwest` and `rustls-platform-verifier`; normal dependency paths reach the tracker application, server packages, and client artifacts.                                                         | Maintainer classification of runtime distribution and any notice-handling work.                         |
| `ring`, `aws-lc-sys`, and `aws-lc-rs`   | TLS dependencies reached through Rustls, Axum server, Reqwest, and related normal dependency paths that reach tracker runtime packages.                                                                         | Maintainer classification of the compound expressions, bundled notices, and any legal escalation.       |
| `encoding_rs` and `unicode-ident`       | Transitive dependencies in the locked workspace graph with verified package license texts and non-routine conjunctive declarations.                                                                             | Maintainer classification of the selected license paths, notices, and any legal escalation.             |

All package declarations, source links, and license-file evidence for these
facts are retained in [evidence.md](evidence.md). A maintainer only needs to
correct a factual statement above, record the resulting disposition, and seek
qualified legal help when the checklist requires it.

## Required Actions

1. Obtain and record qualified legal guidance for `bloom` before any
   compatibility decision or exception.
2. Complete the [Maintainer Decision Checklist](#maintainer-decision-checklist)
   for all five LGPL, CDLA, and conjunctive findings.
3. Obtain explicit unanimous active-maintainer approval after the blocked items
   have a recorded disposition. Until then, retain this report as blocked.
4. Create focused remediation issues for each unapproved finding. No automatic
   enforcement should be added as part of this review.

## Automation Decision

No sufficiently clear, maintainer-approved SPDX policy or exception process
exists yet. Therefore this review does not justify a follow-up enforcement
issue or configure `cargo deny check licenses` in hooks or CI. Reassess after
the outstanding legal and maintainer decisions establish deterministic rules.

## Recurrence

The next scheduled review is 2027-02-28. Use this report and
[evidence.md](evidence.md) as the template. An interim review is required for
the dependency changes specified in the review protocol.
