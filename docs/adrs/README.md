# Architectural Decision Records (ADRs)

This directory contains the architectural decision records (ADRs) for the project.
ADRs document architectural decisions — what was decided, why, and what alternatives
were considered.

More info: <https://adr.github.io/>.

See [index.md](index.md) for the full list of ADRs.

## How to Add a New ADR

Generate the timestamp prefix (UTC):

```shell
date -u +"%Y%m%d%H%M%S"
```

Create a new Markdown file using the format `YYYYMMDDHHMMSS_snake_case_title.md`:

```shell
20230510152112_title.md
```

Then add a row to the [Index](index.md) table.

There is no rigid template. A typical ADR includes:

- **Description** — the problem or context motivating the decision
- **Agreement** — what was decided and why
- **Date** — decision date (`YYYY-MM-DD`)
- **References** — related issues, PRs, external docs
