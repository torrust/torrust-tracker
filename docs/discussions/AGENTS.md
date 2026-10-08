# `docs/discussions/` — Design Discussions

This directory keeps design discussions that maintainers want to review before anything is
decided: conversations with contributors or AI agents, the proposals that started them, and the
positions participants take on each question.

A discussion is input, not a decision. Its conclusions take effect only when the owner of the
affected work records them in the owning issue or EPIC specification, an ADR, or another canonical
document. Do not cite a discussion as the source of truth for a rule; follow its links to the
decision.

## Layout

Group discussions by the issue or EPIC they inform, then by start date and topic. For example:

```text
docs/discussions/
├── AGENTS.md
└── 2003-overhaul-guardrails-and-automation/
    ├── 20261003-goals-and-boundaries/
    │   └── README.md
    ├── 20261003-semantic-linking-knowledge-graph/
    │   ├── README.md
    │   └── initial-proposal.md
    └── 20261003-specifications-and-rationale/
        └── README.md
```

- Name the group folder like the issue specification folder: `<issue-number>-<slug>`.
- Prefix each discussion folder with its start date, as analysis and research folders do.
- `README.md` is the primary document. Start it from the
  [discussion template](../templates/DISCUSSION.md): metadata, context, topics with attributed
  positions, and the Outcome.
- Keep source material as written. If it must change to pass the linters, say so at its top.

## Rounds

A discussion grows through rounds. A round is one participant's pull request that adds or revises
only that participant's own content. Rounds let several people take part without one pull request,
or one author, carrying the whole discussion.

1. **Opening round.** The opening author copies the template, writes the context and the first
   topics, may add their own positions, and requests review from the decision owner, the owner of
   the affected issue or EPIC.
2. **Contribution rounds.** Any participant opens a pull request that appends their own position
   entries, and new topics if needed. Participants take turns at their own pace; no round waits for
   another participant to reply.
3. **Decision round.** The decision owner records the Outcome in a separate pull request.

### Rules for Every Round

- **Own words only.** Write only your own entries. Never transcribe or edit another participant's
  position. A position posted elsewhere, for example in a pull-request comment, is linked from the
  discussion, not copied, until its author adds it in a round.
- **Per topic and append-only.** Each topic has a stable ID (`Q1`, `Q2`, ...) that is never
  renumbered or reused. Entries under a topic name the participant, the date, and the round's pull
  request. To change your position, append a new entry that names the one it replaces; you may
  correct only the wording of your own earlier entries.
- **Attribution and AI assistance.** An entry belongs to the human participant who submits it. If
  an AI assistant drafted or researched it, the entry says so and names the tool and model when
  known. The opening author states AI assistance for the context in the metadata table.
- **One round, one participant.** A pull request that changes several participants' entries is not
  a round; split it.

### Merge Gate for a Round

A round merges when the document is sound, not when reviewers agree with it. A reviewer may block a
round only for:

- a failing linter, frontmatter check, or broken link;
- a missing or wrong attribution;
- a factual claim about the repository or its history that does not hold;
- an edit to another participant's content (an entry, a topic's text, or the opening author's
  context), or to the Outcome outside a decision round.

A reviewer who disagrees with a position replies in their own round.

### Decision Round

Agreement is needed only here. The decision owner writes the Outcome: for each topic, the decision
(accepted, revised, or rejected), the entries it relies on cited by their heading links, and the
canonical document that carries the decision, such as the owning issue or EPIC specification or an
ADR. The decision owner then sets the Status to "Decided".

Once the Outcome is recorded and links the canonical document, the discussion takes no more rounds
and is edited only to repair links. A new disagreement starts a new discussion that links the old
one.

### Discussions Opened Before Rounds

Discussions opened before this convention keep their structure. One that is still open adopts the
template in a round by its opening author, who moves the open questions into Topics without
changing anyone's words.

## Related

- [Analysis documents](../analysis/) — studies of the project's own code
- [Research documents](../research/) — external topics and ecosystem practice
- [Issue specifications](../issues/) — planned work and its decisions
- [ADRs](../adrs/) — architectural decision records
- [Discussion template](../templates/DISCUSSION.md) — the starting point for a new discussion
