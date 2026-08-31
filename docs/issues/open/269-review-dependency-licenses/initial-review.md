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
must not be treated as approval of every dependency. The direct runtime
`bloom` 0.3.2 finding was remediated by its removal in Issue 2114; the remaining
non-routine declarations require maintainer classification and qualified legal
review where required. Traceable authoritative-source evidence must be retained.
All active maintainers must then explicitly approve the protocol and recorded
outcome. This report is technical analysis, not legal advice.

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

| Finding                                                                                                                                           | Evidence   | Status   | Required disposition                                                                                                                              |
| ------------------------------------------------------------------------------------------------------------------------------------------------- | ---------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------- |
| Historical: `bloom` 0.3.2 declared `GPL-2.0` and was a direct normal dependency of `torrust-tracker-udp-core`, reaching tracker runtime packages. | E4, E5, E8 | RESOLVED | [Issue #2114](https://github.com/torrust/torrust-tracker/issues/2114) removed `bloom` and `bit-vec`; no current resolved dependency path remains. |
| `torrust-tracker-client` 0.1.0 declares `LGPL-3.0`.                                                                                               | E1, E2     | PENDING  | Classify distribution and licensing obligations; obtain maintainer approval or legal escalation.                                                  |
| `torrust-tracker-client-lib` 0.1.0 declares `LGPL-3.0`.                                                                                           | E1, E2     | PENDING  | Classify distribution and licensing obligations; obtain maintainer approval or legal escalation.                                                  |
| `torrust-tracker-rest-api-client` 0.1.0 declares `LGPL-3.0`.                                                                                      | E1, E2     | PENDING  | Classify distribution and licensing obligations; obtain maintainer approval or legal escalation.                                                  |
| `openmetrics-parser` 0.4.4 declares `LGPL-3.0` and is runtime-reachable through `torrust-metrics`.                                                | E1, E2, E7 | PENDING  | Classify distribution and licensing obligations; obtain maintainer approval or legal escalation.                                                  |
| `bencode2json` 0.1.0 declares `LGPL-3.0` in the complete locked graph.                                                                            | E1, E7     | PENDING  | Classify its locked-graph role and distribution obligations; obtain maintainer approval or legal escalation.                                      |
| `webpki-root-certs` 1.0.9 declares `CDLA-Permissive-2.0`.                                                                                         | E2, E4     | PENDING  | Classify the license and any applicable notices.                                                                                                  |
| `ring`, `aws-lc-sys`, `aws-lc-rs`, `encoding_rs`, and `unicode-ident` have conjunctive or mixed declarations.                                     | E1, E4     | PENDING  | Record selected paths, retained notices, and maintainer rationale.                                                                                |
| `workspace-coupling` lacked a declared license.                                                                                                   | E6         | RESOLVED | It now inherits the workspace `AGPL-3.0-only` declaration.                                                                                        |

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
     or the maintainer cannot confidently state the relevant facts.
   - **Create remediation work** to remove, replace, reconfigure, or stop
     distributing a dependency when its continued use cannot be approved.
   - **Propose an approved rationale** only after the required legal or policy
     review is complete; link the decision and any required notice-handling
     work.
6. Ask every active maintainer to approve or object to the recorded disposition.
   An approval confirms the documented project decision; it is not an individual
   legal opinion. A missing response leaves the finding pending.

For this first review, maintainers should process the five LGPL findings,
`webpki-root-certs`, and the mixed-expression group in the order recorded above.
No finding becomes approved merely because it appears in the inventory or
because its license text has been located.

## Prepared Factual Briefing

The following factual classification is complete. It is the technical context
needed for the decision checklist; it is not a compatibility conclusion.

| Finding                                 | Verified use and distribution context                                                                                                                                                                                                                                                                                                                                                                                                                        | Decision still required                                                                                                                   |
| --------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------- |
| Historical: `bloom` 0.3.2               | At the initial review snapshot, a normal `torrust-tracker-udp-core` dependency used by the UDP banning service. [Issue #2114](https://github.com/torrust/torrust-tracker/issues/2114) removed it after a focused Criterion comparison found the exact-map path faster; `bloom` and `bit-vec` are absent from the current locked graph.                                                                                                                       | Technical remediation is complete. Assess any obligation for already distributed releases separately if required.                         |
| `torrust-tracker-client` 0.1.0          | A workspace console-client package, explicitly `LGPL-3.0` and eligible for publication through `publish.workspace = true`. It normally depends on `bencode2json` and `torrust-tracker-client-lib`. The draft [client-extraction plan](../../drafts/1669-extract-torrust-tracker-client-to-standalone-repo.md) identifies its CLI binary as the primary artifact and intends a standalone repository; extraction remains blocked by unpublished dependencies. | Maintainer classification of the current separately distributed client artifact and legal escalation if needed; recheck after extraction. |
| `torrust-tracker-client-lib` 0.1.0      | A workspace client library, explicitly `LGPL-3.0` and eligible for publication through `publish.workspace = true`. It is used by the console client and tracker workspace packages.                                                                                                                                                                                                                                                                          | Maintainer classification of library distribution and legal escalation if needed.                                                         |
| `torrust-tracker-rest-api-client` 0.1.0 | A workspace REST client library, explicitly `LGPL-3.0` and eligible for publication through `publish.workspace = true`. It is a normal dependency of tracker workspace packages.                                                                                                                                                                                                                                                                             | Maintainer classification of library distribution and legal escalation if needed.                                                         |
| `openmetrics-parser` 0.4.4              | A normal dependency of `torrust-metrics`; its inverse dependency path reaches the tracker application and server packages.                                                                                                                                                                                                                                                                                                                                   | Maintainer classification of runtime distribution and legal escalation if needed.                                                         |
| `bencode2json` 0.1.0                    | A normal dependency of the publishable `torrust-tracker-client` console-client package. It is present in the complete locked graph; the current evidence does not show it as a main tracker runtime dependency.                                                                                                                                                                                                                                              | Maintainer classification of console-client distribution and legal escalation if needed.                                                  |
| `webpki-root-certs` 1.0.9               | Reached through `reqwest` and `rustls-platform-verifier`; normal dependency paths reach the tracker application, server packages, and client artifacts.                                                                                                                                                                                                                                                                                                      | Maintainer classification of runtime distribution and any notice-handling work.                                                           |
| `ring`, `aws-lc-sys`, and `aws-lc-rs`   | TLS dependencies reached through Rustls, Axum server, Reqwest, and related normal dependency paths that reach tracker runtime packages.                                                                                                                                                                                                                                                                                                                      | Maintainer classification of the compound expressions, bundled notices, and any legal escalation.                                         |
| `encoding_rs` and `unicode-ident`       | Transitive dependencies in the locked workspace graph with verified package license texts and non-routine conjunctive declarations.                                                                                                                                                                                                                                                                                                                          | Maintainer classification of the selected license paths, notices, and any legal escalation.                                               |

All package declarations, source links, and license-file evidence for these
facts are retained in [evidence.md](evidence.md). A maintainer only needs to
correct a factual statement above, record the resulting disposition, and seek
qualified legal help when the checklist requires it.

## Historical Request for Qualified Legal Review: `bloom`

Provide the following request and linked evidence to a qualified software
licensing professional when assessing a release that included `bloom` 0.3.2.
It was prepared before Issue 2114 removed the dependency. It does not block the
current dependency graph and does not ask maintainers to reach a legal
conclusion themselves.

### Factual Assumptions to Confirm

- Torrust Tracker is licensed as `AGPL-3.0-only`.
- At the initial review snapshot, `torrust-tracker-udp-core` declared `bloom`
  0.3.2 as a normal dependency.
- `bloom` 0.3.2 declared `GPL-2.0` and its bundled `LICENSE` is headed “GNU
  GENERAL PUBLIC LICENSE, Version 2, June 1991.”
- Its source files contain notices that state GPL version 2 “or (at your
  option) any later version.” Upstream clarification [issue #11](https://github.com/nicklan/bloom-rs/issues/11),
  requesting a metadata correction, remains unresolved; the repository appears
  inactive.
- At the initial review snapshot, the tracker compiled `bloom` into its UDP
  banning service, where it provided a counting Bloom filter. The dependency
  path reached the tracker application and server packages.
- Issue 2114 removed `bloom` and `bit-vec`; the current locked graph has no
  dependency path to either package.
- The project distributes source code, release binaries, and a container image.
  Confirm the actual release channels and any additional distributed artifacts
  before requesting the assessment.

### Questions for the Reviewer

1. Can the project distribute its source code, compiled tracker binaries, and
   container image under `AGPL-3.0-only` while including `bloom` 0.3.2 under its
   declared `GPL-2.0` license for this runtime use?
2. Do the source-file notices establish a GPL-2.0-or-later grant despite the
   published Cargo metadata, and may the project rely on that interpretation
   without a response from the apparently inactive upstream repository?
3. What obligations apply to each actual distribution channel, including source
   releases, binaries, and container images?
4. Does operating the tracker over a network change any relevant obligations?
5. If continued use is possible, what concrete license-text, notice,
   attribution, source-availability, or other actions must the project take?
6. If continued use is not possible under the current model, does the project
   need to replace `bloom`, use a differently licensed version, or make a
   project licensing decision?
7. Does the answer change for separately distributed client libraries or tools?

### Materials to Provide

- This report and [evidence.md](evidence.md), especially E4 and E5.
- The project license declaration in `Cargo.toml` and the repository `LICENSE`.
- `packages/udp-core/Cargo.toml` and
  `packages/udp-core/src/services/banning.rs`.
- The exact release process and distribution channels used by the project.
- Any proposed exception, replacement, relicensing, or notice-handling plan.

### Requested Deliverable

Ask for a written assessment that records the facts relied upon, a conclusion
for the stated distribution model, required compliance actions, and any
qualifications or facts that could change the answer. Link that assessment from
the `bloom` finding before maintainers choose its disposition.

## Required Actions

1. [Issue #2114](https://github.com/torrust/torrust-tracker/issues/2114)
   removed `bloom` after recording the decision and benchmark evidence in the
   UDP-core package ADR. Retain E4 and E5 as historical evidence for any review
   of releases that included the dependency.
2. Complete the [Maintainer Decision Checklist](#maintainer-decision-checklist)
   for all five LGPL, CDLA, and conjunctive findings.
3. Obtain explicit unanimous active-maintainer approval after the pending items
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
