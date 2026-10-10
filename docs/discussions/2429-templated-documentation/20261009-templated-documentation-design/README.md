---
semantic-links:
  related-artifacts:
    - "issue #2429"
    - "issue #2003"
    - "issue #2264"
    - "issue #2278"
    - docs/discussions/2429-templated-documentation/20261009-templated-documentation-design/maintainer-review-session.md
    - docs/discussions/2003-overhaul-guardrails-and-automation/20261003-semantic-linking-knowledge-graph/README.md
    - docs/discussions/AGENTS.md
    - docs/templates/DISCUSSION.md
    - docs/adrs/index.md
    - docs/schemas/README.md
    - contrib/dev-tools/checks/frontmatter-validator
---

# Templated Documentation

| Field          | Value                                                                                                                                                                                                                                               |
| -------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Status         | Open for rounds                                                                                                                                                                                                                                     |
| Started        | 2026-10-09                                                                                                                                                                                                                                          |
| Opened by      | Jose Celano (`josecelano`)                                                                                                                                                                                                                          |
| Decision owner | Jose Celano (`josecelano`), maintainer; issue #2429 has no assignee yet                                                                                                                                                                             |
| Informs        | Issue #2429 - Templated Documentation                                                                                                                                                                                                               |
| Scope          | Whether and how Markdown documents should render restated facts from committed data, and how that combines with typed frontmatter and semantic links. The execution architecture (EPIC #2003) and the semantic-link model (EPIC #2264) are left out |
| AI assistance  | AI assistant using the Copilot SDK in VS Code (Claude Opus 5.5): research of the proposal and the repository, and the draft of this document and of Jose Celano's entries                                                                           |

## Context

### The Proposal

On 2026-10-03, Cameron Garnham (`da2ce7`) opened PR #2430 with an EPIC specification for issue #2429,
["Templated Documentation"](https://github.com/torrust/torrust-tracker/blob/6535ce21ec34e245f8fe2b7de65289a7f3e66914/docs/issues/open/2429-templated-documentation/EPIC.md)
(PR head `6535ce21e`). Its goal: every fact a Markdown document restates from elsewhere becomes a
rendering of committed data, and a gate fails when a restated fact goes stale. Its design is a
four-stage pipeline: hand-edited TOML inputs, a Rust data generator that emits committed JSON datasets
through one shared dataset crate, a document generator that renders templates over those datasets
with a regenerate-and-compare gate, and the existing linters over the committed renderings. Converted
documents, including every root ADR, become render templates. The proposal is linked, not copied: its
author may add it, and his positions, in his own round.

Cameron mentioned in a weekly meeting that he has been building this approach in the Torrust Index.
It is not on `torrust/torrust-index` `develop` yet; a link can be added once it is published.

### Why a Discussion Instead of an EPIC

Jose Celano reviewed PR #2430 on 2026-10-09 with an AI assistant. An edited record of the session,
written for this round, is in [`maintainer-review-session.md`](maintainer-review-session.md); it is not
a verbatim transcript (the assistant's replies are condensed). Its questions are numbered `S1` to `S9`
to avoid colliding with the topic IDs below. The review concluded:

- The idea is sound, but an EPIC with a binding design is premature. The Torrust Index and the Torrust
  Tracker have diverged in the AI harness built on top of their original code, so the context behind
  the proposal does not carry over directly.
- The proposal and the maintainer's existing approach (typed frontmatter, free-form body, verification)
  solve different parts of the problem and combine well (topics Q1 to Q3).
- Implementation waits until the tracker [v4.0.0 milestone](https://github.com/torrust/torrust-tracker/milestone/5)
  is released and the current work in progress is finished (Q8). A discussion records the reasoning
  for future contributors and AI agents in the meantime.

### Evidence

Facts checked against `develop` at `41add51ab` unless another commit is named:

- Drift exists. `docs/adrs/index.md` has no row for
  `docs/adrs/20260612000000_adopt_sccache_for_ci_bare_builds.md`, the template table in
  `docs/index.md` omits `MANUAL-VERIFICATION-EVIDENCE.md` and `PR-REVIEW-RETROSPECTIVE.md`, and
  `docs/skills/semantic-skill-link-convention.md` lists issue and EPIC frontmatter fields that disagree
  with the validator's strict profiles (all also true at the proposal's anchor, `eb96d2957`). The
  pull request that opens this discussion fixes all three by hand; the drift itself is the evidence.
- The repository already verifies restated facts in living documents. The frontmatter validator reports
  a `spec-path` that is not the specification's location and a `status` that does not match its
  lifecycle folder (`contrib/dev-tools/checks/frontmatter-validator/src/repository.rs`), and runs on the
  staged snapshot in the pre-commit hook.
- The generated-artifact precedent is not gated. `docs/schemas/frontmatter-v1.schema.json` has a
  `frontmatter-schema check` mode that passes today, but no git hook, workflow, or test invokes it;
  `docs/schemas/README.md` documents it as a manual command.
- Template delimiters can collide with existing content: 13 tracked Markdown files contain `{{`, 10 of
  them as GitHub Actions expressions such as `${{ matrix.target }}`.

Nothing here changes a specification or a rule. A conclusion takes effect only when the decision
owner records it in the Outcome and in the canonical document it links.

## Topics

### Q1 - Which documents should be generated, and which stay hand-edited?

The proposal converts documents by importance, including records such as ADRs and issue
specifications, into render templates. An alternative draws the line per fact: a fact is either
original (the document is its source of truth) or restated (a copy of something held elsewhere), and
only restated facts benefit from generation. Output-only documents (indexes, catalogs) are mostly
restated facts; living documents (specifications, ADRs, audit records) are mostly original. Added by
Jose Celano. Session record: S4, S5, S9.

#### Positions

##### Q1 - Jose Celano (`josecelano`), 2026-10-09, PR #2501

Most Markdown files in this repository have two parts: frontmatter, which holds the data that changes,
and a body, which is prose that contributors and agents edit freely. Issue specifications track the
progress of their issue, and their bodies start from a template but diverge. The proposal fits
output-type documents such as the ADR list very well; it does not fit issue specifications. The two
approaches solve different points and can be combined: typed frontmatter, a free body, and
verification for living documents; rendering for output-only documents, with frontmatter as their
data source. Drafted with an AI assistant using the Copilot SDK in VS Code (Claude Opus 5.5);
reviewed and adopted by Jose Celano.

### Q2 - Should a rendering be a second file or a generated region?

The proposal leaves open "a rendering as a second file or as generated regions". With a second file,
the `.md` is output only and every edit, including a prose fix, goes through its template. With a
generated region, the `.md` stays the edited file and only a marked block (for example, the table in
`docs/adrs/index.md`) is rendered. Added by Jose Celano. Session record: S4, S6.

#### Positions

##### Q2 - Jose Celano (`josecelano`), 2026-10-09, PR #2501

The concern is the editing workflow: being able to edit a Markdown file whenever needed, with no
regeneration step, is one of the strengths of the current approach. Generated regions keep that for
everything outside the rendered block, so they look like the better starting point; I would like the
author's view on when a full second file is necessary. Drafted with an AI assistant using the Copilot
SDK in VS Code (Claude Opus 5.5); reviewed and adopted by Jose Celano.

### Q3 - Should living documents become per-document render templates?

The proposal converts records such as ADRs and specifications so that their frontmatter, tables, and
related lists render while their prose stays authored. It leaves template granularity open ("how many
templates, per package or per concept"); only its root-ADR subissue chooses one template per document
("Each of the 29 becomes a template"). This topic examines that per-document option for living
documents. Under it, an agent that ticks a checkbox or changes `status` edits the template and runs
`write`, and placeholders could reference other repository objects, such as another issue's status.
Open questions for the author: in that case, does the data generator read frontmatter from the
template or from the rendering, and how is the resulting loop avoided? Added by Jose Celano. Session
record: S5, S6.

#### Positions

##### Q3 - Jose Celano (`josecelano`), 2026-10-09, PR #2501

For living documents I prefer the current model: an authoring template as the starting point, typed
frontmatter checked by the validator, and a body edited directly. It needs fewer files (no dataset and
no per-document template) and no regeneration. Its limitation is that the body cannot include values
dynamically; for living documents that is acceptable, and restated facts in them can be verified
instead. Drafted with an AI assistant using the Copilot SDK in VS Code (Claude Opus 5.5); reviewed
and adopted by Jose Celano.

### Q4 - Is a committed JSON dataset layer needed from the start?

The proposal commits JSON datasets and makes the renderer read only them. They are aggregates built
from frontmatter, paths, and other sources, not a replacement for frontmatter. They provide reviewable
data diffs and one shared interface for checkers; they add a third file to each change and a merge
conflict point for parallel pull requests. A smaller start renders directly from frontmatter and adds
datasets when a concrete need appears. Added by Jose Celano. Session record: S2, S9.

#### Positions

No entries yet.

### Q5 - Where and when does the gate run?

The proposal leaves gate placement to a subissue and to EPIC #2003. A gate that nothing invokes does
not prevent drift (see the schema precedent in Evidence). A pre-commit gate must check the staged
snapshot, not the working tree, as the frontmatter validator does. Open question: should the hook only
`check`, or also run `write`? Added by Jose Celano. Session record: S2, S3.

#### Positions

##### Q5 - Jose Celano (`josecelano`), 2026-10-09, PR #2501

In the current harness the time to run tools is the pre-commit and pre-push hooks. Work is planned in
small deployable increments, so agents do not do much without committing, and commit time fits
guardrail checks well. This would not hold if agents ran for hours without committing. Drafted with an
AI assistant using the Copilot SDK in VS Code (Claude Opus 5.5); reviewed and adopted by Jose Celano.

### Q6 - How do forge facts enter, and who refreshes them?

Under the proposal, GitHub facts enter only as committed captures stamped "as of" an instant, and the
gate compares renderings with the capture, not with GitHub. When an issue closes, nothing in the tree
changes and the gate still passes. The subissue `Status` column becomes a stamped copy that someone
still refreshes. Open questions for the author: who refreshes captures and when, how a reader knows a
capture is too old, and what the check results among the generator's inputs ("frontmatter, paths,
links, committed captures and check results") contain. Added by Jose Celano. Session record: S7.

#### Positions

No entries yet.

### Q7 - How does template syntax relate to semantic links?

Placeholders are substituted and disappear from the rendering; semantic links (frontmatter
`semantic-links`, `<!-- skill-link: ... -->` markers, `review-finding:` references, and the `[[...]]`
syntax proposed in the
[knowledge-graph discussion](../../2003-overhaul-guardrails-and-automation/20261003-semantic-linking-knowledge-graph/README.md))
stay in it. A template would contain both, and both could offer a way to reference another artifact.
EPIC #2264 still has to decide the prose path-reference syntax (its row 7). Added by Jose Celano.
Session record: S8.

#### Positions

##### Q7 - Jose Celano (`josecelano`), 2026-10-09, PR #2501

Cross-referencing would use a different markup inside Markdown, which can cause misunderstandings:
placeholders are resolved in the final file and semantic links are not, but in a template both appear
side by side. The two mechanisms need a clear boundary. Drafted with an AI assistant using the Copilot
SDK in VS Code (Claude Opus 5.5); reviewed and adopted by Jose Celano.

### Q8 - When should this be implemented?

The proposal's author asked on PR #2430 whether this is work for now or for after version 4. The
tooling phase converts no documents, but it is not isolated either: it adds crates, which this
repository registers explicitly in the root workspace `members` list. The conversion phases touch many
documents and change how every contributor and agent edits them, which conflicts with work in
progress. Added by Jose Celano.

#### Positions

##### Q8 - Jose Celano (`josecelano`), 2026-10-09, PR #2501

Not before the tracker v4.0.0 release and not before the current work in progress is finished. There
are too many things in progress, and a change of this size is better made in isolation, without other
tasks in parallel. Drafted with an AI assistant using the Copilot SDK in VS Code (Claude Opus 5.5);
reviewed and adopted by Jose Celano.

### Q9 - How should ideas mature into planned work?

This proposal arrived as an EPIC with a binding design. A staged path would let ideas mature from high
uncertainty to concrete work: discussion, then feature, then EPIC, then subissues. The path is not
documented today; `docs/discussions/AGENTS.md` only says that a discussion's conclusions are recorded in
an issue, an EPIC specification, or an ADR. Documenting it is out of scope for this discussion. Added
by Jose Celano.

#### Positions

##### Q9 - Jose Celano (`josecelano`), 2026-10-09, PR #2501

Ideas should go from a higher level of abstraction and uncertainty to more concrete things:
discussion, feature, EPIC, subissues. This workflow should be documented, in a separate change.
Drafted with an AI assistant using the Copilot SDK in VS Code (Claude Opus 5.5); reviewed and
adopted by Jose Celano.

## Outcome

Pending: no decision recorded.
