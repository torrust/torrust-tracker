---
semantic-links:
  skill-links:
    - create-adr
  related-artifacts:
    - docs/index.md
    - docs/adrs/index.md
    - .github/skills/dev/planning/create-adr/SKILL.md
---

# Architectural Decision Records (ADRs)

This directory contains the repository-level architectural decision records (ADRs) for the project.
ADRs document architectural decisions — what was decided, why, and what alternatives
were considered.

More info: <https://adr.github.io/>.

See [index.md](index.md) for the full list of root ADRs.

## How to Add a New ADR

Generate the timestamp prefix (UTC):

```shell
date -u +"%Y%m%d%H%M%S"
```

First choose the ADR collection by the decision's architectural scope:

- `docs/adrs/` for repository-wide, multi-package, and inter-package decisions.
- `packages/<package>/docs/adrs/` for decisions owned solely by an extractable package.

Shared configuration, protocol behavior, dependency policy, workspace conventions, and
inter-package contracts are root decisions even when only one package's implementation changes.
Do not choose a location solely from the paths touched by the change.

Create a new Markdown file in the selected collection using the format
`YYYYMMDDHHMMSS_snake_case_title.md`:

```shell
20230510152112_example_decision.md
```

Then add a row only to that collection's index. Every package-local collection requires its own
`README.md` and `index.md`; do not duplicate local ADRs in the root [Index](index.md) table.

When a local decision becomes repository-wide, create a root ADR that links to and supersedes the
local ADR. Keep the local ADR and its local index entry as historical context. The tracker-client
CLI I/O ADR and the root global CLI output ADR are the existing example.

There is no rigid template. A typical ADR includes:

- **Description** — the problem or context motivating the decision
- **Agreement** — what was decided and why
- **Date** — decision date (`YYYY-MM-DD`)
- **References** — related issues, PRs, external docs
