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

<!-- cspell:ignore Deployers -->

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

This discussion was opened in PR #2467 with a single-PR structure. It adopted the
[discussion template](../../../templates/DISCUSSION.md) in a round by its opening author, under the
round rules in [`docs/discussions/AGENTS.md`](../../AGENTS.md). The context below is unchanged; the
open questions moved into [Topics](#topics) without changes to their words.

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

No positions yet.

### Q2 - Which models count

Only models whose output is in the diff, or also models that explored, reviewed, or planned? If
reviewers count, they need a separate key so they are not confused with authors. Added by Jose
Celano.

#### Positions

No positions yet.

### Q3 - Identifier format

Is `<harness>:<vendor>/<model-id>` acceptable, and how should unknown versions be recorded? Added by
Jose Celano.

#### Positions

No positions yet.

### Q4 - Trailer key

`Assisted-by`, for compatibility with the kernel and Fedora, or a project-specific key that adds the
role, such as `AI-Model:` or `Generated-by:`? Added by Jose Celano.

#### Positions

No positions yet.

### Q5 - Source of truth

The harness knows which models ran, through model-switch notices, subagent configuration, and
session logs. Who is responsible for collecting the list: the agent, the harness, or the human
committer? Added by Jose Celano.

#### Positions

No positions yet.

### Q6 - Human edits and rewrites

How is the list kept correct when a human edits AI output, when commits are reworded or rebased, or
when changes move between commits? Added by Jose Celano.

#### Positions

No positions yet.

### Q7 - Enforcement

Document the rule only, or also validate the trailer format in a `commit-msg` check? A tool can
check the format of a declared trailer. It cannot check whether a declaration is complete. Added by
Jose Celano.

#### Positions

No positions yet.

### Q8 - Interim rule

Should `AGENTS.md` forbid provider-default AI trailers before the decision? Added by Jose Celano.

#### Positions

No positions yet.

### Q9 - Legal input

Who reviews the EU AI Act question, and does the answer change the required level of detail? Added
by Jose Celano.

#### Positions

No positions yet.

## Outcome

Pending: no decision recorded.
