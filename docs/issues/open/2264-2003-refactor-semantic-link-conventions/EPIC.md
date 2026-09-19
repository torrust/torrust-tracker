---
doc-type: epic
status: planned
epic: 2003
github-issue: 2264
spec-path: docs/issues/open/2264-2003-refactor-semantic-link-conventions/EPIC.md
epic-owner: null
last-updated-utc: 2026-09-19 08:03
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/skills/semantic-skill-link-convention.md
    - issue #2003
    - issue #2233
    - docs/issues/closed/2233-2003-tune-unified-pr-review-process/code-span-path-case-analysis.md
    - docs/issues/closed/2233-2003-tune-unified-pr-review-process/code-span-path-case-inventory.tsv
    - issue #2185
    - docs/issues/closed/2185-2003-triage-advisory-external-link-check-findings/external-link-baseline.md
    - docs/issues/open/2264-2003-refactor-semantic-link-conventions/external-link-check-residual-failures-2026-09-18.md
    - docs/external-snapshots/open-knowledge-format/0.2/SPEC.md
    - docs/external-snapshots/open-knowledge-format/0.2/PROVENANCE.md
    - .github/lychee-online.toml
    - .github/workflows/external-link-check.yaml
    - docs/testing.md
---

# EPIC #2264 - Refactor Semantic Link and Frontmatter Conventions

**Parent EPIC:** #2003 - Overhaul: Automation Tools and AI Agent Guardrails

## Goal

Define and progressively validate the next version of the repository's Markdown frontmatter,
semantic-link, and path-reference conventions. Deliver the work in that priority order, beginning
with an executable frontmatter source of truth because inconsistent field names, scalar types, and
allowed values are already causing conflicts between AI agents.

This is a child EPIC of #2003. The current branch and pull request contain planning documentation,
the pinned external evidence used by that planning, and the repository policy needed to preserve
that evidence. They do not implement validators or migrate documents. Later subissues may implement
a bounded prototype or an approved, replaceable validator when their specification justifies it.

## Background

The current convention document, `docs/skills/semantic-skill-link-convention.md`, has grown from a
small skill-link convention into a mixed source of truth for several related but distinct concerns:

- frontmatter metadata fields for issue and EPIC specs;
- frontmatter schema expectations and lifecycle statuses;
- semantic links between skills and repository artifacts;
- language-specific inline marker placement;
- issue and review-finding references;
- ADR and workflow cross-reference guidance.

The repository has now accumulated enough real usage to replace open-ended experimentation with a
stronger contract. Current documents use values and shapes that the prose schema does not describe
consistently, including multiple lifecycle vocabularies, date-only and date-time values, EPIC-only
fields on some EPICs but not others, and fields accepted by templates or workflows but absent from
the documented EPIC schema. A validator cannot resolve those disagreements by itself; the accepted
model must be made explicit first.

Issue #2233 exposed a concrete symptom of the broader problem. Markdown inline code spans can look
like repository paths, but they may represent current files, historical files, examples, globs,
templates, directories, comments, or other concepts. Treating every such span as a strict path would
create noise, while ignoring all of them leaves some stale references invisible to Lychee.

This suggests a more general design question: the repository needs typed, explicit references when
a relationship matters to automation or knowledge-graph construction, and lighter prose/path
conventions when the text is only explanatory.

## External Design Inputs

Use these references only as design input before specifying the repository convention. They are not
adopted standards for this repository, and citing them does not imply that this EPIC will adopt
their vocabulary, syntax, data model, or validation approach.

### Open Knowledge Format

The pinned local [Open Knowledge Format v0.2 specification](../../../external-snapshots/open-knowledge-format/0.2/SPEC.md)
is relevant because it defines a vendor-neutral, Git-native Markdown bundle with extensible YAML
frontmatter. The accompanying [provenance record](../../../external-snapshots/open-knowledge-format/0.2/PROVENANCE.md)
identifies the upstream revision and integrity hashes; the
[live upstream specification](https://github.com/GoogleCloudPlatform/open-knowledge-format/blob/main/SPEC.md)
remains the source for detecting later changes. Version 0.2 goes beyond the original minimal format
by defining optional provenance, trust, lifecycle, freshness, and attested-computation fields.

Potentially reusable parts include:

- keep documents readable with ordinary Markdown and reviewable with normal Git workflows;
- require a non-empty document `type` while allowing producer-defined types and extension fields;
- separate permissive core conformance from stricter producer-specific profiles;
- preserve unknown fields and consume unknown types on a best-effort basis;
- use standard Markdown links for ordinary navigation and tolerate incomplete knowledge graphs;
- use `resource` to identify the asset a document describes and `sources` for provenance;
- distinguish who generated content from who or what verified it;
- represent freshness with explicit-offset ISO 8601 timestamps and `stale_after`;
- version the portable base format while allowing domain profiles to evolve independently.

Material mismatches still require design work:

- OKF identifies documents with the path-derived concept ID, while Torrust issue numbers and other
  stable identifiers must survive file moves.
- OKF links are intentionally untyped and broken links remain conformant, while Torrust needs typed
  high-signal relations and repository-aware validation for selected references.
- OKF reserves `index.md` and `log.md` and generally requires frontmatter on every other Markdown
  concept; Torrust currently uses different README/index and optional-frontmatter conventions.
- OKF uses `type`, while Torrust currently uses `doc-type` and document-specific profiles.
- OKF lifecycle `status` means `draft`, `stable`, or `deprecated`, while Torrust issue specifications
  use workflow states such as `planned`, `in-progress`, and `done`.
- OKF core conformance deliberately accepts unknown fields and types. Torrust may need a stricter
  repository profile for known document classes while retaining permissive consumption elsewhere.

**Preliminary conclusion:** OKF v0.2 is a credible portable base or compatibility target, not merely
an informal source of ideas. Full repository conformance is not currently proposed because it would
conflict with existing document identity, lifecycle, index, frontmatter-presence, and link-validation
requirements. The frontmatter inventory must compare four outcomes: adopt OKF v0.2 plus a strict
Torrust profile, expose an OKF-compatible projection, adopt selected compatible field families
without claiming conformance, or reject adoption with recorded reasons. Rust may remain the
canonical implementation of the selected Torrust profile in every outcome. No final adoption
decision has been made.

### Programming as Theory Building

The gist [Programming as Theory Building](https://gist.github.com/onlurking/fc5c81d18cfce9ff81bc968a7f342fb1/13782bd47cf9e8f35ee25d1fd143dbf67f5544c9)
at revision `13782bd47cf9e8f35ee25d1fd143dbf67f5544c9` is relevant because it frames
documentation as a support for shared understanding, not as a complete replacement for human design
knowledge. Its useful ideas for this EPIC are:

- conventions should help future maintainers reconstruct why repository artifacts relate, not only
  where files are located;
- typed links are valuable when they preserve design intent and decision context;
- mechanical validation should protect important invariants without pretending every useful piece of
  documentation can be reduced to rules;
- historical records can be valuable precisely because they preserve the theory and context that
  existed at the time, even when current paths have changed.

The main caution for this repository is to avoid over-formalizing prose. A convention that forces
every explanatory path mention into a strict graph edge may reduce readability and create maintenance
noise without improving understanding.

A full local snapshot is intentionally not included. The gist contains a transcription of Peter
Naur's essay and additional material attributed in its discussion to Alistair Cockburn, but exposes
no license file or explicit redistribution grant. The repository-authored summary above is the
offline design input. If redistribution rights are verified later, an immutable copy may be added
under `docs/external-snapshots/` with provenance, integrity hashes, and the applicable license.

## Scope

### In Scope

- Split the current semantic-link convention into clearly owned convention documents.
- Define versioning rules for convention documents before incompatible schema or syntax changes.
- Extract and parse every tracked Markdown frontmatter block, then apply Torrust's universal
  envelope only to repository-owned document schemas. For externally governed `SKILL.md` and agent
  profiles, validate only the repository-owned `metadata.semantic-links` extension.
- Define strict typed profiles progressively, starting with issue and EPIC specs and then covering
  ADRs, skills, agents, evidence records, and other known document classes.
- Use Rust types as the canonical executable representation for frontmatter metadata, semantic
  links, target types, relation types, and validation constraints.
- Generate a machine-readable schema from the canonical Rust types for editor, agent, and external
  tool consumption; the generated artifact must not become a separately authored source of truth.
- Accept unknown repository-owned document classes during migration while validating the universal
  envelope and applying strict rules to known profiles.
- Define a normalized semantic-link model suitable for building a repository knowledge graph.
- Decide how typed semantic links differ from ordinary path references in prose.
- Decide whether path references need their own explicit syntax and validator separate from semantic
  links.
- Coordinate validator implementation boundaries with EPIC #2003 before choosing whether
  frontmatter, semantic-link, or path-reference validators live in standalone scripts, dedicated
  Rust binaries, or a future unified AI-harness application.
- Define progressive migration and validation subissues.
- Preserve compatibility rules for existing documents during migration.
- Permit a small, read-only, replaceable frontmatter validator to proceed as approved early work
  under #2003 without selecting the long-term automation runner or AI-harness architecture.

### Out of Scope

- Migrating all existing documents in the EPIC issue itself.
- Implementing validators in the specification-only branch or pull request that creates this EPIC.
- Requiring frontmatter on every Markdown document during the first validation stage.
- Requiring strict typed profiles for every existing document class during the first validation
  stage.
- Rewriting historical records only to satisfy new syntax.
- Replacing Lychee as the validator for normal Markdown links.
- Requiring every prose path mention to become a semantic link.
- Selecting the long-term AI-harness architecture, command surface, cache model, or division between
  guardrail checks and repository-mutating actions; those decisions belong to EPIC #2003.

## Proposed Convention Split

The EPIC should decide final names and locations, but this starting split separates concerns:

| Area | Proposed responsibility | Candidate output |
| ---- | ----------------------- | ---------------- |
| Frontmatter metadata | Universal envelope, document profiles, required fields, lifecycle statuses, scalar types, version fields, schemas, and validation expectations. | `docs/conventions/frontmatter-metadata.md` |
| Semantic links | Typed relationships between project concepts, with relation names, target types, and graph semantics. | `docs/conventions/semantic-links.md` |
| Typed model and validators | Canonical Rust types plus generated machine-readable schema for document metadata, semantic-link targets, relations, parsing, diagnostics, and staged enforcement. Initial validator placement is replaceable and must remain compatible with EPIC #2003's automation design. | An existing check tier initially; final package or AI-harness component decided by #2003 |
| Path references | Syntax and validation rules for machine-checkable repository paths in prose, distinct from ordinary Markdown links. | `docs/conventions/path-references.md` |
| Marker placement | Language-specific syntax for comments, frontmatter, and inline markers outside Markdown. | `docs/conventions/reference-placement.md` |
| Migration policy | Versioning, compatibility windows, staged validation, and historical-document handling. | `docs/conventions/convention-migrations.md` |

This split may also require deciding whether `docs/skills/` is still the right folder for these
conventions. The current path is historical: the document began as a skill-link convention, but its
scope now extends beyond skills.

## Representation Decision

Rust types are the canonical executable source of truth. Serde-based parsing can express document
variants, scalar types, optionality, and custom validation entry points while remaining directly
reusable by a future Rust automation or AI-harness binary selected by #2003.

A generated machine-readable schema should expose the supported structure to editors, AI agents,
and non-Rust consumers. It is a derived compatibility artifact, not a second hand-maintained
contract. Generation and drift verification belong to a later implementation subissue.

Prose remains necessary to explain intent, migration policy, and constraints that are inappropriate
for mechanical enforcement. It must refer to the canonical model rather than duplicate field lists
that can drift.

The first model must separate three validation layers:

1. YAML/frontmatter syntax and scalar representation.
2. Structural conformance to the universal envelope and a known document profile.
3. Repository semantics such as path existence, lifecycle/path consistency, skill existence, and
  cross-document relationships.

This separation allows fast string and type feedback without coupling the schema to repository I/O
or to #2003's future orchestration architecture.

Candidate target types:

- `path`: repository-relative file or directory path;
- `markdown-section`: document plus heading or anchor;
- `issue`: GitHub issue number;
- `epic`: repository EPIC issue number;
- `adr`: ADR identifier or path;
- `skill`: Agent Skill name;
- `agent`: repository agent name;
- `review-finding`: normalized PR review finding reference;
- `commit`: Git commit object;
- `rust-module`: Rust module path;
- `rust-item`: Rust type, trait, function, constant, or module item.

Candidate relation types:

- `implements`;
- `specified-by`;
- `informed-by`;
- `supersedes`;
- `depends-on`;
- `related-artifact`;
- `validates`;
- `generated-by`;
- `governs`.

The EPIC should decide whether these names are sufficient, too broad, or too specific.

## Path References Versus Semantic Links

A path reference points to a repository file or directory. It helps a reader navigate or run a
command, but it may not describe a durable relationship between concepts.

A semantic link is a typed relationship between concepts. It should be high signal enough to support
queries, validation, impact analysis, and knowledge-graph construction.

Initial rule to evaluate:

- Use semantic links when the relationship matters to repository knowledge or automation.
- Use normal Markdown links for reader navigation.
- Use plain inline code paths for commands, examples, and local prose.
- Use a typed path-reference syntax only when the author wants machine path validation without
  declaring a semantic relationship.

Example candidate syntax to evaluate:

```text
path:docs/templates/REVIEW-FINDINGS.md
markdown-section:docs/skills/semantic-skill-link-convention.md#marker-catalog
review-finding:pr-2232-f4
adr:20260821172000
rust-module:torrust_tracker_udp_core::services::banning
```

The EPIC should not adopt this syntax without comparing it against existing repository conventions
and parser complexity.

### Provisional Reference Convention

Until the semantic-link subissues decide the final model, frontmatter work uses a frozen v1
reference convention so the first validator does not depend on the later ontology:

- `semantic-links.skill-links`: skill names only.
- `semantic-links.related-artifacts`: only the forms the current convention already sanctions,
  namely repository-relative file or directory paths, `issue #<number>`, and
  `review-finding:pr-<number>-<id>`.
- A document's own issue and EPIC relationships live in dedicated fields such as `github-issue`
  and `epic`, not in `related-artifacts`.
- No new typed prefixes are introduced by frontmatter work. The semantic-link subissues own any
  replacement and its migration.

## Relationship to EPIC #2003

This is a child EPIC of #2003. It owns the domain contract: field names, scalar types, document
profiles, relation semantics, accepted and rejected examples, compatibility rules, validation
behavior, and migration stages.

EPIC #2003 owns the shared automation architecture: final binary and package placement, common
command and event contracts, policy composition, caching, and integration across local hooks, CI,
and agent workflows. This EPIC must not preselect those decisions.

This EPIC may produce deterministic validators for Markdown frontmatter, semantic links, and typed
path references. Those validators are checks in the terminology of
[EPIC #2003](../../open/2003-overhaul-guardrails-and-automation/EPIC.md): they should be read-only by
default, return stable diagnostics, and be callable from local hooks, CI, and agent workflows.

Their implementation may eventually belong in the same Rust application or shared libraries as the
AI harness proposed by #2003, but this EPIC should not decide that architecture by accident. The
convention work should specify the data model, validation semantics, rollout stages, and diagnostics
well enough that #2003 can later choose the execution surface.

The initial frontmatter validator is approved early child work because the current ambiguity is
already affecting document authors. Pre-commit is its single approved integration tier; it may also
accept direct paths and support manual whole-tree validation. CI integration, shared runners,
caching, policy composition, and broader orchestration remain deferred to #2003. Its validation
semantics and fixtures should survive later relocation into the architecture chosen by #2003.

The boundary matters because #2003 also covers repository actions such as dependency updates and
completed-issue cleanup. Frontmatter and semantic-link validation may share parsing, repository
discovery, JSONL/NDJSON reporting, and configuration loading with those actions, but it must remain
clear which commands are read-only checks and which commands mutate the repository.

## Progressive Subissues

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| Order | Phase | Subissue | Expected output | Status |
| ----- | ----- | -------------- | --------------- | ------ |
| 1 | Frontmatter | [#2265 - Inventory and resolve Markdown frontmatter contracts](../2265-2264-inventory-markdown-frontmatter-contracts/ISSUE.md) | Map current prose, templates, skills, fields, scalar forms, document classes, and provisional reference values; classify each variation and approve the v1 contract. | TODO |
| 2 | Frontmatter | [#2266 - Implement the Rust frontmatter model and initial validator](../2266-2264-implement-rust-frontmatter-model-and-validator/ISSUE.md) | Implement the canonical universal envelope and strict issue/EPIC profiles, generated schema, provisional reference union, fixtures, stable diagnostics, and replaceable progressive validator. | BLOCKED |
| 3 | Frontmatter | Extend strict profiles and author guidance | Add approved profiles for ADRs, skills, agents, evidence records, and other known classes; keep prose and editor/agent discovery derived from the canonical Rust model. | TODO |
| 4 | Conventions | Split convention ownership and migration policy | Move mixed normative content to clearly owned convention documents without a repository-wide document migration. | TODO |
| 5 | Semantic links | Normalize the semantic-link model | Canonical relation and target types, graph semantics, accepted/rejected fixtures, and compatibility rules represented by the shared Rust model. | TODO |
| 6 | Semantic links | Design and implement semantic-link validation | Validate target syntax, type, existence where applicable, uniqueness, and invalid combinations through a replaceable check. | TODO |
| 7 | Path references | Decide path-reference scope and syntax | Decide whether machine-checkable prose paths are semantic links, a separate typed reference, or intentionally outside validation. | TODO |
| 8 | Path references | Specify and implement path-reference validation | If the path-reference scope issue approves validation, define historical handling, exclusions, diagnostics, and staged enforcement. | TODO |
| 9 | External links | Classify link importance and checker limitations | Use the #2185 handoff to decide whether link importance belongs in typed metadata, prose policy, or checker configuration. | TODO |
| 10 | Integration | Reconcile validators with the #2003 architecture decision | Preserve domain semantics while migrating checks to the selected execution, output, policy, and caching architecture when required. | TODO |

## Relationship to Issue #2233

Issue #2233 should not solve the full convention problem. Its code-span path inventory is evidence
for the path-reference scope and validation subissues in this EPIC.

The #2233 T2 finding can be resolved by recording that strict Markdown code-span path enforcement is
deferred until this EPIC defines whether path references are semantic links, a separate typed
reference form, or ordinary prose outside validator scope.

## Handoff from Issue #2185

Issue [#2185](https://github.com/torrust/torrust-tracker/issues/2185) set out to clean the broken
links reported by the advisory
[External Link Check workflow](../../../../.github/workflows/external-link-check.yaml). It started
from 461 errors, repaired every genuinely stale reference it found, and added only exact,
commented exclusions to [`.github/lychee-online.toml`](../../../../.github/lychee-online.toml). It
closed with seven persistent errors that no cleanup step can remove and four first-observed
timeouts that still require a confirming rerun. Those cases are
preserved verbatim in
[`external-link-check-residual-failures-2026-09-18.md`](external-link-check-residual-failures-2026-09-18.md)
and are input for the external-link classification subissue. No solution is chosen here.

### What the triage established

- **Most "broken" links were not broken.** 424 of the 461 baseline errors were GitHub
  pull-request review-comment anchors (`#discussion_r<id>`), and a later class of 23 errors were
  `#issuecomment-<id>` and `#pullrequestreview-<id>` anchors on pull-request URLs. GitHub renders
  these anchors client-side, so Lychee's `include_fragments = "full"` check cannot see them even
  though every target exists. The same applies to two GitHub issue-comment anchors and to the
  Star History project selector.
- **Dynamic anchors are generated by a repository process.** The review-comment anchors come from
  the PR review records under `docs/pr-reviews/` and the older `docs/copilot-pr-reviews/`. Any
  future process that writes those records will keep producing links the checker cannot validate,
  so the exclusion pattern is a standing policy, not a one-off cleanup.
- **A `403` does not mean stale.** Medium and Stack Overflow return `403` to automated clients
  while the content still exists. Excluding them hides real link rot on those hosts; keeping them
  produces permanent noise.
- **A transport failure can be a checker limitation.** `www.fsf.org` serves only finite-field
  `DHE` cipher suites; rustls, which Lychee uses, supports `ECDHE` only. The site is live and the
  certificate is valid, but Lychee can never reach it. The affected links are AGPL license
  boilerplate repeated in four README files.
- **Rerun-first works, but only for genuinely transient cases.** A `martinfowler.com` timeout in
  one run disappeared in the next; the `403` and FSF failures persisted across every run. The
  policy in [`docs/testing.md`](../../../../docs/testing.md) is sufficient to separate the two, but
  it has no answer for the persistent non-stale cases.
- **Counts across runs are not comparable.** Every merge changed the checked document set, so the
  issue had to verify each remediation slice by the absence of its exact URLs, not by the error
  count.
- **Repository-owned stale links were few and cheap to fix.** 14 docs.rs package pages, three
  repository paths, one Caddy page, and two retired Docker Cloud pages were the only real rot in
  the baseline.
- **Some important external context may need to live in the repository.** If an unstable external
  URL carries critical knowledge for maintainers or AI agents, the policy may need a third option
  beyond "keep checking" and "exclude": preserve a reviewed copy, excerpt, or simpler distilled
  form in the repository, then treat the original URL as provenance, background reading, or an
  online-check exception. This is only appropriate when the copied material is necessary,
  maintainable, and legally reusable; otherwise the project should prefer a stable replacement
  source or a concise in-repo summary.

### Why the remaining cases are a design question

The residual failures are all license boilerplate or optional background reading, but the
repository currently has no vocabulary to say so. `lychee-online.toml` can only express "check
this URL" or "never check this URL", while the actual distinction is between:

- links whose target the project owns or depends on and should keep checking strictly;
- links that are citations or background reading, where a `403` from a bot-protected host is
  acceptable and a replacement citation would be a policy choice;
- critical references whose contents should remain queryable even when the source site is unstable,
  slow, access-controlled, or unsuitable for automated checking;
- links whose target is reachable by users but not by the checker, where the failure describes the
  tool rather than the document;
- links inside historical or generated records that should never be rewritten to satisfy a
  checker.

These are the same distinctions this EPIC needs for semantic links versus ordinary Markdown links.
Ordinary links and typed links are not separate concerns: the importance, ownership, and lifecycle
of a link decide how it should be validated. The external-link classification subissue should use
the residual report and the #2185
baseline as its evidence and may conclude that the answer is prose policy, a small typed
annotation, or a different checker configuration; the EPIC does not presume which.

## Open Questions

- Which existing frontmatter variations are intentional compatibility cases, and which are errors
  that should be rejected prospectively?
- How should the universal envelope identify document classes that currently have no `doc-type`?
- Should experimental fields use a reserved namespace, remain warning-only, or require a schema
  change before use?
- Which constraints belong in Rust deserialization, post-deserialization domain validation, or
  repository-aware validation?
- Which generated schema format best serves editors and AI agents without becoming a second source
  of truth?
- Should semantic-link target types be URI-like strings, YAML maps, or both?
- Should a file path be represented as `path:...`, a typed YAML object, or a plain string in selected
  fields?
- Which references should be considered knowledge-graph edges, and which are only validation aids?
- Should historical closed issues and PR review records be exempt from new reference syntax?
- How should comments in Rust, shell, TOML, YAML, and Dockerfiles represent typed links without
  harming the host language parser?
- Which OKF principles should be copied directly, adapted, or rejected for a repository whose
  knowledge graph includes code, issues, review findings, ADRs, commits, and Markdown sections?
- Should Torrust define an OKF v0.2 profile, expose only an OKF-compatible projection, or remain a
  separate format, particularly given the `type`/`doc-type`, lifecycle-status, concept-identity,
  reserved-filename, and broken-link differences?
- Which references preserve design theory and intent, and which are only local navigation aids?
- Should frontmatter, semantic-link, and path-reference validators be standalone scripts, dedicated
  Rust binaries, library code consumed by a future AI harness, or subcommands of a unified
  repository automation application?
- If a unified AI harness is built, should it contain only read-only sensors and guardrail checks
  used by pre-commit, pre-push, CI, and agent review, or should it also expose repository actions
  such as dependency updates and completed-issue cleanup?
- Which validation semantics must be decided here, and which execution, caching, output, and
  migration choices should be deferred to EPIC #2003?
- Should link importance (owned dependency, citation, background reading, license boilerplate) be
  expressed in the typed model, in prose policy, or only in checker configuration?
- When an external source is important but unstable, should the repository preserve a vetted copy,
  excerpt, or distilled context and either remove the URL from checked documents or exempt it from
  online Lychee analysis?
- How should the advisory online check report a link that users can reach but the checker cannot
  (bot protection, TLS incompatibility) without either hiding link rot or producing permanent noise?
- Should links inside generated records such as `docs/pr-reviews/` be exempt from online validation
  as a class rather than through per-pattern exclusions?

## Acceptance Criteria

- [ ] AC1: The responsibilities currently mixed in `docs/skills/semantic-skill-link-convention.md`
      are inventoried and assigned to future convention documents or deliberately retired.
- [ ] AC2: A convention versioning and compatibility policy is specified before migrations begin.
- [ ] AC3: Frontmatter metadata conventions are specified separately from semantic-link conventions.
- [ ] AC4: Canonical Rust types define the universal frontmatter envelope and strict known document
      profiles, beginning with issue and EPIC specs.
- [ ] AC5: A generated machine-readable schema exposes the canonical model without creating a
      second hand-maintained source of truth.
- [ ] AC6: All present Markdown frontmatter can be extracted and parsed; Torrust-owned profiles and
  the repository-owned extension of externally governed profiles are checked while strict
  profile enforcement and frontmatter-presence requirements are introduced progressively.
- [ ] AC7: The first replaceable frontmatter validator provides useful field, scalar-type, and
      allowed-value diagnostics without selecting #2003's long-term automation architecture.
- [ ] AC8: Semantic-link relation types and target types are normalized enough to support
      validation and knowledge-graph construction.
- [ ] AC9: The EPIC decides whether machine-checkable path references are semantic links, a
      separate reference type, or intentionally out of scope.
- [ ] AC10: Progressive implementation subissues are created for validation and migration work.
- [ ] AC11: The final plan preserves this EPIC's ownership of validation semantics and #2003's
      ownership of shared execution architecture, integration policy, event contracts, and caching.
- [ ] AC12: Historical records have an explicit no-rewrite or migration policy.
- [ ] AC13: The final plan states which existing validators, including Lychee, remain responsible
      for ordinary Markdown links.
- [ ] AC14: The final plan decides whether Torrust adopts OKF v0.2 with a repository profile,
      exposes a compatible projection, selectively adopts fields without conformance, or remains
      separate, with each material compatibility difference addressed.
- [ ] AC15: The final plan records how theory-building input affected the chosen convention
      boundaries.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1-AC3 | TODO | Inventory subissue and convention-split subissue outputs |
| AC4-AC7 | TODO | Rust model and validator subissue outputs |
| AC8-AC9 | TODO | Semantic-link and path-reference subissue outputs |
| AC10 | TODO | Linked subissues in the Progressive Subissues table |
| AC11 | TODO | Cross-references in this EPIC and in #2003 |
| AC12-AC13 | TODO | Convention-split and migration-policy subissue outputs |
| AC14 | TODO | Inventory subissue OKF comparison |
| AC15 | TODO | Convention-split subissue rationale |

## Delivery Strategy

Deliver frontmatter first because its ambiguity is causing conflicts now, and because the typed
model it produces is the substrate the later semantic-link and path-reference work extends.
Each subissue is independently verifiable and merges through its own PR.

For each completed subissue, the default completion policy is:

1. Run automatic checks (`linter all`, relevant tests, pre-push checks when applicable).
2. Run manual verification scenarios and record evidence.
3. Re-review acceptance criteria after implementation and update verification evidence.
4. Complete an evidence-based implementation review and record whether a retrospective was needed.

### Phase 1: Frontmatter contract and validator

- Outcome: an approved v1 frontmatter contract, canonical Rust types, a generated schema, and a
  replaceable read-only validator integrated at one existing tier.
- Exit criteria: subissues 1 and 2 are done; new issue and EPIC specs are validated mechanically;
  the validator's fixtures and diagnostics are independent of hook or CI orchestration.

### Phase 2: Convention ownership and remaining profiles

- Outcome: the mixed convention document is split by ownership; strict profiles exist for the
  remaining known document classes; historical-document policy is explicit.
- Exit criteria: subissues 3 and 4 are done; no normative rule lives only in the retired document.

### Phase 3: Semantic links, path references, and external links

- Outcome: a normalized semantic-link model, a decision on path references, and a policy for link
  importance, each represented in the shared Rust model where appropriate.
- Exit criteria: subissues 5 through 9 are done; the provisional reference convention is either
  confirmed or superseded with a recorded migration.

### Phase 4: Reconciliation with #2003

- Outcome: validators relocate to the architecture #2003 selects, if it differs, without changing
  their semantics.
- Exit criteria: subissue 10 is done; #2003 records the disposition of this EPIC's checks.

## Progress Tracking

### Workflow Checkpoints

- [x] Epic spec drafted in `docs/issues/drafts/`
- [x] First two subissue drafts created and linked
- [x] Epic spec reviewed and approved by user/maintainer
- [x] GitHub epic issue #2264 created and linked as a sub-issue of #2003
- [x] Epic folder moved to `docs/issues/open/2264-2003-refactor-semantic-link-conventions/` and
  open-state metadata plus live references updated
- [x] Subissue drafts renamed with EPIC #2264 and their `epic` fields set
- [x] Planning/evidence PR branch recorded where the document profile supports `branch`
- [x] Planning/evidence PR #2269 recorded in issue-spec `related-pr` fields
- [x] Planning/evidence PR #2269 merged into `develop`
- [ ] Remaining subissues created and linked in this spec as each phase starts
- [ ] Subissue statuses kept up to date in the `Progressive Subissues` table
- [ ] For each implemented subissue: automatic checks completed and recorded
- [ ] For each implemented subissue: manual verification completed and recorded
- [ ] For each implemented subissue: acceptance criteria reviewed post-implementation
- [ ] For each implemented subissue: implementation completion review recorded
- [ ] Epic acceptance criteria reviewed and checked off
- [ ] Epic issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-09-18 08:45 UTC - GitHub Copilot - Drafted the EPIC from the #2233 and #2185 handoffs
- 2026-09-18 10:55 UTC - GitHub Copilot - Reframed as a frontmatter-first child of #2003 after
  maintainer discussion; recorded the ownership boundary in both EPICs; renamed the draft folder
  with the parent prefix
- 2026-09-18 11:05 UTC - GitHub Copilot - Drafted the inventory and Rust-model subissues
- 2026-09-18 12:15 UTC - GitHub Copilot - Re-evaluated OKF against the canonical v0.2 spec; pinned
  an immutable snapshot under `docs/external-snapshots/`; recorded why the Naur gist is cited
  but not vendored
- 2026-09-18 12:40 UTC - GitHub Copilot - Review pass: fixed the provisional reference union to
  include `issue #<number>`, added missing template sections, bound the validator to the CLI
  output contract ADR and the `clippy-allow-reasons` precedent
- 2026-09-18 12:50 UTC - GitHub Operator - Created EPIC #2264 as a native child of #2003 and
  issues #2265 and #2266 as native children of #2264; promoted all three local specifications
- 2026-09-18 15:25 UTC - GitHub Copilot - Opened spec-only PR #2269 from the validated fork branch;
  recorded the PR on child specs #2265 and #2266
- 2026-09-19 07:58 UTC - GitHub - Merged spec-only PR #2269 into `develop`
- 2026-09-19 08:03 UTC - GitHub Copilot - Started post-merge follow-up for late review findings;
  recorded the merged planning PR and corrected live lifecycle and evidence inconsistencies

## Risks and Trade-offs

- Frontmatter-first may pressure the provisional reference convention to become permanent.
  Mitigation: the convention is explicitly versioned and the semantic-link subissues own its
  replacement.
- A strict contract can produce a large legacy backlog that tempts bulk rewrites. Mitigation:
  per-location enforcement modes and an explicit no-rewrite policy for historical records.
- Adopting OKF partially could produce a hybrid that satisfies neither OKF consumers nor Torrust
  needs. Mitigation: the inventory must recommend one explicit disposition with the differences
  addressed, not an implicit blend.
- The early validator could drift into #2003's architecture territory. Mitigation: one existing
  integration tier, no runner or cache, and a recorded relocation boundary.
- Splitting the convention document can strand rules. Mitigation: the inventory maps every
  normative rule to a destination or a deliberate retirement reason before anything is removed.

## Validation Plan

The planning/evidence branch and pull request do not implement validators. Validation for the EPIC
specification and supporting evidence is documentation-only:

- `linter markdown`
- `linter cspell`
- `linter lychee`
- `git diff --check`
