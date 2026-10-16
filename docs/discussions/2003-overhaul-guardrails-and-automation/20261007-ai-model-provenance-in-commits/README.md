---
semantic-links:
  related-artifacts:
    - "issue #2003"
    - "issue #2458"
    - docs/discussions/2003-overhaul-guardrails-and-automation/20261003-goals-and-boundaries/README.md
    - AGENTS.md
    - .github/skills/dev/git-workflow/commit-changes/SKILL.md
    - .github/agents/committer.agent.md
    - docs/adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md
    - docs/discussions/AGENTS.md
    - docs/templates/DISCUSSION.md
---

<!-- cspell:ignore Deployers tokenizer -->

# AI Model Provenance in Commits

| Field          | Value                                                                                                                                                                                                                                                          |
| -------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Status         | Open for rounds                                                                                                                                                                                                                                                |
| Started        | 2026-10-07                                                                                                                                                                                                                                                     |
| Opened by      | Jose Celano (`josecelano`), who raised the concern and its three purposes (tracking, analytics, legal)                                                                                                                                                         |
| Decision owner | Cameron (`da2ce7`), assignee of EPIC #2003                                                                                                                                                                                                                     |
| Informs        | EPIC #2003 - Overhaul: Automation Tools and AI Agent Guardrails                                                                                                                                                                                                |
| Scope          | Aspect 2 (the commit-metadata contract and its format gate) and aspect 3 (the models an orchestration routes work to) of [Goals and Boundaries](../20261003-goals-and-boundaries/README.md): how a commit records which AI models were involved in its changes |
| AI assistance  | AI assistant using the Copilot SDK in VS Code, model `claude-opus-5.5` (research and draft)                                                                                                                                                                    |

## Context

This discussion was opened in [PR #2467](https://github.com/torrust/torrust-tracker/pull/2467) with
a single-PR structure. It adopted the [discussion template](../../../templates/DISCUSSION.md) in a
round by its opening author, under the round rules in
[`docs/discussions/AGENTS.md`](../../AGENTS.md). The context below is unchanged; the open questions
moved into [Topics](#topics) without changes to their words, and each topic gained the template's
"Added by" attribution.

Cameron (`da2ce7`) posted a draft position on these questions as a pull-request comment on
PR #2467: <https://github.com/torrust/torrust-tracker/pull/2467#issuecomment-6035743101>. Its Part
C answers the topics: C1 (Q1), C2 (Q2), C3 (Q3 and Q4), C4 (Q5 and Q6), C5 (Q7), C6 (Q8), and C7
(Q9). Under the round rules it is linked here, not copied, until he adds it in his own round.

### Why This Discussion

On 2026-10-06, while preparing the spec-only PR #2461 for issue #2458, an AI agent running in the
Copilot SDK for VS Code added this trailer to three commits:

```text
Co-authored-by: Copilot <223556219+Copilot@users.noreply.github.com>
```

The trailer came from a default instruction in the agent harness, not from this repository. No
repository skill, agent definition, or `AGENTS.md` rule asks for it. At `develop` `7836471b3`, the
history has eight `Co-authored-by` trailers, all naming humans, and none naming an AI identity. At
the maintainer's request, the three commits were reworded without the trailer, re-signed, and
force-pushed before the PR was merged.

The trailer is gone, but the question it raised is open: what should a commit record about the AI
models involved in it? This discussion collects the background, the precedent, and the options, so
that the owner of EPIC #2003 can decide before anything is implemented.

Nothing here changes a specification or a rule. A conclusion takes effect only when the owner of
EPIC #2003 records it.

### Why the Default Trailer Is Only Half Useful

1. **It records who made the commit, not who wrote the changes.** The harness adds the trailer
   whenever the agent runs `git commit`. A commit made by an agent can contain changes a human
   wrote, and a commit a human makes can contain changes an AI wrote. The trailer tells you
   neither.
2. **"Copilot" is a product, not a model.** Copilot can route work to models from several
   vendors. The trailer does not say which model or version produced the content.
3. **More than one model can work on one commit.** In the same session, the harness switched the
   main model from `gpt-5.6-terra` to `claude-opus-5.5` partway through. The spec content in the
   PR #2461 commits came from the first model, and the commit rewording came from the second.
   Subagents (explore, review, and task agents) can also run on other models. One product name
   cannot describe that.
4. **Each harness has its own default.** Other agent tools add different trailers or none. The
   history would mix incompatible conventions that a tool cannot reliably parse.
5. **It displays the bot as a co-author.** GitHub shows `Co-authored-by` identities as commit
   co-authors. That reads as shared authorship and accountability, which an AI model cannot hold.

### What the Project Wants from Attribution

- **Tracking:** know, for any change, which models were involved and in what role.
- **Analytics:** compare models with review findings, regressions, and rework. For example: which
  model's changes needed the most review fixes?
- **Legal and compliance readiness:** keep an auditable provenance record of AI involvement. The
  EU AI Act transparency rules are the main reason to ask this now (see below).

### Precedent in Other Projects

Checked on 2026-10-06 on the projects' own pages.

- **Linux kernel**, [AI Coding Assistants](https://docs.kernel.org/process/coding-assistants.html):
  - AI agents must not add `Signed-off-by`. Only humans can certify the Developer Certificate of
    Origin.
  - Contributions should include `Assisted-by: LLM [TOOL1] [TOOL2]`, where the tools are optional
    specialized analysis tools. Basic development tools are not listed.
  - Some secondary sources quote an older `Assisted-by: AGENT_NAME:MODEL_VERSION` form. The live
    page no longer shows it.
- **Fedora**,
  [AI-Assisted Contributions Policy](https://docs.fedoraproject.org/en-US/council/policy/ai-contribution-policy/):
  - Contributors must disclose AI use when a significant part of the contribution is taken from a
    tool without changes, and should disclose other uses where useful.
  - The recommended method for git is an `Assisted-by:` commit trailer, for example
    `Assisted-by: ChatGPTv5`.
  - The contributor is always the author and is fully accountable.

Both projects use `Assisted-by`, not `Co-authored-by`, and both keep a human accountable. Neither
requires a precise model identifier or more than one model per commit. The purposes above would
take this project further than both.

### EU AI Act

This section needs legal review and is not legal advice.

The transparency obligations in Article 50 of Regulation (EU) 2024/1689 apply from
2 August 2026, according to secondary legal summaries such as
[AI Act Explorer](https://artificialintelligenceact.eu/transparency-rules-article-50/).
As summarized there:

- The obligations fall mainly on **providers** of AI systems, for example machine-readable marking
  of generated content.
- **Deployers** have obligations in specific cases, such as deepfakes and text published to
  inform the public on matters of public interest.
- Code is not named explicitly.

Whether an open-source project that commits AI-assisted code has any obligation is **not
established**. The project should not claim compliance. It can record the question for legal input
and design provenance metadata that would support compliance if it turns out to be needed.

### Options

| ID  | Option                                                         | Pros                                                                 | Cons                                                                |
| --- | -------------------------------------------------------------- | -------------------------------------------------------------------- | ------------------------------------------------------------------- |
| O1  | Keep provider defaults (status quo)                            | No work                                                              | Misleading; no model identity; inconsistent between harnesses       |
| O2  | No AI trailers; disclose AI use in the PR body only            | Simple; human-readable                                               | Lost from the history, which matters for code that leaves the PR    |
| O3  | `Assisted-by:` trailer per model, kernel/Fedora-compatible     | Follows precedent; parseable with `git log --format='%(trailers)'`   | No role information; depends on accurate model identifiers          |
| O4  | O3 plus a role-aware provenance record in issue-local evidence | Full analytics (models per task and role) while commits stay concise | Two places to keep consistent; needs harness support to be reliable |

Commit trailers stay with the code after merge, because this repository does not squash. That is
the main argument against O2.

### Draft Conclusions

These are the drafting agent's proposals, not decisions.

1. **Reserve `Co-authored-by` for humans.** AI identities never appear in it, because a model
   cannot hold authorship or accountability.
2. **Adopt O4.** Add one `Assisted-by:` trailer for each model whose output is in the diff, using
   an identifier such as `<harness>:<vendor>/<model-id>` (for example
   `copilot-sdk-vscode:anthropic/claude-opus-5.5`). Keep models that only explored, reviewed, or
   planned out of the trailers, and record them with their role in issue-local evidence.
3. **Take identifiers from the harness, not from the model.** A model cannot reliably state its
   own exact version. When the identifier is unknown, write `unknown` rather than guessing, so the
   record never looks more complete than it is.
4. **Make no compliance claim.** Treat the metadata as provenance until legal input says
   otherwise.
5. **Add an interim rule now.** Until the decision is recorded, `AGENTS.md` tells agents not to add
   provider-default AI trailers.

### Follow-up Work if Accepted

An issue specification, created after the outcome is recorded, carries the decision:

- An ADR in `docs/adrs/`, because the rule is repository-wide, with the chosen option, the
  identifier format, and the rejected alternatives. It must not depend on one provider's harness,
  per the
  [AI-agent context, capability, and portability governance ADR](../../../adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md).
- Updates to `AGENTS.md`, the `commit-changes` skill, and the Committer agent describing the rule,
  including how agents get model identifiers and what to write when one is unknown.
- If enforcement is chosen, a format check with unit tests, written in Rust if its logic is more
  than trivial.
- Manual checks that a multi-model session produces the agreed trailers and that
  `git log --format='%(trailers:key=<key>,valueonly)'` gives per-model counts without parsing
  free text.

Rewriting historical commits, giving legal advice, and changing the GPG-signing policy are out of
scope.

## Topics

### Q1 - Granularity

Should provenance live in commit trailers, in the PR body, in issue-local evidence, or in a
combination? Added by Jose Celano.

#### Positions

##### Q1 - Cameron Garnham (`da2ce7`), 2026-10-08, PR #2486

**`granularity-by-role` — Granularity (Q1).** Commit trailers for project automation under `project-automation-is-a-dependency`; the pull-request body and issue-local evidence for anything optional or role-specific; no mandatory per-commit record for individuals, by `personal-toolchains-are-private`.

First posted as [comment 6035743101 on PR #2467](https://github.com/torrust/torrust-tracker/pull/2467#issuecomment-6035743101) on 2026-10-07 under the number label `C1`.

AI assistance not disclosed: the author's private tooling may include machine-learning tools.

##### Q1 - Jose Celano (`josecelano`), 2026-10-13, PR #2502

I agree with [Cameron's entry](#q1---cameron-garnham-da2ce7-2026-10-08-pr-2486) that
individuals should not be required to declare their personal tools, and that commit trailers are
right for project automation. My concern is the boundary between the two. Most of my commits in
this repository are made by an AI agent running in my editor, but driven by this repository's own
agents, skills, and `AGENTS.md`. If those commits count as personal toolchain use, they carry no
model record, and the tracking and analytics purposes in the Context cannot be met: a comparison of
models built on optional declarations measures who chose to declare, not how the models performed.
I propose that these commits carry an `Assisted-by:` trailer per model whose output survives in
the diff, written by the agent at commit time (see my Q11 entry).

Drafted with an AI assistant using the Copilot SDK in VS Code (`claude-opus-5.5`); reviewed and
adopted by Jose Celano.

### Q2 - Which models count

Only models whose output is in the diff, or also models that explored, reviewed, or planned? If
reviewers count, they need a separate key so they are not confused with authors. Added by Jose
Celano.

#### Positions

##### Q2 - Cameron Garnham (`da2ce7`), 2026-10-08, PR #2486

**`models-whose-content-survives` — Which models count (Q2).** Those whose generated content survives in the diff, by `scope-by-content-not-presence`. Models that only explored, reviewed or planned are excluded; a reviewer role, if ever recorded, uses a separate key so that it is never read as authorship.

First posted as [comment 6035743101 on PR #2467](https://github.com/torrust/torrust-tracker/pull/2467#issuecomment-6035743101) on 2026-10-07 under the number label `C2`.

AI assistance not disclosed: the author's private tooling may include machine-learning tools.

### Q3 - Identifier format

Is `<harness>:<vendor>/<model-id>` acceptable, and how should unknown versions be recorded? Added by
Jose Celano.

#### Positions

##### Q3 - Cameron Garnham (`da2ce7`), 2026-10-08, PR #2486

**`identifier-from-the-runtime-not-the-model` — Identifier format (Q3).** Small questions once `scope-by-content-not-presence` and `project-automation-is-a-dependency` hold. Identifiers come from the runtime that ran the model, never from the model's own account of itself, with `unknown` when that runtime cannot name the model, so that the record never looks more complete than it is.

First posted as [comment 6035743101 on PR #2467](https://github.com/torrust/torrust-tracker/pull/2467#issuecomment-6035743101) on 2026-10-07 under the number label `C3`.

AI assistance not disclosed: the author's private tooling may include machine-learning tools.

### Q4 - Trailer key

`Assisted-by`, for compatibility with the kernel and Fedora, or a project-specific key that adds the
role, such as `AI-Model:` or `Generated-by:`? Added by Jose Celano.

#### Positions

##### Q4 - Cameron Garnham (`da2ce7`), 2026-10-08, PR #2486

**`precedent-key-acceptable` — Trailer key (Q4).** Small questions once `scope-by-content-not-presence` and `project-automation-is-a-dependency` hold. The precedent key is acceptable.

First posted as [comment 6035743101 on PR #2467](https://github.com/torrust/torrust-tracker/pull/2467#issuecomment-6035743101) on 2026-10-07 under the number label `C3`.

AI assistance not disclosed: the author's private tooling may include machine-learning tools.

### Q5 - Source of truth

The harness knows which models ran, through model-switch notices, subagent configuration, and
session logs. Who is responsible for collecting the list: the agent, the harness, or the human
committer? Added by Jose Celano.

#### Positions

##### Q5 - Cameron Garnham (`da2ce7`), 2026-10-08, PR #2486

**`tracked-configuration-is-the-source-of-truth` — Source of truth (Q5).** For project automation the tracked configuration of that automation is the source of truth, by `project-automation-is-a-dependency`.

First posted as [comment 6035743101 on PR #2467](https://github.com/torrust/torrust-tracker/pull/2467#issuecomment-6035743101) on 2026-10-07 under the number label `C4`.

AI assistance not disclosed: the author's private tooling may include machine-learning tools.

### Q6 - Human edits and rewrites

How is the list kept correct when a human edits AI output, when commits are reworded or rebased, or
when changes move between commits? Added by Jose Celano.

#### Positions

##### Q6 - Cameron Garnham (`da2ce7`), 2026-10-08, PR #2486

**`human-edits-need-no-correction` — Human edits (Q6).** For human edits, rewrites and rebases, no correction of a model record is required, because the record is a tool record and the human's certification governs, by `acceptance-is-the-choice` and `tool-record-not-authorship`.

First posted as [comment 6035743101 on PR #2467](https://github.com/torrust/torrust-tracker/pull/2467#issuecomment-6035743101) on 2026-10-07 under the number label `C4`.

AI assistance not disclosed: the author's private tooling may include machine-learning tools.

### Q7 - Enforcement

Document the rule only, or also validate the trailer format in a `commit-msg` check? A tool can
check the format of a declared trailer. It cannot check whether a declaration is complete. Added by
Jose Celano.

#### Positions

##### Q7 - Cameron Garnham (`da2ce7`), 2026-10-08, PR #2486

**`format-checkable-completeness-not` — Enforcement (Q7).** A format check on declared trailers is admissible, because form is checkable; a completeness check is not, because completeness is not, by `tool-record-not-authorship`. The role-aware evidence record of option O4 is optional for the same reason.

First posted as [comment 6035743101 on PR #2467](https://github.com/torrust/torrust-tracker/pull/2467#issuecomment-6035743101) on 2026-10-07 under the number label `C5`.

AI assistance not disclosed: the author's private tooling may include machine-learning tools.

### Q8 - Interim rule

Should `AGENTS.md` forbid provider-default AI trailers before the decision? Added by Jose Celano.

#### Positions

##### Q8 - Cameron Garnham (`da2ce7`), 2026-10-08, PR #2486

**`no-provider-default-trailers-meanwhile` — Interim rule (Q8).** Until the Outcome is recorded, agents do not add provider-default AI trailers, consistent with `humans-author-and-certify`.

First posted as [comment 6035743101 on PR #2467](https://github.com/torrust/torrust-tracker/pull/2467#issuecomment-6035743101) on 2026-10-07 under the number label `C6`.

AI assistance not disclosed: the author's private tooling may include machine-learning tools.

### Q9 - Legal input

Who reviews the EU AI Act question, and does the answer change the required level of detail? Added
by Jose Celano.

#### Positions

##### Q9 - Cameron Garnham (`da2ce7`), 2026-10-08, PR #2486

**`legal-input-parked` — Legal input (Q9).** Parked, by `regulation-targets-systems-acting-on-people` and `provisional-and-regulation-independent`. The rule is written so that a later legal answer changes the level of detail recorded, not the structure of the rule.

First posted as [comment 6035743101 on PR #2467](https://github.com/torrust/torrust-tracker/pull/2467#issuecomment-6035743101) on 2026-10-07 under the number label `C7`.

AI assistance not disclosed: the author's private tooling may include machine-learning tools.

### Q10 - What a provenance record can truthfully contain

What any record of model involvement can truthfully state, whatever its format and location: the premises that the entries under Q1 to Q9 rest on. None of the opening author's open questions covers this; the topic is new in this round. Added by Cameron Garnham (`da2ce7`).

#### Positions

##### Q10 - Cameron Garnham (`da2ce7`), 2026-10-08, PR #2486

**`enumeration-has-no-boundary` — Provenance by enumeration has no stopping point.** Every current toolchain contains a machine-learning model at some layer: spelling and grammar correction, machine translation, code completion, search ranking, and the review bot that reads this pull request. A rule that asks for "the models involved in a change" therefore has no boundary, and a declaration made under it is false by omission at the moment it is made. Neither the EU AI Act nor any current standard supplies a classification that would draw the boundary for us, and none should be expected soon.

**`acceptance-is-the-choice` — Review converts a model's output into the reviewer's choice.** When a model drafts a line and a human reads it, judges it and keeps it, the only fact a repository can record is that the certifying human adopted it. This is the logic of the Developer Certificate of Origin: whoever certifies a change has taken responsibility for it, whatever produced its first draft. "I would have written the same" is the acceptance test, and acceptance is the choice. A record that tries to see past acceptance into the origin of a phrasing records something no reviewer can verify.

**`personal-toolchains-are-private` — Personal toolchains are private.** A contributor's editor, assistant or translation tool is their own business. A mandatory per-model declaration from individuals yields silent non-compliance, which is worse than no data, because it makes the record look complete when it is not. The contrast is the project's own automation (see `project-automation-is-a-dependency`), which is public by construction and can be declared precisely. Provenance is strong exactly where it is public, and absent exactly where it would be private.

**`watermarks-mark-rendering-not-authorship` — Watermarks record phrasing, not authorship.** Article 50 makes machine-readable marking of generated output a provider obligation, and the temptation will be to read a watermark as provenance. A watermark records that a span of text passed through a model's tokenizer at some point. It records nothing about who thought the content, who directed it, or who accepted it: a sentence a human wrote and had translated or spell-checked carries the mark, while a model's idea retyped by hand does not. Read as authorship, the mark inverts `acceptance-is-the-choice`, because it detects the instrument's touch and ignores the certifying human, and it is attached by the provider rather than declared by the accountable party. It also cannot be removed without stripping legitimate human edits.

**`regulation-targets-systems-acting-on-people` — The regulation addresses systems that act on people.** The transparency obligations of Article 50 are written for systems whose decisions have consequences for people who did not choose them: surveillance, synthetic media, public-interest text. A commit acts on nobody until a human accepts and certifies it, at which point the consequence is the human's by `acceptance-is-the-choice`. The document's own statements that code is not named, that an obligation is not established, and that no compliance claim is made are its strongest, and any rule written here should continue to need them.

First posted as [comment 6035743101 on PR #2467](https://github.com/torrust/torrust-tracker/pull/2467#issuecomment-6035743101) on 2026-10-07 under the number labels `A1`, `A2`, `A3`, `A4` and `A5`.

AI assistance not disclosed: the author's private tooling may include machine-learning tools.

### Q11 - The shape of the rule

The proposed Outcome as one rule, whose consequences for Q1 to Q9 the entries under those topics state. None of the opening author's open questions covers this; the topic is new in this round. Added by Cameron Garnham (`da2ce7`).

#### Positions

##### Q11 - Cameron Garnham (`da2ce7`), 2026-10-08, PR #2486

**`humans-author-and-certify` — Humans author and certify.** The co-author trailer is reserved for human identities. A model cannot hold authorship or accountability. Standing rules and direction belong to the human who sets them, so what a model produces under them is a rendering of that intent, not an authorship claim by the model.

**`tool-record-not-authorship` — Models are instruments; the record is a tool record.** Any record of model involvement is an instrument record, of the same kind as a toolchain version in a verification evidence file. Its purposes are analytics and reproducibility of process. It makes no authorship claim and no compliance claim. It is a format contract, not a completeness claim: a check can verify the form of a declaration and never that the declaration is complete.

**`scope-by-content-not-presence` — Scope is bounded by content, not by presence.** A declaration names a model only where that model's generated text or code survives in the diff. Rendering and checking tools applied to the author's own content (spelling, grammar, formatting, translation of the author's own prose) and advisory use (exploration, review, planning) are excluded by definition. Under this rule the question "where do I declare my spell-checker" has the answer "nowhere, by rule", and the enumeration problem of `enumeration-has-no-boundary` does not arise.

**`project-automation-is-a-dependency` — Project automation is a dependency; personal tools are not.** A public process attracts provenance of its own accord: a project tracks what its automation uses for the same reason it ships a lockfile, to constrain the scope of its dependencies, and under this rule a model that acts inside that automation is one of them, so it can be named exactly where a personal tool cannot. The record's subject is not that a model was used but the choices it may make in that process, such as certifying a new tagged release; this rule proposes to pin such a model's identity so that a choice can be traced to it. An AI model that the project's own automation uses (the Copilot review handler, the orchestrated review lanes, any future agent in a workflow) is a dependency of the same kind: its identity is pinned in tracked bytes, changed only through a pull request, and checkable by a gate, in the way a lockfile pins a crate version. That record is an identity record, not a reproducibility guarantee, because a vendor may change a model under an unchanged identifier and a run is not reproducible even at a fixed one; the record states this limit. A contributor's personal tool is not a project dependency, so the project neither pins nor records it; a coarse declaration in the form used by the Linux kernel and Fedora, without a model name, remains optional for anyone who wishes to give one.

**`declared-not-detected` — Provenance is declared, not detected.** The record consists of what the committing human and the project's pinned automation declare. Watermarks and detectors are not read as provenance, because by `watermarks-mark-rendering-not-authorship` they mark rendering rather than authorship, and a mark on a human's edited sentence states nothing the record should repeat.

**`provisional-and-regulation-independent` — The rule is provisional and must not depend on the regulation's terms.** The Outcome is revisited when a standard supplies a classification of model involvement. Nothing in the rule may need to know what the regulation means by "choice", "generated" or "provider"; a future draft that does is over-reaching, by `regulation-targets-systems-acting-on-people`.

First posted as [comment 6035743101 on PR #2467](https://github.com/torrust/torrust-tracker/pull/2467#issuecomment-6035743101) on 2026-10-07 under the number labels `B1`, `B2`, `B3`, `B4`, `B5` and `B6`. `project-automation-is-a-dependency` revises `B4`'s opening sentence in this round (PR #2486 review findings F1 and R1).

AI assistance not disclosed: the author's private tooling may include machine-learning tools.

##### Q11 - Jose Celano (`josecelano`), 2026-10-13, PR #2502

I accept `humans-author-and-certify`, `tool-record-not-authorship`,
`scope-by-content-not-presence`, and `declared-not-detected` from
[Cameron's entry](#q11---cameron-garnham-da2ce7-2026-10-08-pr-2486). I propose one amendment to
`project-automation-is-a-dependency`: a commit made by an AI agent that runs this repository's
tracked agent definitions or skills is treated as project automation for the purpose of the
record, even when the agent runs in a contributor's local harness. For such commits the agent
writes the trailer from the harness's model identifier, or `unknown`, when it commits.

The arguments against mandatory declaration do not apply to this case.
`enumeration-has-no-boundary` and `personal-toolchains-are-private` are about a human listing
every tool by hand: an agent writing its own model identifier lists nothing by hand, and the rule
is followed whenever the repository's commit instructions are followed, not when a person
remembers. Editors, spell checkers, translation, and work a human writes without an agent stay out
of scope, as `scope-by-content-not-presence` already provides.

Question for the decision owner: if this amendment is not accepted, which of the project's
purposes (tracking, analytics) does the rule still serve for agent-made commits, and how?

Drafted with an AI assistant using the Copilot SDK in VS Code (`claude-opus-5.5`); reviewed and
adopted by Jose Celano.

##### Q11 - Cameron Garnham (`da2ce7`), 2026-10-16, PR #TBD

This entry answers the amendment to `project-automation-is-a-dependency` and the question for the decision owner in the [entry above](#q11---jose-celano-josecelano-2026-10-13-pr-2502); it replaces no earlier entry.

**`custody-not-instructions` — Custody, not instructions.** The line between project automation and personal tooling is custody, not instructions. Project automation is what the project runs and can pin in tracked bytes, change through a pull request and check with a gate. A contributor's runtime that follows the repository's tracked skills does not come under the project's custody: the skills are text that a human-controlled tool chooses to follow, and the project cannot pin, gate or even observe what ran them.

**`self-declaration-is-not-a-dependency-record` — A self-declaration is not a dependency record.** Treating such a commit "as project automation for the purpose of the record" records a dependency the project does not have. The commit is made under the contributor's own account with a model the project never chose, and a trailer the agent writes about itself is a self-declaration: a check can verify its form and never its completeness (`tool-record-not-authorship`, `format-checkable-completeness-not`). Writing it from "the harness's model identifier" names the runtime in the sense of `harness-as-agent-runtime` (Q13), which is exactly the sense `personal-toolchains-are-private` keeps out of the record.

**`every-editor-becomes-automation` — Every agent-enabled editor becomes automation.** Under the amendment, every contributor who opens the repository in an agent-enabled editor becomes project automation at their first commit, because what triggers the rule is following the tracked skills, not the content of the change. The amendment's exclusions (editors, spell checkers, translation, unassisted writing) do not prevent this, since `scope-by-content-not-presence` bounds the record by content while the amendment's trigger is presence.

**`purposes-are-served-by-custody` — The purposes are served by custody.** This is my answer to the question put to the decision owner. For commits made with personal tooling the rule serves tracking and analytics not at all, by design: that is what `project-automation-is-a-dependency` means by "A contributor's personal tool is not a project dependency". Tracking and analytics are served for the automation the project runs, where a model's identity is pinned in tracked bytes and a choice can be traced to it. Adding unpinned self-declarations to that set makes the history's record less useful, not more, because a reader cannot tell a pinned identity from a declared one.

AI assistance not disclosed: the author's private tooling may include machine-learning tools.

### Q12 - AI-assistance disclosure in discussion rounds

The round rules in [`docs/discussions/AGENTS.md`](../../AGENTS.md) require an entry to say when an
AI assistant drafted or researched it, and to name the tool and model when known.
`personal-toolchains-are-private` holds that a contributor's tools are their own business, and in
[PR #2486](https://github.com/torrust/torrust-tracker/pull/2486#discussion_r4229731579) its author
declined to say whether his entries were AI-assisted; each of his entries states instead that AI
assistance is not disclosed. Should the disclosure rule for discussion entries follow the
commit-provenance Outcome, or stay as it is? Added by Jose Celano.

#### Positions

##### Q12 - Jose Celano (`josecelano`), 2026-10-13, PR #2502

Discussion entries are different from commits. A commit is judged by its diff and its tests; a
discussion entry is judged by its reasoning, and a reader weighing an argument is entitled to know
whether a person or a model produced it. I would keep the disclosure rule for discussion entries,
but accept an explicit "not disclosed" statement as compliant, so that the rule asks for honesty
about disclosure rather than for an enumeration of tools. Whatever the Outcome decides for commits,
a change to the round rules belongs to the rounds convention in `docs/discussions/AGENTS.md`,
introduced by [issue #2473](https://github.com/torrust/torrust-tracker/issues/2473), not to this
discussion's Outcome alone.

Drafted with an AI assistant using the Copilot SDK in VS Code (`claude-opus-5.5`); reviewed and
adopted by Jose Celano.

### Q13 - What "harness" names

This discussion and the repository use the word "harness" in three senses and define it nowhere, and the scope of the Q11 amendment to `project-automation-is-a-dependency` turns on which sense is meant. Added by Cameron Garnham (`da2ce7`).

#### Positions

##### Q13 - Cameron Garnham (`da2ce7`), 2026-10-16, PR #TBD

**`harness-as-agent-runtime` — The agent runtime.** The product that runs a model and commits for a person: it switches models mid-session, adds default trailers and exposes a model identifier. This is the sense of this discussion's [Context](#context) ("the agent harness", "The harness adds the trailer", "the harness switched the main model"), of the identifier format `<harness>:<vendor>/<model-id>` under [Q3](#q3---identifier-format), and of my own Q3 entry in its first wording, corrected in this round to `identifier-from-the-runtime-not-the-model`. The project neither configures nor pins it; it is personal tooling under `personal-toolchains-are-private`. It is the only sense in which a "harness" can name a model identifier.

**`harness-as-repository-dev-tools` — The repository's development tools.** The tracked Rust tools under `contrib/dev-tools/` (validators, report-contract checks, coverage checks), which the [draft issue to separate them into their own Cargo workspace](../../../issues/drafts/2003-separate-ai-harness-cargo-workspace/ISSUE.md) and the [`add-workspace-member` skill](../../../../.github/skills/dev/maintenance/add-workspace-member/SKILL.md) call the "AI-harness tools". These are tracked bytes, built and gated: a project dependency in the sense of `project-automation-is-a-dependency`. They run no model.

**`harness-as-process-layer` — The process layer.** The skills, hooks, templates and conventions together, as the [templated-documentation discussion](../../2429-templated-documentation/20261009-templated-documentation-design/README.md) ("the AI harness built on top of their original code", "In the current harness") and its [maintainer-review session](../../2429-templated-documentation/20261009-templated-documentation-design/maintainer-review-session.md) ("In the current AI harness") use the word. These are instructions that a tool may follow, not an executor; following them transfers nothing about who runs the tool.

**`undefined-term-carries-no-rule` — An undefined term carries no rule.** A rule whose scope is set by a word that the text never defines and uses in three senses cannot be assessed, and a paragraph that lets the word change sense between its clauses is not sound in the sense the merge gate asks for. My own Q5 entry in its first wording scoped the word to project automation and is corrected in this round to `tracked-configuration-is-the-source-of-truth`, while the Q11 amendment needs the first sense for "the harness's model identifier" and the third for "runs this repository's tracked agent definitions or skills", and slides between them inside one paragraph. The discussion's own identifier format treats the word as a vendor-product name, the repository has an issue draft titled with it in a second sense, and the design discussions use it in a third. Before any rule is hung on the word, the discussion should define it once: I offer the three labels above as that vocabulary, and ask that the bare word be used in no rule.

AI assistance not disclosed: the author's private tooling may include machine-learning tools.

## Outcome

Pending: no decision recorded.
