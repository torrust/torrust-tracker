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

### Q2 - Which models count

Only models whose output is in the diff, or also models that explored, reviewed, or planned? If
reviewers count, they need a separate key so they are not confused with authors. Added by Jose
Celano.

#### Positions

##### Q2 - Cameron Garnham (`da2ce7`), 2026-10-08, PR #2486

**`models-whose-content-survives` — Which models count (Q2).** Those whose generated content survives in the diff, by `scope-by-content-not-presence`. Models that only explored, reviewed or planned are excluded; a reviewer role, if ever recorded, uses a separate key so that it is never read as authorship.

First posted as [comment 6035743101 on PR #2467](https://github.com/torrust/torrust-tracker/pull/2467#issuecomment-6035743101) on 2026-10-07 under the number label `C2`.

### Q3 - Identifier format

Is `<harness>:<vendor>/<model-id>` acceptable, and how should unknown versions be recorded? Added by
Jose Celano.

#### Positions

##### Q3 - Cameron Garnham (`da2ce7`), 2026-10-08, PR #2486

**`identifier-from-the-harness` — Identifier format (Q3).** Small questions once `scope-by-content-not-presence` and `project-automation-is-a-dependency` hold. Identifiers come from the harness, with `unknown` when the harness cannot name the model, so that the record never looks more complete than it is.

First posted as [comment 6035743101 on PR #2467](https://github.com/torrust/torrust-tracker/pull/2467#issuecomment-6035743101) on 2026-10-07 under the number label `C3`.

### Q4 - Trailer key

`Assisted-by`, for compatibility with the kernel and Fedora, or a project-specific key that adds the
role, such as `AI-Model:` or `Generated-by:`? Added by Jose Celano.

#### Positions

##### Q4 - Cameron Garnham (`da2ce7`), 2026-10-08, PR #2486

**`precedent-key-acceptable` — Trailer key (Q4).** Small questions once `scope-by-content-not-presence` and `project-automation-is-a-dependency` hold. The precedent key is acceptable.

First posted as [comment 6035743101 on PR #2467](https://github.com/torrust/torrust-tracker/pull/2467#issuecomment-6035743101) on 2026-10-07 under the number label `C3`.

### Q5 - Source of truth

The harness knows which models ran, through model-switch notices, subagent configuration, and
session logs. Who is responsible for collecting the list: the agent, the harness, or the human
committer? Added by Jose Celano.

#### Positions

##### Q5 - Cameron Garnham (`da2ce7`), 2026-10-08, PR #2486

**`harness-configuration-is-the-source-of-truth` — Source of truth (Q5).** For project automation the harness configuration is the source of truth and is tracked, by `project-automation-is-a-dependency`.

First posted as [comment 6035743101 on PR #2467](https://github.com/torrust/torrust-tracker/pull/2467#issuecomment-6035743101) on 2026-10-07 under the number label `C4`.

### Q6 - Human edits and rewrites

How is the list kept correct when a human edits AI output, when commits are reworded or rebased, or
when changes move between commits? Added by Jose Celano.

#### Positions

##### Q6 - Cameron Garnham (`da2ce7`), 2026-10-08, PR #2486

**`human-edits-need-no-correction` — Human edits (Q6).** For human edits, rewrites and rebases, no correction of a model record is required, because the record is a tool record and the human's certification governs, by `acceptance-is-the-choice` and `tool-record-not-authorship`.

First posted as [comment 6035743101 on PR #2467](https://github.com/torrust/torrust-tracker/pull/2467#issuecomment-6035743101) on 2026-10-07 under the number label `C4`.

### Q7 - Enforcement

Document the rule only, or also validate the trailer format in a `commit-msg` check? A tool can
check the format of a declared trailer. It cannot check whether a declaration is complete. Added by
Jose Celano.

#### Positions

##### Q7 - Cameron Garnham (`da2ce7`), 2026-10-08, PR #2486

**`format-checkable-completeness-not` — Enforcement (Q7).** A format check on declared trailers is admissible, because form is checkable; a completeness check is not, because completeness is not, by `tool-record-not-authorship`. The role-aware evidence record of option O4 is optional for the same reason.

First posted as [comment 6035743101 on PR #2467](https://github.com/torrust/torrust-tracker/pull/2467#issuecomment-6035743101) on 2026-10-07 under the number label `C5`.

### Q8 - Interim rule

Should `AGENTS.md` forbid provider-default AI trailers before the decision? Added by Jose Celano.

#### Positions

##### Q8 - Cameron Garnham (`da2ce7`), 2026-10-08, PR #2486

**`no-provider-default-trailers-meanwhile` — Interim rule (Q8).** Until the Outcome is recorded, agents do not add provider-default AI trailers, consistent with `humans-author-and-certify`.

First posted as [comment 6035743101 on PR #2467](https://github.com/torrust/torrust-tracker/pull/2467#issuecomment-6035743101) on 2026-10-07 under the number label `C6`.

### Q9 - Legal input

Who reviews the EU AI Act question, and does the answer change the required level of detail? Added
by Jose Celano.

#### Positions

##### Q9 - Cameron Garnham (`da2ce7`), 2026-10-08, PR #2486

**`legal-input-parked` — Legal input (Q9).** Parked, by `regulation-targets-systems-acting-on-people` and `provisional-and-regulation-independent`. The rule is written so that a later legal answer changes the level of detail recorded, not the structure of the rule.

First posted as [comment 6035743101 on PR #2467](https://github.com/torrust/torrust-tracker/pull/2467#issuecomment-6035743101) on 2026-10-07 under the number label `C7`.

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

### Q11 - The shape of the rule

The proposed Outcome as one rule, whose consequences for Q1 to Q9 the entries under those topics state. None of the opening author's open questions covers this; the topic is new in this round. Added by Cameron Garnham (`da2ce7`).

#### Positions

##### Q11 - Cameron Garnham (`da2ce7`), 2026-10-08, PR #2486

**`humans-author-and-certify` — Humans author and certify.** The co-author trailer is reserved for human identities. A model cannot hold authorship or accountability. Standing rules and direction belong to the human who sets them, so what a model produces under them is a rendering of that intent, not an authorship claim by the model.

**`tool-record-not-authorship` — Models are instruments; the record is a tool record.** Any record of model involvement is an instrument record, of the same kind as a toolchain version in a verification evidence file. Its purposes are analytics and reproducibility of process. It makes no authorship claim and no compliance claim. It is a format contract, not a completeness claim: a check can verify the form of a declaration and never that the declaration is complete.

**`scope-by-content-not-presence` — Scope is bounded by content, not by presence.** A declaration names a model only where that model's generated text or code survives in the diff. Rendering and checking tools applied to the author's own content (spelling, grammar, formatting, translation of the author's own prose) and advisory use (exploration, review, planning) are excluded by definition. Under this rule the question "where do I declare my spell-checker" has the answer "nowhere, by rule", and the enumeration problem of `enumeration-has-no-boundary` does not arise.

**`project-automation-is-a-dependency` — Project automation is a dependency; personal tools are not.** A public process attracts provenance of its own accord: what the project's automation runs is visible in tracked bytes and changes only through a pull request, so a model that acts inside it can be named exactly where a personal tool cannot. The record's subject is not that a model was used but the choices it may make in that process, such as certifying a new tagged release; this rule proposes to pin such a model's identity so that a choice can be traced to it. An AI model that the project's own automation uses (the Copilot review handler, the orchestrated review lanes, any future agent in a workflow) is a dependency of the same kind: its identity is pinned in tracked bytes, changed only through a pull request, and checkable by a gate, in the way a lockfile pins a crate version. That record is an identity record, not a reproducibility guarantee, because a vendor may change a model under an unchanged identifier and a run is not reproducible even at a fixed one; the record states this limit. A contributor's personal tool is not a project dependency, so the project neither pins nor records it; a coarse declaration in the form used by the Linux kernel and Fedora, without a model name, remains optional for anyone who wishes to give one.

**`declared-not-detected` — Provenance is declared, not detected.** The record consists of what the committing human and the project's pinned automation declare. Watermarks and detectors are not read as provenance, because by `watermarks-mark-rendering-not-authorship` they mark rendering rather than authorship, and a mark on a human's edited sentence states nothing the record should repeat.

**`provisional-and-regulation-independent` — The rule is provisional and must not depend on the regulation's terms.** The Outcome is revisited when a standard supplies a classification of model involvement. Nothing in the rule may need to know what the regulation means by "choice", "generated" or "provider"; a future draft that does is over-reaching, by `regulation-targets-systems-acting-on-people`.

First posted as [comment 6035743101 on PR #2467](https://github.com/torrust/torrust-tracker/pull/2467#issuecomment-6035743101) on 2026-10-07 under the number labels `B1`, `B2`, `B3`, `B4`, `B5` and `B6`.

## Outcome

Pending: no decision recorded.
