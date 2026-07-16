# `docs/research/` — Research Documents

This directory contains research documents that investigate external topics, technologies,
or patterns relevant to the project. Unlike **analysis** documents (which study the project's
own code), research documents look outward — at how other projects solve similar problems,
what the ecosystem offers, or what best practices exist.

## Purpose

A research document answers questions like:

- How do other projects (Rust or otherwise) handle this problem?
- What are the standard patterns, libraries, or approaches?
- What are the trade-offs between different options?
- What does the ecosystem recommend?

Research is the **input** to design decisions: it feeds into feature definitions, ADRs,
and implementation plans.

## Timestamp Prefix Convention

Like analysis folders, research folders use a **timestamp prefix** to make it clear when
the research was conducted:

```text
docs/research/
├── AGENTS.md
├── 20260716-console-shutdown-patterns/
│   └── README.md
└── ...
```

## Lifecycle

- **Research may become outdated** as the ecosystem evolves. Always check the timestamp
  before relying on old research.
- **Research may stay relevant** if the technologies and patterns it covers have not
  changed significantly.
- **Old research can be cleaned up** when no longer relevant.

## Related

- [Analysis documents](../analysis/) — studies of the project's own code
- [Feature definitions](../features/) — product-oriented descriptions of desired features
- [ADRs](../adrs/) — architectural decision records
