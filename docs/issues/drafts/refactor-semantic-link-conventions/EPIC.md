---
doc-type: epic
status: draft
github-issue: null
spec-path: docs/issues/drafts/refactor-semantic-link-conventions/EPIC.md
epic-owner: null
last-updated-utc: 2026-09-18 08:45
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/skills/semantic-skill-link-convention.md
    - docs/issues/open/2003-overhaul-guardrails-and-automation/EPIC.md
    - docs/issues/closed/2233-2003-tune-unified-pr-review-process/ISSUE.md
    - docs/issues/closed/2233-2003-tune-unified-pr-review-process/code-span-path-case-analysis.md
    - docs/issues/closed/2233-2003-tune-unified-pr-review-process/code-span-path-case-inventory.tsv
    - docs/issues/open/2185-2003-triage-advisory-external-link-check-findings/ISSUE.md
    - docs/issues/open/2185-2003-triage-advisory-external-link-check-findings/external-link-baseline.md
    - docs/issues/drafts/refactor-semantic-link-conventions/external-link-check-residual-failures-2026-09-18.md
    - .github/lychee-online.toml
    - .github/workflows/external-link-check.yaml
    - docs/testing.md
---

# EPIC #[To be assigned] - Refactor Semantic Link and Frontmatter Conventions

## Goal

Define the next version of the repository's document metadata, semantic-link, and path-reference
conventions before implementing validators or migrating existing documents.

The EPIC is specification-first. It should produce clear conventions and progressive subissues, not
perform the full repository migration in one step.

## Background

The current convention document, `docs/skills/semantic-skill-link-convention.md`, has grown from a
small skill-link convention into a mixed source of truth for several related but distinct concerns:

- frontmatter metadata fields for issue and EPIC specs;
- frontmatter schema expectations and lifecycle statuses;
- semantic links between skills and repository artifacts;
- language-specific inline marker placement;
- issue and review-finding references;
- ADR and workflow cross-reference guidance.

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

[Open Knowledge Format](https://okf.md/) is relevant because it treats knowledge as Git-native
Markdown with YAML frontmatter and a small validation surface. Its useful ideas for this EPIC are:

- keep documents readable with ordinary Markdown and reviewable with normal Git workflows;
- use typed frontmatter so consumers can route documents without guessing;
- separate minimum conformance from richer producer-specific conventions;
- keep unknown fields and unknown types consumable during migration;
- use standard Markdown links for ordinary navigation and tolerate incomplete knowledge graphs;
- use a `resource`-style field to anchor a concept to the real thing it describes;
- version conventions before introducing stricter validation.

The main caution for this repository is that OKF intentionally leaves link semantics mostly in prose.
That is useful for adoption, but not enough by itself if the repository wants validated relation
types and target types for a knowledge graph.

### Programming as Theory Building

[Programming as Theory Building](https://gist.github.com/onlurking/fc5c81d18cfce9ff81bc968a7f342fb1)
is relevant because it frames documentation as a support for shared understanding, not as a complete
replacement for human design knowledge. Its useful ideas for this EPIC are:

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

## Scope

### In Scope

- Split the current semantic-link convention into clearly owned convention documents.
- Define versioning rules for convention documents before incompatible schema or syntax changes.
- Define typed frontmatter metadata schemas for issue specs, EPIC specs, ADRs, skills, agents, and
  other document classes selected by the EPIC.
- Evaluate Rust types as the canonical representation for frontmatter metadata, semantic links,
  target types, relation types, and validation constraints.
- Define a normalized semantic-link model suitable for building a repository knowledge graph.
- Decide how typed semantic links differ from ordinary path references in prose.
- Decide whether path references need their own explicit syntax and validator separate from semantic
  links.
- Coordinate validator implementation boundaries with EPIC #2003 before choosing whether
  frontmatter, semantic-link, or path-reference validators live in standalone scripts, dedicated
  Rust binaries, or a future unified AI-harness application.
- Define progressive migration and validation subissues.
- Preserve compatibility rules for existing documents during migration.

### Out of Scope

- Migrating all existing documents in the EPIC issue itself.
- Implementing validators in the EPIC issue itself.
- Rewriting historical records only to satisfy new syntax.
- Replacing Lychee as the validator for normal Markdown links.
- Requiring every prose path mention to become a semantic link.
- Selecting the long-term AI-harness architecture, command surface, cache model, or division between
  guardrail checks and repository-mutating actions; those decisions belong to EPIC #2003.

## Proposed Convention Split

The EPIC should decide final names and locations, but this starting split separates concerns:

| Area | Proposed responsibility | Candidate output |
| ---- | ----------------------- | ---------------- |
| Frontmatter metadata | Document types, required fields, lifecycle statuses, version fields, schemas, and validation expectations. | `docs/conventions/frontmatter-metadata.md` |
| Semantic links | Typed relationships between project concepts, with relation names, target types, and graph semantics. | `docs/conventions/semantic-links.md` |
| Typed model and validators | Rust types or another schema source of truth for document metadata, semantic-link targets, relations, parsing, diagnostics, and staged enforcement. Validator placement must remain compatible with EPIC #2003's automation design. | `contrib/dev-tools/checks/`, a dedicated package, or a future AI-harness component, to be decided |
| Path references | Syntax and validation rules for machine-checkable repository paths in prose, distinct from ordinary Markdown links. | `docs/conventions/path-references.md` |
| Marker placement | Language-specific syntax for comments, frontmatter, and inline markers outside Markdown. | `docs/conventions/reference-placement.md` |
| Migration policy | Versioning, compatibility windows, staged validation, and historical-document handling. | `docs/conventions/convention-migrations.md` |

This split may also require deciding whether `docs/skills/` is still the right folder for these
conventions. The current path is historical: the document began as a skill-link convention, but its
scope now extends beyond skills.

## Concept Model to Specify

The EPIC should define a minimal typed model before validators are written.

Candidate representation approaches:

- prose conventions only;
- JSON Schema or another declarative schema language;
- Rust types with parsers and validators as the executable source of truth;
- a hybrid where prose explains the convention and Rust types enforce the selected subset.

The EPIC must compare these approaches before choosing one. Rust types are a first-class candidate,
especially where constraints are richer than field presence, string shape, or simple enum values.

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

## Relationship to EPIC #2003

This EPIC may produce deterministic validators for Markdown frontmatter, semantic links, and typed
path references. Those validators are checks in the terminology of
[EPIC #2003](../../open/2003-overhaul-guardrails-and-automation/EPIC.md): they should be read-only by
default, return stable diagnostics, and be callable from local hooks, CI, and agent workflows.

Their implementation may eventually belong in the same Rust application or shared libraries as the
AI harness proposed by #2003, but this EPIC should not decide that architecture by accident. The
convention work should specify the data model, validation semantics, rollout stages, and diagnostics
well enough that #2003 can later choose the execution surface.

The boundary matters because #2003 also covers repository actions such as dependency updates and
completed-issue cleanup. Frontmatter and semantic-link validation may share parsing, repository
discovery, JSONL/NDJSON reporting, and configuration loading with those actions, but it must remain
clear which commands are read-only checks and which commands mutate the repository.

## Progressive Subissues

| ID | Draft subissue | Expected output |
| -- | -------------- | --------------- |
| S1 | Inventory current convention responsibilities | Map every normative rule in `docs/skills/semantic-skill-link-convention.md` to a future home or deliberate retirement reason. |
| S2 | Define convention versioning and compatibility | Add versioning rules, informed by OKF's permissive migration model, so documents can be migrated progressively without breaking existing validators. |
| S3 | Split frontmatter metadata conventions | Specify document frontmatter schemas, lifecycle statuses, required fields, optional fields, and schema evolution rules. |
| S4 | Choose frontmatter representation | Decide whether frontmatter constraints are represented by prose, JSON Schema, Rust types, or a hybrid before implementing validation. |
| S5 | Design frontmatter validation | Specify validator inputs, outputs, error format, rollout stages, and which document classes are enforced first. |
| S6 | Normalize semantic links | Define relation types, target types, canonical syntax, and how links support a knowledge graph. |
| S7 | Choose semantic-link representation | Decide whether semantic-link constraints are represented by prose, schema, Rust types, or a hybrid. |
| S8 | Design semantic-link validation | Specify how to validate target existence, target type, relation type, uniqueness, and invalid combinations. |
| S9 | Decide path-reference syntax | Decide whether machine-checkable prose paths are semantic links, a separate typed reference, or intentionally out of scope. |
| S10 | Design path-reference validation | If S9 chooses explicit path references, specify validation behavior, exclusions, historical handling, and migration rules. |
| S11 | Review `docs/skills/` ownership | Decide whether convention documents should move from `docs/skills/` to a broader folder such as `docs/conventions/`. |
| S12 | Plan staged migration | Define ordering, branch strategy, compatibility windows, and no-rewrite rules for historical records. |
| S13 | Classify external-link importance and checker limitations | Using the #2185 handoff below, decide how the advisory online check should treat license boilerplate, background reading, bot-protected hosts, TLS-incompatible hosts, and critical unstable references whose important contents may need an in-repository copy or distilled context for agents and maintainers. Decide whether link importance belongs in the typed model. |
| S14 | Coordinate validator placement with EPIC #2003 | Decide which outputs from S5, S8, and S10 are convention-owned validation semantics and which implementation choices must be deferred to EPIC #2003's automation and AI-harness architecture decision. |

## Relationship to Issue #2233

Issue #2233 should not solve the full convention problem. Its code-span path inventory is evidence
for S9 and S10 in this EPIC.

The #2233 T2 finding can be resolved by recording that strict Markdown code-span path enforcement is
deferred until this EPIC defines whether path references are semantic links, a separate typed
reference form, or ordinary prose outside validator scope.

## Handoff from Issue #2185

Issue [#2185](https://github.com/torrust/torrust-tracker/issues/2185) set out to clean the broken
links reported by the advisory
[External Link Check workflow](../../../../.github/workflows/external-link-check.yaml). It started
from 461 errors, repaired every genuinely stale reference it found, and added only exact,
commented exclusions to [`.github/lychee-online.toml`](../../../../.github/lychee-online.toml). It
closed with seven errors and four timeouts that no cleanup step can remove. Those cases are
preserved verbatim in
[`external-link-check-residual-failures-2026-09-18.md`](external-link-check-residual-failures-2026-09-18.md)
and are input for S13. No solution is chosen here.

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
of a link decide how it should be validated. S13 should use the residual report and the #2185
baseline as its evidence and may conclude that the answer is prose policy, a small typed
annotation, or a different checker configuration; the EPIC does not presume which.

## Open Questions

- Which document classes require typed frontmatter first: issue specs, EPIC specs, ADRs, skills,
  agents, or all Markdown documents?
- Should schemas be defined in prose, JSON Schema, Rust types, or another format?
- Which constraints should be enforced by Rust types rather than documented only in Markdown?
- Should Rust types be the source of truth for validators, or should they be generated from a schema?
- Should semantic-link target types be URI-like strings, YAML maps, or both?
- Should a file path be represented as `path:...`, a typed YAML object, or a plain string in selected
  fields?
- Which references should be considered knowledge-graph edges, and which are only validation aids?
- Should historical closed issues and PR review records be exempt from new reference syntax?
- How should comments in Rust, shell, TOML, YAML, and Dockerfiles represent typed links without
  harming the host language parser?
- Which OKF principles should be copied directly, adapted, or rejected for a repository whose
  knowledge graph includes code, issues, review findings, ADRs, commits, and Markdown sections?
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

- [ ] The responsibilities currently mixed in `docs/skills/semantic-skill-link-convention.md` are
      inventoried and assigned to future convention documents or deliberately retired.
- [ ] A convention versioning and compatibility policy is specified before migrations begin.
- [ ] Frontmatter metadata conventions are specified separately from semantic-link conventions.
- [ ] The EPIC compares prose-only, schema-based, Rust-typed, and hybrid representations for
  frontmatter and semantic-link constraints before selecting an enforcement approach.
- [ ] Semantic-link relation types and target types are normalized enough to support validation and
      knowledge-graph construction.
- [ ] The EPIC decides whether machine-checkable path references are semantic links, a separate
      reference type, or intentionally out of scope.
- [ ] Progressive implementation subissues are created for validation and migration work.
- [ ] The final plan states how frontmatter, semantic-link, and path-reference validators relate to
  EPIC #2003 without preselecting the AI-harness architecture.
- [ ] Historical records have an explicit no-rewrite or migration policy.
- [ ] The final plan states which existing validators, including Lychee, remain responsible for
      ordinary Markdown links.
- [ ] The final plan records how OKF and theory-building inputs affected the chosen convention
  boundaries.

## Validation Plan

This draft EPIC does not implement validators. Validation for the EPIC itself is documentation-only:

- `linter markdown`
- `linter cspell`
- `linter lychee`
- `git diff --check`
