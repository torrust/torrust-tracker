# `docs/discussions/` — Design Discussions

This directory keeps design discussions that maintainers want to review before anything is
decided: conversations with contributors or AI agents, the proposals that started them, and the
draft conclusions drawn from them.

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
    └── 20261003-semantic-linking-knowledge-graph/
        ├── README.md
        └── initial-proposal.md
```

- Name the group folder like the issue specification folder: `<issue-number>-<slug>`.
- Prefix each discussion folder with its start date, as analysis and research folders do.
- `README.md` is the primary document: status, participants, reviewer, the review of the source
  material, draft conclusions, and open questions.
- Keep source material as written. If it must change to pass the linters, say so at its top.

## Lifecycle

1. Draft the discussion and its conclusions on a branch.
2. Open a pull request and request review from the owner of the affected work.
3. Record the outcome in the discussion's `README.md` (accepted, revised, or rejected) and link the
   canonical document that carries it.

## Related

- [Analysis documents](../analysis/) — studies of the project's own code
- [Research documents](../research/) — external topics and ecosystem practice
- [Issue specifications](../issues/) — planned work and its decisions
- [ADRs](../adrs/) — architectural decision records
