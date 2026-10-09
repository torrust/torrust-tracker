# Templated Documentation — Maintainer Review Session

> **Source material, not policy.** Record of a review session between Jose Celano (maintainer) and an
> AI assistant (GitHub Copilot in VS Code; model not recorded) about PR #2430 and issue #2429, held on
> 2026-10-09. The maintainer's messages are lightly edited for spelling only; their meaning is
> unchanged. The assistant's replies are condensed. Facts are checked against `develop` at `41add51ab`
> unless another commit is named. Repository paths are written as code spans, not links. The
> discussion, its topics, and Jose Celano's positions are in [`README.md`](README.md).

## Context

- Proposal: PR #2430 adds `docs/issues/open/2429-templated-documentation/EPIC.md` (Cameron
  Garnham), "Templated Documentation", head `6535ce21e`.
- Proposal anchor: `develop` `eb96d2957` (2026-10-03).
- The author's question on the PR: "is this work for now, or for after version 4?"

## Part 1 — Understanding the Proposal

### Maintainer

I don't understand what this new EPIC is about:

- What problem is it trying to solve?
- What's the goal?
- Why?
- How is it supposed to solve the problem?
- Is it the right moment? It looks like a big refactor. Maybe it's better to implement it when all
  work in progress is finished and make the change isolated, without working on other tasks in
  parallel.

### Assistant

**Problem.** Many Markdown documents restate facts that live elsewhere in the repository (lists of
files, counts, statuses, field lists, quoted lines). Nothing compares the copy with its source, so
copies silently go stale. Instances on `develop`:

- `docs/adrs/index.md` lists 28 of the 29 root ADRs at the anchor; the
  `20260612000000_adopt_sccache_for_ci_bare_builds.md` row is missing (still missing at `41add51ab`).
- The template table in `docs/index.md` lists 12 of the 14 templates at the anchor.
- `docs/skills/semantic-skill-link-convention.md` lists EPIC frontmatter fields that disagree with
  the frontmatter validator's EPIC profile.
- EPIC subissue tables copy GitHub state into a hand-kept `Status` column.

**Goal.** Every restated fact becomes a rendering of committed data, and a gate fails when a
committed document no longer matches a fresh render. A precedent exists:
`docs/schemas/frontmatter-v1.schema.json` is generated from Rust and has a `check` mode.

**Why.** Copies drift silently, and the volume of Markdown (970 files at the anchor, 1076 at
`41add51ab`, much of it agent-authored) keeps growing.

**How.** A four-stage pipeline, each output committed and gated:

```text
TOML inputs (hand-edited)            e.g. template descriptions
        │
        ▼
Rust data generator  ── scans tree ──► JSON datasets (committed)   e.g. ADR census
        │
        ▼
Document generator: template + JSON ──► rendered .md (committed)   e.g. docs/adrs/index.md
        │
        ▼
Existing linters on the rendered .md
```

Supporting pieces: one Rust crate owning every JSON dataset, a namespace/version envelope on every
structured file, and a `require` function so prose fails to render when the data no longer supports
it. Six subissues: the dataset crate, two generators, then conversion (root ADRs first, then
specifications and audit records, then the remaining inventories and prose guides).

**Timing.** The EPIC has two different parts:

1. Tooling (Phase 1): new crates and generators. Additive; it touches no existing document, so it
   could run in parallel with other work.
2. Conversion (Phases 2–4): the big refactor. It rewrites existing documents into templates
   (including an editorial pass over every ADR) and changes how every contributor and agent edits
   documents (edit the template, not the `.md`). With about 50 open issue folders and 572 commits
   landing on `develop` between the anchor and `41add51ab`, doing this in parallel means conflicts
   and confusion. The EPIC already makes Phase 3 wait for EPIC #2264 and EPIC #2278, both open.

Open challenges raised by the assistant:

- The cited drift is a handful of instances fixable in a small PR today.
- A cheaper alternative is verification instead of generation: consistency checks (for example,
  every root ADR file has a row in `docs/adrs/index.md`; every template is listed in
  `docs/index.md`). They catch the same drift without templates, a new crate, or a new editing
  workflow. The EPIC does not weigh this alternative.
- Converting ADR bodies conflicts with the EPIC's own "Out of Scope: rewriting closed or historical
  records" and with the ADR rule of superseding rather than editing (`docs/adrs/README.md`).

## Part 2 — Process: Back to a Discussion

### Maintainer

My impression is that this plan is clear to Cameron because he has implemented it in the Torrust
Index. However, the Torrust Index and the Torrust Tracker have diverged a lot in terms of the AI
harness we have each built on top of the initial production code.

1. It's too early to define an EPIC, because we do not have the same context as in the Index repo.
2. I would like to discuss it before implementing it, because I want to add my feedback. It is not
   going to be just a couple of comments: I will ask a lot of questions and I want to clarify the
   current context in the Tracker. I'm not against this implementation, it sounds reasonable, but I
   have alternative ideas and many questions about it.

I propose to go one step back, close the EPIC and the PR, and open a new discussion in
`docs/discussions`, following this workflow:

```text
discussion -> feature -> EPIC -> subissues
```

As the idea becomes more mature, we go from a higher level of abstraction and uncertainty to more
concrete things.

### Assistant

Agreed with going back to a discussion; the repository already has the mechanism
(`docs/discussions/AGENTS.md`): a discussion is input, not a decision, and its Outcome is recorded
in an issue, EPIC specification, or ADR.

- The Index premise could not be verified: `torrust/torrust-index` `develop` has no
  dataset/render/census pipeline (its only templates are HTML), and no matching branch was found on
  the author's fork. It may be unpublished; ask the author where it lives so it can be linked as
  evidence.
- Keep issue #2429 rather than close it: discussions are grouped by issue number
  (`docs/discussions/<issue>-<slug>/<date>-<topic>/`). Retitle it as a proposal and drop the `EPIC`
  label. Precedent: the I2P discussion lives under feature issue #2491.
- Convert PR #2430 into the opening round instead of closing it: the author moves the EPIC text
  verbatim to `initial-proposal.md` in the discussion folder (precedent: the EPIC #2003
  semantic-linking discussion), adds `README.md` from `docs/templates/DISCUSSION.md`, and the
  maintainer contributes in their own round, then records the Outcome in a decision round.
- The maturity ladder (discussion → feature → EPIC → subissues) is not documented anywhere yet;
  documenting it is a follow-up, best done after this discussion has tried it out.

### Maintainer

I agree we can reuse the EPIC and the issue for the discussion. Cameron has been working on this
template system in the Index; he told me in the last weekly meeting, but it may not have been
published yet. I agree we should document the ideas workflow, but that is out of scope now; we can
mention it in the document we write.

How should we proceed? Maybe we can continue discussing, then you write a review on the PR with our
proposal to Cameron, and Cameron converts the PR into a PR that creates the discussion.

### Assistant

Agreed, with one adjustment: the detailed questions belong in the maintainer's own contribution
round, not in the PR review, because the discussion rules say a participant writes only their own
entries in their own round. Sequence:

1. Discuss now; collect the questions in this record (`S1`, `S2`, …), verifying tracker facts.
2. Draft a short request-changes review on PR #2430 asking for conversion into the opening round
   (move the EPIC text verbatim to `initial-proposal.md`, add `README.md` from the template, remove
   the EPIC spec, retitle #2429 and drop its `EPIC` label, name the maintainer as decision owner,
   ask where the Index work lives, note the maturity ladder as a follow-up).
3. The maintainer approves the draft before it is posted.
4. After the opening round merges, prepare the maintainer's contribution round from these topics,
   with AI assistance stated.
5. Decision round later.

The earlier PR review findings (stale counts, issue body out of sync) become moot once the EPIC text
is source material kept as written. The ADR-body concern becomes a topic. The current drift on
`develop` can be fixed in a small separate PR, independent of the discussion, or kept as evidence.

## Part 3 — Session Questions

The questions are numbered `S1`, `S2`, … so they do not collide with the discussion topic IDs
(`Q1`, `Q2`, …) in [`README.md`](README.md).

### S1 — Does generating the ADR index force templates for every ADR?

#### Maintainer

If I get it right, the proposal is to generate Markdown from templates like the ones we already
have, but those templates would contain placeholders. For a concrete problem, take the list of ADRs
in `docs/adrs/index.md`, which can drift from reality. The proposal would be to:

1. Add a new template in `docs/templates` for that file with a placeholder (an array of ADRs with
   their fields).
2. At some point a Rust program generates the final file `docs/adrs/index.md` with the list.

Question: I guess we need to control each template separately; in this case, how would variables
be resolved? We can read the directory and build the final file. However, I guess that would be too
costly: it forces us to also create templates for the ADRs, to separate the fields from the prose.

Working terms:

- **fields**: typed values of documents we want to persist (Cameron wants to do it in JSON);
- **prose**: the rest of the text in the template that does not belong to any typed field of the
  entity.

#### Assistant

Two corrections to the picture:

1. The render template would not live in `docs/templates/`. The EPIC separates *authoring
   templates* (today's `docs/templates/`, starting points copied once) from *render templates*
   (re-rendered on every gate run); their location is left to a subissue.
2. The template does not read the directory. A *data generator* reads the tree (file names,
   frontmatter, headings) and writes a committed JSON dataset, for example an ADR census with one
   record per ADR. The *document generator* renders the template from that JSON only. JSON is
   generated, never hand-edited; hand-authored data goes in TOML inputs.

Answer: generating the index does not require templating the ADRs. Each column of
`docs/adrs/index.md` maps to a source:

| Column            | Source today                                         |
| ----------------- | ---------------------------------------------------- |
| ADR (ID + link)   | File name (`20260617093046_…md`)                     |
| Date              | File-name timestamp                                  |
| Title             | The ADR's `# H1` heading                             |
| Short Description | Nowhere but the index: it is authored only there     |

Three of the four columns can be extracted from the existing files. Scanning 34 files costs
milliseconds; the cost is machinery and a new editing workflow, not runtime. Only the Short
Description needs a machine-readable home: either a new frontmatter field in each ADR (for example
`summary:`; ADR frontmatter today has only `semantic-links`) or a hand-edited TOML input (the EPIC's
stage 1), which keeps the data separate from the ADR it describes.

Templating ADR bodies (the EPIC's Phase 2) is a separate choice from rendering the index; the index
does not need it.

The fields/prose split matches the EPIC's model. The facts show where it bites: fields already
present as metadata (frontmatter, file name, H1) are cheap to extract; fields embedded in prose are
the expensive ones, because extracting them means restructuring the document. That is what drives
the EPIC toward templating the ADRs themselves.

### S2 — How are the generated JSON datasets used, and when are they regenerated?

#### Maintainer

If I get it right, the new Rust program would parse the repository and build a kind of persistence
in JSON, with the entities getting their fields from the files. Once those JSON files are created,
how are they supposed to be used? When are they regenerated?

#### Assistant

**Use.** The datasets are tooling data; the tracker never reads them at runtime. The EPIC gives them
three roles:

1. The only input to rendering: the document generator reads the committed JSON and nothing else
   (not the TOML, the tree, or the forge).
2. A shared input for checkers: validators read the same records through the shared dataset crate
   instead of each re-scanning the tree with its own parser.
3. A review artifact: because the JSON is committed, a pull request shows the data change apart
   from the prose change (for example, `adr-census.json`: one record added).

**Regeneration.** On demand, by whoever changes a source, through two modes:

- `write` regenerates the datasets and renderings and saves them;
- `check` regenerates in memory, compares with the committed bytes, and fails on any difference.

Walk-through for a new ADR:

1. Add `docs/adrs/<timestamp>_new_decision.md`.
2. `check` fails: the ADR census is stale.
3. Run `write`: `adr-census.json` and `docs/adrs/index.md` are updated.
4. Commit all three files.

Where `check` runs (pre-commit, CI, or both) is not specified: the EPIC lists it among the
engineering choices left to a subissue and defers the execution model to EPIC #2003.

**Observations.**

1. The precedent the EPIC cites is not gated. `frontmatter-schema check` exists and passes at
   `41add51ab`, but no git hook, workflow, or test invokes it (`docs/schemas/README.md` documents it
   as a manual command); it stays current only because contributors remember to run `generate`. A
   gate that nothing invokes does not prevent drift, so where and when the gate runs is a core
   design question, not an implementation detail.
2. The committed JSON is a cache of derivable facts. Committing it buys reviewable data diffs and
   decoupled stages; the cost is that every source change touches three files, and parallel pull
   requests that each add an ADR conflict on `adr-census.json` and `docs/adrs/index.md`. The fix is
   to regenerate after rebasing, but it is recurring friction under today's level of parallel work,
   which connects to the timing concern in Part 1.

### S3 — Does the gate catch a missing index row at commit time?

#### Maintainer

In the current AI harness, the time to run tools is the pre-commit and pre-push git hooks. We are
not using other agent tooling yet, even if that is possible. The reason is that we plan everything
in baby steps with small deployable increments, so agents do not do much without committing. That
fits the time to run checks (guardrails) perfectly: at commit. Of course, if we let agents run for
hours without committing, this does not work.

So, suppose I run this check in the pre-commit hook. I have added a new ADR, but the agent forgot
to include it in the list of ADRs. The check recomputes all the JSON (parsing the whole repository)
and finds that the JSON for the list of ADRs is not the same as the committed version. So it has
become stale, and we know we need to recompute it. Is that right?

#### Assistant

Yes, with two refinements and one practical catch.

**Refinement 1 — what the agent "forgot".** In the generated model nobody edits the list by hand,
so the agent cannot forget a row; it forgot to run `write`. The fix is mechanical: run `write` and
commit the regenerated files. The exception is a fact no program can derive, such as the ADR's
Short Description. It must be authored first (in frontmatter or a TOML input), so the generator
should fail with "ADR X has no summary". The gate still sends the agent back to write something,
but it names the exact missing field.

**Refinement 2 — cost.** A full scan at commit is cheap. The hook already runs `linter all` over
every file (markdownlint and cspell each took about 6 s on the merged tree during this review); a
Rust scan of about 1,000 Markdown files adds well under a second once compiled. The real cost is the
first compilation, which the frontmatter validator already pays because the hook runs it through
`cargo run`.

**Practical catch — check the staged snapshot.** A pre-commit gate must check what is being
committed, not the working tree. Otherwise it passes when `write` ran but its output was not staged,
or fails because of unrelated unstaged edits. The frontmatter validator already solves this with
`--staged` and tests for those cases (`contrib/dev-tools/checks/frontmatter-validator/tests/cli.rs`).
A tree-wide generator needs the same discipline, which is harder for it because it reads the whole
tree, not a few staged files.

**Fit with the workflow.** Small committed increments make commit time the natural gate. Hooks can
be skipped (`--no-verify`) or not installed, so CI should run the same check as the backstop,
consistent with "CI is merge authority".

Both approaches catch this example at commit time:

| Aspect                  | Consistency check                    | Generated documentation                                    |
| ----------------------- | ------------------------------------ | ---------------------------------------------------------- |
| Detects the missing ADR | Yes                                  | Yes                                                        |
| Fix                     | The agent adds the row to `index.md` | The agent runs `write` (and adds a summary if missing)     |
| New machinery           | One small check                      | Dataset crate, two generators, templates, editing workflow |
| Files changed per ADR   | 2 (ADR and index)                    | 3 (ADR, dataset, and index)                                |

### S4 — Can hand-edited and generated Markdown coexist during a progressive rollout?

#### Maintainer

To implement this progressively, we could have "normal" Markdown files, edited manually, and
automatically generated Markdown files via templates. Is that right?

#### Assistant

Yes. The EPIC itself plans progressive conversion "by importance", so both kinds coexist for a long
time, possibly permanently: many documents (guides, most ADR prose) may never need generation.

The question hides an open design choice. The EPIC lists "a rendering as a second file or as
generated regions" among the engineering choices left to a subissue, which gives three kinds of
file, not two:

| Kind             | What you edit                                  | Example                                                                       |
| ---------------- | ---------------------------------------------- | ----------------------------------------------------------------------------- |
| Hand-written     | The `.md`                                      | Most documents today                                                          |
| Fully generated  | A template elsewhere; the `.md` is output only | `docs/adrs/index.md` rendered whole from a template                           |
| Generated region | The `.md`, except a marked block               | `docs/adrs/index.md` keeps its hand-written intro; only the table is rendered |

A generated region could look like this:

```markdown
This index lists repository-level ADRs only. ...

<!-- generated:begin adr-index (do not edit; run `write`) -->
| ADR | Date | Title | Short Description |
| ... generated rows ... |
<!-- generated:end adr-index -->
```

Why the choice matters:

- Fully generated: even a typo fix in the prose means editing the template, not the `.md`. This is
  the larger workflow change, and the EPIC's editing boundary rejects any hand edit to a rendering.
- Generated regions: the file stays a normal document and only the restated facts are locked; a
  smaller step. The repository already uses HTML-comment markers (`<!-- skill-link: ... -->`), so
  the pattern is familiar.

Coexistence needs, whichever option is chosen:

1. Every generated file or region is marked ("do not edit; run X"), or people and agents will edit
   the output.
2. The gate knows each template-to-output pairing, so it fails on an orphan output or a hand edit
   (the EPIC covers this: "an unpaired rendering fails").
3. Agent guidance states the rule, for example in `AGENTS.md`: if a file or region is marked
   generated, edit its source.

Generated regions are probably the gentler starting point: they solve the ADR index example without
moving any prose out of the `.md`.

### S5 — Does the input/output model fit living documents such as issue specifications?

#### Maintainer

I see a conflict. This may work for Markdown files that are "output-only", like the list of ADRs,
but most Markdown files have two parts:

- the frontmatter header;
- the body.

Normally the agents edit both parts. And for most Markdown files the body is not standard: it can
start from a template, but it can diverge. For example, issue specifications are used to track the
progress of the issue.

Cameron's proposal somehow assumes that there are two types of Markdown files, input and output:

- input: the ones we collect data from;
- output: the ones we render from data collected in other Markdown files.

And that is a wrong assumption in this repository.

#### Assistant

The conflict is real. One correction to the framing makes it stronger: the EPIC does not assume two
kinds of file. It assumes every converted document splits into an authored source (template prose
plus TOML or frontmatter data) and a generated `.md`, and it includes specifications explicitly
(Class 2: "frontmatter, tables, thread states and related lists render; prose stays authored";
Subissue 4: "Each of the 29 becomes a template"). A converted specification is therefore both input
(read into the census) and output (rendered). Under the EPIC's editing boundary, an agent that ticks
a checkbox, appends a progress-log line, or changes `status` edits the template or the data and runs
`write`; it never touches the `.md`.

Specifications are living documents: agents edit frontmatter and body many times over the issue's
life, and the body diverges from its starting template. Splitting them adds an indirection to every
edit and blurs where a specification's state lives (the rendered frontmatter the data generator
reads, or the template that renders it).

**The line falls per fact, not per file.** The useful question is whether a fact is *original* (this
document is its source of truth) or *restated* (a copy of something held elsewhere). Generation only
helps restated facts. The frontmatter of
`docs/issues/open/2244-2243-review-metric-aggregate-conversions/ISSUE.md`:

| Field                                 | Original or restated                               |
| ------------------------------------- | -------------------------------------------------- |
| `doc-type`, `issue-type`, `priority`  | Original                                           |
| `status`                              | Original, but must agree with the lifecycle folder |
| `github-issue`, `epic`                | Restated from the folder name (`2244-2243-...`)    |
| `spec-path`                           | Restated from the file location                    |
| `branch`                              | Mostly the naming convention                       |
| `related-pr`                          | Restated from the forge                            |
| Body (plan, checkboxes, progress log) | Original                                           |

Most of a specification is original; there is little to generate.

**The repository already verifies restated specification facts.** The frontmatter validator reports
`spec-path-mismatch` when `spec-path` is not the file's location, and reports a `status` that does
not match the lifecycle folder (`contrib/dev-tools/checks/frontmatter-validator/src/repository.rs`,
`contrib/dev-tools/checks/frontmatter-validator/src/diagnostic.rs`). The agent edits the `.md`
directly and the gate fails when a copy drifts: the verification approach, already working here for
living documents.

**Candidate position.** Generate only output-only documents (indexes and catalogs such as
`docs/adrs/index.md` and the template table), preferably as generated regions (S4). Verify, do not
generate, restated facts inside living documents (specifications, ADRs, audit records), extending
the validator's existing pattern.

Question for the author: after a specification is converted, where does its `status` live, and what
exactly does an agent edit to tick a progress checkbox?

### S6 — Clarifying the model: one template per document

#### Maintainer

Maybe I got Cameron's idea wrong. Let me explain myself, and correct me if I'm wrong.

In Cameron's idea you have an authoritative template, for example for the issue specification.

- When you create a new issue, you use that template and you fill in the data.
- However, you do not edit the final file directly (at least the body part). Actually you would have
  two files: the template and the final one that is automatically generated.

Is that right?

That would allow you, for example, to include fields in the body that reference values in the
frontmatter header or other objects in the repository, for example another issue specification's
field. Is that right?

If you want to edit the issue specification, you edit the frontmatter header or the body in the
template, and the final issue specification is generated automatically. Is that right?

#### Assistant

Essentially right, with two corrections and one caveat.

1. **Two different templates.** `docs/templates/ISSUE.md` stays an authoring template, copied once.
   Under the EPIC each converted document gets its own render template ("Each of the 29 becomes a
   template"). A new issue would be copied from the shared skeleton into a new per-issue template,
   and the specification `.md` would be rendered from it. The template is not a form filled with
   data; it is the specification itself, with placeholders only where it restates a fact held
   elsewhere. Per document or per concept is left open ("how many templates, per package or per
   concept"), but the ADR plan points to per document.
2. **Two files: yes, under one option.** Template plus rendering is the "second file" option. The EPIC
   also leaves open "generated regions", where the `.md` stays the edited file and only marked blocks
   render (S4).
3. **References to frontmatter or other repository objects: yes, through the datasets.** The renderer
   reads only committed JSON, never other files. To show another specification's status, the data
   generator first extracts it into a census and the template references that record:

   ```text
   Depends on #2243 ({{ issues["2243"].status }})   →   Depends on #2243 (done)
   ```

   When #2243 changes, the gate fails until `write` refreshes this specification. Under "only the data
   generator derives facts", templates may reference values but not compute them; counts and
   classifications belong in the generator.
4. **Edit the template and the specification regenerates: mostly.**
   - Not automatic on save: someone (person, agent, or hook) runs `write`; otherwise the gate fails.
     Whether the pre-commit hook should run `write` itself, modifying files during a commit, is an
     open choice.
   - Frontmatter is the unclear part of the EPIC. It says frontmatter "renders", so the rendered
     specification's frontmatter is output, yet the data generator reads frontmatter from the tree
     to build the census. If it reads the rendering's frontmatter, the specification's data derives
     from its own output, a loop. The EPIC's "fixed point" claim suggests it is meant to converge,
     but the author should spell it out.

**What this changes in S5.** The corrected picture states the trade-off more precisely than the
input/output framing:

- Benefit: live cross-references. A specification can show another issue's current status, a count,
  or a path, and none of them can go stale.
- Cost: every edit to a living document, even a checkbox, goes through the template and `write`; the
  `.md` you read is never the file you edit.

S5's per-fact observation still holds: most of today's specification content is original, so the
benefit covers a small part of each document. Generated regions would give the cross-reference
benefit without the indirection for the rest.

Questions for the author:

- In the specification case, does the data generator read frontmatter from the template or from the
  rendering, and how is the loop avoided?
- Should the pre-commit hook run `write` automatically, or only `check`?

### S7 — Where does the data in a template's placeholders come from?

#### Maintainer

Where does the data you can put in the placeholders of a Markdown template come from?

#### Assistant

Placeholders read only the committed JSON datasets. The data generator builds those from five kinds of
source (EPIC stage 2: it "reads the inputs and the tree (frontmatter, paths, links, committed captures
and check results)"):

```text
 TOML inputs ───────────────┐
 Repository tree ───────────┤
 Forge captures ────────────┼──► data generator (+ derived facts) ──► JSON datasets ──► placeholders
 Committed check results ───┘
```

| Source                              | What it provides                                | Example in this repository                                                                                         |
| ----------------------------------- | ----------------------------------------------- | ------------------------------------------------------------------------------------------------------------------ |
| TOML inputs (hand-edited)           | Authored data that exists nowhere else          | A template's description in the `docs/index.md` table; an ADR's short description                                  |
| Repository tree                     | File names, paths, frontmatter, headings, links | ADR IDs and dates from file names, titles from H1 headings, a specification's `status`, packages from `Cargo.toml` |
| Forge captures (committed, stamped) | Forge facts                                     | An issue's open or closed state; a pull request's review threads                                                   |
| Committed check results             | Tool output                                     | Not explained in the EPIC (validator findings? coverage?)                                                          |
| Derived facts (generator code)      | Values computed from the above                  | Counts ("29 root ADRs"), classifications, name-to-path resolution                                                  |

Not allowed: the template cannot read TOML, the tree, the forge, or the network, and cannot compute; it
only references dataset values.

**Consequence for forge facts.** Forge data enters only as committed captures stamped "as of" an
instant. The gate compares renderings with the committed capture, not with the forge, so when #2243
closes on GitHub nothing in the tree changes and the gate still passes with the old status. Someone must
run a capture-refresh command (network access and `gh`) and commit the result. The EPIC's own example,
the hand-kept subissue `Status` column, is therefore not fixed: it becomes a stamped, consistent copy
that is still refreshed by hand. More honest, not automatic. A scheduled CI job could refresh captures,
but the EPIC does not describe that machinery.

Questions for the author:

- What are "committed check results", and which documents need them?
- Who refreshes forge captures, when, and how does a reader know a capture is too old?

### S8 — How does templating relate to semantic linking?

#### Maintainer

How is this related to semantic linking? It seems that is another topic. Apparently this resolves
that you can insert values in Markdown files dynamically, so you do not need to maintain them
manually. But when you want to do cross-referencing, I guess we would use a different markup
language inside Markdown, right? If yes, there could be some misunderstanding or conflict. I guess
placeholders are resolved in the final Markdown file and semantic links are not, but in the Markdown
template it can be confusing.

#### Assistant

The intuition is right: different syntax, resolved at different times, both present in a template.

| Aspect             | Semantic linking (EPIC #2264)                                                                             | Templating (#2429)                                                           |
| ------------------ | --------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------- |
| Purpose            | States how two artifacts relate ("affects skill X", "related to finding F1")                              | Inserts a value restated from elsewhere ("29 ADRs", "status: done")          |
| Syntax today       | Frontmatter `semantic-links:`, `<!-- skill-link: x -->` markers, `review-finding:pr-<n>-<id>`             | Not chosen ("the delimiters" are left to a subissue); `{{ ... }}` is typical |
| Planned syntax     | `[[...]]` proposed in the knowledge-graph discussion; #2264 row 7 decides the prose path-reference syntax | Not applicable                                                               |
| In the final `.md` | Kept: validators and agents read it there                                                                 | Gone: replaced by the value                                                  |

Where they meet:

1. The EPIC depends on #2264 already: #2264 "owns what records render against" and supplies "the
   citation targets and the status of prose paths", and the data generator reads "links" from the
   tree, so semantic links become dataset input.
2. Overlap risk: two ways to refer to another artifact. A template could write
   `{{ adr("20260617093046").link }}` (rendered as a Markdown link with the ADR title) while #2264
   defines `[[adr:20260617093046]]` for the same purpose: two resolvers, two syntaxes, two times.
3. Confusion inside a template: `{{ ... }}` is substituted and disappears, while
   `<!-- skill-link: ... -->` and any `[[...]]` stay verbatim. Editors, human or agent, must know
   which is which.

**Delimiter collision.** At `41add51ab`, 13 tracked Markdown files contain `{{`, 10 of them as GitHub
Actions expressions (`${{ matrix.target }}`, `${{ env.... }}`). With a `{{ }}` engine, each converted
document needs escaping, and a missed one fails or renders silently wrong. Delimiters should be
chosen deliberately, not inherited from the engine's default.

**Philosophy.** The knowledge-graph discussion's core principle is "Structure the references, not
the documents": bodies stay free-form and only links become explicit. Templating goes further and
moves restated data out of bodies. The two can coexist (links are edges, templates render facts),
but the order matters. Candidate position: #2264 defines one reference syntax; the template
language does not invent a second one and at most consumes semantic links (for example, rendering a
reference as a link with the target's current title); #2264's path-reference decision (row 7)
precedes the choice of template delimiters.

### S9 — The maintainer's original plan: typed frontmatter, free body

#### Maintainer

My plan, before Cameron proposed this architecture, was to use authoritative Markdown templates for
everything, as is already implemented. All Markdown files have two sections, the frontmatter and the
body:

- The frontmatter holds all the data that is dynamic. For me it is the JSON Cameron wants to store
  elsewhere, but in YAML.
- The Markdown body is the prose. Users can edit it without problems.

Pros:

- We do not need many files. Cameron's design needs extra files: the JSON and the per-document
  template.
- You can edit the Markdown whenever you want. We only need to check the headers.
- No regeneration is needed.

Cons:

- It does not allow inserting data dynamically in the body, which is quite useful, especially for
  what I called the "output-type" documents, like the ADR list. That is a good example for Cameron's
  idea, but the issue specification is not.

What do you think?

#### Assistant

The model is the right default for this repository and composes with the part of the proposal the
maintainer likes, with one correction and two gaps.

**Correction: the datasets are not the frontmatter moved elsewhere.** In this model, frontmatter
stores one document's data. The EPIC's JSON datasets are aggregates across documents (an ADR census,
the tree census), built from frontmatter, file names, and other sources. The plans do not compete at
the data layer: frontmatter can be the source and the dataset a derived aggregate. They differ on the
body and frontmatter of living documents, which the EPIC renders from a per-document template and this
model keeps hand-edited.

**Agreement.** The pros are real, and S5 supports them: most specification content is original, and
the repository already runs this model. Authoring templates exist (`docs/templates/`); frontmatter has
a typed Rust model and a validator in the pre-commit hook (EPIC #2264, partly delivered); and the
validator already verifies restated frontmatter facts (`spec-path` against the location, `status`
against the lifecycle folder).

**Gap 1: frontmatter drifts too.** Some fields restate other facts (`github-issue` and `epic` restate
the folder name; `related-pr` and `status` restate the forge). This model localizes that drift to a
checkable place; it does not remove it. Those fields need verification, as the validator already does
for two of them.

**Gap 2: living-document bodies contain restated facts as well, not only output-type documents.**
Examples: paths and `file:line` citations (the EPIC itself has many, such as `profile.rs:197`), counts,
the `AGENTS.md` package catalog (which restates the workspace members), and `Version: 3.0.0-develop` in
`AGENTS.md`. These need verification, which is EPIC #2264 rows 7–8 (path-reference scope, syntax, and
validation). "We only need to check the headers" becomes "check the headers and machine-checkable
references in the body".

**Synthesis.**

| Document class                                       | Model                        | Mechanism                                                                      |
| ---------------------------------------------------- | ---------------------------- | ------------------------------------------------------------------------------ |
| Living (specifications, ADRs, skills, audit records) | Typed frontmatter, free body | Restated facts verified, not generated                                         |
| Output-only (indexes, catalogs, counts)              | The EPIC's rendering         | Generated regions (S4) whose data comes from the living documents' frontmatter |

The ADR example then needs no TOML and no per-ADR template:

1. Add `summary:` to an ADR frontmatter profile (EPIC #2264 row 3 already plans ADR profiles).
2. Render only the table region of `docs/adrs/index.md` from ADR file names, H1 headings, and
   `summary`.
3. Gate it in pre-commit (`--staged`) and CI.

**Open question: is the committed dataset layer needed at first?** In this synthesis the renderer can
read frontmatter directly. Committed JSON buys reviewable data diffs and a shared checker interface; it
costs a third file per change and more merge conflicts (S2). It could be deferred until a concrete need
appears. Worth asking the author, because his design makes the renderer read only JSON.

**Suggestion.** Instead of an EPIC, run one small experiment: render the ADR index region from
frontmatter. It tests the hard questions (gate placement, staged snapshot, delimiters, whether the
dataset layer pays for itself) on one document before committing to the full design.

## Part 4 — Publishing

### Maintainer

I started seeing that both approaches solve different points and can be combined well. I think we
are at a good point to publish something and let Cameron review it to start a second round. Do you
still think we should convert this into a discussion? I see it differently now; I see that it could
fit, but anyway I would not implement it before releasing tracker version 4 and closing the work in
progress. We have too many things in progress. And I think opening the discussion is good for the
community anyway: it is documentation for future contributors and AI agents about why we reached
this kind of AI harness.

### Assistant

Agreed on all three points. The combination is stronger than either approach alone: typed
frontmatter with a free body and verification for living documents, the EPIC's rendering for
output-only documents, joined by frontmatter as the data source of generated regions. A discussion
fits better now: the design is not settled (the combination revises the EPIC's binding design, and
S6–S9 carry open questions for the author), an EPIC with a binding design would go stale while the
work waits for v4, and the discussion records why the harness took its shape.

The publishing order changed from the one agreed in Part 2: instead of waiting for the author to
convert PR #2430 into the opening round, the maintainer opens the discussion with this record, links
the proposal rather than copying it, and asks the author on PR #2430 to rework it into his own round.
The maintainer chose this order.
