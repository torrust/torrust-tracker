---
semantic-links:
  related-artifacts:
    - "issue #2003"
    - "issue #2264"
    - docs/discussions/2003-overhaul-guardrails-and-automation/20261003-goals-and-boundaries/README.md
    - docs/skills/semantic-skill-link-convention.md
    - contrib/dev-tools/checks/frontmatter-validator/
    - lychee.toml
---

# Semantic Linking and a Repository Knowledge Graph

| Field        | Value                                                                              |
| ------------ | ---------------------------------------------------------------------------------- |
| Status       | Draft for review in the pull request that adds it                                  |
| Started      | 2026-10-03                                                                         |
| Participants | Jose Celano; an AI agent (initial proposal); GitHub Copilot (review and draft)     |
| Reviewer     | Cameron (`da2ce7`), assignee of EPIC #2003 and its child EPIC #2264                |
| Informs      | EPIC #2264 - Refactor Semantic Link and Frontmatter Conventions                    |
| Scope        | Aspect 1 (knowledge graph) of [Goals and Boundaries](../20261003-goals-and-boundaries/README.md) |
| Source       | [`initial-proposal.md`](initial-proposal.md)                                       |

## Context

Jose Celano asked an AI agent how the repository could become a knowledge graph of its artifacts
(code, issue specifications, ADRs, tests, pull requests) without turning every document into
structured data. The answer is kept in [`initial-proposal.md`](initial-proposal.md). This document
reviews it against the repository's current conventions and the planned work in EPIC #2264, and
proposes conclusions for the owner of that EPIC.

The first draft mixed different goals. The
[Goals and Boundaries](../20261003-goals-and-boundaries/README.md) discussion now separates four
aspects; this document covers the first, the knowledge graph. Its points about workflow state,
the lifecycle of issue specifications, and how strictly links are validated moved there.

Nothing here changes a specification. A conclusion takes effect only when the EPIC owner records
it in #2264 or one of its subissues.

## The Proposal in Brief

"Structure the references, not the documents." Markdown bodies stay free-form. Only the links
between artifacts become explicit and machine-resolvable. Addresses such as "this heading in that
document" are derived instead of being maintained by authors. Every edge keeps the evidence that
created it. The relation vocabulary should emerge from real links rather than from an ontology
designed up front. The proposal also suggests a `[[...]]` reference syntax, one resolver per
resource kind, and a graph database as a late phase.

## Relationship to EPIC #2264

Most of the proposal is already planned or delivered:

| Proposal (section)                                    | Existing coverage                                                                                                                   |
| ----------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| Strict frontmatter, free-form body (5)                | v1 Rust frontmatter model and `frontmatter-validator` (#2265, #2266, #2280, and #2281 are done)                                     |
| Resource kinds and resolvers (8, 15)                  | #2264 candidate target types: `path`, `markdown-section`, `issue`, `epic`, `adr`, `skill`, `agent`, `review-finding`, `commit`, `rust-module`, `rust-item` |
| Relation vocabulary (10, 24)                          | #2264 candidate relation types, explicitly open to revision                                                                         |
| Explicit links only where they matter (11, 12, 19)    | #2264 "Path References Versus Semantic Links"                                                                                       |
| One reference syntax in comments and other files (28) | #2264 "Marker placement" convention; existing `skill-link:` comment markers                                                         |
| Theory building after Naur (20)                       | Already a #2264 design input, with the same caution against over-formalizing prose                                                  |
| Graph tooling and database (28)                       | Tool placement and execution belong to #2003; see the AI-harness workspace draft below                                              |

The proposal is an independent confirmation of the direction of #2264, not a new direction.

## Existing Link Corpus

The proposal says to collect real links before designing the vocabulary. The repository already
has them. Measured on `develop` at `eb96d2957` by parsing the frontmatter of every tracked Markdown
file with a throwaway script:

| Measure                                                                    | Count                |
| -------------------------------------------------------------------------- | -------------------- |
| Tracked Markdown files / with frontmatter / with `semantic-links`          | 970 / 793 / 686      |
| `related-artifacts` entries                                                | 3,466                |
| ... repository paths / `issue #N` / `review-finding:` / other              | 3,286 / 82 / 35 / 15 |
| ... `issue` entries whose number YAML dropped (unquoted `#`), in 30 files  | 48                   |
| `skill-links` entries, across 26 distinct skills                           | 649                  |
| Markdown files with an inline `<!-- skill-link: ... -->` marker            | 258                  |
| Non-Markdown files with a `skill-link:` marker                             | 25                   |
| `review-finding:pr-<number>-<id>` occurrences in tracked files             | 542                  |
| Path entries that no longer resolve                                        | 143 (4.4%)           |

Unresolved path entries by the location of the document that contains them:

| Source location        | Unresolved / path entries |
| ---------------------- | ------------------------- |
| `docs/issues/closed/`  | 108 / 2,002               |
| `docs/pr-reviews/`     | 24 / 238                  |
| `docs/adrs/`           | 5 / 167                   |
| `docs/issues/drafts/`  | 3 / 194                   |
| Other documents        | 2 / 305                   |
| `docs/issues/open/`    | 1 / 321                   |
| `.github/`             | 0 / 59                    |

Observations:

- The corpus is large enough to inventory bottom-up today.
- Every `related-artifacts` entry says only that two artifacts are related, never how. A relation
  vocabulary can still be derived from the corpus by classifying pairs of source document type and
  target kind, for example issue specification to skill, or ADR to workflow.
- 48 of the 130 issue references (37%) have lost their number: an unquoted `#` starts a YAML
  comment, so the value loads as the bare word `issue`. The friction register already tracks this
  as `link-convention-unquoted-issue-marker`.
- The 15 "other" values (URLs, a commit, cross-repository issues) are target kinds that authors
  already need and the v1 reference convention does not accept.
- `frontmatter-validator` resolves paths only in draft and open issue specifications. Closed
  specifications are treated as historical ("their paths go stale by design"), and nothing checks
  links in ADRs or PR-review records.

## Input from the EPIC #2003 Thread

All 54 comments on #2003 were read on 2026-10-03. Most of the thread is the friction register
that Cameron keeps: 148 labels for frictions in skills, templates, and tooling, plus two CI
findings, indexed in two comments linked from the "Friction Register" section of the #2003 issue
body. No comment discusses a knowledge graph, but the register bears on this discussion in five
ways:

1. **It is a typed, evidence-backed link set kept by hand.** Each row links a friction to the
   comment that filed it, the review round that surfaced it, and a disposition with evidence:
   `ADOPTED` names the landing commit, `SPECIFIED` the issue that owns the fix, and `SUPERSEDED`
   the change that removed the ground. These are typed edges with evidence, the model of section
   17, and a second corpus for deriving relation types. The register's method line says every
   anchor is resolved by script before posting, so it is the first consumer a resolver would
   serve.
2. **It chose names over ordinals.** EPIC #2003 records the decision of 2026-09-26 to file
   frictions under kebab-case labels that start with the owning artifact, because "a number
   carries no meaning, collides with reviewers' finding IDs and with re-raises, and turns
   deduplication into a separate lookup." `epic-template-ordinal-subissue-references` records the
   same failure for "Subissue N" references in EPIC tables. Identity that must survive reordering
   needs a stable name: the top of the stability ladder in section 14.
3. **It pins positional evidence to a commit.** Every `file:line` in the register names the
   `develop` head it was computed at. Line numbers recorded without one go stale after a rebase or
   a later edit (`process-pr-review-verification-head-not-rebase-stable`). The `Evidence` type in
   section 23 needs the same field.
4. **Several open labels are this discussion's problems.** The "Link and frontmatter convention"
   group (8 labels, 6 open), whose items belong to #2264, includes the lost issue numbers
   (`link-convention-unquoted-issue-marker`, `link-convention-issue-key-loads-null`), the unclear
   scope of the path-stability rule (`link-convention-stability-warning-scope`), and inline markers
   that duplicate frontmatter (`templates-ship-redundant-inline-skill-marker`). Separately,
   `cleanup-completed-issues-audit-links-class-unstated` asks whether a past audit's frontmatter
   links are navigational or historical. The conclusions below cite these labels instead of
   raising them again.
5. **Its most frequent defect class is a relation nobody records.** Two copies of one rule drift
   apart across artifacts (`review-findings-rules-copied-across-three-docs`,
   `pre-push-step-list-copied-in-four-guides`, `link-convention-field-lists-drift-from-schema`). An
   edge from each copy to its source would let a check find every copy when the source changes.

Two drafts placed under #2003 also apply. Whether graph tooling counts as AI-harness tooling, and
where it would live, depends on the five decisions listed in
`docs/issues/drafts/2003-separate-ai-harness-cargo-workspace/ISSUE.md`. And
`docs/issues/drafts/2003-mine-ai-agent-memories/ISSUE.md` sends its frontmatter findings to #2264,
so it is a further input for the same model.

## Points to Carry Forward

1. **Let the vocabulary emerge from real links** (sections 10, 24, 25). The corpus and the register
   above make this possible now instead of after a new syntax has been adopted.
2. **Keep the evidence of every edge** (section 17): source artifact, location, and the original
   reference text. A parser has this information anyway. The location needs a commit as well, as
   the register shows.
3. **Keep explicit and inferred edges apart** (section 18). Edges inferred from Git history or pull
   request metadata are a derived data set, not repository content.
4. **Grade identity by the stability it needs** (section 14): explicit identifier, intrinsic
   identity, derived address, physical location. This framing suits the path-reference subissue.

## Points of Disagreement

1. **No `[[...]]` syntax** (sections 10, 22). GitHub does not render it as a link, so readers lose
   navigation, and no repository tool resolves or validates it. Standard Markdown links already
   cover every target that has a path: `[text](other.md#investigation)` is a heading-derived
   address with no hand-written identifier, and lychee validates it offline
   ([`lychee.toml`](../../../../lychee.toml): `offline = true`, `include_fragments = "full"`). A new
   form is needed only for targets without a path, and the repository already has a pattern for
   those: `review-finding:pr-<number>-<id>`, and `issue #N`, whose `#` fails in YAML (open
   question 2).
2. **Rust symbols are not cheap to resolve** (section 15). Resolution needs rustdoc JSON (nightly
   only) or a rust-analyzer index. Without a resolver, a renamed symbol breaks the reference
   silently. Inside Rust code, rustdoc intra-doc links already resolve symbols and warn when one is
   broken.
3. **Behavioral tests as the durable specification** (section 4) may be right, but it is a
   testing-strategy decision. Folding it into #2264 would widen that EPIC's scope.

## Draft Conclusions

These are drafts for review.

1. **Direction.** The proposal confirms the direction of #2264. No change of course is needed.
2. **Inventory before vocabulary.** Before the #2264 row "Normalize the semantic-link model" fixes
   relation types, classify the existing frontmatter corpus by source document type and target
   kind, and the register's dispositions by edge type, and derive candidate relations from what
   occurs. Treat the free-form values as candidate target kinds.
3. **Names, not positions.** Targets are identified by names that survive reordering and edits:
   issue numbers, register labels, skill names, symbol paths, and heading text. Ordinals and line
   numbers appear only as evidence, pinned to a commit.
4. **Edges carry evidence.** The semantic-link model records, for every edge, the source artifact,
   its location at a named commit, and the reference text, and marks whether the edge is explicit
   or inferred. Inferred edges are a derived data set, not part of the convention.
5. **No new syntax for targets with a path.** They use standard Markdown links, headings included.
   Only targets without a path (issues, review findings, commits, Rust items) use a typed prefix,
   following the existing `review-finding:` form. No `[[...]]` syntax. This is input for the #2264
   row "Decide path-reference scope and syntax".
6. **Rust targets need a resolver first.** Accept `rust-module` and `rust-item` targets only
   together with a check that resolves them. Until then, use file paths, and keep rustdoc
   intra-doc links inside Rust code.
7. **Behavioral tests are a separate topic.** Whether acceptance tests become the durable
   specification is a testing-strategy question outside #2264.
8. **Graph tooling comes later and belongs to #2003.** A graph builder or store is a read-only
   analysis tool. Its placement follows the decisions the AI-harness workspace draft lists. It is
   not part of the convention work.

## Open Questions for the Reviewer

1. Should the #2264 row "Normalize the semantic-link model" start with the inventory from
   conclusion 2?
2. Should every target without a path use one `kind:value` form? 48 of 130 issue references lost
   their number to YAML's comment syntax (`link-convention-unquoted-issue-marker`), and the
   documented `issue: #<number>` key loads as null (`link-convention-issue-key-loads-null`). A form
   such as `issue:2264` avoids both, at the cost of migrating the 130 entries.
3. Should the graph read GitHub-hosted records such as the friction register, or only tracked
   files? The register is the richest typed link set in the project, and it lives in issue
   comments.
4. Where should the outcome be recorded: the #2264 progress log and subissue rows, or elsewhere?

## Outcome

Pending review.
